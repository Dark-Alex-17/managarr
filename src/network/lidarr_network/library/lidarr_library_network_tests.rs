#[cfg(test)]
mod tests {
  use crate::models::lidarr_models::LidarrReleaseDownloadBody;
  use crate::network::lidarr_network::LidarrEvent;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use serde_json::json;

  #[tokio::test]
  async fn test_handle_download_lidarr_release_event_uses_provided_params() {
    let params = LidarrReleaseDownloadBody {
      guid: "1234".to_owned(),
      indexer_id: 2,
    };

    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(json!({
        "guid": "1234",
        "indexerId": 2,
      }))
      .returns(json!({}))
      .build_for(LidarrEvent::DownloadRelease(params.clone()))
      .await;

    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let result = network
      .handle_lidarr_event(LidarrEvent::DownloadRelease(params))
      .await;

    mock.assert_async().await;
    assert_ok!(result);
  }
}
