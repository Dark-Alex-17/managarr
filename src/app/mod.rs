use std::process;

use anyhow::anyhow;
use colored::Colorize;
use log::{debug, error};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;

use crate::app::context_clues::{build_context_clue_string, SERVARR_CONTEXT_CLUES};
use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, RadarrData};
use crate::models::{HorizontallyScrollableText, Route, TabRoute, TabState};
use crate::network::NetworkEvent;

#[cfg(test)]
#[path = "app_tests.rs"]
mod app_tests;
pub mod context_clues;
pub mod key_binding;
mod key_binding_tests;
pub mod radarr;

const DEFAULT_ROUTE: Route = Route::Radarr(ActiveRadarrBlock::Movies, None);

pub struct App<'a> {
  navigation_stack: Vec<Route>,
  network_tx: Option<Sender<NetworkEvent>>,
  cancellation_token: CancellationToken,
  pub server_tabs: TabState,
  pub error: HorizontallyScrollableText,
  pub tick_until_poll: u64,
  pub ticks_until_scroll: u64,
  pub tick_count: u64,
  pub is_routing: bool,
  pub is_loading: bool,
  pub should_refresh: bool,
  pub should_ignore_quit_key: bool,
  pub config: AppConfig,
  pub data: Data<'a>,
}

impl<'a> App<'a> {
  pub fn new(
    network_tx: Sender<NetworkEvent>,
    config: AppConfig,
    cancellation_token: CancellationToken,
  ) -> Self {
    App {
      network_tx: Some(network_tx),
      config,
      cancellation_token,
      ..App::default()
    }
  }

  pub async fn dispatch_network_event(&mut self, action: NetworkEvent) {
    debug!("Dispatching network event: {action:?}");

    if !self.should_refresh {
      self.is_loading = true;
    }

    if let Some(network_tx) = &self.network_tx {
      if let Err(e) = network_tx.send(action).await {
        self.is_loading = false;
        error!("Failed to send event. {e:?}");
        self.handle_error(anyhow!(e));
      }
    }
  }

  pub fn reset_tick_count(&mut self) {
    self.tick_count = 0;
  }

  // Allowing this code for now since we'll eventually be implementing additional Servarr support and we'll need it then
  #[allow(dead_code)]
  pub fn reset(&mut self) {
    self.reset_tick_count();
    self.error = HorizontallyScrollableText::default();
    self.data = Data::default();
  }

  pub fn handle_error(&mut self, error: anyhow::Error) {
    if self.error.text.is_empty() {
      self.error = error.to_string().into();
    }
  }

  pub async fn on_tick(&mut self, is_first_render: bool) {
    if self.tick_count % self.tick_until_poll == 0 || self.is_routing || self.should_refresh {
      if let Route::Radarr(active_radarr_block, _) = self.get_current_route() {
        self
          .radarr_on_tick(*active_radarr_block, is_first_render)
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

  pub fn reset_cancellation_token(&mut self) -> CancellationToken {
    self.cancellation_token = CancellationToken::new();
    self.should_refresh = true;
    self.is_loading = false;

    self.cancellation_token.clone()
  }

  pub fn pop_and_push_navigation_stack(&mut self, route: Route) {
    self.pop_navigation_stack();
    self.push_navigation_stack(route);
  }

  pub fn get_current_route(&self) -> &Route {
    self.navigation_stack.last().unwrap_or(&DEFAULT_ROUTE)
  }
}

impl<'a> Default for App<'a> {
  fn default() -> Self {
    App {
      navigation_stack: vec![DEFAULT_ROUTE],
      network_tx: None,
      cancellation_token: CancellationToken::new(),
      error: HorizontallyScrollableText::default(),
      server_tabs: TabState::new(vec![
        TabRoute {
          title: "Radarr",
          route: ActiveRadarrBlock::Movies.into(),
          help: format!(
            "<↑↓> scroll | ←→ change tab | {}  ",
            build_context_clue_string(&SERVARR_CONTEXT_CLUES)
          ),
          contextual_help: None,
        },
        TabRoute {
          title: "Sonarr",
          route: Route::Sonarr,
          help: format!("{}  ", build_context_clue_string(&SERVARR_CONTEXT_CLUES)),
          contextual_help: None,
        },
      ]),
      tick_until_poll: 400,
      ticks_until_scroll: 4,
      tick_count: 0,
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
pub struct Data<'a> {
  pub radarr_data: RadarrData<'a>,
}

pub trait ServarrConfig {
  fn validate(&self);
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct AppConfig {
  pub radarr: RadarrConfig,
}

impl ServarrConfig for AppConfig {
  fn validate(&self) {
    self.radarr.validate();
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RadarrConfig {
  pub host: Option<String>,
  pub port: Option<u16>,
  pub uri: Option<String>,
  pub api_token: String,
  pub ssl_cert_path: Option<String>,
}

impl ServarrConfig for RadarrConfig {
  fn validate(&self) {
    if self.host.is_none() && self.uri.is_none() {
      log_and_print_error("'host' or 'uri' is required for Radarr configuration".to_owned());
      process::exit(1);
    }
  }
}

impl Default for RadarrConfig {
  fn default() -> Self {
    RadarrConfig {
      host: Some("localhost".to_string()),
      port: Some(7878),
      uri: None,
      api_token: "".to_string(),
      ssl_cert_path: None,
    }
  }
}

pub fn log_and_print_error(error: String) {
  error!("{}", error);
  eprintln!("error: {}", error.red());
}
