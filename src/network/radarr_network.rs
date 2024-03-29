use anyhow::anyhow;
use std::fmt::Debug;

use indoc::formatdoc;
use log::{debug, info, warn};
use serde::Serialize;
use serde_json::{json, Value};
use urlencoding::encode;

use crate::app::RadarrConfig;
use crate::models::radarr_models::{
  AddMovieBody, AddMovieSearchResult, AddOptions, AddRootFolderBody, BlocklistResponse, Collection,
  CollectionMovie, CommandBody, Credit, CreditType, DiskSpace, DownloadRecord, DownloadsResponse,
  Indexer, IndexerSettings, IndexerTestResult, LogResponse, Movie, MovieCommandBody,
  MovieHistoryItem, QualityProfile, QueueEvent, Release, ReleaseDownloadBody, RootFolder,
  SystemStatus, Tag, Task, Update,
};
use crate::models::servarr_data::radarr::modals::{
  AddMovieModal, EditCollectionModal, EditIndexerModal, EditMovieModal, IndexerTestResultModalItem,
  MovieDetailsModal,
};
use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
use crate::models::stateful_table::StatefulTable;
use crate::models::{HorizontallyScrollableText, Route, Scrollable, ScrollableText};
use crate::network::{Network, NetworkEvent, RequestMethod, RequestProps};
use crate::utils::{convert_runtime, convert_to_gb};

#[cfg(test)]
#[path = "radarr_network_tests.rs"]
mod radarr_network_tests;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum RadarrEvent {
  AddMovie,
  AddRootFolder,
  ClearBlocklist,
  DeleteBlocklistItem,
  DeleteDownload,
  DeleteIndexer,
  DeleteMovie,
  DeleteRootFolder,
  DownloadRelease,
  EditAllIndexerSettings,
  EditCollection,
  EditIndexer,
  EditMovie,
  GetBlocklist,
  GetCollections,
  GetDownloads,
  GetIndexers,
  GetIndexerSettings,
  GetLogs,
  GetMovieCredits,
  GetMovieDetails,
  GetMovieHistory,
  GetMovies,
  GetOverview,
  GetQualityProfiles,
  GetQueuedEvents,
  GetReleases,
  GetRootFolders,
  GetStatus,
  GetTags,
  GetTasks,
  GetUpdates,
  HealthCheck,
  SearchNewMovie,
  StartTask,
  TestIndexer,
  TestAllIndexers,
  TriggerAutomaticSearch,
  UpdateAllMovies,
  UpdateAndScan,
  UpdateCollections,
  UpdateDownloads,
}

