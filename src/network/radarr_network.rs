use anyhow::anyhow;
use chrono::{DateTime, Utc};
use derivative::Derivative;
use indoc::formatdoc;
use log::{debug, error};
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Number;
use tokio::sync::MutexGuard;

use crate::app::models::{HorizontallyScrollableText, ScrollableText};
use crate::app::{App, RadarrConfig};
use crate::network::utils::get_movie_status;
use crate::network::{utils, Network, NetworkEvent};
use crate::utils::{convert_runtime, convert_to_gb};

#[derive(Debug, Eq, PartialEq)]
pub enum RadarrEvent {
  GetDownloads,
  GetMovies,
  GetMovieCredits,
  GetMovieDetails,
  GetMovieHistory,
  GetOverview,
  GetQualityProfiles,
  GetStatus,
  HealthCheck,
}

impl RadarrEvent {
  const fn resource(self) -> &'static str {
    match self {
      RadarrEvent::GetDownloads => "/queue",
      RadarrEvent::GetMovies | RadarrEvent::GetMovieDetails => "/movie",
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DiskSpace {
  pub free_space: Number,
  pub total_space: Number,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SystemStatus {
  version: String,
  start_time: DateTime<Utc>,
}

#[derive(Derivative, Deserialize, Debug, Clone)]
#[derivative(Default)]
#[serde(rename_all = "camelCase")]
pub struct Movie {
  #[derivative(Default(value = "Number::from(0)"))]
  pub id: Number,
  pub title: String,
  pub original_language: Language,
  #[derivative(Default(value = "Number::from(0)"))]
  pub size_on_disk: Number,
  pub status: String,
  pub overview: String,
  pub path: String,
  pub studio: String,
  pub genres: Vec<String>,
  #[derivative(Default(value = "Number::from(0)"))]
  pub year: Number,
  pub monitored: bool,
  pub has_file: bool,
  #[derivative(Default(value = "Number::from(0)"))]
  pub runtime: Number,
  #[derivative(Default(value = "Number::from(0)"))]
  pub quality_profile_id: Number,
  pub certification: Option<String>,
  pub ratings: RatingsList,
}

#[derive(Default, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RatingsList {
  pub imdb: Option<Rating>,
  pub tmdb: Option<Rating>,
  pub rotten_tomatoes: Option<Rating>,
}

#[derive(Derivative, Deserialize, Debug, Clone)]
#[derivative(Default)]
pub struct Rating {
  #[derivative(Default(value = "Number::from(0)"))]
  pub value: Number,
}

#[derive(Derivative, Deserialize, Debug)]
#[derivative(Default)]
#[serde(rename_all = "camelCase")]
pub struct DownloadsResponse {
  pub records: Vec<DownloadRecord>,
}

#[derive(Derivative, Deserialize, Debug, Clone, PartialEq, Eq)]
#[derivative(Default)]
#[serde(rename_all = "camelCase")]
pub struct DownloadRecord {
  pub title: String,
  pub status: String,
  #[derivative(Default(value = "Number::from(0)"))]
  pub movie_id: Number,
  #[derivative(Default(value = "Number::from(0)"))]
  pub size: Number,
  #[derivative(Default(value = "Number::from(0)"))]
  pub sizeleft: Number,
  pub output_path: HorizontallyScrollableText,
  pub indexer: String,
  pub download_client: String,
}

#[derive(Derivative, Deserialize, Debug)]
#[derivative(Default)]
#[serde(rename_all = "camelCase")]
struct QualityProfile {
  #[derivative(Default(value = "Number::from(0)"))]
  pub id: Number,
  pub name: String,
}

#[derive(Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MovieHistoryItem {
  pub source_title: HorizontallyScrollableText,
  pub quality: QualityHistory,
  pub languages: Vec<Language>,
  pub date: DateTime<Utc>,
  pub event_type: String,
}

#[derive(Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct Language {
  pub name: String,
}

#[derive(Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct QualityHistory {
  pub quality: Quality,
}

