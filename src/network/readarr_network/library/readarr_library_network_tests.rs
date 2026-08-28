#[cfg(test)]
mod tests {
  use crate::models::servarr_data::Notification;
  use crate::models::servarr_models::ReleaseDownloadBody;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::readarr_network::ReadarrEvent;
  use pretty_assertions::assert_eq;
  use serde_json::json;

  #[tokio::test]
  async fn test_handle_download_readarr_release_event_uses_provided_params() {
    let params = ReleaseDownloadBody {
      guid: "test-release-guid".to_owned(),
      indexer_id: 6,
    };

    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(json!({
        "guid": "test-release-guid",
        "indexerId": 6,
      }))
      .returns(json!({ "id": 3 }))
      .build_for(ReadarrEvent::DownloadRelease(params.clone()))
      .await;

    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::DownloadRelease(params))
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
  async fn test_handle_download_readarr_release_event_sets_failure_notification_on_error() {
    let params = ReleaseDownloadBody {
      guid: "test-release-guid".to_owned(),
      indexer_id: 6,
    };

    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(json!({
        "guid": "test-release-guid",
        "indexerId": 6,
      }))
      .returns(json!({ "id": 3 }))
      .status(500)
      .build_for(ReadarrEvent::DownloadRelease(params.clone()))
      .await;

    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::DownloadRelease(params))
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
