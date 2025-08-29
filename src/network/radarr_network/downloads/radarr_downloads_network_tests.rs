#[cfg(test)]
mod tests {
  use crate::models::radarr_models::{DownloadsResponse, RadarrSerdeable};
  use crate::network::network_tests::test_utils::mock_servarr_api;
  use crate::network::radarr_network::radarr_network_test_utils::test_utils::downloads_response;
  use crate::network::radarr_network::RadarrEvent;
  use crate::network::{Network, RequestMethod};
  use reqwest::Client;
  use serde_json::json;
  use tokio_util::sync::CancellationToken;

  #[tokio::test]
  async fn test_handle_delete_radarr_download_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Delete,
      None,
      None,
      None,
      RadarrEvent::DeleteDownload(1),
      Some("/1"),
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::DeleteDownload(1))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_get_radarr_downloads_event() {
    let downloads_response_json = json!({
      "records": [{
        "title": "Test Download Title",
        "status": "downloading",
        "id": 1,
        "movieId": 1,
        "size": 3543348019u64,
        "sizeleft": 1771674009,
        "outputPath": "/nfs/movies/Test",
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
      RadarrEvent::GetDownloads(500),
      None,
      Some("pageSize=500"),
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::DownloadsResponse(downloads) = network
      .handle_radarr_event(RadarrEvent::GetDownloads(500))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      pretty_assertions::assert_eq!(
        app_arc.lock().await.data.radarr_data.downloads.items,
        downloads_response().records
      );
      pretty_assertions::assert_eq!(downloads, response);
    }
  }

  #[tokio::test]
  async fn test_handle_update_radarr_downloads_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "RefreshMonitoredDownloads"
      })),
      Some(json!({})),
      None,
      RadarrEvent::UpdateDownloads,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::UpdateDownloads)
      .await
      .is_ok());

    async_server.assert_async().await;
  }
}
