use anyhow::{anyhow, Result};
use std::fmt::Debug;

use indoc::formatdoc;
use log::{debug, info, warn};
use serde_json::{json, Value};
use urlencoding::encode;

use crate::models::radarr_models::{
  AddMovieBody, AddMovieSearchResult, AddOptions, BlocklistResponse, Collection, CollectionMovie,
  CommandBody, Credit, CreditType, DeleteMovieParams, DiskSpace, DownloadRecord, DownloadsResponse,
  EditCollectionParams, EditIndexerParams, EditMovieParams, IndexerSettings, IndexerTestResult,
  Movie, MovieCommandBody, MovieHistoryItem, RadarrSerdeable, ReleaseDownloadBody, SystemStatus,
  Task, TaskName, Update,
};
use crate::models::servarr_data::radarr::modals::{
  AddMovieModal, EditCollectionModal, EditIndexerModal, EditMovieModal, IndexerTestResultModalItem,
  MovieDetailsModal,
};
use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
use crate::models::servarr_models::{
  AddRootFolderBody, HostConfig, Indexer, LogResponse, QualityProfile, QueueEvent, Release,
  RootFolder, SecurityConfig, Tag,
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
  AddMovie(Option<AddMovieBody>),
  AddRootFolder(Option<String>),
  AddTag(String),
  ClearBlocklist,
  DeleteBlocklistItem(Option<i64>),
  DeleteDownload(Option<i64>),
  DeleteIndexer(Option<i64>),
  DeleteMovie(Option<DeleteMovieParams>),
  DeleteRootFolder(Option<i64>),
  DeleteTag(i64),
  DownloadRelease(Option<ReleaseDownloadBody>),
  EditAllIndexerSettings(Option<IndexerSettings>),
  EditCollection(Option<EditCollectionParams>),
  EditIndexer(Option<EditIndexerParams>),
  EditMovie(Option<EditMovieParams>),
  GetBlocklist,
  GetCollections,
  GetDownloads,
  GetHostConfig,
  GetIndexers,
  GetAllIndexerSettings,
  GetLogs(Option<u64>),
  GetMovieCredits(Option<i64>),
  GetMovieDetails(Option<i64>),
  GetMovieHistory(Option<i64>),
  GetMovies,
  GetOverview,
  GetQualityProfiles,
  GetQueuedEvents,
  GetReleases(Option<i64>),
  GetRootFolders,
  GetSecurityConfig,
  GetStatus,
  GetTags,
  GetTasks,
  GetUpdates,
  HealthCheck,
  SearchNewMovie(Option<String>),
  StartTask(Option<TaskName>),
  TestIndexer(Option<i64>),
  TestAllIndexers,
  TriggerAutomaticSearch(Option<i64>),
  UpdateAllMovies,
  UpdateAndScan(Option<i64>),
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
      RadarrEvent::GetOverview => "/diskspace",
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

impl<'a, 'b> Network<'a, 'b> {
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
        .download_release(params)
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::EditAllIndexerSettings(params) => self
        .edit_all_indexer_settings(params)
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::EditCollection(params) => self
        .edit_collection(params)
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::EditIndexer(params) => {
        self.edit_indexer(params).await.map(RadarrSerdeable::from)
      }
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
      RadarrEvent::GetOverview => self.get_diskspace().await.map(RadarrSerdeable::from),
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
      RadarrEvent::GetTasks => self.get_tasks().await.map(RadarrSerdeable::from),
      RadarrEvent::GetUpdates => self.get_updates().await.map(RadarrSerdeable::from),
      RadarrEvent::HealthCheck => self
        .get_radarr_healthcheck()
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::SearchNewMovie(query) => {
        self.search_movie(query).await.map(RadarrSerdeable::from)
      }
      RadarrEvent::StartTask(task_name) => {
        self.start_task(task_name).await.map(RadarrSerdeable::from)
      }
      RadarrEvent::TestIndexer(indexer_id) => self
        .test_indexer(indexer_id)
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::TestAllIndexers => self.test_all_indexers().await.map(RadarrSerdeable::from),
      RadarrEvent::TriggerAutomaticSearch(movie_id) => self
        .trigger_automatic_search(movie_id)
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::UpdateAllMovies => self.update_all_movies().await.map(RadarrSerdeable::from),
      RadarrEvent::UpdateAndScan(movie_id) => self
        .update_and_scan(movie_id)
        .await
        .map(RadarrSerdeable::from),
      RadarrEvent::UpdateCollections => self.update_collections().await.map(RadarrSerdeable::from),
      RadarrEvent::UpdateDownloads => self.update_downloads().await.map(RadarrSerdeable::from),
    }
  }

  async fn add_movie(&mut self, add_movie_body_option: Option<AddMovieBody>) -> Result<Value> {
    info!("Adding new movie to Radarr");
    let event = RadarrEvent::AddMovie(None);
    let body = if let Some(add_movie_body) = add_movie_body_option {
      add_movie_body
    } else {
      let tags = self
        .app
        .lock()
        .await
        .data
        .radarr_data
        .add_movie_modal
        .as_ref()
        .unwrap()
        .tags
        .text
        .clone();
      let tag_ids_vec = self.extract_and_add_tag_ids_vec(tags).await;
      let mut app = self.app.lock().await;
      let AddMovieModal {
        root_folder_list,
        monitor_list,
        minimum_availability_list,
        quality_profile_list,
        ..
      } = app.data.radarr_data.add_movie_modal.as_ref().unwrap();
      let (tmdb_id, title) = if let Route::Radarr(active_radarr_block, _) = *app.get_current_route()
      {
        if active_radarr_block == ActiveRadarrBlock::CollectionDetails {
          let CollectionMovie { tmdb_id, title, .. } = app
            .data
            .radarr_data
            .collection_movies
            .current_selection()
            .clone();
          (tmdb_id, title.text)
        } else {
          let AddMovieSearchResult { tmdb_id, title, .. } = app
            .data
            .radarr_data
            .add_searched_movies
            .as_ref()
            .unwrap()
            .current_selection()
            .clone();
          (tmdb_id, title.text)
        }
      } else {
        let AddMovieSearchResult { tmdb_id, title, .. } = app
          .data
          .radarr_data
          .add_searched_movies
          .as_ref()
          .unwrap()
          .current_selection()
          .clone();
        (tmdb_id, title.text)
      };
      let quality_profile = quality_profile_list.current_selection();
      let quality_profile_id = *app
        .data
        .radarr_data
        .quality_profile_map
        .iter()
        .filter(|(_, value)| *value == quality_profile)
        .map(|(key, _)| key)
        .next()
        .unwrap();

      let path = root_folder_list.current_selection().path.clone();
      let monitor = monitor_list.current_selection().to_string();
      let minimum_availability = minimum_availability_list.current_selection().to_string();

      app.data.radarr_data.add_movie_modal = None;

      AddMovieBody {
        tmdb_id,
        title,
        root_folder_path: path,
        minimum_availability,
        monitored: true,
        quality_profile_id,
        tags: tag_ids_vec,
        add_options: AddOptions {
          monitor,
          search_for_movie: true,
        },
      }
    };

    debug!("Add movie body: {body:?}");

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(body), None, None)
      .await;

    self
      .handle_request::<AddMovieBody, Value>(request_props, |_, _| ())
      .await
  }

  async fn add_radarr_root_folder(&mut self, root_folder: Option<String>) -> Result<Value> {
    info!("Adding new root folder to Radarr");
    let event = RadarrEvent::AddRootFolder(None);
    let body = if let Some(path) = root_folder {
      AddRootFolderBody { path }
    } else {
      let mut app = self.app.lock().await;
      let path = app
        .data
        .radarr_data
        .edit_root_folder
        .as_ref()
        .unwrap()
        .text
        .clone();

      app.data.radarr_data.edit_root_folder = None;

      AddRootFolderBody { path }
    };

    debug!("Add root folder body: {body:?}");

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(body), None, None)
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

  async fn delete_radarr_blocklist_item(&mut self, blocklist_item_id: Option<i64>) -> Result<()> {
    let event = RadarrEvent::DeleteBlocklistItem(None);
    let id = if let Some(b_id) = blocklist_item_id {
      b_id
    } else {
      self
        .app
        .lock()
        .await
        .data
        .radarr_data
        .blocklist
        .current_selection()
        .id
    };

    info!("Deleting Radarr blocklist item for item with id: {id}");

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

  async fn delete_radarr_download(&mut self, download_id: Option<i64>) -> Result<()> {
    let event = RadarrEvent::DeleteDownload(None);
    let id = if let Some(dl_id) = download_id {
      dl_id
    } else {
      self
        .app
        .lock()
        .await
        .data
        .radarr_data
        .downloads
        .current_selection()
        .id
    };

    info!("Deleting Radarr download for download with id: {id}");

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

  async fn delete_radarr_indexer(&mut self, indexer_id: Option<i64>) -> Result<()> {
    let event = RadarrEvent::DeleteIndexer(None);
    let id = if let Some(i_id) = indexer_id {
      i_id
    } else {
      self
        .app
        .lock()
        .await
        .data
        .radarr_data
        .indexers
        .current_selection()
        .id
    };

    info!("Deleting Radarr indexer for indexer with id: {id}");

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

  async fn delete_movie(&mut self, delete_movie_params: Option<DeleteMovieParams>) -> Result<()> {
    let event = RadarrEvent::DeleteMovie(None);
    let (movie_id, delete_files, add_import_exclusion) = if let Some(params) = delete_movie_params {
      (
        params.id,
        params.delete_movie_files,
        params.add_list_exclusion,
      )
    } else {
      let (movie_id, _) = self.extract_movie_id(None).await;
      let delete_files = self.app.lock().await.data.radarr_data.delete_movie_files;
      let add_import_exclusion = self.app.lock().await.data.radarr_data.add_list_exclusion;

      (movie_id, delete_files, add_import_exclusion)
    };

    info!("Deleting Radarr movie with ID: {movie_id} with deleteFiles={delete_files} and addImportExclusion={add_import_exclusion}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Delete,
        None::<()>,
        Some(format!("/{movie_id}")),
        Some(format!(
          "deleteFiles={delete_files}&addImportExclusion={add_import_exclusion}"
        )),
      )
      .await;

    let resp = self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await;

    self
      .app
      .lock()
      .await
      .data
      .radarr_data
      .reset_delete_movie_preferences();

    resp
  }

  async fn delete_radarr_root_folder(&mut self, root_folder_id: Option<i64>) -> Result<()> {
    let event = RadarrEvent::DeleteRootFolder(None);
    let id = if let Some(rf_id) = root_folder_id {
      rf_id
    } else {
      self
        .app
        .lock()
        .await
        .data
        .radarr_data
        .root_folders
        .current_selection()
        .id
    };

    info!("Deleting Radarr root folder for folder with id: {id}");

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

  async fn download_release(&mut self, params: Option<ReleaseDownloadBody>) -> Result<Value> {
    let event = RadarrEvent::DownloadRelease(None);
    let body = if let Some(release_download_body) = params {
      info!("Downloading release with params: {release_download_body:?}");
      release_download_body
    } else {
      let (movie_id, _) = self.extract_movie_id(None).await;
      let (guid, title, indexer_id) = {
        let app = self.app.lock().await;
        let Release {
          guid,
          title,
          indexer_id,
          ..
        } = app
          .data
          .radarr_data
          .movie_details_modal
          .as_ref()
          .unwrap()
          .movie_releases
          .current_selection();

        (guid.clone(), title.clone(), *indexer_id)
      };

      info!("Downloading release: {title}");

      ReleaseDownloadBody {
        guid,
        indexer_id,
        movie_id,
      }
    };

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(body), None, None)
      .await;

    self
      .handle_request::<ReleaseDownloadBody, Value>(request_props, |_, _| ())
      .await
  }

  async fn edit_all_indexer_settings(&mut self, params: Option<IndexerSettings>) -> Result<Value> {
    info!("Updating Radarr indexer settings");
    let event = RadarrEvent::EditAllIndexerSettings(None);

    let body = if let Some(indexer_settings) = params {
      indexer_settings
    } else {
      self
        .app
        .lock()
        .await
        .data
        .radarr_data
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

    self.app.lock().await.data.radarr_data.indexer_settings = None;

    resp
  }

  async fn edit_collection(
    &mut self,
    edit_collection_params: Option<EditCollectionParams>,
  ) -> Result<()> {
    info!("Editing Radarr collection");
    let detail_event = RadarrEvent::GetCollections;
    let event = RadarrEvent::EditCollection(None);
    info!("Fetching collection details");

    let collection_id = if let Some(ref params) = edit_collection_params {
      params.collection_id
    } else {
      self.extract_collection_id().await
    };
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

    let mut detailed_collection_body: Value = serde_json::from_str(&response).unwrap();
    let (monitored, minimum_availability, quality_profile_id, root_folder_path, search_on_add) =
      if let Some(params) = edit_collection_params {
        let monitored = params.monitored.unwrap_or_else(|| {
          detailed_collection_body["monitored"]
            .as_bool()
            .expect("Unable to deserialize 'monitored' bool")
        });
        let minimum_availability = params
          .minimum_availability
          .unwrap_or_else(|| {
            serde_json::from_value(detailed_collection_body["minimumAvailability"].clone())
              .expect("Unable to deserialize 'minimumAvailability'")
          })
          .to_string();
        let quality_profile_id = params.quality_profile_id.unwrap_or_else(|| {
          detailed_collection_body["qualityProfileId"]
            .as_i64()
            .expect("Unable to deserialize 'qualityProfileId'")
        });
        let root_folder_path = params.root_folder_path.unwrap_or_else(|| {
          detailed_collection_body["rootFolderPath"]
            .as_str()
            .expect("Unable to deserialize 'rootFolderPath'")
            .to_owned()
        });
        let search_on_add = params.search_on_add.unwrap_or_else(|| {
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
      } else {
        let mut app = self.app.lock().await;
        let EditCollectionModal {
          path,
          search_on_add,
          minimum_availability_list,
          monitored,
          quality_profile_list,
        } = app.data.radarr_data.edit_collection_modal.as_ref().unwrap();
        let quality_profile = quality_profile_list.current_selection();
        let quality_profile_id = *app
          .data
          .radarr_data
          .quality_profile_map
          .iter()
          .filter(|(_, value)| *value == quality_profile)
          .map(|(key, _)| key)
          .next()
          .unwrap();

        let root_folder_path: String = path.text.clone();
        let monitored = monitored.unwrap_or_default();
        let search_on_add = search_on_add.unwrap_or_default();
        let minimum_availability = minimum_availability_list.current_selection().to_string();
        app.data.radarr_data.edit_collection_modal = None;

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

  async fn edit_indexer(&mut self, edit_indexer_params: Option<EditIndexerParams>) -> Result<()> {
    let detail_event = RadarrEvent::GetIndexers;
    let event = RadarrEvent::EditIndexer(None);
    let id = if let Some(ref params) = edit_indexer_params {
      params.indexer_id
    } else {
      self
        .app
        .lock()
        .await
        .data
        .radarr_data
        .indexers
        .current_selection()
        .id
    };
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

    let mut detailed_indexer_body: Value = serde_json::from_str(&response).unwrap();
    let priority = detailed_indexer_body["priority"]
      .as_i64()
      .expect("Unable to deserialize 'priority'");

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
        .radarr_data
        .edit_indexer_modal
        .as_ref()
        .unwrap()
        .tags
        .text
        .clone();
      let tag_ids_vec = self.extract_and_add_tag_ids_vec(tags).await;
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
          ..
        } = app.data.radarr_data.edit_indexer_modal.as_ref().unwrap();

        (
          name.text.clone(),
          enable_rss.unwrap_or_default(),
          enable_automatic_search.unwrap_or_default(),
          enable_interactive_search.unwrap_or_default(),
          url.text.clone(),
          api_key.text.clone(),
          seed_ratio.text.clone(),
          tag_ids_vec,
          priority,
        )
      };

      app.data.radarr_data.edit_indexer_modal = None;

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
        None,
      )
      .await;

    self
      .handle_request::<Value, ()>(request_props, |_, _| ())
      .await
  }

  async fn edit_movie(&mut self, edit_movie_params: Option<EditMovieParams>) -> Result<()> {
    info!("Editing Radarr movie");
    let detail_event = RadarrEvent::GetMovieDetails(None);
    let event = RadarrEvent::EditMovie(None);

    let (movie_id, _) = if let Some(ref params) = edit_movie_params {
      self.extract_movie_id(Some(params.movie_id)).await
    } else {
      self.extract_movie_id(None).await
    };
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

    let mut detailed_movie_body: Value = serde_json::from_str(&response).unwrap();
    let (monitored, minimum_availability, quality_profile_id, root_folder_path, tags) =
      if let Some(params) = edit_movie_params {
        let monitored = params.monitored.unwrap_or(
          detailed_movie_body["monitored"]
            .as_bool()
            .expect("Unable to deserialize 'monitored'"),
        );
        let minimum_availability = params
          .minimum_availability
          .unwrap_or_else(|| {
            serde_json::from_value(detailed_movie_body["minimumAvailability"].clone())
              .expect("Unable to deserialize 'minimumAvailability'")
          })
          .to_string();
        let quality_profile_id = params.quality_profile_id.unwrap_or_else(|| {
          detailed_movie_body["qualityProfileId"]
            .as_i64()
            .expect("Unable to deserialize 'qualityProfileId'")
        });
        let root_folder_path = params.root_folder_path.unwrap_or_else(|| {
          detailed_movie_body["path"]
            .as_str()
            .expect("Unable to deserialize 'path'")
            .to_owned()
        });
        let tags = if params.clear_tags {
          vec![]
        } else {
          params.tags.unwrap_or(
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
      } else {
        let tags = self
          .app
          .lock()
          .await
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .tags
          .text
          .clone();
        let tag_ids_vec = self.extract_and_add_tag_ids_vec(tags).await;
        let mut app = self.app.lock().await;

        let params = {
          let EditMovieModal {
            monitored,
            path,
            minimum_availability_list,
            quality_profile_list,
            ..
          } = app.data.radarr_data.edit_movie_modal.as_ref().unwrap();
          let quality_profile = quality_profile_list.current_selection();
          let quality_profile_id = *app
            .data
            .radarr_data
            .quality_profile_map
            .iter()
            .filter(|(_, value)| *value == quality_profile)
            .map(|(key, _)| key)
            .next()
            .unwrap();

          (
            monitored.unwrap_or_default(),
            minimum_availability_list.current_selection().to_string(),
            quality_profile_id,
            path.text.clone(),
            tag_ids_vec,
          )
        };

        app.data.radarr_data.edit_movie_modal = None;

        params
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

  async fn get_credits(&mut self, movie_id: Option<i64>) -> Result<Vec<Credit>> {
    info!("Fetching Radarr movie credits");
    let event = RadarrEvent::GetMovieCredits(None);
    let (_, movie_id_param) = self.extract_movie_id(movie_id).await;

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(movie_id_param),
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

  async fn get_diskspace(&mut self) -> Result<Vec<DiskSpace>> {
    info!("Fetching Radarr disk space");
    let event = RadarrEvent::GetOverview;

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

  async fn get_radarr_logs(&mut self, events: Option<u64>) -> Result<LogResponse> {
    info!("Fetching Radarr logs");
    let event = RadarrEvent::GetLogs(events);

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

        app.data.radarr_data.logs.set_items(log_lines);
        app.data.radarr_data.logs.scroll_to_bottom();
      })
      .await
  }

  async fn get_movie_details(&mut self, movie_id: Option<i64>) -> Result<Movie> {
    info!("Fetching Radarr movie details");
    let event = RadarrEvent::GetMovieDetails(None);
    let (id, _) = self.extract_movie_id(movie_id).await;

    info!("Fetching movie details for movie with ID: {id}");

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

  async fn get_movie_history(&mut self, movie_id: Option<i64>) -> Result<Vec<MovieHistoryItem>> {
    info!("Fetching Radarr movie history");
    let event = RadarrEvent::GetMovieHistory(None);

    let (_, movie_id_param) = self.extract_movie_id(movie_id).await;
    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(movie_id_param),
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

  async fn get_movie_releases(&mut self, movie_id: Option<i64>) -> Result<Vec<Release>> {
    let (id, movie_id_param) = self.extract_movie_id(movie_id).await;
    info!("Fetching releases for movie with ID: {id}");
    let event = RadarrEvent::GetReleases(None);

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(movie_id_param),
      )
      .await;

    self
      .handle_request::<(), Vec<Release>>(request_props, |release_vec, mut app| {
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

  async fn get_tasks(&mut self) -> Result<Vec<Task>> {
    info!("Fetching Radarr tasks");
    let event = RadarrEvent::GetTasks;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<Task>>(request_props, |tasks_vec, mut app| {
        app.data.radarr_data.tasks.set_items(tasks_vec);
      })
      .await
  }

  async fn get_updates(&mut self) -> Result<Vec<Update>> {
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

  async fn search_movie(&mut self, query: Option<String>) -> Result<Vec<AddMovieSearchResult>> {
    info!("Searching for specific Radarr movie");
    let event = RadarrEvent::SearchNewMovie(None);
    let search = if let Some(search_query) = query {
      Ok(search_query.into())
    } else {
      self
        .app
        .lock()
        .await
        .data
        .radarr_data
        .add_movie_search
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
          .handle_request::<(), Vec<AddMovieSearchResult>>(request_props, |movie_vec, mut app| {
            if movie_vec.is_empty() {
              app.pop_and_push_navigation_stack(
                ActiveRadarrBlock::AddMovieEmptySearchResults.into(),
              );
            } else if let Some(add_searched_movies) =
              app.data.radarr_data.add_searched_movies.as_mut()
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

  async fn start_task(&mut self, task: Option<TaskName>) -> Result<Value> {
    let event = RadarrEvent::StartTask(None);
    let task_name = if let Some(t_name) = task {
      t_name
    } else {
      self
        .app
        .lock()
        .await
        .data
        .radarr_data
        .tasks
        .current_selection()
        .task_name
    }
    .to_string();

    info!("Starting Radarr task: {task_name}");

    let body = CommandBody { name: task_name };

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(body), None, None)
      .await;

    self
      .handle_request::<CommandBody, Value>(request_props, |_, _| ())
      .await
  }

  async fn test_indexer(&mut self, indexer_id: Option<i64>) -> Result<Value> {
    let detail_event = RadarrEvent::GetIndexers;
    let event = RadarrEvent::TestIndexer(None);
    let id = if let Some(i_id) = indexer_id {
      i_id
    } else {
      self
        .app
        .lock()
        .await
        .data
        .radarr_data
        .indexers
        .current_selection()
        .id
    };
    info!("Testing Radarr indexer with ID: {id}");

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
          app.data.radarr_data.indexer_test_error = Some(
            test_results.as_array().unwrap()[0]
              .get("errorMessage")
              .unwrap()
              .to_string(),
          );
        };
      })
      .await
  }

  async fn test_all_indexers(&mut self) -> Result<Vec<IndexerTestResult>> {
    info!("Testing all indexers");
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

  async fn trigger_automatic_search(&mut self, movie_id: Option<i64>) -> Result<Value> {
    let event = RadarrEvent::TriggerAutomaticSearch(None);
    let (id, _) = self.extract_movie_id(movie_id).await;
    info!("Searching indexers for movie with ID: {id}");
    let body = MovieCommandBody {
      name: "MoviesSearch".to_owned(),
      movie_ids: vec![id],
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

  async fn update_and_scan(&mut self, movie_id: Option<i64>) -> Result<Value> {
    let (id, _) = self.extract_movie_id(movie_id).await;
    let event = RadarrEvent::UpdateAndScan(None);
    info!("Updating and scanning movie with ID: {id}");
    let body = MovieCommandBody {
      name: "RefreshMovie".to_owned(),
      movie_ids: vec![id],
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

  async fn update_downloads(&mut self) -> Result<Value> {
    info!("Updating downloads");
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

  async fn extract_and_add_tag_ids_vec(&mut self, edit_tags: String) -> Vec<i64> {
    let tags_map = self.app.lock().await.data.radarr_data.tags_map.clone();
    let tags = edit_tags.clone();
    let missing_tags_vec = edit_tags
      .split(',')
      .filter(|&tag| !tag.is_empty() && tags_map.get_by_right(tag.trim()).is_none())
      .collect::<Vec<&str>>();

    for tag in missing_tags_vec {
      self
        .add_radarr_tag(tag.trim().to_owned())
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
          .radarr_data
          .tags_map
          .get_by_right(tag.trim())
          .unwrap()
      })
      .collect()
  }

  async fn extract_movie_id(&mut self, movie_id: Option<i64>) -> (i64, String) {
    let movie_id = if let Some(id) = movie_id {
      id
    } else {
      self
        .app
        .lock()
        .await
        .data
        .radarr_data
        .movies
        .current_selection()
        .id
    };
    (movie_id, format!("movieId={movie_id}"))
  }

  async fn extract_collection_id(&mut self) -> i64 {
    self
      .app
      .lock()
      .await
      .data
      .radarr_data
      .collections
      .current_selection()
      .id
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
