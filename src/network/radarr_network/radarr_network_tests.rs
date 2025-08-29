#[cfg(test)]
mod test {
  use super::super::*;
  use crate::models::radarr_models::{
    EditCollectionParams, EditMovieParams, IndexerSettings, RadarrTaskName,
  };
  use crate::models::servarr_data::radarr::modals::EditMovieModal;
  use crate::models::servarr_models::EditIndexerParams;
  use crate::network::network_tests::test_utils::mock_servarr_api;
  use crate::App;
  use bimap::BiMap;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use reqwest::Client;
  use rstest::rstest;
  use serde_json::json;
  use std::sync::Arc;
  use tokio::sync::Mutex;
  use tokio_util::sync::CancellationToken;

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
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      None,
      None,
      RadarrEvent::HealthCheck,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    let _ = network.handle_radarr_event(RadarrEvent::HealthCheck).await;

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_get_radarr_quality_profiles_event() {
    let quality_profile_json = json!([{
      "id": 2222,
      "name": "HD - 1080p"
    }]);
    let response: Vec<QualityProfile> =
      serde_json::from_value(quality_profile_json.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(quality_profile_json),
      None,
      RadarrEvent::GetQualityProfiles,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::QualityProfiles(quality_profiles) = network
      .handle_radarr_event(RadarrEvent::GetQualityProfiles)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.radarr_data.quality_profile_map,
        BiMap::from_iter([(2222i64, "HD - 1080p".to_owned())])
      );
      assert_eq!(quality_profiles, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_radarr_tags_event() {
    let tags_json = json!([{
      "id": 2222,
      "label": "usenet"
    }]);
    let response: Vec<Tag> = serde_json::from_value(tags_json.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(tags_json),
      None,
      RadarrEvent::GetTags,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::Tags(tags) = network
      .handle_radarr_event(RadarrEvent::GetTags)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.radarr_data.tags_map,
        BiMap::from_iter([(2222i64, "usenet".to_owned())])
      );
      assert_eq!(tags, response);
    }
  }

  #[tokio::test]
  async fn test_handle_add_radarr_tag() {
    let tag_json = json!({ "id": 3, "label": "testing" });
    let response: Tag = serde_json::from_value(tag_json.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({ "label": "testing" })),
      Some(tag_json),
      None,
      RadarrEvent::AddTag(String::new()),
      None,
      None,
    )
    .await;
    app_arc.lock().await.data.radarr_data.tags_map =
      BiMap::from_iter([(1, "usenet".to_owned()), (2, "test".to_owned())]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::Tag(tag) = network
      .handle_radarr_event(RadarrEvent::AddTag("testing".to_owned()))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.radarr_data.tags_map,
        BiMap::from_iter([
          (1, "usenet".to_owned()),
          (2, "test".to_owned()),
          (3, "testing".to_owned())
        ])
      );
      assert_eq!(tag, response);
    }
  }

  #[tokio::test]
  async fn test_handle_delete_radarr_tag_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Delete,
      None,
      None,
      None,
      RadarrEvent::DeleteTag(1),
      Some("/1"),
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::DeleteTag(1))
      .await
      .is_ok());

    async_server.assert_async().await;
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
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert_eq!(
      network.extract_and_add_radarr_tag_ids_vec(tags).await,
      vec![2, 3, 1]
    );
  }

  #[tokio::test]
  async fn test_extract_and_add_radarr_tag_ids_vec_add_missing_tags_first() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({ "label": "TESTING" })),
      Some(json!({ "id": 3, "label": "testing" })),
      None,
      RadarrEvent::GetTags,
      None,
      None,
    )
    .await;
    let tags = "usenet, test, TESTING";
    {
      let mut app = app_arc.lock().await;
      app.data.radarr_data.edit_movie_modal = Some(EditMovieModal {
        tags: tags.into(),
        ..EditMovieModal::default()
      });
      app.data.radarr_data.tags_map =
        BiMap::from_iter([(1, "usenet".to_owned()), (2, "test".to_owned())]);
    }
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    let tag_ids_vec = network.extract_and_add_radarr_tag_ids_vec(tags).await;

    async_server.assert_async().await;
    assert_eq!(tag_ids_vec, vec![1, 2, 3]);
    assert_eq!(
      app_arc.lock().await.data.radarr_data.tags_map,
      BiMap::from_iter([
        (1, "usenet".to_owned()),
        (2, "test".to_owned()),
        (3, "testing".to_owned())
      ])
    );
  }
}
