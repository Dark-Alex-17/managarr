use std::io;
use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use crossterm::execute;
use crossterm::terminal::{
  disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use log::debug;
use tokio::sync::{mpsc, Mutex};
use tokio::sync::mpsc::Receiver;
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
mod network;
mod ui;
mod utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {}

#[tokio::main]
async fn main() -> Result<()> {
  log4rs::init_config(utils::init_logging_config())?;
  Cli::parse();

  let config = confy::load("managarr", "config")?;
  let (sync_network_tx, sync_network_rx) = mpsc::channel(500);

  let app = Arc::new(Mutex::new(App::new(sync_network_tx, 5000 / 250, config)));

  let app_nw = Arc::clone(&app);

  std::thread::spawn(move || start_networking(sync_network_rx, &app_nw));

  start_ui(&app).await?;

  Ok(())
}

#[tokio::main]
async fn start_networking(mut network_rx: Receiver<NetworkEvent>, app: &Arc<Mutex<App>>) {
  let network = Network::new(reqwest::Client::new(), app);

  while let Some(network_event) = network_rx.recv().await {
    debug!("Received network event: {:?}", network_event);
    network.handle_network_event(network_event).await;
  }
}

async fn start_ui(app: &Arc<Mutex<App>>) -> Result<()> {
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
        if key == Key::Char('q') {
          break;
        }

        handlers::handle_key_events(key, &mut app).await;
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
