use anyhow::Result;
use std::fmt::Debug;

use log::info;
use serde_json::{json, Value};

use crate::models::radarr_models::{
  AddMovieBody, DeleteMovieParams, EditCollectionParams, EditMovieParams, IndexerSettings,
  RadarrReleaseDownloadBody, RadarrSerdeable, RadarrTaskName,
};
use crate::models::servarr_models::{AddRootFolderBody, EditIndexerParams, QualityProfile, Tag};
use crate::network::{Network, NetworkEvent, RequestMethod};

use super::NetworkResource;

mod blocklist;
mod collections;
mod downloads;
mod indexers;
mod library;
mod root_folders;
mod system;

#[cfg(test)]
#[path = "radarr_network_tests.rs"]
mod radarr_network_tests;

#[cfg(test)]
#[path = "radarr_network_test_utils.rs"]
mod radarr_network_test_utils;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum RadarrEvent {
  AddMovie(AddMovieBody),
  AddRootFolder(AddRootFolderBody),
  AddTag(String),
  ClearBlocklist,
  DeleteBlocklistItem(i64),
  DeleteDownload(i64),
  DeleteIndexer(i64),
  DeleteMovie(DeleteMovieParams),
  DeleteRootFolder(i64),
  DeleteTag(i64),
  DownloadRelease(RadarrReleaseDownloadBody),
  EditAllIndexerSettings(IndexerSettings),
  EditCollection(EditCollectionParams),
  EditIndexer(EditIndexerParams),
  EditMovie(EditMovieParams),
  GetBlocklist,
  GetCollections,
  GetDownloads(u64),
  GetHostConfig,
  GetIndexers,
  GetAllIndexerSettings,
  GetLogs(u64),
  GetMovieCredits(i64),
  GetMovieDetails(i64),
  GetMovieHistory(i64),
  GetMovies,
  GetDiskSpace,
  GetQualityProfiles,
  GetQueuedEvents,
  GetReleases(i64),
  GetRootFolders,
  GetSecurityConfig,
  GetStatus,
  GetTags,
  GetTasks,
  GetUpdates,
  HealthCheck,
  SearchNewMovie(String),
  StartTask(RadarrTaskName),
  TestIndexer(i64),
  TestAllIndexers,
  ToggleMovieMonitoring(i64),
  TriggerAutomaticSearch(i64),
  UpdateAllMovies,
  UpdateAndScan(i64),
  UpdateCollections,
  UpdateDownloads,
}

impl NetworkResource for RadarrEvent {
  fn resource(&self) -> &'static str {
    match &self {
      RadarrEvent::ClearBlocklist => "/blocklist/bulk",
      RadarrEvent::DeleteBlocklistItem(_) => "/blocklist",
      RadarrEvent::GetBlocklist => "/blocklist?page=1&pageSize=10000",
      RadarrEvent::GetCollections | RadarrEvent::EditCollection(_) => "/collection",
      RadarrEvent::GetDownloads(_) | RadarrEvent::DeleteDownload(_) => "/queue",
      RadarrEvent::GetHostConfig | RadarrEvent::GetSecurityConfig => "/config/host",
      RadarrEvent::GetIndexers | RadarrEvent::EditIndexer(_) | RadarrEvent::DeleteIndexer(_) => {
        "/indexer"
      }
      RadarrEvent::GetAllIndexerSettings | RadarrEvent::EditAllIndexerSettings(_) => {
        "/config/indexer"
      }
      RadarrEvent::GetLogs(_) => "/log",
      RadarrEvent::AddMovie(_)
      | RadarrEvent::EditMovie(_)
      | RadarrEvent::GetMovies
      | RadarrEvent::GetMovieDetails(_)
      | RadarrEvent::DeleteMovie(_)
      | RadarrEvent::ToggleMovieMonitoring(_) => "/movie",
      RadarrEvent::SearchNewMovie(_) => "/movie/lookup",
      RadarrEvent::GetMovieCredits(_) => "/credit",
      RadarrEvent::GetMovieHistory(_) => "/history/movie",
      RadarrEvent::GetDiskSpace => "/diskspace",
      RadarrEvent::GetQualityProfiles => "/qualityprofile",
      RadarrEvent::GetReleases(_) | RadarrEvent::DownloadRelease(_) => "/release",
      RadarrEvent::AddRootFolder(_)
      | RadarrEvent::GetRootFolders
      | RadarrEvent::DeleteRootFolder(_) => "/rootfolder",
      RadarrEvent::GetStatus => "/system/status",
      RadarrEvent::GetTags | RadarrEvent::AddTag(_) | RadarrEvent::DeleteTag(_) => "/tag",
      RadarrEvent::GetTasks => "/system/task",
      RadarrEvent::GetUpdates => "/update",
      RadarrEvent::TestIndexer(_) => "/indexer/test",
      RadarrEvent::TestAllIndexers => "/indexer/testall",
      RadarrEvent::StartTask(_)
      | RadarrEvent::GetQueuedEvents
      | RadarrEvent::TriggerAutomaticSearch(_)
      | RadarrEvent::UpdateAndScan(_)
      | RadarrEvent::UpdateAllMovies
      | RadarrEvent::UpdateDownloads
      | RadarrEvent::UpdateCollections => "/command",
      RadarrEvent::HealthCheck => "/health",
    }
  }
}

