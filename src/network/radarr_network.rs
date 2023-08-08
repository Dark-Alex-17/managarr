use std::fmt::Debug;

use indoc::formatdoc;
use log::{debug, info};
use serde::Serialize;
use serde_json::{json, Number, Value};
use urlencoding::encode;

use crate::app::radarr::ActiveRadarrBlock;
use crate::app::RadarrConfig;
use crate::models::radarr_models::{
  AddMovieBody, AddMovieSearchResult, AddOptions, Collection, CollectionMovie, CommandBody, Credit,
  CreditType, DiskSpace, DownloadRecord, DownloadsResponse, Movie, MovieCommandBody,
  MovieHistoryItem, QualityProfile, Release, ReleaseDownloadBody, RootFolder, SystemStatus, Tag,
};
use crate::models::{Route, ScrollableText};
use crate::network::{Network, NetworkEvent, RequestMethod, RequestProps};
use crate::utils::{convert_runtime, convert_to_gb};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum RadarrEvent {
  AddMovie,
  DeleteDownload,
  DeleteMovie,
  DownloadRelease,
  EditMovie,
  GetCollections,
  GetDownloads,
  GetMovieCredits,
  GetMovieDetails,
  GetMovieHistory,
  GetMovies,
  GetOverview,
  GetQualityProfiles,
  GetReleases,
  GetRootFolders,
  GetStatus,
  GetTags,
  HealthCheck,
  SearchNewMovie,
  TriggerAutomaticSearch,
  UpdateAllMovies,
  UpdateAndScan,
  UpdateCollections,
  UpdateDownloads,
}

