#![warn(rust_2018_idioms)]

use std::fs::{self, File};
use std::io::BufReader;
use std::panic::PanicHookInfo;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use std::{io, panic, process};

use anyhow::anyhow;
use anyhow::Result;
use app::{log_and_print_error, AppConfig, ServarrConfig};
use clap::{
  command, crate_authors, crate_description, crate_name, crate_version, CommandFactory, Parser,
};
use clap_complete::generate;
use colored::Colorize;
use crossterm::execute;
use crossterm::terminal::{
  disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use log::{error, warn};
use network::NetworkTrait;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use reqwest::{Certificate, Client};
use tokio::select;
use tokio::sync::mpsc::Receiver;
use tokio::sync::{mpsc, Mutex};
use tokio_util::sync::CancellationToken;
use utils::tail_logs;

use crate::app::App;
use crate::cli::Command;
use crate::event::input_event::{Events, InputEvent};
use crate::event::Key;
use crate::network::{Network, NetworkEvent};
use crate::ui::ui;

mod app;
mod cli;
mod event;
mod handlers;
mod logos;
mod models;
mod network;
mod ui;
mod utils;

#[derive(Debug, Parser)]
#[command(
  name = crate_name!(),
  author = crate_authors!(),
  version = crate_version!(),
  about = crate_description!(),
  help_template = "\
{before-help}{name} {version}
{author-with-newline}
{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
"
)]
struct Cli {
  #[command(subcommand)]
  command: Option<Command>,
  #[arg(
    long,
    global = true,
    value_parser,
    env = "MANAGARR_CONFIG_FILE",
    help = "The Managarr configuration file to use"
  )]
  config: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
  log4rs::init_config(utils::init_logging_config())?;
  panic::set_hook(Box::new(|info| {
    panic_hook(info);
  }));
  let running = Arc::new(AtomicBool::new(true));
  let r = running.clone();
  let args = Cli::parse();
  let config = if let Some(ref config_file) = args.config {
    load_config(config_file.to_str().expect("Invalid config file specified"))?
  } else {
    confy::load("managarr", "config")?
  };
  config.validate();
  let reqwest_client = build_network_client(&config);
  let (sync_network_tx, sync_network_rx) = mpsc::channel(500);
  let cancellation_token = CancellationToken::new();
  let ctrlc_cancellation_token = cancellation_token.clone();

  ctrlc::set_handler(move || {
    ctrlc_cancellation_token.cancel();
    r.store(false, Ordering::SeqCst);
    process::exit(1);
  })
  .expect("Error setting Ctrl-C handler");

  let app = Arc::new(Mutex::new(App::new(
    sync_network_tx,
    config,
    cancellation_token.clone(),
  )));

  match args.command {
    Some(command) => match command {
      Command::Radarr(_) => {
        let app_nw = Arc::clone(&app);
        let mut network = Network::new(&app_nw, cancellation_token, reqwest_client);

        if let Err(e) = cli::handle_command(&app, command, &mut network).await {
          eprintln!("error: {}", e.to_string().red());
          process::exit(1);
        }
      }
      Command::Completions { shell } => {
        let mut cli = Cli::command();
        generate(shell, &mut cli, "managarr", &mut io::stdout())
      }
      Command::TailLogs { no_color } => tail_logs(no_color).await,
    },
    None => {
      let app_nw = Arc::clone(&app);
      std::thread::spawn(move || {
        start_networking(sync_network_rx, &app_nw, cancellation_token, reqwest_client)
      });
      start_ui(&app).await?;
    }
  }

  Ok(())
}

#[tokio::main]
async fn start_networking(
  mut network_rx: Receiver<NetworkEvent>,
  app: &Arc<Mutex<App<'_>>>,
  cancellation_token: CancellationToken,
  client: Client,
) {
  let mut network = Network::new(app, cancellation_token, client);

  loop {
    select! {
      Some(network_event) = network_rx.recv() => {
        if let Err(e) = network.handle_network_event(network_event).await {
          error!("Encountered an error handling network event: {e:?}");
        }
      }
      _ = network.cancellation_token.cancelled() => {
        warn!("Clearing network channel");
        while network_rx.try_recv().is_ok() {
          // Discard the message
        }
        network.reset_cancellation_token().await;
      }
    }
  }
}