impl From<RadarrEvent> for NetworkEvent {
  fn from(radarr_event: RadarrEvent) -> Self {
    NetworkEvent::Radarr(radarr_event)
  }
}

impl Network<'_, '_> {
  pub async fn handle_radarr_event(
    &mut self,
    radarr_event: RadarrEvent,
  ) -> Result<RadarrSerdeable> {
    match radarr_event {
      RadarrEvent::AddMovie(body) => self.add_movie(body).await.map(RadarrSerdeable::from),
      RadarrEvent::AddRootFolder(path) => self
        .add_radarr_root_folder(path)
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::AddTag(tag) => self.add_radarr_tag(tag).await.map(RadarrSerdeable::from),
      RadarrEvent::ClearBlocklist => self
        .clear_radarr_blocklist()
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::DeleteBlocklistItem(blocklist_item_id) => self
        .delete_radarr_blocklist_item(blocklist_item_id)
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::DeleteDownload(download_id) => self
        .delete_radarr_download(download_id)
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::DeleteIndexer(indexer_id) => self
        .delete_radarr_indexer(indexer_id)
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::DeleteMovie(params) => {
        self.delete_movie(params).await.map(RadarrSerdeable::from)
      }
      RadarrEvent::DeleteRootFolder(root_folder_id) => self
        .delete_radarr_root_folder(root_folder_id)
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::DeleteTag(tag_id) => self
        .delete_radarr_tag(tag_id)
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::DownloadRelease(params) => self
        .download_radarr_release(params)
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::EditAllIndexerSettings(params) => self
        .edit_all_radarr_indexer_settings(params)
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::EditCollection(params) => self
        .edit_collection(params)
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::EditIndexer(params) => self
        .edit_radarr_indexer(params)
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::EditMovie(params) => self.edit_movie(params).await.map(RadarrSerdeable::from),
      RadarrEvent::GetAllIndexerSettings => self
        .get_all_radarr_indexer_settings()
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::GetBlocklist => self.get_radarr_blocklist().await.map(RadarrSerdeable::from),
      RadarrEvent::GetCollections => self.get_collections().await.map(RadarrSerdeable::from),
      RadarrEvent::GetDownloads(count) => self
        .get_radarr_downloads(count)
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::GetHostConfig => self
        .get_radarr_host_config()
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::GetIndexers => self.get_radarr_indexers().await.map(RadarrSerdeable::from),
      RadarrEvent::GetLogs(events) => self
        .get_radarr_logs(events)
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::GetMovieCredits(movie_id) => {
        self.get_credits(movie_id).await.map(RadarrSerdeable::from)
      }
      RadarrEvent::GetMovieDetails(movie_id) => self
        .get_movie_details(movie_id)
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::GetMovieHistory(movie_id) => self
        .get_movie_history(movie_id)
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::GetMovies => self.get_movies().await.map(RadarrSerdeable::from),
      RadarrEvent::GetDiskSpace => self.get_radarr_diskspace().await.map(RadarrSerdeable::from),
      RadarrEvent::GetQualityProfiles => self
        .get_radarr_quality_profiles()
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::GetQueuedEvents => self
        .get_queued_radarr_events()
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::GetReleases(movie_id) => self
        .get_movie_releases(movie_id)
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::GetRootFolders => self
        .get_radarr_root_folders()
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::GetSecurityConfig => self
        .get_radarr_security_config()
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::GetStatus => self.get_radarr_status().await.map(RadarrSerdeable::from),
      RadarrEvent::GetTags => self.get_radarr_tags().await.map(RadarrSerdeable::from),
      RadarrEvent::GetTasks => self.get_radarr_tasks().await.map(RadarrSerdeable::from),
      RadarrEvent::GetUpdates => self.get_radarr_updates().await.map(RadarrSerdeable::from),
      RadarrEvent::HealthCheck => self
        .get_radarr_healthcheck()
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::SearchNewMovie(query) => {
        self.search_movie(query).await.map(RadarrSerdeable::from)
      }
      RadarrEvent::StartTask(task_name) => self
        .start_radarr_task(task_name)
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::TestIndexer(indexer_id) => self
        .test_radarr_indexer(indexer_id)
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::TestAllIndexers => self
        .test_all_radarr_indexers()
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::ToggleMovieMonitoring(movie_id) => self
        .toggle_movie_monitoring(movie_id)
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::TriggerAutomaticSearch(movie_id) => self
        .trigger_automatic_movie_search(movie_id)
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::UpdateAllMovies => self.update_all_movies().await.map(RadarrSerdeable::from),
      RadarrEvent::UpdateAndScan(movie_id) => self
        .update_and_scan_movie(movie_id)
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::UpdateCollections => self.update_collections().await.map(RadarrSerdeable::from),
      RadarrEvent::UpdateDownloads => self
        .update_radarr_downloads()
        .await
        .map(RadarrSerdeable::from),
    }
  }