impl RadarrEvent {
  const fn resource(self) -> &'static str {
    match self {
      RadarrEvent::GetCollections => "/collection",
      RadarrEvent::GetDownloads | RadarrEvent::DeleteDownload => "/queue",
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
      RadarrEvent::GetRootFolders => "/rootfolder",
      RadarrEvent::GetStatus => "/system/status",
      RadarrEvent::GetTags => "/tag",
      RadarrEvent::TriggerAutomaticSearch
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

impl<'a> Network<'a> {
  pub async fn handle_radarr_event(&self, radarr_event: RadarrEvent) {
    match radarr_event {
      RadarrEvent::AddMovie => self.add_movie().await,
      RadarrEvent::DeleteMovie => self.delete_movie().await,
      RadarrEvent::DeleteDownload => self.delete_download().await,
      RadarrEvent::DownloadRelease => self.download_release().await,
      RadarrEvent::EditMovie => self.edit_movie().await,
      RadarrEvent::GetCollections => self.get_collections().await,
      RadarrEvent::GetDownloads => self.get_downloads().await,
      RadarrEvent::GetMovieCredits => self.get_credits().await,
      RadarrEvent::GetMovieDetails => self.get_movie_details().await,
      RadarrEvent::GetMovieHistory => self.get_movie_history().await,
      RadarrEvent::GetMovies => self.get_movies().await,
      RadarrEvent::GetOverview => self.get_diskspace().await,
      RadarrEvent::GetQualityProfiles => self.get_quality_profiles().await,
      RadarrEvent::GetReleases => self.get_releases().await,
      RadarrEvent::GetRootFolders => self.get_root_folders().await,
      RadarrEvent::GetStatus => self.get_status().await,
      RadarrEvent::GetTags => self.get_tags().await,
      RadarrEvent::HealthCheck => self.get_healthcheck().await,
      RadarrEvent::SearchNewMovie => self.search_movie().await,
      RadarrEvent::TriggerAutomaticSearch => self.trigger_automatic_search().await,
      RadarrEvent::UpdateAllMovies => self.update_all_movies().await,
      RadarrEvent::UpdateAndScan => self.update_and_scan().await,
      RadarrEvent::UpdateCollections => self.update_collections().await,
      RadarrEvent::UpdateDownloads => self.update_downloads().await,
    }
  }

  async fn get_healthcheck(&self) {
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

  async fn get_diskspace(&self) {
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

  async fn get_status(&self) {
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

  async fn get_movies(&self) {
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

  async fn get_releases(&self) {
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

  async fn search_movie(&self) {
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

  async fn trigger_automatic_search(&self) {
    let movie_id = self.extract_movie_id().await;
    info!("Searching indexers for movie with id: {}", movie_id);
    let body = MovieCommandBody {
      name: "MovieSearch".to_owned(),
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

  async fn update_and_scan(&self) {
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

  async fn update_all_movies(&self) {
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

  async fn update_downloads(&self) {
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

  async fn update_collections(&self) {
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

  async fn get_movie_details(&self) {
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

  async fn get_movie_history(&self) {
    info!("Fetching Radarr movie history");

    let request_props = self
      .radarr_request_props_from(
        self
          .append_movie_id_param(RadarrEvent::GetMovieHistory.resource())
          .await
          .as_str(),
        RequestMethod::Get,
        None::<()>,
      )
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

  async fn get_collections(&self) {
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

  async fn get_downloads(&self) {
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

  async fn get_quality_profiles(&self) {
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

  async fn get_tags(&self) {
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

  async fn add_tag(&self, tag: String) {
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

  async fn get_root_folders(&self) {
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
        app.data.radarr_data.root_folders = root_folders;
      })
      .await;
  }

  async fn get_credits(&self) {
    info!("Fetching Radarr movie credits");

    let request_props = self
      .radarr_request_props_from(
        self
          .append_movie_id_param(RadarrEvent::GetMovieCredits.resource())
          .await
          .as_str(),
        RequestMethod::Get,
        None::<()>,
      )
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

  async fn delete_movie(&self) {
    let movie_id = self.extract_movie_id().await;

    info!("Deleting Radarr movie with id: {}", movie_id);

    let request_props = self
      .radarr_request_props_from(
        format!("{}/{}", RadarrEvent::DeleteMovie.resource(), movie_id).as_str(),
        RequestMethod::Delete,
        None::<()>,
      )
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await;
  }

  async fn delete_download(&self) {
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

  async fn add_movie(&self) {
    info!("Adding new movie to Radarr");
    let body = {
      let quality_profile_id = self.extract_quality_profile_id().await;
      let tag_ids_vec = self.extract_and_add_tag_ids_vec().await;
      let app = self.app.lock().await;
      let root_folders = app.data.radarr_data.root_folders.to_vec();
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
        .movie_monitor_list
        .current_selection()
        .to_string();
      let minimum_availability = app
        .data
        .radarr_data
        .movie_minimum_availability_list
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

  async fn edit_movie(&self) {
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
      let mut detailed_movie_body: Value = serde_json::from_str(&app.response).unwrap();
      app.response = String::default();

      let path: String = app.data.radarr_data.edit_path.drain();

      let monitored = app.data.radarr_data.edit_monitored.unwrap_or_default();
      let minimum_availability = app
        .data
        .radarr_data
        .movie_minimum_availability_list
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

  async fn download_release(&self) {
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
      .handle_request::<ReleaseDownloadBody, ()>(request_props, |_, _| ())
      .await;
  }

  async fn extract_quality_profile_id(&self) -> u64 {
    let app = self.app.lock().await;
    let quality_profile = app
      .data
      .radarr_data
      .movie_quality_profile_list
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

  async fn extract_and_add_tag_ids_vec(&self) -> Vec<u64> {
    let tags_map = self.app.lock().await.data.radarr_data.tags_map.clone();
    let edit_tags = self
      .app
      .lock()
      .await
      .data
      .radarr_data
      .edit_tags
      .text
      .clone();
    let missing_tags_vec = edit_tags
      .split(',')
      .filter(|&tag| !tag.is_empty() && tags_map.get_by_right(tag.trim()).is_none())
      .collect::<Vec<&str>>();

    for tag in missing_tags_vec {
      self.add_tag(tag.trim().to_owned()).await;
    }

    let app = self.app.lock().await;
    app
      .data
      .radarr_data
      .edit_tags
      .text
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

  async fn extract_movie_id(&self) -> u64 {
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

  async fn append_movie_id_param(&self, resource: &str) -> String {
    let movie_id = self.extract_movie_id().await;
    format!("{}?movieId={}", resource, movie_id)
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

#[cfg(test)]
mod test {
  use std::sync::Arc;

  use bimap::BiMap;
  use chrono::{DateTime, Utc};
  use mockito::{Matcher, Mock, Server, ServerGuard};
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use serde_json::{json, Value};
  use strum::IntoEnumIterator;
  use tokio::sync::Mutex;

  use crate::app::radarr::ActiveRadarrBlock;
  use crate::models::radarr_models::{
    CollectionMovie, Language, MediaInfo, MinimumAvailability, Monitor, MovieFile, Quality,
    QualityWrapper, Rating, RatingsList,
  };
  use crate::models::HorizontallyScrollableText;
  use crate::App;

  use super::*;

  const MOVIE_JSON: &str = r#"{
        "id": 1,
        "title": "Test",
        "tmdbId": 1234,
        "originalLanguage": {
          "name": "English"
        },
        "sizeOnDisk": 3543348019,
        "status": "Downloaded",
        "overview": "Blah blah blah",
        "path": "/nfs/movies",
        "studio": "21st Century Alex",
        "genres": ["cool", "family", "fun"],
        "year": 2023,
        "monitored": true,
        "hasFile": true,
        "runtime": 120,
        "qualityProfileId": 2222,
        "minimumAvailability": "announced",
        "certification": "R",
        "tags": [1],
        "ratings": {
          "imdb": {
            "value": 9.9
          },
          "tmdb": {
            "value": 9.9
          },
          "rottenTomatoes": {
            "value": 9.9
          }
        },
        "movieFile": {
          "relativePath": "Test.mkv",
          "path": "/nfs/movies/Test.mkv",
          "dateAdded": "2022-12-30T07:37:56Z",
          "mediaInfo": {
            "audioBitrate": 0,
            "audioChannels": 7.1,
            "audioCodec": "AAC",
            "audioLanguages": "eng",
            "audioStreamCount": 1,
            "videoBitDepth": 10,
            "videoBitrate": 0,
            "videoCodec": "x265",
            "videoFps": 23.976,
            "resolution": "1920x804",
            "runTime": "2:00:00",
            "scanType": "Progressive"
          }
        },
        "collection": {
          "title": "Test Collection",
          "searchOnAdd": true,
          "overview": "Collection blah blah blah",
          "qualityProfileId": 2222,
          "movies": [
            {
              "title": "Test",
              "overview": "Collection blah blah blah",
              "year": 2023,
              "runtime": 120,
              "tmdbId": 1234,
              "genres": ["cool", "family", "fun"],
              "ratings": {
                "imdb": {
                  "value": 9.9
                },
                "tmdb": {
                  "value": 9.9
                },
                "rottenTomatoes": {
                  "value": 9.9
                }
              }
            }
          ]
        }
      }"#;

  #[rstest]
  fn test_resource_movie(
    #[values(
      RadarrEvent::AddMovie,
      RadarrEvent::GetMovies,
      RadarrEvent::GetMovieDetails,
      RadarrEvent::DeleteMovie
    )]
    event: RadarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/movie");
  }

  #[rstest]
  fn test_resource_release(
    #[values(RadarrEvent::GetReleases, RadarrEvent::DownloadRelease)] event: RadarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/release");
  }

  #[rstest]
  fn test_resource_queue(
    #[values(RadarrEvent::GetDownloads, RadarrEvent::DeleteDownload)] event: RadarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/queue");
  }

  #[rstest]
  fn test_resource_command(
    #[values(
      RadarrEvent::TriggerAutomaticSearch,
      RadarrEvent::UpdateAndScan,
      RadarrEvent::UpdateAllMovies,
      RadarrEvent::UpdateDownloads,
      RadarrEvent::UpdateCollections
    )]
    event: RadarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/command");
  }

  #[rstest]
  fn test_resource(
    #[values(
      RadarrEvent::GetCollections,
      RadarrEvent::SearchNewMovie,
      RadarrEvent::GetMovieCredits,
      RadarrEvent::GetMovieHistory,
      RadarrEvent::GetOverview,
      RadarrEvent::GetQualityProfiles,
      RadarrEvent::GetRootFolders,
      RadarrEvent::GetStatus,
      RadarrEvent::HealthCheck
    )]
    event: RadarrEvent,
  ) {
    let expected_resource = match event {
      RadarrEvent::GetCollections => "/collection",
      RadarrEvent::SearchNewMovie => "/movie/lookup",
      RadarrEvent::GetMovieCredits => "/credit",
      RadarrEvent::GetMovieHistory => "/history/movie",
      RadarrEvent::GetOverview => "/diskspace",
      RadarrEvent::GetQualityProfiles => "/qualityprofile",
      RadarrEvent::GetRootFolders => "/rootfolder",
      RadarrEvent::GetStatus => "/system/status",
      RadarrEvent::HealthCheck => "/health",
      _ => "",
    };

    assert_str_eq!(event.resource(), expected_resource);
  }

  #[test]
  fn test_from_radarr_event() {
    assert_eq!(
      NetworkEvent::Radarr(RadarrEvent::HealthCheck),
      NetworkEvent::from(RadarrEvent::HealthCheck)
    );
  }

  #[tokio::test]
  async fn test_handle_get_healthcheck_event() {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      None,
      RadarrEvent::HealthCheck.resource(),
    )
    .await;
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network.handle_radarr_event(RadarrEvent::HealthCheck).await;

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_get_diskspace_event() {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(json!([
        {
          "freeSpace": 1111,
          "totalSpace": 2222,
        },
        {
          "freeSpace": 3333,
          "totalSpace": 4444
        }
      ])),
      RadarrEvent::GetOverview.resource(),
    )
    .await;
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network.handle_radarr_event(RadarrEvent::GetOverview).await;

    async_server.assert_async().await;
    assert_eq!(
      app_arc.lock().await.data.radarr_data.disk_space_vec,
      vec![
        DiskSpace {
          free_space: Number::from(1111),
          total_space: Number::from(2222)
        },
        DiskSpace {
          free_space: Number::from(3333),
          total_space: Number::from(4444)
        }
      ]
    );
  }

  #[tokio::test]
  async fn test_handle_get_status_event() {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(json!({
        "version": "v1",
        "startTime": "2023-02-25T20:16:43Z"
      })),
      RadarrEvent::GetStatus.resource(),
    )
    .await;
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network.handle_radarr_event(RadarrEvent::GetStatus).await;

    async_server.assert_async().await;
    assert_str_eq!(app_arc.lock().await.data.radarr_data.version, "v1");
    assert_eq!(
      app_arc.lock().await.data.radarr_data.start_time,
      DateTime::from(DateTime::parse_from_rfc3339("2023-02-25T20:16:43Z").unwrap())
        as DateTime<Utc>
    );
  }

  #[tokio::test]
  async fn test_handle_get_movies_event() {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(serde_json::from_str(format!("[ {} ]", MOVIE_JSON).as_str()).unwrap()),
      RadarrEvent::GetMovies.resource(),
    )
    .await;
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network.handle_radarr_event(RadarrEvent::GetMovies).await;

    async_server.assert_async().await;
    assert_eq!(
      app_arc.lock().await.data.radarr_data.movies.items,
      vec![movie()]
    );
  }

  #[tokio::test]
  async fn test_handle_get_releases_event() {
    let release_json = json!([{
      "guid": "1234",
      "protocol": "torrent",
      "age": 1,
      "title": "Test Release",
      "indexer": "kickass torrents",
      "indexerId": 2,
      "size": 1234,
      "rejected": true,
      "rejections": [ "Unknown quality profile", "Release is already mapped" ],
      "seeders": 2,
      "leechers": 1,
      "languages": [ { "name": "English" } ],
      "quality": { "quality": { "name": "HD - 1080p" }}
    }]);
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(release_json),
      format!("{}?movieId=1", RadarrEvent::GetReleases.resource()).as_str(),
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .movies
      .set_items(vec![movie()]);
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network.handle_radarr_event(RadarrEvent::GetReleases).await;

    async_server.assert_async().await;
    assert_eq!(
      app_arc.lock().await.data.radarr_data.movie_releases.items,
      vec![release()]
    );
  }

  #[tokio::test]
  async fn test_handle_search_new_movie_event() {
    let add_movie_search_result_json = json!([{
      "tmdbId": 1234,
      "title": "Test",
      "originalLanguage": { "name": "English" },
      "status": "released",
      "overview": "New movie blah blah blah",
      "genres": ["cool", "family", "fun"],
      "year": 2023,
      "runtime": 120,
      "ratings": {
        "imdb": {
          "value": 9.9
        },
        "tmdb": {
          "value": 9.9
        },
        "rottenTomatoes": {
          "value": 9.9
        }
      }
    }]);
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(add_movie_search_result_json),
      format!(
        "{}?term=test%20term",
        RadarrEvent::SearchNewMovie.resource()
      )
      .as_str(),
    )
    .await;
    app_arc.lock().await.data.radarr_data.search = "test term".to_owned().into();
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network
      .handle_radarr_event(RadarrEvent::SearchNewMovie)
      .await;

    async_server.assert_async().await;
    assert_eq!(
      app_arc
        .lock()
        .await
        .data
        .radarr_data
        .add_searched_movies
        .items,
      vec![add_movie_search_result()]
    );
  }

  #[tokio::test]
  async fn test_handle_search_new_movie_event_no_results() {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(json!([])),
      format!(
        "{}?term=test%20term",
        RadarrEvent::SearchNewMovie.resource()
      )
      .as_str(),
    )
    .await;
    app_arc.lock().await.data.radarr_data.search = "test term".to_owned().into();
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network
      .handle_radarr_event(RadarrEvent::SearchNewMovie)
      .await;

    async_server.assert_async().await;
    assert!(app_arc
      .lock()
      .await
      .data
      .radarr_data
      .add_searched_movies
      .items
      .is_empty());
    assert_eq!(
      app_arc.lock().await.get_current_route(),
      &ActiveRadarrBlock::AddMovieEmptySearchResults.into()
    );
  }

  #[tokio::test]
  async fn test_handle_trigger_automatic_search_event() {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "MovieSearch",
        "movieIds": [ 1 ]
      })),
      None,
      RadarrEvent::TriggerAutomaticSearch.resource(),
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .movies
      .set_items(vec![movie()]);
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network
      .handle_radarr_event(RadarrEvent::TriggerAutomaticSearch)
      .await;

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_update_and_scan_event() {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "RefreshMovie",
        "movieIds": [ 1 ]
      })),
      None,
      RadarrEvent::UpdateAndScan.resource(),
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .movies
      .set_items(vec![movie()]);
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network
      .handle_radarr_event(RadarrEvent::UpdateAndScan)
      .await;

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_update_all_movies_event() {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "RefreshMovie",
        "movieIds": []
      })),
      None,
      RadarrEvent::UpdateAllMovies.resource(),
    )
    .await;
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network
      .handle_radarr_event(RadarrEvent::UpdateAllMovies)
      .await;

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_update_downloads_event() {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "RefreshMonitoredDownloads"
      })),
      None,
      RadarrEvent::UpdateDownloads.resource(),
    )
    .await;
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network
      .handle_radarr_event(RadarrEvent::UpdateDownloads)
      .await;

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_update_collections_event() {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "RefreshCollections"
      })),
      None,
      RadarrEvent::UpdateCollections.resource(),
    )
    .await;
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network
      .handle_radarr_event(RadarrEvent::UpdateCollections)
      .await;

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_get_movie_details_event() {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(serde_json::from_str(MOVIE_JSON).unwrap()),
      format!("{}/1", RadarrEvent::GetMovieDetails.resource()).as_str(),
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .movies
      .set_items(vec![movie()]);
    app_arc.lock().await.data.radarr_data.quality_profile_map =
      BiMap::from_iter([(2222, "HD - 1080p".to_owned())]);
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network
      .handle_radarr_event(RadarrEvent::GetMovieDetails)
      .await;

    async_server.assert_async().await;
    assert_str_eq!(
      app_arc
        .lock()
        .await
        .data
        .radarr_data
        .movie_details
        .get_text(),
      formatdoc!(
        "Title: Test
          Year: 2023
          Runtime: 2h 0m
          Rating: R
          Collection: Test Collection
          Status: Downloaded
          Description: Blah blah blah
          TMDB: 99%
          IMDB: 9.9
          Rotten Tomatoes: 
          Quality Profile: HD - 1080p
          Size: 3.30 GB
          Path: /nfs/movies
          Studio: 21st Century Alex
          Genres: cool, family, fun"
      )
    );
    assert_str_eq!(
      app_arc.lock().await.data.radarr_data.file_details,
      formatdoc!(
        "Relative Path: Test.mkv
        Absolute Path: /nfs/movies/Test.mkv
        Size: 3.30 GB
        Date Added: 2022-12-30 07:37:56 UTC"
      )
    );
    assert_str_eq!(
      app_arc.lock().await.data.radarr_data.audio_details,
      formatdoc!(
        "Bitrate: 0
        Channels: 7.1
        Codec: AAC
        Languages: eng
        Stream Count: 1"
      )
    );
    assert_str_eq!(
      app_arc.lock().await.data.radarr_data.video_details,
      formatdoc!(
        "Bit Depth: 10
        Bitrate: 0
        Codec: x265
        FPS: 23.976
        Resolution: 1920x804
        Scan Type: Progressive
        Runtime: 2:00:00"
      )
    );
  }

  #[tokio::test]
  async fn test_handle_get_movie_details_event_empty_options_give_correct_defaults() {
    let movie_json_with_missing_fields = json!({
      "id": 1,
      "title": "Test",
      "originalLanguage": {
        "name": "English"
      },
      "sizeOnDisk": 0,
      "status": "Downloaded",
      "overview": "Blah blah blah",
      "path": "/nfs/movies",
      "studio": "21st Century Alex",
      "genres": ["cool", "family", "fun"],
      "year": 2023,
      "monitored": true,
      "hasFile": false,
      "runtime": 120,
      "tmdbId": 1234,
      "qualityProfileId": 2222,
      "tags": [1],
      "minimumAvailability": "released",
      "ratings": {}
    });
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(movie_json_with_missing_fields),
      format!("{}/1", RadarrEvent::GetMovieDetails.resource()).as_str(),
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .movies
      .set_items(vec![movie()]);
    app_arc.lock().await.data.radarr_data.quality_profile_map =
      BiMap::from_iter([(2222, "HD - 1080p".to_owned())]);
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network
      .handle_radarr_event(RadarrEvent::GetMovieDetails)
      .await;

    async_server.assert_async().await;
    assert_str_eq!(
      app_arc
        .lock()
        .await
        .data
        .radarr_data
        .movie_details
        .get_text(),
      formatdoc!(
        "Title: Test
          Year: 2023
          Runtime: 2h 0m
          Rating: 
          Collection: 
          Status: Missing
          Description: Blah blah blah
          TMDB: 
          IMDB: 
          Rotten Tomatoes: 
          Quality Profile: HD - 1080p
          Size: 0.00 GB
          Path: /nfs/movies
          Studio: 21st Century Alex
          Genres: cool, family, fun"
      )
    );
    assert!(app_arc
      .lock()
      .await
      .data
      .radarr_data
      .file_details
      .is_empty());
    assert!(app_arc
      .lock()
      .await
      .data
      .radarr_data
      .audio_details
      .is_empty());
    assert!(app_arc
      .lock()
      .await
      .data
      .radarr_data
      .video_details
      .is_empty());
  }

  #[tokio::test]
  async fn test_handle_get_movie_history_event() {
    let movie_history_item_json = json!([{
      "sourceTitle": "Test",
      "quality": { "quality": { "name": "HD - 1080p" }},
      "languages": [ { "name": "English" } ],
      "date": "2022-12-30T07:37:56Z",
      "eventType": "grabbed"
    }]);
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(movie_history_item_json),
      format!("{}?movieId=1", RadarrEvent::GetMovieHistory.resource()).as_str(),
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .movies
      .set_items(vec![movie()]);
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network
      .handle_radarr_event(RadarrEvent::GetMovieHistory)
      .await;

    async_server.assert_async().await;
    assert_eq!(
      app_arc.lock().await.data.radarr_data.movie_history.items,
      vec![movie_history_item()]
    );
  }

  #[tokio::test]
  async fn test_handle_get_collections_event() {
    let collection_json = json!([{
      "title": "Test Collection",
      "searchOnAdd": true,
      "overview": "Collection blah blah blah",
      "qualityProfileId": 2222,
      "movies": [{
        "title": "Test",
        "overview": "Collection blah blah blah",
        "year": 2023,
        "runtime": 120,
        "tmdbId": 1234,
        "genres": ["cool", "family", "fun"],
        "ratings": {
          "imdb": {
            "value": 9.9
          },
          "tmdb": {
            "value": 9.9
          },
          "rottenTomatoes": {
            "value": 9.9
          }
        }
      }],
    }]);
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(collection_json),
      RadarrEvent::GetCollections.resource(),
    )
    .await;
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network
      .handle_radarr_event(RadarrEvent::GetCollections)
      .await;

    async_server.assert_async().await;
    assert_eq!(
      app_arc.lock().await.data.radarr_data.collections.items,
      vec![collection()]
    );
  }

  #[tokio::test]
  async fn test_handle_get_downloads_event() {
    let downloads_response_json = json!({
      "records": [{
        "title": "Test Download Title",
        "status": "downloading",
        "id": 1,
        "movieId": 1,
        "size": 3543348019u64,
        "sizeleft": 1771674009,
        "outputPath": "/nfs/movies/Test",
        "indexer": "kickass torrents",
        "downloadClient": "transmission",
      }]
    });
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(downloads_response_json),
      RadarrEvent::GetDownloads.resource(),
    )
    .await;
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network.handle_radarr_event(RadarrEvent::GetDownloads).await;

    async_server.assert_async().await;
    assert_eq!(
      app_arc.lock().await.data.radarr_data.downloads.items,
      downloads_response().records
    );
  }

  #[tokio::test]
  async fn test_handle_get_quality_profiles_event() {
    let quality_profile_json = json!([{
      "id": 2222,
      "name": "HD - 1080p"
    }]);
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(quality_profile_json),
      RadarrEvent::GetQualityProfiles.resource(),
    )
    .await;
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network
      .handle_radarr_event(RadarrEvent::GetQualityProfiles)
      .await;

    async_server.assert_async().await;
    assert_eq!(
      app_arc.lock().await.data.radarr_data.quality_profile_map,
      BiMap::from_iter([(2222u64, "HD - 1080p".to_owned())])
    );
  }

  #[tokio::test]
  async fn test_handle_get_tags_event() {
    let tags_json = json!([{
      "id": 2222,
      "label": "usenet"
    }]);
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(tags_json),
      RadarrEvent::GetTags.resource(),
    )
    .await;
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network.handle_radarr_event(RadarrEvent::GetTags).await;

    async_server.assert_async().await;
    assert_eq!(
      app_arc.lock().await.data.radarr_data.tags_map,
      BiMap::from_iter([(2222u64, "usenet".to_owned())])
    );
  }

