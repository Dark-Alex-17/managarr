use chrono::{DateTime, Utc};
use derivative::Derivative;
use log::{debug, error};
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Number;
use tokio::sync::MutexGuard;

use crate::app::{App, RadarrConfig};
use crate::network::{Network, RadarrEvent, utils};

impl RadarrEvent {
  const fn resource(self) -> &'static str {
    match self {
      RadarrEvent::HealthCheck => "/health",
      RadarrEvent::GetOverview => "/diskspace",
      RadarrEvent::GetStatus => "/system/status",
      RadarrEvent::GetMovies => "/movie",
    }
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
  pub year: Number,
  pub monitored: bool,
  pub has_file: bool,
}

impl<'a> Network<'a> {
  pub async fn handle_radarr_event(&self, radarr_event: RadarrEvent) {
    match radarr_event {
      RadarrEvent::HealthCheck => self.healthcheck(RadarrEvent::HealthCheck.resource()).await,
      RadarrEvent::GetOverview => self.diskspace(RadarrEvent::GetOverview.resource()).await,
      RadarrEvent::GetStatus => self.status(RadarrEvent::GetStatus.resource()).await,
      RadarrEvent::GetMovies => self.movies(RadarrEvent::GetMovies.resource()).await
    }

    let mut app = self.app.lock().await;
    app.reset_tick_count();
  }

  async fn healthcheck(&self, resource: &str) {
    if let Err(e) = self.call_radarr_api(resource).await.send().await {
      error!("Healthcheck failed. {:?}", e)
    }
  }

  async fn diskspace(&self, resource: &str) {
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

  async fn status(&self, resource: &str) {
    self.handle_get_request::<SystemStatus>(resource, | system_status, mut app | {
      app.data.radarr_data.version = system_status.version;
      app.data.radarr_data.start_time = system_status.start_time;
    }).await;
  }

  async fn movies(&self, resource: &str) {
    self.handle_get_request::<Vec<Movie>>(resource, |movie_vec, mut app| {
      app.data.radarr_data.movies.set_items(movie_vec);
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
          Err(e) => error!("Failed to parse movie response! {:?}", e)
        }
      }
      Err(e) => error!("Failed to fetch movies. {:?}", e)
    }
  }
}