impl RadarrEvent {
  const fn resource(self) -> &'static str {
    match self {
      RadarrEvent::ClearBlocklist => "/blocklist/bulk",
      RadarrEvent::DeleteBlocklistItem => "/blocklist",
      RadarrEvent::GetBlocklist => "/blocklist?page=1&pageSize=10000",
      RadarrEvent::GetCollections | RadarrEvent::EditCollection => "/collection",
      RadarrEvent::GetDownloads | RadarrEvent::DeleteDownload => "/queue",
      RadarrEvent::GetIndexers | RadarrEvent::EditIndexer | RadarrEvent::DeleteIndexer => {
        "/indexer"
      }
      RadarrEvent::GetIndexerSettings | RadarrEvent::EditAllIndexerSettings => "/config/indexer",
      RadarrEvent::GetLogs => "/log",
      RadarrEvent::AddMovie
      | RadarrEvent::EditMovie
      | RadarrEvent::GetMovies
      | RadarrEvent::GetMovieDetails
      | RadarrEvent::DeleteMovie => "/movie",
      RadarrEvent::SearchNewMovie => "/movie/lookup",
      RadarrEvent::GetMovieCredits => "/credit",
      RadarrEvent::GetMovieHistory => "/history/movie",
      RadarrEvent::GetOverview => "/diskspace",
      RadarrEvent::GetQualityProfiles => "/qualityprofile",
      RadarrEvent::GetReleases | RadarrEvent::DownloadRelease => "/release",
      RadarrEvent::AddRootFolder | RadarrEvent::GetRootFolders | RadarrEvent::DeleteRootFolder => {
        "/rootfolder"
      }
      RadarrEvent::GetStatus => "/system/status",
      RadarrEvent::GetTags => "/tag",
      RadarrEvent::GetTasks => "/system/task",
      RadarrEvent::GetUpdates => "/update",
      RadarrEvent::TestIndexer => "/indexer/test",
      RadarrEvent::TestAllIndexers => "/indexer/testall",
      RadarrEvent::StartTask
      | RadarrEvent::GetQueuedEvents
      | RadarrEvent::TriggerAutomaticSearch
      | RadarrEvent::UpdateAndScan
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
  pub async fn handle_radarr_event(&mut self, radarr_event: RadarrEvent) {
    match radarr_event {
      RadarrEvent::AddMovie => self.add_movie().await,
      RadarrEvent::AddRootFolder => self.add_root_folder().await,
      RadarrEvent::ClearBlocklist => self.clear_blocklist().await,
      RadarrEvent::DeleteBlocklistItem => self.delete_blocklist_item().await,
      RadarrEvent::DeleteDownload => self.delete_download().await,
      RadarrEvent::DeleteIndexer => self.delete_indexer().await,
      RadarrEvent::DeleteMovie => self.delete_movie().await,
      RadarrEvent::DeleteRootFolder => self.delete_root_folder().await,
      RadarrEvent::DownloadRelease => self.download_release().await,
      RadarrEvent::EditAllIndexerSettings => self.edit_all_indexer_settings().await,
      RadarrEvent::EditCollection => self.edit_collection().await,
      RadarrEvent::EditIndexer => self.edit_indexer().await,
      RadarrEvent::EditMovie => self.edit_movie().await,
      RadarrEvent::GetBlocklist => self.get_blocklist().await,
      RadarrEvent::GetCollections => self.get_collections().await,
      RadarrEvent::GetDownloads => self.get_downloads().await,
      RadarrEvent::GetIndexers => self.get_indexers().await,
      RadarrEvent::GetIndexerSettings => self.get_indexer_settings().await,
      RadarrEvent::GetLogs => self.get_logs().await,
      RadarrEvent::GetMovieCredits => self.get_credits().await,
      RadarrEvent::GetMovieDetails => self.get_movie_details().await,
      RadarrEvent::GetMovieHistory => self.get_movie_history().await,
      RadarrEvent::GetMovies => self.get_movies().await,
      RadarrEvent::GetOverview => self.get_diskspace().await,
      RadarrEvent::GetQualityProfiles => self.get_quality_profiles().await,
      RadarrEvent::GetQueuedEvents => self.get_queued_events().await,
      RadarrEvent::GetReleases => self.get_releases().await,
      RadarrEvent::GetRootFolders => self.get_root_folders().await,
      RadarrEvent::GetStatus => self.get_status().await,
      RadarrEvent::GetTags => self.get_tags().await,
      RadarrEvent::GetTasks => self.get_tasks().await,
      RadarrEvent::GetUpdates => self.get_updates().await,
      RadarrEvent::HealthCheck => self.get_healthcheck().await,
      RadarrEvent::SearchNewMovie => self.search_movie().await,
      RadarrEvent::StartTask => self.start_task().await,
      RadarrEvent::TestIndexer => self.test_indexer().await,
      RadarrEvent::TestAllIndexers => self.test_all_indexers().await,
      RadarrEvent::TriggerAutomaticSearch => self.trigger_automatic_search().await,
      RadarrEvent::UpdateAllMovies => self.update_all_movies().await,
      RadarrEvent::UpdateAndScan => self.update_and_scan().await,
      RadarrEvent::UpdateCollections => self.update_collections().await,
      RadarrEvent::UpdateDownloads => self.update_downloads().await,
    }
  }

  async fn add_movie(&mut self) {
    info!("Adding new movie to Radarr");
    let body = {
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
      .radarr_request_props_from(
        RadarrEvent::AddMovie.resource(),
        RequestMethod::Post,
        Some(body),
      )
      .await;

    self
      .handle_request::<AddMovieBody, Value>(request_props, |_, _| ())
      .await;
  }

  async fn add_root_folder(&mut self) {
    info!("Adding new root folder to Radarr");
    let body = {
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
      .radarr_request_props_from(
        RadarrEvent::AddRootFolder.resource(),
        RequestMethod::Post,
        Some(body),
      )
      .await;

    self
      .handle_request::<AddRootFolderBody, Value>(request_props, |_, _| ())
      .await;
  }

  async fn add_tag(&mut self, tag: String) {
    info!("Adding a new Radarr tag");

    let request_props = self
      .radarr_request_props_from(
        RadarrEvent::GetTags.resource(),
        RequestMethod::Post,
        Some(json!({ "label": tag })),
      )
      .await;

    self
      .handle_request::<Value, Tag>(request_props, |tag, mut app| {
        app.data.radarr_data.tags_map.insert(tag.id, tag.label);
      })
      .await;
  }

  async fn clear_blocklist(&mut self) {
    info!("Clearing Radarr blocklist");

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
      .radarr_request_props_from(
        RadarrEvent::ClearBlocklist.resource(),
        RequestMethod::Delete,
        Some(json!({"ids": ids})),
      )
      .await;

    self
      .handle_request::<Value, ()>(request_props, |_, _| ())
      .await;
  }

  async fn delete_blocklist_item(&mut self) {
    let blocklist_item_id = self
      .app
      .lock()
      .await
      .data
      .radarr_data
      .blocklist
      .current_selection()
      .id;

    info!("Deleting Radarr blocklist item for item with id: {blocklist_item_id}");

    let request_props = self
      .radarr_request_props_from(
        format!(
          "{}/{blocklist_item_id}",
          RadarrEvent::DeleteBlocklistItem.resource()
        )
        .as_str(),
        RequestMethod::Delete,
        None::<()>,
      )
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await;
  }

  async fn delete_download(&mut self) {
    let download_id = self
      .app
      .lock()
      .await
      .data
      .radarr_data
      .downloads
      .current_selection()
      .id;

    info!("Deleting Radarr download for download with id: {download_id}");

    let request_props = self
      .radarr_request_props_from(
        format!("{}/{download_id}", RadarrEvent::DeleteDownload.resource()).as_str(),
        RequestMethod::Delete,
        None::<()>,
      )
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await;
  }

  async fn delete_indexer(&mut self) {
    let indexer_id = self
      .app
      .lock()
      .await
      .data
      .radarr_data
      .indexers
      .current_selection()
      .id;

    info!("Deleting Radarr indexer for indexer with id: {indexer_id}");

    let request_props = self
      .radarr_request_props_from(
        format!("{}/{indexer_id}", RadarrEvent::DeleteIndexer.resource()).as_str(),
        RequestMethod::Delete,
        None::<()>,
      )
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await;
  }

  async fn delete_movie(&mut self) {
    let (movie_id, tmdb_id) = self.extract_movie_id().await;
    let delete_files = self.app.lock().await.data.radarr_data.delete_movie_files;
    let add_import_exclusion = self.app.lock().await.data.radarr_data.add_list_exclusion;

    info!("Deleting Radarr movie with tmdb_id {tmdb_id} and Radarr id: {movie_id} with deleteFiles={delete_files} and addImportExclusion={add_import_exclusion}");

    let request_props = self
      .radarr_request_props_from(
        format!(
          "{}/{movie_id}?deleteFiles={delete_files}&addImportExclusion={add_import_exclusion}",
          RadarrEvent::DeleteMovie.resource()
        )
        .as_str(),
        RequestMethod::Delete,
        None::<()>,
      )
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await;

    self
      .app
      .lock()
      .await
      .data
      .radarr_data
      .reset_delete_movie_preferences();
  }

  async fn delete_root_folder(&mut self) {
    let root_folder_id = self
      .app
      .lock()
      .await
      .data
      .radarr_data
      .root_folders
      .current_selection()
      .id;

    info!("Deleting Radarr root folder for folder with id: {root_folder_id}");

    let request_props = self
      .radarr_request_props_from(
        format!(
          "{}/{root_folder_id}",
          RadarrEvent::DeleteRootFolder.resource()
        )
        .as_str(),
        RequestMethod::Delete,
        None::<()>,
      )
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await;
  }

  async fn download_release(&mut self) {
    let (movie_id, _) = self.extract_movie_id().await;
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

    let download_release_body = ReleaseDownloadBody {
      guid,
      indexer_id,
      movie_id,
    };

    let request_props = self
      .radarr_request_props_from(
        RadarrEvent::DownloadRelease.resource(),
        RequestMethod::Post,
        Some(download_release_body),
      )
      .await;

    self
      .handle_request::<ReleaseDownloadBody, Value>(request_props, |_, _| ())
      .await;
  }

  async fn edit_all_indexer_settings(&mut self) {
    info!("Updating Radarr indexer settings");

    let body = self
      .app
      .lock()
      .await
      .data
      .radarr_data
      .indexer_settings
      .as_ref()
      .unwrap()
      .clone();

    debug!("Indexer settings body: {body:?}");

    let request_props = self
      .radarr_request_props_from(
        RadarrEvent::EditAllIndexerSettings.resource(),
        RequestMethod::Put,
        Some(body),
      )
      .await;

    self
      .handle_request::<IndexerSettings, Value>(request_props, |_, _| {})
      .await;

    self.app.lock().await.data.radarr_data.indexer_settings = None;
  }

  async fn edit_collection(&mut self) {
    info!("Editing Radarr collection");

    info!("Fetching collection details");
    let collection_id = self.extract_collection_id().await;
    let request_props = self
      .radarr_request_props_from(
        format!("{}/{collection_id}", RadarrEvent::GetCollections.resource()).as_str(),
        RequestMethod::Get,
        None::<()>,
      )
      .await;

    let mut response = String::new();

    self
      .handle_request::<(), Value>(request_props, |detailed_collection_body, _| {
        response = detailed_collection_body.to_string()
      })
      .await;

    info!("Constructing edit collection body");

    let body = {
      let mut app = self.app.lock().await;
      let mut detailed_collection_body: Value = serde_json::from_str(&response).unwrap();
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

      *detailed_collection_body.get_mut("monitored").unwrap() = json!(monitored);
      *detailed_collection_body
        .get_mut("minimumAvailability")
        .unwrap() = json!(minimum_availability);
      *detailed_collection_body
        .get_mut("qualityProfileId")
        .unwrap() = json!(quality_profile_id);
      *detailed_collection_body.get_mut("rootFolderPath").unwrap() = json!(root_folder_path);
      *detailed_collection_body.get_mut("searchOnAdd").unwrap() = json!(search_on_add);

      app.data.radarr_data.edit_collection_modal = None;

      detailed_collection_body
    };

    debug!("Edit collection body: {body:?}");

    let request_props = self
      .radarr_request_props_from(
        format!("{}/{collection_id}", RadarrEvent::EditCollection.resource()).as_str(),
        RequestMethod::Put,
        Some(body),
      )
      .await;

    self
      .handle_request::<Value, ()>(request_props, |_, _| ())
      .await;
  }

  async fn edit_indexer(&mut self) {
    let id = self
      .app
      .lock()
      .await
      .data
      .radarr_data
      .indexers
      .current_selection()
      .id;
    info!("Updating Radarr indexer with ID: {id}");

    info!("Fetching indexer details for indexer with ID: {id}");

    let request_props = self
      .radarr_request_props_from(
        format!("{}/{id}", RadarrEvent::GetIndexers.resource()).as_str(),
        RequestMethod::Get,
        None::<()>,
      )
      .await;

    let mut response = String::new();

    self
      .handle_request::<(), Value>(request_props, |detailed_indexer_body, _| {
        response = detailed_indexer_body.to_string()
      })
      .await;

    info!("Constructing edit indexer body");

    let body = {
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
      let mut detailed_indexer_body: Value = serde_json::from_str(&response).unwrap();

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

      *detailed_indexer_body.get_mut("name").unwrap() = json!(name.text.clone());
      *detailed_indexer_body.get_mut("enableRss").unwrap() = json!(enable_rss.unwrap_or_default());
      *detailed_indexer_body
        .get_mut("enableAutomaticSearch")
        .unwrap() = json!(enable_automatic_search.unwrap_or_default());
      *detailed_indexer_body
        .get_mut("enableInteractiveSearch")
        .unwrap() = json!(enable_interactive_search.unwrap_or_default());
      *detailed_indexer_body
        .get_mut("fields")
        .unwrap()
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|field| field["name"] == "baseUrl")
        .unwrap()
        .get_mut("value")
        .unwrap() = json!(url.text.clone());
      *detailed_indexer_body
        .get_mut("fields")
        .unwrap()
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|field| field["name"] == "apiKey")
        .unwrap()
        .get_mut("value")
        .unwrap() = json!(api_key.text.clone());
      *detailed_indexer_body.get_mut("tags").unwrap() = json!(tag_ids_vec);
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
          .insert("value".to_string(), json!(seed_ratio.text.clone()));
      }

      app.data.radarr_data.edit_indexer_modal = None;

      detailed_indexer_body
    };

    debug!("Edit indexer body: {body:?}");

    let request_props = self
      .radarr_request_props_from(
        format!("{}/{id}", RadarrEvent::EditIndexer.resource()).as_str(),
        RequestMethod::Put,
        Some(body),
      )
      .await;

    self
      .handle_request::<Value, ()>(request_props, |_, _| ())
      .await;
  }

