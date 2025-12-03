#[cfg(test)]
mod tests {
  use crate::models::radarr_models::{DownloadsResponse, RadarrSerdeable};
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::radarr_network::RadarrEvent;
  use crate::network::radarr_network::radarr_network_test_utils::test_utils::downloads_response;
  use serde_json::json;

  #[tokio::test]
  async fn test_handle_delete_radarr_download_event() {
    let (mock, app, _server) = MockServarrApi::delete()
      .path("/1")
      .build_for(RadarrEvent::DeleteDownload(1))
      .await;
    let mut network = test_network(&app);

    assert!(
      network
        .handle_radarr_event(RadarrEvent::DeleteDownload(1))
        .await
        .is_ok()
    );

    mock.assert_async().await;
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
    let (mock, app, _server) = MockServarrApi::get()
      .returns(downloads_response_json)
      .path("?pageSize=500")
      .build_for(RadarrEvent::GetDownloads(500))
      .await;
    let mut network = test_network(&app);

    let RadarrSerdeable::DownloadsResponse(downloads) = network
      .handle_radarr_event(RadarrEvent::GetDownloads(500))
      .await
      .unwrap()
    else {
      panic!("Expected DownloadsResponse")
    };
    mock.assert_async().await;
    pretty_assertions::assert_eq!(
      app.lock().await.data.radarr_data.downloads.items,
      downloads_response().records
    );
    pretty_assertions::assert_eq!(downloads, response);
  }

  #[tokio::test]
  async fn test_handle_update_radarr_downloads_event() {
    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(json!({
        "name": "RefreshMonitoredDownloads"
      }))
      .returns(json!({}))
      .build_for(RadarrEvent::UpdateDownloads)
      .await;
    let mut network = test_network(&app);

    assert!(
      network
        .handle_radarr_event(RadarrEvent::UpdateDownloads)
        .await
        .is_ok()
    );

    mock.assert_async().await;
  }
}
