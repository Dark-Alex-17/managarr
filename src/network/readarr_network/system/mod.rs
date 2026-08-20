use crate::models::servarr_models::{DiskSpace, HostConfig, SystemStatus};
use crate::network::readarr_network::ReadarrEvent;
use crate::network::{Network, RequestMethod};
use anyhow::Result;
use log::info;

#[cfg(test)]
#[path = "readarr_system_network_tests.rs"]
mod readarr_system_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::readarr_network) async fn get_readarr_diskspace(
    &mut self,
  ) -> Result<Vec<DiskSpace>> {
    info!("Fetching Readarr disk space");
    let event = ReadarrEvent::GetDiskSpace;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<DiskSpace>>(request_props, |disk_space_vec, mut app| {
        app.data.readarr_data.disk_space_vec = disk_space_vec;
      })
      .await
  }

  pub(in crate::network::readarr_network) async fn get_readarr_host_config(
    &mut self,
  ) -> Result<HostConfig> {
    info!("Fetching Readarr host config");
    let event = ReadarrEvent::GetHostConfig;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), HostConfig>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::readarr_network) async fn get_readarr_status(
    &mut self,
  ) -> Result<SystemStatus> {
    info!("Fetching Readarr system status");
    let event = ReadarrEvent::GetStatus;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), SystemStatus>(request_props, |system_status, mut app| {
        app.data.readarr_data.version = system_status.version;
        app.data.readarr_data.start_time = system_status.start_time;
      })
      .await
  }
}
