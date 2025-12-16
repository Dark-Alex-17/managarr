use anyhow::{Error, Result, anyhow};
use colored::Colorize;
use itertools::Itertools;
use log::{debug, error};
use regex::Regex;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::{fs, process};
use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;
use veil::Redact;

use crate::cli::Command;
use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, RadarrData};
use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, SonarrData};
use crate::models::servarr_models::KeybindingItem;
use crate::models::stateful_table::StatefulTable;
use crate::models::{HorizontallyScrollableText, Route, TabRoute, TabState};
use crate::network::NetworkEvent;
#[cfg(test)]
use {
  crate::models::ScrollableText,
  crate::models::radarr_models,
  crate::models::radarr_models::MinimumAvailability,
  crate::models::radarr_models::MovieMonitor,
  crate::models::servarr_data::modals::EditIndexerModal,
  crate::models::servarr_data::radarr::modals::AddMovieModal,
  crate::models::servarr_data::radarr::modals::{
    EditCollectionModal, EditMovieModal, MovieDetailsModal,
  },
  crate::network::radarr_network::radarr_network_test_utils::test_utils::*,
  chrono::DateTime,
  strum::IntoEnumIterator,
  std::fmt::Debug,
  crate::models::stateful_table::SortOption
};

#[cfg(test)]
mod app_tests;
pub mod context_clues;
pub mod key_binding;
mod key_binding_tests;
pub mod radarr;
pub mod sonarr;

#[cfg(test)]
macro_rules! sort_option {
  ($field:ident) => {
    SortOption {
      name: "Something",
      cmp_fn: Some(|a, b| a.$field.cmp(&b.$field))
    }
  }
}

