use crate::models::radarr_models::DownloadsResponse;
use crate::models::servarr_models::CommandBody;
use crate::network::radarr_network::RadarrEvent;
use crate::network::{Network, RequestMethod};
use anyhow::Result;
use log::info;
use serde_json::Value;

#[cfg(test)]
#[path = "radarr_downloads_network_tests.rs"]
mod radarr_downloads_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::radarr_network) async fn delete_radarr_download(
    &mut self,
    download_id: i64,
  ) -> Result<()> {
    let event = RadarrEvent::DeleteDownload(download_id);
    info!("Deleting Radarr download for download with id: {download_id}");

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

  pub(in crate::network::radarr_network) async fn get_radarr_downloads(
    &mut self,
    count: u64,
  ) -> Result<DownloadsResponse> {
    info!("Fetching Radarr downloads");
    let event = RadarrEvent::GetDownloads(count);

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
          .radarr_data
          .downloads
          .set_items(queue_response.records);
      })
      .await
  }

  pub(in crate::network::radarr_network) async fn update_radarr_downloads(
    &mut self,
  ) -> Result<Value> {
    info!("Updating Radarr downloads");
    let event = RadarrEvent::UpdateDownloads;
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
