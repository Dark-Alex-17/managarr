use anyhow::{anyhow, Error, Result};
use colored::Colorize;
use itertools::Itertools;
use log::{debug, error};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{fs, process};
use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;
use veil::Redact;

use crate::app::context_clues::{build_context_clue_string, SERVARR_CONTEXT_CLUES};
use crate::cli::Command;
use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, RadarrData};
use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, SonarrData};
use crate::models::{HorizontallyScrollableText, Route, TabRoute, TabState};
use crate::network::NetworkEvent;

#[cfg(test)]
#[path = "app_tests.rs"]
mod app_tests;
pub mod context_clues;
pub mod key_binding;
mod key_binding_tests;
pub mod radarr;
pub mod sonarr;

pub struct App<'a> {
  navigation_stack: Vec<Route>,
  network_tx: Option<Sender<NetworkEvent>>,
  pub cancellation_token: CancellationToken,
  pub is_first_render: bool,
  pub server_tabs: TabState,
  pub error: HorizontallyScrollableText,
  pub tick_until_poll: u64,
  pub ticks_until_scroll: u64,
  pub tick_count: u64,
  pub is_routing: bool,
  pub is_loading: bool,
  pub should_refresh: bool,
  pub should_ignore_quit_key: bool,
  pub cli_mode: bool,
  pub data: Data<'a>,
}

