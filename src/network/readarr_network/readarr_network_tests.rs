#[cfg(test)]
mod tests {
  use crate::models::readarr_models::ReadarrSerdeable;
  use crate::models::servarr_models::{MetadataProfile, QualityProfile, Tag};
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::{NetworkEvent, NetworkResource, readarr_network::ReadarrEvent};
  use bimap::BiMap;
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
  #[case(ReadarrEvent::GetMetadataProfiles, "/metadataprofile")]
  #[case(ReadarrEvent::GetQualityProfiles, "/qualityprofile")]
  #[case(ReadarrEvent::GetStatus, "/system/status")]
  #[case(ReadarrEvent::GetTasks, "/system/task")]
  #[case(ReadarrEvent::GetTags, "/tag")]
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
  async fn test_handle_get_metadata_profiles_event() {
    let metadata_profiles_json = json!([
      {
        "id": 1,
        "name": "Standard"
      },
      {
        "id": 2,
        "name": "None"
      }
    ]);
    let response: Vec<MetadataProfile> =
      serde_json::from_value(metadata_profiles_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(metadata_profiles_json)
      .build_for(ReadarrEvent::GetMetadataProfiles)
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetMetadataProfiles)
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::MetadataProfiles(metadata_profiles) = result.unwrap() else {
      panic!("Expected MetadataProfiles");
    };

    assert_eq!(metadata_profiles, response);

    let app = app.lock().await;

    assert_some_eq_x!(
      app.data.readarr_data.metadata_profile_map.get_by_left(&1),
      &"Standard".to_owned()
    );
    assert_some_eq_x!(
      app.data.readarr_data.metadata_profile_map.get_by_left(&2),
      &"None".to_owned()
    );
  }

  #[tokio::test]
  async fn test_handle_get_metadata_profiles_event_empty_response() {
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!([]))
      .build_for(ReadarrEvent::GetMetadataProfiles)
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.data.readarr_data.metadata_profile_map =
        BiMap::from_iter([(99i64, "Stale Profile".to_owned())]);
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetMetadataProfiles)
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::MetadataProfiles(metadata_profiles) = result.unwrap() else {
      panic!("Expected MetadataProfiles");
    };

    assert_is_empty!(metadata_profiles);
    assert_is_empty!(app.lock().await.data.readarr_data.metadata_profile_map);
  }

  #[tokio::test]
  async fn test_handle_get_metadata_profiles_event_failure() {
    let seeded_map = BiMap::from_iter([(99i64, "Stale Profile".to_owned())]);
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!({}))
      .status(500)
      .build_for(ReadarrEvent::GetMetadataProfiles)
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.data.readarr_data.metadata_profile_map = seeded_map.clone();
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetMetadataProfiles)
      .await;

    mock.assert_async().await;
    assert_err!(result);
    assert_eq!(
      app.lock().await.data.readarr_data.metadata_profile_map,
      seeded_map
    );
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
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.data.readarr_data.quality_profile_map =
        BiMap::from_iter([(99i64, "Stale Profile".to_owned())]);
    }
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
    let seeded_map = BiMap::from_iter([(99i64, "Stale Profile".to_owned())]);
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!({}))
      .status(500)
      .build_for(ReadarrEvent::GetQualityProfiles)
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.data.readarr_data.quality_profile_map = seeded_map.clone();
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetQualityProfiles)
      .await;

    mock.assert_async().await;
    assert_err!(result);
    assert_eq!(
      app.lock().await.data.readarr_data.quality_profile_map,
      seeded_map
    );
  }

  #[tokio::test]
  async fn test_handle_get_tags_event() {
    let tags_json = json!([
      {
        "id": 1,
        "label": "huntarr-missing"
      }
    ]);
    let response: Vec<Tag> = serde_json::from_value(tags_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(tags_json)
      .build_for(ReadarrEvent::GetTags)
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network.handle_readarr_event(ReadarrEvent::GetTags).await;

    mock.assert_async().await;

    let ReadarrSerdeable::Tags(tags) = result.unwrap() else {
      panic!("Expected Tags");
    };

    assert_eq!(tags, response);
    assert_some_eq_x!(
      app.lock().await.data.readarr_data.tags_map.get_by_left(&1),
      &"huntarr-missing".to_owned()
    );
  }

  #[tokio::test]
  async fn test_handle_get_tags_event_empty_response() {
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!([]))
      .build_for(ReadarrEvent::GetTags)
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.data.readarr_data.tags_map = BiMap::from_iter([(99i64, "stale-tag".to_owned())]);
    }
    let mut network = test_network(&app);

    let result = network.handle_readarr_event(ReadarrEvent::GetTags).await;

    mock.assert_async().await;

    let ReadarrSerdeable::Tags(tags) = result.unwrap() else {
      panic!("Expected Tags");
    };

    assert_is_empty!(tags);
    assert_is_empty!(app.lock().await.data.readarr_data.tags_map);
  }

  #[tokio::test]
  async fn test_handle_get_tags_event_failure() {
    let seeded_map = BiMap::from_iter([(99i64, "stale-tag".to_owned())]);
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!({}))
      .status(500)
      .build_for(ReadarrEvent::GetTags)
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.data.readarr_data.tags_map = seeded_map.clone();
    }
    let mut network = test_network(&app);

    let result = network.handle_readarr_event(ReadarrEvent::GetTags).await;

    mock.assert_async().await;
    assert_err!(result);
    assert_eq!(app.lock().await.data.readarr_data.tags_map, seeded_map);
  }
}