  pub(in crate::network::radarr_network) async fn add_radarr_tag(
    &mut self,
    tag: String,
  ) -> Result<Tag> {
    info!("Adding a new Radarr tag");
    let event = RadarrEvent::AddTag(String::new());

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
        app.data.radarr_data.tags_map.insert(tag.id, tag.label);
      })
      .await
  }

  pub(in crate::network::radarr_network) async fn delete_radarr_tag(
    &mut self,
    id: i64,
  ) -> Result<()> {
    info!("Deleting Radarr tag with id: {id}");
    let event = RadarrEvent::DeleteTag(id);

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

  async fn get_radarr_healthcheck(&mut self) -> Result<()> {
    info!("Performing Radarr health check");
    let event = RadarrEvent::HealthCheck;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }

  async fn get_radarr_quality_profiles(&mut self) -> Result<Vec<QualityProfile>> {
    info!("Fetching Radarr quality profiles");
    let event = RadarrEvent::GetQualityProfiles;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<QualityProfile>>(request_props, |quality_profiles, mut app| {
        app.data.radarr_data.quality_profile_map = quality_profiles
          .into_iter()
          .map(|profile| (profile.id, profile.name))
          .collect();
      })
      .await
  }

  async fn get_radarr_tags(&mut self) -> Result<Vec<Tag>> {
    info!("Fetching Radarr tags");
    let event = RadarrEvent::GetTags;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<Tag>>(request_props, |tags_vec, mut app| {
        app.data.radarr_data.tags_map = tags_vec
          .into_iter()
          .map(|tag| (tag.id, tag.label))
          .collect();
      })
      .await
  }

  pub(in crate::network::radarr_network) async fn extract_and_add_radarr_tag_ids_vec(
    &mut self,
    edit_tags: &str,
  ) -> Vec<i64> {
    let missing_tags_vec = {
      let tags_map = &self.app.lock().await.data.radarr_data.tags_map;
      edit_tags
        .split(',')
        .filter(|&tag| {
          !tag.is_empty() && tags_map.get_by_right(tag.to_lowercase().trim()).is_none()
        })
        .collect::<Vec<&str>>()
    };

    for tag in missing_tags_vec {
      self
        .add_radarr_tag(tag.trim().to_owned())
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
          .radarr_data
          .tags_map
          .get_by_right(tag.to_lowercase().trim())
          .unwrap()
      })
      .collect()
  }
}
