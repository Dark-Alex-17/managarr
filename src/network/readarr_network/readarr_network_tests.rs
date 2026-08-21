#[cfg(test)]
mod tests {
  use crate::models::readarr_models::ReadarrSerdeable;
  use crate::models::servarr_models::QualityProfile;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::{NetworkEvent, NetworkResource, readarr_network::ReadarrEvent};
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use serde_json::json;

  #[rstest]
  #[case(ReadarrEvent::GetQueuedEvents, "/command")]
  #[case(ReadarrEvent::StartTask(Default::default()), "/command")]
  #[case(ReadarrEvent::GetHostConfig, "/config/host")]
  #[case(ReadarrEvent::GetSecurityConfig, "/config/host")]
  #[case(ReadarrEvent::GetDiskSpace, "/diskspace")]
  #[case(ReadarrEvent::HealthCheck, "/health")]
  #[case(ReadarrEvent::GetLogs(500), "/log")]
  #[case(ReadarrEvent::GetQualityProfiles, "/qualityprofile")]
  #[case(ReadarrEvent::GetStatus, "/system/status")]
  #[case(ReadarrEvent::GetTasks, "/system/task")]
  #[case(ReadarrEvent::GetUpdates, "/update")]
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

  #[tokio::test]
  async fn test_handle_get_quality_profiles_event() {
    let quality_profiles_json = json!([
      {
        "id": 1,
        "name": "eBook"
      },
      {
        "id": 2,
        "name": "Spoken"
      }
    ]);
    let response: Vec<QualityProfile> =
      serde_json::from_value(quality_profiles_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(quality_profiles_json)
      .build_for(ReadarrEvent::GetQualityProfiles)
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetQualityProfiles)
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::QualityProfiles(quality_profiles) = result.unwrap() else {
      panic!("Expected QualityProfiles");
    };

    assert_eq!(quality_profiles, response);

    let app = app.lock().await;

    assert_some_eq_x!(
      app.data.readarr_data.quality_profile_map.get_by_left(&1),
      &"eBook".to_owned()
    );
    assert_some_eq_x!(
      app.data.readarr_data.quality_profile_map.get_by_left(&2),
      &"Spoken".to_owned()
    );
  }

  #[tokio::test]
  async fn test_handle_get_quality_profiles_event_empty_response() {
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!([]))
      .build_for(ReadarrEvent::GetQualityProfiles)
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetQualityProfiles)
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::QualityProfiles(quality_profiles) = result.unwrap() else {
      panic!("Expected QualityProfiles");
    };

    assert_is_empty!(quality_profiles);
    assert_is_empty!(app.lock().await.data.readarr_data.quality_profile_map);
  }

  #[tokio::test]
  async fn test_handle_get_quality_profiles_event_failure() {
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!({}))
      .status(500)
      .build_for(ReadarrEvent::GetQualityProfiles)
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetQualityProfiles)
      .await;

    mock.assert_async().await;
    assert_err!(result);
    assert_is_empty!(app.lock().await.data.readarr_data.quality_profile_map);
  }
}
