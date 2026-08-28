use anyhow::Result;
use log::info;

use crate::models::readarr_models::DownloadsResponse;
use crate::network::readarr_network::ReadarrEvent;
use crate::network::{Network, RequestMethod};

#[cfg(test)]
#[path = "readarr_downloads_network_tests.rs"]
mod readarr_downloads_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::readarr_network) async fn get_readarr_downloads(
    &mut self,
    count: u64,
  ) -> Result<DownloadsResponse> {
    info!("Fetching Readarr downloads");
    let event = ReadarrEvent::GetDownloads(count);

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
          .readarr_data
          .downloads
          .set_items(queue_response.records);
      })
      .await
  }
}
