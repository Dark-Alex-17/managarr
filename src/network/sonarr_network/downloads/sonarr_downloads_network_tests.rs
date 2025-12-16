#[cfg(test)]
mod tests {
  use crate::models::sonarr_models::{DownloadsResponse, SonarrSerdeable};
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::sonarr_network::SonarrEvent;
  use crate::network::sonarr_network::sonarr_network_test_utils::test_utils::{
    download_record, downloads_response,
  };
  use pretty_assertions::assert_eq;
  use serde_json::json;

  #[tokio::test]
  async fn test_handle_delete_sonarr_download_event() {
    let (mock, app, _server) = MockServarrApi::delete()
      .path("/1")
      .build_for(SonarrEvent::DeleteDownload(1))
      .await;
    app
      .lock()
      .await
      .data
      .sonarr_data
      .downloads
      .set_items(vec![download_record()]);
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    assert!(
      network
        .handle_sonarr_event(SonarrEvent::DeleteDownload(1))
        .await
        .is_ok()
    );

    mock.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_get_sonarr_downloads_event() {
    let downloads_response_json = json!({
      "records": [{
        "title": "Test Download Title",
        "status": "downloading",
        "id": 1,
        "episodeId": 1,
        "size": 3543348019f64,
        "sizeleft": 1771674009f64,
        "outputPath": "/nfs/tv/Test show/season 1/",
        "indexer": "kickass torrents",
        "downloadClient": "transmission",
      }]
    });
    let response: DownloadsResponse =
      serde_json::from_value(downloads_response_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(downloads_response_json)
      .query("pageSize=500")
      .build_for(SonarrEvent::GetDownloads(500))
      .await;
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    let SonarrSerdeable::DownloadsResponse(downloads) = network
      .handle_sonarr_event(SonarrEvent::GetDownloads(500))
      .await
      .unwrap()
    else {
      panic!("Expected DownloadsResponse")
    };
    mock.assert_async().await;
    assert_eq!(
      app.lock().await.data.sonarr_data.downloads.items,
      downloads_response().records
    );
    assert_eq!(downloads, response);
  }

  #[tokio::test]
  async fn test_handle_update_sonarr_downloads_event() {
    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(json!({
        "name": "RefreshMonitoredDownloads"
      }))
      .returns(json!({}))
      .build_for(SonarrEvent::UpdateDownloads)
      .await;
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    assert!(
      network
        .handle_sonarr_event(SonarrEvent::UpdateDownloads)
        .await
        .is_ok()
    );

    mock.assert_async().await;
  }
}
