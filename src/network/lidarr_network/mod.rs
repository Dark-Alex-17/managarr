use anyhow::Result;
use log::info;

use super::{Network, NetworkEvent, NetworkResource};
use crate::{
  models::lidarr_models::{Artist, LidarrSerdeable},
  network::RequestMethod,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum LidarrEvent {
  HealthCheck,
  ListArtists,
}

impl NetworkResource for LidarrEvent {
  fn resource(&self) -> &'static str {
    match &self {
      LidarrEvent::HealthCheck => "/health",
      LidarrEvent::ListArtists => "/artist",
    }
  }
}

impl From<LidarrEvent> for NetworkEvent {
  fn from(lidarr_event: LidarrEvent) -> Self {
    NetworkEvent::Lidarr(lidarr_event)
  }
}

impl Network<'_, '_> {
  pub async fn handle_lidarr_event(
    &mut self,
    lidarr_event: LidarrEvent,
  ) -> Result<LidarrSerdeable> {
    match lidarr_event {
      LidarrEvent::HealthCheck => self
        .get_lidarr_healthcheck()
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::ListArtists => self.list_artists().await.map(LidarrSerdeable::from),
    }
  }

  async fn get_lidarr_healthcheck(&mut self) -> Result<()> {
    info!("Performing Lidarr health check");
    let event = LidarrEvent::HealthCheck;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }

  async fn list_artists(&mut self) -> Result<Vec<Artist>> {
    info!("Fetching Lidarr artists");
    let event = LidarrEvent::ListArtists;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<Artist>>(request_props, |_, _| ())
      .await
  }
}
