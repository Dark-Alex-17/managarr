#[cfg(test)]
mod tests {
  use crate::models::sonarr_models::SonarrReleaseDownloadBody;
  use crate::network::network_tests::test_utils::mock_servarr_api;
  use crate::network::sonarr_network::SonarrEvent;
  use crate::network::{Network, RequestMethod};
  use reqwest::Client;
  use serde_json::json;
  use tokio_util::sync::CancellationToken;

  #[tokio::test]
  async fn test_handle_download_sonarr_release_event_uses_provided_params() {
    let params = SonarrReleaseDownloadBody {
      guid: "1234".to_owned(),
      indexer_id: 2,
      series_id: Some(1),
      ..SonarrReleaseDownloadBody::default()
    };
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "guid": "1234",
        "indexerId": 2,
        "seriesId": 1,
      })),
      Some(json!({})),
      None,
      SonarrEvent::DownloadRelease(params.clone()),
      None,
      None,
    )
    .await;
    app_arc.lock().await.server_tabs.next();
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::DownloadRelease(params))
      .await
      .is_ok());

    async_server.assert_async().await;
  }
}
