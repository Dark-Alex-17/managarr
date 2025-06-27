use super::{Network, NetworkEvent, NetworkResource};
use crate::models::lidarr_models::{
  AddArtistBody, AddArtistSearchResult, BlocklistItem, BlocklistResponse, DeleteArtistParams,
  DownloadRecord, DownloadsResponse, EditArtistParams, IndexerSettings, LidarrCommandBody,
  LidarrReleaseDownloadBody, LidarrSerdeable, LidarrTaskName, MonitorAlbumBody, SystemStatus,
};
use crate::models::servarr_data::lidarr::modals::album_details_modal::{
  AlbumDetailsModal, TrackDetailsModal,
};
use crate::models::servarr_models::{
  AddRootFolderBody, CommandBody, DiskSpace, EditIndexerParams, HostConfig, Indexer, Language,
  LogResponse, QualityProfile, QueueEvent, RootFolder, SecurityConfig, Tag, Update,
};
use crate::models::stateful_table::StatefulTable;
use crate::models::{
  lidarr_models::{
    Album, Artist, LidarrHistoryItem, LidarrHistoryWrapper, LidarrRelease, LidarrTask, Track,
    TrackFile,
  },
  radarr_models::IndexerTestResult,
  servarr_data::{lidarr::lidarr_data::ActiveLidarrBlock, modals::IndexerTestResultModalItem},
  HorizontallyScrollableText, Route, Scrollable, ScrollableText,
};
use crate::network::RequestMethod;
use crate::utils::convert_to_gb;
use anyhow::Result;
use indoc::formatdoc;
use log::{debug, info, warn};
use serde_json::{json, Number, Value};
use urlencoding::encode;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum LidarrEvent {
  AddRootFolder(AddRootFolderBody),
  AddArtist(AddArtistBody),
  AddTag(String),
  ClearBlocklist,
  DeleteBlocklistItem(i64),
  DeleteDownload(i64),
  DeleteTrackFile(i64),
  DeleteIndexer(i64),
  DeleteRootFolder(i64),
  DeleteArtist(DeleteArtistParams),
  DeleteTag(i64),
  DownloadRelease(LidarrReleaseDownloadBody),
  EditAllIndexerSettings(IndexerSettings),
  EditIndexer(EditIndexerParams),
  EditArtist(EditArtistParams),
  GetAllIndexerSettings,
  GetBlocklist,
  GetDownloads,
  GetHistory(u64),
  GetHostConfig,
  GetIndexers,
  GetTrackDetails(i64),
  GetTracks(i64),
  GetTrackFiles(i64),
  GetTrackHistory(i64),
  GetUpdates,
  GetMetadataProfiles,
  GetLogs(u64),
  GetDiskSpace,
  GetQualityProfiles,
  GetQueuedEvents,
  GetRootFolders,
  GetTrackReleases(i64),
  GetAlbumHistory((i64, i64)),
  GetAlbumReleases((i64, i64)),
  GetSecurityConfig,
  GetArtistDetails(i64),
  GetArtistHistory(i64),
  GetStatus,
  GetTags,
  GetTasks,
  HealthCheck,
  ListArtists,
  MarkHistoryItemAsFailed(i64),
  SearchNewArtist(String),
  StartTask(LidarrTaskName),
  TestIndexer(i64),
  TestAllIndexers,
  ToggleAlbumMonitoring((i64, i64)),
  ToggleTrackMonitoring(i64),
  TriggerAutomaticTrackSearch(i64),
  TriggerAutomaticAlbumSearch((i64, i64)),
  TriggerAutomaticArtistSearch(i64),
  UpdateAllArtists,
  UpdateAndScanArtist(i64),
  UpdateDownloads,
}
impl NetworkResource for LidarrEvent {
  fn resource(&self) -> &'static str {
    match &self {
      LidarrEvent::AddTag(_) | LidarrEvent::DeleteTag(_) | LidarrEvent::GetTags => "/tag",
      LidarrEvent::ClearBlocklist => "/blocklist/bulk",
      LidarrEvent::DownloadRelease(_) => "/release",
      LidarrEvent::DeleteBlocklistItem(_) => "/blocklist",
      LidarrEvent::GetAllIndexerSettings | LidarrEvent::EditAllIndexerSettings(_) => {
        "/config/indexer"
      }
      LidarrEvent::GetTrackFiles(_) | LidarrEvent::DeleteTrackFile(_) => "/trackfile",
      LidarrEvent::GetBlocklist => "/blocklist?page=1&pageSize=10000",
      LidarrEvent::GetDownloads | LidarrEvent::DeleteDownload(_) => "/queue",
      LidarrEvent::GetTracks(_) | LidarrEvent::GetTrackDetails(_) => "/track",
      LidarrEvent::GetHistory(_) | LidarrEvent::GetTrackHistory(_) => "/history",
      LidarrEvent::GetHostConfig | LidarrEvent::GetSecurityConfig => "/config/host",
      LidarrEvent::GetIndexers | LidarrEvent::DeleteIndexer(_) | LidarrEvent::EditIndexer(_) => {
        "/indexer"
      }
      LidarrEvent::GetMetadataProfiles => "/metadataprofile",
      LidarrEvent::GetLogs(_) => "/log",
      LidarrEvent::GetDiskSpace => "/diskspace",
      LidarrEvent::GetQualityProfiles => "/qualityprofile",
      LidarrEvent::GetQueuedEvents
      | LidarrEvent::StartTask(_)
      | LidarrEvent::TriggerAutomaticArtistSearch(_)
      | LidarrEvent::TriggerAutomaticAlbumSearch(_)
      | LidarrEvent::TriggerAutomaticTrackSearch(_)
      | LidarrEvent::UpdateAllArtists
      | LidarrEvent::UpdateAndScanArtist(_)
      | LidarrEvent::UpdateDownloads => "/command",
      LidarrEvent::GetRootFolders
      | LidarrEvent::DeleteRootFolder(_)
      | LidarrEvent::AddRootFolder(_) => "/rootfolder",
      LidarrEvent::GetAlbumReleases(_) | LidarrEvent::GetTrackReleases(_) => "/release",
      LidarrEvent::GetArtistHistory(_) | LidarrEvent::GetAlbumHistory(_) => "/history/artist",
      LidarrEvent::GetStatus => "/system/status",
      LidarrEvent::GetTasks => "/system/task",
      LidarrEvent::GetUpdates => "/update",
      LidarrEvent::HealthCheck => "/health",
      LidarrEvent::AddArtist(_)
      | LidarrEvent::ListArtists
      | LidarrEvent::GetArtistDetails(_)
      | LidarrEvent::DeleteArtist(_)
      | LidarrEvent::EditArtist(_)
      | LidarrEvent::ToggleAlbumMonitoring(_) => "/artist",
      LidarrEvent::SearchNewArtist(_) => "/artist/lookup",
      LidarrEvent::MarkHistoryItemAsFailed(_) => "/history/failed",
      LidarrEvent::TestIndexer(_) => "/indexer/test",
      LidarrEvent::TestAllIndexers => "/indexer/testall",
      LidarrEvent::ToggleTrackMonitoring(_) => "/track/monitor",
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
      LidarrEvent::AddRootFolder(path) => self
        .add_lidarr_root_folder(path)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::AddArtist(body) => self
        .add_lidarr_artist(body)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::AddTag(tag) => self.add_lidarr_tag(tag).await.map(LidarrSerdeable::from),
      LidarrEvent::ClearBlocklist => self
        .clear_lidarr_blocklist()
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetAllIndexerSettings => self
        .get_all_lidarr_indexer_settings()
        .await
        .map(LidarrSerdeable::from),
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
      LidarrEvent::DeleteIndexer(indexer_id) => self
        .delete_lidarr_indexer(indexer_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::DeleteRootFolder(root_folder_id) => self
        .delete_lidarr_root_folder(root_folder_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::DeleteArtist(params) => {
        self.delete_artist(params).await.map(LidarrSerdeable::from)
      }
      LidarrEvent::DeleteTag(tag_id) => self
        .delete_lidarr_tag(tag_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::DownloadRelease(lidarr_release_download_body) => self
        .download_lidarr_release(lidarr_release_download_body)
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
      LidarrEvent::EditArtist(params) => self
        .edit_lidarr_artist(params)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetBlocklist => self.get_lidarr_blocklist().await.map(LidarrSerdeable::from),
      LidarrEvent::GetDownloads => self.get_lidarr_downloads().await.map(LidarrSerdeable::from),
      LidarrEvent::GetTracks(artist_id) => {
        self.get_tracks(artist_id).await.map(LidarrSerdeable::from)
      }
      LidarrEvent::GetTrackFiles(artist_id) => self
        .get_track_files(artist_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetTrackDetails(track_id) => self
        .get_track_details(track_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetTrackHistory(track_id) => self
        .get_lidarr_track_history(track_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetHistory(events) => self
        .get_lidarr_history(events)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetHostConfig => self
        .get_lidarr_host_config()
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetIndexers => self.get_lidarr_indexers().await.map(LidarrSerdeable::from),
      LidarrEvent::GetMetadataProfiles => self
        .get_lidarr_metadata_profiles()
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetLogs(events) => self
        .get_lidarr_logs(events)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetDiskSpace => self.get_lidarr_diskspace().await.map(LidarrSerdeable::from),
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
      LidarrEvent::GetTrackReleases(params) => self
        .get_track_releases(params)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetAlbumHistory(params) => self
        .get_lidarr_album_history(params)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetAlbumReleases(params) => self
        .get_album_releases(params)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetSecurityConfig => self
        .get_lidarr_security_config()
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetArtistDetails(artist_id) => self
        .get_artist_details(artist_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetArtistHistory(artist_id) => self
        .get_lidarr_artist_history(artist_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::ListArtists => self.list_artists().await.map(LidarrSerdeable::from),
      LidarrEvent::GetStatus => self.get_lidarr_status().await.map(LidarrSerdeable::from),
      LidarrEvent::GetTags => self.get_lidarr_tags().await.map(LidarrSerdeable::from),
      LidarrEvent::GetTasks => self.get_lidarr_tasks().await.map(LidarrSerdeable::from),
      LidarrEvent::GetUpdates => self.get_lidarr_updates().await.map(LidarrSerdeable::from),
      LidarrEvent::HealthCheck => self
        .get_lidarr_healthcheck()
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::MarkHistoryItemAsFailed(history_item_id) => self
        .mark_lidarr_history_item_as_failed(history_item_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::SearchNewArtist(query) => self
        .search_lidarr_artist(query)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::StartTask(task_name) => self
        .start_lidarr_task(task_name)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::TestIndexer(indexer_id) => self
        .test_lidarr_indexer(indexer_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::TestAllIndexers => self
        .test_all_lidarr_indexers()
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::ToggleTrackMonitoring(track_id) => self
        .toggle_lidarr_track_monitoring(track_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::ToggleAlbumMonitoring(params) => self
        .toggle_lidarr_album_monitoring(params)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::TriggerAutomaticAlbumSearch(params) => self
        .trigger_automatic_album_search(params)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::TriggerAutomaticArtistSearch(artist_id) => self
        .trigger_automatic_artist_search(artist_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::TriggerAutomaticTrackSearch(track_id) => self
        .trigger_automatic_track_search(track_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::UpdateAllArtists => self.update_all_artists().await.map(LidarrSerdeable::from),
      LidarrEvent::UpdateAndScanArtist(artist_id) => self
        .update_and_scan_artist(artist_id)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::UpdateDownloads => self
        .update_lidarr_downloads()
        .await
        .map(LidarrSerdeable::from),
    }
  }
  async fn add_lidarr_root_folder(
    &mut self,
    add_root_folder_body: AddRootFolderBody,
  ) -> Result<Value> {
    info!("Adding new root folder to Lidarr");
    let event = LidarrEvent::AddRootFolder(AddRootFolderBody::default());
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
  async fn add_lidarr_artist(&mut self, mut add_artist_body: AddArtistBody) -> Result<Value> {
    info!("Adding new artist to Lidarr");
    let event = LidarrEvent::AddArtist(AddArtistBody::default());
    if let Some(tag_input_str) = add_artist_body.tag_input_string.as_ref() {
      let tag_ids_vec = self.extract_and_add_lidarr_tag_ids_vec(tag_input_str).await;
      add_artist_body.tags = tag_ids_vec;
    }
    debug!("Add artist body: {add_artist_body:?}");
    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Post,
        Some(add_artist_body),
        None,
        None,
      )
      .await;
    self
      .handle_request::<AddArtistBody, Value>(request_props, |_, _| ())
      .await
  }
  async fn add_lidarr_tag(&mut self, tag: String) -> Result<Tag> {
    info!("Adding a new Lidarr tag");
    let event = LidarrEvent::AddTag(String::new());
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
        app.data.lidarr_data.tags_map.insert(tag.id, tag.label);
      })
      .await
  }
  async fn clear_lidarr_blocklist(&mut self) -> Result<()> {
    info!("Clearing Lidarr blocklist");
    let event = LidarrEvent::ClearBlocklist;
    let ids = self
      .app
      .lock()
      .await
      .data
      .lidarr_data
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
  async fn delete_lidarr_blocklist_item(&mut self, blocklist_item_id: i64) -> Result<()> {
    let event = LidarrEvent::DeleteBlocklistItem(blocklist_item_id);
    info!("Deleting Lidarr blocklist item for item with id: {blocklist_item_id}");
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
  async fn delete_lidarr_track_file(&mut self, track_file_id: i64) -> Result<()> {
    let event = LidarrEvent::DeleteTrackFile(track_file_id);
    info!("Deleting Lidarr track file for track file with id: {track_file_id}");
    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Delete,
        None::<()>,
        Some(format!("/{track_file_id}")),
        None,
      )
      .await;
    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }
  async fn delete_lidarr_download(&mut self, download_id: i64) -> Result<()> {
    let event = LidarrEvent::DeleteDownload(download_id);
    info!("Deleting Lidarr download for download with id: {download_id}");
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
  async fn delete_lidarr_indexer(&mut self, indexer_id: i64) -> Result<()> {
    let event = LidarrEvent::DeleteIndexer(indexer_id);
    info!("Deleting Lidarr indexer for indexer with id: {indexer_id}");
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
  async fn delete_lidarr_root_folder(&mut self, root_folder_id: i64) -> Result<()> {
    let event = LidarrEvent::DeleteRootFolder(root_folder_id);
    info!("Deleting Lidarr root folder for folder with id: {root_folder_id}");
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
  async fn delete_artist(&mut self, delete_artist_params: DeleteArtistParams) -> Result<()> {
    let event = LidarrEvent::DeleteArtist(DeleteArtistParams::default());
    let DeleteArtistParams {
      id,
      delete_artist_files,
      add_list_exclusion,
    } = delete_artist_params;
    info!("Deleting Lidarr artist with ID: {id} with deleteFiles={delete_artist_files} and addImportExclusion={add_list_exclusion}");
    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Delete,
        None::<()>,
        Some(format!("/{id}")),
        Some(format!(
          "deleteFiles={delete_artist_files}&addImportExclusion={add_list_exclusion}"
        )),
      )
      .await;
    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }
  async fn delete_lidarr_tag(&mut self, id: i64) -> Result<()> {
    info!("Deleting Lidarr tag with id: {id}");
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
  async fn download_lidarr_release(
    &mut self,
    lidarr_release_download_body: LidarrReleaseDownloadBody,
  ) -> Result<Value> {
    let event = LidarrEvent::DownloadRelease(LidarrReleaseDownloadBody::default());
    info!("Downloading Lidarr release with params: {lidarr_release_download_body:?}");
    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Post,
        Some(lidarr_release_download_body),
        None,
        None,
      )
      .await;
    self
      .handle_request::<LidarrReleaseDownloadBody, Value>(request_props, |_, _| ())
      .await
  }
  async fn edit_all_lidarr_indexer_settings(&mut self, params: IndexerSettings) -> Result<Value> {
    info!("Updating Lidarr indexer settings");
    let event = LidarrEvent::EditAllIndexerSettings(IndexerSettings::default());
    debug!("Indexer settings body: {params:?}");
    let request_props = self
      .request_props_from(event, RequestMethod::Put, Some(params), None, None)
      .await;
    self
      .handle_request::<IndexerSettings, Value>(request_props, |_, _| {})
      .await
  }
  async fn edit_lidarr_indexer(
    &mut self,
    mut edit_indexer_params: EditIndexerParams,
  ) -> Result<()> {
    if let Some(tag_input_str) = edit_indexer_params.tag_input_string.as_ref() {
      let tag_ids_vec = self.extract_and_add_lidarr_tag_ids_vec(tag_input_str).await;
      edit_indexer_params.tags = Some(tag_ids_vec);
    }
    let detail_event = LidarrEvent::GetIndexers;
    let event = LidarrEvent::EditIndexer(EditIndexerParams::default());
    let id = edit_indexer_params.indexer_id;
    info!("Updating Lidarr indexer with ID: {id}");
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
    ) = {
      let priority = detailed_indexer_body["priority"]
        .as_i64()
        .expect("Unable to deserialize 'priority'");
      let seed_ratio_field_option = detailed_indexer_body["fields"]
        .as_array()
        .unwrap()
        .iter()
        .find(|field| field["name"] == "seedCriteria.seedRatio");
      let name = edit_indexer_params.name.unwrap_or(
        detailed_indexer_body["name"]
          .as_str()
          .expect("Unable to deserialize 'name'")
          .to_owned(),
      );
      let enable_rss = edit_indexer_params.enable_rss.unwrap_or(
        detailed_indexer_body["enableRss"]
          .as_bool()
          .expect("Unable to deserialize 'enableRss'"),
      );
      let enable_automatic_search = edit_indexer_params.enable_automatic_search.unwrap_or(
        detailed_indexer_body["enableAutomaticSearch"]
          .as_bool()
          .expect("Unable to deserialize 'enableAutomaticSearch"),
      );
      let enable_interactive_search = edit_indexer_params.enable_interactive_search.unwrap_or(
        detailed_indexer_body["enableInteractiveSearch"]
          .as_bool()
          .expect("Unable to deserialize 'enableInteractiveSearch'"),
      );
      let url = edit_indexer_params.url.unwrap_or(
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
      let api_key = edit_indexer_params.api_key.unwrap_or(
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
      let seed_ratio = edit_indexer_params.seed_ratio.unwrap_or_else(|| {
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
      let tags = if edit_indexer_params.clear_tags {
        vec![]
      } else {
        edit_indexer_params.tags.unwrap_or(
          detailed_indexer_body["tags"]
            .as_array()
            .expect("Unable to deserialize 'tags'")
            .iter()
            .map(|item| item.as_i64().expect("Unable to deserialize tag ID"))
            .collect(),
        )
      };
      let priority = edit_indexer_params.priority.unwrap_or(priority);
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
  async fn edit_lidarr_artist(&mut self, mut edit_artist_params: EditArtistParams) -> Result<()> {
    info!("Editing Lidarr artist");
    if let Some(tag_input_str) = edit_artist_params.tag_input_string.as_ref() {
      let tag_ids_vec = self.extract_and_add_lidarr_tag_ids_vec(tag_input_str).await;
      edit_artist_params.tags = Some(tag_ids_vec);
    }
    let artist_id = edit_artist_params.artist_id;
    let detail_event = LidarrEvent::GetArtistDetails(artist_id);
    let event = LidarrEvent::EditArtist(EditArtistParams::default());
    info!("Fetching artist details for artist with ID: {artist_id}");
    let request_props = self
      .request_props_from(
        detail_event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{artist_id}")),
        None,
      )
      .await;
    let mut response = String::new();
    self
      .handle_request::<(), Value>(request_props, |detailed_artist_body, _| {
        response = detailed_artist_body.to_string()
      })
      .await?;
    info!("Constructing edit artist body");
    let mut detailed_artist_body: Value = serde_json::from_str(&response)?;
    let (monitored, quality_profile_id, metadata_profile_id, root_folder_path, tags) = {
      let monitored = edit_artist_params.monitored.unwrap_or(
        detailed_artist_body["monitored"]
          .as_bool()
          .expect("Unable to deserialize 'monitored'"),
      );
      let quality_profile_id = edit_artist_params.quality_profile_id.unwrap_or_else(|| {
        detailed_artist_body["qualityProfileId"]
          .as_i64()
          .expect("Unable to deserialize 'qualityProfileId'")
      });
      let metadata_profile_id = edit_artist_params.metadata_profile_id.unwrap_or_else(|| {
        detailed_artist_body["metadataProfileId"]
          .as_i64()
          .expect("Unable to deserialize 'metadataProfileId'")
      });
      let root_folder_path = edit_artist_params.root_folder_path.unwrap_or_else(|| {
        detailed_artist_body["path"]
          .as_str()
          .expect("Unable to deserialize 'path'")
          .to_owned()
      });
      let tags = if edit_artist_params.clear_tags {
        vec![]
      } else {
        edit_artist_params.tags.unwrap_or(
          detailed_artist_body["tags"]
            .as_array()
            .expect("Unable to deserialize 'tags'")
            .iter()
            .map(|item| item.as_i64().expect("Unable to deserialize tag ID"))
            .collect(),
        )
      };
      (
        monitored,
        quality_profile_id,
        metadata_profile_id,
        root_folder_path,
        tags,
      )
    };
    *detailed_artist_body.get_mut("monitored").unwrap() = json!(monitored);
    *detailed_artist_body.get_mut("qualityProfileId").unwrap() = json!(quality_profile_id);
    *detailed_artist_body.get_mut("metadataProfileId").unwrap() = json!(metadata_profile_id);
    *detailed_artist_body.get_mut("path").unwrap() = json!(root_folder_path);
    *detailed_artist_body.get_mut("tags").unwrap() = json!(tags);
    debug!("Edit artist body: {detailed_artist_body:?}");
    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Put,
        Some(detailed_artist_body),
        Some(format!("/{artist_id}")),
        None,
      )
      .await;
    self
      .handle_request::<Value, ()>(request_props, |_, _| ())
      .await
  }
  async fn toggle_lidarr_album_monitoring(
    &mut self,
    artist_id_album_number_tuple: (i64, i64),
  ) -> Result<()> {
    let event = LidarrEvent::ToggleAlbumMonitoring(artist_id_album_number_tuple);
    let (artist_id, album_id) = artist_id_album_number_tuple;
    let detail_event = LidarrEvent::GetArtistDetails(artist_id);
    info!("Toggling album monitoring for album {album_id} in artist with ID: {artist_id}");
    info!("Fetching artist details for artist with ID: {artist_id}");
    let request_props = self
      .request_props_from(
        detail_event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{artist_id}")),
        None,
      )
      .await;
    let mut response = String::new();
    self
      .handle_request::<(), Value>(request_props, |detailed_artist_body, _| {
        response = detailed_artist_body.to_string()
      })
      .await?;
    info!("Constructing toggle album monitoring body");
    match serde_json::from_str::<Value>(&response) {
      Ok(mut detailed_artist_body) => {
        let monitored = detailed_artist_body
          .get("albums")
          .unwrap()
          .as_array()
          .unwrap()
          .iter()
          .find(|album| album["id"] == album_id)
          .unwrap()
          .get("monitored")
          .unwrap()
          .as_bool()
          .unwrap();
        *detailed_artist_body
          .get_mut("albums")
          .unwrap()
          .as_array_mut()
          .unwrap()
          .iter_mut()
          .find(|album| album["id"] == album_id)
          .unwrap()
          .get_mut("monitored")
          .unwrap() = json!(!monitored);
        debug!("Toggle album monitoring body: {detailed_artist_body:?}");
        let request_props = self
          .request_props_from(
            event,
            RequestMethod::Put,
            Some(detailed_artist_body),
            Some(format!("/{artist_id}")),
            None,
          )
          .await;
        self
          .handle_request::<Value, ()>(request_props, |_, _| ())
          .await
      }
      Err(_) => {
        warn!("Request for detailed artist body was interrupted");
        Ok(())
      }
    }
  }
  async fn get_all_lidarr_indexer_settings(&mut self) -> Result<IndexerSettings> {
    info!("Fetching Lidarr indexer settings");
    let event = LidarrEvent::GetAllIndexerSettings;
    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;
    self
      .handle_request::<(), IndexerSettings>(request_props, |indexer_settings, mut app| {
        if app.data.lidarr_data.indexer_settings.is_none() {
          app.data.lidarr_data.indexer_settings = Some(indexer_settings);
        } else {
          debug!("Indexer Settings are being modified. Ignoring update...");
        }
      })
      .await
  }
  async fn get_lidarr_healthcheck(&mut self) -> Result<()> {
    info!("Performing Lidarr health check");
    let event = LidarrEvent::HealthCheck;
    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;
    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }
  async fn get_lidarr_blocklist(&mut self) -> Result<BlocklistResponse> {
    info!("Fetching Lidarr blocklist");
    let event = LidarrEvent::GetBlocklist;
    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;
    self
      .handle_request::<(), BlocklistResponse>(request_props, |blocklist_resp, mut app| {
        if !matches!(
          app.get_current_route(),
          Route::Lidarr(ActiveLidarrBlock::BlocklistSortPrompt, _)
        ) {
          let mut blocklist_vec: Vec<BlocklistItem> = blocklist_resp
            .records
            .into_iter()
            .map(|item| {
              if let Some(artist) = app
                .data
                .lidarr_data
                .artists
                .items
                .iter()
                .find(|it| it.id == item.artist_id)
              {
                BlocklistItem {
                  artist_name: Some(artist.artist_name.text.clone()),
                  ..item
                }
              } else {
                item
              }
            })
            .collect();
          blocklist_vec.sort_by(|a, b| a.id.cmp(&b.id));
          app.data.lidarr_data.blocklist.set_items(blocklist_vec);
          app.data.lidarr_data.blocklist.apply_sorting_toggle(false);
        }
      })
      .await
  }
  async fn get_lidarr_downloads(&mut self) -> Result<DownloadsResponse> {
    info!("Fetching Lidarr downloads");
    let event = LidarrEvent::GetDownloads;
    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;
    self
      .handle_request::<(), DownloadsResponse>(request_props, |queue_response, mut app| {
        app
          .data
          .lidarr_data
          .downloads
          .set_items(queue_response.records);
      })
      .await
  }
  async fn get_tracks(&mut self, artist_id: i64) -> Result<Vec<Track>> {
    let event = LidarrEvent::GetTracks(artist_id);
    info!("Fetching tracks for Lidarr artist with ID: {artist_id}");
    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("artistId={artist_id}")),
      )
      .await;
    self
      .handle_request::<(), Vec<Track>>(request_props, |mut track_vec, mut app| {
        track_vec.sort_by(|a, b| a.id.cmp(&b.id));
        if !matches!(
          app.get_current_route(),
          Route::Lidarr(ActiveLidarrBlock::TracksSortPrompt, _)
        ) {
          if app.data.lidarr_data.album_details_modal.is_none() {
            app.data.lidarr_data.album_details_modal = Some(AlbumDetailsModal::default());
          }
          let album_tracks_vec = if !app.data.lidarr_data.albums.is_empty() {
            let album_id = app.data.lidarr_data.albums.current_selection().id;
            track_vec
              .into_iter()
              .filter(|track| track.album_id == album_id)
              .collect()
          } else {
            track_vec
          };
          app
            .data
            .lidarr_data
            .album_details_modal
            .as_mut()
            .unwrap()
            .tracks
            .set_items(album_tracks_vec);
          app
            .data
            .lidarr_data
            .album_details_modal
            .as_mut()
            .unwrap()
            .tracks
            .apply_sorting_toggle(false);
        }
      })
      .await
  }
  async fn get_track_files(&mut self, artist_id: i64) -> Result<Vec<TrackFile>> {
    let event = LidarrEvent::GetTrackFiles(artist_id);
    info!("Fetching tracks files for Lidarr artist with ID: {artist_id}");
    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("artistId={artist_id}")),
      )
      .await;
    self
      .handle_request::<(), Vec<TrackFile>>(request_props, |track_file_vec, mut app| {
        if app.data.lidarr_data.album_details_modal.is_none() {
          app.data.lidarr_data.album_details_modal = Some(AlbumDetailsModal::default());
        }
        app
          .data
          .lidarr_data
          .album_details_modal
          .as_mut()
          .unwrap()
          .track_files
          .set_items(track_file_vec);
      })
      .await
  }
  async fn get_lidarr_track_history(&mut self, track_id: i64) -> Result<LidarrHistoryWrapper> {
    info!("Fetching Lidarr history for track with ID: {track_id}");
    let event = LidarrEvent::GetTrackHistory(track_id);
    let params = format!("trackId={track_id}&pageSize=1000&sortDirection=descending&sortKey=date");
    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, Some(params))
      .await;
    self
      .handle_request::<(), LidarrHistoryWrapper>(request_props, |history_response, mut app| {
        if app.data.lidarr_data.album_details_modal.is_none() {
          app.data.lidarr_data.album_details_modal = Some(AlbumDetailsModal::default());
        }
        if app
          .data
          .lidarr_data
          .album_details_modal
          .as_ref()
          .unwrap()
          .track_details_modal
          .is_none()
        {
          app
            .data
            .lidarr_data
            .album_details_modal
            .as_mut()
            .unwrap()
            .track_details_modal = Some(TrackDetailsModal::default());
        }
        let mut history_vec = history_response.records;
        history_vec.sort_by(|a, b| a.id.cmp(&b.id));
        app
          .data
          .lidarr_data
          .album_details_modal
          .as_mut()
          .unwrap()
          .track_details_modal
          .as_mut()
          .unwrap()
          .track_history
          .set_items(history_vec);
        app
          .data
          .lidarr_data
          .album_details_modal
          .as_mut()
          .unwrap()
          .track_details_modal
          .as_mut()
          .unwrap()
          .track_history
          .apply_sorting_toggle(false);
      })
      .await
  }
  async fn get_track_details(&mut self, track_id: i64) -> Result<Track> {
    info!("Fetching Lidarr track details");
    let event = LidarrEvent::GetTrackDetails(track_id);
    info!("Fetching track details for track with ID: {track_id}");
    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{track_id}")),
        None,
      )
      .await;
    
    self.handle_request::<(), Track>(request_props, |track_response, mut app| {                
      if app.cli_mode {                    
        app.data.lidarr_data.album_details_modal = Some(AlbumDetailsModal::default());                
      }                
      
      if app                    
        .data                    
        .lidarr_data                    
        .album_details_modal                    
        .as_mut()                    
        .expect("Album details modal is empty")                    
        .track_details_modal                    
        .is_none()                
      {                    
        app                        
          .data                        
          .lidarr_data                        
          .album_details_modal                        
          .as_mut()                        
          .unwrap()                        
          .track_details_modal = Some(TrackDetailsModal::default());                
      }                
      let Track {                    
        id,                    
        title,                    
        track_number,                    
        has_file,                    
        track_file,                    
        ..                
      } = track_response;                
      let status = get_track_status(has_file, &app.data.lidarr_data.downloads.items, id);                
      let track_details_modal = app                    
        .data                    
        .lidarr_data                    
        .album_details_modal                    
        .as_mut()                    
        .unwrap()                    
        .track_details_modal                    
        .as_mut()                    
        .unwrap();                
      track_details_modal.track_details = ScrollableText::with_string(formatdoc!(                    
        "            
        Title: {}            
        Track Number: {track_number}            
        Status: {status}",
        title,                
      ));                
      if let Some(file) = track_file {                    
        let size = convert_to_gb(file.size);                    
        track_details_modal.file_details = formatdoc!(                        
          "            
          Relative Path: {}            
          Absolute Path: {}            
          Size: {size:.2} GB            
          Date Added: {}",                        
          file.relative_path,                        
          file.path,
          file.date_added,                    
        );                    
        if let Some(media_info) = file.media_info {                        
          track_details_modal.audio_details = formatdoc!(                            
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
          track_details_modal.video_details = formatdoc!(                            
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
            media_info.video_codec.unwrap_or_default(),                            
            media_info.video_fps.as_f64().unwrap(),                            
            media_info.resolution,                            
            media_info.scan_type,                            
            media_info.run_time,                            
            media_info.subtitles.unwrap_or_default()                        
          );                    
        }                
      };            
    }).await
  }
  async fn get_lidarr_host_config(&mut self) -> Result<HostConfig> {
    info!("Fetching Lidarr host config");
    let event = LidarrEvent::GetHostConfig;
    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;
    self
      .handle_request::<(), HostConfig>(request_props, |_, _| ())
      .await
  }
  async fn get_lidarr_history(&mut self, events: u64) -> Result<LidarrHistoryWrapper> {
    info!("Fetching all Lidarr history events");
    let event = LidarrEvent::GetHistory(events);
    let params = format!("pageSize={}&sortDirection=descending&sortKey=date", events);
    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, Some(params))
      .await;
    self
      .handle_request::<(), LidarrHistoryWrapper>(request_props, |history_response, mut app| {
        if !matches!(
          app.get_current_route(),
          Route::Lidarr(ActiveLidarrBlock::HistorySortPrompt, _)
        ) {
          let mut history_vec = history_response.records;
          history_vec.sort_by(|a, b| a.id.cmp(&b.id));
          app.data.lidarr_data.history.set_items(history_vec);
          app.data.lidarr_data.history.apply_sorting_toggle(false);
        }
      })
      .await
  }
  async fn get_lidarr_indexers(&mut self) -> Result<Vec<Indexer>> {
    info!("Fetching Lidarr indexers");
    let event = LidarrEvent::GetIndexers;
    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;
    self
      .handle_request::<(), Vec<Indexer>>(request_props, |indexers, mut app| {
        app.data.lidarr_data.indexers.set_items(indexers);
      })
      .await
  }
  async fn get_lidarr_metadata_profiles(&mut self) -> Result<Vec<Language>> {
    info!("Fetching Lidarr metadata profiles");
    let event = LidarrEvent::GetMetadataProfiles;
    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;
    self
      .handle_request::<(), Vec<Language>>(request_props, |metadata_profiles_vec, mut app| {
        app.data.lidarr_data.metadata_profiles_map = metadata_profiles_vec
          .into_iter()
          .map(|metadata| (metadata.id, metadata.name))
          .collect();
      })
      .await
  }
  async fn get_lidarr_logs(&mut self, events: u64) -> Result<LogResponse> {
    info!("Fetching Lidarr logs");
    let event = LidarrEvent::GetLogs(events);
    let params = format!("pageSize={}&sortDirection=descending&sortKey=time", events);
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
        app.data.lidarr_data.logs.set_items(log_lines);
        app.data.lidarr_data.logs.scroll_to_bottom();
      })
      .await
  }
  async fn get_lidarr_diskspace(&mut self) -> Result<Vec<DiskSpace>> {
    info!("Fetching Lidarr disk space");
    let event = LidarrEvent::GetDiskSpace;
    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;
    self
      .handle_request::<(), Vec<DiskSpace>>(request_props, |disk_space_vec, mut app| {
        app.data.lidarr_data.disk_space_vec = disk_space_vec;
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
  async fn get_queued_lidarr_events(&mut self) -> Result<Vec<QueueEvent>> {
    info!("Fetching Lidarr queued events");
    let event = LidarrEvent::GetQueuedEvents;
    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;
    self
      .handle_request::<(), Vec<QueueEvent>>(request_props, |queued_events_vec, mut app| {
        app
          .data
          .lidarr_data
          .queued_events
          .set_items(queued_events_vec);
      })
      .await
  }
  async fn get_lidarr_root_folders(&mut self) -> Result<Vec<RootFolder>> {
    info!("Fetching Lidarr root folders");
    let event = LidarrEvent::GetRootFolders;
    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;
    self
      .handle_request::<(), Vec<RootFolder>>(request_props, |root_folders, mut app| {
        app.data.lidarr_data.root_folders.set_items(root_folders);
      })
      .await
  }
  async fn get_track_releases(&mut self, track_id: i64) -> Result<Vec<LidarrRelease>> {
    let event = LidarrEvent::GetTrackReleases(track_id);
    info!("Fetching releases for track with ID: {track_id}");
    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("trackId={track_id}")),
      )
      .await;
    self
      .handle_request::<(), Vec<LidarrRelease>>(request_props, |release_vec, mut app| {
        if app.data.lidarr_data.album_details_modal.is_none() {
          app.data.lidarr_data.album_details_modal = Some(AlbumDetailsModal::default());
        }
        if app
          .data
          .lidarr_data
          .album_details_modal
          .as_mut()
          .unwrap()
          .track_details_modal
          .is_none()
        {
          app
            .data
            .lidarr_data
            .album_details_modal
            .as_mut()
            .unwrap()
            .track_details_modal = Some(TrackDetailsModal::default());
        }
        app
          .data
          .lidarr_data
          .album_details_modal
          .as_mut()
          .unwrap()
          .track_details_modal
          .as_mut()
          .unwrap()
          .track_releases
          .set_items(release_vec);
      })
      .await
  }
  async fn get_album_releases(
    &mut self,
    artist_album_id_tuple: (i64, i64),
  ) -> Result<Vec<LidarrRelease>> {
    let event = LidarrEvent::GetAlbumReleases(artist_album_id_tuple);
    let (artist_id, album_id) = artist_album_id_tuple;
    info!("Fetching releases for artist with ID: {artist_id} and album ID: {album_id}");
    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("artistId={}&albumId={}", artist_id, album_id)),
      )
      .await;
    self
      .handle_request::<(), Vec<LidarrRelease>>(request_props, |release_vec, mut app| {
        if app.data.lidarr_data.album_details_modal.is_none() {
          app.data.lidarr_data.album_details_modal = Some(AlbumDetailsModal::default());
        }
        let album_releases_vec = release_vec
          .into_iter()
          .filter(|release| release.full_album)
          .collect();
        app
          .data
          .lidarr_data
          .album_details_modal
          .as_mut()
          .unwrap()
          .album_releases
          .set_items(album_releases_vec);
      })
      .await
  }
  async fn get_lidarr_album_history(
    &mut self,
    artist_album_id_tuple: (i64, i64),
  ) -> Result<Vec<LidarrHistoryItem>> {
    let event = LidarrEvent::GetAlbumHistory(artist_album_id_tuple);
    let (artist_id, album_id) = artist_album_id_tuple;
    info!("Fetching history for artist with ID: {artist_id} and album ID: {album_id}");
    let params = format!("artistId={artist_id}&albumId={album_id}",);
    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, Some(params))
      .await;
    self
      .handle_request::<(), Vec<LidarrHistoryItem>>(request_props, |history_items, mut app| {
        if app.data.lidarr_data.album_details_modal.is_none() {
          app.data.lidarr_data.album_details_modal = Some(AlbumDetailsModal::default());
        }
        let mut history_vec = history_items;
        history_vec.sort_by(|a, b| a.id.cmp(&b.id));
        app
          .data
          .lidarr_data
          .album_details_modal
          .as_mut()
          .unwrap()
          .album_history
          .set_items(history_vec);
        app
          .data
          .lidarr_data
          .album_details_modal
          .as_mut()
          .unwrap()
          .album_history
          .apply_sorting_toggle(false);
      })
      .await
  }
  async fn get_lidarr_security_config(&mut self) -> Result<SecurityConfig> {
    info!("Fetching Lidarr security config");
    let event = LidarrEvent::GetSecurityConfig;
    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;
    self
      .handle_request::<(), SecurityConfig>(request_props, |_, _| ())
      .await
  }
  async fn get_artist_details(&mut self, artist_id: i64) -> Result<Artist> {
    info!("Fetching details for Lidarr artist with ID: {artist_id}");
    let event = LidarrEvent::GetArtistDetails(artist_id);
    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{artist_id}")),
        None,
      )
      .await;
    self
      .handle_request::<(), Artist>(request_props, |_, _| ())
      .await
  }
  async fn get_lidarr_artist_history(&mut self, artist_id: i64) -> Result<Vec<LidarrHistoryItem>> {
    info!("Fetching Lidarr artist history for artist with ID: {artist_id}");
    let event = LidarrEvent::GetArtistHistory(artist_id);
    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("artistId={artist_id}")),
      )
      .await;
    self
      .handle_request::<(), Vec<LidarrHistoryItem>>(request_props, |mut history_vec, mut app| {
        if app.data.lidarr_data.artist_history.is_none() {
          app.data.lidarr_data.artist_history = Some(StatefulTable::default());
        }
        if !matches!(
          app.get_current_route(),
          Route::Lidarr(ActiveLidarrBlock::ArtistHistorySortPrompt, _)
        ) {
          history_vec.sort_by(|a, b| a.id.cmp(&b.id));
          app
            .data
            .lidarr_data
            .artist_history
            .as_mut()
            .unwrap()
            .set_items(history_vec);
          app
            .data
            .lidarr_data
            .artist_history
            .as_mut()
            .unwrap()
            .apply_sorting_toggle(false);
        }
      })
      .await
  }
  async fn list_artists(&mut self) -> Result<Vec<Artist>> {
    info!("Fetching Lidarr library");
    let event = LidarrEvent::ListArtists;
    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;
    self
      .handle_request::<(), Vec<Artist>>(request_props, |mut artist_vec, mut app| {
        if !matches!(
          app.get_current_route(),
          Route::Lidarr(ActiveLidarrBlock::ArtistSortPrompt, _)
        ) {
          artist_vec.sort_by(|a, b| a.id.cmp(&b.id));
          app.data.lidarr_data.artists.set_items(artist_vec);
          app.data.lidarr_data.artists.apply_sorting_toggle(false);
        }
      })
      .await
  }
  async fn get_lidarr_status(&mut self) -> Result<SystemStatus> {
    info!("Fetching Lidarr system status");
    let event = LidarrEvent::GetStatus;
    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;
    self
      .handle_request::<(), SystemStatus>(request_props, |system_status, mut app| {
        app.data.lidarr_data.version = system_status.version;
        app.data.lidarr_data.start_time = system_status.start_time;
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
  async fn get_lidarr_tasks(&mut self) -> Result<Vec<LidarrTask>> {
    info!("Fetching Lidarr tasks");
    let event = LidarrEvent::GetTasks;
    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;
    self
      .handle_request::<(), Vec<LidarrTask>>(request_props, |tasks_vec, mut app| {
        app.data.lidarr_data.tasks.set_items(tasks_vec);
      })
      .await
  }
  async fn get_lidarr_updates(&mut self) -> Result<Vec<Update>> {
    info!("Fetching Lidarr updates");
    let event = LidarrEvent::GetUpdates;
    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;
    self.handle_request::<(), Vec<Update>>(request_props, |updates_vec, mut app| {               
      let latest_installed = if updates_vec.iter()                    
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
            "{} - {} {install_status}\n
            {}",                         
            update.version,                  
            update.release_date,                   
            "-".repeat(200)                  
          );                    
          if let Some(new_changes) = update.changes.new {         
            let changes = vec_to_bullet_points(new_changes);           
            update_info = formatdoc!(                  
              "{update_info}\n
              New:\n
              {changes}"                           
            )                      
          }                    
          if let Some(fixes) = update.changes.fixed {    
            let fixes = vec_to_bullet_points(fixes);           
            update_info = formatdoc!(                        
              "{update_info}\n
              Fixed:\n
              {fixes}"                            
            );                  
          }                      
          update_info            
        })                 
        .reduce(|version_1, version_2| format!("{version_1}\n\n\n{version_2}"))       
        .unwrap();          
      app.data.lidarr_data.updates = ScrollableText::with_string(formatdoc!(        
        "The latest version of Lidarr is {latest_installed} installed\n
        \n
        {updates}"
      ));          
    }).await
  }
  async fn mark_lidarr_history_item_as_failed(&mut self, history_item_id: i64) -> Result<Value> {
    info!("Marking the Lidarr history item with ID: {history_item_id} as 'failed'");
    let event = LidarrEvent::MarkHistoryItemAsFailed(history_item_id);
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
  async fn search_lidarr_artist(&mut self, query: String) -> Result<Vec<AddArtistSearchResult>> {
    info!("Searching for specific Lidarr artist");
    let event = LidarrEvent::SearchNewArtist(String::new());
    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("term={}", encode(&query))),
      )
      .await;
    self
      .handle_request::<(), Vec<AddArtistSearchResult>>(request_props, |mut artist_vec, mut app| {
        if artist_vec.is_empty() {
          app.pop_and_push_navigation_stack(ActiveLidarrBlock::AddArtistEmptySearchResults.into());
        } else if let Some(add_searched_artists) = app.data.lidarr_data.add_searched_artist.as_mut()
        {
          add_searched_artists.set_items(artist_vec);
        } else {
          let mut add_searched_artists = StatefulTable::default();
          add_searched_artists.set_items(artist_vec);
          app.data.lidarr_data.add_searched_artist = Some(add_searched_artists);
        }
      })
      .await
  }
  async fn start_lidarr_task(&mut self, task_name: LidarrTaskName) -> Result<Value> {
    info!("Starting Lidarr task: {task_name}");
    let event = LidarrEvent::StartTask(task_name);
    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Post,
        Some(CommandBody {
          name: task_name.to_string(),
        }),
        None,
        None,
      )
      .await;
    self
      .handle_request::<CommandBody, Value>(request_props, |_, _| ())
      .await
  }
  async fn test_lidarr_indexer(&mut self, indexer_id: i64) -> Result<Value> {
    info!("Testing Lidarr indexer with ID: {indexer_id}");
    let detail_event = LidarrEvent::GetIndexers;
    let event = LidarrEvent::TestIndexer(indexer_id);
    let request_props = self
      .request_props_from(
        detail_event,
        RequestMethod::Post,
        Some(json!({ "id": indexer_id })),
        None,
        None,
      )
      .await;

    let mut test_body: Value = Value::default();

    self
      .handle_request::<Value, Value>(request_props, |detailed_indexer_body, _| {
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
          app.data.lidarr_data.indexer_test_errors = Some(
            test_results.as_array().unwrap()[0]
              .get("errorMessage")
              .unwrap()
              .to_string(),
          );
        } else {
          app.data.lidarr_data.indexer_test_errors = Some(String::new());
        };
      })
      .await
  }
  async fn test_all_lidarr_indexers(&mut self) -> Result<Vec<IndexerTestResult>> {
    info!("Testing all Lidarr indexers");
    let event = LidarrEvent::TestAllIndexers;

    let mut request_props = self
      .request_props_from(event, RequestMethod::Post, None, None, None)
      .await;
    request_props.ignore_status_code = true;

    self
      .handle_request::<(), Vec<IndexerTestResult>>(request_props, |test_results, mut app| {
        let mut test_all_indexer_results = StatefulTable::default();
        let indexers = app.data.lidarr_data.indexers.items.clone();
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
        app.data.lidarr_data.indexer_test_all_results = Some(test_all_indexer_results);
      })
      .await
  }

  async fn toggle_lidarr_track_monitoring(&mut self, track_id: i64) -> Result<()> {
    info!("Toggling track monitoring for track with ID: {track_id}");
    let event = LidarrEvent::ToggleTrackMonitoring(track_id);
    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Post,
        Some(MonitorAlbumBody {
          album_ids: vec![track_id],
          monitored: true,
        }),
        None,
        None,
      )
      .await;
    self
      .handle_request::<MonitorAlbumBody, ()>(request_props, |_, _| ())
      .await
  }
  async fn trigger_automatic_album_search(
    &mut self,
    artist_album_id_tuple: (i64, i64),
  ) -> Result<Value> {
    let event = LidarrEvent::TriggerAutomaticAlbumSearch(artist_album_id_tuple);
    let (artist_id, album_id) = artist_album_id_tuple;
    info!(
      "Triggering automatic album search for artist with ID: {artist_id} and album ID: {album_id}"
    );
    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Post,
        Some(LidarrCommandBody {
          name: LidarrTaskName::RefreshArtists.to_string(),
          artist_id: Some(artist_id),
          album_ids: Some(vec![album_id]),
        }),
        None,
        None,
      )
      .await;
    self
      .handle_request::<LidarrCommandBody, Value>(request_props, |_, _| ())
      .await
  }
  async fn trigger_automatic_artist_search(&mut self, artist_id: i64) -> Result<Value> {
    info!("Triggering automatic artist search for artist with ID: {artist_id}");
    let event = LidarrEvent::TriggerAutomaticArtistSearch(artist_id);
    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Post,
        Some(LidarrCommandBody {
          name: LidarrTaskName::RefreshArtists.to_string(),
          artist_id: Some(artist_id),
          ..LidarrCommandBody::default()
        }),
        None,
        None,
      )
      .await;
    self
      .handle_request::<LidarrCommandBody, Value>(request_props, |_, _| ())
      .await
  }
  async fn trigger_automatic_track_search(&mut self, track_id: i64) -> Result<Value> {
    info!("Triggering automatic track search for track with ID: {track_id}");
    let event = LidarrEvent::TriggerAutomaticTrackSearch(track_id);
    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Post,
        Some(MonitorAlbumBody {
          album_ids: vec![track_id],
          monitored: true,
        }),
        None,
        None,
      )
      .await;
    self
      .handle_request::<MonitorAlbumBody, Value>(request_props, |_, _| ())
      .await
  }
  async fn update_all_artists(&mut self) -> Result<Value> {
    info!("Updating all artists");
    let event = LidarrEvent::UpdateAllArtists;
    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Post,
        Some(CommandBody {
          name: LidarrTaskName::RefreshArtists.to_string(),
          ..CommandBody::default()
        }),
        None,
        None,
      )
      .await;
    self
      .handle_request::<CommandBody, Value>(request_props, |_, _| ())
      .await
  }
  async fn update_and_scan_artist(&mut self, artist_id: i64) -> Result<Value> {
    info!("Updating and scanning artist with ID: {artist_id}");
    let event = LidarrEvent::UpdateAndScanArtist(artist_id);
    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Post,
        Some(LidarrCommandBody {
          name: LidarrTaskName::RefreshArtists.to_string(),
          artist_id: Some(artist_id),
          ..LidarrCommandBody::default()
        }),
        None,
        None,
      )
      .await;
    self
      .handle_request::<LidarrCommandBody, Value>(request_props, |_, _| ())
      .await
  }
  async fn update_lidarr_downloads(&mut self) -> Result<Value> {
    info!("Updating Lidarr downloads");
    let event = LidarrEvent::UpdateDownloads;
    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Post,
        Some(CommandBody {
          name: LidarrTaskName::RefreshMonitoredDownloads.to_string(),
          ..CommandBody::default()
        }),
        None,
        None,
      )
      .await;
    self
      .handle_request::<CommandBody, Value>(request_props, |_, _| ())
      .await
  }
  async fn extract_and_add_lidarr_tag_ids_vec(&mut self, edit_tags: &str) -> Vec<i64> {
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
fn get_track_status(has_file: bool, downloads: &[DownloadRecord], album_id: i64) -> String {
  if has_file {
    "Downloaded".to_owned()
  } else if downloads
    .iter()
    .any(|d| d.album_id == Some(Number::from(album_id)))
  {
    "Downloading".to_owned()
  } else {
    "Missing".to_owned()
  }
}
