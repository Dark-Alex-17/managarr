use anyhow::anyhow;
use indoc::formatdoc;
use log::{debug, error};
use reqwest::{RequestBuilder, StatusCode};
use serde::de::DeserializeOwned;
use tokio::sync::MutexGuard;
use urlencoding::encode;

use crate::app::{App, RadarrConfig};
use crate::models::radarr_models::{
  AddMovieSearchResult, Collection, Credit, CreditType, DiskSpace, DownloadsResponse, Movie,
  MovieHistoryItem, QualityProfile, SystemStatus,
};
use crate::models::ScrollableText;
use crate::network::utils::get_movie_status;
use crate::network::{utils, Network, NetworkEvent};
use crate::utils::{convert_runtime, convert_to_gb};

#[derive(Debug, Eq, PartialEq)]
pub enum RadarrEvent {
  DeleteMovie,
  GetCollections,
  GetDownloads,
  GetMovies,
  GetMovieCredits,
  GetMovieDetails,
  GetMovieHistory,
  GetOverview,
  GetQualityProfiles,
  GetStatus,
  SearchNewMovie,
  HealthCheck,
}

#[derive(Clone)]
enum RequestMethod {
  GET,
  DELETE,
}

struct RequestProps<T> {
  pub resource: String,
  pub method: RequestMethod,
  pub body: Option<T>,
}

