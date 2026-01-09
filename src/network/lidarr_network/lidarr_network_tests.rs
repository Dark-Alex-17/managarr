#[cfg(test)]
mod tests {
  use crate::app::App;
  use crate::models::lidarr_models::{
    AddArtistBody, DeleteParams, EditArtistParams, LidarrSerdeable, MetadataProfile,
  };
  use crate::models::servarr_data::lidarr::modals::EditArtistModal;
  use crate::models::servarr_models::{QualityProfile, Tag};
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::{NetworkEvent, NetworkResource, lidarr_network::LidarrEvent};
  use bimap::BiMap;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use serde_json::json;
  use std::sync::Arc;
  use tokio::sync::Mutex;

  #[rstest]
  fn test_resource_artist(
    #[values(
      LidarrEvent::GetArtistDetails(0),
      LidarrEvent::ListArtists,
      LidarrEvent::AddArtist(AddArtistBody::default()),
      LidarrEvent::ToggleArtistMonitoring(0),
      LidarrEvent::DeleteArtist(DeleteParams::default()),
      LidarrEvent::EditArtist(EditArtistParams::default())
    )]
    event: LidarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/artist");
  }

  #[rstest]
  fn test_resource_tag(
    #[values(
      LidarrEvent::AddTag(String::new()),
      LidarrEvent::DeleteTag(0),
      LidarrEvent::GetTags
    )]
    event: LidarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/tag");
  }

  #[rstest]
  fn test_resource_config(
    #[values(LidarrEvent::GetHostConfig, LidarrEvent::GetSecurityConfig)] event: LidarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/config/host");
  }

  #[rstest]
  fn test_resource_command(
    #[values(
      LidarrEvent::UpdateAllArtists,
      LidarrEvent::TriggerAutomaticArtistSearch(0),
      LidarrEvent::UpdateAndScanArtist(0)
    )]
    event: LidarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/command");
  }

  #[rstest]
  fn test_resource_album(
    #[values(
      LidarrEvent::GetAlbums(0),
      LidarrEvent::ToggleAlbumMonitoring(0),
      LidarrEvent::GetAlbumDetails(0),
      LidarrEvent::DeleteAlbum(DeleteParams::default())
    )]
    event: LidarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/album");
  }

  #[rstest]
  #[case(LidarrEvent::GetDiskSpace, "/diskspace")]
  #[case(LidarrEvent::GetDownloads(500), "/queue")]
  #[case(LidarrEvent::GetMetadataProfiles, "/metadataprofile")]
  #[case(LidarrEvent::GetQualityProfiles, "/qualityprofile")]
  #[case(LidarrEvent::GetRootFolders, "/rootfolder")]
  #[case(LidarrEvent::GetStatus, "/system/status")]
  #[case(LidarrEvent::GetTags, "/tag")]
  #[case(LidarrEvent::HealthCheck, "/health")]
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
  async fn test_handle_get_lidarr_healthcheck_event() {
    let (mock, app, _server) = MockServarrApi::get()
      .build_for(LidarrEvent::HealthCheck)
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let _ = network.handle_lidarr_event(LidarrEvent::HealthCheck).await;

    mock.assert_async().await;
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

  #[tokio::test]
  async fn test_handle_add_lidarr_tag_event() {
    let tag_json = json!({
      "id": 1,
      "label": "usenet"
    });
    let response: Tag = serde_json::from_value(tag_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(json!({ "label": "usenet" }))
      .returns(tag_json)
      .build_for(LidarrEvent::AddTag("usenet".to_owned()))
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let result = network
      .handle_lidarr_event(LidarrEvent::AddTag("usenet".to_owned()))
      .await;

    mock.assert_async().await;

    let LidarrSerdeable::Tag(tag) = result.unwrap() else {
      panic!("Expected Tag");
    };

    assert_eq!(tag, response);
    assert_eq!(
      app.lock().await.data.lidarr_data.tags_map.get_by_left(&1),
      Some(&"usenet".to_owned())
    );
  }

  #[tokio::test]
  async fn test_handle_delete_lidarr_tag_event() {
    let (mock, app, _server) = MockServarrApi::delete()
      .path("/1")
      .build_for(LidarrEvent::DeleteTag(1))
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let result = network.handle_lidarr_event(LidarrEvent::DeleteTag(1)).await;

    mock.assert_async().await;
    assert!(result.is_ok());
  }

  #[tokio::test]
  async fn test_extract_and_add_lidarr_tag_ids_vec() {
    let app_arc = Arc::new(Mutex::new(App::test_default()));
    let tags = "    test,HI ,, usenet ";
    {
      let mut app = app_arc.lock().await;
      app.data.lidarr_data.tags_map = BiMap::from_iter([
        (1, "usenet".to_owned()),
        (2, "test".to_owned()),
        (3, "hi".to_owned()),
      ]);
    }
    app_arc.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app_arc);

    assert_eq!(
      network.extract_and_add_lidarr_tag_ids_vec(tags).await,
      vec![2, 3, 1]
    );
  }

  #[tokio::test]
  async fn test_extract_and_add_lidarr_tag_ids_vec_add_missing_tags_first() {
    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(json!({ "label": "TESTING" }))
      .returns(json!({ "id": 3, "label": "testing" }))
      .build_for(LidarrEvent::GetTags)
      .await;
    let tags = "usenet, test, TESTING";
    {
      let mut app_guard = app.lock().await;
      app_guard.data.lidarr_data.edit_artist_modal = Some(EditArtistModal {
        tags: tags.into(),
        ..EditArtistModal::default()
      });
      app_guard.data.lidarr_data.tags_map =
        BiMap::from_iter([(1, "usenet".to_owned()), (2, "test".to_owned())]);
    }
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let tag_ids_vec = network.extract_and_add_lidarr_tag_ids_vec(tags).await;

    mock.assert_async().await;
    assert_eq!(tag_ids_vec, vec![1, 2, 3]);
    assert_eq!(
      app.lock().await.data.lidarr_data.tags_map,
      BiMap::from_iter([
        (1, "usenet".to_owned()),
        (2, "test".to_owned()),
        (3, "testing".to_owned())
      ])
    );
  }
}
