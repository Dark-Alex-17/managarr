use anyhow::Result;
use std::fmt::Debug;

use indoc::formatdoc;
use log::{debug, info};
use serde_json::{json, Value};
use urlencoding::encode;

use crate::models::radarr_models::{
  AddMovieBody, AddMovieSearchResult, BlocklistResponse, Collection, Credit, CreditType,
  DeleteMovieParams, DownloadRecord, DownloadsResponse, EditCollectionParams, EditMovieParams,
  IndexerSettings, IndexerTestResult, Movie, MovieCommandBody, MovieHistoryItem, RadarrRelease,
  RadarrReleaseDownloadBody, RadarrSerdeable, RadarrTask, RadarrTaskName, SystemStatus,
};
use crate::models::servarr_data::modals::IndexerTestResultModalItem;
use crate::models::servarr_data::radarr::modals::MovieDetailsModal;
use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
use crate::models::servarr_models::{
  AddRootFolderBody, CommandBody, DiskSpace, EditIndexerParams, HostConfig, Indexer, LogResponse,
  QualityProfile, QueueEvent, RootFolder, SecurityConfig, Tag, Update,
};
use crate::models::stateful_table::StatefulTable;
use crate::models::{HorizontallyScrollableText, Route, Scrollable, ScrollableText};
use crate::network::{Network, NetworkEvent, RequestMethod};
use crate::utils::{convert_runtime, convert_to_gb};

use super::NetworkResource;

