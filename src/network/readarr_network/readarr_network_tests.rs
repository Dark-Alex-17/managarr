#[cfg(test)]
mod tests {
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::{NetworkEvent, NetworkResource, readarr_network::ReadarrEvent};
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;

  #[rstest]
  #[case(ReadarrEvent::GetHostConfig, "/config/host")]
  #[case(ReadarrEvent::GetDiskSpace, "/diskspace")]
  #[case(ReadarrEvent::HealthCheck, "/health")]
  #[case(ReadarrEvent::GetStatus, "/system/status")]
  fn test_resource(#[case] event: ReadarrEvent, #[case] expected_uri: &str) {
    assert_str_eq!(event.resource(), expected_uri);
  }

  #[test]
  fn test_from_readarr_event() {
    assert_eq!(
      NetworkEvent::Readarr(ReadarrEvent::HealthCheck),
      NetworkEvent::from(ReadarrEvent::HealthCheck)
    );
  }

  #[tokio::test]
  async fn test_handle_get_readarr_healthcheck_event() {
    let (mock, app, _server) = MockServarrApi::get()
      .build_for(ReadarrEvent::HealthCheck)
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let _ = network
      .handle_readarr_event(ReadarrEvent::HealthCheck)
      .await;

    mock.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_get_readarr_healthcheck_event_failure() {
    let (mock, app, _server) = MockServarrApi::get()
      .status(500)
      .build_for(ReadarrEvent::HealthCheck)
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::HealthCheck)
      .await;

    mock.assert_async().await;
    assert_err!(result);
  }
}
