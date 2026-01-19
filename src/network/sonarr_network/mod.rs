use anyhow::Result;
use log::info;
use serde_json::{Value, json};

use super::{Network, NetworkEvent, NetworkResource};
use crate::{
  models::{
    servarr_models::{
      AddRootFolderBody, EditIndexerParams, IndexerSettings, Language, QualityProfile, Tag,
    },
    sonarr_models::{
      AddSeriesBody, DeleteSeriesParams, EditSeriesParams, SonarrReleaseDownloadBody,
      SonarrSerdeable, SonarrTaskName,
    },
  },
  network::RequestMethod,
};
#[cfg(test)]
#[path = "sonarr_network_tests.rs"]
mod sonarr_network_tests;

#[cfg(test)]
#[path = "sonarr_network_test_utils.rs"]
pub mod sonarr_network_test_utils;

mod blocklist;
mod downloads;
mod history;
mod indexers;
mod library;
mod root_folders;
mod system;

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
  EditAllIndexerSettings(IndexerSettings),
  EditIndexer(EditIndexerParams),
  EditSeries(EditSeriesParams),
  GetAllIndexerSettings,
  GetBlocklist,
  GetDownloads(u64),
  GetHistory(u64),
  GetHostConfig,
  GetIndexers,
  GetEpisodeDetails(i64),
  GetEpisodes(i64),
  GetEpisodeFiles(i64),
  GetEpisodeHistory(i64),
  GetLanguageProfiles,
  GetLogs(u64),
  GetDiskSpace,
  GetQualityProfiles,
  GetQueuedEvents,
  GetRootFolders,
  GetEpisodeReleases(i64),
  GetSeasonHistory(i64, i64),
  GetSeasonReleases(i64, i64),
  GetSecurityConfig,
  GetSeriesDetails(i64),
  GetSeriesHistory(i64),
  GetStatus,
  GetUpdates,
  GetTags,
  GetTasks,
  HealthCheck,
  ListSeries,
  MarkHistoryItemAsFailed(i64),
  SearchNewSeries(String),
  StartTask(SonarrTaskName),
  TestIndexer(i64),
  TestAllIndexers,
  ToggleSeasonMonitoring(i64, i64),
  ToggleSeriesMonitoring(i64),
  ToggleEpisodeMonitoring(i64),
  TriggerAutomaticEpisodeSearch(i64),
  TriggerAutomaticSeasonSearch(i64, i64),
  TriggerAutomaticSeriesSearch(i64),
  UpdateAllSeries,
  UpdateAndScanSeries(i64),
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
      SonarrEvent::GetDownloads(_) | SonarrEvent::DeleteDownload(_) => "/queue",
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
      | SonarrEvent::TriggerAutomaticSeasonSearch(_, _)
      | SonarrEvent::TriggerAutomaticEpisodeSearch(_)
      | SonarrEvent::UpdateAllSeries
      | SonarrEvent::UpdateAndScanSeries(_)
      | SonarrEvent::UpdateDownloads => "/command",
      SonarrEvent::GetRootFolders
      | SonarrEvent::DeleteRootFolder(_)
      | SonarrEvent::AddRootFolder(_) => "/rootfolder",
      SonarrEvent::GetSeasonReleases(_, _) | SonarrEvent::GetEpisodeReleases(_) => "/release",
      SonarrEvent::GetSeriesHistory(_) | SonarrEvent::GetSeasonHistory(_, _) => "/history/series",
      SonarrEvent::GetStatus => "/system/status",
      SonarrEvent::GetTasks => "/system/task",
      SonarrEvent::GetUpdates => "/update",
      SonarrEvent::HealthCheck => "/health",
      SonarrEvent::AddSeries(_)
      | SonarrEvent::ListSeries
      | SonarrEvent::GetSeriesDetails(_)
      | SonarrEvent::DeleteSeries(_)
      | SonarrEvent::EditSeries(_)
      | SonarrEvent::ToggleSeasonMonitoring(_, _)
      | SonarrEvent::ToggleSeriesMonitoring(_) => "/series",
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

impl Network<'_, '_> {
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
      SonarrEvent::GetDownloads(count) => self
        .get_sonarr_downloads(count)
        .await
        .map(SonarrSerdeable::from),
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
      SonarrEvent::GetSeasonHistory(series_id, season_number) => self
        .get_sonarr_season_history(series_id, season_number)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::GetSeasonReleases(series_id, season_number) => self
        .get_season_releases(series_id, season_number)
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
      SonarrEvent::ToggleSeasonMonitoring(series_id, season_number) => self
        .toggle_sonarr_season_monitoring(series_id, season_number)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::ToggleSeriesMonitoring(series_id) => self
        .toggle_sonarr_series_monitoring(series_id)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::TriggerAutomaticSeasonSearch(series_id, season_number) => self
        .trigger_automatic_season_search(series_id, season_number)
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

  pub(in crate::network::sonarr_network) async fn extract_and_add_sonarr_tag_ids_vec(
    &mut self,
    edit_tags: &str,
  ) -> Vec<i64> {
    let missing_tags_vec = {
      let tags_map = &self.app.lock().await.data.sonarr_data.tags_map;
      edit_tags
        .split(',')
        .filter(|&tag| {
          !tag.is_empty() && tags_map.get_by_right(tag.to_lowercase().trim()).is_none()
        })
        .collect::<Vec<&str>>()
    };

    for tag in missing_tags_vec {
      self
        .add_sonarr_tag(tag.trim().to_owned())
        .await
        .expect("Unable to add tag");
    }

    let app = self.app.lock().await;
    edit_tags
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
}
