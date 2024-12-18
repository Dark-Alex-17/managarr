use anyhow::{anyhow, Result};
use indoc::formatdoc;
use log::{debug, info, warn};
use serde_json::{json, Value};
use urlencoding::encode;

use super::{Network, NetworkEvent, NetworkResource};
use crate::models::sonarr_models::{DownloadStatus, MonitorEpisodeBody};
use crate::{
  models::{
    radarr_models::IndexerTestResult,
    servarr_data::{
      modals::{EditIndexerModal, IndexerTestResultModalItem},
      sonarr::{
        modals::{EditSeriesModal, EpisodeDetailsModal, SeasonDetailsModal},
        sonarr_data::ActiveSonarrBlock,
      },
    },
    servarr_models::{
      AddRootFolderBody, CommandBody, DiskSpace, EditIndexerParams, HostConfig, Indexer, Language,
      LogResponse, QualityProfile, QueueEvent, RootFolder, SecurityConfig, Tag, Update,
    },
    sonarr_models::{
      AddSeriesBody, AddSeriesSearchResult, BlocklistItem, BlocklistResponse, DeleteSeriesParams,
      DownloadRecord, DownloadsResponse, EditSeriesParams, Episode, EpisodeFile, IndexerSettings,
      Series, SonarrCommandBody, SonarrHistoryItem, SonarrHistoryWrapper, SonarrRelease,
      SonarrReleaseDownloadBody, SonarrSerdeable, SonarrTask, SonarrTaskName, SystemStatus,
    },
    stateful_table::StatefulTable,
    HorizontallyScrollableText, Route, Scrollable, ScrollableText,
  },
  network::RequestMethod,
  utils::convert_to_gb,
};
#[cfg(test)]
#[path = "sonarr_network_tests.rs"]
mod sonarr_network_tests;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SonarrEvent {
  AddRootFolder(AddRootFolderBody),
  AddSeries(AddSeriesBody),
  AddTag(String),
  ClearBlocklist,
  DeleteBlocklistItem(i64),
  DeleteDownload(i64),
  DeleteEpisodeFile(i64),
  DeleteIndexer(i64),
  DeleteRootFolder(i64),
  DeleteSeries(DeleteSeriesParams),
  DeleteTag(i64),
  DownloadRelease(SonarrReleaseDownloadBody),
  EditAllIndexerSettings(Option<IndexerSettings>),
  EditIndexer(Option<EditIndexerParams>),
  EditSeries(Option<EditSeriesParams>),
  GetAllIndexerSettings,
  GetBlocklist,
  GetDownloads,
  GetHistory(Option<u64>),
  GetHostConfig,
  GetIndexers,
  GetEpisodeDetails(Option<i64>),
  GetEpisodes(Option<i64>),
  GetEpisodeFiles(Option<i64>),
  GetEpisodeHistory(Option<i64>),
  GetLanguageProfiles,
  GetLogs(Option<u64>),
  GetDiskSpace,
  GetQualityProfiles,
  GetQueuedEvents,
  GetRootFolders,
  GetEpisodeReleases(Option<i64>),
  GetSeasonHistory(Option<(i64, i64)>),
  GetSeasonReleases(Option<(i64, i64)>),
  GetSecurityConfig,
  GetSeriesDetails(Option<i64>),
  GetSeriesHistory(Option<i64>),
  GetStatus,
  GetUpdates,
  GetTags,
  GetTasks,
  HealthCheck,
  ListSeries,
  MarkHistoryItemAsFailed(i64),
  SearchNewSeries(Option<String>),
  StartTask(Option<SonarrTaskName>),
  TestIndexer(Option<i64>),
  TestAllIndexers,
  ToggleSeasonMonitoring(Option<(i64, i64)>),
  ToggleEpisodeMonitoring(Option<i64>),
  TriggerAutomaticEpisodeSearch(Option<i64>),
  TriggerAutomaticSeasonSearch(Option<(i64, i64)>),
  TriggerAutomaticSeriesSearch(Option<i64>),
  UpdateAllSeries,
  UpdateAndScanSeries(Option<i64>),
  UpdateDownloads,
}

impl NetworkResource for SonarrEvent {
  fn resource(&self) -> &'static str {
    match &self {
      SonarrEvent::AddTag(_) | SonarrEvent::DeleteTag(_) | SonarrEvent::GetTags => "/tag",
      SonarrEvent::ClearBlocklist => "/blocklist/bulk",
      SonarrEvent::DownloadRelease(_) => "/release",
      SonarrEvent::DeleteBlocklistItem(_) => "/blocklist",
      SonarrEvent::GetAllIndexerSettings | SonarrEvent::EditAllIndexerSettings(_) => {
        "/config/indexer"
      }
      SonarrEvent::GetEpisodeFiles(_) | SonarrEvent::DeleteEpisodeFile(_) => "/episodefile",
      SonarrEvent::GetBlocklist => "/blocklist?page=1&pageSize=10000",
      SonarrEvent::GetDownloads | SonarrEvent::DeleteDownload(_) => "/queue",
      SonarrEvent::GetEpisodes(_) | SonarrEvent::GetEpisodeDetails(_) => "/episode",
      SonarrEvent::GetHistory(_) | SonarrEvent::GetEpisodeHistory(_) => "/history",
      SonarrEvent::GetHostConfig | SonarrEvent::GetSecurityConfig => "/config/host",
      SonarrEvent::GetIndexers | SonarrEvent::DeleteIndexer(_) | SonarrEvent::EditIndexer(_) => {
        "/indexer"
      }
      SonarrEvent::GetLanguageProfiles => "/language",
      SonarrEvent::GetLogs(_) => "/log",
      SonarrEvent::GetDiskSpace => "/diskspace",
      SonarrEvent::GetQualityProfiles => "/qualityprofile",
      SonarrEvent::GetQueuedEvents
      | SonarrEvent::StartTask(_)
      | SonarrEvent::TriggerAutomaticSeriesSearch(_)
      | SonarrEvent::TriggerAutomaticSeasonSearch(_)
      | SonarrEvent::TriggerAutomaticEpisodeSearch(_)
      | SonarrEvent::UpdateAllSeries
      | SonarrEvent::UpdateAndScanSeries(_)
      | SonarrEvent::UpdateDownloads => "/command",
      SonarrEvent::GetRootFolders
      | SonarrEvent::DeleteRootFolder(_)
      | SonarrEvent::AddRootFolder(_) => "/rootfolder",
      SonarrEvent::GetSeasonReleases(_) | SonarrEvent::GetEpisodeReleases(_) => "/release",
      SonarrEvent::GetSeriesHistory(_) | SonarrEvent::GetSeasonHistory(_) => "/history/series",
      SonarrEvent::GetStatus => "/system/status",
      SonarrEvent::GetTasks => "/system/task",
      SonarrEvent::GetUpdates => "/update",
      SonarrEvent::HealthCheck => "/health",
      SonarrEvent::AddSeries(_)
      | SonarrEvent::ListSeries
      | SonarrEvent::GetSeriesDetails(_)
      | SonarrEvent::DeleteSeries(_)
      | SonarrEvent::EditSeries(_)
      | SonarrEvent::ToggleSeasonMonitoring(_) => "/series",
      SonarrEvent::SearchNewSeries(_) => "/series/lookup",
      SonarrEvent::MarkHistoryItemAsFailed(_) => "/history/failed",
      SonarrEvent::TestIndexer(_) => "/indexer/test",
      SonarrEvent::TestAllIndexers => "/indexer/testall",
      SonarrEvent::ToggleEpisodeMonitoring(_) => "/episode/monitor",
    }
  }
}

impl From<SonarrEvent> for NetworkEvent {
  fn from(sonarr_event: SonarrEvent) -> Self {
    NetworkEvent::Sonarr(sonarr_event)
  }
}

