#[cfg(test)]
mod tests {
  use crate::models::lidarr_models::{DownloadsResponse, LidarrSerdeable};
  use crate::network::lidarr_network::LidarrEvent;
  use crate::network::lidarr_network::lidarr_network_test_utils::test_utils::download_record;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use pretty_assertions::assert_eq;
  use serde_json::json;

  #[tokio::test]
  async fn test_handle_delete_lidarr_download_event() {
    let (mock, app, _server) = MockServarrApi::delete()
      .path("/1")
      .build_for(LidarrEvent::DeleteDownload(1))
      .await;
    app
      .lock()
      .await
      .data
      .lidarr_data
      .downloads
      .set_items(vec![download_record()]);
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    assert!(
      network
        .handle_lidarr_event(LidarrEvent::DeleteDownload(1))
        .await
        .is_ok()
    );

    mock.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_get_downloads_event() {
    let downloads_json = json!({
      "records": [{
        "title": "Test Album",
        "status": "downloading",
        "id": 1,
        "size": 100.0,
        "sizeleft": 50.0,
        "indexer": "test-indexer"
      }]
    });
    let response: DownloadsResponse = serde_json::from_value(downloads_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(downloads_json)
      .query("pageSize=500")
      .build_for(LidarrEvent::GetDownloads(500))
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let result = network
      .handle_lidarr_event(LidarrEvent::GetDownloads(500))
      .await;

    mock.assert_async().await;

    let LidarrSerdeable::DownloadsResponse(downloads_response) = result.unwrap() else {
      panic!("Expected DownloadsResponse");
    };

    assert_eq!(downloads_response, response);
    assert!(!app.lock().await.data.lidarr_data.downloads.is_empty());
  }

  #[tokio::test]
  async fn test_handle_update_lidarr_downloads_event() {
    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(json!({
        "name": "RefreshMonitoredDownloads"
      }))
      .returns(json!({}))
      .build_for(LidarrEvent::UpdateDownloads)
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    assert!(
      network
        .handle_lidarr_event(LidarrEvent::UpdateDownloads)
        .await
        .is_ok()
    );

    mock.assert_async().await;
  }
}
