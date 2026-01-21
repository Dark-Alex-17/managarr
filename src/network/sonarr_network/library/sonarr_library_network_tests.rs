#[cfg(test)]
mod tests {
  use crate::models::sonarr_models::SonarrReleaseDownloadBody;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::sonarr_network::SonarrEvent;
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
  }
}