impl<'a, 'b> Network<'a, 'b> {
  pub async fn handle_sonarr_event(
    &mut self,
    sonarr_event: SonarrEvent,
  ) -> Result<SonarrSerdeable> {
    match sonarr_event {
      SonarrEvent::AddRootFolder(path) => self
        .add_sonarr_root_folder(path)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::AddSeries(body) => self
        .add_sonarr_series(body)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::AddTag(tag) => self.add_sonarr_tag(tag).await.map(SonarrSerdeable::from),
      SonarrEvent::ClearBlocklist => self
        .clear_sonarr_blocklist()
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::GetAllIndexerSettings => self
        .get_all_sonarr_indexer_settings()
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::DeleteBlocklistItem(blocklist_item_id) => self
        .delete_sonarr_blocklist_item(blocklist_item_id)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::DeleteDownload(download_id) => self
        .delete_sonarr_download(download_id)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::DeleteEpisodeFile(episode_file_id) => self
        .delete_sonarr_episode_file(episode_file_id)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::DeleteIndexer(indexer_id) => self
        .delete_sonarr_indexer(indexer_id)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::DeleteRootFolder(root_folder_id) => self
        .delete_sonarr_root_folder(root_folder_id)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::DeleteSeries(params) => {
        self.delete_series(params).await.map(SonarrSerdeable::from)
      }
      SonarrEvent::DeleteTag(tag_id) => self
        .delete_sonarr_tag(tag_id)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::DownloadRelease(sonarr_release_download_body) => self
        .download_sonarr_release(sonarr_release_download_body)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::EditAllIndexerSettings(params) => self
        .edit_all_sonarr_indexer_settings(params)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::EditIndexer(params) => self
        .edit_sonarr_indexer(params)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::EditSeries(params) => self
        .edit_sonarr_series(params)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::GetBlocklist => self.get_sonarr_blocklist().await.map(SonarrSerdeable::from),
      SonarrEvent::GetDownloads => self.get_sonarr_downloads().await.map(SonarrSerdeable::from),
      SonarrEvent::GetEpisodes(series_id) => self
        .get_episodes(series_id)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::GetEpisodeFiles(series_id) => self
        .get_episode_files(series_id)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::GetEpisodeDetails(episode_id) => self
        .get_episode_details(episode_id)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::GetEpisodeHistory(episode_id) => self
        .get_sonarr_episode_history(episode_id)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::GetHistory(events) => self
        .get_sonarr_history(events)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::GetHostConfig => self
        .get_sonarr_host_config()
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::GetIndexers => self.get_sonarr_indexers().await.map(SonarrSerdeable::from),
      SonarrEvent::GetLanguageProfiles => self
        .get_sonarr_language_profiles()
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::GetLogs(events) => self
        .get_sonarr_logs(events)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::GetDiskSpace => self.get_sonarr_diskspace().await.map(SonarrSerdeable::from),
      SonarrEvent::GetQualityProfiles => self
        .get_sonarr_quality_profiles()
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::GetQueuedEvents => self
        .get_queued_sonarr_events()
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::GetRootFolders => self
        .get_sonarr_root_folders()
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::GetEpisodeReleases(params) => self
        .get_episode_releases(params)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::GetSeasonHistory(params) => self
        .get_sonarr_season_history(params)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::GetSeasonReleases(params) => self
        .get_season_releases(params)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::GetSecurityConfig => self
        .get_sonarr_security_config()
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::GetSeriesDetails(series_id) => self
        .get_series_details(series_id)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::GetSeriesHistory(series_id) => self
        .get_sonarr_series_history(series_id)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::GetStatus => self.get_sonarr_status().await.map(SonarrSerdeable::from),
      SonarrEvent::GetTags => self.get_sonarr_tags().await.map(SonarrSerdeable::from),
      SonarrEvent::GetTasks => self.get_sonarr_tasks().await.map(SonarrSerdeable::from),
      SonarrEvent::GetUpdates => self.get_sonarr_updates().await.map(SonarrSerdeable::from),
      SonarrEvent::HealthCheck => self
        .get_sonarr_healthcheck()
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::ListSeries => self.list_series().await.map(SonarrSerdeable::from),
      SonarrEvent::MarkHistoryItemAsFailed(history_item_id) => self
        .mark_sonarr_history_item_as_failed(history_item_id)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::SearchNewSeries(query) => self
        .search_sonarr_series(query)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::StartTask(task_name) => self
        .start_sonarr_task(task_name)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::TestIndexer(indexer_id) => self
        .test_sonarr_indexer(indexer_id)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::TestAllIndexers => self
        .test_all_sonarr_indexers()
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::ToggleEpisodeMonitoring(episode_id) => self
        .toggle_sonarr_episode_monitoring(episode_id)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::ToggleSeasonMonitoring(params) => self
        .toggle_sonarr_season_monitoring(params)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::TriggerAutomaticSeasonSearch(params) => self
        .trigger_automatic_season_search(params)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::TriggerAutomaticSeriesSearch(series_id) => self
        .trigger_automatic_series_search(series_id)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::TriggerAutomaticEpisodeSearch(episode_id) => self
        .trigger_automatic_episode_search(episode_id)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::UpdateAllSeries => self.update_all_series().await.map(SonarrSerdeable::from),
      SonarrEvent::UpdateAndScanSeries(series_id) => self
        .update_and_scan_series(series_id)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::UpdateDownloads => self
        .update_sonarr_downloads()
        .await
        .map(SonarrSerdeable::from),
    }
  }

  async fn add_sonarr_root_folder(
    &mut self,
    add_root_folder_body: AddRootFolderBody,
  ) -> Result<Value> {
    info!("Adding new root folder to Sonarr");
    let event = SonarrEvent::AddRootFolder(add_root_folder_body.clone());

    debug!("Add root folder body: {add_root_folder_body:?}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Post,
        Some(add_root_folder_body),
        None,
        None,
      )
      .await;

    self
      .handle_request::<AddRootFolderBody, Value>(request_props, |_, _| ())
      .await
  }

  async fn add_sonarr_series(&mut self, mut add_series_body: AddSeriesBody) -> Result<Value> {
    info!("Adding new series to Sonarr");
    let event = SonarrEvent::AddSeries(add_series_body.clone());
    if let Some(tag_input_string) = add_series_body.tag_input_string.as_ref() {
      let tag_ids_vec = self
        .extract_and_add_sonarr_tag_ids_vec(tag_input_string.clone())
        .await;
      add_series_body.tags = tag_ids_vec;
    }

    debug!("Add series body: {add_series_body:?}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Post,
        Some(add_series_body),
        None,
        None,
      )
      .await;

    self
      .handle_request::<AddSeriesBody, Value>(request_props, |_, _| ())
      .await
  }

  async fn add_sonarr_tag(&mut self, tag: String) -> Result<Tag> {
    info!("Adding a new Sonarr tag");
    let event = SonarrEvent::AddTag(String::new());

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Post,
        Some(json!({ "label": tag })),
        None,
        None,
      )
      .await;