pub struct App<'a> {
  navigation_stack: Vec<Route>,
  network_tx: Option<Sender<NetworkEvent>>,
  pub cancellation_token: CancellationToken,
  pub is_first_render: bool,
  pub server_tabs: TabState,
  pub keymapping_table: Option<StatefulTable<KeybindingItem>>,
  pub error: HorizontallyScrollableText,
  pub tick_until_poll: u64,
  pub ticks_until_scroll: u64,
  pub tick_count: u64,
  pub is_routing: bool,
  pub is_loading: bool,
  pub should_refresh: bool,
  pub ignore_special_keys_for_textbox_input: bool,
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

    if let Some(radarr_configs) = config.radarr {
      let mut unnamed_idx = 0;
      let radarr_tabs = radarr_configs.into_iter().map(|radarr_config| {
        let name = if let Some(name) = radarr_config.name.clone() {
          name
        } else {
          unnamed_idx += 1;
          format!("Radarr {unnamed_idx}")
        };

        TabRoute {
          title: name,
          route: ActiveRadarrBlock::Movies.into(),
          contextual_help: None,
          config: Some(radarr_config),
        }
      });
      server_tabs.extend(radarr_tabs);
    }

    if let Some(sonarr_configs) = config.sonarr {
      let mut unnamed_idx = 0;
      let sonarr_tabs = sonarr_configs.into_iter().map(|sonarr_config| {
        let name = if let Some(name) = sonarr_config.name.clone() {
          name
        } else {
          unnamed_idx += 1;
          format!("Sonarr {unnamed_idx}")
        };

        TabRoute {
          title: name,
          route: ActiveSonarrBlock::Series.into(),
          contextual_help: None,
          config: Some(sonarr_config),
        }
      });
      server_tabs.extend(sonarr_tabs);
    }

    let weight_sorted_tabs = server_tabs
      .into_iter()
      .sorted_by(|tab1, tab2| {
        Ord::cmp(
          tab1
            .config
            .as_ref()
            .unwrap()
            .weight
            .as_ref()
            .unwrap_or(&1000),
          tab2
            .config
            .as_ref()
            .unwrap()
            .weight
            .as_ref()
            .unwrap_or(&1000),
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

    if let Some(network_tx) = &self.network_tx
      && let Err(e) = network_tx.send(action).await
    {
      self.is_loading = false;
      error!("Failed to send event. {e:?}");
      self.handle_error(anyhow!(e));
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
    if self.tick_count.is_multiple_of(self.tick_until_poll)
      || self.is_routing
      || self.should_refresh
    {
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
    *self.navigation_stack.last().unwrap_or(
      &self
        .server_tabs
        .tabs
        .first()
        .expect("At least one server tab must exist")
        .route,
    )
  }
}

impl Default for App<'_> {
  fn default() -> Self {
    App {
      navigation_stack: Vec::new(),
      network_tx: None,
      cancellation_token: CancellationToken::new(),
      keymapping_table: None,
      error: HorizontallyScrollableText::default(),
      is_first_render: true,
      server_tabs: TabState::new(Vec::new()),
      tick_until_poll: 400,
      ticks_until_scroll: 4,
      tick_count: 0,
      is_loading: false,
      is_routing: false,
      should_refresh: false,
      ignore_special_keys_for_textbox_input: false,
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
          contextual_help: None,
          config: Some(ServarrConfig::default()),
        },
        TabRoute {
          title: "Sonarr".to_owned(),
          route: ActiveSonarrBlock::Series.into(),
          contextual_help: None,
          config: Some(ServarrConfig::default()),
        },
      ]),
      ..App::default()
    }
  }

  pub fn test_default_fully_populated() -> Self {
    let quality_profile_name = "HD - 1080p".to_owned();
    let mut add_movie_modal = AddMovieModal {
      tags: "alex".into(),
      ..AddMovieModal::default()
    };
    add_movie_modal
      .root_folder_list
      .set_items(vec![root_folder()]);
    add_movie_modal
      .monitor_list
      .set_items(MovieMonitor::iter().collect());
    add_movie_modal
      .minimum_availability_list
      .set_items(MinimumAvailability::iter().collect());
    add_movie_modal
      .quality_profile_list
      .set_items(vec![quality_profile_name.clone()]);

    let mut add_movie_search_result_table = StatefulTable::default();
    add_movie_search_result_table.set_items(vec![add_movie_search_result()]);
    add_movie_search_result_table.sorting(vec![sort_option!(tmdb_id)]);
    add_movie_search_result_table.search = Some("something".into());
    add_movie_search_result_table.filter = Some("something".into());

    let mut edit_movie_modal = EditMovieModal {
      monitored: Some(true),
      path: "/nfs/movies".into(),
      tags: "alex".into(),
      ..EditMovieModal::default()
    };
    edit_movie_modal
      .minimum_availability_list
      .set_items(MinimumAvailability::iter().collect());
    edit_movie_modal
      .quality_profile_list
      .set_items(vec![quality_profile_name.clone()]);

    let mut edit_collection_modal = EditCollectionModal {
      monitored: Some(true),
      path: "/nfs/movies".into(),
      search_on_add: Some(true),
      ..EditCollectionModal::default()
    };
    edit_collection_modal
      .minimum_availability_list
      .set_items(MinimumAvailability::iter().collect());
    edit_collection_modal
      .quality_profile_list
      .set_items(vec![quality_profile_name.clone()]);

    let edit_indexer_modal = EditIndexerModal {
      name: "DrunkenSlug".into(),
      enable_rss: Some(true),
      enable_automatic_search: Some(true),
      enable_interactive_search: Some(true),
      url: "http://127.0.0.1:9696/1/".into(),
      api_key: "someApiKey".into(),
      seed_ratio: "ratio".into(),
      tags: "25".into(),
      priority: 1,
    };

    let indexer_settings = radarr_models::IndexerSettings {
      allow_hardcoded_subs: true,
      availability_delay: 0,
      id: 1,
      maximum_size: 1234,
      minimum_age: 12,
      prefer_indexer_flags: true,
      retention: 30,
      rss_sync_interval: 60,
      whitelisted_hardcoded_subs: "eng".into(),
    };

    let mut indexer_test_results = StatefulTable::default();
    indexer_test_results.set_items(vec![indexer_test_result()]);
    indexer_test_results.sorting(vec![sort_option!(name)]);
    indexer_test_results.search = Some("something".into());
    indexer_test_results.filter = Some("something".into());

    let mut movie_details_modal = MovieDetailsModal {
      movie_details: ScrollableText::with_string("Some information".to_owned()),
      file_details: "Some file info".to_owned(),
      audio_details: "Some audio info".to_owned(),
      video_details: "Some video info".to_owned(),
      ..MovieDetailsModal::default()
    };
    movie_details_modal
      .movie_history
      .set_items(vec![movie_history_item()]);
    movie_details_modal
      .movie_cast
      .set_items(vec![cast_credit()]);
    movie_details_modal
      .movie_crew
      .set_items(vec![crew_credit()]);
    movie_details_modal
      .movie_releases
      .set_items(vec![torrent_release(), usenet_release()]);
    movie_details_modal.movie_releases.sorting(vec![sort_option!(indexer_id)]);

    let mut radarr_data = RadarrData {
      disk_space_vec: vec![diskspace()],
      version: "1.2.3.4".to_owned(),
      quality_profile_map: quality_profile_map(),
      tags_map: tags_map(),
      updates: updates(),
      start_time: DateTime::from(DateTime::parse_from_rfc3339("2023-05-20T21:29:16Z").unwrap()),
      add_movie_search: Some("test".into()),
      add_movie_modal: Some(add_movie_modal),
      add_searched_movies: Some(add_movie_search_result_table),
      edit_movie_modal: Some(edit_movie_modal),
      edit_collection_modal: Some(edit_collection_modal),
      edit_indexer_modal: Some(edit_indexer_modal),
      edit_root_folder: Some("/nfs/movies".into()),
      indexer_settings: Some(indexer_settings),
      indexer_test_errors: Some("error".into()),
      indexer_test_all_results: Some(indexer_test_results),
      movie_details_modal: Some(movie_details_modal),
      delete_movie_files: true,
      ..RadarrData::default()
    };
    radarr_data.root_folders.set_items(vec![root_folder()]);
    radarr_data.movies.set_items(vec![movie()]);
    radarr_data.movies.sorting(vec![sort_option!(id)]);
    radarr_data.movies.search = Some("Something".into());
    radarr_data.movies.filter = Some("Something".into());
    radarr_data.collections.set_items(vec![collection()]);
    radarr_data.collections.sorting(vec![sort_option!(id)]);
    radarr_data.collections.search = Some("Something".into());
    radarr_data.collections.filter = Some("Something".into());
    radarr_data.collection_movies.set_items(vec![collection_movie()]);
    radarr_data.downloads.set_items(vec![download_record()]);
    radarr_data.blocklist.set_items(vec![blocklist_item()]);
    radarr_data.blocklist.sorting(vec![sort_option!(id)]);
    radarr_data.indexers.set_items(vec![indexer()]);
    radarr_data.indexers.sorting(vec![sort_option!(id)]);
    radarr_data.indexers.search = Some("Something".into());
    radarr_data.indexers.filter = Some("Something".into());
    radarr_data.logs.set_items(vec![log_line().into()]);
    radarr_data.log_details.set_items(vec![log_line().into()]);
    radarr_data.tasks.set_items(vec![task()]);
    radarr_data.queued_events.set_items(vec![queued_event()]);
    App {
      data: Data {
        radarr_data,
        sonarr_data: SonarrData::default(),
      },
      server_tabs: TabState::new(vec![
        TabRoute {
          title: "Radarr".to_owned(),
          route: ActiveRadarrBlock::Movies.into(),
          contextual_help: None,
          config: Some(ServarrConfig::default()),
        },
        TabRoute {
          title: "Sonarr".to_owned(),
          route: ActiveSonarrBlock::Series.into(),
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
  pub theme: Option<String>,
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
        "{servarr} configuration missing; Unable to run any {servarr} commands."
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
  #[serde(
    default,
    deserialize_with = "deserialize_optional_env_var_header_map",
    serialize_with = "serialize_header_map"
  )]
  pub custom_headers: Option<HeaderMap>,
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
          "The specified {api_token_file} API token file does not exist"
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
      custom_headers: None,
    }
  }
}

