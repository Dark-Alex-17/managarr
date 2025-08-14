use crate::models::servarr_models::CommandBody;
use crate::models::sonarr_models::DownloadsResponse;
use crate::network::sonarr_network::SonarrEvent;
use crate::network::{Network, RequestMethod};
use anyhow::Result;
use log::info;
use serde_json::Value;

#[cfg(test)]
#[path = "sonarr_downloads_network_tests.rs"]
mod sonarr_downloads_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::sonarr_network) async fn delete_sonarr_download(
    &mut self,
    download_id: i64,
  ) -> Result<()> {
    let event = SonarrEvent::DeleteDownload(download_id);
    info!("Deleting Sonarr download for download with id: {download_id}");

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

  pub(in crate::network::sonarr_network) async fn get_sonarr_downloads(
    &mut self,
    count: u64,
  ) -> Result<DownloadsResponse> {
    info!("Fetching Sonarr downloads");
    let event = SonarrEvent::GetDownloads(count);

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
          .sonarr_data
          .downloads
          .set_items(queue_response.records);
      })
      .await
  }

  pub(in crate::network::sonarr_network) async fn update_sonarr_downloads(
    &mut self,
  ) -> Result<Value> {
    info!("Updating Sonarr downloads");
    let event = SonarrEvent::UpdateDownloads;
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
