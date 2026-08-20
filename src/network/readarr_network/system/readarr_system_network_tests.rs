#[cfg(test)]
mod tests {
  use crate::models::readarr_models::ReadarrSerdeable;
  use crate::models::servarr_models::SystemStatus;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::readarr_network::ReadarrEvent;
  use chrono::DateTime;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use serde_json::json;

  #[tokio::test]
  async fn test_handle_get_readarr_status_event() {
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!({
        "version": "0.4.20.129",
        "startTime": "2026-08-17T03:55:02Z"
      }))
      .build_for(ReadarrEvent::GetStatus)
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);
    let date_time = DateTime::from(DateTime::parse_from_rfc3339("2026-08-17T03:55:02Z").unwrap());

    let result = network.handle_readarr_event(ReadarrEvent::GetStatus).await;

    mock.assert_async().await;

    let ReadarrSerdeable::SystemStatus(status) = result.unwrap() else {
      panic!("Expected SystemStatus")
    };

    assert_eq!(
      status,
      SystemStatus {
        version: "0.4.20.129".to_owned(),
        start_time: date_time
      }
    );
    assert_str_eq!(app.lock().await.data.readarr_data.version, "0.4.20.129");
    assert_eq!(app.lock().await.data.readarr_data.start_time, date_time);
  }

  #[tokio::test]
  async fn test_handle_get_readarr_status_event_failure() {
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!({}))
      .status(500)
      .build_for(ReadarrEvent::GetStatus)
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network.handle_readarr_event(ReadarrEvent::GetStatus).await;

    mock.assert_async().await;
    assert_err!(result);
    assert_is_empty!(app.lock().await.data.readarr_data.version);
  }
}