pub fn log_and_print_error(error: String) {
  error!("{error}");
  eprintln!("error: {}", error.red());
}

fn serialize_header_map<S>(headers: &Option<HeaderMap>, serializer: S) -> Result<S::Ok, S::Error>
where
  S: serde::Serializer,
{
  if let Some(headers) = headers {
    let mut map = HashMap::new();
    for (name, value) in headers.iter() {
      let name_str = name.as_str().to_string();
      let value_str = value
        .to_str()
        .map_err(serde::ser::Error::custom)?
        .to_string();

      map.insert(name_str, value_str);
    }
    map.serialize(serializer)
  } else {
    serializer.serialize_none()
  }
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

fn deserialize_optional_env_var_header_map<'de, D>(
  deserializer: D,
) -> Result<Option<HeaderMap>, D::Error>
where
  D: serde::Deserializer<'de>,
{
  let opt: Option<HashMap<String, String>> = Option::deserialize(deserializer)?;
  match opt {
    Some(map) => {
      let mut header_map = HeaderMap::new();
      for (k, v) in map.iter() {
        let name = HeaderName::from_bytes(k.as_bytes()).map_err(serde::de::Error::custom)?;
        let value_str = interpolate_env_vars(v);
        let value = HeaderValue::from_str(&value_str).map_err(serde::de::Error::custom)?;
        header_map.insert(name, value);
      }
      Ok(Some(header_map))
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
  let scrubbing_regex = Regex::new(r#"[\s{}!$^()\[\]\\|`'"]+"#).unwrap();
  let var_regex = Regex::new(r"\$\{(.*?)}").unwrap();

  var_regex
    .replace_all(s, |caps: &regex::Captures<'_>| {
      if let Some(mat) = caps.get(1)
        && let Ok(value) = std::env::var(mat.as_str())
      {
        return scrubbing_regex.replace_all(&value, "").to_string();
      }

      scrubbing_regex.replace_all(&result, "").to_string()
    })
    .to_string()
}
