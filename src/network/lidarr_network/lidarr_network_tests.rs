#[cfg(test)]
mod tests {
  use crate::models::lidarr_models::{LidarrSerdeable, MetadataProfile};
  use crate::models::servarr_models::{QualityProfile, Tag};
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::{NetworkEvent, NetworkResource, lidarr_network::LidarrEvent};
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use serde_json::json;

  #[rstest]
  #[case(LidarrEvent::GetDiskSpace, "/diskspace")]
  #[case(LidarrEvent::GetDownloads(500), "/queue")]
  #[case(LidarrEvent::GetMetadataProfiles, "/metadataprofile")]
  #[case(LidarrEvent::GetQualityProfiles, "/qualityprofile")]
  #[case(LidarrEvent::GetRootFolders, "/rootfolder")]
  #[case(LidarrEvent::GetStatus, "/system/status")]
  #[case(LidarrEvent::GetTags, "/tag")]
  #[case(LidarrEvent::HealthCheck, "/health")]
  #[case(LidarrEvent::ListArtists, "/artist")]
  fn test_resource(#[case] event: LidarrEvent, #[case] expected_uri: &str) {
    assert_str_eq!(event.resource(), expected_uri);
  }

  #[test]
  fn test_from_lidarr_event() {
    assert_eq!(
      NetworkEvent::Lidarr(LidarrEvent::HealthCheck),
      NetworkEvent::from(LidarrEvent::HealthCheck)
    );
  }

  #[tokio::test]
  async fn test_handle_get_metadata_profiles_event() {
    let metadata_profiles_json = json!([{
      "id": 1,
      "name": "Standard"
    }]);
    let response: Vec<MetadataProfile> =
      serde_json::from_value(metadata_profiles_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(metadata_profiles_json)
      .build_for(LidarrEvent::GetMetadataProfiles)
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let result = network
      .handle_lidarr_event(LidarrEvent::GetMetadataProfiles)
      .await;

    mock.assert_async().await;

    let LidarrSerdeable::MetadataProfiles(metadata_profiles) = result.unwrap() else {
      panic!("Expected MetadataProfiles");
    };

    assert_eq!(metadata_profiles, response);
    assert_eq!(
      app
        .lock()
        .await
        .data
        .lidarr_data
        .metadata_profile_map
        .get_by_left(&1),
      Some(&"Standard".to_owned())
    );
  }

  #[tokio::test]
  async fn test_handle_get_quality_profiles_event() {
    let quality_profiles_json = json!([{
      "id": 1,
      "name": "Lossless"
    }]);
    let response: Vec<QualityProfile> =
      serde_json::from_value(quality_profiles_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(quality_profiles_json)
      .build_for(LidarrEvent::GetQualityProfiles)
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let result = network
      .handle_lidarr_event(LidarrEvent::GetQualityProfiles)
      .await;

    mock.assert_async().await;

    let LidarrSerdeable::QualityProfiles(quality_profiles) = result.unwrap() else {
      panic!("Expected QualityProfiles");
    };

    assert_eq!(quality_profiles, response);
    assert_eq!(
      app
        .lock()
        .await
        .data
        .lidarr_data
        .quality_profile_map
        .get_by_left(&1),
      Some(&"Lossless".to_owned())
    );
  }

  #[tokio::test]
  async fn test_handle_get_tags_event() {
    let tags_json = json!([{
      "id": 1,
      "label": "usenet"
    }]);
    let response: Vec<Tag> = serde_json::from_value(tags_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(tags_json)
      .build_for(LidarrEvent::GetTags)
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let result = network.handle_lidarr_event(LidarrEvent::GetTags).await;

    mock.assert_async().await;

    let LidarrSerdeable::Tags(tags) = result.unwrap() else {
      panic!("Expected Tags");
    };

    assert_eq!(tags, response);
    assert_eq!(
      app.lock().await.data.lidarr_data.tags_map.get_by_left(&1),
      Some(&"usenet".to_owned())
    );
  }
}
