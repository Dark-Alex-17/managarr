use anyhow::Result;
use log::info;

use crate::models::lidarr_models::SystemStatus;
use crate::models::servarr_models::DiskSpace;
use crate::network::lidarr_network::LidarrEvent;
use crate::network::{Network, RequestMethod};

#[cfg(test)]
#[path = "lidarr_system_network_tests.rs"]
mod lidarr_system_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::lidarr_network) async fn get_lidarr_healthcheck(&mut self) -> Result<()> {
    info!("Performing Lidarr health check");
    let event = LidarrEvent::HealthCheck;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::lidarr_network) async fn get_lidarr_diskspace(
    &mut self,
  ) -> Result<Vec<DiskSpace>> {
    info!("Fetching Lidarr disk space");
    let event = LidarrEvent::GetDiskSpace;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<DiskSpace>>(request_props, |disk_space_vec, mut app| {
        app.data.lidarr_data.disk_space_vec = disk_space_vec;
      })
      .await
  }

  pub(in crate::network::lidarr_network) async fn get_lidarr_status(
    &mut self,
  ) -> Result<SystemStatus> {
    info!("Fetching Lidarr system status");
    let event = LidarrEvent::GetStatus;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), SystemStatus>(request_props, |system_status, mut app| {
        app.data.lidarr_data.version = system_status.version;
        app.data.lidarr_data.start_time = system_status.start_time;
      })
      .await
  }
}
