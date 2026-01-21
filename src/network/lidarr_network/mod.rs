use anyhow::Result;
use log::info;

use super::{NetworkEvent, NetworkResource};
use crate::models::lidarr_models::{
  AddArtistBody, AddLidarrRootFolderBody, DeleteParams, EditArtistParams,
  LidarrReleaseDownloadBody, LidarrSerdeable, LidarrTaskName, MetadataProfile,
};
use crate::models::servarr_models::{EditIndexerParams, IndexerSettings, QualityProfile, Tag};
use crate::network::{Network, RequestMethod};

mod blocklist;
mod downloads;
mod history;
mod indexers;
mod library;
mod root_folders;
mod system;

#[cfg(test)]
#[path = "lidarr_network_tests.rs"]
mod lidarr_network_tests;

#[cfg(test)]
#[path = "lidarr_network_test_utils.rs"]
pub mod lidarr_network_test_utils;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum LidarrEvent {
  AddArtist(AddArtistBody),
  AddRootFolder(AddLidarrRootFolderBody),
  AddTag(String),
  ClearBlocklist,
  DeleteAlbum(DeleteParams),
  DeleteArtist(DeleteParams),
  DeleteBlocklistItem(i64),
  DeleteDownload(i64),
  DeleteIndexer(i64),
  DeleteRootFolder(i64),
  DeleteTag(i64),
  DeleteTrackFile(i64),
  DownloadRelease(LidarrReleaseDownloadBody),
  EditArtist(EditArtistParams),
  EditAllIndexerSettings(IndexerSettings),
  EditIndexer(EditIndexerParams),
  GetAlbums(i64),
  GetAlbumDetails(i64),
  GetAlbumHistory(i64, i64),
  GetAlbumReleases(i64, i64),
  GetArtistHistory(i64),
  GetAllIndexerSettings,
  GetArtistDetails(i64),
  GetBlocklist,
  GetDiscographyReleases(i64),
  GetDiskSpace,
  GetDownloads(u64),
  GetHistory(u64),
  GetHostConfig,
  GetIndexers,
  GetLogs(u64),
  MarkHistoryItemAsFailed(i64),
  GetMetadataProfiles,
  GetQualityProfiles,
  GetQueuedEvents,
  GetRootFolders,
  GetSecurityConfig,
  GetStatus,
  GetTrackDetails(i64),
  GetTracks(i64, i64),
  GetTrackFiles(i64),
  GetTrackHistory(i64, i64, i64),
  GetUpdates,
  GetTags,
  GetTasks,
  HealthCheck,
  ListArtists,
  SearchNewArtist(String),
  StartTask(LidarrTaskName),
  TestIndexer(i64),
  TestAllIndexers,
  ToggleAlbumMonitoring(i64),
  ToggleArtistMonitoring(i64),
  TriggerAutomaticArtistSearch(i64),
  TriggerAutomaticAlbumSearch(i64),
  UpdateAllArtists,
  UpdateAndScanArtist(i64),
  UpdateDownloads,
}

