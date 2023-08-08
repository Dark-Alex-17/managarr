use std::time::Duration;

use log::{debug, error};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;
use tokio::time::Instant;

use crate::app::radarr::{ActiveRadarrBlock, RadarrData};
use crate::network::radarr_network::RadarrEvent;
use crate::network::NetworkEvent;

pub(crate) mod key_binding;
pub mod models;
pub mod radarr;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Route {
  Radarr(ActiveRadarrBlock),
}

impl From<ActiveRadarrBlock> for Route {
  fn from(active_radarr_block: ActiveRadarrBlock) -> Route {
    Route::Radarr(active_radarr_block)
  }
}

const DEFAULT_ROUTE: Route = Route::Radarr(ActiveRadarrBlock::Movies);

pub struct App {
  navigation_stack: Vec<Route>,
  network_tx: Option<Sender<NetworkEvent>>,
  pub client: Client,
  pub title: &'static str,
  pub tick_until_poll: u64,
  pub tick_count: u64,
  pub last_tick: Instant,
  pub network_tick_frequency: Duration,
  pub is_routing: bool,
  pub is_loading: bool,
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

  pub async fn dispatch(&mut self, action: NetworkEvent) {
    if let Some(network_tx) = &self.network_tx {
      if let Err(e) = network_tx.send(action).await {
        self.is_loading = false;
        error!("Failed to send event. {:?}", e);
      }
    }
  }

  pub fn reset_tick_count(&mut self) {
    self.tick_count = 0;
  }

  pub fn reset(&mut self) {
    self.reset_tick_count();
    self.data = Data::default();
  }

  pub async fn on_tick(&mut self, is_first_render: bool) {
    if self.tick_count % self.tick_until_poll == 0 || self.is_routing {
      match self.get_current_route() {
        Route::Radarr(active_radarr_block) => {
          let active_block = active_radarr_block.clone();

          if is_first_render {
            self.dispatch(RadarrEvent::GetQualityProfiles.into()).await;
            self.dispatch(RadarrEvent::GetOverview.into()).await;
            self.dispatch(RadarrEvent::GetStatus.into()).await;
            self.dispatch_by_radarr_block(active_block.clone()).await;
          }

          if self.is_routing
            || self
              .network_tick_frequency
              .checked_sub(self.last_tick.elapsed())
              .unwrap_or_else(|| Duration::from_secs(0))
              .is_zero()
          {
            self.dispatch_by_radarr_block(active_block).await;
          }
        }
      }

      self.is_routing = false;
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

  pub fn get_current_route(&self) -> &Route {
    self.navigation_stack.last().unwrap_or(&DEFAULT_ROUTE)
  }
}

impl Default for App {
  fn default() -> Self {
    App {
      navigation_stack: vec![DEFAULT_ROUTE],
      network_tx: None,
      client: Client::new(),
      title: "Managarr",
      tick_until_poll: 20,
      tick_count: 0,
      network_tick_frequency: Duration::from_secs(10),
      last_tick: Instant::now(),
      is_loading: false,
      is_routing: false,
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
