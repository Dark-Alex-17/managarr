#[cfg(test)]
mod tests {
  use crate::models::servarr_data::Notification;
  use crate::models::sonarr_models::SonarrReleaseDownloadBody;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::sonarr_network::SonarrEvent;
  use pretty_assertions::assert_eq;
  use serde_json::json;

  #[tokio::test]
  async fn test_handle_download_sonarr_release_event_uses_provided_params() {
    let params = SonarrReleaseDownloadBody {
      guid: "1234".to_owned(),
      indexer_id: 2,
      series_id: Some(1),
      ..SonarrReleaseDownloadBody::default()
    };

    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(json!({
        "guid": "1234",
        "indexerId": 2,
        "seriesId": 1,
      }))
      .returns(json!({}))
      .build_for(SonarrEvent::DownloadRelease(params.clone()))
      .await;

    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    let result = network
      .handle_sonarr_event(SonarrEvent::DownloadRelease(params))
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
  async fn test_handle_download_sonarr_release_event_sets_failure_notification_on_error() {
    let params = SonarrReleaseDownloadBody {
      guid: "1234".to_owned(),
      indexer_id: 2,
      series_id: Some(1),
      ..SonarrReleaseDownloadBody::default()
    };

    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(json!({
        "guid": "1234",
        "indexerId": 2,
        "seriesId": 1,
      }))
      .returns(json!({}))
      .status(500)
      .build_for(SonarrEvent::DownloadRelease(params.clone()))
      .await;

    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    let result = network
      .handle_sonarr_event(SonarrEvent::DownloadRelease(params))
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
