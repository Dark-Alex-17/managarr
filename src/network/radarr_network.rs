use std::fmt::Debug;

use indoc::formatdoc;
use log::{debug, info};
use serde::Serialize;
use urlencoding::encode;

use crate::app::RadarrConfig;
use crate::models::radarr_models::{
  AddMovieBody, AddMovieSearchResult, AddOptions, Collection, CommandBody, Credit, CreditType,
  DiskSpace, DownloadsResponse, Movie, MovieHistoryItem, QualityProfile, Release, RootFolder,
  SystemStatus,
};
use crate::models::ScrollableText;
use crate::network::utils::get_movie_status;
use crate::network::{Network, NetworkEvent, RequestMethod, RequestProps};
use crate::utils::{convert_runtime, convert_to_gb};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum RadarrEvent {
  AddMovie,
  DeleteDownload,
  DeleteMovie,
  GetCollections,
  GetDownloads,
  GetMovies,
  GetMovieCredits,
  GetMovieDetails,
  GetMovieHistory,
  GetOverview,
  GetQualityProfiles,
  GetReleases,
  GetRootFolders,
  GetStatus,
  SearchNewMovie,
  TriggerAutomaticSearch,
  RefreshAndScan,
  HealthCheck,
}

impl RadarrEvent {
  const fn resource(self) -> &'static str {
    match self {
      RadarrEvent::GetCollections => "/collection",
      RadarrEvent::GetDownloads | RadarrEvent::DeleteDownload => "/queue",
      RadarrEvent::AddMovie
      | RadarrEvent::GetMovies
      | RadarrEvent::GetMovieDetails
      | RadarrEvent::DeleteMovie => "/movie",
      RadarrEvent::SearchNewMovie => "/movie/lookup",
      RadarrEvent::GetMovieCredits => "/credit",
      RadarrEvent::GetMovieHistory => "/history/movie",
      RadarrEvent::GetOverview => "/diskspace",
      RadarrEvent::GetQualityProfiles => "/qualityprofile",
      RadarrEvent::GetReleases => "/release",
      RadarrEvent::GetRootFolders => "/rootfolder",
      RadarrEvent::GetStatus => "/system/status",
      RadarrEvent::TriggerAutomaticSearch | RadarrEvent::RefreshAndScan => "/command",
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
      RadarrEvent::GetCollections => self.get_collections().await,
      RadarrEvent::HealthCheck => self.get_healthcheck().await,
      RadarrEvent::GetOverview => self.get_diskspace().await,
      RadarrEvent::GetStatus => self.get_status().await,
      RadarrEvent::GetMovies => self.get_movies().await,
      RadarrEvent::DeleteMovie => self.delete_movie().await,
      RadarrEvent::DeleteDownload => self.delete_download().await,
      RadarrEvent::GetMovieCredits => self.get_credits().await,
      RadarrEvent::GetMovieDetails => self.get_movie_details().await,
      RadarrEvent::GetMovieHistory => self.get_movie_history().await,
      RadarrEvent::GetDownloads => self.get_downloads().await,
      RadarrEvent::GetQualityProfiles => self.get_quality_profiles().await,
      RadarrEvent::GetReleases => self.get_releases().await,
      RadarrEvent::GetRootFolders => self.get_root_folders().await,
      RadarrEvent::SearchNewMovie => self.search_movie().await,
      RadarrEvent::AddMovie => self.add_movie().await,
      RadarrEvent::TriggerAutomaticSearch => self.trigger_automatic_search().await,
      RadarrEvent::RefreshAndScan => self.refresh_and_scan().await,
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

    let search_string = self.app.lock().await.data.radarr_data.search.clone();
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
        app
          .data
          .radarr_data
          .add_searched_movies
          .set_items(movie_vec)
      })
      .await;
  }

  async fn trigger_automatic_search(&self) {
    let movie_id = self.extract_movie_id().await;
    info!("Searching indexers for movie with id: {}", movie_id);
    let body = CommandBody {
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
      .handle_request::<CommandBody, ()>(request_props, |_, _| ())
      .await;
  }

  async fn refresh_and_scan(&self) {
    let movie_id = self.extract_movie_id().await;
    info!("Refreshing and scanning movie with id: {}", movie_id);
    let body = CommandBody {
      name: "RefreshMovie".to_owned(),
      movie_ids: vec![movie_id],
    };

    let request_props = self
      .radarr_request_props_from(
        RadarrEvent::RefreshAndScan.resource(),
        RequestMethod::Post,
        Some(body),
      )
      .await;

    self
      .handle_request::<CommandBody, ()>(request_props, |_, _| ())
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
          .get(&quality_profile_id.as_u64().unwrap())
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

          let media_info = file.media_info;

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
          .iter()
          .map(|profile| (profile.id.as_u64().unwrap(), profile.name.clone()))
          .collect();
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
    let movie_id = self
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

    info!("Deleting Radarr download for movie with id: {}", movie_id);

    let request_props = self
      .radarr_request_props_from(
        format!("{}/{}", RadarrEvent::DeleteDownload.resource(), movie_id).as_str(),
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
      let app = self.app.lock().await;
      let root_folders = app.data.radarr_data.root_folders.to_vec();
      let current_selection = app
        .data
        .radarr_data
        .add_searched_movies
        .current_selection_clone();
      let quality_profile_map = app.data.radarr_data.quality_profile_map.clone();

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
        .add_movie_monitor_list
        .current_selection()
        .to_string();
      let minimum_availability = app
        .data
        .radarr_data
        .add_movie_minimum_availability_list
        .current_selection()
        .to_string();
      let quality_profile = app
        .data
        .radarr_data
        .add_movie_quality_profile_list
        .current_selection_clone();
      let AddMovieSearchResult { tmdb_id, title, .. } = current_selection;
      let quality_profile_id = quality_profile_map
        .iter()
        .filter(|(_, value)| **value == quality_profile)
        .map(|(key, _)| key)
        .next()
        .unwrap();

      AddMovieBody {
        tmdb_id: tmdb_id.as_u64().unwrap(),
        title: title.to_string(),
        root_folder_path: path.to_owned(),
        minimum_availability,
        monitored: true,
        quality_profile_id: *quality_profile_id,
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
      .handle_request::<AddMovieBody, ()>(request_props, |_, _| ())
      .await;
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
        .clone()
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
        .clone()
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
