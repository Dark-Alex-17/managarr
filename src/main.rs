#![warn(rust_2018_idioms)]

use std::panic::PanicHookInfo;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{io, panic, process};

use anyhow::anyhow;
use anyhow::Result;
use clap::{
  command, crate_authors, crate_description, crate_name, crate_version, CommandFactory, Parser,
};
use clap_complete::generate;
use colored::Colorize;
use crossterm::execute;
use crossterm::terminal::{
  disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
};
use log::error;
use network::NetworkTrait;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use tokio::sync::mpsc::Receiver;
use tokio::sync::{mpsc, Mutex};
use tokio_util::sync::CancellationToken;

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

static MIN_TERM_WIDTH: u16 = 205;
static MIN_TERM_HEIGHT: u16 = 40;

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
  let config = confy::load("managarr", "config")?;
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
        let mut network = Network::new(&app_nw, cancellation_token);

        if let Err(e) = cli::handle_command(&app, command, &mut network).await {
          eprintln!("error: {}", e.to_string().red());
          process::exit(1);
        }
      }
      Command::Completions { shell } => {
        let mut cli = Cli::command();
        generate(shell, &mut cli, "managarr", &mut io::stdout())
      }
    },
    None => {
      let app_nw = Arc::clone(&app);
      std::thread::spawn(move || start_networking(sync_network_rx, &app_nw, cancellation_token));
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
) {
  let mut network = Network::new(app, cancellation_token);

  while let Some(network_event) = network_rx.recv().await {
    if let Err(e) = network.handle_network_event(network_event).await {
      error!("Encountered an error handling network event: {e:?}");
    }
  }
}

async fn start_ui(app: &Arc<Mutex<App<'_>>>) -> Result<()> {
  let (width, height) = size()?;
  if width < MIN_TERM_WIDTH || height < MIN_TERM_HEIGHT {
    return Err(anyhow!(
      "Terminal too small. Minimum size required: {}x{}; current terminal size: {}x{}",
      MIN_TERM_WIDTH,
      MIN_TERM_HEIGHT,
      width,
      height
    ));
  }

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
