use chrono::{DateTime, Utc};
use derivative::Derivative;
use log::{debug, error};
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Number;
use tokio::sync::MutexGuard;

use crate::app::{App, RadarrConfig};
use crate::network::{Network, NetworkEvent, utils};

#[derive(Debug, Eq, PartialEq)]
pub enum RadarrEvent {
  HealthCheck,
  GetDownloads,
  GetOverview,
  GetStatus,
  GetMovies,
  GetQualityProfiles,
}

impl RadarrEvent {
  const fn resource(self) -> &'static str {
    match self {
      RadarrEvent::HealthCheck => "/health",
      RadarrEvent::GetOverview => "/diskspace",
      RadarrEvent::GetStatus => "/system/status",
      RadarrEvent::GetMovies => "/movie",
      RadarrEvent::GetDownloads => "/queue",
      RadarrEvent::GetQualityProfiles => "/qualityprofile"
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
struct DiskSpace {
  pub path: String,
  pub label: String,
  pub free_space: Number,
  pub total_space: Number,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SystemStatus {
  version: String,
  start_time: DateTime<Utc>,
}

#[derive(Derivative, Deserialize, Debug)]
#[derivative(Default)]
#[serde(rename_all = "camelCase")]
pub struct Movie {
  #[derivative(Default(value = "Number::from(0)"))]
  pub id: Number,
  pub title: String,
  #[derivative(Default(value = "Number::from(0)"))]
  pub size_on_disk: Number,
  pub status: String,
  #[derivative(Default(value = "Number::from(0)"))]
  pub year: Number,
  pub monitored: bool,
  pub has_file: bool,
  #[derivative(Default(value = "Number::from(0)"))]
  pub runtime: Number,
  #[derivative(Default(value = "Number::from(0)"))]
  pub quality_profile_id: Number,
}

#[derive(Derivative, Deserialize, Debug)]
#[derivative(Default)]
#[serde(rename_all = "camelCase")]
pub struct DownloadsResponse {
  pub records: Vec<DownloadRecord>
}

#[derive(Derivative, Deserialize, Debug)]
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
  pub output_path: String,
  pub indexer: String,
  pub download_client: String
}

#[derive(Derivative, Deserialize, Debug)]
#[derivative(Default)]
#[serde(rename_all = "camelCase")]
struct QualityProfile {
  #[derivative(Default(value = "Number::from(0)"))]
  pub id: Number,
  pub name: String
}

impl<'a> Network<'a> {
  pub async fn handle_radarr_event(&self, radarr_event: RadarrEvent) {
    match radarr_event {
      RadarrEvent::HealthCheck => self.get_healthcheck(RadarrEvent::HealthCheck.resource()).await,
      RadarrEvent::GetOverview => self.get_diskspace(RadarrEvent::GetOverview.resource()).await,
      RadarrEvent::GetStatus => self.get_status(RadarrEvent::GetStatus.resource()).await,
      RadarrEvent::GetMovies => self.get_movies(RadarrEvent::GetMovies.resource()).await,
      RadarrEvent::GetDownloads => self.get_downloads(RadarrEvent::GetDownloads.resource()).await,
      RadarrEvent::GetQualityProfiles => self.get_quality_profiles(RadarrEvent::GetQualityProfiles.resource()).await
    }

    let mut app = self.app.lock().await;
    app.reset_tick_count();
  }

  async fn get_healthcheck(&self, resource: &str) {
    if let Err(e) = self.call_radarr_api(resource).await.send().await {
      error!("Healthcheck failed. {:?}", e)
    }
  }

  async fn get_diskspace(&self, resource: &str) {
    self.handle_get_request::<Vec<DiskSpace>>(resource, | disk_space_vec, mut app | {
      let DiskSpace {
        free_space,
        total_space,
        ..
      } = &disk_space_vec[0];

      app.data.radarr_data.free_space = free_space.as_u64().unwrap();
      app.data.radarr_data.total_space = total_space.as_u64().unwrap();
    }).await;
  }

  async fn get_status(&self, resource: &str) {
    self.handle_get_request::<SystemStatus>(resource, | system_status, mut app | {
      app.data.radarr_data.version = system_status.version;
      app.data.radarr_data.start_time = system_status.start_time;
    }).await;
  }

  async fn get_movies(&self, resource: &str) {
    self.handle_get_request::<Vec<Movie>>(resource, |movie_vec, mut app| {
      app.data.radarr_data.movies.set_items(movie_vec);
    }).await;
  }

  async fn get_downloads(&self, resource: &str) {
    self.handle_get_request::<DownloadsResponse>(resource, |queue_response, mut app | {
      app.data.radarr_data.downloads.set_items(queue_response.records);
    }).await
  }

  async fn get_quality_profiles(&self, resource: &str) {
    self.handle_get_request::<Vec<QualityProfile>>(resource, | quality_profiles, mut app | {
      app.data.radarr_data.quality_profile_map = quality_profiles.into_iter()
          .map(| profile | (profile.id.as_u64().unwrap(), profile.name))
          .collect();
    }).await;
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

  async fn handle_get_request<T>(&self, resource: &str, mut app_update_fn: impl FnMut(T, MutexGuard<App>))
    where
        T: DeserializeOwned {
    match self.call_radarr_api(resource)
        .await
        .send()
        .await {
      Ok(response) => {
        match utils::parse_response::<T>(response).await {
          Ok(value) => {
            let app = self.app.lock().await;
            app_update_fn(value, app);
          }
          Err(e) => error!("Failed to parse response! {:?}", e)
        }
      }
      Err(e) => error!("Failed to fetch resource. {:?}", e)
    }
  }
}
