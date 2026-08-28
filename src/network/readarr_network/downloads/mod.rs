use anyhow::Result;
use log::info;
use serde_json::Value;

use crate::models::readarr_models::DownloadsResponse;
use crate::models::servarr_models::CommandBody;
use crate::network::readarr_network::ReadarrEvent;
use crate::network::{Network, RequestMethod};

#[cfg(test)]
#[path = "readarr_downloads_network_tests.rs"]
mod readarr_downloads_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::readarr_network) async fn delete_readarr_download(
    &mut self,
    download_id: i64,
  ) -> Result<()> {
    let event = ReadarrEvent::DeleteDownload(download_id);
    info!("Deleting Readarr download for download with id: {download_id}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Delete,
        None::<()>,
        Some(format!("/{download_id}")),
        None,
      )
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }

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

  pub(in crate::network::readarr_network) async fn update_readarr_downloads(
    &mut self,
  ) -> Result<Value> {
    info!("Updating Readarr downloads");
    let event = ReadarrEvent::UpdateDownloads;
    let body = CommandBody {
      name: "RefreshMonitoredDownloads".to_owned(),
    };

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(body), None, None)
      .await;

    self
      .handle_request::<CommandBody, Value>(request_props, |_, _| ())
      .await
  }
}