  async fn edit_movie(&mut self) {
    info!("Editing Radarr movie");

    let (movie_id, tmdb_id) = self.extract_movie_id().await;
    info!("Fetching movie details for movie with TMDB ID: {tmdb_id}");

    let request_props = self
      .radarr_request_props_from(
        format!("{}/{movie_id}", RadarrEvent::GetMovieDetails.resource()).as_str(),
        RequestMethod::Get,
        None::<()>,
      )
      .await;

    let mut response = String::new();

    self
      .handle_request::<(), Value>(request_props, |detailed_movie_body, _| {
        response = detailed_movie_body.to_string()
      })
      .await;

    info!("Constructing edit movie body");

    let body = {
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
      let mut detailed_movie_body: Value = serde_json::from_str(&response).unwrap();

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

      *detailed_movie_body.get_mut("monitored").unwrap() = json!(monitored.unwrap_or_default());
      *detailed_movie_body.get_mut("minimumAvailability").unwrap() =
        json!(minimum_availability_list.current_selection().to_string());
      *detailed_movie_body.get_mut("qualityProfileId").unwrap() = json!(quality_profile_id);
      *detailed_movie_body.get_mut("path").unwrap() = json!(path.text.clone());
      *detailed_movie_body.get_mut("tags").unwrap() = json!(tag_ids_vec);

      app.data.radarr_data.edit_movie_modal = None;

      detailed_movie_body
    };

    debug!("Edit movie body: {body:?}");

    let request_props = self
      .radarr_request_props_from(
        format!("{}/{movie_id}", RadarrEvent::EditMovie.resource()).as_str(),
        RequestMethod::Put,
        Some(body),
      )
      .await;

    self
      .handle_request::<Value, ()>(request_props, |_, _| ())
      .await;
  }