    self
      .handle_request::<Value, Tag>(request_props, |tag, mut app| {
        app.data.sonarr_data.tags_map.insert(tag.id, tag.label);
      })
      .await
  }

  async fn clear_sonarr_blocklist(&mut self) -> Result<()> {
    info!("Clearing Sonarr blocklist");
    let event = SonarrEvent::ClearBlocklist;

    let ids = self
      .app
      .lock()
      .await
      .data
      .sonarr_data
      .blocklist
      .items
      .iter()
      .map(|item| item.id)
      .collect::<Vec<i64>>();

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Delete,
        Some(json!({"ids": ids})),
        None,
        None,
      )
      .await;

    self
      .handle_request::<Value, ()>(request_props, |_, _| ())
      .await
  }

  async fn delete_sonarr_blocklist_item(&mut self, blocklist_item_id: i64) -> Result<()> {
    let event = SonarrEvent::DeleteBlocklistItem(blocklist_item_id);
    info!("Deleting Sonarr blocklist item for item with id: {blocklist_item_id}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Delete,
        None::<()>,
        Some(format!("/{blocklist_item_id}")),
        None,
      )
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }

  async fn delete_sonarr_episode_file(&mut self, episode_file_id: i64) -> Result<()> {
    let event = SonarrEvent::DeleteEpisodeFile(episode_file_id);
    info!("Deleting Sonarr episode file for episode file with id: {episode_file_id}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Delete,
        None::<()>,
        Some(format!("/{episode_file_id}")),
        None,
      )
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }

  async fn delete_sonarr_download(&mut self, download_id: i64) -> Result<()> {
    let event = SonarrEvent::DeleteDownload(download_id);
    info!("Deleting Sonarr download for download with id: {download_id}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Delete,
        None::<()>,
        Some(format!("/{download_id}")),
        None,
      )
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }

  async fn delete_sonarr_indexer(&mut self, indexer_id: i64) -> Result<()> {
    let event = SonarrEvent::DeleteIndexer(indexer_id);
    info!("Deleting Sonarr indexer for indexer with id: {indexer_id}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Delete,
        None::<()>,
        Some(format!("/{indexer_id}")),
        None,
      )
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }

  async fn delete_sonarr_root_folder(&mut self, root_folder_id: i64) -> Result<()> {
    let event = SonarrEvent::DeleteRootFolder(root_folder_id);
    info!("Deleting Sonarr root folder for folder with id: {root_folder_id}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Delete,
        None::<()>,
        Some(format!("/{root_folder_id}")),
        None,
      )
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }

  async fn delete_series(&mut self, delete_series_params: DeleteSeriesParams) -> Result<()> {
    let event = SonarrEvent::DeleteSeries(delete_series_params.clone());
    let DeleteSeriesParams {
      id,
      delete_series_files,
      add_list_exclusion,
    } = delete_series_params;

    info!("Deleting Sonarr series with ID: {id} with deleteFiles={delete_series_files} and addImportExclusion={add_list_exclusion}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Delete,
        None::<()>,
        Some(format!("/{id}")),
        Some(format!(
          "deleteFiles={delete_series_files}&addImportExclusion={add_list_exclusion}"
        )),
      )
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }

  async fn delete_sonarr_tag(&mut self, id: i64) -> Result<()> {
    info!("Deleting Sonarr tag with id: {id}");
    let event = SonarrEvent::DeleteTag(id);

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Delete,
        None::<()>,
        Some(format!("/{id}")),
        None,
      )
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }

  async fn download_sonarr_release(
    &mut self,
    sonarr_release_download_body: SonarrReleaseDownloadBody,
  ) -> Result<Value> {
    let event = SonarrEvent::DownloadRelease(sonarr_release_download_body.clone());
    info!("Downloading Sonarr release with params: {sonarr_release_download_body:?}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Post,
        Some(sonarr_release_download_body),
        None,
        None,
      )
      .await;

    self
      .handle_request::<SonarrReleaseDownloadBody, Value>(request_props, |_, _| ())
      .await
  }

  async fn edit_all_sonarr_indexer_settings(
    &mut self,
    params: Option<IndexerSettings>,
  ) -> Result<Value> {
    info!("Updating Sonarr indexer settings");
    let event = SonarrEvent::EditAllIndexerSettings(None);

    let body = if let Some(indexer_settings) = params {
      indexer_settings
    } else {
      self
        .app
        .lock()
        .await
        .data
        .sonarr_data
        .indexer_settings
        .as_ref()
        .unwrap()
        .clone()
    };

    debug!("Indexer settings body: {body:?}");

    let request_props = self
      .request_props_from(event, RequestMethod::Put, Some(body), None, None)
      .await;

    let resp = self
      .handle_request::<IndexerSettings, Value>(request_props, |_, _| {})
      .await;

    self.app.lock().await.data.sonarr_data.indexer_settings = None;

    resp
  }

  async fn edit_sonarr_indexer(
    &mut self,
    edit_indexer_params: Option<EditIndexerParams>,
  ) -> Result<()> {
    let detail_event = SonarrEvent::GetIndexers;
    let event = SonarrEvent::EditIndexer(None);
    let id = if let Some(ref params) = edit_indexer_params {
      params.indexer_id
    } else {
      self
        .app
        .lock()
        .await
        .data
        .sonarr_data
        .indexers
        .current_selection()
        .id
    };
    info!("Updating Sonarr indexer with ID: {id}");

    info!("Fetching indexer details for indexer with ID: {id}");

    let request_props = self
      .request_props_from(
        detail_event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{id}")),
        None,
      )
      .await;

    let mut response = String::new();

    self
      .handle_request::<(), Value>(request_props, |detailed_indexer_body, _| {
        response = detailed_indexer_body.to_string()
      })
      .await?;

    info!("Constructing edit indexer body");

    let mut detailed_indexer_body: Value = serde_json::from_str(&response)?;

    let (
      name,
      enable_rss,
      enable_automatic_search,
      enable_interactive_search,
      url,
      api_key,
      seed_ratio,
      tags,
      priority,
    ) = if let Some(params) = edit_indexer_params {
      let priority = detailed_indexer_body["priority"]
        .as_i64()
        .expect("Unable to deserialize 'priority'");
      let seed_ratio_field_option = detailed_indexer_body["fields"]
        .as_array()
        .unwrap()
        .iter()
        .find(|field| field["name"] == "seedCriteria.seedRatio");
      let name = params.name.unwrap_or(
        detailed_indexer_body["name"]
          .as_str()
          .expect("Unable to deserialize 'name'")
          .to_owned(),
      );
      let enable_rss = params.enable_rss.unwrap_or(
        detailed_indexer_body["enableRss"]
          .as_bool()
          .expect("Unable to deserialize 'enableRss'"),
      );
      let enable_automatic_search = params.enable_automatic_search.unwrap_or(
        detailed_indexer_body["enableAutomaticSearch"]
          .as_bool()
          .expect("Unable to deserialize 'enableAutomaticSearch"),
      );
      let enable_interactive_search = params.enable_interactive_search.unwrap_or(
        detailed_indexer_body["enableInteractiveSearch"]
          .as_bool()
          .expect("Unable to deserialize 'enableInteractiveSearch'"),
      );
      let url = params.url.unwrap_or(
        detailed_indexer_body["fields"]
          .as_array()
          .expect("Unable to deserialize 'fields'")
          .iter()
          .find(|field| field["name"] == "baseUrl")
          .expect("Field 'baseUrl' was not found in the 'fields' array")
          .get("value")
          .unwrap_or(&json!(""))
          .as_str()
          .expect("Unable to deserialize 'baseUrl value'")
          .to_owned(),
      );
      let api_key = params.api_key.unwrap_or(
        detailed_indexer_body["fields"]
          .as_array()
          .expect("Unable to deserialize 'fields'")
          .iter()
          .find(|field| field["name"] == "apiKey")
          .expect("Field 'apiKey' was not found in the 'fields' array")
          .get("value")
          .unwrap_or(&json!(""))
          .as_str()
          .expect("Unable to deserialize 'apiKey value'")
          .to_owned(),
      );
      let seed_ratio = params.seed_ratio.unwrap_or_else(|| {
        if let Some(seed_ratio_field) = seed_ratio_field_option {
          return seed_ratio_field
            .get("value")
            .unwrap_or(&json!(""))
            .as_str()
            .expect("Unable to deserialize 'seedCriteria.seedRatio value'")
            .to_owned();
        }

        String::new()
      });
      let tags = if params.clear_tags {
        vec![]
      } else {
        params.tags.unwrap_or(
          detailed_indexer_body["tags"]
            .as_array()
            .expect("Unable to deserialize 'tags'")
            .iter()
            .map(|item| item.as_i64().expect("Unable to deserialize tag ID"))
            .collect(),
        )
      };
      let priority = params.priority.unwrap_or(priority);

      (
        name,
        enable_rss,
        enable_automatic_search,
        enable_interactive_search,
        url,
        api_key,
        seed_ratio,
        tags,
        priority,
      )
    } else {
      let tags = self
        .app
        .lock()
        .await
        .data
        .sonarr_data
        .edit_indexer_modal
        .as_ref()
        .unwrap()
        .tags
        .text
        .clone();
      let tag_ids_vec = self.extract_and_add_sonarr_tag_ids_vec(tags).await;
      let mut app = self.app.lock().await;

      let params = {
        let EditIndexerModal {
          name,
          enable_rss,
          enable_automatic_search,
          enable_interactive_search,
          url,
          api_key,
          seed_ratio,
          priority,
          ..
        } = app.data.sonarr_data.edit_indexer_modal.as_ref().unwrap();

        (
          name.text.clone(),
          enable_rss.unwrap_or_default(),
          enable_automatic_search.unwrap_or_default(),
          enable_interactive_search.unwrap_or_default(),
          url.text.clone(),
          api_key.text.clone(),
          seed_ratio.text.clone(),
          tag_ids_vec,
          *priority,
        )
      };

      app.data.sonarr_data.edit_indexer_modal = None;

      params
    };

    *detailed_indexer_body.get_mut("name").unwrap() = json!(name);
    *detailed_indexer_body.get_mut("priority").unwrap() = json!(priority);
    *detailed_indexer_body.get_mut("enableRss").unwrap() = json!(enable_rss);
    *detailed_indexer_body
      .get_mut("enableAutomaticSearch")
      .unwrap() = json!(enable_automatic_search);
    *detailed_indexer_body
      .get_mut("enableInteractiveSearch")
      .unwrap() = json!(enable_interactive_search);
    *detailed_indexer_body
      .get_mut("fields")
      .unwrap()
      .as_array_mut()
      .unwrap()
      .iter_mut()
      .find(|field| field["name"] == "baseUrl")
      .unwrap()
      .get_mut("value")
      .unwrap() = json!(url);
    *detailed_indexer_body
      .get_mut("fields")
      .unwrap()
      .as_array_mut()
      .unwrap()
      .iter_mut()
      .find(|field| field["name"] == "apiKey")
      .unwrap()
      .get_mut("value")
      .unwrap() = json!(api_key);
    *detailed_indexer_body.get_mut("tags").unwrap() = json!(tags);
    let seed_ratio_field_option = detailed_indexer_body
      .get_mut("fields")
      .unwrap()
      .as_array_mut()
      .unwrap()
      .iter_mut()
      .find(|field| field["name"] == "seedCriteria.seedRatio");
    if let Some(seed_ratio_field) = seed_ratio_field_option {
      seed_ratio_field
        .as_object_mut()
        .unwrap()
        .insert("value".to_string(), json!(seed_ratio));
    }

    debug!("Edit indexer body: {detailed_indexer_body:?}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Put,
        Some(detailed_indexer_body),
        Some(format!("/{id}")),
        Some("forceSave=true".to_owned()),
      )
      .await;

    self
      .handle_request::<Value, ()>(request_props, |_, _| ())
      .await
  }

  async fn edit_sonarr_series(
    &mut self,
    edit_series_params: Option<EditSeriesParams>,
  ) -> Result<()> {
    info!("Editing Sonarr series");
    let detail_event = SonarrEvent::GetSeriesDetails(None);
    let event = SonarrEvent::EditSeries(None);

    let (series_id, _) = if let Some(ref params) = edit_series_params {
      self.extract_series_id(Some(params.series_id)).await
    } else {
      self.extract_series_id(None).await
    };
    info!("Fetching series details for series with ID: {series_id}");

    let request_props = self
      .request_props_from(
        detail_event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{series_id}")),
        None,
      )
      .await;

    let mut response = String::new();

    self
      .handle_request::<(), Value>(request_props, |detailed_series_body, _| {
        response = detailed_series_body.to_string()
      })
      .await?;

    info!("Constructing edit series body");

    let mut detailed_series_body: Value = serde_json::from_str(&response)?;
    let (
      monitored,
      use_season_folders,
      series_type,
      quality_profile_id,
      language_profile_id,
      root_folder_path,
      tags,
    ) = if let Some(params) = edit_series_params {
      let monitored = params.monitored.unwrap_or(
        detailed_series_body["monitored"]
          .as_bool()
          .expect("Unable to deserialize 'monitored'"),
      );
      let use_season_folders = params.use_season_folders.unwrap_or(
        detailed_series_body["seasonFolder"]
          .as_bool()
          .expect("Unable to deserialize 'season_folder'"),
      );
      let series_type = params
        .series_type
        .unwrap_or_else(|| {
          serde_json::from_value(detailed_series_body["seriesType"].clone())
            .expect("Unable to deserialize 'seriesType'")
        })
        .to_string();
      let quality_profile_id = params.quality_profile_id.unwrap_or_else(|| {
        detailed_series_body["qualityProfileId"]
          .as_i64()
          .expect("Unable to deserialize 'qualityProfileId'")
      });
      let language_profile_id = params.language_profile_id.unwrap_or_else(|| {
        detailed_series_body["languageProfileId"]
          .as_i64()
          .expect("Unable to deserialize 'languageProfileId'")
      });
      let root_folder_path = params.root_folder_path.unwrap_or_else(|| {
        detailed_series_body["path"]
          .as_str()
          .expect("Unable to deserialize 'path'")
          .to_owned()
      });
      let tags = if params.clear_tags {
        vec![]
      } else {
        params.tags.unwrap_or(
          detailed_series_body["tags"]
            .as_array()
            .expect("Unable to deserialize 'tags'")
            .iter()
            .map(|item| item.as_i64().expect("Unable to deserialize tag ID"))
            .collect(),
        )
      };

      (
        monitored,
        use_season_folders,
        series_type,
        quality_profile_id,
        language_profile_id,
        root_folder_path,
        tags,
      )
    } else {
      let tags = self
        .app
        .lock()
        .await
        .data
        .sonarr_data
        .edit_series_modal
        .as_ref()
        .unwrap()
        .tags
        .text
        .clone();
      let tag_ids_vec = self.extract_and_add_sonarr_tag_ids_vec(tags).await;
      let mut app = self.app.lock().await;

      let params = {
        let EditSeriesModal {
          monitored,
          use_season_folders,
          path,
          series_type_list,
          quality_profile_list,
          language_profile_list,
          ..
        } = app.data.sonarr_data.edit_series_modal.as_ref().unwrap();
        let quality_profile = quality_profile_list.current_selection();
        let quality_profile_id = *app
          .data
          .sonarr_data
          .quality_profile_map
          .iter()
          .filter(|(_, value)| *value == quality_profile)
          .map(|(key, _)| key)
          .next()
          .unwrap();
        let language_profile = language_profile_list.current_selection();
        let language_profile_id = *app
          .data
          .sonarr_data
          .language_profiles_map
          .iter()
          .filter(|(_, value)| *value == language_profile)
          .map(|(key, _)| key)
          .next()
          .unwrap();

        (
          monitored.unwrap_or_default(),
          use_season_folders.unwrap_or_default(),
          series_type_list.current_selection().to_string(),
          quality_profile_id,
          language_profile_id,
          path.text.clone(),
          tag_ids_vec,
        )
      };

      app.data.sonarr_data.edit_series_modal = None;

      params
    };

    *detailed_series_body.get_mut("monitored").unwrap() = json!(monitored);
    *detailed_series_body.get_mut("seasonFolder").unwrap() = json!(use_season_folders);
    *detailed_series_body.get_mut("seriesType").unwrap() = json!(series_type);
    *detailed_series_body.get_mut("qualityProfileId").unwrap() = json!(quality_profile_id);
    *detailed_series_body.get_mut("languageProfileId").unwrap() = json!(language_profile_id);
    *detailed_series_body.get_mut("path").unwrap() = json!(root_folder_path);
    *detailed_series_body.get_mut("tags").unwrap() = json!(tags);

    debug!("Edit series body: {detailed_series_body:?}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Put,
        Some(detailed_series_body),
        Some(format!("/{series_id}")),
        None,
      )
      .await;

    self
      .handle_request::<Value, ()>(request_props, |_, _| ())
      .await
  }

  async fn toggle_sonarr_season_monitoring(
    &mut self,
    series_id_season_number_tuple: Option<(i64, i64)>,
  ) -> Result<()> {
    let detail_event = SonarrEvent::GetSeriesDetails(None);
    let event = SonarrEvent::ToggleSeasonMonitoring(series_id_season_number_tuple);
    let (series_id, season_number) =
      if let Some((series_id, season_number)) = series_id_season_number_tuple {
        (Some(series_id), Some(season_number))
      } else {
        (None, None)
      };

    let (series_id, _) = self.extract_series_id(series_id).await;
    if let Ok((season_number, _)) = self.extract_season_number(season_number).await {
      info!("Toggling season monitoring for season {season_number} in series with ID: {series_id}");
      info!("Fetching series details for series with ID: {series_id}");

      let request_props = self
        .request_props_from(
          detail_event,
          RequestMethod::Get,
          None::<()>,
          Some(format!("/{series_id}")),
          None,
        )
        .await;

      let mut response = String::new();

      self
        .handle_request::<(), Value>(request_props, |detailed_series_body, _| {
          response = detailed_series_body.to_string()
        })
        .await?;

      info!("Constructing toggle season monitoring body");

      let mut detailed_series_body: Value =
        serde_json::from_str(&response).expect("Request for detailed series body was interrupted");
      let monitored = detailed_series_body
        .get("seasons")
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .find(|season| season["seasonNumber"] == season_number)
        .unwrap()
        .get("monitored")
        .unwrap()
        .as_bool()
        .unwrap();

      *detailed_series_body
        .get_mut("seasons")
        .unwrap()
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|season| season["seasonNumber"] == season_number)
        .unwrap()
        .get_mut("monitored")
        .unwrap() = json!(!monitored);

      debug!("Toggle season monitoring body: {detailed_series_body:?}");

      let request_props = self
        .request_props_from(
          event,
          RequestMethod::Put,
          Some(detailed_series_body),
          Some(format!("/{series_id}")),
          None,
        )
        .await;

      self
        .handle_request::<Value, ()>(request_props, |_, _| ())
        .await
    } else {
      warn!("Season number was not provided. Aborting...");
      Ok(())
    }
  }

  async fn get_all_sonarr_indexer_settings(&mut self) -> Result<IndexerSettings> {
    info!("Fetching Sonarr indexer settings");
    let event = SonarrEvent::GetAllIndexerSettings;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), IndexerSettings>(request_props, |indexer_settings, mut app| {
        if app.data.sonarr_data.indexer_settings.is_none() {
          app.data.sonarr_data.indexer_settings = Some(indexer_settings);
        } else {
          debug!("Indexer Settings are being modified. Ignoring update...");
        }
      })
      .await
  }

  async fn get_sonarr_healthcheck(&mut self) -> Result<()> {
    info!("Performing Sonarr health check");
    let event = SonarrEvent::HealthCheck;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }

  async fn get_sonarr_blocklist(&mut self) -> Result<BlocklistResponse> {
    info!("Fetching Sonarr blocklist");
    let event = SonarrEvent::GetBlocklist;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), BlocklistResponse>(request_props, |blocklist_resp, mut app| {
        if !matches!(
          app.get_current_route(),
          Route::Sonarr(ActiveSonarrBlock::BlocklistSortPrompt, _)
        ) {
          let mut blocklist_vec: Vec<BlocklistItem> = blocklist_resp
            .records
            .into_iter()
            .map(|item| {
              if let Some(series) = app
                .data
                .sonarr_data
                .series
                .items
                .iter()
                .find(|it| it.id == item.series_id)
              {
                BlocklistItem {
                  series_title: Some(series.title.text.clone()),
                  ..item
                }
              } else {
                item
              }
            })
            .collect();
          blocklist_vec.sort_by(|a, b| a.id.cmp(&b.id));
          app.data.sonarr_data.blocklist.set_items(blocklist_vec);
          app.data.sonarr_data.blocklist.apply_sorting_toggle(false);
        }
      })
      .await
  }

  async fn get_sonarr_downloads(&mut self) -> Result<DownloadsResponse> {
    info!("Fetching Sonarr downloads");
    let event = SonarrEvent::GetDownloads;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), DownloadsResponse>(request_props, |queue_response, mut app| {
        app
          .data
          .sonarr_data
          .downloads
          .set_items(queue_response.records);
      })
      .await
  }

  async fn get_episodes(&mut self, series_id: Option<i64>) -> Result<Vec<Episode>> {
    let event = SonarrEvent::GetEpisodes(series_id);
    let (id, series_id_param) = self.extract_series_id(series_id).await;
    info!("Fetching episodes for Sonarr series with ID: {id}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(series_id_param),
      )
      .await;

    self
      .handle_request::<(), Vec<Episode>>(request_props, |mut episode_vec, mut app| {
        episode_vec.sort_by(|a, b| a.id.cmp(&b.id));
        if !matches!(
          app.get_current_route(),
          Route::Sonarr(ActiveSonarrBlock::EpisodesSortPrompt, _)
        ) {
          if app.data.sonarr_data.season_details_modal.is_none() {
            app.data.sonarr_data.season_details_modal = Some(SeasonDetailsModal::default());
          }

          let season_episodes_vec = if !app.data.sonarr_data.seasons.is_empty() {
            let season_number = app
              .data
              .sonarr_data
              .seasons
              .current_selection()
              .season_number;

            episode_vec
              .into_iter()
              .filter(|episode| episode.season_number == season_number)
              .collect()
          } else {
            episode_vec
          };

          app
            .data
            .sonarr_data
            .season_details_modal
            .as_mut()
            .unwrap()
            .episodes
            .set_items(season_episodes_vec);
          app
            .data
            .sonarr_data
            .season_details_modal
            .as_mut()
            .unwrap()
            .episodes
            .apply_sorting_toggle(false);
        }
      })
      .await
  }

  async fn get_episode_files(&mut self, series_id: Option<i64>) -> Result<Vec<EpisodeFile>> {
    let event = SonarrEvent::GetEpisodeFiles(series_id);
    let (id, series_id_param) = self.extract_series_id(series_id).await;
    info!("Fetching episodes files for Sonarr series with ID: {id}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(series_id_param),
      )
      .await;

    self
      .handle_request::<(), Vec<EpisodeFile>>(request_props, |episode_file_vec, mut app| {
        if app.data.sonarr_data.season_details_modal.is_none() {
          app.data.sonarr_data.season_details_modal = Some(SeasonDetailsModal::default());
        }

        app
          .data
          .sonarr_data
          .season_details_modal
          .as_mut()
          .unwrap()
          .episode_files
          .set_items(episode_file_vec);
      })
      .await
  }

  async fn get_sonarr_episode_history(
    &mut self,
    episode_id: Option<i64>,
  ) -> Result<SonarrHistoryWrapper> {
    let id = self.extract_episode_id(episode_id).await;
    info!("Fetching Sonarr history for episode with ID: {id}");
    let event = SonarrEvent::GetEpisodeHistory(episode_id);

    let params = format!("episodeId={id}&pageSize=1000&sortDirection=descending&sortKey=date",);
    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, Some(params))
      .await;

    self
      .handle_request::<(), SonarrHistoryWrapper>(request_props, |history_response, mut app| {
        if app.data.sonarr_data.season_details_modal.is_none() {
          app.data.sonarr_data.season_details_modal = Some(SeasonDetailsModal::default());
        }

        if app
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .episode_details_modal
          .is_none()
        {
          app
            .data
            .sonarr_data
            .season_details_modal
            .as_mut()
            .unwrap()
            .episode_details_modal = Some(EpisodeDetailsModal::default());
        }

        let mut history_vec = history_response.records;
        history_vec.sort_by(|a, b| a.id.cmp(&b.id));
        app
          .data
          .sonarr_data
          .season_details_modal
          .as_mut()
          .unwrap()
          .episode_details_modal
          .as_mut()
          .unwrap()
          .episode_history
          .set_items(history_vec);
        app
          .data
          .sonarr_data
          .season_details_modal
          .as_mut()
          .unwrap()
          .episode_details_modal
          .as_mut()
          .unwrap()
          .episode_history
          .apply_sorting_toggle(false);
      })
      .await
  }

  async fn get_episode_details(&mut self, episode_id: Option<i64>) -> Result<Episode> {
    info!("Fetching Sonarr episode details");
    let event = SonarrEvent::GetEpisodeDetails(None);
    let id = self.extract_episode_id(episode_id).await;

    info!("Fetching episode details for episode with ID: {id}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{id}")),
        None,
      )
      .await;

    self
      .handle_request::<(), Episode>(request_props, |episode_response, mut app| {
        if app.cli_mode {
          app.data.sonarr_data.season_details_modal = Some(SeasonDetailsModal::default());
        }

        if app
          .data
          .sonarr_data
          .season_details_modal
          .as_mut()
          .expect("Season details modal is empty")
          .episode_details_modal
          .is_none()
        {
          app
            .data
            .sonarr_data
            .season_details_modal
            .as_mut()
            .unwrap()
            .episode_details_modal = Some(EpisodeDetailsModal::default());
        }

        let Episode {
          id,
          title,
          air_date_utc,
          overview,
          has_file,
          season_number,
          episode_number,
          episode_file,
          ..
        } = episode_response;
        let status = get_episode_status(has_file, &app.data.sonarr_data.downloads.items, id);
        let air_date = if let Some(air_date) = air_date_utc {
          format!("{air_date}")
        } else {
          String::new()
        };
        let episode_details_modal = app
          .data
          .sonarr_data
          .season_details_modal
          .as_mut()
          .unwrap()
          .episode_details_modal
          .as_mut()
          .unwrap();
        episode_details_modal.episode_details = ScrollableText::with_string(formatdoc!(
          "
            Title: {}
            Season: {season_number}
            Episode Number: {episode_number}
            Air Date: {air_date}
            Status: {status}
            Description: {}",
          title,
          overview.unwrap_or_default(),
        ));
        if let Some(file) = episode_file {
          let size = convert_to_gb(file.size);
          episode_details_modal.file_details = formatdoc!(
            "
            Relative Path: {}
            Absolute Path: {}
            Size: {size:.2} GB
            Language: {}
            Date Added: {}",
            file.relative_path,
            file.path,
            file.languages.first().unwrap_or(&Language::default()).name,
            file.date_added,
          );

          if let Some(media_info) = file.media_info {
            episode_details_modal.audio_details = formatdoc!(
              "
              Bitrate: {}
              Channels: {:.1}
              Codec: {}
              Languages: {}
              Stream Count: {}",
              media_info.audio_bitrate,
              media_info.audio_channels.as_f64().unwrap(),
              media_info.audio_codec.unwrap_or_default(),
              media_info.audio_languages.unwrap_or_default(),
              media_info.audio_stream_count
            );

            episode_details_modal.video_details = formatdoc!(
              "
              Bit Depth: {}
              Bitrate: {}
              Codec: {}
              FPS: {}
              Resolution: {}
              Scan Type: {}
              Runtime: {}
              Subtitles: {}",
              media_info.video_bit_depth,
              media_info.video_bitrate,
              media_info.video_codec,
              media_info.video_fps.as_f64().unwrap(),
              media_info.resolution,
              media_info.scan_type,
              media_info.run_time,
              media_info.subtitles.unwrap_or_default()
            );
          }
        };
      })
      .await
  }

  async fn get_sonarr_host_config(&mut self) -> Result<HostConfig> {
    info!("Fetching Sonarr host config");
    let event = SonarrEvent::GetHostConfig;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), HostConfig>(request_props, |_, _| ())
      .await
  }

  async fn get_sonarr_history(&mut self, events: Option<u64>) -> Result<SonarrHistoryWrapper> {
    info!("Fetching all Sonarr history events");
    let event = SonarrEvent::GetHistory(events);

    let params = format!(
      "pageSize={}&sortDirection=descending&sortKey=date",
      events.unwrap_or(500)
    );
    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, Some(params))
      .await;

    self
      .handle_request::<(), SonarrHistoryWrapper>(request_props, |history_response, mut app| {
        if !matches!(
          app.get_current_route(),
          Route::Sonarr(ActiveSonarrBlock::HistorySortPrompt, _)
        ) {
          let mut history_vec = history_response.records;
          history_vec.sort_by(|a, b| a.id.cmp(&b.id));
          app.data.sonarr_data.history.set_items(history_vec);
          app.data.sonarr_data.history.apply_sorting_toggle(false);
        }
      })
      .await
  }

  async fn get_sonarr_indexers(&mut self) -> Result<Vec<Indexer>> {
    info!("Fetching Sonarr indexers");
    let event = SonarrEvent::GetIndexers;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<Indexer>>(request_props, |indexers, mut app| {
        app.data.sonarr_data.indexers.set_items(indexers);
      })
      .await
  }

  async fn get_sonarr_language_profiles(&mut self) -> Result<Vec<Language>> {
    info!("Fetching Sonarr language profiles");
    let event = SonarrEvent::GetLanguageProfiles;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<Language>>(request_props, |language_profiles_vec, mut app| {
        app.data.sonarr_data.language_profiles_map = language_profiles_vec
          .into_iter()
          .map(|language| (language.id, language.name))
          .collect();
      })
      .await
  }

  async fn get_sonarr_logs(&mut self, events: Option<u64>) -> Result<LogResponse> {
    info!("Fetching Sonarr logs");
    let event = SonarrEvent::GetLogs(events);

    let params = format!(
      "pageSize={}&sortDirection=descending&sortKey=time",
      events.unwrap_or(500)
    );
    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, Some(params))
      .await;

    self
      .handle_request::<(), LogResponse>(request_props, |log_response, mut app| {
        let mut logs = log_response.records;
        logs.reverse();

        let log_lines = logs
          .into_iter()
          .map(|log| {
            if log.exception.is_some() {
              HorizontallyScrollableText::from(format!(
                "{}|{}|{}|{}|{}",
                log.time,
                log.level.to_uppercase(),
                log.logger.as_ref().unwrap(),
                log.exception_type.as_ref().unwrap(),
                log.exception.as_ref().unwrap()
              ))
            } else {
              HorizontallyScrollableText::from(format!(
                "{}|{}|{}|{}",
                log.time,
                log.level.to_uppercase(),
                log.logger.as_ref().unwrap(),
                log.message.as_ref().unwrap()
              ))
            }
          })
          .collect();

        app.data.sonarr_data.logs.set_items(log_lines);
        app.data.sonarr_data.logs.scroll_to_bottom();
      })
      .await
  }

  async fn get_sonarr_diskspace(&mut self) -> Result<Vec<DiskSpace>> {
    info!("Fetching Sonarr disk space");
    let event = SonarrEvent::GetDiskSpace;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<DiskSpace>>(request_props, |disk_space_vec, mut app| {
        app.data.sonarr_data.disk_space_vec = disk_space_vec;
      })
      .await
  }

  async fn get_sonarr_quality_profiles(&mut self) -> Result<Vec<QualityProfile>> {
    info!("Fetching Sonarr quality profiles");
    let event = SonarrEvent::GetQualityProfiles;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<QualityProfile>>(request_props, |quality_profiles, mut app| {
        app.data.sonarr_data.quality_profile_map = quality_profiles
          .into_iter()
          .map(|profile| (profile.id, profile.name))
          .collect();
      })
      .await
  }

  async fn get_queued_sonarr_events(&mut self) -> Result<Vec<QueueEvent>> {
    info!("Fetching Sonarr queued events");
    let event = SonarrEvent::GetQueuedEvents;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<QueueEvent>>(request_props, |queued_events_vec, mut app| {
        app
          .data
          .sonarr_data
          .queued_events
          .set_items(queued_events_vec);
      })
      .await
  }

  async fn get_sonarr_root_folders(&mut self) -> Result<Vec<RootFolder>> {
    info!("Fetching Sonarr root folders");
    let event = SonarrEvent::GetRootFolders;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<RootFolder>>(request_props, |root_folders, mut app| {
        app.data.sonarr_data.root_folders.set_items(root_folders);
      })
      .await
  }

  async fn get_episode_releases(&mut self, episode_id: Option<i64>) -> Result<Vec<SonarrRelease>> {
    let event = SonarrEvent::GetEpisodeReleases(None);
    let id = self.extract_episode_id(episode_id).await;

    info!("Fetching releases for episode with ID: {id}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("episodeId={id}")),
      )
      .await;

    self
      .handle_request::<(), Vec<SonarrRelease>>(request_props, |release_vec, mut app| {
        if app.data.sonarr_data.season_details_modal.is_none() {
          app.data.sonarr_data.season_details_modal = Some(SeasonDetailsModal::default());
        }

        if app
          .data
          .sonarr_data
          .season_details_modal
          .as_mut()
          .unwrap()
          .episode_details_modal
          .is_none()
        {
          app
            .data
            .sonarr_data
            .season_details_modal
            .as_mut()
            .unwrap()
            .episode_details_modal = Some(EpisodeDetailsModal::default());
        }

        app
          .data
          .sonarr_data
          .season_details_modal
          .as_mut()
          .unwrap()
          .episode_details_modal
          .as_mut()
          .unwrap()
          .episode_releases
          .set_items(release_vec);
      })
      .await
  }

  async fn get_season_releases(
    &mut self,
    series_season_id_tuple: Option<(i64, i64)>,
  ) -> Result<Vec<SonarrRelease>> {
    let event = SonarrEvent::GetSeasonReleases(None);
    let (series_id, season_number) =
      if let Some((series_id, season_number)) = series_season_id_tuple {
        (Some(series_id), Some(season_number))
      } else {
        (None, None)
      };

    let (series_id, series_id_param) = self.extract_series_id(series_id).await;
    let (season_number, season_number_param) = self.extract_season_number(season_number).await?;

    info!("Fetching releases for series with ID: {series_id} and season number: {season_number}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("{}&{}", series_id_param, season_number_param)),
      )
      .await;

    self
      .handle_request::<(), Vec<SonarrRelease>>(request_props, |release_vec, mut app| {
        if app.data.sonarr_data.season_details_modal.is_none() {
          app.data.sonarr_data.season_details_modal = Some(SeasonDetailsModal::default());
        }

        let season_releases_vec = release_vec
          .into_iter()
          .filter(|release| release.full_season)
          .collect();

        app
          .data
          .sonarr_data
          .season_details_modal
          .as_mut()
          .unwrap()
          .season_releases
          .set_items(season_releases_vec);
      })
      .await
  }

  async fn get_sonarr_season_history(
    &mut self,
    series_season_id_tuple: Option<(i64, i64)>,
  ) -> Result<Vec<SonarrHistoryItem>> {
    let event = SonarrEvent::GetSeasonHistory(None);
    let (series_id, season_number) =
      if let Some((series_id, season_number)) = series_season_id_tuple {
        (Some(series_id), Some(season_number))
      } else {
        (None, None)
      };

    let (series_id, series_id_param) = self.extract_series_id(series_id).await;
    let (season_number, season_number_param) = self.extract_season_number(season_number).await?;

    info!("Fetching history for series with ID: {series_id} and season number: {season_number}");

    let params = format!("{series_id_param}&{season_number_param}",);
    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, Some(params))
      .await;

    self
      .handle_request::<(), Vec<SonarrHistoryItem>>(request_props, |history_items, mut app| {
        if app.data.sonarr_data.season_details_modal.is_none() {
          app.data.sonarr_data.season_details_modal = Some(SeasonDetailsModal::default());
        }

        let mut history_vec = history_items;
        history_vec.sort_by(|a, b| a.id.cmp(&b.id));
        app
          .data
          .sonarr_data
          .season_details_modal
          .as_mut()
          .unwrap()
          .season_history
          .set_items(history_vec);
        app
          .data
          .sonarr_data
          .season_details_modal
          .as_mut()
          .unwrap()
          .season_history
          .apply_sorting_toggle(false);
      })
      .await
  }

  async fn get_sonarr_security_config(&mut self) -> Result<SecurityConfig> {
    info!("Fetching Sonarr security config");
    let event = SonarrEvent::GetSecurityConfig;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), SecurityConfig>(request_props, |_, _| ())
      .await
  }

  async fn get_series_details(&mut self, series_id: Option<i64>) -> Result<Series> {
    let (id, _) = self.extract_series_id(series_id).await;
    info!("Fetching details for Sonarr series with ID: {id}");
    let event = SonarrEvent::GetSeriesDetails(series_id);

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{id}")),
        None,
      )
      .await;

    self
      .handle_request::<(), Series>(request_props, |_, _| ())
      .await
  }

  async fn get_sonarr_series_history(
    &mut self,
    series_id: Option<i64>,
  ) -> Result<Vec<SonarrHistoryItem>> {
    let (id, series_id_param) = self.extract_series_id(series_id).await;
    info!("Fetching Sonarr series history for series with ID: {id}");
    let event = SonarrEvent::GetSeriesHistory(series_id);

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(series_id_param),
      )
      .await;

    self
      .handle_request::<(), Vec<SonarrHistoryItem>>(request_props, |mut history_vec, mut app| {
        if app.data.sonarr_data.series_history.is_none() {
          app.data.sonarr_data.series_history = Some(StatefulTable::default());
        }

        if !matches!(
          app.get_current_route(),
          Route::Sonarr(ActiveSonarrBlock::SeriesHistorySortPrompt, _)
        ) {
          history_vec.sort_by(|a, b| a.id.cmp(&b.id));
          app
            .data
            .sonarr_data
            .series_history
            .as_mut()
            .unwrap()
            .set_items(history_vec);
          app
            .data
            .sonarr_data
            .series_history
            .as_mut()
            .unwrap()
            .apply_sorting_toggle(false);
        }
      })
      .await
  }

  async fn list_series(&mut self) -> Result<Vec<Series>> {
    info!("Fetching Sonarr library");
    let event = SonarrEvent::ListSeries;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<Series>>(request_props, |mut series_vec, mut app| {
        if !matches!(
          app.get_current_route(),
          Route::Sonarr(ActiveSonarrBlock::SeriesSortPrompt, _)
        ) {
          series_vec.sort_by(|a, b| a.id.cmp(&b.id));
          app.data.sonarr_data.series.set_items(series_vec);
          app.data.sonarr_data.series.apply_sorting_toggle(false);
        }
      })
      .await
  }

  async fn get_sonarr_status(&mut self) -> Result<SystemStatus> {
    info!("Fetching Sonarr system status");
    let event = SonarrEvent::GetStatus;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), SystemStatus>(request_props, |system_status, mut app| {
        app.data.sonarr_data.version = system_status.version;
        app.data.sonarr_data.start_time = system_status.start_time;
      })
      .await
  }

  async fn get_sonarr_tags(&mut self) -> Result<Vec<Tag>> {
    info!("Fetching Sonarr tags");
    let event = SonarrEvent::GetTags;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<Tag>>(request_props, |tags_vec, mut app| {
        app.data.sonarr_data.tags_map = tags_vec
          .into_iter()
          .map(|tag| (tag.id, tag.label))
          .collect();
      })
      .await
  }

  async fn get_sonarr_tasks(&mut self) -> Result<Vec<SonarrTask>> {
    info!("Fetching Sonarr tasks");
    let event = SonarrEvent::GetTasks;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<SonarrTask>>(request_props, |tasks_vec, mut app| {
        app.data.sonarr_data.tasks.set_items(tasks_vec);
      })
      .await
  }

  async fn get_sonarr_updates(&mut self) -> Result<Vec<Update>> {
    info!("Fetching Sonarr updates");
    let event = SonarrEvent::GetUpdates;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<Update>>(request_props, |updates_vec, mut app| {
        let latest_installed = if updates_vec
          .iter()
          .any(|update| update.latest && update.installed_on.is_some())
        {
          "already".to_owned()
        } else {
          "not".to_owned()
        };
        let updates = updates_vec
          .into_iter()
          .map(|update| {
            let install_status = if update.installed_on.is_some() {
              if update.installed {
                "(Currently Installed)".to_owned()
              } else {
                "(Previously Installed)".to_owned()
              }
            } else {
              String::new()
            };
            let vec_to_bullet_points = |vec: Vec<String>| {
              vec
                .iter()
                .map(|change| format!("  * {change}"))
                .collect::<Vec<String>>()
                .join("\n")
            };

            let mut update_info = formatdoc!(
              "{} - {} {install_status}
              {}",
              update.version,
              update.release_date,
              "-".repeat(200)
            );

            if let Some(new_changes) = update.changes.new {
              let changes = vec_to_bullet_points(new_changes);
              update_info = formatdoc!(
                "{update_info}
              New:
              {changes}"
              )
            }

            if let Some(fixes) = update.changes.fixed {
              let fixes = vec_to_bullet_points(fixes);
              update_info = formatdoc!(
                "{update_info}
              Fixed:
              {fixes}"
              );
            }

            update_info
          })
          .reduce(|version_1, version_2| format!("{version_1}\n\n\n{version_2}"))
          .unwrap();

        app.data.sonarr_data.updates = ScrollableText::with_string(formatdoc!(
          "The latest version of Sonarr is {latest_installed} installed
          
          {updates}"
        ));
      })
      .await
  }

  async fn mark_sonarr_history_item_as_failed(&mut self, history_item_id: i64) -> Result<Value> {
    info!("Marking the Sonarr history item with ID: {history_item_id} as 'failed'");
    let event = SonarrEvent::MarkHistoryItemAsFailed(history_item_id);

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Post,
        None,
        Some(format!("/{history_item_id}")),
        None,
      )
      .await;

    self
      .handle_request::<(), Value>(request_props, |_, _| ())
      .await
  }

  async fn search_sonarr_series(
    &mut self,
    query: Option<String>,
  ) -> Result<Vec<AddSeriesSearchResult>> {
    info!("Searching for specific Sonarr series");
    let event = SonarrEvent::SearchNewSeries(None);
    let search = if let Some(search_query) = query {
      Ok(search_query.into())
    } else {
      self
        .app
        .lock()
        .await
        .data
        .sonarr_data
        .add_series_search
        .clone()
        .ok_or(anyhow!("Encountered a race condition"))
    };

    match search {
      Ok(search_string) => {
        let request_props = self
          .request_props_from(
            event,
            RequestMethod::Get,
            None::<()>,
            None,
            Some(format!("term={}", encode(&search_string.text))),
          )
          .await;

        self
          .handle_request::<(), Vec<AddSeriesSearchResult>>(request_props, |series_vec, mut app| {
            if series_vec.is_empty() {
              app.pop_and_push_navigation_stack(
                ActiveSonarrBlock::AddSeriesEmptySearchResults.into(),
              );
            } else if let Some(add_searched_seriess) =
              app.data.sonarr_data.add_searched_series.as_mut()
            {
              add_searched_seriess.set_items(series_vec);
            } else {
              let mut add_searched_seriess = StatefulTable::default();
              add_searched_seriess.set_items(series_vec);
              app.data.sonarr_data.add_searched_series = Some(add_searched_seriess);
            }
          })
          .await
      }
      Err(e) => {
        warn!(
          "Encountered a race condition: {e}\n \
          This is most likely caused by the user trying to navigate between modals rapidly. \
          Ignoring search request."
        );
        Ok(Vec::default())
      }
    }
  }

  async fn start_sonarr_task(&mut self, task: Option<SonarrTaskName>) -> Result<Value> {
    let event = SonarrEvent::StartTask(None);
    let task_name = if let Some(t_name) = task {
      t_name
    } else {
      self
        .app
        .lock()
        .await
        .data
        .sonarr_data
        .tasks
        .current_selection()
        .task_name
    }
    .to_string();

    info!("Starting Sonarr task: {task_name}");

    let body = CommandBody { name: task_name };

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(body), None, None)
      .await;

    self
      .handle_request::<CommandBody, Value>(request_props, |_, _| ())
      .await
  }

  async fn test_sonarr_indexer(&mut self, indexer_id: Option<i64>) -> Result<Value> {
    let detail_event = SonarrEvent::GetIndexers;
    let event = SonarrEvent::TestIndexer(None);
    let id = if let Some(i_id) = indexer_id {
      i_id
    } else {
      self
        .app
        .lock()
        .await
        .data
        .sonarr_data
        .indexers
        .current_selection()
        .id
    };
    info!("Testing Sonarr indexer with ID: {id}");

    info!("Fetching indexer details for indexer with ID: {id}");

    let request_props = self
      .request_props_from(
        detail_event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{id}")),
        None,
      )
      .await;

    let mut test_body: Value = Value::default();

    self
      .handle_request::<(), Value>(request_props, |detailed_indexer_body, _| {
        test_body = detailed_indexer_body;
      })
      .await?;

    info!("Testing indexer");

    let mut request_props = self
      .request_props_from(event, RequestMethod::Post, Some(test_body), None, None)
      .await;
    request_props.ignore_status_code = true;

    self
      .handle_request::<Value, Value>(request_props, |test_results, mut app| {
        if test_results.as_object().is_none() {
          app.data.sonarr_data.indexer_test_errors = Some(
            test_results.as_array().unwrap()[0]
              .get("errorMessage")
              .unwrap()
              .to_string(),
          );
        } else {
          app.data.sonarr_data.indexer_test_errors = Some(String::new());
        };
      })
      .await
  }

  async fn test_all_sonarr_indexers(&mut self) -> Result<Vec<IndexerTestResult>> {
    info!("Testing all Sonarr indexers");
    let event = SonarrEvent::TestAllIndexers;

    let mut request_props = self
      .request_props_from(event, RequestMethod::Post, None, None, None)
      .await;
    request_props.ignore_status_code = true;

    self
      .handle_request::<(), Vec<IndexerTestResult>>(request_props, |test_results, mut app| {
        let mut test_all_indexer_results = StatefulTable::default();
        let indexers = app.data.sonarr_data.indexers.items.clone();
        let modal_test_results = test_results
          .iter()
          .map(|result| {
            let name = indexers
              .iter()
              .filter(|&indexer| indexer.id == result.id)
              .map(|indexer| indexer.name.clone())
              .nth(0)
              .unwrap_or_default();
            let validation_failures = result
              .validation_failures
              .iter()
              .map(|failure| {
                format!(
                  "Failure for field '{}': {}",
                  failure.property_name, failure.error_message
                )
              })
              .collect::<Vec<String>>()
              .join(", ");

            IndexerTestResultModalItem {
              name: name.unwrap_or_default(),
              is_valid: result.is_valid,
              validation_failures: validation_failures.into(),
            }
          })
          .collect();
        test_all_indexer_results.set_items(modal_test_results);
        app.data.sonarr_data.indexer_test_all_results = Some(test_all_indexer_results);
      })
      .await
  }

  async fn toggle_sonarr_episode_monitoring(&mut self, episode_id: Option<i64>) -> Result<()> {
    let event = SonarrEvent::ToggleEpisodeMonitoring(episode_id);
    let detail_event = SonarrEvent::GetEpisodeDetails(None);

    let (id, monitored) = if let Some(episode_id) = episode_id {
      info!("Fetching episode details for episode id: {episode_id}");
      let request_props = self
        .request_props_from(
          detail_event,
          RequestMethod::Get,
          None::<()>,
          Some(format!("/{episode_id}")),
          None,
        )
        .await;

      let mut monitored = false;

      self
        .handle_request::<(), Value>(request_props, |detailed_episode_body, _| {
          monitored = detailed_episode_body
            .get("monitored")
            .unwrap()
            .as_bool()
            .unwrap();
        })
        .await?;

      (episode_id, monitored)
    } else {
      let app = self.app.lock().await;
      let current_selection = app
        .data
        .sonarr_data
        .season_details_modal
        .as_ref()
        .unwrap()
        .episodes
        .current_selection();
      (current_selection.id, current_selection.monitored)
    };

    info!("Toggling monitoring for episode id: {id}");

    let body = MonitorEpisodeBody {
      episode_ids: vec![id],
      monitored: !monitored,
    };

    let request_props = self
      .request_props_from(event, RequestMethod::Put, Some(body), None, None)
      .await;

    self
      .handle_request::<MonitorEpisodeBody, ()>(request_props, |_, _| ())
      .await
  }

  async fn trigger_automatic_series_search(&mut self, series_id: Option<i64>) -> Result<Value> {
    let event = SonarrEvent::TriggerAutomaticSeriesSearch(series_id);
    let (id, _) = self.extract_series_id(series_id).await;
    info!("Searching indexers for series with ID: {id}");

    let body = SonarrCommandBody {
      name: "SeriesSearch".to_owned(),
      series_id: Some(id),
      ..SonarrCommandBody::default()
    };

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(body), None, None)
      .await;

    self
      .handle_request::<SonarrCommandBody, Value>(request_props, |_, _| ())
      .await
  }

  async fn trigger_automatic_season_search(
    &mut self,
    series_season_id_tuple: Option<(i64, i64)>,
  ) -> Result<Value> {
    let event = SonarrEvent::TriggerAutomaticSeasonSearch(series_season_id_tuple);
    let (series_id, season_number) =
      if let Some((series_id, season_number)) = series_season_id_tuple {
        (Some(series_id), Some(season_number))
      } else {
        (None, None)
      };

    let (series_id, _) = self.extract_series_id(series_id).await;
    let (season_number, _) = self.extract_season_number(season_number).await?;
    info!("Searching indexers for series with ID: {series_id} and season number: {season_number}");

    let body = SonarrCommandBody {
      name: "SeasonSearch".to_owned(),
      season_number: Some(season_number),
      series_id: Some(series_id),
      ..SonarrCommandBody::default()
    };

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(body), None, None)
      .await;

    self
      .handle_request::<SonarrCommandBody, Value>(request_props, |_, _| ())
      .await
  }

  async fn trigger_automatic_episode_search(&mut self, episode_id: Option<i64>) -> Result<Value> {
    let event = SonarrEvent::TriggerAutomaticEpisodeSearch(episode_id);
    let id = self.extract_episode_id(episode_id).await;
    info!("Searching indexers for episode with ID: {id}");

    let body = SonarrCommandBody {
      name: "EpisodeSearch".to_owned(),
      episode_ids: Some(vec![id]),
      ..SonarrCommandBody::default()
    };

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(body), None, None)
      .await;

    self
      .handle_request::<SonarrCommandBody, Value>(request_props, |_, _| ())
      .await
  }

  async fn update_all_series(&mut self) -> Result<Value> {
    info!("Updating all series");
    let event = SonarrEvent::UpdateAllSeries;
    let body = SonarrCommandBody {
      name: "RefreshSeries".to_owned(),
      ..SonarrCommandBody::default()
    };

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(body), None, None)
      .await;

    self
      .handle_request::<SonarrCommandBody, Value>(request_props, |_, _| ())
      .await
  }

  async fn update_and_scan_series(&mut self, series_id: Option<i64>) -> Result<Value> {
    let (id, _) = self.extract_series_id(series_id).await;
    let event = SonarrEvent::UpdateAndScanSeries(None);
    info!("Updating and scanning series with ID: {id}");
    let body = SonarrCommandBody {
      name: "RefreshSeries".to_owned(),
      series_id: Some(id),
      ..SonarrCommandBody::default()
    };

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(body), None, None)
      .await;

    self
      .handle_request::<SonarrCommandBody, Value>(request_props, |_, _| ())
      .await
  }

  async fn update_sonarr_downloads(&mut self) -> Result<Value> {
    info!("Updating Sonarr downloads");
    let event = SonarrEvent::UpdateDownloads;
    let body = CommandBody {
      name: "RefreshMonitoredDownloads".to_owned(),
    };

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(body), None, None)
      .await;

    self
      .handle_request::<CommandBody, Value>(request_props, |_, _| ())
      .await
  }

  async fn extract_and_add_sonarr_tag_ids_vec(&mut self, edit_tags: String) -> Vec<i64> {
    let tags_map = self.app.lock().await.data.sonarr_data.tags_map.clone();
    let tags = edit_tags.clone();
    let missing_tags_vec = edit_tags
      .split(',')
      .filter(|&tag| !tag.is_empty() && tags_map.get_by_right(tag.to_lowercase().trim()).is_none())
      .collect::<Vec<&str>>();

    for tag in missing_tags_vec {
      self
        .add_sonarr_tag(tag.trim().to_owned())
        .await
        .expect("Unable to add tag");
    }

    let app = self.app.lock().await;
    tags
      .split(',')
      .filter(|tag| !tag.is_empty())
      .map(|tag| {
        *app
          .data
          .sonarr_data
          .tags_map
          .get_by_right(tag.to_lowercase().trim())
          .unwrap()
      })
      .collect()
  }

  async fn extract_series_id(&mut self, series_id: Option<i64>) -> (i64, String) {
    let series_id = if let Some(id) = series_id {
      id
    } else {
      self
        .app
        .lock()
        .await
        .data
        .sonarr_data
        .series
        .current_selection()
        .id
    };
    (series_id, format!("seriesId={series_id}"))
  }

  async fn extract_season_number(&mut self, season_number: Option<i64>) -> Result<(i64, String)> {
    if let Some(number) = season_number {
      Ok((number, format!("seasonNumber={number}")))
    } else if !self.app.lock().await.data.sonarr_data.seasons.is_empty() {
      let season_number = self
        .app
        .lock()
        .await
        .data
        .sonarr_data
        .seasons
        .current_selection()
        .season_number;
      Ok((season_number, format!("seasonNumber={season_number}")))
    } else {
      Err(anyhow!("No season number provided"))
    }
  }

  async fn extract_episode_id(&mut self, episode_id: Option<i64>) -> i64 {
    let episode_id = if let Some(id) = episode_id {
      id
    } else {
      self
        .app
        .lock()
        .await
        .data
        .sonarr_data
        .season_details_modal
        .as_ref()
        .expect("Season details have not been loaded")
        .episodes
        .current_selection()
        .id
    };

    episode_id
  }
}

fn get_episode_status(has_file: bool, downloads_vec: &[DownloadRecord], episode_id: i64) -> String {
  if !has_file {
    if let Some(download) = downloads_vec
      .iter()
      .find(|&download| download.episode_id == episode_id)
    {
      if download.status == DownloadStatus::Downloading {
        return "Downloading".to_owned();
      }

      if download.status == DownloadStatus::Completed {
        return "Awaiting Import".to_owned();
      }
    }

    return "Missing".to_owned();
  }

  "Downloaded".to_owned()
}
