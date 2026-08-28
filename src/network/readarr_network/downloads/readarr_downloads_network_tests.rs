#[cfg(test)]
mod tests {
  use crate::models::readarr_models::ReadarrSerdeable;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::readarr_network::ReadarrEvent;
  use crate::network::readarr_network::readarr_network_test_utils::test_utils::{
    DOWNLOAD_RECORD_JSON, download_record, downloads_response, stale_download_record,
  };
  use pretty_assertions::assert_eq;
  use serde_json::{Value, json};

  #[tokio::test]
  async fn test_handle_delete_readarr_download_event() {
    let (mock, app, _server) = MockServarrApi::delete()
      .path("/7")
      .build_for(ReadarrEvent::DeleteDownload(7))
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::DeleteDownload(7))
      .await;

    mock.assert_async().await;
    assert_ok!(result);
  }

  #[tokio::test]
  async fn test_handle_delete_readarr_download_event_failure() {
    let (mock, app, _server) = MockServarrApi::delete()
      .path("/7")
      .status(500)
      .build_for(ReadarrEvent::DeleteDownload(7))
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::DeleteDownload(7))
      .await;

    mock.assert_async().await;
    assert_err!(result);
  }

  #[tokio::test]
  async fn test_handle_get_downloads_event() {
    let downloads_json = json!({
      "records": [serde_json::from_str::<Value>(DOWNLOAD_RECORD_JSON).unwrap()]
    });
    let (mock, app, _server) = MockServarrApi::get()
      .returns(downloads_json)
      .query("pageSize=250")
      .build_for(ReadarrEvent::GetDownloads(250))
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app
        .data
        .readarr_data
        .downloads
        .set_items(vec![stale_download_record()]);
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetDownloads(250))
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::DownloadsResponse(downloads) = result.unwrap() else {
      panic!("Expected DownloadsResponse")
    };

    assert_eq!(downloads, downloads_response());
    assert_eq!(
      app.lock().await.data.readarr_data.downloads.items,
      vec![download_record()]
    );
  }

  #[tokio::test]
  async fn test_handle_get_downloads_event_failure() {
    let stale_downloads = vec![stale_download_record()];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!({
        "records": [serde_json::from_str::<Value>(DOWNLOAD_RECORD_JSON).unwrap()]
      }))
      .status(500)
      .query("pageSize=500")
      .build_for(ReadarrEvent::GetDownloads(500))
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app
        .data
        .readarr_data
        .downloads
        .set_items(stale_downloads.clone());
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetDownloads(500))
      .await;

    mock.assert_async().await;

    assert_err!(result);
    assert_eq!(
      app.lock().await.data.readarr_data.downloads.items,
      stale_downloads
    );
  }
}
