use std::time::Duration;

use anyhow::anyhow;
use log::error;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;
use tokio::time::Instant;

use crate::app::radarr::{ActiveRadarrBlock, RadarrData};
use crate::models::{HorizontallyScrollableText, Route, TabRoute, TabState};
use crate::network::NetworkEvent;

pub(crate) mod key_binding;
pub mod radarr;

const DEFAULT_ROUTE: Route = Route::Radarr(ActiveRadarrBlock::Movies);

pub struct App {
  navigation_stack: Vec<Route>,
  network_tx: Option<Sender<NetworkEvent>>,
  pub server_tabs: TabState,
  pub error: HorizontallyScrollableText,
  pub client: Client,
  pub title: &'static str,
  pub tick_until_poll: u64,
  pub tick_count: u64,
  pub last_tick: Instant,
  pub network_tick_frequency: Duration,
  pub is_routing: bool,
  pub is_loading: bool,
  pub should_refresh: bool,
  pub should_ignore_quit_key: bool,
  pub config: AppConfig,
  pub data: Data,
}

impl App {
  pub fn new(network_tx: Sender<NetworkEvent>, config: AppConfig) -> Self {
    App {
      network_tx: Some(network_tx),
      config,
      ..App::default()
    }
  }

  pub async fn dispatch_network_event(&mut self, action: NetworkEvent) {
    if let Some(network_tx) = &self.network_tx {
      if let Err(e) = network_tx.send(action).await {
        self.is_loading = false;
        error!("Failed to send event. {:?}", e);
        self.handle_error(anyhow!(e));
      }
    }
  }

  pub fn reset_tick_count(&mut self) {
    self.tick_count = 0;
  }

  pub fn reset(&mut self) {
    self.reset_tick_count();
    self.error = HorizontallyScrollableText::default();
    self.data = Data::default();
  }

  pub fn handle_error(&mut self, error: anyhow::Error) {
    if self.error.text.is_empty() {
      self.error = HorizontallyScrollableText::new(error.to_string());
    }
  }

  pub async fn on_tick(&mut self, is_first_render: bool) {
    if self.tick_count % self.tick_until_poll == 0 || self.is_routing || self.should_refresh {
      if let Route::Radarr(active_radarr_block) = self.get_current_route() {
        self
          .radarr_on_tick(active_radarr_block.clone(), is_first_render)
          .await;
      }

      self.is_routing = false;
      self.should_refresh = false;
    }

    self.tick_count += 1;
  }

  pub fn push_navigation_stack(&mut self, route: Route) {
    self.navigation_stack.push(route);
    self.is_routing = true;
  }

  pub fn pop_navigation_stack(&mut self) {
    self.is_routing = true;
    if self.navigation_stack.len() > 1 {
      self.navigation_stack.pop();
    }
  }

  pub fn pop_and_push_navigation_stack(&mut self, route: Route) {
    self.pop_navigation_stack();
    self.push_navigation_stack(route);
  }

  pub fn get_current_route(&self) -> &Route {
    self.navigation_stack.last().unwrap_or(&DEFAULT_ROUTE)
  }
}

impl Default for App {
  fn default() -> Self {
    App {
      navigation_stack: vec![DEFAULT_ROUTE],
      network_tx: None,
      error: HorizontallyScrollableText::default(),
      server_tabs: TabState::new(vec![
        TabRoute {
          title: "Radarr".to_owned(),
          route: ActiveRadarrBlock::Movies.into(),
          help: "<↑↓> scroll | ←→ change tab | <tab> change servarr | <?> help | <q> quit  "
            .to_owned(),
          contextual_help: None,
        },
        TabRoute {
          title: "Sonarr".to_owned(),
          route: Route::Sonarr,
          help: "<tab> change servarr | <?> help | <q> quit  ".to_owned(),
          contextual_help: None,
        },
      ]),
      client: Client::new(),
      title: "Managarr",
      tick_until_poll: 50,
      tick_count: 0,
      network_tick_frequency: Duration::from_secs(20),
      last_tick: Instant::now(),
      is_loading: false,
      is_routing: false,
      should_refresh: false,
      should_ignore_quit_key: false,
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

#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use tokio::sync::mpsc;

  use crate::network::radarr_network::RadarrEvent;

  use super::*;

  #[test]
  fn test_navigation_stack_methods() {
    let mut app = App::default();

    assert_eq!(app.get_current_route(), &DEFAULT_ROUTE);

    app.push_navigation_stack(ActiveRadarrBlock::Downloads.into());

    assert_eq!(
      app.get_current_route(),
      &ActiveRadarrBlock::Downloads.into()
    );
    assert!(app.is_routing);

    app.is_routing = false;
    app.pop_and_push_navigation_stack(ActiveRadarrBlock::Collections.into());

    assert_eq!(
      app.get_current_route(),
      &ActiveRadarrBlock::Collections.into()
    );
    assert!(app.is_routing);

    app.is_routing = false;
    app.pop_navigation_stack();

    assert_eq!(app.get_current_route(), &DEFAULT_ROUTE);
    assert!(app.is_routing);

    app.is_routing = false;
    app.pop_navigation_stack();

    assert_eq!(app.get_current_route(), &DEFAULT_ROUTE);
    assert!(app.is_routing);
  }

  #[test]
  fn test_reset_tick_count() {
    let mut app = App {
      tick_count: 2,
      ..App::default()
    };

    app.reset_tick_count();

    assert_eq!(app.tick_count, 0);
  }

  #[test]
  fn test_reset() {
    let mut app = App {
      tick_count: 2,
      error: String::from("Test error").into(),
      data: Data {
        radarr_data: RadarrData {
          version: "test".to_owned(),
          ..RadarrData::default()
        },
      },
      ..App::default()
    };

    app.reset();

    assert_eq!(app.tick_count, 0);
    assert_eq!(app.error, HorizontallyScrollableText::default());
    assert_eq!(app.data.radarr_data.version, String::default());
  }

  #[test]
  fn test_handle_error() {
    let mut app = App::default();
    let test_string = "Testing";

    app.handle_error(anyhow!(test_string));

    assert_eq!(app.error.stationary_style(), test_string);

    app.handle_error(anyhow!("Testing a different error"));

    assert_eq!(app.error.stationary_style(), test_string);
  }

  #[tokio::test]
  async fn test_on_tick_first_render() {
    let (sync_network_tx, mut sync_network_rx) = mpsc::channel::<NetworkEvent>(500);

    let mut app = App {
      tick_until_poll: 2,
      network_tx: Some(sync_network_tx),
      ..App::default()
    };

    assert_eq!(app.tick_count, 0);

    app.on_tick(true).await;
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetQualityProfiles.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetRootFolders.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetOverview.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetStatus.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetMovies.into()
    );
    assert_eq!(
      sync_network_rx.recv().await.unwrap(),
      RadarrEvent::GetDownloads.into()
    );
    assert!(!app.is_routing);
    assert!(!app.should_refresh);
    assert_eq!(app.tick_count, 1);
  }

  #[tokio::test]
  async fn test_on_tick_routing() {
    let mut app = App {
      tick_until_poll: 2,
      tick_count: 2,
      is_routing: true,
      ..App::default()
    };

    app.on_tick(false).await;
    assert!(!app.is_routing);
  }

  #[tokio::test]
  async fn test_on_tick_should_refresh() {
    let mut app = App {
      tick_until_poll: 2,
      tick_count: 2,
      should_refresh: true,
      ..App::default()
    };

    app.on_tick(false).await;
    assert!(!app.should_refresh);
  }
}
