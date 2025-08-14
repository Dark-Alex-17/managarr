#[cfg(test)]
mod tests {
  use crate::models::sonarr_models::{DownloadsResponse, SonarrSerdeable};
  use crate::network::network_tests::test_utils::mock_servarr_api;
  use crate::network::sonarr_network::sonarr_network_test_utils::test_utils::{
    download_record, downloads_response,
  };
  use crate::network::sonarr_network::SonarrEvent;
  use crate::network::{Network, RequestMethod};
  use pretty_assertions::assert_eq;
  use reqwest::Client;
  use serde_json::json;
  use tokio_util::sync::CancellationToken;

  #[tokio::test]
  async fn test_handle_delete_sonarr_download_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Delete,
      None,
      None,
      None,
      SonarrEvent::DeleteDownload(1),
      Some("/1"),
      None,
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .downloads
      .set_items(vec![download_record()]);
    app_arc.lock().await.server_tabs.next();
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::DeleteDownload(1))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_get_sonarr_downloads_event() {
    let downloads_response_json = json!({
      "records": [{
        "title": "Test Download Title",
        "status": "downloading",
        "id": 1,
        "episodeId": 1,
        "size": 3543348019f64,
        "sizeleft": 1771674009f64,
        "outputPath": "/nfs/tv/Test show/season 1/",
        "indexer": "kickass torrents",
        "downloadClient": "transmission",
      }]
    });
    let response: DownloadsResponse =
      serde_json::from_value(downloads_response_json.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(downloads_response_json),
      None,
      SonarrEvent::GetDownloads(500),
      None,
      Some("pageSize=500"),
    )
    .await;
    app_arc.lock().await.server_tabs.next();
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::DownloadsResponse(downloads) = network
      .handle_sonarr_event(SonarrEvent::GetDownloads(500))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.sonarr_data.downloads.items,
        downloads_response().records
      );
      assert_eq!(downloads, response);
    }
  }

  #[tokio::test]
  async fn test_handle_update_sonarr_downloads_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "RefreshMonitoredDownloads"
      })),
      Some(json!({})),
      None,
      SonarrEvent::UpdateDownloads,
      None,
      None,
    )
    .await;
    app_arc.lock().await.server_tabs.next();
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::UpdateDownloads)
      .await
      .is_ok());

    async_server.assert_async().await;
  }
}