impl App<'_> {
  pub fn new(
    network_tx: Sender<NetworkEvent>,
    config: AppConfig,
    cancellation_token: CancellationToken,
  ) -> Self {
    let mut server_tabs = Vec::new();
    let help = format!(
      "<↑↓> scroll | ←→ change tab | {}  ",
      build_context_clue_string(&SERVARR_CONTEXT_CLUES)
    );

    if let Some(radarr_configs) = config.radarr {
      let mut idx = 0;
      for radarr_config in radarr_configs {
        let name = if let Some(name) = radarr_config.name.clone() {
          name
        } else {
          idx+=1;
          format!("Radarr {}", idx)
        };
        
        server_tabs.push(TabRoute {
          title: name,
          route: ActiveRadarrBlock::Movies.into(),
          help: help.clone(),
          contextual_help: None,
          config: Some(radarr_config),
        });
      }
    }

    if let Some(sonarr_configs) = config.sonarr {
      let mut idx = 0;
      
      for sonarr_config in sonarr_configs {
        let name = if let Some(name) = sonarr_config.name.clone() {
          name
        } else {
          idx+=1;
          format!("Sonarr {}", idx)
        };
        
        server_tabs.push(TabRoute {
          title: name,
          route: ActiveSonarrBlock::Series.into(),
          help: help.clone(),
          contextual_help: None,
          config: Some(sonarr_config),
        });
      }
    }

    let weight_sorted_tabs = server_tabs
      .into_iter()
      .sorted_by(|tab1, tab2| {
        Ord::cmp(
          tab1.config.as_ref().unwrap().weight.as_ref().unwrap_or(&1000),
          tab2.config.as_ref().unwrap().weight.as_ref().unwrap_or(&1000),
        )
      })
      .collect();

    App {
      network_tx: Some(network_tx),
      cancellation_token,
      server_tabs: TabState::new(weight_sorted_tabs),
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

  #[allow(dead_code)]
  pub fn reset(&mut self) {
    self.reset_tick_count();
    self.error = HorizontallyScrollableText::default();
    self.is_first_render = true;
    self.data = Data::default();
  }

  pub fn handle_error(&mut self, error: Error) {
    if self.error.text.is_empty() {
      self.error = error.to_string().into();
    }
  }

  pub async fn on_tick(&mut self) {
    if self.tick_count % self.tick_until_poll == 0 || self.is_routing || self.should_refresh {
      match self.get_current_route() {
        Route::Radarr(active_radarr_block, _) => self.radarr_on_tick(active_radarr_block).await,
        Route::Sonarr(active_sonarr_block, _) => self.sonarr_on_tick(active_sonarr_block).await,
        _ => (),
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
    if !self.navigation_stack.is_empty() {
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

  pub fn get_current_route(&self) -> Route {
    *self
      .navigation_stack
      .last()
      .unwrap_or(&self.server_tabs.tabs.first().unwrap().route)
  }
}

impl Default for App<'_> {
  fn default() -> Self {
    App {
      navigation_stack: Vec::new(),
      network_tx: None,
      cancellation_token: CancellationToken::new(),
      error: HorizontallyScrollableText::default(),
      is_first_render: true,
      server_tabs: TabState::new(Vec::new()),
      tick_until_poll: 400,
      ticks_until_scroll: 4,
      tick_count: 0,
      is_loading: false,
      is_routing: false,
      should_refresh: false,
      should_ignore_quit_key: false,
      cli_mode: false,
      data: Data::default(),
    }
  }
}

#[cfg(test)]
impl App<'_> {
  pub fn test_default() -> Self {
    App {
      server_tabs: TabState::new(vec![
        TabRoute {
          title: "Radarr".to_owned(),
          route: ActiveRadarrBlock::Movies.into(),
          help: format!(
            "<↑↓> scroll | ←→ change tab | {}  ",
            build_context_clue_string(&SERVARR_CONTEXT_CLUES)
          ),
          contextual_help: None,
          config: Some(ServarrConfig::default()),
        },
        TabRoute {
          title: "Sonarr".to_owned(),
          route: ActiveSonarrBlock::Series.into(),
          help: format!(
            "<↑↓> scroll | ←→ change tab | {}  ",
            build_context_clue_string(&SERVARR_CONTEXT_CLUES)
          ),
          contextual_help: None,
          config: Some(ServarrConfig::default()),
        },
      ]),
      ..App::default()
    }
  }
}

#[derive(Default)]
pub struct Data<'a> {
  pub radarr_data: RadarrData<'a>,
  pub sonarr_data: SonarrData<'a>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct AppConfig {
  pub radarr: Option<Vec<ServarrConfig>>,
  pub sonarr: Option<Vec<ServarrConfig>>,
}

impl AppConfig {
  pub fn validate(&self) {
    if self.radarr.is_none() && self.sonarr.is_none() {
      log_and_print_error(
        "No Servarr configuration provided in the specified configuration file".to_owned(),
      );
      process::exit(1);
    }

    if let Some(radarr_configs) = &self.radarr {
      radarr_configs.iter().for_each(|config| config.validate());
    }

    if let Some(sonarr_configs) = &self.sonarr {
      sonarr_configs.iter().for_each(|config| config.validate());
    }
  }

  pub fn verify_config_present_for_cli(&self, command: &Command) {
    let msg = |servarr: &str| {
      log_and_print_error(format!(
        "{} configuration missing; Unable to run any {} commands.",
        servarr, servarr
      ))
    };
    match command {
      Command::Radarr(_) if self.radarr.is_none() => {
        msg("Radarr");
        process::exit(1);
      }
      Command::Sonarr(_) if self.sonarr.is_none() => {
        msg("Sonarr");
        process::exit(1);
      }
      _ => (),
    }
  }

  pub fn post_process_initialization(&mut self) {
    if let Some(radarr_configs) = self.radarr.as_mut() {
      for radarr_config in radarr_configs {
        radarr_config.post_process_initialization();
      }
    }

    if let Some(sonarr_configs) = self.sonarr.as_mut() {
      for sonarr_config in sonarr_configs {
        sonarr_config.post_process_initialization();
      }
    }
  }
}

#[derive(Redact, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct ServarrConfig {
  #[serde(default, deserialize_with = "deserialize_optional_env_var")]
  pub name: Option<String>,
  #[serde(default, deserialize_with = "deserialize_optional_env_var")]
  pub host: Option<String>,
  #[serde(default, deserialize_with = "deserialize_u16_env_var")]
  pub port: Option<u16>,
  #[serde(default, deserialize_with = "deserialize_optional_env_var")]
  pub uri: Option<String>,
  #[serde(default, deserialize_with = "deserialize_u16_env_var")]
  pub weight: Option<u16>,
  #[serde(default, deserialize_with = "deserialize_optional_env_var")]
  #[redact]
  pub api_token: Option<String>,
  #[serde(default, deserialize_with = "deserialize_optional_env_var")]
  pub api_token_file: Option<String>,
  #[serde(default, deserialize_with = "deserialize_optional_env_var")]
  pub ssl_cert_path: Option<String>,
}

impl ServarrConfig {
  fn validate(&self) {
    if self.host.is_none() && self.uri.is_none() {
      log_and_print_error("'host' or 'uri' is required for configuration".to_owned());
      process::exit(1);
    }

    if self.api_token_file.is_none() && self.api_token.is_none() {
      log_and_print_error(
        "'api_token' or 'api_token_path' is required for configuration".to_owned(),
      );
      process::exit(1);
    }
  }

  pub fn post_process_initialization(&mut self) {
    if let Some(api_token_file) = self.api_token_file.as_ref() {
      if !PathBuf::from(api_token_file).exists() {
        log_and_print_error(format!(
          "The specified {} API token file does not exist",
          api_token_file
        ));
        process::exit(1);
      }

      let api_token = fs::read_to_string(api_token_file)
        .map_err(|e| anyhow!(e))
        .unwrap();
      self.api_token = Some(api_token.trim().to_owned());
    }
  }
}

impl Default for ServarrConfig {
  fn default() -> Self {
    ServarrConfig {
      name: None,
      host: Some("localhost".to_string()),
      port: None,
      uri: None,
      weight: None,
      api_token: Some(String::new()),
      api_token_file: None,
      ssl_cert_path: None,
    }
  }
}

pub fn log_and_print_error(error: String) {
  error!("{}", error);
  eprintln!("error: {}", error.red());
}

fn deserialize_optional_env_var<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
  D: serde::Deserializer<'de>,
{
  let s: Option<String> = Option::deserialize(deserializer)?;
  match s {
    Some(value) => {
      let interpolated = interpolate_env_vars(&value);
      Ok(Some(interpolated))
    }
    None => Ok(None),
  }
}

fn deserialize_u16_env_var<'de, D>(deserializer: D) -> Result<Option<u16>, D::Error>
where
  D: serde::Deserializer<'de>,
{
  let s: Option<String> = Option::deserialize(deserializer)?;
  match s {
    Some(value) => {
      let interpolated = interpolate_env_vars(&value);
      interpolated
        .parse::<u16>()
        .map(Some)
        .map_err(serde::de::Error::custom)
    }
    None => Ok(None),
  }
}

fn interpolate_env_vars(s: &str) -> String {
  let result = s.to_string();
  let scrubbing_regex = Regex::new(r#"[\s\{\}!\$^\(\)\[\]\\\|`'"]+"#).unwrap();
  let var_regex = Regex::new(r"\$\{(.*?)\}").unwrap();

  var_regex
    .replace_all(s, |caps: &regex::Captures<'_>| {
      if let Some(mat) = caps.get(1) {
        if let Ok(value) = std::env::var(mat.as_str()) {
          return scrubbing_regex.replace_all(&value, "").to_string();
        }
      }

      scrubbing_regex.replace_all(&result, "").to_string()
    })
    .to_string()
}
