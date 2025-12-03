#[cfg(test)]
mod test {
  use super::super::*;
  use crate::App;
  use crate::models::radarr_models::{
    EditCollectionParams, EditMovieParams, IndexerSettings, RadarrTaskName,
  };
  use crate::models::servarr_data::radarr::modals::EditMovieModal;
  use crate::models::servarr_models::EditIndexerParams;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::radarr_network::radarr_network_test_utils::test_utils::{
    quality_profile, tag,
  };
  use bimap::BiMap;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use serde_json::json;
  use std::sync::Arc;
  use tokio::sync::Mutex;

  #[rstest]
  fn test_resource_movie(
    #[values(
      RadarrEvent::AddMovie(AddMovieBody::default()),
      RadarrEvent::EditMovie(EditMovieParams::default()),
      RadarrEvent::GetMovies,
      RadarrEvent::GetMovieDetails(0),
      RadarrEvent::DeleteMovie(DeleteMovieParams::default()),
      RadarrEvent::ToggleMovieMonitoring(0)
    )]
    event: RadarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/movie");
  }

  #[rstest]
  fn test_resource_collection(
    #[values(
      RadarrEvent::GetCollections,
      RadarrEvent::EditCollection(EditCollectionParams::default())
    )]
    event: RadarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/collection");
  }

  #[rstest]
  fn test_resource_indexer(
    #[values(
      RadarrEvent::GetIndexers,
      RadarrEvent::DeleteIndexer(0),
      RadarrEvent::EditIndexer(EditIndexerParams::default())
    )]
    event: RadarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/indexer");
  }

  #[rstest]
  fn test_resource_all_indexer_settings(
    #[values(
      RadarrEvent::GetAllIndexerSettings,
      RadarrEvent::EditAllIndexerSettings(IndexerSettings::default())
    )]
    event: RadarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/config/indexer");
  }

  #[rstest]
  fn test_resource_root_folder(
    #[values(
      RadarrEvent::AddRootFolder(AddRootFolderBody::default()),
      RadarrEvent::GetRootFolders,
      RadarrEvent::DeleteRootFolder(0)
    )]
    event: RadarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/rootfolder");
  }

  #[rstest]
  fn test_resource_tag(
    #[values(
      RadarrEvent::AddTag(String::new()),
      RadarrEvent::GetTags,
      RadarrEvent::DeleteTag(0)
    )]
    event: RadarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/tag");
  }

  #[rstest]
  fn test_resource_release(
    #[values(
      RadarrEvent::GetReleases(0),
      RadarrEvent::DownloadRelease(RadarrReleaseDownloadBody::default())
    )]
    event: RadarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/release");
  }

  #[rstest]
  fn test_resource_queue(
    #[values(RadarrEvent::GetDownloads(0), RadarrEvent::DeleteDownload(0))] event: RadarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/queue");
  }

  #[rstest]
  fn test_resource_host_config(
    #[values(RadarrEvent::GetHostConfig, RadarrEvent::GetSecurityConfig)] event: RadarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/config/host");
  }

  #[rstest]
  fn test_resource_command(
    #[values(
      RadarrEvent::StartTask(RadarrTaskName::default()),
      RadarrEvent::GetQueuedEvents,
      RadarrEvent::TriggerAutomaticSearch(0),
      RadarrEvent::UpdateAndScan(0),
      RadarrEvent::UpdateAllMovies,
      RadarrEvent::UpdateDownloads,
      RadarrEvent::UpdateCollections
    )]
    event: RadarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/command");
  }

  #[rstest]
  #[case(RadarrEvent::ClearBlocklist, "/blocklist/bulk")]
  #[case(RadarrEvent::DeleteBlocklistItem(1), "/blocklist")]
  #[case(RadarrEvent::GetBlocklist, "/blocklist?page=1&pageSize=10000")]
  #[case(RadarrEvent::GetLogs(500), "/log")]
  #[case(RadarrEvent::SearchNewMovie(String::new()), "/movie/lookup")]
  #[case(RadarrEvent::GetMovieCredits(0), "/credit")]
  #[case(RadarrEvent::GetMovieHistory(0), "/history/movie")]
  #[case(RadarrEvent::GetDiskSpace, "/diskspace")]
  #[case(RadarrEvent::GetQualityProfiles, "/qualityprofile")]
  #[case(RadarrEvent::GetStatus, "/system/status")]
  #[case(RadarrEvent::GetTasks, "/system/task")]
  #[case(RadarrEvent::GetUpdates, "/update")]
  #[case(RadarrEvent::TestIndexer(0), "/indexer/test")]
  #[case(RadarrEvent::TestAllIndexers, "/indexer/testall")]
  #[case(RadarrEvent::HealthCheck, "/health")]
  fn test_resource(#[case] event: RadarrEvent, #[case] expected_uri: String) {
    assert_str_eq!(event.resource(), expected_uri);
  }

  #[test]
  fn test_from_radarr_event() {
    assert_eq!(
      NetworkEvent::Radarr(RadarrEvent::HealthCheck),
      NetworkEvent::from(RadarrEvent::HealthCheck)
    );
  }

  #[tokio::test]
  async fn test_handle_get_radarr_healthcheck_event() {
    let (mock, app, _server) = MockServarrApi::get()
      .build_for(RadarrEvent::HealthCheck)
      .await;

    let mut network = test_network(&app);

    let _ = network.handle_radarr_event(RadarrEvent::HealthCheck).await;

    mock.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_get_radarr_quality_profiles_event() {
    let expected: Vec<QualityProfile> = vec![QualityProfile {
      id: 2222,
      name: "HD - 1080p".to_owned(),
    }];

    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!([quality_profile()]))
      .build_for(RadarrEvent::GetQualityProfiles)
      .await;

    let mut network = test_network(&app);

    let result = network
      .handle_radarr_event(RadarrEvent::GetQualityProfiles)
      .await;

    mock.assert_async().await;

    let RadarrSerdeable::QualityProfiles(quality_profiles) = result.unwrap() else {
      panic!("Expected QualityProfiles variant");
    };
    assert_eq!(quality_profiles, expected);
    assert_eq!(
      app.lock().await.data.radarr_data.quality_profile_map,
      BiMap::from_iter([(2222i64, "HD - 1080p".to_owned())])
    );
  }

  #[tokio::test]
  async fn test_handle_get_radarr_tags_event() {
    let expected: Vec<Tag> = vec![Tag {
      id: 2222,
      label: "usenet".to_owned(),
    }];

    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!([{
        "id": 2222,
        "label": "usenet"
      }]))
      .build_for(RadarrEvent::GetTags)
      .await;

    let mut network = test_network(&app);

    let result = network.handle_radarr_event(RadarrEvent::GetTags).await;

    mock.assert_async().await;

    let RadarrSerdeable::Tags(tags) = result.unwrap() else {
      panic!("Expected Tags variant");
    };
    assert_eq!(tags, expected);
    assert_eq!(
      app.lock().await.data.radarr_data.tags_map,
      BiMap::from_iter([(2222i64, "usenet".to_owned())])
    );
  }

  #[tokio::test]
  async fn test_handle_add_radarr_tag() {
    let expected = Tag {
      id: 3,
      label: "testing".to_owned(),
    };

    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(json!({ "label": "testing" }))
      .returns(tag())
      .build_for(RadarrEvent::AddTag(String::new()))
      .await;

    app.lock().await.data.radarr_data.tags_map =
      BiMap::from_iter([(1, "usenet".to_owned()), (2, "test".to_owned())]);

    let mut network = test_network(&app);

    let result = network
      .handle_radarr_event(RadarrEvent::AddTag("testing".to_owned()))
      .await;

    mock.assert_async().await;

    let RadarrSerdeable::Tag(tag) = result.unwrap() else {
      panic!("Expected Tag variant");
    };
    assert_eq!(tag, expected);
    assert_eq!(
      app.lock().await.data.radarr_data.tags_map,
      BiMap::from_iter([
        (1, "usenet".to_owned()),
        (2, "test".to_owned()),
        (3, "testing".to_owned())
      ])
    );
  }

  #[tokio::test]
  async fn test_handle_delete_radarr_tag_event() {
    let (mock, app, _server) = MockServarrApi::delete()
      .path("/1")
      .build_for(RadarrEvent::DeleteTag(1))
      .await;

    let mut network = test_network(&app);

    let result = network.handle_radarr_event(RadarrEvent::DeleteTag(1)).await;

    mock.assert_async().await;
    assert!(result.is_ok());
  }

  #[tokio::test]
  async fn test_extract_and_add_radarr_tag_ids_vec() {
    let app_arc = Arc::new(Mutex::new(App::test_default()));
    let tags = "    test,HI ,, usenet ";
    {
      let mut app = app_arc.lock().await;
      app.data.radarr_data.tags_map = BiMap::from_iter([
        (1, "usenet".to_owned()),
        (2, "test".to_owned()),
        (3, "hi".to_owned()),
      ]);
    }
    let mut network = test_network(&app_arc);

    assert_eq!(
      network.extract_and_add_radarr_tag_ids_vec(tags).await,
      vec![2, 3, 1]
    );
  }

  #[tokio::test]
  async fn test_extract_and_add_radarr_tag_ids_vec_add_missing_tags_first() {
    let (async_server, app, _server) = MockServarrApi::post()
      .with_request_body(json!({ "label": "TESTING" }))
      .returns(json!({ "id": 3, "label": "testing" }))
      .build_for(RadarrEvent::GetTags)
      .await;
    let tags = "usenet, test, TESTING";
    {
      let mut app_guard = app.lock().await;
      app_guard.data.radarr_data.edit_movie_modal = Some(EditMovieModal {
        tags: tags.into(),
        ..EditMovieModal::default()
      });
      app_guard.data.radarr_data.tags_map =
        BiMap::from_iter([(1, "usenet".to_owned()), (2, "test".to_owned())]);
    }
    let mut network = test_network(&app);

    let tag_ids_vec = network.extract_and_add_radarr_tag_ids_vec(tags).await;

    async_server.assert_async().await;
    assert_eq!(tag_ids_vec, vec![1, 2, 3]);
    assert_eq!(
      app.lock().await.data.radarr_data.tags_map,
      BiMap::from_iter([
        (1, "usenet".to_owned()),
        (2, "test".to_owned()),
        (3, "testing".to_owned())
      ])
    );
  }
}
