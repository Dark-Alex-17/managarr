#[cfg(test)]
mod tests {
  use crate::models::lidarr_models::LidarrReleaseDownloadBody;
  use crate::models::servarr_data::Notification;
  use crate::network::lidarr_network::LidarrEvent;
  use crate::network::network_tests::test_utils::{test_network, MockServarrApi};
  use pretty_assertions::assert_eq;
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
    assert_eq!(
      app.lock().await.notification,
      Some(Notification::new(
        "Download Result".to_owned(),
        "Download request sent successfully".to_owned(),
        true,
      ))
    );
  }

  #[tokio::test]
  async fn test_handle_download_lidarr_release_event_sets_failure_notification_on_error() {
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
      .status(500)
      .build_for(LidarrEvent::DownloadRelease(params.clone()))
      .await;

    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let result = network
      .handle_lidarr_event(LidarrEvent::DownloadRelease(params))
      .await;

    mock.assert_async().await;
    assert_err!(result);
    let app = app.lock().await;
    assert_is_empty!(app.error.text);
    assert_some_eq_x!(
      &app.notification,
      &Notification::new(
        "Download Failed".to_owned(),
        "Download request failed. Check the logs for more details.".to_owned(),
        false,
      )
    );
  }
}