impl NetworkResource for LidarrEvent {
  fn resource(&self) -> &'static str {
    match &self {
      LidarrEvent::AddTag(_) | LidarrEvent::DeleteTag(_) | LidarrEvent::GetTags => "/tag",
      LidarrEvent::ClearBlocklist => "/blocklist/bulk",
      LidarrEvent::DeleteTrackFile(_) | LidarrEvent::GetTrackFiles(_) => "/trackfile",
      LidarrEvent::DeleteBlocklistItem(_) => "/blocklist",
      LidarrEvent::GetAllIndexerSettings | LidarrEvent::EditAllIndexerSettings(_) => {
        "/config/indexer"
      }
      LidarrEvent::DeleteArtist(_)
      | LidarrEvent::EditArtist(_)
      | LidarrEvent::GetArtistDetails(_)
      | LidarrEvent::ListArtists
      | LidarrEvent::AddArtist(_)
      | LidarrEvent::ToggleArtistMonitoring(_) => "/artist",
      LidarrEvent::GetAlbums(_)
      | LidarrEvent::ToggleAlbumMonitoring(_)
      | LidarrEvent::GetAlbumDetails(_)
      | LidarrEvent::DeleteAlbum(_) => "/album",
      LidarrEvent::GetArtistHistory(_)
      | LidarrEvent::GetAlbumHistory(_, _)
      | LidarrEvent::GetTrackHistory(_, _, _) => "/history/artist",
      LidarrEvent::GetBlocklist => "/blocklist?page=1&pageSize=10000",
      LidarrEvent::GetLogs(_) => "/log",
      LidarrEvent::GetDiskSpace => "/diskspace",
      LidarrEvent::GetDownloads(_) | LidarrEvent::DeleteDownload(_) => "/queue",
      LidarrEvent::GetHistory(_) => "/history",
      LidarrEvent::MarkHistoryItemAsFailed(_) => "/history/failed",
      LidarrEvent::GetDiscographyReleases(_)
      | LidarrEvent::DownloadRelease(_)
      | LidarrEvent::GetAlbumReleases(_, _) => "/release",
      LidarrEvent::GetHostConfig | LidarrEvent::GetSecurityConfig => "/config/host",
      LidarrEvent::GetIndexers | LidarrEvent::DeleteIndexer(_) | LidarrEvent::EditIndexer(_) => {
        "/indexer"
      }
      LidarrEvent::TriggerAutomaticArtistSearch(_)
      | LidarrEvent::UpdateAllArtists
      | LidarrEvent::UpdateAndScanArtist(_)
      | LidarrEvent::UpdateDownloads
      | LidarrEvent::GetQueuedEvents
      | LidarrEvent::StartTask(_)
      | LidarrEvent::TriggerAutomaticAlbumSearch(_) => "/command",
      LidarrEvent::GetMetadataProfiles => "/metadataprofile",
      LidarrEvent::GetQualityProfiles => "/qualityprofile",
      LidarrEvent::GetRootFolders
      | LidarrEvent::AddRootFolder(_)
      | LidarrEvent::DeleteRootFolder(_) => "/rootfolder",
      LidarrEvent::TestIndexer(_) => "/indexer/test",
      LidarrEvent::TestAllIndexers => "/indexer/testall",
      LidarrEvent::GetStatus => "/system/status",
      LidarrEvent::GetTasks => "/system/task",
      LidarrEvent::GetTracks(_, _) | LidarrEvent::GetTrackDetails(_) => "/track",
      LidarrEvent::GetUpdates => "/update",
      LidarrEvent::HealthCheck => "/health",
      LidarrEvent::SearchNewArtist(_) => "/artist/lookup",
    }
  }
}

impl From<LidarrEvent> for NetworkEvent {
  fn from(lidarr_event: LidarrEvent) -> Self {
    NetworkEvent::Lidarr(lidarr_event)
  }
}