  async fn get_blocklist(&mut self) {
    info!("Fetching blocklist");

    let request_props = self
      .radarr_request_props_from(
        RadarrEvent::GetBlocklist.resource(),
        RequestMethod::Get,
        None::<()>,
      )
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
      .await;
  }

  async fn get_collections(&mut self) {
    info!("Fetching Radarr collections");

    let request_props = self
      .radarr_request_props_from(
        RadarrEvent::GetCollections.resource(),
        RequestMethod::Get,
        None::<()>,
      )
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
      .await;
  }

  async fn get_credits(&mut self) {
    info!("Fetching Radarr movie credits");

    let request_uri = self
      .append_movie_id_param(RadarrEvent::GetMovieCredits.resource())
      .await;
    let request_props = self
      .radarr_request_props_from(request_uri.as_str(), RequestMethod::Get, None::<()>)
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
      .await;
  }

  async fn get_diskspace(&mut self) {
    info!("Fetching Radarr disk space");

    let request_props = self
      .radarr_request_props_from(
        RadarrEvent::GetOverview.resource(),
        RequestMethod::Get,
        None::<()>,
      )
      .await;

    self
      .handle_request::<(), Vec<DiskSpace>>(request_props, |disk_space_vec, mut app| {
        app.data.radarr_data.disk_space_vec = disk_space_vec;
      })
      .await;
  }

