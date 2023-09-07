#![warn(rust_2018_idioms)]

use std::panic::PanicInfo;
use std::sync::Arc;
use std::{io, panic};

use anyhow::Result;
use crossterm::execute;
use crossterm::terminal::{
  disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use tokio::sync::mpsc::Receiver;
use tokio::sync::{mpsc, Mutex};
use tokio_util::sync::CancellationToken;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::app::App;
use crate::event::input_event::{Events, InputEvent};
use crate::event::Key;
use crate::network::{Network, NetworkEvent};
use crate::ui::ui;

mod app;
mod event;
mod handlers;
mod logos;
mod models;
mod network;
mod ui;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
  log4rs::init_config(utils::init_logging_config())?;
  panic::set_hook(Box::new(|info| {
    panic_hook(info);
  }));

  let config = confy::load("managarr", "config")?;
  let (sync_network_tx, sync_network_rx) = mpsc::channel(500);
  let cancellation_token = CancellationToken::new();

  let app = Arc::new(Mutex::new(App::new(
    sync_network_tx,
    config,
    cancellation_token.clone(),
  )));

  let app_nw = Arc::clone(&app);

  std::thread::spawn(move || start_networking(sync_network_rx, &app_nw, cancellation_token));

  start_ui(&app).await?;

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
    network.handle_network_event(network_event).await;
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
fn panic_hook(info: &PanicInfo<'_>) {
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
fn panic_hook(info: &PanicInfo<'_>) {
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
