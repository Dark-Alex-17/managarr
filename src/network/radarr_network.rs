use std::fmt::Debug;

use indoc::formatdoc;
use log::{debug, info};
use serde::Serialize;
use serde_json::{json, Number, Value};
use urlencoding::encode;

use crate::app::RadarrConfig;
use crate::models::radarr_models::{
  AddMovieBody, AddMovieSearchResult, AddOptions, AddRootFolderBody, Collection, CollectionMovie,
  CommandBody, Credit, CreditType, DiskSpace, DownloadRecord, DownloadsResponse, Indexer,
  IndexerSettings, LogResponse, Movie, MovieCommandBody, MovieHistoryItem, QualityProfile,
  QueueEvent, Release, ReleaseDownloadBody, RootFolder, SystemStatus, Tag, Task, Update,
};
use crate::models::servarr_data::radarr_data::ActiveRadarrBlock;
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
  DeleteDownload,
  DeleteIndexer,
  DeleteMovie,
  DeleteRootFolder,
  DownloadRelease,
  EditCollection,
  EditMovie,
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
  TriggerAutomaticSearch,
  UpdateAllMovies,
  UpdateAndScan,
  UpdateCollections,
  UpdateDownloads,
  UpdateIndexerSettings,
}

impl RadarrEvent {
  const fn resource(self) -> &'static str {
    match self {
      RadarrEvent::GetCollections | RadarrEvent::EditCollection => "/collection",
      RadarrEvent::GetDownloads | RadarrEvent::DeleteDownload => "/queue",
      RadarrEvent::GetIndexers | RadarrEvent::DeleteIndexer => "/indexer",
      RadarrEvent::GetIndexerSettings | RadarrEvent::UpdateIndexerSettings => "/config/indexer",
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
      RadarrEvent::DeleteDownload => self.delete_download().await,
      RadarrEvent::DeleteIndexer => self.delete_indexer().await,
      RadarrEvent::DeleteMovie => self.delete_movie().await,
      RadarrEvent::DeleteRootFolder => self.delete_root_folder().await,
      RadarrEvent::DownloadRelease => self.download_release().await,
      RadarrEvent::EditCollection => self.edit_collection().await,
      RadarrEvent::EditMovie => self.edit_movie().await,
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
      RadarrEvent::TriggerAutomaticSearch => self.trigger_automatic_search().await,
      RadarrEvent::UpdateAllMovies => self.update_all_movies().await,
      RadarrEvent::UpdateAndScan => self.update_and_scan().await,
      RadarrEvent::UpdateCollections => self.update_collections().await,
      RadarrEvent::UpdateDownloads => self.update_downloads().await,
      RadarrEvent::UpdateIndexerSettings => self.update_indexer_settings().await,
    }
  }

  async fn add_movie(&mut self) {
    info!("Adding new movie to Radarr");
    let body = {
      let quality_profile_id = self.extract_quality_profile_id().await;
      let tag_ids_vec = self.extract_and_add_tag_ids_vec().await;
      let app = self.app.lock().await;
      let root_folders = app.data.radarr_data.root_folders.items.to_vec();
      let (tmdb_id, title) = if let Route::Radarr(active_radarr_block, _) = app.get_current_route()
      {
        if *active_radarr_block == ActiveRadarrBlock::CollectionDetails {
          let CollectionMovie { tmdb_id, title, .. } =
            app.data.radarr_data.collection_movies.current_selection();
          (tmdb_id, title.text.clone())
        } else {
          let AddMovieSearchResult { tmdb_id, title, .. } =
            app.data.radarr_data.add_searched_movies.current_selection();
          (tmdb_id, title.text.clone())
        }
      } else {
        let AddMovieSearchResult { tmdb_id, title, .. } =
          app.data.radarr_data.add_searched_movies.current_selection();
        (tmdb_id, title.text.clone())
      };

      let RootFolder { path, .. } = root_folders
        .iter()
        .filter(|folder| folder.accessible)
        .reduce(|a, b| {
          if a.free_space.as_u64().unwrap() > b.free_space.as_u64().unwrap() {
            a
          } else {
            b
          }
        })
        .unwrap();
      let monitor = app
        .data
        .radarr_data
        .monitor_list
        .current_selection()
        .to_string();
      let minimum_availability = app
        .data
        .radarr_data
        .minimum_availability_list
        .current_selection()
        .to_string();

      AddMovieBody {
        tmdb_id: tmdb_id.as_u64().unwrap(),
        title,
        root_folder_path: path.to_owned(),
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

    debug!("Add movie body: {:?}", body);

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
    let body = AddRootFolderBody {
      path: self.app.lock().await.data.radarr_data.edit_path.drain(),
    };

    debug!("Add root folder body: {:?}", body);

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
        app
          .data
          .radarr_data
          .tags_map
          .insert(tag.id.as_u64().unwrap(), tag.label);
      })
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
      .id
      .as_u64()
      .unwrap();

    info!(
      "Deleting Radarr download for download with id: {}",
      download_id
    );

    let request_props = self
      .radarr_request_props_from(
        format!("{}/{}", RadarrEvent::DeleteDownload.resource(), download_id).as_str(),
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
      .id
      .as_u64()
      .unwrap();

    info!(
      "Deleting Radarr indexer for indexer with id: {}",
      indexer_id
    );

    let request_props = self
      .radarr_request_props_from(
        format!("{}/{}", RadarrEvent::DeleteIndexer.resource(), indexer_id).as_str(),
        RequestMethod::Delete,
        None::<()>,
      )
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await;
  }

  async fn delete_movie(&mut self) {
    let movie_id = self.extract_movie_id().await;
    let delete_files = self.app.lock().await.data.radarr_data.delete_movie_files;
    let add_import_exclusion = self.app.lock().await.data.radarr_data.add_list_exclusion;

    info!(
      "Deleting Radarr movie with id: {} with deleteFiles={} and addImportExclusion={}",
      movie_id, delete_files, add_import_exclusion
    );

    let request_props = self
      .radarr_request_props_from(
        format!(
          "{}/{}?deleteFiles={}&addImportExclusion={}",
          RadarrEvent::DeleteMovie.resource(),
          movie_id,
          delete_files,
          add_import_exclusion
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
      .id
      .as_u64()
      .unwrap();

    info!(
      "Deleting Radarr root folder for folder with id: {}",
      root_folder_id
    );

    let request_props = self
      .radarr_request_props_from(
        format!(
          "{}/{}",
          RadarrEvent::DeleteRootFolder.resource(),
          root_folder_id
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
    let (guid, title, indexer_id) = {
      let app = self.app.lock().await;
      let Release {
        guid,
        title,
        indexer_id,
        ..
      } = app.data.radarr_data.movie_releases.current_selection();

      (guid.clone(), title.clone(), indexer_id.as_u64().unwrap())
    };

    info!("Downloading release: {}", title);

    let download_release_body = ReleaseDownloadBody { guid, indexer_id };

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

  async fn edit_collection(&mut self) {
    info!("Editing Radarr collection");

    info!("Fetching collection details");
    let collection_id = self.extract_collection_id().await;
    let request_props = self
      .radarr_request_props_from(
        format!(
          "{}/{}",
          RadarrEvent::GetCollections.resource(),
          collection_id
        )
        .as_str(),
        RequestMethod::Get,
        None::<()>,
      )
      .await;

    self
      .handle_request::<(), Value>(request_props, |detailed_collection_body, mut app| {
        app.response = detailed_collection_body.to_string()
      })
      .await;

    info!("Constructing edit collection body");

    let body = {
      let quality_profile_id = self.extract_quality_profile_id().await;
      let mut app = self.app.lock().await;
      let response = app.response.drain(..).collect::<String>();
      let mut detailed_collection_body: Value = serde_json::from_str(&response).unwrap();

      let root_folder_path: String = app.data.radarr_data.edit_path.drain();

      let monitored = app.data.radarr_data.edit_monitored.unwrap_or_default();
      let search_on_add = app.data.radarr_data.edit_search_on_add.unwrap_or_default();
      let minimum_availability = app
        .data
        .radarr_data
        .minimum_availability_list
        .current_selection()
        .to_string();

      *detailed_collection_body.get_mut("monitored").unwrap() = json!(monitored);
      *detailed_collection_body
        .get_mut("minimumAvailability")
        .unwrap() = json!(minimum_availability);
      *detailed_collection_body
        .get_mut("qualityProfileId")
        .unwrap() = json!(quality_profile_id);
      *detailed_collection_body.get_mut("rootFolderPath").unwrap() = json!(root_folder_path);
      *detailed_collection_body.get_mut("searchOnAdd").unwrap() = json!(search_on_add);

      detailed_collection_body
    };

    debug!("Edit collection body: {:?}", body);

    let request_props = self
      .radarr_request_props_from(
        format!(
          "{}/{}",
          RadarrEvent::EditCollection.resource(),
          collection_id
        )
        .as_str(),
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

    info!("Fetching movie details");
    let movie_id = self.extract_movie_id().await;
    let request_props = self
      .radarr_request_props_from(
        format!("{}/{}", RadarrEvent::GetMovieDetails.resource(), movie_id).as_str(),
        RequestMethod::Get,
        None::<()>,
      )
      .await;

    self
      .handle_request::<(), Value>(request_props, |detailed_movie_body, mut app| {
        app.response = detailed_movie_body.to_string()
      })
      .await;

    info!("Constructing edit movie body");

    let body = {
      let quality_profile_id = self.extract_quality_profile_id().await;
      let tag_ids_vec = self.extract_and_add_tag_ids_vec().await;
      let mut app = self.app.lock().await;
      let response = app.response.drain(..).collect::<String>();
      let mut detailed_movie_body: Value = serde_json::from_str(&response).unwrap();

      let path: String = app.data.radarr_data.edit_path.drain();

      let monitored = app.data.radarr_data.edit_monitored.unwrap_or_default();
      let minimum_availability = app
        .data
        .radarr_data
        .minimum_availability_list
        .current_selection()
        .to_string();

      *detailed_movie_body.get_mut("monitored").unwrap() = json!(monitored);
      *detailed_movie_body.get_mut("minimumAvailability").unwrap() = json!(minimum_availability);
      *detailed_movie_body.get_mut("qualityProfileId").unwrap() = json!(quality_profile_id);
      *detailed_movie_body.get_mut("path").unwrap() = json!(path);
      *detailed_movie_body.get_mut("tags").unwrap() = json!(tag_ids_vec);

      detailed_movie_body
    };

    debug!("Edit movie body: {:?}", body);

    let request_props = self
      .radarr_request_props_from(
        format!("{}/{}", RadarrEvent::EditMovie.resource(), movie_id).as_str(),
        RequestMethod::Put,
        Some(body),
      )
      .await;

    self
      .handle_request::<Value, ()>(request_props, |_, _| ())
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
      .handle_request::<(), Vec<Collection>>(request_props, |collections_vec, mut app| {
        app.data.radarr_data.collections.set_items(collections_vec);
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
          .cloned()
          .filter(|credit| credit.credit_type == CreditType::Cast)
          .collect();
        let crew_vec: Vec<Credit> = credit_vec
          .iter()
          .cloned()
          .filter(|credit| credit.credit_type == CreditType::Crew)
          .collect();

        app.data.radarr_data.movie_cast.set_items(cast_vec);
        app.data.radarr_data.movie_crew.set_items(crew_vec);
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
        app.data.radarr_data.indexer_settings = Some(indexer_settings);
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
      "{}?pageSize=100&sortDirection=descending&sortKey=time",
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

    let movie_id = self.extract_movie_id().await;
    let request_props = self
      .radarr_request_props_from(
        format!("{}/{}", RadarrEvent::GetMovieDetails.resource(), movie_id).as_str(),
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
        let (hours, minutes) = convert_runtime(runtime.as_u64().unwrap());
        let size = convert_to_gb(size_on_disk.as_u64().unwrap());
        let quality_profile = app
          .data
          .radarr_data
          .quality_profile_map
          .get_by_left(&quality_profile_id.as_u64().unwrap())
          .unwrap()
          .to_owned();
        let imdb_rating = if let Some(rating) = ratings.imdb {
          if let Some(value) = rating.value.as_f64() {
            format!("{:.1}", value)
          } else {
            "".to_owned()
          }
        } else {
          "".to_owned()
        };

        let tmdb_rating = if let Some(rating) = ratings.tmdb {
          if let Some(value) = rating.value.as_f64() {
            format!("{}%", (value * 10f64).ceil())
          } else {
            "".to_owned()
          }
        } else {
          "".to_owned()
        };

        let rotten_tomatoes_rating = if let Some(rating) = ratings.rotten_tomatoes {
          if let Some(value) = rating.value.as_u64() {
            format!("{}%", value)
          } else {
            "".to_owned()
          }
        } else {
          "".to_owned()
        };

        let status = get_movie_status(has_file, &app.data.radarr_data.downloads.items, id);
        let collection = collection.unwrap_or_default();

        app.data.radarr_data.movie_details = ScrollableText::with_string(formatdoc!(
          "Title: {}
          Year: {}
          Runtime: {}h {}m
          Rating: {}
          Collection: {}
          Status: {}
          Description: {}
          TMDB: {}
          IMDB: {}
          Rotten Tomatoes: {}
          Quality Profile: {}
          Size: {:.2} GB
          Path: {}
          Studio: {}
          Genres: {}",
          title,
          year,
          hours,
          minutes,
          certification.unwrap_or_default(),
          collection.title,
          status,
          overview,
          tmdb_rating,
          imdb_rating,
          rotten_tomatoes_rating,
          quality_profile,
          size,
          path,
          studio,
          genres.join(", ")
        ));

        if let Some(file) = movie_file {
          app.data.radarr_data.file_details = formatdoc!(
            "Relative Path: {}
              Absolute Path: {}
              Size: {:.2} GB
              Date Added: {}",
            file.relative_path,
            file.path,
            size,
            file.date_added
          );

          if let Some(media_info) = file.media_info {
            app.data.radarr_data.audio_details = formatdoc!(
              "Bitrate: {}
              Channels: {:.1}
              Codec: {}
              Languages: {}
              Stream Count: {}",
              media_info.audio_bitrate.as_u64().unwrap(),
              media_info.audio_channels.as_f64().unwrap(),
              media_info.audio_codec.unwrap_or_default(),
              media_info.audio_languages.unwrap_or_default(),
              media_info.audio_stream_count.as_u64().unwrap()
            );

            app.data.radarr_data.video_details = formatdoc!(
              "Bit Depth: {}
              Bitrate: {}
              Codec: {}
              FPS: {}
              Resolution: {}
              Scan Type: {}
              Runtime: {}",
              media_info.video_bit_depth.as_u64().unwrap(),
              media_info.video_bitrate.as_u64().unwrap(),
              media_info.video_codec,
              media_info.video_fps.as_f64().unwrap(),
              media_info.resolution,
              media_info.scan_type,
              media_info.run_time
            );
          }
        }
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
        app
          .data
          .radarr_data
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
      .handle_request::<(), Vec<Movie>>(request_props, |movie_vec, mut app| {
        app.data.radarr_data.movies.set_items(movie_vec)
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
          .map(|profile| (profile.id.as_u64().unwrap(), profile.name))
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
    let movie_id = self.extract_movie_id().await;
    info!("Fetching releases for movie with id: {}", movie_id);

    let request_props = self
      .radarr_request_props_from(
        format!(
          "{}?movieId={}",
          RadarrEvent::GetReleases.resource(),
          movie_id
        )
        .as_str(),
        RequestMethod::Get,
        None::<()>,
      )
      .await;

    self
      .handle_request::<(), Vec<Release>>(request_props, |release_vec, mut app| {
        app.data.radarr_data.movie_releases.set_items(release_vec)
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
          .map(|tag| (tag.id.as_u64().unwrap(), tag.label))
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
                .map(|change| format!("  * {}", change))
                .collect::<Vec<String>>()
                .join("\n")
            };

            let mut update_info = formatdoc!(
              "{} - {} {}
              {}",
              update.version,
              update.release_date,
              install_status,
              "-".repeat(200)
            );

            if let Some(new_changes) = update.changes.new {
              let changes = vec_to_bullet_points(new_changes);
              update_info = formatdoc!(
                "{}
              New:
              {}",
                update_info,
                changes
              )
            }

            if let Some(fixes) = update.changes.fixed {
              let fixes = vec_to_bullet_points(fixes);
              update_info = formatdoc!(
                "{}
              Fixed:
              {}",
                update_info,
                fixes
              );
            }

            update_info
          })
          .reduce(|version_1, version_2| format!("{}\n\n\n{}", version_1, version_2))
          .unwrap();

        app.data.radarr_data.updates = ScrollableText::with_string(formatdoc!(
          "{}
          
          {}",
          format!(
            "The latest version of Radarr is {} installed",
            latest_installed
          ),
          updates
        ));
      })
      .await;
  }

  async fn search_movie(&mut self) {
    info!("Searching for specific Radarr movie");

    let search_string = self.app.lock().await.data.radarr_data.search.text.clone();
    let request_props = self
      .radarr_request_props_from(
        format!(
          "{}?term={}",
          RadarrEvent::SearchNewMovie.resource(),
          encode(&search_string)
        )
        .as_str(),
        RequestMethod::Get,
        None::<()>,
      )
      .await;

    self
      .handle_request::<(), Vec<AddMovieSearchResult>>(request_props, |movie_vec, mut app| {
        if movie_vec.is_empty() {
          app.pop_and_push_navigation_stack(ActiveRadarrBlock::AddMovieEmptySearchResults.into());
        } else {
          app
            .data
            .radarr_data
            .add_searched_movies
            .set_items(movie_vec);
        }
      })
      .await;
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

    info!("Starting Radarr task: {}", task_name);

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

  async fn trigger_automatic_search(&mut self) {
    let movie_id = self.extract_movie_id().await;
    info!("Searching indexers for movie with id: {}", movie_id);
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
    let movie_id = self.extract_movie_id().await;
    info!("Updating and scanning movie with id: {}", movie_id);
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

  async fn update_indexer_settings(&mut self) {
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

    debug!("Indexer settings body: {:?}", body);

    let request_props = self
      .radarr_request_props_from(
        RadarrEvent::UpdateIndexerSettings.resource(),
        RequestMethod::Put,
        Some(body),
      )
      .await;

    self
      .handle_request::<IndexerSettings, Value>(request_props, |_, _| {})
      .await;

    self.app.lock().await.data.radarr_data.indexer_settings = None;
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
    let uri = format!(
      "http://{}:{}/api/v3{}",
      host,
      port.unwrap_or(7878),
      resource
    );

    RequestProps {
      uri,
      method,
      body,
      api_token: api_token.to_owned(),
    }
  }

  async fn extract_quality_profile_id(&mut self) -> u64 {
    let app = self.app.lock().await;
    let quality_profile = app
      .data
      .radarr_data
      .quality_profile_list
      .current_selection();
    *app
      .data
      .radarr_data
      .quality_profile_map
      .iter()
      .filter(|(_, value)| *value == quality_profile)
      .map(|(key, _)| key)
      .next()
      .unwrap()
  }

  async fn extract_and_add_tag_ids_vec(&mut self) -> Vec<u64> {
    let tags_map = self.app.lock().await.data.radarr_data.tags_map.clone();
    let edit_tags = self.app.lock().await.data.radarr_data.edit_tags.drain();
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

  async fn extract_movie_id(&mut self) -> u64 {
    if !self
      .app
      .lock()
      .await
      .data
      .radarr_data
      .filtered_movies
      .items
      .is_empty()
    {
      self
        .app
        .lock()
        .await
        .data
        .radarr_data
        .filtered_movies
        .current_selection()
        .id
        .as_u64()
        .unwrap()
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
        .as_u64()
        .unwrap()
    }
  }

  async fn extract_collection_id(&mut self) -> u64 {
    if !self
      .app
      .lock()
      .await
      .data
      .radarr_data
      .filtered_collections
      .items
      .is_empty()
    {
      self
        .app
        .lock()
        .await
        .data
        .radarr_data
        .filtered_collections
        .current_selection()
        .id
        .as_u64()
        .unwrap()
    } else {
      self
        .app
        .lock()
        .await
        .data
        .radarr_data
        .collections
        .current_selection()
        .id
        .as_u64()
        .unwrap()
    }
  }

  async fn append_movie_id_param(&mut self, resource: &str) -> String {
    let movie_id = self.extract_movie_id().await;
    format!("{}?movieId={}", resource, movie_id)
  }
}

fn get_movie_status(has_file: bool, downloads_vec: &[DownloadRecord], movie_id: Number) -> String {
  if !has_file {
    if let Some(download) = downloads_vec
      .iter()
      .find(|&download| download.movie_id.as_u64().unwrap() == movie_id.as_u64().unwrap())
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