impl Network<'_, '_> {
  pub async fn handle_lidarr_event(
    &mut self,
    lidarr_event: LidarrEvent,
  ) -> Result<LidarrSerdeable> {
    match lidarr_event {
      LidarrEvent::AddTag(tag) => self.add_lidarr_tag(tag).await.map(LidarrSerdeable::from),
      LidarrEvent::AddRootFolder(path) => self
        .add_lidarr_root_folder(path)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::ClearBlocklist => self
        .clear_lidarr_blocklist()
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::DeleteAlbum(params) => {
        self.delete_album(params).await.map(LidarrSerdeable::from)
      }
      LidarrEvent::DeleteArtist(params) => {
        self.delete_artist(params).await.map(LidarrSerdeable::from)
      }
      LidarrEvent::DeleteBlocklistItem(blocklist_item_id) => self
        .delete_lidarr_blocklist_item(blocklist_item_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::DeleteDownload(download_id) => self
        .delete_lidarr_download(download_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::DeleteTrackFile(track_file_id) => self
        .delete_lidarr_track_file(track_file_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::EditAllIndexerSettings(params) => self
        .edit_all_lidarr_indexer_settings(params)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::EditIndexer(params) => self
        .edit_lidarr_indexer(params)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::DeleteIndexer(indexer_id) => self
        .delete_lidarr_indexer(indexer_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::DeleteRootFolder(root_folder_id) => self
        .delete_lidarr_root_folder(root_folder_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::DeleteTag(tag_id) => self
        .delete_lidarr_tag(tag_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::DownloadRelease(lidarr_release_download_body) => self
        .download_lidarr_release(lidarr_release_download_body)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetAlbums(artist_id) => {
        self.get_albums(artist_id).await.map(LidarrSerdeable::from)
      }
      LidarrEvent::GetAllIndexerSettings => self
        .get_all_lidarr_indexer_settings()
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetArtistDetails(artist_id) => self
        .get_artist_details(artist_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetAlbumDetails(album_id) => self
        .get_album_details(album_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetAlbumHistory(artist_id, album_id) => self
        .get_lidarr_album_history(artist_id, album_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetAlbumReleases(artist_id, album_id) => self
        .get_album_releases(artist_id, album_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetBlocklist => self.get_lidarr_blocklist().await.map(LidarrSerdeable::from),
      LidarrEvent::GetDiscographyReleases(artist_id) => self
        .get_artist_discography_releases(artist_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetDiskSpace => self.get_lidarr_diskspace().await.map(LidarrSerdeable::from),
      LidarrEvent::GetDownloads(count) => self
        .get_lidarr_downloads(count)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetIndexers => self.get_lidarr_indexers().await.map(LidarrSerdeable::from),
      LidarrEvent::GetHistory(events) => self
        .get_lidarr_history(events)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetArtistHistory(artist_id) => self
        .get_lidarr_artist_history(artist_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetLogs(events) => self
        .get_lidarr_logs(events)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::MarkHistoryItemAsFailed(history_item_id) => self
        .mark_lidarr_history_item_as_failed(history_item_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetHostConfig => self
        .get_lidarr_host_config()
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetMetadataProfiles => self
        .get_lidarr_metadata_profiles()
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetQualityProfiles => self
        .get_lidarr_quality_profiles()
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetQueuedEvents => self
        .get_queued_lidarr_events()
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetRootFolders => self
        .get_lidarr_root_folders()
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetSecurityConfig => self
        .get_lidarr_security_config()
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetStatus => self.get_lidarr_status().await.map(LidarrSerdeable::from),
      LidarrEvent::GetTags => self.get_lidarr_tags().await.map(LidarrSerdeable::from),
      LidarrEvent::GetTasks => self.get_lidarr_tasks().await.map(LidarrSerdeable::from),
      LidarrEvent::GetTrackDetails(track_id) => self
        .get_track_details(track_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetTracks(artist_id, album_id) => self
        .get_tracks(artist_id, album_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetTrackFiles(album_id) => self
        .get_track_files(album_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetTrackHistory(artist_id, album_id, track_id) => self
        .get_lidarr_track_history(artist_id, album_id, track_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetUpdates => self.get_lidarr_updates().await.map(LidarrSerdeable::from),
      LidarrEvent::HealthCheck => self
        .get_lidarr_healthcheck()
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::ListArtists => self.list_artists().await.map(LidarrSerdeable::from),
      LidarrEvent::SearchNewArtist(query) => {
        self.search_artist(query).await.map(LidarrSerdeable::from)
      }
      LidarrEvent::StartTask(task_name) => self
        .start_lidarr_task(task_name)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::ToggleAlbumMonitoring(album_id) => self
        .toggle_album_monitoring(album_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::ToggleArtistMonitoring(artist_id) => self
        .toggle_artist_monitoring(artist_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::TriggerAutomaticArtistSearch(artist_id) => self
        .trigger_automatic_artist_search(artist_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::TriggerAutomaticAlbumSearch(album_id) => self
        .trigger_automatic_album_search(album_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::UpdateAllArtists => self.update_all_artists().await.map(LidarrSerdeable::from),
      LidarrEvent::UpdateAndScanArtist(artist_id) => self
        .update_and_scan_artist(artist_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::EditArtist(params) => self.edit_artist(params).await.map(LidarrSerdeable::from),
      LidarrEvent::AddArtist(body) => self.add_artist(body).await.map(LidarrSerdeable::from),
      LidarrEvent::UpdateDownloads => self
        .update_lidarr_downloads()
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::TestAllIndexers => self
        .test_all_lidarr_indexers()
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::TestIndexer(indexer_id) => self
        .test_lidarr_indexer(indexer_id)
        .await
        .map(LidarrSerdeable::from),
    }
  }

  pub(in crate::network::lidarr_network) async fn get_lidarr_healthcheck(&mut self) -> Result<()> {
    info!("Performing Lidarr health check");
    let event = LidarrEvent::HealthCheck;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }

  async fn get_lidarr_metadata_profiles(&mut self) -> Result<Vec<MetadataProfile>> {
    info!("Fetching Lidarr metadata profiles");
    let event = LidarrEvent::GetMetadataProfiles;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<MetadataProfile>>(request_props, |metadata_profiles, mut app| {
        app.data.lidarr_data.metadata_profile_map = metadata_profiles
          .into_iter()
          .map(|profile| (profile.id, profile.name))
          .collect();
      })
      .await
  }

  async fn get_lidarr_quality_profiles(&mut self) -> Result<Vec<QualityProfile>> {
    info!("Fetching Lidarr quality profiles");
    let event = LidarrEvent::GetQualityProfiles;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<QualityProfile>>(request_props, |quality_profiles, mut app| {
        app.data.lidarr_data.quality_profile_map = quality_profiles
          .into_iter()
          .map(|profile| (profile.id, profile.name))
          .collect();
      })
      .await
  }

  async fn get_lidarr_tags(&mut self) -> Result<Vec<Tag>> {
    info!("Fetching Lidarr tags");
    let event = LidarrEvent::GetTags;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<Tag>>(request_props, |tags_vec, mut app| {
        app.data.lidarr_data.tags_map = tags_vec
          .into_iter()
          .map(|tag| (tag.id, tag.label))
          .collect();
      })
      .await
  }

  async fn add_lidarr_tag(&mut self, tag: String) -> Result<Tag> {
    info!("Adding a new Lidarr tag");
    let event = LidarrEvent::AddTag(String::new());

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Post,
        Some(serde_json::json!({ "label": tag })),
        None,
        None,
      )
      .await;

    self
      .handle_request::<serde_json::Value, Tag>(request_props, |tag, mut app| {
        app.data.lidarr_data.tags_map.insert(tag.id, tag.label);
      })
      .await
  }

  async fn delete_lidarr_tag(&mut self, id: i64) -> Result<()> {
    info!("Deleting Lidarr tag with ID: {id}");
    let event = LidarrEvent::DeleteTag(id);

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

  pub(in crate::network::lidarr_network) async fn extract_and_add_lidarr_tag_ids_vec(
    &mut self,
    edit_tags: &str,
  ) -> Vec<i64> {
    let missing_tags_vec = {
      let tags_map = &self.app.lock().await.data.lidarr_data.tags_map;
      edit_tags
        .split(',')
        .filter(|&tag| {
          !tag.is_empty() && tags_map.get_by_right(tag.to_lowercase().trim()).is_none()
        })
        .collect::<Vec<&str>>()
    };

    for tag in missing_tags_vec {
      self
        .add_lidarr_tag(tag.trim().to_owned())
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
          .lidarr_data
          .tags_map
          .get_by_right(tag.to_lowercase().trim())
          .unwrap()
      })
      .collect()
  }
}
