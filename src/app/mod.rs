use log::error;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;
use tui::widgets::TableState;

use crate::app::radarr::RadarrData;

use super::network::RadarrEvent;

pub(crate) mod key_binding;
pub mod radarr;

pub struct App {
  network_tx: Option<Sender<RadarrEvent>>,
  pub client: Client,
  pub title: &'static str,
  pub tick_until_poll: u64,
  pub tick_count: u64,
  pub config: AppConfig,
  pub data: Data,
}

impl App {
  pub fn new(network_tx: Sender<RadarrEvent>, tick_until_poll: u64, config: AppConfig) -> Self {
    App {
      network_tx: Some(network_tx),
      tick_until_poll,
      config,
      ..App::default()
    }
  }

  pub async fn dispatch(&mut self, action: RadarrEvent) {
    if let Some(network_tx) = &self.network_tx {
      if let Err(e) = network_tx.send(action).await {
        error!("Failed to send event. {:?}", e);
      }
    }
  }

  pub fn reset_tick_count(&mut self) {
    self.tick_count = 0;
  }

  pub async fn on_tick(&mut self) {
    if self.tick_count % self.tick_until_poll == 0 {
      self.dispatch(RadarrEvent::GetOverview).await;
      self.dispatch(RadarrEvent::GetStatus).await;
      self.dispatch(RadarrEvent::GetMovies).await;
    }

    self.tick_count += 1;
  }
}

impl Default for App {
  fn default() -> Self {
    App {
      network_tx: None,
      client: Client::new(),
      title: "DevTools",
      tick_until_poll: 0,
      tick_count: 0,
      config: AppConfig::default(),
      data: Data::default(),
    }
  }
}

#[derive(Default)]
pub struct Data {
  pub radarr_data: RadarrData,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct AppConfig {
  pub radarr: RadarrConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RadarrConfig {
  pub host: String,
  pub port: Option<u16>,
  pub api_token: String,
}

impl Default for RadarrConfig {
  fn default() -> Self {
    RadarrConfig {
      host: "localhost".to_string(),
      port: Some(7878),
      api_token: "".to_string(),
    }
  }
}

pub struct StatefulTable<T> {
  pub state: TableState,
  pub items: Vec<T>
}

impl<T> Default for StatefulTable<T> {
  fn default() -> StatefulTable<T> {
    StatefulTable {
      state: TableState::default(),
      items: Vec::new()
    }
  }
}

impl<T> StatefulTable<T> {
  pub fn set_items(&mut self, items: Vec<T>) {
    let items_len = items.len();
    self.items = items;
    if !self.items.is_empty() {
      let selected_row = self.state.selected().map_or(0, |i| {
        if i > 0 && i < items_len {
          i
        } else if i >= items_len {
          items_len - 1
        } else {
          0
        }
      });
      self.state.select(Some(selected_row));
    }
  }

  pub fn scroll_down(&mut self) {
    let selected_row = match self.state.selected() {
      Some(i) => {
        if i >= self.items.len() - 1 {
          0
        } else {
          i + 1
        }
      }
      None => 0
    };

    self.state.select(Some(selected_row));
  }

  pub fn scroll_up(&mut self) {
    let selected_row = match self.state.selected() {
      Some(i) => {
        if i == 0 {
          self.items.len() - 1
        } else {
          i - 1
        }
      }
      None => 0
    };

    self.state.select(Some(selected_row));
  }
}