#[cfg(test)]
#[path = "radarr_network_tests.rs"]
mod radarr_network_tests;

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
  GetDownloads,
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
      RadarrEvent::GetDownloads | RadarrEvent::DeleteDownload(_) => "/queue",
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
      | RadarrEvent::DeleteMovie(_) => "/movie",
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
      RadarrEvent::GetDownloads => self.get_radarr_downloads().await.map(RadarrSerdeable::from),
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

  async fn add_movie(&mut self, mut add_movie_body: AddMovieBody) -> Result<Value> {
    info!("Adding new movie to Radarr");
    let event = RadarrEvent::AddMovie(AddMovieBody::default());
    if let Some(tag_input_str) = add_movie_body.tag_input_string.as_ref() {
      let tag_ids_vec = self.extract_and_add_radarr_tag_ids_vec(tag_input_str).await;
      add_movie_body.tags = tag_ids_vec;
    }

    debug!("Add movie body: {add_movie_body:?}");

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(add_movie_body), None, None)
      .await;

    self
      .handle_request::<AddMovieBody, Value>(request_props, |_, _| ())
      .await
  }

  async fn add_radarr_root_folder(
    &mut self,
    add_root_folder_body: AddRootFolderBody,
  ) -> Result<Value> {
    info!("Adding new root folder to Radarr");
    let event = RadarrEvent::AddRootFolder(AddRootFolderBody::default());

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

  async fn add_radarr_tag(&mut self, tag: String) -> Result<Tag> {
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

  async fn delete_radarr_tag(&mut self, id: i64) -> Result<()> {
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

  async fn clear_radarr_blocklist(&mut self) -> Result<()> {
    info!("Clearing Radarr blocklist");
    let event = RadarrEvent::ClearBlocklist;

    let ids = self
      .app
      .lock()
      .await
      .data
      .radarr_data
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

  async fn delete_radarr_blocklist_item(&mut self, blocklist_item_id: i64) -> Result<()> {
    let event = RadarrEvent::DeleteBlocklistItem(blocklist_item_id);

    info!("Deleting Radarr blocklist item for item with id: {blocklist_item_id}");

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

  async fn delete_radarr_download(&mut self, download_id: i64) -> Result<()> {
    let event = RadarrEvent::DeleteDownload(download_id);
    info!("Deleting Radarr download for download with id: {download_id}");

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

  async fn delete_radarr_indexer(&mut self, indexer_id: i64) -> Result<()> {
    let event = RadarrEvent::DeleteIndexer(indexer_id);
    info!("Deleting Radarr indexer for indexer with id: {indexer_id}");

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

  async fn delete_movie(&mut self, delete_movie_params: DeleteMovieParams) -> Result<()> {
    let event = RadarrEvent::DeleteMovie(DeleteMovieParams::default());
    let DeleteMovieParams {
      id,
      delete_movie_files,
      add_list_exclusion,
    } = delete_movie_params;
    info!("Deleting Radarr movie with ID: {id} with deleteFiles={delete_movie_files} and addImportExclusion={add_list_exclusion}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Delete,
        None::<()>,
        Some(format!("/{id}")),
        Some(format!(
          "deleteFiles={delete_movie_files}&addImportExclusion={add_list_exclusion}"
        )),
      )
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }

  async fn delete_radarr_root_folder(&mut self, root_folder_id: i64) -> Result<()> {
    let event = RadarrEvent::DeleteRootFolder(root_folder_id);
    info!("Deleting Radarr root folder for folder with id: {root_folder_id}");

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

  async fn download_radarr_release(&mut self, params: RadarrReleaseDownloadBody) -> Result<Value> {
    let event = RadarrEvent::DownloadRelease(RadarrReleaseDownloadBody::default());
    info!("Downloading Radarr release with params: {params:?}");

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(params), None, None)
      .await;

    self
      .handle_request::<RadarrReleaseDownloadBody, Value>(request_props, |_, _| ())
      .await
  }

  async fn edit_all_radarr_indexer_settings(&mut self, params: IndexerSettings) -> Result<Value> {
    info!("Updating Radarr indexer settings");
    let event = RadarrEvent::EditAllIndexerSettings(IndexerSettings::default());

    debug!("Indexer settings body: {params:?}");

    let request_props = self
      .request_props_from(event, RequestMethod::Put, Some(params), None, None)
      .await;

    self
      .handle_request::<IndexerSettings, Value>(request_props, |_, _| {})
      .await
  }

  async fn edit_collection(&mut self, edit_collection_params: EditCollectionParams) -> Result<()> {
    info!("Editing Radarr collection");
    let detail_event = RadarrEvent::GetCollections;
    let event = RadarrEvent::EditCollection(EditCollectionParams::default());
    info!("Fetching collection details");
    let collection_id = edit_collection_params.collection_id;

    let request_props = self
      .request_props_from(
        detail_event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{collection_id}")),
        None,
      )
      .await;

    let mut response = String::new();

    self
      .handle_request::<(), Value>(request_props, |detailed_collection_body, _| {
        response = detailed_collection_body.to_string()
      })
      .await?;

    info!("Constructing edit collection body");

    let mut detailed_collection_body: Value = serde_json::from_str(&response)?;
    let (monitored, minimum_availability, quality_profile_id, root_folder_path, search_on_add) = {
      let monitored = edit_collection_params.monitored.unwrap_or_else(|| {
        detailed_collection_body["monitored"]
          .as_bool()
          .expect("Unable to deserialize 'monitored' bool")
      });
      let minimum_availability = edit_collection_params
        .minimum_availability
        .unwrap_or_else(|| {
          serde_json::from_value(detailed_collection_body["minimumAvailability"].clone())
            .expect("Unable to deserialize 'minimumAvailability'")
        })
        .to_string();
      let quality_profile_id = edit_collection_params
        .quality_profile_id
        .unwrap_or_else(|| {
          detailed_collection_body["qualityProfileId"]
            .as_i64()
            .expect("Unable to deserialize 'qualityProfileId'")
        });
      let root_folder_path = edit_collection_params.root_folder_path.unwrap_or_else(|| {
        detailed_collection_body["rootFolderPath"]
          .as_str()
          .expect("Unable to deserialize 'rootFolderPath'")
          .to_owned()
      });
      let search_on_add = edit_collection_params.search_on_add.unwrap_or_else(|| {
        detailed_collection_body["searchOnAdd"]
          .as_bool()
          .expect("Unable to deserialize 'searchOnAdd'")
      });

      (
        monitored,
        minimum_availability,
        quality_profile_id,
        root_folder_path,
        search_on_add,
      )
    };

    *detailed_collection_body.get_mut("monitored").unwrap() = json!(monitored);
    *detailed_collection_body
      .get_mut("minimumAvailability")
      .unwrap() = json!(minimum_availability);
    *detailed_collection_body
      .get_mut("qualityProfileId")
      .unwrap() = json!(quality_profile_id);
    *detailed_collection_body.get_mut("rootFolderPath").unwrap() = json!(root_folder_path);
    *detailed_collection_body.get_mut("searchOnAdd").unwrap() = json!(search_on_add);

    debug!("Edit collection body: {detailed_collection_body:?}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Put,
        Some(detailed_collection_body),
        Some(format!("/{collection_id}")),
        None,
      )
      .await;

    self
      .handle_request::<Value, ()>(request_props, |_, _| ())
      .await
  }

  async fn edit_radarr_indexer(
    &mut self,
    mut edit_indexer_params: EditIndexerParams,
  ) -> Result<()> {
    let detail_event = RadarrEvent::GetIndexers;
    let event = RadarrEvent::EditIndexer(EditIndexerParams::default());
    let id = edit_indexer_params.indexer_id;
    if let Some(tag_input_str) = edit_indexer_params.tag_input_string.as_ref() {
      let tag_ids_vec = self.extract_and_add_radarr_tag_ids_vec(tag_input_str).await;
      edit_indexer_params.tags = Some(tag_ids_vec);
    }
    info!("Updating Radarr indexer with ID: {id}");

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

  async fn edit_movie(&mut self, mut edit_movie_params: EditMovieParams) -> Result<()> {
    info!("Editing Radarr movie");
    let movie_id = edit_movie_params.movie_id;
    let detail_event = RadarrEvent::GetMovieDetails(movie_id);
    let event = RadarrEvent::EditMovie(EditMovieParams::default());
    if let Some(tag_input_str) = edit_movie_params.tag_input_string.as_ref() {
      let tag_ids_vec = self.extract_and_add_radarr_tag_ids_vec(tag_input_str).await;
      edit_movie_params.tags = Some(tag_ids_vec);
    }

    info!("Fetching movie details for movie with ID: {movie_id}");

    let request_props = self
      .request_props_from(
        detail_event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{movie_id}")),
        None,
      )
      .await;

    let mut response = String::new();

    self
      .handle_request::<(), Value>(request_props, |detailed_movie_body, _| {
        response = detailed_movie_body.to_string()
      })
      .await?;

    info!("Constructing edit movie body");

    let mut detailed_movie_body: Value = serde_json::from_str(&response)?;
    let (monitored, minimum_availability, quality_profile_id, root_folder_path, tags) = {
      let monitored = edit_movie_params.monitored.unwrap_or(
        detailed_movie_body["monitored"]
          .as_bool()
          .expect("Unable to deserialize 'monitored'"),
      );
      let minimum_availability = edit_movie_params
        .minimum_availability
        .unwrap_or_else(|| {
          serde_json::from_value(detailed_movie_body["minimumAvailability"].clone())
            .expect("Unable to deserialize 'minimumAvailability'")
        })
        .to_string();
      let quality_profile_id = edit_movie_params.quality_profile_id.unwrap_or_else(|| {
        detailed_movie_body["qualityProfileId"]
          .as_i64()
          .expect("Unable to deserialize 'qualityProfileId'")
      });
      let root_folder_path = edit_movie_params.root_folder_path.unwrap_or_else(|| {
        detailed_movie_body["path"]
          .as_str()
          .expect("Unable to deserialize 'path'")
          .to_owned()
      });
      let tags = if edit_movie_params.clear_tags {
        vec![]
      } else {
        edit_movie_params.tags.unwrap_or(
          detailed_movie_body["tags"]
            .as_array()
            .expect("Unable to deserialize 'tags'")
            .iter()
            .map(|item| item.as_i64().expect("Unable to deserialize tag ID"))
            .collect(),
        )
      };

      (
        monitored,
        minimum_availability,
        quality_profile_id,
        root_folder_path,
        tags,
      )
    };

    *detailed_movie_body.get_mut("monitored").unwrap() = json!(monitored);
    *detailed_movie_body.get_mut("minimumAvailability").unwrap() = json!(minimum_availability);
    *detailed_movie_body.get_mut("qualityProfileId").unwrap() = json!(quality_profile_id);
    *detailed_movie_body.get_mut("path").unwrap() = json!(root_folder_path);
    *detailed_movie_body.get_mut("tags").unwrap() = json!(tags);

    debug!("Edit movie body: {detailed_movie_body:?}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Put,
        Some(detailed_movie_body),
        Some(format!("/{movie_id}")),
        None,
      )
      .await;

    self
      .handle_request::<Value, ()>(request_props, |_, _| ())
      .await
  }

  async fn get_radarr_blocklist(&mut self) -> Result<BlocklistResponse> {
    info!("Fetching Radarr blocklist");
    let event = RadarrEvent::GetBlocklist;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), BlocklistResponse>(request_props, |blocklist_resp, mut app| {
        if !matches!(
          app.get_current_route(),
          Route::Radarr(ActiveRadarrBlock::BlocklistSortPrompt, _)
        ) {
          let mut blocklist_vec = blocklist_resp.records;
          blocklist_vec.sort_by(|a, b| a.id.cmp(&b.id));
          app.data.radarr_data.blocklist.set_items(blocklist_vec);
          app.data.radarr_data.blocklist.apply_sorting_toggle(false);
        }
      })
      .await
  }

  async fn get_collections(&mut self) -> Result<Vec<Collection>> {
    info!("Fetching Radarr collections");
    let event = RadarrEvent::GetCollections;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<Collection>>(request_props, |mut collections_vec, mut app| {
        if !matches!(
          app.get_current_route(),
          Route::Radarr(ActiveRadarrBlock::CollectionsSortPrompt, _)
        ) {
          collections_vec.sort_by(|a, b| a.id.cmp(&b.id));
          app.data.radarr_data.collections.set_items(collections_vec);
          app.data.radarr_data.collections.apply_sorting_toggle(false);
        }
      })
      .await
  }

  async fn get_credits(&mut self, movie_id: i64) -> Result<Vec<Credit>> {
    info!("Fetching Radarr movie credits");
    let event = RadarrEvent::GetMovieCredits(movie_id);

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("movieId={movie_id}")),
      )
      .await;

    self
      .handle_request::<(), Vec<Credit>>(request_props, |credit_vec, mut app| {
        let cast_vec: Vec<Credit> = credit_vec
          .iter()
          .filter(|&credit| credit.credit_type == CreditType::Cast)
          .cloned()
          .collect();
        let crew_vec: Vec<Credit> = credit_vec
          .iter()
          .filter(|&credit| credit.credit_type == CreditType::Crew)
          .cloned()
          .collect();

        if app.data.radarr_data.movie_details_modal.is_none() {
          app.data.radarr_data.movie_details_modal = Some(MovieDetailsModal::default());
        }

        app
          .data
          .radarr_data
          .movie_details_modal
          .as_mut()
          .unwrap()
          .movie_cast
          .set_items(cast_vec);
        app
          .data
          .radarr_data
          .movie_details_modal
          .as_mut()
          .unwrap()
          .movie_crew
          .set_items(crew_vec);
      })
      .await
  }

  async fn get_radarr_diskspace(&mut self) -> Result<Vec<DiskSpace>> {
    info!("Fetching Radarr disk space");
    let event = RadarrEvent::GetDiskSpace;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<DiskSpace>>(request_props, |disk_space_vec, mut app| {
        app.data.radarr_data.disk_space_vec = disk_space_vec;
      })
      .await
  }

  async fn get_radarr_downloads(&mut self) -> Result<DownloadsResponse> {
    info!("Fetching Radarr downloads");
    let event = RadarrEvent::GetDownloads;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), DownloadsResponse>(request_props, |queue_response, mut app| {
        app
          .data
          .radarr_data
          .downloads
          .set_items(queue_response.records);
      })
      .await
  }

  async fn get_radarr_host_config(&mut self) -> Result<HostConfig> {
    info!("Fetching Radarr host config");
    let event = RadarrEvent::GetHostConfig;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), HostConfig>(request_props, |_, _| ())
      .await
  }

  async fn get_radarr_indexers(&mut self) -> Result<Vec<Indexer>> {
    info!("Fetching Radarr indexers");
    let event = RadarrEvent::GetIndexers;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<Indexer>>(request_props, |indexers, mut app| {
        app.data.radarr_data.indexers.set_items(indexers);
      })
      .await
  }

  async fn get_all_radarr_indexer_settings(&mut self) -> Result<IndexerSettings> {
    info!("Fetching Radarr indexer settings");
    let event = RadarrEvent::GetAllIndexerSettings;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), IndexerSettings>(request_props, |indexer_settings, mut app| {
        if app.data.radarr_data.indexer_settings.is_none() {
          app.data.radarr_data.indexer_settings = Some(indexer_settings);
        } else {
          debug!("Indexer Settings are being modified. Ignoring update...");
        }
      })
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

  async fn get_radarr_logs(&mut self, events: u64) -> Result<LogResponse> {
    info!("Fetching Radarr logs");
    let event = RadarrEvent::GetLogs(events);

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

        app.data.radarr_data.logs.set_items(log_lines);
        app.data.radarr_data.logs.scroll_to_bottom();
      })
      .await
  }

  async fn get_movie_details(&mut self, movie_id: i64) -> Result<Movie> {
    info!("Fetching Radarr movie details");
    let event = RadarrEvent::GetMovieDetails(movie_id);

    info!("Fetching movie details for movie with ID: {movie_id}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{movie_id}")),
        None,
      )
      .await;

    self
      .handle_request::<(), Movie>(request_props, |movie_response, mut app| {
        let Movie {
          id,
          title,
          year,
          overview,
          path,
          studio,
          has_file,
          quality_profile_id,
          size_on_disk,
          genres,
          runtime,
          certification,
          ratings,
          movie_file,
          collection,
          ..
        } = movie_response;
        let (hours, minutes) = convert_runtime(runtime);
        let size = convert_to_gb(size_on_disk);
        let quality_profile = app
          .data
          .radarr_data
          .quality_profile_map
          .get_by_left(&quality_profile_id)
          .unwrap_or(&"".to_owned())
          .to_owned();
        let imdb_rating = if let Some(rating) = ratings.imdb {
          if let Some(value) = rating.value.as_f64() {
            format!("{:.1}", value)
          } else {
            String::new()
          }
        } else {
          String::new()
        };

        let tmdb_rating = if let Some(rating) = ratings.tmdb {
          if let Some(value) = rating.value.as_f64() {
            format!("{}%", (value * 10f64).ceil())
          } else {
            String::new()
          }
        } else {
          String::new()
        };

        let rotten_tomatoes_rating = if let Some(rating) = ratings.rotten_tomatoes {
          if let Some(value) = rating.value.as_u64() {
            format!("{}%", value)
          } else {
            String::new()
          }
        } else {
          String::new()
        };

        let status = get_movie_status(has_file, &app.data.radarr_data.downloads.items, id);
        let collection = collection.unwrap_or_default();

        let mut movie_details_modal = MovieDetailsModal {
          movie_details: ScrollableText::with_string(formatdoc!(
            "Title: {title}
            Year: {year}
            Runtime: {hours}h {minutes}m
            Rating: {}
            Collection: {}
            Status: {status}
            Description: {overview}
            TMDB: {tmdb_rating}
            IMDB: {imdb_rating}
            Rotten Tomatoes: {rotten_tomatoes_rating}
            Quality Profile: {quality_profile}
            Size: {size:.2} GB
            Path: {path}
            Studio: {studio}
            Genres: {}",
            certification.unwrap_or_default(),
            collection
              .title
              .as_ref()
              .unwrap_or(&String::new())
              .to_owned(),
            genres.join(", ")
          )),
          ..MovieDetailsModal::default()
        };

        if let Some(file) = movie_file {
          movie_details_modal.file_details = formatdoc!(
            "Relative Path: {}
              Absolute Path: {}
              Size: {size:.2} GB
              Date Added: {}",
            file.relative_path,
            file.path,
            file.date_added
          );

          if let Some(media_info) = file.media_info {
            movie_details_modal.audio_details = formatdoc!(
              "Bitrate: {}
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

            movie_details_modal.video_details = formatdoc!(
              "Bit Depth: {}
              Bitrate: {}
              Codec: {}
              FPS: {}
              Resolution: {}
              Scan Type: {}
              Runtime: {}",
              media_info.video_bit_depth,
              media_info.video_bitrate,
              media_info.video_codec,
              media_info.video_fps.as_f64().unwrap(),
              media_info.resolution,
              media_info.scan_type,
              media_info.run_time
            );
          }
        }

        app.data.radarr_data.movie_details_modal = Some(movie_details_modal);
      })
      .await
  }

  async fn get_movie_history(&mut self, movie_id: i64) -> Result<Vec<MovieHistoryItem>> {
    info!("Fetching Radarr movie history");
    let event = RadarrEvent::GetMovieHistory(movie_id);

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("movieId={movie_id}")),
      )
      .await;

    self
      .handle_request::<(), Vec<MovieHistoryItem>>(request_props, |movie_history_vec, mut app| {
        let mut reversed_movie_history_vec = movie_history_vec.to_vec();
        reversed_movie_history_vec.reverse();

        if app.data.radarr_data.movie_details_modal.is_none() {
          app.data.radarr_data.movie_details_modal = Some(MovieDetailsModal::default())
        }

        app
          .data
          .radarr_data
          .movie_details_modal
          .as_mut()
          .unwrap()
          .movie_history
          .set_items(reversed_movie_history_vec)
      })
      .await
  }

  async fn get_movies(&mut self) -> Result<Vec<Movie>> {
    info!("Fetching Radarr library");
    let event = RadarrEvent::GetMovies;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<Movie>>(request_props, |mut movie_vec, mut app| {
        if !matches!(
          app.get_current_route(),
          Route::Radarr(ActiveRadarrBlock::MoviesSortPrompt, _)
        ) {
          movie_vec.sort_by(|a, b| a.id.cmp(&b.id));
          app.data.radarr_data.movies.set_items(movie_vec);
          app.data.radarr_data.movies.apply_sorting_toggle(false);
        }
      })
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

  async fn get_queued_radarr_events(&mut self) -> Result<Vec<QueueEvent>> {
    info!("Fetching Radarr queued events");
    let event = RadarrEvent::GetQueuedEvents;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<QueueEvent>>(request_props, |queued_events_vec, mut app| {
        app
          .data
          .radarr_data
          .queued_events
          .set_items(queued_events_vec);
      })
      .await
  }

  async fn get_movie_releases(&mut self, movie_id: i64) -> Result<Vec<RadarrRelease>> {
    info!("Fetching releases for movie with ID: {movie_id}");
    let event = RadarrEvent::GetReleases(movie_id);

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("movieId={movie_id}")),
      )
      .await;

    self
      .handle_request::<(), Vec<RadarrRelease>>(request_props, |release_vec, mut app| {
        if app.data.radarr_data.movie_details_modal.is_none() {
          app.data.radarr_data.movie_details_modal = Some(MovieDetailsModal::default());
        }

        app
          .data
          .radarr_data
          .movie_details_modal
          .as_mut()
          .unwrap()
          .movie_releases
          .set_items(release_vec);
      })
      .await
  }

  async fn get_radarr_root_folders(&mut self) -> Result<Vec<RootFolder>> {
    info!("Fetching Radarr root folders");
    let event = RadarrEvent::GetRootFolders;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<RootFolder>>(request_props, |root_folders, mut app| {
        app.data.radarr_data.root_folders.set_items(root_folders);
      })
      .await
  }

  async fn get_radarr_security_config(&mut self) -> Result<SecurityConfig> {
    info!("Fetching Radarr security config");
    let event = RadarrEvent::GetSecurityConfig;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), SecurityConfig>(request_props, |_, _| ())
      .await
  }

  async fn get_radarr_status(&mut self) -> Result<SystemStatus> {
    info!("Fetching Radarr system status");
    let event = RadarrEvent::GetStatus;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), SystemStatus>(request_props, |system_status, mut app| {
        app.data.radarr_data.version = system_status.version;
        app.data.radarr_data.start_time = system_status.start_time;
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

  async fn get_radarr_tasks(&mut self) -> Result<Vec<RadarrTask>> {
    info!("Fetching Radarr tasks");
    let event = RadarrEvent::GetTasks;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<RadarrTask>>(request_props, |tasks_vec, mut app| {
        app.data.radarr_data.tasks.set_items(tasks_vec);
      })
      .await
  }

  async fn get_radarr_updates(&mut self) -> Result<Vec<Update>> {
    info!("Fetching Radarr updates");
    let event = RadarrEvent::GetUpdates;

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

        app.data.radarr_data.updates = ScrollableText::with_string(formatdoc!(
          "The latest version of Radarr is {latest_installed} installed
          
          {updates}"
        ));
      })
      .await
  }

  async fn search_movie(&mut self, query: String) -> Result<Vec<AddMovieSearchResult>> {
    info!("Searching for specific Radarr movie");
    let event = RadarrEvent::SearchNewMovie(String::new());

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
      .handle_request::<(), Vec<AddMovieSearchResult>>(request_props, |movie_vec, mut app| {
        if movie_vec.is_empty() {
          app.pop_and_push_navigation_stack(ActiveRadarrBlock::AddMovieEmptySearchResults.into());
        } else if let Some(add_searched_movies) = app.data.radarr_data.add_searched_movies.as_mut()
        {
          add_searched_movies.set_items(movie_vec);
        } else {
          let mut add_searched_movies = StatefulTable::default();
          add_searched_movies.set_items(movie_vec);
          app.data.radarr_data.add_searched_movies = Some(add_searched_movies);
        }
      })
      .await
  }

  async fn start_radarr_task(&mut self, task_name: RadarrTaskName) -> Result<Value> {
    let event = RadarrEvent::StartTask(task_name);

    info!("Starting Radarr task: {task_name}");

    let body = CommandBody {
      name: task_name.to_string(),
    };

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(body), None, None)
      .await;

    self
      .handle_request::<CommandBody, Value>(request_props, |_, _| ())
      .await
  }

  async fn test_radarr_indexer(&mut self, indexer_id: i64) -> Result<Value> {
    let detail_event = RadarrEvent::GetIndexers;
    let event = RadarrEvent::TestIndexer(indexer_id);
    info!("Testing Radarr indexer with ID: {indexer_id}");

    info!("Fetching indexer details for indexer with ID: {indexer_id}");

    let request_props = self
      .request_props_from(
        detail_event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{indexer_id}")),
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
          app.data.radarr_data.indexer_test_errors = Some(
            test_results.as_array().unwrap()[0]
              .get("errorMessage")
              .unwrap()
              .to_string(),
          );
        } else {
          app.data.radarr_data.indexer_test_errors = Some(String::new());
        };
      })
      .await
  }

  async fn test_all_radarr_indexers(&mut self) -> Result<Vec<IndexerTestResult>> {
    info!("Testing all Radarr indexers");
    let event = RadarrEvent::TestAllIndexers;

    let mut request_props = self
      .request_props_from(event, RequestMethod::Post, None, None, None)
      .await;
    request_props.ignore_status_code = true;

    self
      .handle_request::<(), Vec<IndexerTestResult>>(request_props, |test_results, mut app| {
        let mut test_all_indexer_results = StatefulTable::default();
        let indexers = app.data.radarr_data.indexers.items.clone();
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
        app.data.radarr_data.indexer_test_all_results = Some(test_all_indexer_results);
      })
      .await
  }

  async fn trigger_automatic_movie_search(&mut self, movie_id: i64) -> Result<Value> {
    let event = RadarrEvent::TriggerAutomaticSearch(movie_id);
    info!("Searching indexers for movie with ID: {movie_id}");
    let body = MovieCommandBody {
      name: "MoviesSearch".to_owned(),
      movie_ids: vec![movie_id],
    };

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(body), None, None)
      .await;

    self
      .handle_request::<MovieCommandBody, Value>(request_props, |_, _| ())
      .await
  }

  async fn update_all_movies(&mut self) -> Result<Value> {
    info!("Updating all movies");
    let event = RadarrEvent::UpdateAllMovies;
    let body = MovieCommandBody {
      name: "RefreshMovie".to_owned(),
      movie_ids: Vec::new(),
    };

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(body), None, None)
      .await;

    self
      .handle_request::<MovieCommandBody, Value>(request_props, |_, _| ())
      .await
  }

  async fn update_and_scan_movie(&mut self, movie_id: i64) -> Result<Value> {
    let event = RadarrEvent::UpdateAndScan(movie_id);
    info!("Updating and scanning movie with ID: {movie_id}");
    let body = MovieCommandBody {
      name: "RefreshMovie".to_owned(),
      movie_ids: vec![movie_id],
    };

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(body), None, None)
      .await;

    self
      .handle_request::<MovieCommandBody, Value>(request_props, |_, _| ())
      .await
  }

  async fn update_collections(&mut self) -> Result<Value> {
    info!("Updating collections");
    let event = RadarrEvent::UpdateCollections;
    let body = CommandBody {
      name: "RefreshCollections".to_owned(),
    };

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(body), None, None)
      .await;

    self
      .handle_request::<CommandBody, Value>(request_props, |_, _| ())
      .await
  }

  async fn update_radarr_downloads(&mut self) -> Result<Value> {
    info!("Updating Radarr downloads");
    let event = RadarrEvent::UpdateDownloads;
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

  async fn extract_and_add_radarr_tag_ids_vec(&mut self, edit_tags: &str) -> Vec<i64> {
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

fn get_movie_status(has_file: bool, downloads_vec: &[DownloadRecord], movie_id: i64) -> String {
  if !has_file {
    if let Some(download) = downloads_vec
      .iter()
      .find(|&download| download.movie_id == movie_id)
    {
      if download.status == "downloading" {
        return "Downloading".to_owned();
      }

      if download.status == "completed" {
        return "Awaiting Import".to_owned();
      }
    }

    return "Missing".to_owned();
  }

  "Downloaded".to_owned()
}