  async fn get_downloads(&mut self) {
    info!("Fetching Radarr downloads");

    let request_props = self
      .radarr_request_props_from(
        RadarrEvent::GetDownloads.resource(),
        RequestMethod::Get,
        None::<()>,
      )
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

  async fn get_indexers(&mut self) {
    info!("Fetching Radarr indexers");

    let request_props = self
      .radarr_request_props_from(
        RadarrEvent::GetIndexers.resource(),
        RequestMethod::Get,
        None::<()>,
      )
      .await;

    self
      .handle_request::<(), Vec<Indexer>>(request_props, |indexers, mut app| {
        app.data.radarr_data.indexers.set_items(indexers);
      })
      .await
  }

  async fn get_indexer_settings(&mut self) {
    info!("Fetching Radarr indexer settings");

    let request_props = self
      .radarr_request_props_from(
        RadarrEvent::GetIndexerSettings.resource(),
        RequestMethod::Get,
        None::<()>,
      )
      .await;

    self
      .handle_request::<(), IndexerSettings>(request_props, |indexer_settings, mut app| {
        if app.data.radarr_data.indexer_settings.is_none() {
          app.data.radarr_data.indexer_settings = Some(indexer_settings);
        } else {
          debug!("Indexer Settings are being modified. Ignoring update...");
        }
      })
      .await;
  }

  async fn get_healthcheck(&mut self) {
    info!("Performing Radarr health check");

    let request_props = self
      .radarr_request_props_from(
        RadarrEvent::HealthCheck.resource(),
        RequestMethod::Get,
        None::<()>,
      )
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await;
  }

  async fn get_logs(&mut self) {
    info!("Fetching Radarr logs");

    let resource = format!(
      "{}?pageSize=500&sortDirection=descending&sortKey=time",
      RadarrEvent::GetLogs.resource()
    );
    let request_props = self
      .radarr_request_props_from(&resource, RequestMethod::Get, None::<()>)
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
      .await;
  }

  async fn get_movie_details(&mut self) {
    info!("Fetching Radarr movie details");

    let (movie_id, tmdb_id) = self.extract_movie_id().await;
    info!("Fetching movie details for movie with TMDB ID: {tmdb_id}");

    let request_props = self
      .radarr_request_props_from(
        format!("{}/{movie_id}", RadarrEvent::GetMovieDetails.resource()).as_str(),
        RequestMethod::Get,
        None::<()>,
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
          .unwrap()
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
      .await;
  }

  async fn get_movie_history(&mut self) {
    info!("Fetching Radarr movie history");

    let request_uri = self
      .append_movie_id_param(RadarrEvent::GetMovieHistory.resource())
      .await;
    let request_props = self
      .radarr_request_props_from(request_uri.as_str(), RequestMethod::Get, None::<()>)
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
      .await;
  }

  async fn get_movies(&mut self) {
    info!("Fetching Radarr library");

    let request_props = self
      .radarr_request_props_from(
        RadarrEvent::GetMovies.resource(),
        RequestMethod::Get,
        None::<()>,
      )
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
      .await;
  }

  async fn get_quality_profiles(&mut self) {
    info!("Fetching Radarr quality profiles");

    let request_props = self
      .radarr_request_props_from(
        RadarrEvent::GetQualityProfiles.resource(),
        RequestMethod::Get,
        None::<()>,
      )
      .await;

    self
      .handle_request::<(), Vec<QualityProfile>>(request_props, |quality_profiles, mut app| {
        app.data.radarr_data.quality_profile_map = quality_profiles
          .into_iter()
          .map(|profile| (profile.id, profile.name))
          .collect();
      })
      .await;
  }

  async fn get_queued_events(&mut self) {
    info!("Fetching Radarr queued events");

    let request_props = self
      .radarr_request_props_from(
        RadarrEvent::GetQueuedEvents.resource(),
        RequestMethod::Get,
        None::<()>,
      )
      .await;

    self
      .handle_request::<(), Vec<QueueEvent>>(request_props, |queued_events_vec, mut app| {
        app
          .data
          .radarr_data
          .queued_events
          .set_items(queued_events_vec);
      })
      .await;
  }

  async fn get_releases(&mut self) {
    let (movie_id, tmdb_id) = self.extract_movie_id().await;
    info!("Fetching releases for movie with TMDB id {tmdb_id} and with Radarr id: {movie_id}");

    let request_props = self
      .radarr_request_props_from(
        format!("{}?movieId={movie_id}", RadarrEvent::GetReleases.resource()).as_str(),
        RequestMethod::Get,
        None::<()>,
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
      .await;
  }

  async fn get_root_folders(&mut self) {
    info!("Fetching Radarr root folders");

    let request_props = self
      .radarr_request_props_from(
        RadarrEvent::GetRootFolders.resource(),
        RequestMethod::Get,
        None::<()>,
      )
      .await;

    self
      .handle_request::<(), Vec<RootFolder>>(request_props, |root_folders, mut app| {
        app.data.radarr_data.root_folders.set_items(root_folders);
      })
      .await;
  }

  async fn get_status(&mut self) {
    info!("Fetching Radarr system status");

    let request_props = self
      .radarr_request_props_from(
        RadarrEvent::GetStatus.resource(),
        RequestMethod::Get,
        None::<()>,
      )
      .await;

    self
      .handle_request::<(), SystemStatus>(request_props, |system_status, mut app| {
        app.data.radarr_data.version = system_status.version;
        app.data.radarr_data.start_time = system_status.start_time;
      })
      .await;
  }

  async fn get_tags(&mut self) {
    info!("Fetching Radarr tags");

    let request_props = self
      .radarr_request_props_from(
        RadarrEvent::GetTags.resource(),
        RequestMethod::Get,
        None::<()>,
      )
      .await;

    self
      .handle_request::<(), Vec<Tag>>(request_props, |tags_vec, mut app| {
        app.data.radarr_data.tags_map = tags_vec
          .into_iter()
          .map(|tag| (tag.id, tag.label))
          .collect();
      })
      .await;
  }

  async fn get_tasks(&mut self) {
    info!("Fetching Radarr tasks");

    let request_props = self
      .radarr_request_props_from(
        RadarrEvent::GetTasks.resource(),
        RequestMethod::Get,
        None::<()>,
      )
      .await;

    self
      .handle_request::<(), Vec<Task>>(request_props, |tasks_vec, mut app| {
        app.data.radarr_data.tasks.set_items(tasks_vec);
      })
      .await;
  }

  async fn get_updates(&mut self) {
    info!("Fetching Radarr updates");

    let request_props = self
      .radarr_request_props_from(
        RadarrEvent::GetUpdates.resource(),
        RequestMethod::Get,
        None::<()>,
      )
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
      .await;
  }

  async fn search_movie(&mut self) {
    info!("Searching for specific Radarr movie");
    let search = self
      .app
      .lock()
      .await
      .data
      .radarr_data
      .add_movie_search
      .clone()
      .ok_or(anyhow!("Encountered a race condition"));

    match search {
      Ok(search_string) => {
        let request_props = self
          .radarr_request_props_from(
            format!(
              "{}?term={}",
              RadarrEvent::SearchNewMovie.resource(),
              encode(&search_string.text)
            )
            .as_str(),
            RequestMethod::Get,
            None::<()>,
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
          .await;
      }
      Err(e) => {
        warn!(
          "Encountered a race condition: {e}\n \
          This is most likely caused by the user trying to navigate between modals rapidly. \
          Ignoring search request."
        );
      }
    }
  }

  async fn start_task(&mut self) {
    let task_name = self
      .app
      .lock()
      .await
      .data
      .radarr_data
      .tasks
      .current_selection()
      .task_name
      .clone();

    info!("Starting Radarr task: {task_name}");

    let body = CommandBody { name: task_name };

    let request_props = self
      .radarr_request_props_from(
        RadarrEvent::StartTask.resource(),
        RequestMethod::Post,
        Some(body),
      )
      .await;

    self
      .handle_request::<CommandBody, Value>(request_props, |_, _| ())
      .await;
  }

  async fn test_indexer(&mut self) {
    let id = self
      .app
      .lock()
      .await
      .data
      .radarr_data
      .indexers
      .current_selection()
      .id;
    info!("Testing Radarr indexer with ID: {id}");

    info!("Fetching indexer details for indexer with ID: {id}");

    let request_props = self
      .radarr_request_props_from(
        format!("{}/{id}", RadarrEvent::GetIndexers.resource()).as_str(),
        RequestMethod::Get,
        None::<()>,
      )
      .await;

    let mut test_body: Value = Value::default();

    self
      .handle_request::<(), Value>(request_props, |detailed_indexer_body, _| {
        test_body = detailed_indexer_body;
      })
      .await;

    info!("Testing indexer");

    let mut request_props = self
      .radarr_request_props_from(
        RadarrEvent::TestIndexer.resource(),
        RequestMethod::Post,
        Some(test_body),
      )
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
      .await;
  }

  async fn test_all_indexers(&mut self) {
    info!("Testing all indexers");

    let mut request_props = self
      .radarr_request_props_from(
        RadarrEvent::TestAllIndexers.resource(),
        RequestMethod::Post,
        None,
      )
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
      .await;
  }

  async fn trigger_automatic_search(&mut self) {
    let (movie_id, tmdb_id) = self.extract_movie_id().await;
    info!("Searching indexers for movie with TMDB id {tmdb_id} and with Radarr id: {movie_id}");
    let body = MovieCommandBody {
      name: "MoviesSearch".to_owned(),
      movie_ids: vec![movie_id],
    };

    let request_props = self
      .radarr_request_props_from(
        RadarrEvent::TriggerAutomaticSearch.resource(),
        RequestMethod::Post,
        Some(body),
      )
      .await;

    self
      .handle_request::<MovieCommandBody, Value>(request_props, |_, _| ())
      .await;
  }

  async fn update_all_movies(&mut self) {
    info!("Updating all movies");
    let body = MovieCommandBody {
      name: "RefreshMovie".to_owned(),
      movie_ids: Vec::new(),
    };

    let request_props = self
      .radarr_request_props_from(
        RadarrEvent::UpdateAllMovies.resource(),
        RequestMethod::Post,
        Some(body),
      )
      .await;

    self
      .handle_request::<MovieCommandBody, Value>(request_props, |_, _| ())
      .await;
  }

  async fn update_and_scan(&mut self) {
    let (movie_id, tmdb_id) = self.extract_movie_id().await;
    info!("Updating and scanning movie with TMDB id {tmdb_id} and with Radarr id: {movie_id}");
    let body = MovieCommandBody {
      name: "RefreshMovie".to_owned(),
      movie_ids: vec![movie_id],
    };

    let request_props = self
      .radarr_request_props_from(
        RadarrEvent::UpdateAndScan.resource(),
        RequestMethod::Post,
        Some(body),
      )
      .await;

    self
      .handle_request::<MovieCommandBody, Value>(request_props, |_, _| ())
      .await;
  }

  async fn update_collections(&mut self) {
    info!("Updating collections");
    let body = CommandBody {
      name: "RefreshCollections".to_owned(),
    };

    let request_props = self
      .radarr_request_props_from(
        RadarrEvent::UpdateCollections.resource(),
        RequestMethod::Post,
        Some(body),
      )
      .await;

    self
      .handle_request::<CommandBody, Value>(request_props, |_, _| ())
      .await;
  }

  async fn update_downloads(&mut self) {
    info!("Updating downloads");
    let body = CommandBody {
      name: "RefreshMonitoredDownloads".to_owned(),
    };

    let request_props = self
      .radarr_request_props_from(
        RadarrEvent::UpdateDownloads.resource(),
        RequestMethod::Post,
        Some(body),
      )
      .await;

    self
      .handle_request::<CommandBody, Value>(request_props, |_, _| ())
      .await;
  }

  async fn radarr_request_props_from<T: Serialize + Debug>(
    &self,
    resource: &str,
    method: RequestMethod,
    body: Option<T>,
  ) -> RequestProps<T> {
    let app = self.app.lock().await;
    let RadarrConfig {
      host,
      port,
      api_token,
    } = &app.config.radarr;
    let uri = format!("http://{host}:{}/api/v3{resource}", port.unwrap_or(7878));

    RequestProps {
      uri,
      method,
      body,
      api_token: api_token.to_owned(),
      ignore_status_code: false,
    }
  }

  async fn extract_and_add_tag_ids_vec(&mut self, edit_tags: String) -> Vec<i64> {
    let tags_map = self.app.lock().await.data.radarr_data.tags_map.clone();
    let tags = edit_tags.clone();
    let missing_tags_vec = edit_tags
      .split(',')
      .filter(|&tag| !tag.is_empty() && tags_map.get_by_right(tag.trim()).is_none())
      .collect::<Vec<&str>>();

    for tag in missing_tags_vec {
      self.add_tag(tag.trim().to_owned()).await;
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

  async fn extract_movie_id(&mut self) -> (i64, i64) {
    let app = self.app.lock().await;
    (
      app.data.radarr_data.movies.current_selection().id,
      app.data.radarr_data.movies.current_selection().tmdb_id,
    )
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

  async fn append_movie_id_param(&mut self, resource: &str) -> String {
    let (movie_id, _) = self.extract_movie_id().await;
    format!("{resource}?movieId={movie_id}")
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
