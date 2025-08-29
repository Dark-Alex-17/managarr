#[cfg(test)]
mod test {
  use crate::app::App;
  use crate::models::servarr_data::sonarr::modals::AddSeriesModal;
  use crate::models::servarr_models::{
    AddRootFolderBody, EditIndexerParams, Language, QualityProfile, Tag,
  };
  use crate::models::sonarr_models::{
    AddSeriesBody, EditSeriesParams, IndexerSettings, SonarrTaskName,
  };
  use crate::models::sonarr_models::{DeleteSeriesParams, SonarrSerdeable};
  use crate::network::{
    network_tests::test_utils::mock_servarr_api, sonarr_network::SonarrEvent, Network,
    NetworkEvent, NetworkResource, RequestMethod,
  };
  use bimap::BiMap;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use reqwest::Client;
  use rstest::rstest;
  use serde_json::json;
  use std::sync::Arc;
  use tokio::sync::Mutex;
  use tokio_util::sync::CancellationToken;

  #[rstest]
  fn test_resource_all_indexer_settings(
    #[values(
      SonarrEvent::GetAllIndexerSettings,
      SonarrEvent::EditAllIndexerSettings(IndexerSettings::default())
    )]
    event: SonarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/config/indexer");
  }

  #[rstest]
  fn test_resource_episode(
    #[values(SonarrEvent::GetEpisodes(0), SonarrEvent::GetEpisodeDetails(0))] event: SonarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/episode");
  }

  #[rstest]
  fn test_resource_series(
    #[values(
      SonarrEvent::AddSeries(AddSeriesBody::default()),
      SonarrEvent::ListSeries,
      SonarrEvent::GetSeriesDetails(0),
      SonarrEvent::DeleteSeries(DeleteSeriesParams::default()),
      SonarrEvent::EditSeries(EditSeriesParams::default()),
      SonarrEvent::ToggleSeasonMonitoring((0, 0)),
      SonarrEvent::ToggleSeriesMonitoring(0),
    )]
    event: SonarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/series");
  }

  #[rstest]
  fn test_resource_tag(
    #[values(
      SonarrEvent::AddTag(String::new()),
      SonarrEvent::DeleteTag(0),
      SonarrEvent::GetTags
    )]
    event: SonarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/tag");
  }

  #[rstest]
  fn test_resource_host_config(
    #[values(SonarrEvent::GetHostConfig, SonarrEvent::GetSecurityConfig)] event: SonarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/config/host");
  }

  #[rstest]
  fn test_resource_command(
    #[values(
      SonarrEvent::GetQueuedEvents,
      SonarrEvent::StartTask(SonarrTaskName::default()),
      SonarrEvent::TriggerAutomaticEpisodeSearch(0),
      SonarrEvent::TriggerAutomaticSeasonSearch((0, 0)),
      SonarrEvent::TriggerAutomaticSeriesSearch(0),
      SonarrEvent::UpdateAllSeries,
      SonarrEvent::UpdateAndScanSeries(0),
      SonarrEvent::UpdateDownloads
    )]
    event: SonarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/command");
  }

  #[rstest]
  fn test_resource_indexer(
    #[values(
      SonarrEvent::GetIndexers,
      SonarrEvent::DeleteIndexer(0),
      SonarrEvent::EditIndexer(EditIndexerParams::default())
    )]
    event: SonarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/indexer");
  }

  #[rstest]
  fn test_resource_history(
    #[values(SonarrEvent::GetHistory(0), SonarrEvent::GetEpisodeHistory(0))] event: SonarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/history");
  }

  #[rstest]
  fn test_resource_series_history(
    #[values(
      SonarrEvent::GetSeriesHistory(0),
      SonarrEvent::GetSeasonHistory((0, 0))
    )]
    event: SonarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/history/series");
  }

  #[rstest]
  fn test_resource_queue(
    #[values(SonarrEvent::GetDownloads(0), SonarrEvent::DeleteDownload(0))] event: SonarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/queue");
  }

  #[rstest]
  fn test_resource_root_folder(
    #[values(
      SonarrEvent::GetRootFolders,
      SonarrEvent::DeleteRootFolder(0),
      SonarrEvent::AddRootFolder(AddRootFolderBody::default())
    )]
    event: SonarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/rootfolder");
  }

  #[rstest]
  fn test_resource_release(
    #[values(
      SonarrEvent::GetSeasonReleases((0, 0)),
      SonarrEvent::GetEpisodeReleases(0)
    )]
    event: SonarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/release");
  }

  #[rstest]
  fn test_resource_episode_file(
    #[values(SonarrEvent::GetEpisodeFiles(0), SonarrEvent::DeleteEpisodeFile(0))]
    event: SonarrEvent,
  ) {
    assert_str_eq!(event.resource(), "/episodefile");
  }

  #[rstest]
  #[case(SonarrEvent::ClearBlocklist, "/blocklist/bulk")]
  #[case(SonarrEvent::DeleteBlocklistItem(0), "/blocklist")]
  #[case(SonarrEvent::HealthCheck, "/health")]
  #[case(SonarrEvent::GetBlocklist, "/blocklist?page=1&pageSize=10000")]
  #[case(SonarrEvent::GetDiskSpace, "/diskspace")]
  #[case(SonarrEvent::GetLanguageProfiles, "/language")]
  #[case(SonarrEvent::GetLogs(500), "/log")]
  #[case(SonarrEvent::GetQualityProfiles, "/qualityprofile")]
  #[case(SonarrEvent::GetStatus, "/system/status")]
  #[case(SonarrEvent::GetTasks, "/system/task")]
  #[case(SonarrEvent::GetUpdates, "/update")]
  #[case(SonarrEvent::MarkHistoryItemAsFailed(0), "/history/failed")]
  #[case(SonarrEvent::SearchNewSeries(String::new()), "/series/lookup")]
  #[case(SonarrEvent::TestIndexer(0), "/indexer/test")]
  #[case(SonarrEvent::TestAllIndexers, "/indexer/testall")]
  #[case(SonarrEvent::ToggleEpisodeMonitoring(0), "/episode/monitor")]
  fn test_resource(#[case] event: SonarrEvent, #[case] expected_uri: String) {
    assert_str_eq!(event.resource(), expected_uri);
  }

  #[test]
  fn test_from_sonarr_event() {
    assert_eq!(
      NetworkEvent::Sonarr(SonarrEvent::HealthCheck),
      NetworkEvent::from(SonarrEvent::HealthCheck)
    );
  }

  #[tokio::test]
  async fn test_handle_add_sonarr_tag() {
    let tag_json = json!({ "id": 3, "label": "testing" });
    let response: Tag = serde_json::from_value(tag_json.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({ "label": "testing" })),
      Some(tag_json),
      None,
      SonarrEvent::AddTag(String::new()),
      None,
      None,
    )
    .await;
    app_arc.lock().await.data.sonarr_data.tags_map =
      BiMap::from_iter([(1, "usenet".to_owned()), (2, "test".to_owned())]);
    app_arc.lock().await.server_tabs.next();
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::Tag(tag) = network
      .handle_sonarr_event(SonarrEvent::AddTag("testing".to_owned()))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.sonarr_data.tags_map,
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
  async fn test_handle_delete_sonarr_tag_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Delete,
      None,
      None,
      None,
      SonarrEvent::DeleteTag(1),
      Some("/1"),
      None,
    )
    .await;
    app_arc.lock().await.server_tabs.next();
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::DeleteTag(1))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_get_sonarr_healthcheck_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      None,
      None,
      SonarrEvent::HealthCheck,
      None,
      None,
    )
    .await;
    app_arc.lock().await.server_tabs.next();
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    let _ = network.handle_sonarr_event(SonarrEvent::HealthCheck).await;

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_get_sonarr_language_profiles_event() {
    let language_profiles_json = json!([{
      "id": 2222,
      "name": "English"
    }]);
    let response: Vec<Language> = serde_json::from_value(language_profiles_json.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(language_profiles_json),
      None,
      SonarrEvent::GetLanguageProfiles,
      None,
      None,
    )
    .await;
    app_arc.lock().await.server_tabs.next();
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::LanguageProfiles(language_profiles) = network
      .handle_sonarr_event(SonarrEvent::GetLanguageProfiles)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.sonarr_data.language_profiles_map,
        BiMap::from_iter([(2222i64, "English".to_owned())])
      );
      assert_eq!(language_profiles, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_sonarr_quality_profiles_event() {
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
      SonarrEvent::GetQualityProfiles,
      None,
      None,
    )
    .await;
    app_arc.lock().await.server_tabs.next();
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::QualityProfiles(quality_profiles) = network
      .handle_sonarr_event(SonarrEvent::GetQualityProfiles)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.sonarr_data.quality_profile_map,
        BiMap::from_iter([(2222i64, "HD - 1080p".to_owned())])
      );
      assert_eq!(quality_profiles, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_sonarr_tags_event() {
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
      SonarrEvent::GetTags,
      None,
      None,
    )
    .await;
    app_arc.lock().await.server_tabs.next();
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::Tags(tags) = network
      .handle_sonarr_event(SonarrEvent::GetTags)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.sonarr_data.tags_map,
        BiMap::from_iter([(2222i64, "usenet".to_owned())])
      );
      assert_eq!(tags, response);
    }
  }

  #[tokio::test]
  async fn test_extract_and_add_sonarr_tag_ids_vec() {
    let app_arc = Arc::new(Mutex::new(App::test_default()));
    let tags = "    test,HI ,, usenet ";
    {
      let mut app = app_arc.lock().await;
      app.data.sonarr_data.tags_map = BiMap::from_iter([
        (1, "usenet".to_owned()),
        (2, "test".to_owned()),
        (3, "hi".to_owned()),
      ]);
    }
    app_arc.lock().await.server_tabs.next();
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert_eq!(
      network.extract_and_add_sonarr_tag_ids_vec(tags).await,
      vec![2, 3, 1]
    );
  }

  #[tokio::test]
  async fn test_extract_and_add_sonarr_tag_ids_vec_add_missing_tags_first() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({ "label": "TESTING" })),
      Some(json!({ "id": 3, "label": "testing" })),
      None,
      SonarrEvent::GetTags,
      None,
      None,
    )
    .await;
    let tags = "usenet, test, TESTING";
    {
      let mut app = app_arc.lock().await;
      app.data.sonarr_data.add_series_modal = Some(AddSeriesModal {
        tags: tags.into(),
        ..AddSeriesModal::default()
      });
      app.data.sonarr_data.tags_map =
        BiMap::from_iter([(1, "usenet".to_owned()), (2, "test".to_owned())]);
    }
    app_arc.lock().await.server_tabs.next();
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    let tag_ids_vec = network.extract_and_add_sonarr_tag_ids_vec(tags).await;

    async_server.assert_async().await;
    assert_eq!(tag_ids_vec, vec![1, 2, 3]);
    assert_eq!(
      app_arc.lock().await.data.sonarr_data.tags_map,
      BiMap::from_iter([
        (1, "usenet".to_owned()),
        (2, "test".to_owned()),
        (3, "testing".to_owned())
      ])
    );
  }
}
