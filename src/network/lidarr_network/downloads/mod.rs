use crate::models::lidarr_models::DownloadsResponse;
use crate::models::servarr_models::CommandBody;
use crate::network::lidarr_network::LidarrEvent;
use crate::network::{Network, RequestMethod};
use anyhow::Result;
use log::info;
use serde_json::Value;

#[cfg(test)]
#[path = "lidarr_downloads_network_tests.rs"]
mod lidarr_downloads_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::lidarr_network) async fn delete_lidarr_download(
    &mut self,
    download_id: i64,
  ) -> Result<()> {
    let event = LidarrEvent::DeleteDownload(download_id);
    info!("Deleting Lidarr download for download with id: {download_id}");

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

  pub(in crate::network::lidarr_network) async fn update_lidarr_downloads(
    &mut self,
  ) -> Result<Value> {
    info!("Updating Lidarr downloads");
    let event = LidarrEvent::UpdateDownloads;
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
