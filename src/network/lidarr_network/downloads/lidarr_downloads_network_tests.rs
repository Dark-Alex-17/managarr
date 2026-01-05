#[cfg(test)]
mod tests {
  use crate::models::lidarr_models::{DownloadsResponse, LidarrSerdeable};
  use crate::network::lidarr_network::LidarrEvent;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use pretty_assertions::assert_eq;
  use serde_json::json;

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
}
