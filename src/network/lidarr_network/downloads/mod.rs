use anyhow::Result;
use log::info;

use crate::models::lidarr_models::DownloadsResponse;
use crate::network::lidarr_network::LidarrEvent;
use crate::network::{Network, RequestMethod};

#[cfg(test)]
#[path = "lidarr_downloads_network_tests.rs"]
mod lidarr_downloads_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::lidarr_network) async fn get_lidarr_downloads(
    &mut self,
    count: u64,
  ) -> Result<DownloadsResponse> {
    info!("Fetching Lidarr downloads");
    let event = LidarrEvent::GetDownloads(count);

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("pageSize={count}")),
      )
      .await;

    self
      .handle_request::<(), DownloadsResponse>(request_props, |queue_response, mut app| {
        app
          .data
          .lidarr_data
          .downloads
          .set_items(queue_response.records);
      })
      .await
  }
}