  #[tokio::test]
  async fn test_add_tag() {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Post,
      Some(json!({ "label": "testing" })),
      Some(json!({ "id": 3, "label": "testing" })),
      RadarrEvent::GetTags.resource(),
    )
    .await;
    app_arc.lock().await.data.radarr_data.tags_map =
      BiMap::from_iter([(1, "usenet".to_owned()), (2, "test".to_owned())]);
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network.add_tag("testing".to_owned()).await;

    async_server.assert_async().await;
    assert_eq!(
      app_arc.lock().await.data.radarr_data.tags_map,
      BiMap::from_iter([
        (1, "usenet".to_owned()),
        (2, "test".to_owned()),
        (3, "testing".to_owned())
      ])
    );
  }

  #[tokio::test]
  async fn test_handle_get_root_folders_event() {
    let root_folder_json = json!([{
      "path": "/nfs",
      "accessible": true,
      "freeSpace": 219902325555200u64,
    }]);
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(root_folder_json),
      RadarrEvent::GetRootFolders.resource(),
    )
    .await;
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network
      .handle_radarr_event(RadarrEvent::GetRootFolders)
      .await;

    async_server.assert_async().await;
    assert_eq!(
      app_arc.lock().await.data.radarr_data.root_folders,
      vec![root_folder()]
    );
  }

  #[tokio::test]
  async fn test_handle_get_movie_credits_event() {
    let credits_json = json!([
        {
          "personName": "Madison Clarke",
          "character": "Johnny Blaze",
          "type": "cast",
        },
        {
          "personName": "Alex Clarke",
          "department": "Music",
          "job": "Composition",
          "type": "crew",
        }
    ]);
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(credits_json),
      format!("{}?movieId=1", RadarrEvent::GetMovieCredits.resource()).as_str(),
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .movies
      .set_items(vec![movie()]);
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network
      .handle_radarr_event(RadarrEvent::GetMovieCredits)
      .await;

    async_server.assert_async().await;
    assert_eq!(
      app_arc.lock().await.data.radarr_data.movie_cast.items,
      vec![cast_credit()]
    );
    assert_eq!(
      app_arc.lock().await.data.radarr_data.movie_crew.items,
      vec![crew_credit()]
    );
  }

  #[tokio::test]
  async fn test_handle_delete_movie_event() {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Delete,
      None,
      None,
      format!("{}/1", RadarrEvent::DeleteMovie.resource()).as_str(),
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .movies
      .set_items(vec![movie()]);
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network.handle_radarr_event(RadarrEvent::DeleteMovie).await;

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_delete_download_event() {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Delete,
      None,
      None,
      format!("{}/1", RadarrEvent::DeleteDownload.resource()).as_str(),
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .downloads
      .set_items(vec![download_record()]);
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network
      .handle_radarr_event(RadarrEvent::DeleteDownload)
      .await;

    async_server.assert_async().await;
  }

  #[rstest]
  #[tokio::test]
  async fn test_handle_add_movie_event(#[values(true, false)] collection_details_context: bool) {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Post,
      Some(json!({
        "tmdbId": 1234,
        "title": "Test",
        "rootFolderPath": "/nfs",
        "minimumAvailability": "announced",
        "monitored": true,
        "qualityProfileId": 2222,
        "tags": [1, 2],
        "addOptions": {
          "monitor": "movieOnly",
          "searchForMovie": true
        }
      })),
      None,
      RadarrEvent::AddMovie.resource(),
    )
    .await;

    {
      let mut app = app_arc.lock().await;
      app.data.radarr_data.root_folders = vec![
        RootFolder {
          path: "/nfs".to_owned(),
          accessible: true,
          free_space: Number::from(219902325555200u64),
        },
        RootFolder {
          path: "/nfs2".to_owned(),
          accessible: true,
          free_space: Number::from(21990232555520u64),
        },
      ];
      app.data.radarr_data.quality_profile_map =
        BiMap::from_iter([(2222, "HD - 1080p".to_owned())]);
      app.data.radarr_data.tags_map =
        BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
      app.data.radarr_data.edit_tags = "usenet, testing".to_owned().into();
      app
        .data
        .radarr_data
        .movie_quality_profile_list
        .set_items(vec!["HD - 1080p".to_owned()]);
      app
        .data
        .radarr_data
        .movie_monitor_list
        .set_items(Vec::from_iter(Monitor::iter()));
      app
        .data
        .radarr_data
        .movie_minimum_availability_list
        .set_items(Vec::from_iter(MinimumAvailability::iter()));
      if collection_details_context {
        app
          .data
          .radarr_data
          .collection_movies
          .set_items(vec![collection_movie()]);
        app.push_navigation_stack(ActiveRadarrBlock::CollectionDetails.into());
      } else {
        app
          .data
          .radarr_data
          .add_searched_movies
          .set_items(vec![add_movie_search_result()]);
      }
    }
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network.handle_radarr_event(RadarrEvent::AddMovie).await;

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_movie_event() {
    let mut expected_body: Value = serde_json::from_str(MOVIE_JSON).unwrap();
    *expected_body.get_mut("monitored").unwrap() = json!(false);
    *expected_body.get_mut("minimumAvailability").unwrap() = json!("announced");
    *expected_body.get_mut("qualityProfileId").unwrap() = json!(1111);
    *expected_body.get_mut("path").unwrap() = json!("/nfs/Test Path");
    *expected_body.get_mut("tags").unwrap() = json!([1, 2]);

    let (async_details_server, app_arc, mut server) = mock_radarr_api(
      RequestMethod::Get,
      None,
      Some(serde_json::from_str(MOVIE_JSON).unwrap()),
      format!("{}/1", RadarrEvent::GetMovieDetails.resource()).as_str(),
    )
    .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!("/api/v3{}/1", RadarrEvent::EditMovie.resource()).as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .create_async()
      .await;
    {
      let mut app = app_arc.lock().await;
      app.data.radarr_data.tags_map =
        BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
      app.data.radarr_data.edit_tags = "usenet, testing".to_owned().into();
      app.data.radarr_data.edit_path = "/nfs/Test Path".to_owned().into();
      app.data.radarr_data.edit_monitored = Some(false);
      app
        .data
        .radarr_data
        .movie_quality_profile_list
        .set_items(vec!["Any".to_owned(), "HD - 1080p".to_owned()]);
      app
        .data
        .radarr_data
        .movie_minimum_availability_list
        .set_items(Vec::from_iter(MinimumAvailability::iter()));
      app.data.radarr_data.movies.set_items(vec![Movie {
        monitored: false,
        ..movie()
      }]);
      app.data.radarr_data.quality_profile_map =
        BiMap::from_iter([(1111, "Any".to_owned()), (2222, "HD - 1080p".to_owned())]);
    }
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network.handle_radarr_event(RadarrEvent::EditMovie).await;

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;

    {
      let app = app_arc.lock().await;
      assert!(app.data.radarr_data.edit_path.text.is_empty());
      assert!(app.data.radarr_data.movie_details.items.is_empty());
    }
  }

  #[tokio::test]
  async fn test_handle_download_release_event() {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Post,
      Some(json!({
        "guid": "1234",
        "indexerId": 2
      })),
      None,
      RadarrEvent::DownloadRelease.resource(),
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .movie_releases
      .set_items(vec![release()]);
    let network = Network::new(reqwest::Client::new(), &app_arc);

    network
      .handle_radarr_event(RadarrEvent::DownloadRelease)
      .await;

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_extract_quality_profile_id() {
    let app_arc = Arc::new(Mutex::new(App::default()));
    {
      let mut app = app_arc.lock().await;
      app
        .data
        .radarr_data
        .movie_quality_profile_list
        .set_items(vec!["Any".to_owned(), "HD - 1080p".to_owned()]);
      app.data.radarr_data.quality_profile_map =
        BiMap::from_iter([(1, "Any".to_owned()), (2, "HD - 1080p".to_owned())]);
    }
    let network = Network::new(reqwest::Client::new(), &app_arc);

    assert_eq!(network.extract_quality_profile_id().await, 1);
  }

  #[tokio::test]
  async fn test_extract_and_add_tag_ids_vec() {
    let app_arc = Arc::new(Mutex::new(App::default()));
    {
      let mut app = app_arc.lock().await;
      app.data.radarr_data.edit_tags = "    test,hi ,, usenet ".to_owned().into();
      app.data.radarr_data.tags_map = BiMap::from_iter([
        (1, "usenet".to_owned()),
        (2, "test".to_owned()),
        (3, "hi".to_owned()),
      ]);
    }
    let network = Network::new(reqwest::Client::new(), &app_arc);

    assert_eq!(network.extract_and_add_tag_ids_vec().await, vec![2, 3, 1]);
  }

  #[tokio::test]
  async fn test_extract_and_add_tag_ids_vec_add_missing_tags_first() {
    let (async_server, app_arc, _server) = mock_radarr_api(
      RequestMethod::Post,
      Some(json!({ "label": "testing" })),
      Some(json!({ "id": 3, "label": "testing" })),
      RadarrEvent::GetTags.resource(),
    )
    .await;
    {
      let mut app = app_arc.lock().await;
      app.data.radarr_data.edit_tags = "usenet, test, testing".to_owned().into();
      app.data.radarr_data.tags_map =
        BiMap::from_iter([(1, "usenet".to_owned()), (2, "test".to_owned())]);
    }
    let network = Network::new(reqwest::Client::new(), &app_arc);

    let tag_ids_vec = network.extract_and_add_tag_ids_vec().await;

    async_server.assert_async().await;
    assert_eq!(tag_ids_vec, vec![1, 2, 3]);
    assert_eq!(
      app_arc.lock().await.data.radarr_data.tags_map,
      BiMap::from_iter([
        (1, "usenet".to_owned()),
        (2, "test".to_owned()),
        (3, "testing".to_owned())
      ])
    );
  }

  #[tokio::test]
  async fn test_extract_movie_id() {
    let app_arc = Arc::new(Mutex::new(App::default()));
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .movies
      .set_items(vec![Movie {
        id: Number::from(1),
        ..Movie::default()
      }]);
    let network = Network::new(reqwest::Client::new(), &app_arc);

    assert_eq!(network.extract_movie_id().await, 1);
  }

  #[tokio::test]
  async fn test_extract_movie_id_filtered_movies() {
    let app_arc = Arc::new(Mutex::new(App::default()));
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .filtered_movies
      .set_items(vec![Movie {
        id: Number::from(1),
        ..Movie::default()
      }]);
    let network = Network::new(reqwest::Client::new(), &app_arc);

    assert_eq!(network.extract_movie_id().await, 1);
  }

  #[tokio::test]
  async fn test_append_movie_id_param() {
    let app_arc = Arc::new(Mutex::new(App::default()));
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .movies
      .set_items(vec![Movie {
        id: Number::from(1),
        ..Movie::default()
      }]);
    let network = Network::new(reqwest::Client::new(), &app_arc);

    assert_str_eq!(
      network.append_movie_id_param("/test").await,
      "/test?movieId=1"
    );
  }

  #[tokio::test]
  async fn test_radarr_request_props_from_default_radarr_config() {
    let app_arc = Arc::new(Mutex::new(App::default()));
    let network = Network::new(reqwest::Client::new(), &app_arc);

    let request_props = network
      .radarr_request_props_from("/test", RequestMethod::Get, None::<()>)
      .await;

    assert_str_eq!(request_props.uri, "http://localhost:7878/api/v3/test");
    assert_eq!(request_props.method, RequestMethod::Get);
    assert_eq!(request_props.body, None);
    assert!(request_props.api_token.is_empty());

    app_arc.lock().await.config.radarr = RadarrConfig {
      host: "192.168.0.123".to_owned(),
      port: Some(8080),
      api_token: "testToken1234".to_owned(),
    };
  }

  #[tokio::test]
  async fn test_radarr_request_props_from_custom_radarr_config() {
    let api_token = "testToken1234".to_owned();
    let app_arc = Arc::new(Mutex::new(App::default()));
    app_arc.lock().await.config.radarr = RadarrConfig {
      host: "192.168.0.123".to_owned(),
      port: Some(8080),
      api_token: api_token.clone(),
    };
    let network = Network::new(reqwest::Client::new(), &app_arc);

    let request_props = network
      .radarr_request_props_from("/test", RequestMethod::Get, None::<()>)
      .await;

    assert_str_eq!(request_props.uri, "http://192.168.0.123:8080/api/v3/test");
    assert_eq!(request_props.method, RequestMethod::Get);
    assert_eq!(request_props.body, None);
    assert_str_eq!(request_props.api_token, api_token);
  }

  #[test]
  fn test_get_movie_status_downloaded() {
    assert_str_eq!(get_movie_status(true, &[], Number::from(0)), "Downloaded");
  }

  #[test]
  fn test_get_movie_status_missing() {
    let download_record = DownloadRecord {
      movie_id: 1.into(),
      ..DownloadRecord::default()
    };

    assert_str_eq!(
      get_movie_status(false, &[download_record.clone()], 0.into()),
      "Missing"
    );

    assert_str_eq!(
      get_movie_status(false, &[download_record], 1.into()),
      "Missing"
    );
  }

  #[test]
  fn test_get_movie_status_downloading() {
    assert_str_eq!(
      get_movie_status(
        false,
        &[DownloadRecord {
          movie_id: 1.into(),
          status: "downloading".to_owned(),
          ..DownloadRecord::default()
        }],
        1.into()
      ),
      "Downloading"
    );
  }

  #[test]
  fn test_get_movie_status_awaiting_import() {
    assert_str_eq!(
      get_movie_status(
        false,
        &[DownloadRecord {
          movie_id: 1.into(),
          status: "completed".to_owned(),
          ..DownloadRecord::default()
        }],
        1.into()
      ),
      "Awaiting Import"
    );
  }

  async fn mock_radarr_api(
    method: RequestMethod,
    request_body: Option<Value>,
    response_body: Option<Value>,
    resource: &str,
  ) -> (Mock, Arc<Mutex<App>>, ServerGuard) {
    let mut server = Server::new_async().await;
    let mut async_server = server
      .mock(
        &method.to_string().to_uppercase(),
        format!("/api/v3{}", resource).as_str(),
      )
      .match_header("X-Api-Key", "test1234");

    if let Some(body) = request_body {
      async_server = async_server.match_body(Matcher::Json(body));
    }

    if let Some(body) = response_body {
      async_server = async_server.with_body(body.to_string());
    }

    async_server = async_server.create_async().await;

    let host = server.host_with_port().split(':').collect::<Vec<&str>>()[0].to_owned();
    let port = Some(
      server.host_with_port().split(':').collect::<Vec<&str>>()[1]
        .parse()
        .unwrap(),
    );
    let mut app = App::default();
    let radarr_config = RadarrConfig {
      host,
      port,
      api_token: "test1234".to_owned(),
    };
    app.config.radarr = radarr_config;
    let app_arc = Arc::new(Mutex::new(app));

    (async_server, app_arc, server)
  }

  fn language() -> Language {
    Language {
      name: "English".to_owned(),
    }
  }

  fn genres() -> Vec<String> {
    vec!["cool".to_owned(), "family".to_owned(), "fun".to_owned()]
  }

  fn rating() -> Rating {
    Rating {
      value: Number::from_f64(9.9).unwrap(),
    }
  }

  fn ratings_list() -> RatingsList {
    RatingsList {
      imdb: Some(rating()),
      tmdb: Some(rating()),
      rotten_tomatoes: Some(rating()),
    }
  }

  fn media_info() -> MediaInfo {
    MediaInfo {
      audio_bitrate: Number::from(0),
      audio_channels: Number::from_f64(7.1).unwrap(),
      audio_codec: Some("AAC".to_owned()),
      audio_languages: Some("eng".to_owned()),
      audio_stream_count: Number::from(1),
      video_bit_depth: Number::from(10),
      video_bitrate: Number::from(0),
      video_codec: "x265".to_owned(),
      video_fps: Number::from_f64(23.976).unwrap(),
      resolution: "1920x804".to_owned(),
      run_time: "2:00:00".to_owned(),
      scan_type: "Progressive".to_owned(),
    }
  }

  fn movie_file() -> MovieFile {
    MovieFile {
      relative_path: "Test.mkv".to_owned(),
      path: "/nfs/movies/Test.mkv".to_owned(),
      date_added: DateTime::from(DateTime::parse_from_rfc3339("2022-12-30T07:37:56Z").unwrap()),
      media_info: Some(media_info()),
    }
  }

  fn collection_movie() -> CollectionMovie {
    CollectionMovie {
      title: "Test".to_owned().into(),
      overview: "Collection blah blah blah".to_owned(),
      year: Number::from(2023),
      runtime: Number::from(120),
      tmdb_id: Number::from(1234),
      genres: genres(),
      ratings: ratings_list(),
    }
  }

  fn collection() -> Collection {
    Collection {
      title: "Test Collection".to_owned().into(),
      root_folder_path: None,
      search_on_add: true,
      overview: Some("Collection blah blah blah".to_owned()),
      quality_profile_id: Number::from(2222),
      movies: Some(vec![collection_movie()]),
    }
  }

  fn movie() -> Movie {
    Movie {
      id: Number::from(1),
      title: "Test".to_owned().into(),
      original_language: language(),
      size_on_disk: Number::from(3543348019u64),
      status: "Downloaded".to_owned(),
      overview: "Blah blah blah".to_owned(),
      path: "/nfs/movies".to_owned(),
      studio: "21st Century Alex".to_owned(),
      genres: genres(),
      year: Number::from(2023),
      monitored: true,
      has_file: true,
      runtime: Number::from(120),
      tmdb_id: Number::from(1234),
      quality_profile_id: Number::from(2222),
      minimum_availability: MinimumAvailability::Announced,
      certification: Some("R".to_owned()),
      tags: vec![Number::from(1)],
      ratings: ratings_list(),
      movie_file: Some(movie_file()),
      collection: Some(collection()),
    }
  }

  fn rejections() -> Vec<String> {
    vec![
      "Unknown quality profile".to_owned(),
      "Release is already mapped".to_owned(),
    ]
  }

  fn quality() -> Quality {
    Quality {
      name: "HD - 1080p".to_owned(),
    }
  }

  fn quality_wrapper() -> QualityWrapper {
    QualityWrapper { quality: quality() }
  }

  fn release() -> Release {
    Release {
      guid: "1234".to_owned(),
      protocol: "torrent".to_owned(),
      age: Number::from(1),
      title: HorizontallyScrollableText::from("Test Release".to_owned()),
      indexer: "kickass torrents".to_owned(),
      indexer_id: Number::from(2),
      size: Number::from(1234),
      rejected: true,
      rejections: Some(rejections()),
      seeders: Some(Number::from(2)),
      leechers: Some(Number::from(1)),
      languages: Some(vec![language()]),
      quality: quality_wrapper(),
    }
  }

  fn add_movie_search_result() -> AddMovieSearchResult {
    AddMovieSearchResult {
      tmdb_id: Number::from(1234),
      title: HorizontallyScrollableText::from("Test".to_owned()),
      original_language: language(),
      status: "released".to_owned(),
      overview: "New movie blah blah blah".to_owned(),
      genres: genres(),
      year: Number::from(2023),
      runtime: Number::from(120),
      ratings: ratings_list(),
    }
  }

  fn movie_history_item() -> MovieHistoryItem {
    MovieHistoryItem {
      source_title: HorizontallyScrollableText::from("Test".to_owned()),
      quality: quality_wrapper(),
      languages: vec![language()],
      date: DateTime::from(DateTime::parse_from_rfc3339("2022-12-30T07:37:56Z").unwrap()),
      event_type: "grabbed".to_owned(),
    }
  }

  fn download_record() -> DownloadRecord {
    DownloadRecord {
      title: "Test Download Title".to_owned(),
      status: "downloading".to_owned(),
      id: Number::from(1),
      movie_id: Number::from(1),
      size: Number::from(3543348019u64),
      sizeleft: Number::from(1771674009u64),
      output_path: Some(HorizontallyScrollableText::from(
        "/nfs/movies/Test".to_owned(),
      )),
      indexer: "kickass torrents".to_owned(),
      download_client: "transmission".to_owned(),
    }
  }

  fn downloads_response() -> DownloadsResponse {
    DownloadsResponse {
      records: vec![download_record()],
    }
  }

  fn root_folder() -> RootFolder {
    RootFolder {
      path: "/nfs".to_owned(),
      accessible: true,
      free_space: Number::from(219902325555200u64),
    }
  }

  fn cast_credit() -> Credit {
    Credit {
      person_name: "Madison Clarke".to_owned(),
      character: Some("Johnny Blaze".to_owned()),
      department: None,
      job: None,
      credit_type: CreditType::Cast,
    }
  }

  fn crew_credit() -> Credit {
    Credit {
      person_name: "Alex Clarke".to_owned(),
      character: None,
      department: Some("Music".to_owned()),
      job: Some("Composition".to_owned()),
      credit_type: CreditType::Crew,
    }
  }
}