#[derive(Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct Quality {
  pub name: String,
}

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CreditType {
  Cast,
  Crew,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Credit {
  pub person_name: String,
  pub character: Option<String>,
  pub department: Option<String>,
  pub job: Option<String>,
  #[serde(rename(deserialize = "type"))]
  pub credit_type: CreditType,
}

impl<'a> Network<'a> {
  pub async fn handle_radarr_event(&self, radarr_event: RadarrEvent) {
    match radarr_event {
      RadarrEvent::HealthCheck => {
        self
          .get_healthcheck(RadarrEvent::HealthCheck.resource())
          .await
      }
      RadarrEvent::GetOverview => {
        self
          .get_diskspace(RadarrEvent::GetOverview.resource())
          .await
      }
      RadarrEvent::GetStatus => self.get_status(RadarrEvent::GetStatus.resource()).await,
      RadarrEvent::GetMovies => self.get_movies(RadarrEvent::GetMovies.resource()).await,
      RadarrEvent::GetMovieCredits => {
        self
          .get_credits(RadarrEvent::GetMovieCredits.resource())
          .await
      }
      RadarrEvent::GetMovieDetails => {
        self
          .get_movie_details(RadarrEvent::GetMovieDetails.resource())
          .await
      }
      RadarrEvent::GetMovieHistory => {
        self
          .get_movie_history(RadarrEvent::GetMovieHistory.resource())
          .await
      }
      RadarrEvent::GetDownloads => {
        self
          .get_downloads(RadarrEvent::GetDownloads.resource())
          .await
      }
      RadarrEvent::GetQualityProfiles => {
        self
          .get_quality_profiles(RadarrEvent::GetQualityProfiles.resource())
          .await
      }
    }
  }

  async fn get_healthcheck(&self, resource: &str) {
    if let Err(e) = self.call_radarr_api(resource).await.send().await {
      error!("Healthcheck failed. {:?}", e);
      self.app.lock().await.handle_error(anyhow!(e));
    }
  }

  async fn get_diskspace(&self, resource: &str) {
    self
      .handle_get_request::<Vec<DiskSpace>>(resource, |disk_space_vec, mut app| {
        app.data.radarr_data.disk_space_vec = disk_space_vec;
      })
      .await;
  }

  async fn get_status(&self, resource: &str) {
    self
      .handle_get_request::<SystemStatus>(resource, |system_status, mut app| {
        app.data.radarr_data.version = system_status.version;
        app.data.radarr_data.start_time = system_status.start_time;
      })
      .await;
  }

  async fn get_movies(&self, resource: &str) {
    self
      .handle_get_request::<Vec<Movie>>(resource, |movie_vec, mut app| {
        app.data.radarr_data.movies.set_items(movie_vec)
      })
      .await;
  }

  async fn get_movie_details(&self, resource: &str) {
    let movie_id = self.extract_movie_id().await;
    self
      .handle_get_request::<Movie>(
        format!("{}/{}", resource, movie_id).as_str(),
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
              format!("{}%", value * 10f64)
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

          app.data.radarr_data.movie_details = ScrollableText::with_string(formatdoc!(
            "Title: {}
          Year: {}
          Runtime: {}h {}m
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
          ))
        },
      )
      .await;
  }

  async fn get_movie_history(&self, resource: &str) {
    self
      .handle_get_request::<Vec<MovieHistoryItem>>(
        self.append_movie_id_param(resource).await.as_str(),
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

  async fn get_downloads(&self, resource: &str) {
    self
      .handle_get_request::<DownloadsResponse>(resource, |queue_response, mut app| {
        app
          .data
          .radarr_data
          .downloads
          .set_items(queue_response.records);
      })
      .await
  }

  async fn get_quality_profiles(&self, resource: &str) {
    self
      .handle_get_request::<Vec<QualityProfile>>(resource, |quality_profiles, mut app| {
        app.data.radarr_data.quality_profile_map = quality_profiles
          .iter()
          .map(|profile| (profile.id.as_u64().unwrap(), profile.name.clone()))
          .collect();
      })
      .await;
  }

  async fn get_credits(&self, resource: &str) {
    self
      .handle_get_request::<Vec<Credit>>(
        self.append_movie_id_param(resource).await.as_str(),
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

  async fn call_radarr_api(&self, resource: &str) -> RequestBuilder {
    debug!("Creating RequestBuilder for resource: {:?}", resource);
    let app = self.app.lock().await;
    let RadarrConfig {
      host,
      port,
      api_token,
    } = &app.config.radarr;

    app
      .client
      .get(format!(
        "http://{}:{}/api/v3{}",
        host,
        port.unwrap_or(7878),
        resource
      ))
      .header("X-Api-Key", api_token)
  }

  async fn handle_get_request<T>(
    &self,
    resource: &str,
    mut app_update_fn: impl FnMut(T, MutexGuard<App>),
  ) where
    T: DeserializeOwned,
  {
    match self.call_radarr_api(resource).await.send().await {
      Ok(response) => match utils::parse_response::<T>(response).await {
        Ok(value) => {
          let app = self.app.lock().await;
          app_update_fn(value, app);
        }
        Err(e) => {
          error!("Failed to parse response! {:?}", e);
          self.app.lock().await.handle_error(anyhow!(e));
        }
      },
      Err(e) => {
        error!("Failed to fetch resource. {:?}", e);
        self.app.lock().await.handle_error(anyhow!(e));
      }
    }
  }

  async fn extract_movie_id(&self) -> u64 {
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

  async fn append_movie_id_param(&self, resource: &str) -> String {
    let movie_id = self.extract_movie_id().await;
    format!("{}?movieId={}", resource.to_owned(), movie_id.to_owned())
  }
}