async fn start_ui(app: &Arc<Mutex<App<'_>>>) -> Result<()> {
  let mut stdout = io::stdout();
  enable_raw_mode()?;

  execute!(stdout, EnterAlternateScreen)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;
  terminal.clear()?;
  terminal.hide_cursor()?;

  let input_events = Events::new();
  let mut is_first_render = true;

  loop {
    let mut app = app.lock().await;

    terminal.draw(|f| ui(f, &mut app))?;

    match input_events.next()? {
      InputEvent::KeyEvent(key) => {
        if key == Key::Char('q') && !app.should_ignore_quit_key {
          break;
        }

        handlers::handle_events(key, &mut app);
      }

      InputEvent::Tick => app.on_tick(is_first_render).await,
    }

    is_first_render = false;
  }

  terminal.show_cursor()?;
  disable_raw_mode()?;
  execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
  terminal.show_cursor()?;

  Ok(())
}

#[cfg(debug_assertions)]
fn panic_hook(info: &PanicHookInfo<'_>) {
  use backtrace::Backtrace;
  use crossterm::style::Print;

  let location = info.location().unwrap();

  let msg = match info.payload().downcast_ref::<&'static str>() {
    Some(s) => *s,
    None => match info.payload().downcast_ref::<String>() {
      Some(s) => &s[..],
      None => "Box<Any>",
    },
  };

  let stacktrace: String = format!("{:?}", Backtrace::new()).replace('\n', "\n\r");

  disable_raw_mode().unwrap();
  execute!(
    io::stdout(),
    LeaveAlternateScreen,
    Print(format!(
      "thread '<unnamed>' panicked at '{msg}', {location}\n\r{stacktrace}"
    )),
  )
  .unwrap();
}

fn load_config(path: &str) -> Result<AppConfig> {
  let file = File::open(path).map_err(|e| anyhow!(e))?;
  let reader = BufReader::new(file);
  let config = serde_yaml::from_reader(reader)?;
  Ok(config)
}

fn build_network_client(config: &AppConfig) -> Client {
  let mut client_builder = Client::builder()
    .pool_max_idle_per_host(10)
    .http2_keep_alive_interval(Duration::from_secs(5))
    .tcp_keepalive(Duration::from_secs(5));

  if let Some(ref cert_path) = config.radarr.ssl_cert_path {
    let cert = create_cert(cert_path, "Radarr");
    client_builder = client_builder.add_root_certificate(cert);
  }

  match client_builder.build() {
    Ok(client) => client,
    Err(e) => {
      error!("Unable to create reqwest client: {}", e);
      eprintln!("error: {}", e.to_string().red());
      process::exit(1);
    }
  }
}

fn create_cert(cert_path: &String, servarr_name: &str) -> Certificate {
  match fs::read(cert_path) {
    Ok(cert) => match Certificate::from_pem(&cert) {
      Ok(certificate) => certificate,
      Err(_) => {
        log_and_print_error(format!(
          "Unable to read the specified {} SSL certificate",
          servarr_name
        ));
        process::exit(1);
      }
    },
    Err(_) => {
      log_and_print_error(format!(
        "Unable to open specified {} SSL certificate",
        servarr_name
      ));
      process::exit(1);
    }
  }
}

#[cfg(not(debug_assertions))]
fn panic_hook(info: &PanicHookInfo<'_>) {
  use human_panic::{handle_dump, print_msg, Metadata};

  let meta = Metadata {
    version: env!("CARGO_PKG_VERSION").into(),
    name: env!("CARGO_PKG_NAME").into(),
    authors: env!("CARGO_PKG_AUTHORS").replace(":", ", ").into(),
    homepage: env!("CARGO_PKG_HOMEPAGE").into(),
  };
  let file_path = handle_dump(&meta, info);
  disable_raw_mode().unwrap();
  execute!(io::stdout(), LeaveAlternateScreen).unwrap();
  print_msg(file_path, &meta).expect("human-panic: printing error message to console failed");
}