impl RadarrEvent {
  const fn resource(self) -> &'static str {
    match self {
      RadarrEvent::GetCollections => "/collection",
      RadarrEvent::GetDownloads => "/queue",
      RadarrEvent::GetMovies | RadarrEvent::GetMovieDetails | RadarrEvent::DeleteMovie => "/movie",
      RadarrEvent::SearchNewMovie => "/movie/lookup",
      RadarrEvent::GetMovieCredits => "/credit",
      RadarrEvent::GetMovieHistory => "/history/movie",
      RadarrEvent::GetOverview => "/diskspace",
      RadarrEvent::GetQualityProfiles => "/qualityprofile",
      RadarrEvent::GetStatus => "/system/status",
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
      RadarrEvent::GetCollections => {
        self
          .get_collections(RadarrEvent::GetCollections.resource().to_owned())
          .await
      }
      RadarrEvent::HealthCheck => {
        self
          .get_healthcheck(RadarrEvent::HealthCheck.resource().to_owned())
          .await
      }
      RadarrEvent::GetOverview => {
        self
          .get_diskspace(RadarrEvent::GetOverview.resource().to_owned())
          .await
      }
      RadarrEvent::GetStatus => {
        self
          .get_status(RadarrEvent::GetStatus.resource().to_owned())
          .await
      }
      RadarrEvent::GetMovies => {
        self
          .get_movies(RadarrEvent::GetMovies.resource().to_owned())
          .await
      }
      RadarrEvent::DeleteMovie => {
        self
          .delete_movie(RadarrEvent::DeleteMovie.resource().to_owned())
          .await
      }
      RadarrEvent::GetMovieCredits => {
        self
          .get_credits(RadarrEvent::GetMovieCredits.resource().to_owned())
          .await
      }
      RadarrEvent::GetMovieDetails => {
        self
          .get_movie_details(RadarrEvent::GetMovieDetails.resource().to_owned())
          .await
      }
      RadarrEvent::GetMovieHistory => {
        self
          .get_movie_history(RadarrEvent::GetMovieHistory.resource().to_owned())
          .await
      }
      RadarrEvent::GetDownloads => {
        self
          .get_downloads(RadarrEvent::GetDownloads.resource().to_owned())
          .await
      }
      RadarrEvent::GetQualityProfiles => {
        self
          .get_quality_profiles(RadarrEvent::GetQualityProfiles.resource().to_owned())
          .await
      }
      RadarrEvent::SearchNewMovie => {
        self
          .search_movie(RadarrEvent::SearchNewMovie.resource().to_owned())
          .await
      }
    }
  }

  async fn get_healthcheck(&self, resource: String) {
    if let Err(e) = self
      .call_radarr_api::<()>(RequestProps {
        resource,
        method: RequestMethod::GET,
        body: None::<()>,
      })
      .await
      .send()
      .await
    {
      error!("Healthcheck failed. {:?}", e);
      self.app.lock().await.handle_error(anyhow!(e));
    }
  }

  async fn get_diskspace(&self, resource: String) {
    type ResponseType = Vec<DiskSpace>;
    self
      .handle_request::<ResponseType>(
        RequestProps {
          resource,
          method: RequestMethod::GET,
          body: None::<ResponseType>,
        },
        |disk_space_vec, mut app| {
          app.data.radarr_data.disk_space_vec = disk_space_vec;
        },
      )
      .await;
  }

  async fn get_status(&self, resource: String) {
    self
      .handle_request::<SystemStatus>(
        RequestProps {
          resource,
          method: RequestMethod::GET,
          body: None::<SystemStatus>,
        },
        |system_status, mut app| {
          app.data.radarr_data.version = system_status.version;
          app.data.radarr_data.start_time = system_status.start_time;
        },
      )
      .await;
  }

  async fn get_movies(&self, resource: String) {
    type ResponseType = Vec<Movie>;
    self
      .handle_request::<ResponseType>(
        RequestProps {
          resource,
          method: RequestMethod::GET,
          body: None::<ResponseType>,
        },
        |movie_vec, mut app| app.data.radarr_data.movies.set_items(movie_vec),
      )
      .await;
  }

  async fn search_movie(&self, resource: String) {
    type ResponseType = Vec<AddMovieSearchResult>;
    let search_string = self.app.lock().await.data.radarr_data.search.clone();
    debug!(
      "Searching for movie: {:?}",
      format!("{}?term={}", resource, encode(search_string.as_str()))
    );
    self
      .handle_request::<ResponseType>(
        RequestProps {
          resource: format!("{}?term={}", resource, encode(&search_string)),
          method: RequestMethod::GET,
          body: None::<ResponseType>,
        },
        |movie_vec, mut app| {
          app
            .data
            .radarr_data
            .add_searched_movies
            .set_items(movie_vec)
        },
      )
      .await;
  }

  async fn get_movie_details(&self, resource: String) {
    let movie_id = self.extract_movie_id().await;
    self
      .handle_request::<Movie>(
        RequestProps {
          resource: format!("{}/{}", resource, movie_id),
          method: RequestMethod::GET,
          body: None::<Movie>,
        },
        |movie_response, mut app| {
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
        },
      )
      .await;
  }

  async fn get_movie_history(&self, resource: String) {
    type ResponseType = Vec<MovieHistoryItem>;
    self
      .handle_request::<ResponseType>(
        RequestProps {
          resource: self.append_movie_id_param(&resource).await,
          method: RequestMethod::GET,
          body: None::<ResponseType>,
        },
        |movie_history_vec, mut app| {
          let mut reversed_movie_history_vec = movie_history_vec.to_vec();
          reversed_movie_history_vec.reverse();
          app
            .data
            .radarr_data
            .movie_history
            .set_items(reversed_movie_history_vec)
        },
      )
      .await;
  }

  async fn get_collections(&self, resource: String) {
    type ResponseType = Vec<Collection>;
    self
      .handle_request::<ResponseType>(
        RequestProps {
          resource,
          method: RequestMethod::GET,
          body: None::<ResponseType>,
        },
        |collections_vec, mut app| {
          app.data.radarr_data.collections.set_items(collections_vec);
        },
      )
      .await;
  }

  async fn get_downloads(&self, resource: String) {
    self
      .handle_request::<DownloadsResponse>(
        RequestProps {
          resource,
          method: RequestMethod::GET,
          body: None::<DownloadsResponse>,
        },
        |queue_response, mut app| {
          app
            .data
            .radarr_data
            .downloads
            .set_items(queue_response.records);
        },
      )
      .await
  }

  async fn get_quality_profiles(&self, resource: String) {
    type ResponseType = Vec<QualityProfile>;
    self
      .handle_request::<ResponseType>(
        RequestProps {
          resource,
          method: RequestMethod::GET,
          body: None::<ResponseType>,
        },
        |quality_profiles, mut app| {
          app.data.radarr_data.quality_profile_map = quality_profiles
            .iter()
            .map(|profile| (profile.id.as_u64().unwrap(), profile.name.clone()))
            .collect();
        },
      )
      .await;
  }

  async fn get_credits(&self, resource: String) {
    type ResponseType = Vec<Credit>;
    self
      .handle_request::<ResponseType>(
        RequestProps {
          resource: self.append_movie_id_param(&resource).await,
          method: RequestMethod::GET,
          body: None::<ResponseType>,
        },
        |credit_vec, mut app| {
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
        },
      )
      .await;
  }

  async fn delete_movie(&self, resource: String) {
    let movie_id = self.extract_movie_id().await;
    self
      .handle_request::<()>(
        RequestProps {
          resource: format!("{}/{}", resource, movie_id),
          method: RequestMethod::DELETE,
          body: None::<()>,
        },
        |_, _| (),
      )
      .await;
  }

  async fn call_radarr_api<T>(&self, request_props: RequestProps<T>) -> RequestBuilder {
    let RequestProps {
      resource,
      method,
      body,
    } = request_props;
    debug!("Creating RequestBuilder for resource: {:?}", resource);
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

    match method {
      RequestMethod::GET => app.client.get(uri).header("X-Api-Key", api_token),
      RequestMethod::DELETE => app.client.delete(uri).header("X-Api-Key", api_token),
    }
  }

  async fn handle_request<T>(
    &self,
    request_props: RequestProps<T>,
    mut app_update_fn: impl FnMut(T, MutexGuard<App>),
  ) where
    T: DeserializeOwned,
  {
    let method = request_props.method.clone();
    match self.call_radarr_api(request_props).await.send().await {
      Ok(response) => match method {
        RequestMethod::GET => match utils::parse_response::<T>(response).await {
          Ok(value) => {
            let app = self.app.lock().await;
            app_update_fn(value, app);
          }
          Err(e) => {
            error!("Failed to parse response! {:?}", e);
            self.app.lock().await.handle_error(anyhow!(e));
          }
        },
        RequestMethod::DELETE => {
          if response.status() != StatusCode::OK {
            error!(
              "Received the following code for delete operation: {:?}",
              response.status()
            );
            self.app.lock().await.handle_error(anyhow!(
              "Received a non 200 OK response for delete operation"
            ));
          }
        }
      },
      Err(e) => {
        error!("Failed to send request. {:?}", e);
        self.app.lock().await.handle_error(anyhow!(e));
      }
    }
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
    format!("{}?movieId={}", resource.to_owned(), movie_id.to_owned())
  }
}
