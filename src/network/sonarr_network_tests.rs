#[cfg(test)]
mod test {
  use std::sync::Arc;

  use bimap::BiMap;
  use chrono::DateTime;
  use indoc::formatdoc;
  use mockito::Matcher;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use reqwest::Client;
  use rstest::rstest;
  use serde_json::json;
  use serde_json::{Number, Value};
  use tokio::sync::Mutex;
  use tokio_util::sync::CancellationToken;

  use crate::models::sonarr_models::{
    AddSeriesBody, AddSeriesOptions, AddSeriesSearchResult, AddSeriesSearchResultStatistics,
    DownloadStatus, EditSeriesParams, IndexerSettings, MonitorEpisodeBody, SonarrHistoryEventType,
  };

  use crate::app::App;
  use crate::models::radarr_models::IndexerTestResult;
  use crate::models::servarr_data::modals::IndexerTestResultModalItem;
  use crate::models::servarr_data::sonarr::modals::{
    AddSeriesModal, EpisodeDetailsModal, SeasonDetailsModal,
  };
  use crate::models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock;
  use crate::models::servarr_models::{
    AddRootFolderBody, DiskSpace, EditIndexerParams, HostConfig, Indexer, IndexerField, Language,
    LogResponse, Quality, QualityProfile, QualityWrapper, QueueEvent, RootFolder, SecurityConfig,
    Tag, Update,
  };
  use crate::models::sonarr_models::{
    BlocklistItem, DeleteSeriesParams, DownloadRecord, DownloadsResponse, Episode, EpisodeFile,
    MediaInfo, SonarrRelease, SonarrReleaseDownloadBody, SonarrTaskName,
  };
  use crate::models::sonarr_models::{
    BlocklistResponse, SonarrHistoryData, SonarrHistoryItem, SonarrHistoryWrapper,
  };
  use crate::models::sonarr_models::{SonarrTask, SystemStatus};
  use crate::models::stateful_table::StatefulTable;
  use crate::models::{sonarr_models::SonarrSerdeable, stateful_table::SortOption};
  use crate::models::{HorizontallyScrollableText, ScrollableText};

  use crate::network::sonarr_network::get_episode_status;
  use crate::{
    models::sonarr_models::{
      Rating, Season, SeasonStatistics, Series, SeriesStatistics, SeriesStatus, SeriesType,
    },
    network::{
      network_tests::test_utils::mock_servarr_api, sonarr_network::SonarrEvent, Network,
      NetworkEvent, NetworkResource, RequestMethod,
    },
  };

  const SERIES_JSON: &str = r#"{
        "title": "Test",
        "status": "continuing",
        "ended": false,
        "overview": "Blah blah blah",
        "network": "HBO",
        "seasons": [
            {
                "seasonNumber": 1,
                "monitored": true,
                "statistics": {
                    "previousAiring": "2022-10-24T01:00:00Z",
                    "episodeFileCount": 10,
                    "episodeCount": 10,
                    "totalEpisodeCount": 10,
                    "sizeOnDisk": 36708563419,
                    "percentOfEpisodes": 100.0
                }
            }
        ],
        "year": 2022,
        "path": "/nfs/tv/Test",
        "qualityProfileId": 6,
        "languageProfileId": 1,
        "seasonFolder": true,
        "monitored": true,
        "runtime": 63,
        "tvdbId": 371572,
        "seriesType": "standard",
        "certification": "TV-MA",
        "genres": ["cool", "family", "fun"],
        "tags": [3],
        "ratings": {"votes": 406744, "value": 8.4},
        "statistics": {
            "seasonCount": 2,
            "episodeFileCount": 18,
            "episodeCount": 18,
            "totalEpisodeCount": 50,
            "sizeOnDisk": 63894022699,
            "percentOfEpisodes": 100.0
        },
        "id": 1
    }
"#;
  const EPISODE_JSON: &str = r#"{
    "seriesId": 1,
    "tvdbId": 1234,
    "episodeFileId": 1,
    "seasonNumber": 1,
    "episodeNumber": 1,
    "title": "Something cool",
    "airDateUtc": "2024-02-10T07:28:45Z",
    "overview": "Okay so this one time at band camp...",
    "episodeFile": {
        "id": 1,
        "relativePath": "/season 1/episode 1.mkv",
        "path": "/nfs/tv/series/season 1/episode 1.mkv",
        "size": 3543348019,
        "dateAdded": "2024-02-10T07:28:45Z",
        "languages": [{ "id": 1, "name": "English" }],
        "quality": { "quality": { "name": "Bluray-1080p" } },
        "mediaInfo": {
            "audioBitrate": 0,
            "audioChannels": 7.1,
            "audioCodec": "AAC",
            "audioLanguages": "eng",
            "audioStreamCount": 1,
            "videoBitDepth": 10,
            "videoBitrate": 0,
            "videoCodec": "x265",
            "videoFps": 23.976,
            "resolution": "1920x1080",
            "runTime": "23:51",
            "scanType": "Progressive",
            "subtitles": "English"
        }
    },
    "hasFile": true,
    "monitored": true,
    "id": 1
  }"#;

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
      SonarrEvent::ToggleSeasonMonitoring((0, 0))
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
      SonarrEvent::TriggerAutomaticSeriesSearch(None),
      SonarrEvent::UpdateAllSeries,
      SonarrEvent::UpdateAndScanSeries(None),
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
    #[values(SonarrEvent::GetDownloads, SonarrEvent::DeleteDownload(0))] event: SonarrEvent,
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
  async fn test_handle_add_sonarr_root_folder_event() {
    let expected_add_root_folder_body = AddRootFolderBody {
      path: "/nfs/test".to_owned(),
    };
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "path": "/nfs/test"
      })),
      Some(json!({})),
      None,
      SonarrEvent::AddRootFolder(expected_add_root_folder_body.clone()),
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::AddRootFolder(expected_add_root_folder_body))
      .await
      .is_ok());

    async_server.assert_async().await;
    assert!(app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .edit_root_folder
      .is_none());
  }

  #[tokio::test]
  async fn test_handle_add_sonarr_series_event() {
    let expected_add_series_body = AddSeriesBody {
      tvdb_id: 1234,
      title: "Test".to_owned(),
      monitored: true,
      root_folder_path: "/nfs2".to_owned(),
      quality_profile_id: 2222,
      language_profile_id: 2222,
      series_type: "standard".to_owned(),
      season_folder: true,
      tags: Vec::new(),
      tag_input_string: Some("usenet, testing".to_owned()),
      add_options: AddSeriesOptions {
        monitor: "all".to_owned(),
        search_for_cutoff_unmet_episodes: true,
        search_for_missing_episodes: true,
      },
    };
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "tvdbId": 1234,
        "title": "Test",
        "monitored": true,
        "rootFolderPath": "/nfs2",
        "qualityProfileId": 2222,
        "languageProfileId": 2222,
        "seriesType": "standard",
        "seasonFolder": true,
        "tags": [1, 2],
        "addOptions": {
          "monitor": "all",
          "searchForCutoffUnmetEpisodes": true,
          "searchForMissingEpisodes": true
        }
      })),
      Some(json!({})),
      None,
      SonarrEvent::AddSeries(expected_add_series_body.clone()),
      None,
      None,
    )
    .await;
    app_arc.lock().await.data.sonarr_data.tags_map =
      BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::AddSeries(expected_add_series_body))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_add_sonarr_series_event_does_not_overwrite_tags_vec_when_tag_input_string_is_none(
  ) {
    let expected_add_series_body = AddSeriesBody {
      tvdb_id: 1234,
      title: "Test".to_owned(),
      monitored: true,
      root_folder_path: "/nfs2".to_owned(),
      quality_profile_id: 2222,
      language_profile_id: 2222,
      series_type: "standard".to_owned(),
      season_folder: true,
      tags: vec![1, 2],
      tag_input_string: None,
      add_options: AddSeriesOptions {
        monitor: "all".to_owned(),
        search_for_cutoff_unmet_episodes: true,
        search_for_missing_episodes: true,
      },
    };
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "tvdbId": 1234,
        "title": "Test",
        "monitored": true,
        "rootFolderPath": "/nfs2",
        "qualityProfileId": 2222,
        "languageProfileId": 2222,
        "seriesType": "standard",
        "seasonFolder": true,
        "tags": [1, 2],
        "addOptions": {
          "monitor": "all",
          "searchForCutoffUnmetEpisodes": true,
          "searchForMissingEpisodes": true
        }
      })),
      Some(json!({})),
      None,
      SonarrEvent::AddSeries(expected_add_series_body.clone()),
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::AddSeries(expected_add_series_body))
      .await
      .is_ok());

    async_server.assert_async().await;
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
  async fn test_handle_clear_radarr_blocklist_event() {
    let blocklist_items = vec![
      BlocklistItem {
        id: 1,
        ..blocklist_item()
      },
      BlocklistItem {
        id: 2,
        ..blocklist_item()
      },
      BlocklistItem {
        id: 3,
        ..blocklist_item()
      },
    ];
    let expected_request_json = json!({ "ids": [1, 2, 3]});
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Delete,
      Some(expected_request_json),
      None,
      None,
      SonarrEvent::ClearBlocklist,
      None,
      None,
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .blocklist
      .set_items(blocklist_items);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::ClearBlocklist)
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_delete_sonarr_blocklist_item_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Delete,
      None,
      None,
      None,
      SonarrEvent::DeleteBlocklistItem(1),
      Some("/1"),
      None,
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .blocklist
      .set_items(vec![blocklist_item()]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::DeleteBlocklistItem(1))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_delete_sonarr_episode_file_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Delete,
      None,
      None,
      None,
      SonarrEvent::DeleteEpisodeFile(1),
      Some("/1"),
      None,
    )
    .await;
    app_arc.lock().await.data.sonarr_data.season_details_modal =
      Some(SeasonDetailsModal::default());
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::DeleteEpisodeFile(1))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_delete_sonarr_download_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Delete,
      None,
      None,
      None,
      SonarrEvent::DeleteDownload(1),
      Some("/1"),
      None,
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .downloads
      .set_items(vec![download_record()]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::DeleteDownload(1))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_delete_sonarr_indexer_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Delete,
      None,
      None,
      None,
      SonarrEvent::DeleteIndexer(1),
      Some("/1"),
      None,
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .indexers
      .set_items(vec![indexer()]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::DeleteIndexer(1))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_delete_sonarr_root_folder_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Delete,
      None,
      None,
      None,
      SonarrEvent::DeleteRootFolder(1),
      Some("/1"),
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::DeleteRootFolder(1))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_delete_series_event() {
    let delete_series_params = DeleteSeriesParams {
      id: 1,
      delete_series_files: true,
      add_list_exclusion: true,
    };
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Delete,
      None,
      None,
      None,
      SonarrEvent::DeleteSeries(delete_series_params.clone()),
      Some("/1"),
      Some("deleteFiles=true&addImportExclusion=true"),
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::DeleteSeries(delete_series_params))
      .await
      .is_ok());

    async_server.assert_async().await;
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
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::DeleteTag(1))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_download_sonarr_release_event_uses_provided_params() {
    let params = SonarrReleaseDownloadBody {
      guid: "1234".to_owned(),
      indexer_id: 2,
      series_id: Some(1),
      ..SonarrReleaseDownloadBody::default()
    };
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "guid": "1234",
        "indexerId": 2,
        "seriesId": 1,
      })),
      Some(json!({})),
      None,
      SonarrEvent::DownloadRelease(params.clone()),
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::DownloadRelease(params))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_all_indexer_settings_event() {
    let indexer_settings_json = json!({
        "id": 1,
        "minimumAge": 1,
        "maximumSize": 12345,
        "retention": 1,
        "rssSyncInterval": 60
    });
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Put,
      Some(indexer_settings_json),
      None,
      None,
      SonarrEvent::EditAllIndexerSettings(indexer_settings()),
      None,
      None,
    )
    .await;

    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::EditAllIndexerSettings(indexer_settings()))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_sonarr_indexer_event() {
    let expected_edit_indexer_params = EditIndexerParams {
      indexer_id: 1,
      name: Some("Test Update".to_owned()),
      enable_rss: Some(false),
      enable_automatic_search: Some(false),
      enable_interactive_search: Some(false),
      url: Some("https://localhost:9696/1/".to_owned()),
      api_key: Some("test1234".to_owned()),
      seed_ratio: Some("1.3".to_owned()),
      tag_input_string: Some("usenet, testing".to_owned()),
      priority: Some(0),
      ..EditIndexerParams::default()
    };
    let indexer_details_json = json!({
        "enableRss": true,
        "enableAutomaticSearch": true,
        "enableInteractiveSearch": true,
        "name": "Test Indexer",
        "priority": 1,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://test.com",
            },
            {
                "name": "apiKey",
                "value": "",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.2",
            },
        ],
        "tags": [1],
        "id": 1
    });
    let expected_indexer_edit_body_json = json!({
        "enableRss": false,
        "enableAutomaticSearch": false,
        "enableInteractiveSearch": false,
        "name": "Test Update",
        "priority": 0,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://localhost:9696/1/",
            },
            {
                "name": "apiKey",
                "value": "test1234",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.3",
            },
        ],
        "tags": [1, 2],
        "id": 1
    });
    let (async_details_server, app_arc, mut server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(indexer_details_json),
      None,
      SonarrEvent::GetIndexers,
      Some("/1"),
      None,
    )
    .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v3{}/1?forceSave=true",
          SonarrEvent::EditIndexer(expected_edit_indexer_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_indexer_edit_body_json))
      .create_async()
      .await;
    app_arc.lock().await.data.sonarr_data.tags_map =
      BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::EditIndexer(expected_edit_indexer_params))
      .await
      .is_ok());

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_sonarr_indexer_event_does_not_overwrite_tags_vec_if_tag_input_string_is_none(
  ) {
    let expected_edit_indexer_params = EditIndexerParams {
      indexer_id: 1,
      name: Some("Test Update".to_owned()),
      enable_rss: Some(false),
      enable_automatic_search: Some(false),
      enable_interactive_search: Some(false),
      url: Some("https://localhost:9696/1/".to_owned()),
      api_key: Some("test1234".to_owned()),
      seed_ratio: Some("1.3".to_owned()),
      tags: Some(vec![1, 2]),
      priority: Some(0),
      ..EditIndexerParams::default()
    };
    let indexer_details_json = json!({
        "enableRss": true,
        "enableAutomaticSearch": true,
        "enableInteractiveSearch": true,
        "name": "Test Indexer",
        "priority": 1,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://test.com",
            },
            {
                "name": "apiKey",
                "value": "",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.2",
            },
        ],
        "tags": [1],
        "id": 1
    });
    let expected_indexer_edit_body_json = json!({
        "enableRss": false,
        "enableAutomaticSearch": false,
        "enableInteractiveSearch": false,
        "name": "Test Update",
        "priority": 0,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://localhost:9696/1/",
            },
            {
                "name": "apiKey",
                "value": "test1234",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.3",
            },
        ],
        "tags": [1, 2],
        "id": 1
    });
    let (async_details_server, app_arc, mut server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(indexer_details_json),
      None,
      SonarrEvent::GetIndexers,
      Some("/1"),
      None,
    )
    .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v3{}/1?forceSave=true",
          SonarrEvent::EditIndexer(expected_edit_indexer_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_indexer_edit_body_json))
      .create_async()
      .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::EditIndexer(expected_edit_indexer_params))
      .await
      .is_ok());

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_sonarr_indexer_event_does_not_add_seed_ratio_when_seed_ratio_field_is_none_in_details(
  ) {
    let expected_edit_indexer_params = EditIndexerParams {
      indexer_id: 1,
      name: Some("Test Update".to_owned()),
      enable_rss: Some(false),
      enable_automatic_search: Some(false),
      enable_interactive_search: Some(false),
      url: Some("https://localhost:9696/1/".to_owned()),
      api_key: Some("test1234".to_owned()),
      seed_ratio: Some("1.3".to_owned()),
      tag_input_string: Some("usenet, testing".to_owned()),
      priority: Some(0),
      ..EditIndexerParams::default()
    };
    let indexer_details_json = json!({
        "enableRss": true,
        "enableAutomaticSearch": true,
        "enableInteractiveSearch": true,
        "name": "Test Indexer",
        "priority": 1,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://test.com",
            },
            {
                "name": "apiKey",
                "value": "",
            },
        ],
        "tags": [1],
        "id": 1
    });
    let expected_indexer_edit_body_json = json!({
        "enableRss": false,
        "enableAutomaticSearch": false,
        "enableInteractiveSearch": false,
        "name": "Test Update",
        "priority": 0,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://localhost:9696/1/",
            },
            {
                "name": "apiKey",
                "value": "test1234",
            },
        ],
        "tags": [1, 2],
        "id": 1
    });

    let (async_details_server, app_arc, mut server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(indexer_details_json),
      None,
      SonarrEvent::GetIndexers,
      Some("/1"),
      None,
    )
    .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v3{}/1?forceSave=true",
          SonarrEvent::EditIndexer(expected_edit_indexer_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_indexer_edit_body_json))
      .create_async()
      .await;
    app_arc.lock().await.data.sonarr_data.tags_map =
      BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::EditIndexer(expected_edit_indexer_params))
      .await
      .is_ok());

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_sonarr_indexer_event_populates_the_seed_ratio_value_when_seed_ratio_field_is_present_in_details(
  ) {
    let expected_edit_indexer_params = EditIndexerParams {
      indexer_id: 1,
      name: Some("Test Update".to_owned()),
      enable_rss: Some(false),
      enable_automatic_search: Some(false),
      enable_interactive_search: Some(false),
      url: Some("https://localhost:9696/1/".to_owned()),
      api_key: Some("test1234".to_owned()),
      seed_ratio: Some("1.3".to_owned()),
      tag_input_string: Some("usenet, testing".to_owned()),
      priority: Some(0),
      ..EditIndexerParams::default()
    };
    let indexer_details_json = json!({
        "enableRss": true,
        "enableAutomaticSearch": true,
        "enableInteractiveSearch": true,
        "name": "Test Indexer",
        "priority": 1,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://test.com",
            },
            {
                "name": "apiKey",
                "value": "",
            },
            {
                "name": "seedCriteria.seedRatio",
            },
        ],
        "tags": [1],
        "id": 1
    });
    let expected_indexer_edit_body_json = json!({
        "enableRss": false,
        "enableAutomaticSearch": false,
        "enableInteractiveSearch": false,
        "name": "Test Update",
        "priority": 0,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://localhost:9696/1/",
            },
            {
                "name": "apiKey",
                "value": "test1234",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.3",
            },
        ],
        "tags": [1, 2],
        "id": 1
    });

    let (async_details_server, app_arc, mut server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(indexer_details_json),
      None,
      SonarrEvent::GetIndexers,
      Some("/1"),
      None,
    )
    .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v3{}/1?forceSave=true",
          SonarrEvent::EditIndexer(expected_edit_indexer_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_indexer_edit_body_json))
      .create_async()
      .await;
    app_arc.lock().await.data.sonarr_data.tags_map =
      BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::EditIndexer(expected_edit_indexer_params))
      .await
      .is_ok());

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_sonarr_indexer_event_defaults_to_previous_values() {
    let indexer_details_json = json!({
        "enableRss": true,
        "enableAutomaticSearch": true,
        "enableInteractiveSearch": true,
        "name": "Test Indexer",
        "priority": 1,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://test.com",
            },
            {
                "name": "apiKey",
                "value": "",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.2",
            },
        ],
        "tags": [1],
        "id": 1
    });
    let edit_indexer_params = EditIndexerParams {
      indexer_id: 1,
      ..EditIndexerParams::default()
    };
    let (async_details_server, app_arc, mut server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(indexer_details_json.clone()),
      None,
      SonarrEvent::GetIndexers,
      Some("/1"),
      None,
    )
    .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v3{}/1?forceSave=true",
          SonarrEvent::EditIndexer(edit_indexer_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(indexer_details_json))
      .create_async()
      .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::EditIndexer(edit_indexer_params))
      .await
      .is_ok());

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_sonarr_indexer_event_clears_tags_when_clear_tags_is_true() {
    let indexer_details_json = json!({
        "enableRss": true,
        "enableAutomaticSearch": true,
        "enableInteractiveSearch": true,
        "name": "Test Indexer",
        "priority": 1,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://test.com",
            },
            {
                "name": "apiKey",
                "value": "",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.2",
            },
        ],
        "tags": [1, 2],
        "id": 1
    });
    let expected_edit_indexer_body = json!({
        "enableRss": true,
        "enableAutomaticSearch": true,
        "enableInteractiveSearch": true,
        "name": "Test Indexer",
        "priority": 1,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://test.com",
            },
            {
                "name": "apiKey",
                "value": "",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.2",
            },
        ],
        "tags": [],
        "id": 1
    });
    let edit_indexer_params = EditIndexerParams {
      indexer_id: 1,
      clear_tags: true,
      ..EditIndexerParams::default()
    };

    let (async_details_server, app_arc, mut server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(indexer_details_json),
      None,
      SonarrEvent::GetIndexers,
      Some("/1"),
      None,
    )
    .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v3{}/1?forceSave=true",
          SonarrEvent::EditIndexer(edit_indexer_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_edit_indexer_body))
      .create_async()
      .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::EditIndexer(edit_indexer_params))
      .await
      .is_ok());

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_series_event() {
    let mut expected_body: Value = serde_json::from_str(SERIES_JSON).unwrap();
    *expected_body.get_mut("monitored").unwrap() = json!(false);
    *expected_body.get_mut("seasonFolder").unwrap() = json!(false);
    *expected_body.get_mut("seriesType").unwrap() = json!("standard");
    *expected_body.get_mut("qualityProfileId").unwrap() = json!(1111);
    *expected_body.get_mut("languageProfileId").unwrap() = json!(1111);
    *expected_body.get_mut("path").unwrap() = json!("/nfs/Test Path");
    *expected_body.get_mut("tags").unwrap() = json!([1, 2]);
    let edit_series_params = EditSeriesParams {
      series_id: 1,
      monitored: Some(false),
      use_season_folders: Some(false),
      series_type: Some(SeriesType::Standard),
      quality_profile_id: Some(1111),
      language_profile_id: Some(1111),
      root_folder_path: Some("/nfs/Test Path".to_owned()),
      tag_input_string: Some("usenet, testing".to_owned()),
      ..EditSeriesParams::default()
    };

    let (async_details_server, app_arc, mut server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(serde_json::from_str(SERIES_JSON).unwrap()),
      None,
      SonarrEvent::GetSeriesDetails(1),
      Some("/1"),
      None,
    )
    .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v3{}/1",
          SonarrEvent::EditSeries(edit_series_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .create_async()
      .await;
    app_arc.lock().await.data.sonarr_data.tags_map =
      BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::EditSeries(edit_series_params))
      .await
      .is_ok());

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_series_event_does_not_overwrite_tag_ids_vec_when_tag_input_string_is_none(
  ) {
    let mut expected_body: Value = serde_json::from_str(SERIES_JSON).unwrap();
    *expected_body.get_mut("monitored").unwrap() = json!(false);
    *expected_body.get_mut("seasonFolder").unwrap() = json!(false);
    *expected_body.get_mut("seriesType").unwrap() = json!("standard");
    *expected_body.get_mut("qualityProfileId").unwrap() = json!(1111);
    *expected_body.get_mut("languageProfileId").unwrap() = json!(1111);
    *expected_body.get_mut("path").unwrap() = json!("/nfs/Test Path");
    *expected_body.get_mut("tags").unwrap() = json!([1, 2]);
    let edit_series_params = EditSeriesParams {
      series_id: 1,
      monitored: Some(false),
      use_season_folders: Some(false),
      series_type: Some(SeriesType::Standard),
      quality_profile_id: Some(1111),
      language_profile_id: Some(1111),
      root_folder_path: Some("/nfs/Test Path".to_owned()),
      tags: Some(vec![1, 2]),
      ..EditSeriesParams::default()
    };

    let (async_details_server, app_arc, mut server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(serde_json::from_str(SERIES_JSON).unwrap()),
      None,
      SonarrEvent::GetSeriesDetails(1),
      Some("/1"),
      None,
    )
    .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v3{}/1",
          SonarrEvent::EditSeries(edit_series_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .create_async()
      .await;
    app_arc.lock().await.data.sonarr_data.tags_map =
      BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::EditSeries(edit_series_params))
      .await
      .is_ok());

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_series_event_defaults_to_previous_values() {
    let edit_series_params = EditSeriesParams {
      series_id: 1,
      ..EditSeriesParams::default()
    };
    let expected_body: Value = serde_json::from_str(SERIES_JSON).unwrap();
    let (async_details_server, app_arc, mut server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(serde_json::from_str(SERIES_JSON).unwrap()),
      None,
      SonarrEvent::GetSeriesDetails(1),
      Some("/1"),
      None,
    )
    .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v3{}/1",
          SonarrEvent::EditSeries(edit_series_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .create_async()
      .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::EditSeries(edit_series_params))
      .await
      .is_ok());

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_series_event_returns_empty_tags_vec_when_clear_tags_is_true() {
    let mut expected_body: Value = serde_json::from_str(SERIES_JSON).unwrap();
    *expected_body.get_mut("tags").unwrap() = json!([]);

    let (async_details_server, app_arc, mut server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(serde_json::from_str(SERIES_JSON).unwrap()),
      None,
      SonarrEvent::GetSeriesDetails(1),
      Some("/1"),
      None,
    )
    .await;
    let edit_series_params = EditSeriesParams {
      series_id: 1,
      clear_tags: true,
      ..EditSeriesParams::default()
    };
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v3{}/1",
          SonarrEvent::EditSeries(edit_series_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .create_async()
      .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::EditSeries(edit_series_params))
      .await
      .is_ok());

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[rstest]
  #[tokio::test]
  async fn test_handle_get_sonarr_blocklist_event(#[values(true, false)] use_custom_sorting: bool) {
    let blocklist_json = json!({"records": [{
        "seriesId": 1007,
        "episodeIds": [42020],
        "sourceTitle": "z series",
        "languages": [{ "id": 1, "name": "English" }],
        "quality": { "quality": { "name": "Bluray-1080p" }},
        "date": "2024-02-10T07:28:45Z",
        "protocol": "usenet",
        "indexer": "NZBgeek (Prowlarr)",
        "message": "test message",
        "id": 123
    },
    {
        "seriesId": 2001,
        "episodeIds": [42018],
        "sourceTitle": "A Series",
        "languages": [{ "id": 1, "name": "English" }],
        "quality": { "quality": { "name": "Bluray-1080p" }},
        "date": "2024-02-10T07:28:45Z",
        "protocol": "usenet",
        "indexer": "NZBgeek (Prowlarr)",
        "message": "test message",
        "id": 456
    }]});
    let response: BlocklistResponse = serde_json::from_value(blocklist_json.clone()).unwrap();
    let mut expected_blocklist = vec![
      BlocklistItem {
        id: 123,
        series_id: 1007,
        series_title: Some("Z Series".into()),
        source_title: "z series".into(),
        episode_ids: vec![Number::from(42020)],
        ..blocklist_item()
      },
      BlocklistItem {
        id: 456,
        series_id: 2001,
        source_title: "A Series".into(),
        episode_ids: vec![Number::from(42018)],
        ..blocklist_item()
      },
    ];
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(blocklist_json),
      None,
      SonarrEvent::GetBlocklist,
      None,
      None,
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .set_items(vec![Series {
        id: 1007,
        title: "Z Series".into(),
        ..series()
      }]);
    app_arc.lock().await.data.sonarr_data.blocklist.sort_asc = true;
    if use_custom_sorting {
      let cmp_fn = |a: &BlocklistItem, b: &BlocklistItem| {
        a.source_title
          .to_lowercase()
          .cmp(&b.source_title.to_lowercase())
      };
      expected_blocklist.sort_by(cmp_fn);

      let blocklist_sort_option = SortOption {
        name: "Source Title",
        cmp_fn: Some(cmp_fn),
      };
      app_arc
        .lock()
        .await
        .data
        .sonarr_data
        .blocklist
        .sorting(vec![blocklist_sort_option]);
    }
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::BlocklistResponse(blocklist) = network
      .handle_sonarr_event(SonarrEvent::GetBlocklist)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.sonarr_data.blocklist.items,
        expected_blocklist
      );
      assert!(app_arc.lock().await.data.sonarr_data.blocklist.sort_asc);
      assert_eq!(blocklist, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_sonarr_blocklist_event_no_op_when_user_is_selecting_sort_options() {
    let blocklist_json = json!({"records": [{
        "seriesId": 1007,
        "episodeIds": [42020],
        "sourceTitle": "z series",
        "languages": [{ "id": 1, "name": "English" }],
        "quality": { "quality": { "name": "Bluray-1080p" }},
        "date": "2024-02-10T07:28:45Z",
        "protocol": "usenet",
        "indexer": "NZBgeek (Prowlarr)",
        "message": "test message",
        "id": 123
    },
    {
        "seriesId": 2001,
        "episodeIds": [42018],
        "sourceTitle": "A Series",
        "languages": [{ "id": 1, "name": "English" }],
        "quality": { "quality": { "name": "Bluray-1080p" }},
        "date": "2024-02-10T07:28:45Z",
        "protocol": "usenet",
        "indexer": "NZBgeek (Prowlarr)",
        "message": "test message",
        "id": 456
    }]});
    let response: BlocklistResponse = serde_json::from_value(blocklist_json.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(blocklist_json),
      None,
      SonarrEvent::GetBlocklist,
      None,
      None,
    )
    .await;
    app_arc.lock().await.data.sonarr_data.blocklist.sort_asc = true;
    app_arc
      .lock()
      .await
      .push_navigation_stack(ActiveSonarrBlock::BlocklistSortPrompt.into());
    let cmp_fn = |a: &BlocklistItem, b: &BlocklistItem| {
      a.source_title
        .to_lowercase()
        .cmp(&b.source_title.to_lowercase())
    };
    let blocklist_sort_option = SortOption {
      name: "Source Title",
      cmp_fn: Some(cmp_fn),
    };
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .blocklist
      .sorting(vec![blocklist_sort_option]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::BlocklistResponse(blocklist) = network
      .handle_sonarr_event(SonarrEvent::GetBlocklist)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert!(app_arc.lock().await.data.sonarr_data.blocklist.is_empty());
      assert!(app_arc.lock().await.data.sonarr_data.blocklist.sort_asc);
      assert_eq!(blocklist, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_sonarr_downloads_event() {
    let downloads_response_json = json!({
      "records": [{
        "title": "Test Download Title",
        "status": "downloading",
        "id": 1,
        "episodeId": 1,
        "size": 3543348019f64,
        "sizeleft": 1771674009f64,
        "outputPath": "/nfs/tv/Test show/season 1/",
        "indexer": "kickass torrents",
        "downloadClient": "transmission",
      }]
    });
    let response: DownloadsResponse =
      serde_json::from_value(downloads_response_json.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(downloads_response_json),
      None,
      SonarrEvent::GetDownloads,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::DownloadsResponse(downloads) = network
      .handle_sonarr_event(SonarrEvent::GetDownloads)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.sonarr_data.downloads.items,
        downloads_response().records
      );
      assert_eq!(downloads, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_sonarr_diskspace_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(json!([
        {
          "freeSpace": 1111,
          "totalSpace": 2222,
        },
        {
          "freeSpace": 3333,
          "totalSpace": 4444
        }
      ])),
      None,
      SonarrEvent::GetDiskSpace,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());
    let disk_space_vec = vec![
      DiskSpace {
        free_space: 1111,
        total_space: 2222,
      },
      DiskSpace {
        free_space: 3333,
        total_space: 4444,
      },
    ];

    if let SonarrSerdeable::DiskSpaces(disk_space) = network
      .handle_sonarr_event(SonarrEvent::GetDiskSpace)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.sonarr_data.disk_space_vec,
        disk_space_vec
      );
      assert_eq!(disk_space, disk_space_vec);
    }
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
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    let _ = network.handle_sonarr_event(SonarrEvent::HealthCheck).await;

    async_server.assert_async().await;
  }

  #[rstest]
  #[tokio::test]
  async fn test_handle_get_episodes_event(#[values(true, false)] use_custom_sorting: bool) {
    let episode_1 = Episode {
      title: "z test".to_owned(),
      episode_file: None,
      ..episode()
    };
    let episode_2 = Episode {
      id: 2,
      title: "A test".to_owned(),
      episode_file_id: 2,
      season_number: 2,
      episode_number: 2,
      episode_file: None,
      ..episode()
    };
    let episode_3 = Episode {
      id: 3,
      title: "A test".to_owned(),
      episode_file_id: 3,
      season_number: 1,
      episode_number: 2,
      episode_file: None,
      ..episode()
    };
    let expected_episodes = vec![episode_1.clone(), episode_2.clone(), episode_3.clone()];
    let mut expected_sorted_episodes = vec![episode_1.clone(), episode_3.clone()];
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(json!([episode_1, episode_2, episode_3])),
      None,
      SonarrEvent::GetEpisodes(1),
      None,
      Some("seriesId=1"),
    )
    .await;
    let mut season_details_modal = SeasonDetailsModal::default();
    season_details_modal.episodes.sort_asc = true;
    if use_custom_sorting {
      let cmp_fn = |a: &Episode, b: &Episode| a.title.to_lowercase().cmp(&b.title.to_lowercase());
      expected_sorted_episodes.sort_by(cmp_fn);
      let title_sort_option = SortOption {
        name: "Title",
        cmp_fn: Some(cmp_fn),
      };
      season_details_modal
        .episodes
        .sorting(vec![title_sort_option]);
    }
    app_arc.lock().await.data.sonarr_data.season_details_modal = Some(season_details_modal);
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .set_items(vec![Series {
        id: 1,
        ..Series::default()
      }]);
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .seasons
      .set_items(vec![Season {
        season_number: 1,
        ..Season::default()
      }]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::Episodes(episodes) = network
      .handle_sonarr_event(SonarrEvent::GetEpisodes(1))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc
          .lock()
          .await
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .episodes
          .items,
        expected_sorted_episodes
      );
      assert!(
        app_arc
          .lock()
          .await
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .episodes
          .sort_asc
      );
      assert_eq!(episodes, expected_episodes);
    }
  }

  #[tokio::test]
  async fn test_handle_get_episodes_event_empty_seasons_table_returns_all_episodes_by_default() {
    let episode_1 = Episode {
      title: "z test".to_owned(),
      episode_file: None,
      ..episode()
    };
    let episode_2 = Episode {
      id: 2,
      title: "A test".to_owned(),
      episode_file_id: 2,
      season_number: 2,
      episode_number: 2,
      episode_file: None,
      ..episode()
    };
    let episode_3 = Episode {
      id: 3,
      title: "A test".to_owned(),
      episode_file_id: 3,
      season_number: 1,
      episode_number: 2,
      episode_file: None,
      ..episode()
    };
    let expected_episodes = vec![episode_1.clone(), episode_2.clone(), episode_3.clone()];
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(json!([episode_1, episode_2, episode_3])),
      None,
      SonarrEvent::GetEpisodes(1),
      None,
      Some("seriesId=1"),
    )
    .await;
    let mut season_details_modal = SeasonDetailsModal::default();
    season_details_modal.episodes.sort_asc = true;
    app_arc.lock().await.data.sonarr_data.season_details_modal = Some(season_details_modal);
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .set_items(vec![Series {
        id: 1,
        ..Series::default()
      }]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::Episodes(episodes) = network
      .handle_sonarr_event(SonarrEvent::GetEpisodes(1))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc
          .lock()
          .await
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .episodes
          .items,
        expected_episodes
      );
      assert!(
        app_arc
          .lock()
          .await
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .episodes
          .sort_asc
      );
      assert_eq!(episodes, expected_episodes);
    }
  }

  #[tokio::test]
  async fn test_handle_get_episodes_event_empty_season_details_modal() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(json!([episode()])),
      None,
      SonarrEvent::GetEpisodes(1),
      None,
      Some("seriesId=1"),
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .set_items(vec![Series {
        id: 1,
        ..Series::default()
      }]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::Episodes(episodes) = network
      .handle_sonarr_event(SonarrEvent::GetEpisodes(1))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc
          .lock()
          .await
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .episodes
          .items,
        vec![episode()]
      );
      assert_eq!(episodes, vec![episode()]);
    }
  }

  #[tokio::test]
  async fn test_handle_get_episodes_event_no_op_while_user_is_selecting_sort_options_on_table() {
    let episodes_json = json!([
      {
          "id": 2,
          "seriesId": 1,
          "tvdbId": 1234,
          "episodeFileId": 2,
          "seasonNumber": 2,
          "episodeNumber": 2,
          "title": "Something cool",
          "airDateUtc": "2024-02-10T07:28:45Z",
          "overview": "Okay so this one time at band camp...",
          "hasFile": true,
          "monitored": true
      },
      {
          "id": 1,
          "seriesId": 1,
          "tvdbId": 1234,
          "episodeFileId": 1,
          "seasonNumber": 1,
          "episodeNumber": 1,
          "title": "Something cool",
          "airDateUtc": "2024-02-10T07:28:45Z",
          "overview": "Okay so this one time at band camp...",
          "hasFile": true,
          "monitored": true
      }
    ]);
    let episode_1 = Episode {
      episode_file: None,
      ..episode()
    };
    let episode_2 = Episode {
      id: 2,
      episode_file_id: 2,
      season_number: 2,
      episode_number: 2,
      episode_file: None,
      ..episode()
    };
    let mut expected_episodes = vec![episode_2.clone(), episode_1.clone()];
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(episodes_json),
      None,
      SonarrEvent::GetEpisodes(1),
      None,
      Some("seriesId=1"),
    )
    .await;
    app_arc
      .lock()
      .await
      .push_navigation_stack(ActiveSonarrBlock::EpisodesSortPrompt.into());
    let mut season_details_modal = SeasonDetailsModal::default();
    season_details_modal.episodes.sort_asc = true;
    let cmp_fn = |a: &Episode, b: &Episode| a.title.to_lowercase().cmp(&b.title.to_lowercase());
    expected_episodes.sort_by(cmp_fn);
    let title_sort_option = SortOption {
      name: "Title",
      cmp_fn: Some(cmp_fn),
    };
    season_details_modal
      .episodes
      .sorting(vec![title_sort_option]);
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .set_items(vec![Series {
        id: 1,
        ..Series::default()
      }]);
    app_arc.lock().await.data.sonarr_data.season_details_modal = Some(season_details_modal);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::Episodes(episodes) = network
      .handle_sonarr_event(SonarrEvent::GetEpisodes(1))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert!(app_arc
        .lock()
        .await
        .data
        .sonarr_data
        .season_details_modal
        .as_ref()
        .unwrap()
        .episodes
        .is_empty());
      assert!(
        app_arc
          .lock()
          .await
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .episodes
          .sort_asc
      );
      assert_eq!(episodes, expected_episodes);
    }
  }

  #[tokio::test]
  async fn test_handle_get_episode_files_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(json!([episode_file()])),
      None,
      SonarrEvent::GetEpisodeFiles(1),
      None,
      Some("seriesId=1"),
    )
    .await;
    app_arc.lock().await.data.sonarr_data.season_details_modal =
      Some(SeasonDetailsModal::default());
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .set_items(vec![Series {
        id: 1,
        ..Series::default()
      }]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::EpisodeFiles(episode_files) = network
      .handle_sonarr_event(SonarrEvent::GetEpisodeFiles(1))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc
          .lock()
          .await
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .episode_files
          .items,
        vec![episode_file()]
      );
      assert_eq!(episode_files, vec![episode_file()]);
    }
  }

  #[tokio::test]
  async fn test_handle_get_episode_files_event_empty_season_details_modal() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(json!([episode_file()])),
      None,
      SonarrEvent::GetEpisodeFiles(1),
      None,
      Some("seriesId=1"),
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .set_items(vec![Series {
        id: 1,
        ..Series::default()
      }]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::EpisodeFiles(episode_files) = network
      .handle_sonarr_event(SonarrEvent::GetEpisodeFiles(1))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert!(app_arc
        .lock()
        .await
        .data
        .sonarr_data
        .season_details_modal
        .is_some());
      assert_eq!(
        app_arc
          .lock()
          .await
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .episode_files
          .items,
        vec![episode_file()]
      );
      assert_eq!(episode_files, vec![episode_file()]);
    }
  }

  #[tokio::test]
  async fn test_handle_get_sonarr_host_config_event() {
    let host_config_response = json!({
      "bindAddress": "*",
      "port": 7878,
      "urlBase": "some.test.site/sonarr",
      "instanceName": "Sonarr",
      "applicationUrl": "https://some.test.site:7878/sonarr",
      "enableSsl": true,
      "sslPort": 9898,
      "sslCertPath": "/app/sonarr.pfx",
      "sslCertPassword": "test"
    });
    let response: HostConfig = serde_json::from_value(host_config_response.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(host_config_response),
      None,
      SonarrEvent::GetHostConfig,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::HostConfig(host_config) = network
      .handle_sonarr_event(SonarrEvent::GetHostConfig)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(host_config, response);
    }
  }

  #[rstest]
  #[tokio::test]
  async fn test_handle_get_sonarr_history_event(#[values(true, false)] use_custom_sorting: bool) {
    let history_json = json!({"records": [{
      "id": 123,
      "sourceTitle": "z episode",
      "episodeId": 1007,
      "quality": { "quality": { "name": "Bluray-1080p" } },
      "languages": [{ "id": 1, "name": "English" }],
      "date": "2024-02-10T07:28:45Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/series/Coolness/something.cool.mkv",
        "importedPath": "/nfs/tv/Coolness/Season 1/Coolness - S01E01 - Something Cool Bluray-1080p.mkv"
      }
    },
    {
      "id": 456,
      "sourceTitle": "A Episode",
      "episodeId": 2001,
      "quality": { "quality": { "name": "Bluray-1080p" } },
      "languages": [{ "id": 1, "name": "English" }],
      "date": "2024-02-10T07:28:45Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/series/Coolness/something.cool.mkv",
        "importedPath": "/nfs/tv/Coolness/Season 1/Coolness - S01E01 - Something Cool Bluray-1080p.mkv"
      }
    }]});
    let response: SonarrHistoryWrapper = serde_json::from_value(history_json.clone()).unwrap();
    let mut expected_history_items = vec![
      SonarrHistoryItem {
        id: 123,
        episode_id: 1007,
        source_title: "z episode".into(),
        ..history_item()
      },
      SonarrHistoryItem {
        id: 456,
        episode_id: 2001,
        source_title: "A Episode".into(),
        ..history_item()
      },
    ];
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(history_json),
      None,
      SonarrEvent::GetHistory(500),
      None,
      Some("pageSize=500&sortDirection=descending&sortKey=date"),
    )
    .await;
    app_arc.lock().await.data.sonarr_data.history.sort_asc = true;
    if use_custom_sorting {
      let cmp_fn = |a: &SonarrHistoryItem, b: &SonarrHistoryItem| {
        a.source_title
          .text
          .to_lowercase()
          .cmp(&b.source_title.text.to_lowercase())
      };
      expected_history_items.sort_by(cmp_fn);

      let history_sort_option = SortOption {
        name: "Source Title",
        cmp_fn: Some(cmp_fn),
      };
      app_arc
        .lock()
        .await
        .data
        .sonarr_data
        .history
        .sorting(vec![history_sort_option]);
    }
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::SonarrHistoryWrapper(history) = network
      .handle_sonarr_event(SonarrEvent::GetHistory(500))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.sonarr_data.history.items,
        expected_history_items
      );
      assert!(app_arc.lock().await.data.sonarr_data.history.sort_asc);
      assert_eq!(history, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_sonarr_history_event_no_op_when_user_is_selecting_sort_options() {
    let history_json = json!({"records": [{
      "id": 123,
      "sourceTitle": "z episode",
      "episodeId": 1007,
      "quality": { "quality": { "name": "Bluray-1080p" } },
      "languages": [{ "id": 1, "name": "English" }],
      "date": "2024-02-10T07:28:45Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/series/Coolness/something.cool.mkv",
        "importedPath": "/nfs/tv/Coolness/Season 1/Coolness - S01E01 - Something Cool Bluray-1080p.mkv"
      }
    },
    {
      "id": 456,
      "sourceTitle": "A Episode",
      "episodeId": 2001,
      "quality": { "quality": { "name": "Bluray-1080p" } },
      "languages": [{ "id": 1, "name": "English" }],
      "date": "2024-02-10T07:28:45Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/series/Coolness/something.cool.mkv",
        "importedPath": "/nfs/tv/Coolness/Season 1/Coolness - S01E01 - Something Cool Bluray-1080p.mkv"
      }
    }]});
    let response: SonarrHistoryWrapper = serde_json::from_value(history_json.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(history_json),
      None,
      SonarrEvent::GetHistory(500),
      None,
      Some("pageSize=500&sortDirection=descending&sortKey=date"),
    )
    .await;
    app_arc.lock().await.data.sonarr_data.history.sort_asc = true;
    app_arc
      .lock()
      .await
      .push_navigation_stack(ActiveSonarrBlock::HistorySortPrompt.into());
    let cmp_fn = |a: &SonarrHistoryItem, b: &SonarrHistoryItem| {
      a.source_title
        .text
        .to_lowercase()
        .cmp(&b.source_title.text.to_lowercase())
    };
    let history_sort_option = SortOption {
      name: "Source Title",
      cmp_fn: Some(cmp_fn),
    };
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .history
      .sorting(vec![history_sort_option]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::SonarrHistoryWrapper(history) = network
      .handle_sonarr_event(SonarrEvent::GetHistory(500))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert!(app_arc.lock().await.data.sonarr_data.history.is_empty());
      assert!(app_arc.lock().await.data.sonarr_data.history.sort_asc);
      assert_eq!(history, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_sonarr_indexers_event() {
    let indexers_response_json = json!([{
        "enableRss": true,
        "enableAutomaticSearch": true,
        "enableInteractiveSearch": true,
        "supportsRss": true,
        "supportsSearch": true,
        "protocol": "torrent",
        "priority": 25,
        "downloadClientId": 0,
        "name": "Test Indexer",
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://test.com",
            },
            {
                "name": "apiKey",
                "value": "",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.2",
            },
        ],
        "implementationName": "Torznab",
        "implementation": "Torznab",
        "configContract": "TorznabSettings",
        "tags": [1],
        "id": 1
    }]);
    let response: Vec<Indexer> = serde_json::from_value(indexers_response_json.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(indexers_response_json),
      None,
      SonarrEvent::GetIndexers,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::Indexers(indexers) = network
      .handle_sonarr_event(SonarrEvent::GetIndexers)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.sonarr_data.indexers.items,
        vec![indexer()]
      );
      assert_eq!(indexers, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_episode_details_event() {
    let response: Episode = serde_json::from_str(EPISODE_JSON).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(serde_json::from_str(EPISODE_JSON).unwrap()),
      None,
      SonarrEvent::GetEpisodeDetails(1),
      Some("/1"),
      None,
    )
    .await;
    let mut episode_details_modal = EpisodeDetailsModal::default();
    episode_details_modal.episode_details_tabs.next();
    let mut season_details_modal = SeasonDetailsModal::default();
    season_details_modal.episodes.set_items(vec![episode()]);
    season_details_modal.episode_details_modal = Some(episode_details_modal);
    app_arc.lock().await.data.sonarr_data.season_details_modal = Some(season_details_modal);
    app_arc
      .lock()
      .await
      .push_navigation_stack(ActiveSonarrBlock::EpisodeDetails.into());
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::Episode(episode) = network
      .handle_sonarr_event(SonarrEvent::GetEpisodeDetails(1))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert!(app_arc
        .lock()
        .await
        .data
        .sonarr_data
        .season_details_modal
        .as_ref()
        .unwrap()
        .episode_details_modal
        .is_some());
      assert_eq!(
        app_arc
          .lock()
          .await
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .episode_details_modal
          .as_ref()
          .unwrap()
          .episode_details_tabs
          .get_active_route(),
        ActiveSonarrBlock::EpisodeHistory.into()
      );
      assert_eq!(episode, response);

      let app = app_arc.lock().await;
      let episode_details_modal = app
        .data
        .sonarr_data
        .season_details_modal
        .as_ref()
        .unwrap()
        .episode_details_modal
        .as_ref()
        .unwrap();
      assert_str_eq!(
        episode_details_modal.episode_details.get_text(),
        formatdoc!(
          "Title: Something cool
          Season: 1
          Episode Number: 1
          Air Date: 2024-02-10 07:28:45 UTC
          Status: Downloaded
          Description: Okay so this one time at band camp..."
        )
      );
      assert_str_eq!(
        episode_details_modal.file_details,
        formatdoc!(
          "Relative Path: /season 1/episode 1.mkv
          Absolute Path: /nfs/tv/series/season 1/episode 1.mkv
          Size: 3.30 GB
          Language: English
          Date Added: 2024-02-10 07:28:45 UTC"
        )
      );
      assert_str_eq!(
        episode_details_modal.audio_details,
        formatdoc!(
          "Bitrate: 0
          Channels: 7.1
          Codec: AAC
          Languages: eng
          Stream Count: 1"
        )
      );
      assert_str_eq!(
        episode_details_modal.video_details,
        formatdoc!(
          "Bit Depth: 10
          Bitrate: 0
          Codec: x265
          FPS: 23.976
          Resolution: 1920x1080
          Scan Type: Progressive
          Runtime: 23:51
          Subtitles: English"
        )
      );
    }
  }

  #[tokio::test]
  async fn test_handle_get_episode_details_event_empty_episode_details_modal() {
    let response: Episode = serde_json::from_str(EPISODE_JSON).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(serde_json::from_str(EPISODE_JSON).unwrap()),
      None,
      SonarrEvent::GetEpisodeDetails(1),
      Some("/1"),
      None,
    )
    .await;
    let mut season_details_modal = SeasonDetailsModal::default();
    season_details_modal.episodes.set_items(vec![episode()]);
    app_arc.lock().await.data.sonarr_data.season_details_modal = Some(season_details_modal);
    app_arc
      .lock()
      .await
      .push_navigation_stack(ActiveSonarrBlock::EpisodeDetails.into());
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::Episode(episode) = network
      .handle_sonarr_event(SonarrEvent::GetEpisodeDetails(1))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert!(app_arc
        .lock()
        .await
        .data
        .sonarr_data
        .season_details_modal
        .as_ref()
        .unwrap()
        .episode_details_modal
        .is_some());
      assert_eq!(episode, response);

      let app = app_arc.lock().await;
      let episode_details_modal = app
        .data
        .sonarr_data
        .season_details_modal
        .as_ref()
        .unwrap()
        .episode_details_modal
        .as_ref()
        .unwrap();
      assert_str_eq!(
        episode_details_modal.episode_details.get_text(),
        formatdoc!(
          "Title: Something cool
          Season: 1
          Episode Number: 1
          Air Date: 2024-02-10 07:28:45 UTC
          Status: Downloaded
          Description: Okay so this one time at band camp..."
        )
      );
      assert_str_eq!(
        episode_details_modal.file_details,
        formatdoc!(
          "Relative Path: /season 1/episode 1.mkv
          Absolute Path: /nfs/tv/series/season 1/episode 1.mkv
          Size: 3.30 GB
          Language: English
          Date Added: 2024-02-10 07:28:45 UTC"
        )
      );
      assert_str_eq!(
        episode_details_modal.audio_details,
        formatdoc!(
          "Bitrate: 0
          Channels: 7.1
          Codec: AAC
          Languages: eng
          Stream Count: 1"
        )
      );
      assert_str_eq!(
        episode_details_modal.video_details,
        formatdoc!(
          "Bit Depth: 10
          Bitrate: 0
          Codec: x265
          FPS: 23.976
          Resolution: 1920x1080
          Scan Type: Progressive
          Runtime: 23:51
          Subtitles: English"
        )
      );
    }
  }

  #[tokio::test]
  async fn test_handle_get_sonarr_episode_history_event() {
    let history_json = json!({"records": [{
      "id": 123,
      "sourceTitle": "z episode",
      "episodeId": 1007,
      "quality": { "quality": { "name": "Bluray-1080p" } },
      "languages": [{ "id": 1, "name": "English" }],
      "date": "2024-02-10T07:28:45Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/series/Coolness/something.cool.mkv",
        "importedPath": "/nfs/tv/Coolness/Season 1/Coolness - S01E01 - Something Cool Bluray-1080p.mkv"
      }
    },
    {
      "id": 456,
      "sourceTitle": "A Episode",
      "episodeId": 2001,
      "quality": { "quality": { "name": "Bluray-1080p" } },
      "languages": [{ "id": 1, "name": "English" }],
      "date": "2024-02-10T07:28:45Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/series/Coolness/something.cool.mkv",
        "importedPath": "/nfs/tv/Coolness/Season 1/Coolness - S01E01 - Something Cool Bluray-1080p.mkv"
      }
    }]});
    let response: SonarrHistoryWrapper = serde_json::from_value(history_json.clone()).unwrap();
    let expected_history_items = vec![
      SonarrHistoryItem {
        id: 123,
        episode_id: 1007,
        source_title: "z episode".into(),
        ..history_item()
      },
      SonarrHistoryItem {
        id: 456,
        episode_id: 2001,
        source_title: "A Episode".into(),
        ..history_item()
      },
    ];
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(history_json),
      None,
      SonarrEvent::GetEpisodeHistory(1),
      None,
      Some("episodeId=1&pageSize=1000&sortDirection=descending&sortKey=date"),
    )
    .await;
    app_arc.lock().await.data.sonarr_data.season_details_modal =
      Some(SeasonDetailsModal::default());
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .season_details_modal
      .as_mut()
      .unwrap()
      .episodes
      .set_items(vec![episode()]);
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .season_details_modal
      .as_mut()
      .unwrap()
      .episode_details_modal = Some(EpisodeDetailsModal::default());
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .season_details_modal
      .as_mut()
      .unwrap()
      .episode_details_modal
      .as_mut()
      .unwrap()
      .episode_history
      .sort_asc = true;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::SonarrHistoryWrapper(history) = network
      .handle_sonarr_event(SonarrEvent::GetEpisodeHistory(1))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc
          .lock()
          .await
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .episode_details_modal
          .as_ref()
          .unwrap()
          .episode_history
          .items,
        expected_history_items
      );
      assert!(
        app_arc
          .lock()
          .await
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .episode_details_modal
          .as_ref()
          .unwrap()
          .episode_history
          .sort_asc
      );
      assert_eq!(history, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_sonarr_episode_history_event_empty_episode_details_modal() {
    let history_json = json!({"records": [{
      "id": 123,
      "sourceTitle": "z episode",
      "episodeId": 1007,
      "quality": { "quality": { "name": "Bluray-1080p" } },
      "languages": [{ "id": 1, "name": "English" }],
      "date": "2024-02-10T07:28:45Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/series/Coolness/something.cool.mkv",
        "importedPath": "/nfs/tv/Coolness/Season 1/Coolness - S01E01 - Something Cool Bluray-1080p.mkv"
      }
    },
    {
      "id": 456,
      "sourceTitle": "A Episode",
      "episodeId": 2001,
      "quality": { "quality": { "name": "Bluray-1080p" } },
      "languages": [{ "id": 1, "name": "English" }],
      "date": "2024-02-10T07:28:45Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/series/Coolness/something.cool.mkv",
        "importedPath": "/nfs/tv/Coolness/Season 1/Coolness - S01E01 - Something Cool Bluray-1080p.mkv"
      }
    }]});
    let response: SonarrHistoryWrapper = serde_json::from_value(history_json.clone()).unwrap();
    let expected_history_items = vec![
      SonarrHistoryItem {
        id: 123,
        episode_id: 1007,
        source_title: "z episode".into(),
        ..history_item()
      },
      SonarrHistoryItem {
        id: 456,
        episode_id: 2001,
        source_title: "A Episode".into(),
        ..history_item()
      },
    ];
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(history_json),
      None,
      SonarrEvent::GetEpisodeHistory(1),
      None,
      Some("episodeId=1&pageSize=1000&sortDirection=descending&sortKey=date"),
    )
    .await;
    app_arc.lock().await.data.sonarr_data.season_details_modal =
      Some(SeasonDetailsModal::default());
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .season_details_modal
      .as_mut()
      .unwrap()
      .episodes
      .set_items(vec![episode()]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::SonarrHistoryWrapper(history) = network
      .handle_sonarr_event(SonarrEvent::GetEpisodeHistory(1))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc
          .lock()
          .await
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .episode_details_modal
          .as_ref()
          .unwrap()
          .episode_history
          .items,
        expected_history_items
      );
      assert!(
        !app_arc
          .lock()
          .await
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .episode_details_modal
          .as_ref()
          .unwrap()
          .episode_history
          .sort_asc
      );
      assert_eq!(history, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_sonarr_episode_history_event_empty_season_details_modal() {
    let history_json = json!({"records": [{
      "id": 123,
      "sourceTitle": "z episode",
      "episodeId": 1007,
      "quality": { "quality": { "name": "Bluray-1080p" } },
      "languages": [{ "id": 1, "name": "English" }],
      "date": "2024-02-10T07:28:45Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/series/Coolness/something.cool.mkv",
        "importedPath": "/nfs/tv/Coolness/Season 1/Coolness - S01E01 - Something Cool Bluray-1080p.mkv"
      }
    },
    {
      "id": 456,
      "sourceTitle": "A Episode",
      "episodeId": 2001,
      "quality": { "quality": { "name": "Bluray-1080p" } },
      "languages": [{ "id": 1, "name": "English" }],
      "date": "2024-02-10T07:28:45Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/series/Coolness/something.cool.mkv",
        "importedPath": "/nfs/tv/Coolness/Season 1/Coolness - S01E01 - Something Cool Bluray-1080p.mkv"
      }
    }]});
    let response: SonarrHistoryWrapper = serde_json::from_value(history_json.clone()).unwrap();
    let expected_history_items = vec![
      SonarrHistoryItem {
        id: 123,
        episode_id: 1007,
        source_title: "z episode".into(),
        ..history_item()
      },
      SonarrHistoryItem {
        id: 456,
        episode_id: 2001,
        source_title: "A Episode".into(),
        ..history_item()
      },
    ];
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(history_json),
      None,
      SonarrEvent::GetEpisodeHistory(1),
      None,
      Some("episodeId=1&pageSize=1000&sortDirection=descending&sortKey=date"),
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::SonarrHistoryWrapper(history) = network
      .handle_sonarr_event(SonarrEvent::GetEpisodeHistory(1))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc
          .lock()
          .await
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .episode_details_modal
          .as_ref()
          .unwrap()
          .episode_history
          .items,
        expected_history_items
      );
      assert!(
        !app_arc
          .lock()
          .await
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .episode_details_modal
          .as_ref()
          .unwrap()
          .episode_history
          .sort_asc
      );
      assert_eq!(history, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_episode_details_event_season_details_modal_not_required_in_cli_mode() {
    let response: Episode = serde_json::from_str(EPISODE_JSON).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(serde_json::from_str(EPISODE_JSON).unwrap()),
      None,
      SonarrEvent::GetEpisodeDetails(1),
      Some("/1"),
      None,
    )
    .await;
    app_arc.lock().await.cli_mode = true;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::Episode(episode) = network
      .handle_sonarr_event(SonarrEvent::GetEpisodeDetails(1))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(episode, response);
    }
  }

  #[tokio::test]
  #[should_panic(expected = "Season details modal is empty")]
  async fn test_handle_get_episode_details_event_requires_season_details_modal_to_be_some_when_in_tui_mode(
  ) {
    let (_async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(serde_json::from_str(EPISODE_JSON).unwrap()),
      None,
      SonarrEvent::GetEpisodeDetails(1),
      Some("/1"),
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    network
      .handle_sonarr_event(SonarrEvent::GetEpisodeDetails(1))
      .await
      .unwrap();
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
  async fn test_handle_get_sonarr_logs_event() {
    let expected_logs = vec![
      HorizontallyScrollableText::from(
        "2023-05-20 21:29:16 UTC|FATAL|SonarrError|Some.Big.Bad.Exception|test exception",
      ),
      HorizontallyScrollableText::from("2023-05-20 21:29:16 UTC|INFO|TestLogger|test message"),
    ];
    let logs_response_json = json!({
      "page": 1,
      "pageSize": 500,
      "sortKey": "time",
      "sortDirection": "descending",
      "totalRecords": 2,
      "records": [
          {
              "time": "2023-05-20T21:29:16Z",
              "level": "info",
              "logger": "TestLogger",
              "message": "test message",
              "id": 1
          },
          {
              "time": "2023-05-20T21:29:16Z",
              "level": "fatal",
              "logger": "SonarrError",
              "exception": "test exception",
              "exceptionType": "Some.Big.Bad.Exception",
              "id": 2
          }
        ]
    });
    let response: LogResponse = serde_json::from_value(logs_response_json.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(logs_response_json),
      None,
      SonarrEvent::GetLogs(500),
      None,
      Some("pageSize=500&sortDirection=descending&sortKey=time"),
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::LogResponse(logs) = network
      .handle_sonarr_event(SonarrEvent::GetLogs(500))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.sonarr_data.logs.items,
        expected_logs
      );
      assert!(app_arc
        .lock()
        .await
        .data
        .sonarr_data
        .logs
        .current_selection()
        .text
        .contains("INFO"));
      assert_eq!(logs, response);
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
  async fn test_handle_get_queued_sonarr_events_event() {
    let queued_events_json = json!([{
        "name": "RefreshMonitoredDownloads",
        "commandName": "Refresh Monitored Downloads",
        "status": "completed",
        "queued": "2023-05-20T21:29:16Z",
        "started": "2023-05-20T21:29:16Z",
        "ended": "2023-05-20T21:29:16Z",
        "duration": "00:00:00.5111547",
        "trigger": "scheduled",
    }]);
    let response: Vec<QueueEvent> = serde_json::from_value(queued_events_json.clone()).unwrap();
    let timestamp = DateTime::from(DateTime::parse_from_rfc3339("2023-05-20T21:29:16Z").unwrap());
    let expected_event = QueueEvent {
      name: "RefreshMonitoredDownloads".to_owned(),
      command_name: "Refresh Monitored Downloads".to_owned(),
      status: "completed".to_owned(),
      queued: timestamp,
      started: Some(timestamp),
      ended: Some(timestamp),
      duration: Some("00:00:00.5111547".to_owned()),
      trigger: "scheduled".to_owned(),
    };

    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(queued_events_json),
      None,
      SonarrEvent::GetQueuedEvents,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::QueueEvents(events) = network
      .handle_sonarr_event(SonarrEvent::GetQueuedEvents)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.sonarr_data.queued_events.items,
        vec![expected_event]
      );
      assert_eq!(events, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_sonarr_root_folders_event() {
    let root_folder_json = json!([{
      "id": 1,
      "path": "/nfs",
      "accessible": true,
      "freeSpace": 219902325555200u64,
    }]);
    let response: Vec<RootFolder> = serde_json::from_value(root_folder_json.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(root_folder_json),
      None,
      SonarrEvent::GetRootFolders,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::RootFolders(root_folders) = network
      .handle_sonarr_event(SonarrEvent::GetRootFolders)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.sonarr_data.root_folders.items,
        vec![root_folder()]
      );
      assert_eq!(root_folders, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_episode_releases_event() {
    let release_json = json!([{
      "guid": "1234",
      "protocol": "torrent",
      "age": 1,
      "title": "Test Release",
      "indexer": "kickass torrents",
      "indexerId": 2,
      "size": 1234,
      "rejected": true,
      "rejections": [ "Unknown quality profile", "Release is already mapped" ],
      "seeders": 2,
      "leechers": 1,
      "languages": [ { "id": 1, "name": "English" } ],
      "quality": { "quality": { "name": "Bluray-1080p" }}
    }]);
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(release_json),
      None,
      SonarrEvent::GetEpisodeReleases(1),
      None,
      Some("episodeId=1"),
    )
    .await;
    let mut season_details_modal = SeasonDetailsModal::default();
    season_details_modal.episodes.set_items(vec![episode()]);
    app_arc.lock().await.data.sonarr_data.season_details_modal = Some(season_details_modal);
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .season_details_modal
      .as_mut()
      .unwrap()
      .episode_details_modal = Some(EpisodeDetailsModal::default());
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::Releases(releases_vec) = network
      .handle_sonarr_event(SonarrEvent::GetEpisodeReleases(1))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc
          .lock()
          .await
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .episode_details_modal
          .as_ref()
          .unwrap()
          .episode_releases
          .items,
        vec![release()]
      );
      assert_eq!(releases_vec, vec![release()]);
    }
  }

  #[tokio::test]
  async fn test_handle_get_episode_releases_event_empty_episode_details_modal() {
    let release_json = json!([{
      "guid": "1234",
      "protocol": "torrent",
      "age": 1,
      "title": "Test Release",
      "indexer": "kickass torrents",
      "indexerId": 2,
      "size": 1234,
      "rejected": true,
      "rejections": [ "Unknown quality profile", "Release is already mapped" ],
      "seeders": 2,
      "leechers": 1,
      "languages": [ { "id": 1, "name": "English" } ],
      "quality": { "quality": { "name": "Bluray-1080p" }}
    }]);
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(release_json),
      None,
      SonarrEvent::GetEpisodeReleases(1),
      None,
      Some("episodeId=1"),
    )
    .await;
    let mut season_details_modal = SeasonDetailsModal::default();
    season_details_modal.episodes.set_items(vec![episode()]);
    app_arc.lock().await.data.sonarr_data.season_details_modal = Some(season_details_modal);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::Releases(releases_vec) = network
      .handle_sonarr_event(SonarrEvent::GetEpisodeReleases(1))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc
          .lock()
          .await
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .episode_details_modal
          .as_ref()
          .unwrap()
          .episode_releases
          .items,
        vec![release()]
      );
      assert_eq!(releases_vec, vec![release()]);
    }
  }

  #[tokio::test]
  async fn test_handle_get_sonarr_season_history_event() {
    let history_json = json!([{
      "id": 123,
      "sourceTitle": "z episode",
      "episodeId": 1007,
      "quality": { "quality": { "name": "Bluray-1080p" } },
      "languages": [{ "id": 1, "name": "English" }],
      "date": "2024-02-10T07:28:45Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/series/Coolness/something.cool.mkv",
        "importedPath": "/nfs/tv/Coolness/Season 1/Coolness - S01E01 - Something Cool Bluray-1080p.mkv"
      }
    },
    {
      "id": 456,
      "sourceTitle": "A Episode",
      "episodeId": 2001,
      "quality": { "quality": { "name": "Bluray-1080p" } },
      "languages": [{ "id": 1, "name": "English" }],
      "date": "2024-02-10T07:28:45Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/series/Coolness/something.cool.mkv",
        "importedPath": "/nfs/tv/Coolness/Season 1/Coolness - S01E01 - Something Cool Bluray-1080p.mkv"
      }
    }]);
    let response: Vec<SonarrHistoryItem> = serde_json::from_value(history_json.clone()).unwrap();
    let expected_history_items = vec![
      SonarrHistoryItem {
        id: 123,
        episode_id: 1007,
        source_title: "z episode".into(),
        ..history_item()
      },
      SonarrHistoryItem {
        id: 456,
        episode_id: 2001,
        source_title: "A Episode".into(),
        ..history_item()
      },
    ];
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(history_json),
      None,
      SonarrEvent::GetSeasonHistory((1, 1)),
      None,
      Some("seriesId=1&seasonNumber=1"),
    )
    .await;
    app_arc.lock().await.data.sonarr_data.season_details_modal =
      Some(SeasonDetailsModal::default());
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .set_items(vec![series()]);
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .seasons
      .set_items(vec![season()]);
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .season_details_modal
      .as_mut()
      .unwrap()
      .season_history
      .sort_asc = true;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::SonarrHistoryItems(history) = network
      .handle_sonarr_event(SonarrEvent::GetSeasonHistory((1, 1)))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc
          .lock()
          .await
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .season_history
          .items,
        expected_history_items
      );
      assert!(
        app_arc
          .lock()
          .await
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .season_history
          .sort_asc
      );
      assert_eq!(history, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_sonarr_season_history_event_empty_season_details_modal() {
    let history_json = json!([{
      "id": 123,
      "sourceTitle": "z episode",
      "episodeId": 1007,
      "quality": { "quality": { "name": "Bluray-1080p" } },
      "languages": [{ "id": 1, "name": "English" }],
      "date": "2024-02-10T07:28:45Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/series/Coolness/something.cool.mkv",
        "importedPath": "/nfs/tv/Coolness/Season 1/Coolness - S01E01 - Something Cool Bluray-1080p.mkv"
      }
    },
    {
      "id": 456,
      "sourceTitle": "A Episode",
      "episodeId": 2001,
      "quality": { "quality": { "name": "Bluray-1080p" } },
      "languages": [{ "id": 1, "name": "English" }],
      "date": "2024-02-10T07:28:45Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/series/Coolness/something.cool.mkv",
        "importedPath": "/nfs/tv/Coolness/Season 1/Coolness - S01E01 - Something Cool Bluray-1080p.mkv"
      }
    }]);
    let response: Vec<SonarrHistoryItem> = serde_json::from_value(history_json.clone()).unwrap();
    let expected_history_items = vec![
      SonarrHistoryItem {
        id: 123,
        episode_id: 1007,
        source_title: "z episode".into(),
        ..history_item()
      },
      SonarrHistoryItem {
        id: 456,
        episode_id: 2001,
        source_title: "A Episode".into(),
        ..history_item()
      },
    ];
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(history_json),
      None,
      SonarrEvent::GetSeasonHistory((1, 1)),
      None,
      Some("seriesId=1&seasonNumber=1"),
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .set_items(vec![series()]);
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .seasons
      .set_items(vec![season()]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::SonarrHistoryItems(history) = network
      .handle_sonarr_event(SonarrEvent::GetSeasonHistory((1, 1)))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert!(app_arc
        .lock()
        .await
        .data
        .sonarr_data
        .season_details_modal
        .is_some());
      assert_eq!(
        app_arc
          .lock()
          .await
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .season_history
          .items,
        expected_history_items
      );
      assert!(
        !app_arc
          .lock()
          .await
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .season_history
          .sort_asc
      );
      assert_eq!(history, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_season_releases_event() {
    let release_json = json!([
      {
        "guid": "1234",
        "protocol": "torrent",
        "age": 1,
        "title": "Test Release",
        "indexer": "kickass torrents",
        "indexerId": 2,
        "size": 1234,
        "rejected": true,
        "rejections": [ "Unknown quality profile", "Release is already mapped" ],
        "seeders": 2,
        "leechers": 1,
        "languages": [ { "id": 1, "name": "English" } ],
        "quality": { "quality": { "name": "Bluray-1080p" }},
        "fullSeason": true
      },
      {
        "guid": "4567",
        "protocol": "torrent",
        "age": 1,
        "title": "Test Release",
        "indexer": "kickass torrents",
        "indexerId": 2,
        "size": 1234,
        "rejected": true,
        "rejections": [ "Unknown quality profile", "Release is already mapped" ],
        "seeders": 2,
        "leechers": 1,
        "languages": [ { "id": 1, "name": "English" } ],
        "quality": { "quality": { "name": "Bluray-1080p" }},
      }
    ]);
    let expected_filtered_sonarr_release = SonarrRelease {
      full_season: true,
      ..release()
    };
    let expected_raw_sonarr_releases = vec![
      SonarrRelease {
        full_season: true,
        ..release()
      },
      SonarrRelease {
        guid: "4567".to_owned(),
        ..release()
      },
    ];
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(release_json),
      None,
      SonarrEvent::GetSeasonReleases((1, 1)),
      None,
      Some("seriesId=1&seasonNumber=1"),
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .set_items(vec![series()]);
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .seasons
      .set_items(vec![season()]);
    app_arc.lock().await.data.sonarr_data.season_details_modal =
      Some(SeasonDetailsModal::default());
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::Releases(releases_vec) = network
      .handle_sonarr_event(SonarrEvent::GetSeasonReleases((1, 1)))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc
          .lock()
          .await
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .season_releases
          .items,
        vec![expected_filtered_sonarr_release]
      );
      assert_eq!(releases_vec, expected_raw_sonarr_releases);
    }
  }

  #[tokio::test]
  async fn test_handle_get_season_releases_event_empty_season_details_modal() {
    let release_json = json!([
      {
        "guid": "1234",
        "protocol": "torrent",
        "age": 1,
        "title": "Test Release",
        "indexer": "kickass torrents",
        "indexerId": 2,
        "size": 1234,
        "rejected": true,
        "rejections": [ "Unknown quality profile", "Release is already mapped" ],
        "seeders": 2,
        "leechers": 1,
        "languages": [ { "id": 1, "name": "English" } ],
        "quality": { "quality": { "name": "Bluray-1080p" }},
        "fullSeason": true
      },
      {
        "guid": "4567",
        "protocol": "usenet",
        "age": 1,
        "title": "Test Release",
        "indexer": "kickass torrents",
        "indexerId": 2,
        "size": 1234,
        "rejected": true,
        "rejections": [ "Unknown quality profile", "Release is already mapped" ],
        "seeders": 2,
        "leechers": 1,
        "languages": [ { "id": 1, "name": "English" } ],
        "quality": { "quality": { "name": "Bluray-1080p" }},
      }
    ]);
    let expected_sonarr_release = SonarrRelease {
      full_season: true,
      ..release()
    };
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(release_json),
      None,
      SonarrEvent::GetSeasonReleases((1, 1)),
      None,
      Some("seriesId=1&seasonNumber=1"),
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .set_items(vec![series()]);
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .seasons
      .set_items(vec![season()]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::GetSeasonReleases((1, 1)))
      .await
      .is_ok());

    async_server.assert_async().await;
    assert_eq!(
      app_arc
        .lock()
        .await
        .data
        .sonarr_data
        .season_details_modal
        .as_ref()
        .unwrap()
        .season_releases
        .items,
      vec![expected_sonarr_release]
    );
  }

  #[rstest]
  #[tokio::test]
  async fn test_handle_list_series_event(#[values(true, false)] use_custom_sorting: bool) {
    let mut series_1: Value = serde_json::from_str(SERIES_JSON).unwrap();
    let mut series_2: Value = serde_json::from_str(SERIES_JSON).unwrap();
    *series_1.get_mut("id").unwrap() = json!(1);
    *series_1.get_mut("title").unwrap() = json!("z test");
    *series_2.get_mut("id").unwrap() = json!(2);
    *series_2.get_mut("title").unwrap() = json!("A test");
    let expected_series = vec![
      Series {
        id: 1,
        title: "z test".into(),
        ..series()
      },
      Series {
        id: 2,
        title: "A test".into(),
        ..series()
      },
    ];
    let mut expected_sorted_series = vec![
      Series {
        id: 1,
        title: "z test".into(),
        ..series()
      },
      Series {
        id: 2,
        title: "A test".into(),
        ..series()
      },
    ];
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(json!([series_1, series_2])),
      None,
      SonarrEvent::ListSeries,
      None,
      None,
    )
    .await;
    app_arc.lock().await.data.sonarr_data.series.sort_asc = true;
    if use_custom_sorting {
      let cmp_fn = |a: &Series, b: &Series| {
        a.title
          .text
          .to_lowercase()
          .cmp(&b.title.text.to_lowercase())
      };
      expected_sorted_series.sort_by(cmp_fn);
      let title_sort_option = SortOption {
        name: "Title",
        cmp_fn: Some(cmp_fn),
      };
      app_arc
        .lock()
        .await
        .data
        .sonarr_data
        .series
        .sorting(vec![title_sort_option]);
    }
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::SeriesVec(series) = network
      .handle_sonarr_event(SonarrEvent::ListSeries)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.sonarr_data.series.items,
        expected_sorted_series
      );
      assert!(app_arc.lock().await.data.sonarr_data.series.sort_asc);
      assert_eq!(series, expected_series);
    }
  }

  #[tokio::test]
  async fn test_handle_get_series_details_event() {
    let expected_series: Series = serde_json::from_str(SERIES_JSON).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(serde_json::from_str(SERIES_JSON).unwrap()),
      None,
      SonarrEvent::GetSeriesDetails(1),
      Some("/1"),
      None,
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .set_items(vec![series()]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::Series(series) = network
      .handle_sonarr_event(SonarrEvent::GetSeriesDetails(1))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(series, expected_series);
    }
  }

  #[rstest]
  #[tokio::test]
  async fn test_handle_get_sonarr_series_history_event(
    #[values(true, false)] use_custom_sorting: bool,
  ) {
    let history_json = json!([{
      "id": 123,
      "sourceTitle": "z episode",
      "episodeId": 1007,
      "quality": { "quality": { "name": "Bluray-1080p" } },
      "languages": [{ "id": 1, "name": "English" }],
      "date": "2024-02-10T07:28:45Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/series/Coolness/something.cool.mkv",
        "importedPath": "/nfs/tv/Coolness/Season 1/Coolness - S01E01 - Something Cool Bluray-1080p.mkv"
      }
    },
    {
      "id": 456,
      "sourceTitle": "A Episode",
      "episodeId": 2001,
      "quality": { "quality": { "name": "Bluray-1080p" } },
      "languages": [{ "id": 1, "name": "English" }],
      "date": "2024-02-10T07:28:45Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/series/Coolness/something.cool.mkv",
        "importedPath": "/nfs/tv/Coolness/Season 1/Coolness - S01E01 - Something Cool Bluray-1080p.mkv"
      }
    }]);
    let response: Vec<SonarrHistoryItem> = serde_json::from_value(history_json.clone()).unwrap();
    let mut expected_history_items = vec![
      SonarrHistoryItem {
        id: 123,
        episode_id: 1007,
        source_title: "z episode".into(),
        ..history_item()
      },
      SonarrHistoryItem {
        id: 456,
        episode_id: 2001,
        source_title: "A Episode".into(),
        ..history_item()
      },
    ];
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(history_json),
      None,
      SonarrEvent::GetSeriesHistory(1),
      None,
      Some("seriesId=1"),
    )
    .await;
    let mut series_history_table = StatefulTable {
      sort_asc: true,
      ..StatefulTable::default()
    };
    if use_custom_sorting {
      let cmp_fn = |a: &SonarrHistoryItem, b: &SonarrHistoryItem| {
        a.source_title
          .text
          .to_lowercase()
          .cmp(&b.source_title.text.to_lowercase())
      };
      expected_history_items.sort_by(cmp_fn);

      let history_sort_option = SortOption {
        name: "Source Title",
        cmp_fn: Some(cmp_fn),
      };
      series_history_table.sorting(vec![history_sort_option]);
    }
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .set_items(vec![series()]);
    app_arc.lock().await.data.sonarr_data.series_history = Some(series_history_table);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::SonarrHistoryItems(history_items) = network
      .handle_sonarr_event(SonarrEvent::GetSeriesHistory(1))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert!(app_arc
        .lock()
        .await
        .data
        .sonarr_data
        .series_history
        .is_some());
      assert_eq!(
        app_arc
          .lock()
          .await
          .data
          .sonarr_data
          .series_history
          .as_ref()
          .unwrap()
          .items,
        expected_history_items
      );
      assert!(
        app_arc
          .lock()
          .await
          .data
          .sonarr_data
          .series_history
          .as_ref()
          .unwrap()
          .sort_asc
      );
      assert_eq!(history_items, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_sonarr_series_history_event_empty_series_history_table() {
    let history_json = json!([{
      "id": 123,
      "sourceTitle": "z episode",
      "episodeId": 1007,
      "quality": { "quality": { "name": "Bluray-1080p" } },
      "languages": [{ "id": 1, "name": "English" }],
      "date": "2024-02-10T07:28:45Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/series/Coolness/something.cool.mkv",
        "importedPath": "/nfs/tv/Coolness/Season 1/Coolness - S01E01 - Something Cool Bluray-1080p.mkv"
      }
    },
    {
      "id": 456,
      "sourceTitle": "A Episode",
      "episodeId": 2001,
      "quality": { "quality": { "name": "Bluray-1080p" } },
      "languages": [{ "id": 1, "name": "English" }],
      "date": "2024-02-10T07:28:45Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/series/Coolness/something.cool.mkv",
        "importedPath": "/nfs/tv/Coolness/Season 1/Coolness - S01E01 - Something Cool Bluray-1080p.mkv"
      }
    }]);
    let response: Vec<SonarrHistoryItem> = serde_json::from_value(history_json.clone()).unwrap();
    let expected_history_items = vec![
      SonarrHistoryItem {
        id: 123,
        episode_id: 1007,
        source_title: "z episode".into(),
        ..history_item()
      },
      SonarrHistoryItem {
        id: 456,
        episode_id: 2001,
        source_title: "A Episode".into(),
        ..history_item()
      },
    ];
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(history_json),
      None,
      SonarrEvent::GetSeriesHistory(1),
      None,
      Some("seriesId=1"),
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .set_items(vec![series()]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::SonarrHistoryItems(history_items) = network
      .handle_sonarr_event(SonarrEvent::GetSeriesHistory(1))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert!(app_arc
        .lock()
        .await
        .data
        .sonarr_data
        .series_history
        .is_some());
      assert_eq!(
        app_arc
          .lock()
          .await
          .data
          .sonarr_data
          .series_history
          .as_ref()
          .unwrap()
          .items,
        expected_history_items
      );
      assert!(
        !app_arc
          .lock()
          .await
          .data
          .sonarr_data
          .series_history
          .as_ref()
          .unwrap()
          .sort_asc
      );
      assert_eq!(history_items, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_sonarr_series_history_event_no_op_when_user_is_selecting_sort_options() {
    let history_json = json!([{
      "id": 123,
      "sourceTitle": "z episode",
      "episodeId": 1007,
      "quality": { "quality": { "name": "Bluray-1080p" } },
      "languages": [{ "id": 1, "name": "English" }],
      "date": "2024-02-10T07:28:45Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/series/Coolness/something.cool.mkv",
        "importedPath": "/nfs/tv/Coolness/Season 1/Coolness - S01E01 - Something Cool Bluray-1080p.mkv"
      }
    },
    {
      "id": 456,
      "sourceTitle": "A Episode",
      "episodeId": 2001,
      "quality": { "quality": { "name": "Bluray-1080p" } },
      "languages": [{ "id": 1, "name": "English" }],
      "date": "2024-02-10T07:28:45Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/series/Coolness/something.cool.mkv",
        "importedPath": "/nfs/tv/Coolness/Season 1/Coolness - S01E01 - Something Cool Bluray-1080p.mkv"
      }
    }]);
    let response: Vec<SonarrHistoryItem> = serde_json::from_value(history_json.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(history_json),
      None,
      SonarrEvent::GetSeriesHistory(1),
      None,
      Some("seriesId=1"),
    )
    .await;
    let cmp_fn = |a: &SonarrHistoryItem, b: &SonarrHistoryItem| {
      a.source_title
        .text
        .to_lowercase()
        .cmp(&b.source_title.text.to_lowercase())
    };
    let history_sort_option = SortOption {
      name: "Source Title",
      cmp_fn: Some(cmp_fn),
    };
    let mut series_history_table = StatefulTable {
      sort_asc: true,
      ..StatefulTable::default()
    };
    series_history_table.sorting(vec![history_sort_option]);
    app_arc.lock().await.data.sonarr_data.series_history = Some(series_history_table);
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .set_items(vec![series()]);
    app_arc
      .lock()
      .await
      .push_navigation_stack(ActiveSonarrBlock::SeriesHistorySortPrompt.into());
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::SonarrHistoryItems(history_items) = network
      .handle_sonarr_event(SonarrEvent::GetSeriesHistory(1))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert!(app_arc
        .lock()
        .await
        .data
        .sonarr_data
        .series_history
        .is_some());
      assert!(app_arc
        .lock()
        .await
        .data
        .sonarr_data
        .series_history
        .as_ref()
        .unwrap()
        .is_empty());
      assert!(
        app_arc
          .lock()
          .await
          .data
          .sonarr_data
          .series_history
          .as_ref()
          .unwrap()
          .sort_asc
      );
      assert_eq!(history_items, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_series_event_no_op_while_user_is_selecting_sort_options() {
    let mut series_1: Value = serde_json::from_str(SERIES_JSON).unwrap();
    let mut series_2: Value = serde_json::from_str(SERIES_JSON).unwrap();
    *series_1.get_mut("id").unwrap() = json!(1);
    *series_1.get_mut("title").unwrap() = json!("z test");
    *series_2.get_mut("id").unwrap() = json!(2);
    *series_2.get_mut("title").unwrap() = json!("A test");
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(json!([series_1, series_2])),
      None,
      SonarrEvent::ListSeries,
      None,
      None,
    )
    .await;
    app_arc
      .lock()
      .await
      .push_navigation_stack(ActiveSonarrBlock::SeriesSortPrompt.into());
    app_arc.lock().await.data.sonarr_data.series.sort_asc = true;
    let cmp_fn = |a: &Series, b: &Series| {
      a.title
        .text
        .to_lowercase()
        .cmp(&b.title.text.to_lowercase())
    };
    let title_sort_option = SortOption {
      name: "Title",
      cmp_fn: Some(cmp_fn),
    };
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .sorting(vec![title_sort_option]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::ListSeries)
      .await
      .is_ok());

    async_server.assert_async().await;
    assert!(app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .items
      .is_empty());
    assert!(app_arc.lock().await.data.sonarr_data.series.sort_asc);
  }

  #[tokio::test]
  async fn test_handle_get_sonarr_security_config_event() {
    let security_config_response = json!({
      "authenticationMethod": "forms",
      "authenticationRequired": "disabledForLocalAddresses",
      "username": "test",
      "password": "some password",
      "apiKey": "someApiKey12345",
      "certificateValidation": "disabledForLocalAddresses",
    });
    let response: SecurityConfig =
      serde_json::from_value(security_config_response.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(security_config_response),
      None,
      SonarrEvent::GetSecurityConfig,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::SecurityConfig(security_config) = network
      .handle_sonarr_event(SonarrEvent::GetSecurityConfig)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(security_config, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_status_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(json!({
        "version": "v1",
        "startTime": "2023-02-25T20:16:43Z"
      })),
      None,
      SonarrEvent::GetStatus,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());
    let date_time = DateTime::from(DateTime::parse_from_rfc3339("2023-02-25T20:16:43Z").unwrap());

    if let SonarrSerdeable::SystemStatus(status) = network
      .handle_sonarr_event(SonarrEvent::GetStatus)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_str_eq!(app_arc.lock().await.data.sonarr_data.version, "v1");
      assert_eq!(app_arc.lock().await.data.sonarr_data.start_time, date_time);
      assert_eq!(
        status,
        SystemStatus {
          version: "v1".to_owned(),
          start_time: date_time
        }
      );
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
  async fn test_handle_get_sonarr_tasks_event() {
    let tasks_json = json!([{
        "name": "Application Update Check",
        "taskName": "ApplicationUpdateCheck",
        "interval": 360,
        "lastExecution": "2023-05-20T21:29:16Z",
        "nextExecution": "2023-05-20T21:29:16Z",
    },
    {
        "name": "Backup",
        "taskName": "Backup",
        "interval": 10080,
        "lastExecution": "2023-05-20T21:29:16Z",
        "nextExecution": "2023-05-20T21:29:16Z",
    }]);
    let response: Vec<SonarrTask> = serde_json::from_value(tasks_json.clone()).unwrap();
    let timestamp = DateTime::from(DateTime::parse_from_rfc3339("2023-05-20T21:29:16Z").unwrap());
    let expected_tasks = vec![
      SonarrTask {
        name: "Application Update Check".to_owned(),
        task_name: SonarrTaskName::ApplicationUpdateCheck,
        interval: 360,
        last_execution: timestamp,
        next_execution: timestamp,
      },
      SonarrTask {
        name: "Backup".to_owned(),
        task_name: SonarrTaskName::Backup,
        interval: 10080,
        last_execution: timestamp,
        next_execution: timestamp,
      },
    ];
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(tasks_json),
      None,
      SonarrEvent::GetTasks,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::Tasks(tasks) = network
      .handle_sonarr_event(SonarrEvent::GetTasks)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.sonarr_data.tasks.items,
        expected_tasks
      );
      assert_eq!(tasks, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_sonarr_updates_event() {
    let updates_json = json!([{
      "version": "4.3.2.1",
      "releaseDate": "2023-04-15T02:02:53Z",
      "installed": true,
      "installedOn": "2023-04-15T02:02:53Z",
      "latest": true,
      "changes": {
        "new": [
          "Cool new thing"
        ],
        "fixed": [
          "Some bugs killed"
        ]
      },
    },
      {
        "version": "3.2.1.0",
        "releaseDate": "2023-04-15T02:02:53Z",
        "installed": false,
        "installedOn": "2023-04-15T02:02:53Z",
        "latest": false,
        "changes": {
          "new": [
            "Cool new thing (old)",
            "Other cool new thing (old)"
            ],
        },
    },
    {
      "version": "2.1.0",
      "releaseDate": "2023-04-15T02:02:53Z",
      "installed": false,
      "latest": false,
      "changes": {
        "fixed": [
          "Killed bug 1",
          "Fixed bug 2"
        ]
      },
    }]);
    let response: Vec<Update> = serde_json::from_value(updates_json.clone()).unwrap();
    let line_break = "-".repeat(200);
    let expected_text = ScrollableText::with_string(formatdoc!(
      "
    The latest version of Sonarr is already installed

    4.3.2.1 - 2023-04-15 02:02:53 UTC (Currently Installed)
    {line_break}
    New:
      * Cool new thing
    Fixed:
      * Some bugs killed
    
    
    3.2.1.0 - 2023-04-15 02:02:53 UTC (Previously Installed)
    {line_break}
    New:
      * Cool new thing (old)
      * Other cool new thing (old)
    
    
    2.1.0 - 2023-04-15 02:02:53 UTC 
    {line_break}
    Fixed:
      * Killed bug 1
      * Fixed bug 2"
    ));
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(updates_json),
      None,
      SonarrEvent::GetUpdates,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::Updates(updates) = network
      .handle_sonarr_event(SonarrEvent::GetUpdates)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_str_eq!(
        app_arc.lock().await.data.sonarr_data.updates.get_text(),
        expected_text.get_text()
      );
      assert_eq!(updates, response);
    }
  }

  #[tokio::test]
  async fn test_handle_mark_sonarr_history_item_as_failed_event() {
    let expected_history_item_id = 1;
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      None,
      Some(json!({})),
      None,
      SonarrEvent::MarkHistoryItemAsFailed(expected_history_item_id),
      Some("/1"),
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::MarkHistoryItemAsFailed(
        expected_history_item_id
      ))
      .await
      .is_ok());
    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_search_new_series_event() {
    let add_series_search_result_json = json!([{
      "tvdbId": 1234,
      "title": "Test",
      "status": "continuing",
      "ended": false,
      "overview": "New series blah blah blah",
      "genres": ["cool", "family", "fun"],
      "year": 2023,
      "network": "Prime Video",
      "runtime": 60,
      "ratings": { "votes": 406744, "value": 8.4 },
      "statistics": { "seasonCount": 3 }
    }]);
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(add_series_search_result_json),
      None,
      SonarrEvent::SearchNewSeries("test term".into()),
      None,
      Some("term=test%20term"),
    )
    .await;
    app_arc.lock().await.data.sonarr_data.add_series_search = Some("test term".into());
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::AddSeriesSearchResults(add_series_search_results) = network
      .handle_sonarr_event(SonarrEvent::SearchNewSeries("test term".into()))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert!(app_arc
        .lock()
        .await
        .data
        .sonarr_data
        .add_searched_series
        .is_some());
      assert_eq!(
        app_arc
          .lock()
          .await
          .data
          .sonarr_data
          .add_searched_series
          .as_ref()
          .unwrap()
          .items,
        vec![add_series_search_result()]
      );
      assert_eq!(add_series_search_results, vec![add_series_search_result()]);
    }
  }

  #[tokio::test]
  async fn test_handle_search_new_series_event_no_results() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(json!([])),
      None,
      SonarrEvent::SearchNewSeries("test term".into()),
      None,
      Some("term=test%20term"),
    )
    .await;
    app_arc.lock().await.data.sonarr_data.add_series_search = Some("test term".into());
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::SearchNewSeries("test term".into()))
      .await
      .is_ok());

    async_server.assert_async().await;
    assert!(app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .add_searched_series
      .is_none());
    assert_eq!(
      app_arc.lock().await.get_current_route(),
      ActiveSonarrBlock::AddSeriesEmptySearchResults.into()
    );
  }

  #[tokio::test]
  async fn test_handle_start_sonarr_task_event() {
    let response = json!({ "test": "test"});
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "ApplicationUpdateCheck"
      })),
      Some(response.clone()),
      None,
      SonarrEvent::StartTask(SonarrTaskName::ApplicationUpdateCheck),
      None,
      None,
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .tasks
      .set_items(vec![SonarrTask {
        task_name: SonarrTaskName::default(),
        ..SonarrTask::default()
      }]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::Value(value) = network
      .handle_sonarr_event(SonarrEvent::StartTask(
        SonarrTaskName::ApplicationUpdateCheck,
      ))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(value, response);
    }
  }

  #[tokio::test]
  async fn test_handle_test_sonarr_indexer_event_error() {
    let indexer_details_json = json!({
        "enableRss": true,
        "enableAutomaticSearch": true,
        "enableInteractiveSearch": true,
        "name": "Test Indexer",
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://test.com",
            },
            {
                "name": "apiKey",
                "value": "",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.2",
            },
        ],
        "tags": [1],
        "id": 1
    });
    let response_json = json!([
    {
        "isWarning": false,
        "propertyName": "",
        "errorMessage": "test failure",
        "severity": "error"
    }]);
    let (async_details_server, app_arc, mut server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(indexer_details_json.clone()),
      None,
      SonarrEvent::GetIndexers,
      Some("/1"),
      None,
    )
    .await;
    let async_test_server = server
      .mock(
        "POST",
        format!("/api/v3{}", SonarrEvent::TestIndexer(1).resource()).as_str(),
      )
      .with_status(400)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(indexer_details_json.clone()))
      .with_body(response_json.to_string())
      .create_async()
      .await;
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .indexers
      .set_items(vec![indexer()]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::Value(value) = network
      .handle_sonarr_event(SonarrEvent::TestIndexer(1))
      .await
      .unwrap()
    {
      async_details_server.assert_async().await;
      async_test_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.sonarr_data.indexer_test_errors,
        Some("\"test failure\"".to_owned())
      );
      assert_eq!(value, response_json)
    }
  }

  #[tokio::test]
  async fn test_handle_test_sonarr_indexer_event_success() {
    let indexer_details_json = json!({
        "enableRss": true,
        "enableAutomaticSearch": true,
        "enableInteractiveSearch": true,
        "name": "Test Indexer",
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://test.com",
            },
            {
                "name": "apiKey",
                "value": "",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.2",
            },
        ],
        "tags": [1],
        "id": 1
    });
    let (async_details_server, app_arc, mut server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(indexer_details_json.clone()),
      None,
      SonarrEvent::GetIndexers,
      Some("/1"),
      None,
    )
    .await;
    let async_test_server = server
      .mock(
        "POST",
        format!("/api/v3{}", SonarrEvent::TestIndexer(1).resource()).as_str(),
      )
      .with_status(200)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(indexer_details_json.clone()))
      .with_body("{}")
      .create_async()
      .await;
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .indexers
      .set_items(vec![indexer()]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::Value(value) = network
      .handle_sonarr_event(SonarrEvent::TestIndexer(1))
      .await
      .unwrap()
    {
      async_details_server.assert_async().await;
      async_test_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.sonarr_data.indexer_test_errors,
        Some(String::new())
      );
      assert_eq!(value, json!({}));
    }
  }

  #[tokio::test]
  async fn test_handle_test_all_sonarr_indexers_event() {
    let indexers = vec![
      Indexer {
        id: 1,
        name: Some("Test 1".to_owned()),
        ..Indexer::default()
      },
      Indexer {
        id: 2,
        name: Some("Test 2".to_owned()),
        ..Indexer::default()
      },
    ];
    let indexer_test_results_modal_items = vec![
			IndexerTestResultModalItem {
				name: "Test 1".to_owned(),
				is_valid: true,
				validation_failures: HorizontallyScrollableText::default(),
			},
			IndexerTestResultModalItem {
				name: "Test 2".to_owned(),
				is_valid: false,
				validation_failures: "Failure for field 'test field 1': test error message, Failure for field 'test field 2': test error message 2".into(),
			},
		];
    let response_json = json!([
    {
      "id": 1,
      "isValid": true,
      "validationFailures": []
    },
    {
      "id": 2,
      "isValid": false,
      "validationFailures": [
          {
              "propertyName": "test field 1",
              "errorMessage": "test error message",
              "severity": "error"
          },
          {
              "propertyName": "test field 2",
              "errorMessage": "test error message 2",
              "severity": "error"
          },
      ]
    }]);
    let response: Vec<IndexerTestResult> = serde_json::from_value(response_json.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      None,
      Some(response_json),
      Some(400),
      SonarrEvent::TestAllIndexers,
      None,
      None,
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .indexers
      .set_items(indexers);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::IndexerTestResults(results) = network
      .handle_sonarr_event(SonarrEvent::TestAllIndexers)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert!(app_arc
        .lock()
        .await
        .data
        .sonarr_data
        .indexer_test_all_results
        .is_some());
      assert_eq!(
        app_arc
          .lock()
          .await
          .data
          .sonarr_data
          .indexer_test_all_results
          .as_ref()
          .unwrap()
          .items,
        indexer_test_results_modal_items
      );
      assert_eq!(results, response);
    }
  }

  #[tokio::test]
  async fn test_handle_toggle_episode_monitoring_event() {
    let expected_body = MonitorEpisodeBody {
      episode_ids: vec![2],
      monitored: false,
    };
    let body = Episode { id: 2, ..episode() };

    let (async_details_server, app_arc, mut server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(json!(body)),
      None,
      SonarrEvent::GetEpisodeDetails(2),
      Some("/2"),
      None,
    )
    .await;
    let async_toggle_server = server
      .mock(
        "PUT",
        format!(
          "/api/v3{}",
          SonarrEvent::ToggleEpisodeMonitoring(2).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(json!(expected_body)))
      .create_async()
      .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::ToggleEpisodeMonitoring(2))
      .await
      .is_ok());

    async_details_server.assert_async().await;
    async_toggle_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_toggle_season_monitoring_event() {
    let mut expected_body: Value = serde_json::from_str(SERIES_JSON).unwrap();
    *expected_body
      .get_mut("seasons")
      .unwrap()
      .as_array_mut()
      .unwrap()
      .iter_mut()
      .find(|season| season["seasonNumber"] == 1)
      .unwrap()
      .get_mut("monitored")
      .unwrap() = json!(false);

    let (async_details_server, app_arc, mut server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(serde_json::from_str(SERIES_JSON).unwrap()),
      None,
      SonarrEvent::GetSeriesDetails(1),
      Some("/1"),
      None,
    )
    .await;
    let async_toggle_server = server
      .mock(
        "PUT",
        format!(
          "/api/v3{}/1",
          SonarrEvent::ToggleSeasonMonitoring((1, 1)).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .create_async()
      .await;
    {
      let mut app = app_arc.lock().await;
      app.data.sonarr_data.series.set_items(vec![series()]);
      app.data.sonarr_data.seasons.set_items(vec![season()]);
    }
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::ToggleSeasonMonitoring((1, 1)))
      .await
      .is_ok());

    async_details_server.assert_async().await;
    async_toggle_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_trigger_automatic_episode_search_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "EpisodeSearch",
        "episodeIds": [ 1 ]
      })),
      Some(json!({})),
      None,
      SonarrEvent::TriggerAutomaticEpisodeSearch(1),
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::TriggerAutomaticEpisodeSearch(1))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_trigger_automatic_season_search_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "SeasonSearch",
        "seriesId": 1,
        "seasonNumber": 1
      })),
      Some(json!({})),
      None,
      SonarrEvent::TriggerAutomaticSeasonSearch((1, 1)),
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::TriggerAutomaticSeasonSearch((1, 1)))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_trigger_automatic_series_search_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "SeriesSearch",
        "seriesId": 1
      })),
      Some(json!({})),
      None,
      SonarrEvent::TriggerAutomaticSeriesSearch(None),
      None,
      None,
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .set_items(vec![series()]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::TriggerAutomaticSeriesSearch(None))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_trigger_automatic_series_search_event_uses_provided_id() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "SeriesSearch",
        "seriesId": 1
      })),
      Some(json!({})),
      None,
      SonarrEvent::TriggerAutomaticSeriesSearch(None),
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::TriggerAutomaticSeriesSearch(Some(1)))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_update_all_series_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "RefreshSeries",
      })),
      Some(json!({})),
      None,
      SonarrEvent::UpdateAllSeries,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::UpdateAllSeries)
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_update_and_scan_series_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "RefreshSeries",
        "seriesId": 1,
      })),
      Some(json!({})),
      None,
      SonarrEvent::UpdateAndScanSeries(None),
      None,
      None,
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .set_items(vec![series()]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::UpdateAndScanSeries(None))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_update_and_scan_series_event_uses_provied_series_id() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "RefreshSeries",
        "seriesId": 1
      })),
      Some(json!({})),
      None,
      SonarrEvent::UpdateAndScanSeries(Some(1)),
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::UpdateAndScanSeries(Some(1)))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_update_sonarr_downloads_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "RefreshMonitoredDownloads"
      })),
      Some(json!({})),
      None,
      SonarrEvent::UpdateDownloads,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::UpdateDownloads)
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_extract_and_add_sonarr_tag_ids_vec() {
    let app_arc = Arc::new(Mutex::new(App::default()));
    let tags = "    test,HI ,, usenet ".to_owned();
    {
      let mut app = app_arc.lock().await;
      app.data.sonarr_data.tags_map = BiMap::from_iter([
        (1, "usenet".to_owned()),
        (2, "test".to_owned()),
        (3, "hi".to_owned()),
      ]);
    }
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
    let tags = "usenet, test, TESTING".to_owned();
    {
      let mut app = app_arc.lock().await;
      app.data.sonarr_data.add_series_modal = Some(AddSeriesModal {
        tags: tags.clone().into(),
        ..AddSeriesModal::default()
      });
      app.data.sonarr_data.tags_map =
        BiMap::from_iter([(1, "usenet".to_owned()), (2, "test".to_owned())]);
    }
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

  #[tokio::test]
  async fn test_extract_series_id() {
    let app_arc = Arc::new(Mutex::new(App::default()));
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .set_items(vec![Series {
        id: 1,
        ..Series::default()
      }]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    let (id, series_id_param) = network.extract_series_id(None).await;

    assert_eq!(id, 1);
    assert_str_eq!(series_id_param, "seriesId=1");
  }

  #[tokio::test]
  async fn test_extract_series_id_uses_provided_id() {
    let app_arc = Arc::new(Mutex::new(App::default()));
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .set_items(vec![Series {
        id: 1,
        ..Series::default()
      }]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    let (id, series_id_param) = network.extract_series_id(Some(2)).await;

    assert_eq!(id, 2);
    assert_str_eq!(series_id_param, "seriesId=2");
  }

  #[tokio::test]
  async fn test_extract_series_id_filtered_series() {
    let app_arc = Arc::new(Mutex::new(App::default()));
    let mut filtered_series = StatefulTable::default();
    filtered_series.set_filtered_items(vec![Series {
      id: 1,
      ..Series::default()
    }]);
    app_arc.lock().await.data.sonarr_data.series = filtered_series;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    let (id, series_id_param) = network.extract_series_id(None).await;

    assert_eq!(id, 1);
    assert_str_eq!(series_id_param, "seriesId=1");
  }

  #[tokio::test]
  async fn test_extract_season_number() {
    let app_arc = Arc::new(Mutex::new(App::default()));
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .seasons
      .set_items(vec![Season {
        season_number: 1,
        ..Season::default()
      }]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    let (id, season_number_param) = network.extract_season_number(None).await.unwrap();

    assert_eq!(id, 1);
    assert_str_eq!(season_number_param, "seasonNumber=1");
  }

  #[tokio::test]
  async fn test_extract_season_number_uses_provided_season_number() {
    let app_arc = Arc::new(Mutex::new(App::default()));
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .seasons
      .set_items(vec![Season {
        season_number: 1,
        ..Season::default()
      }]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());
    let (id, season_number_param) = network.extract_season_number(Some(2)).await.unwrap();

    assert_eq!(id, 2);
    assert_str_eq!(season_number_param, "seasonNumber=2");
  }

  #[tokio::test]
  async fn test_extract_season_number_filtered_seasons() {
    let app_arc = Arc::new(Mutex::new(App::default()));
    let mut filtered_seasons = StatefulTable::default();
    filtered_seasons.set_items(vec![Season::default()]);
    filtered_seasons.set_filtered_items(vec![Season {
      season_number: 1,
      ..Season::default()
    }]);
    app_arc.lock().await.data.sonarr_data.seasons = filtered_seasons;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    let (id, season_number_param) = network.extract_season_number(None).await.unwrap();

    assert_eq!(id, 1);
    assert_str_eq!(season_number_param, "seasonNumber=1");
  }

  #[tokio::test]
  async fn test_extract_season_number_empty_seasons_table() {
    let app_arc = Arc::new(Mutex::new(App::default()));
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());
    let season_number = network.extract_season_number(None).await;

    assert!(season_number.is_err());
  }

  #[tokio::test]
  async fn test_extract_episode_id() {
    let app_arc = Arc::new(Mutex::new(App::default()));
    let mut season_details_modal = SeasonDetailsModal::default();
    season_details_modal.episodes.set_items(vec![Episode {
      id: 1,
      ..Episode::default()
    }]);
    app_arc.lock().await.data.sonarr_data.season_details_modal = Some(season_details_modal);
    app_arc
      .lock()
      .await
      .push_navigation_stack(ActiveSonarrBlock::EpisodeDetails.into());
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    let id = network.extract_episode_id(None).await;

    assert_eq!(id, 1);
  }

  #[tokio::test]
  async fn test_extract_episode_id_uses_provided_id() {
    let app_arc = Arc::new(Mutex::new(App::default()));
    let mut season_details_modal = SeasonDetailsModal::default();
    season_details_modal.episodes.set_items(vec![Episode {
      id: 1,
      ..Episode::default()
    }]);
    app_arc.lock().await.data.sonarr_data.season_details_modal = Some(season_details_modal);
    app_arc
      .lock()
      .await
      .push_navigation_stack(ActiveSonarrBlock::EpisodeDetails.into());
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    let id = network.extract_episode_id(Some(2)).await;

    assert_eq!(id, 2);
  }

  #[tokio::test]
  async fn test_extract_episode_id_filtered_series() {
    let app_arc = Arc::new(Mutex::new(App::default()));
    let mut filtered_episodes = StatefulTable::default();
    filtered_episodes.set_filtered_items(vec![Episode {
      id: 1,
      ..Episode::default()
    }]);
    let season_details_modal = SeasonDetailsModal {
      episodes: filtered_episodes,
      ..SeasonDetailsModal::default()
    };
    app_arc.lock().await.data.sonarr_data.season_details_modal = Some(season_details_modal);
    app_arc
      .lock()
      .await
      .push_navigation_stack(ActiveSonarrBlock::EpisodeDetails.into());
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    let id = network.extract_episode_id(None).await;

    assert_eq!(id, 1);
  }

  #[test]
  fn test_get_episode_status_downloaded() {
    assert_str_eq!(get_episode_status(true, &[], 0), "Downloaded");
  }

  #[test]
  fn test_get_episode_status_missing() {
    let download_record = DownloadRecord {
      episode_id: 1,
      ..DownloadRecord::default()
    };

    assert_str_eq!(
      get_episode_status(false, &[download_record.clone()], 0),
      "Missing"
    );

    assert_str_eq!(get_episode_status(false, &[download_record], 1), "Missing");
  }

  #[test]
  fn test_get_episode_status_downloading() {
    assert_str_eq!(
      get_episode_status(
        false,
        &[DownloadRecord {
          episode_id: 1,
          status: DownloadStatus::Downloading,
          ..DownloadRecord::default()
        }],
        1
      ),
      "Downloading"
    );
  }

  #[test]
  fn test_get_episode_status_awaiting_import() {
    assert_str_eq!(
      get_episode_status(
        false,
        &[DownloadRecord {
          episode_id: 1,
          status: DownloadStatus::Completed,
          ..DownloadRecord::default()
        }],
        1
      ),
      "Awaiting Import"
    );
  }

  fn add_series_search_result() -> AddSeriesSearchResult {
    AddSeriesSearchResult {
      tvdb_id: 1234,
      title: HorizontallyScrollableText::from("Test"),
      status: Some("continuing".to_owned()),
      ended: false,
      overview: Some("New series blah blah blah".to_owned()),
      genres: genres(),
      year: 2023,
      network: Some("Prime Video".to_owned()),
      runtime: 60,
      ratings: Some(rating()),
      statistics: Some(add_series_search_result_statistics()),
    }
  }

  fn add_series_search_result_statistics() -> AddSeriesSearchResultStatistics {
    AddSeriesSearchResultStatistics { season_count: 3 }
  }

  fn blocklist_item() -> BlocklistItem {
    BlocklistItem {
      id: 1,
      series_id: 1,
      series_title: None,
      episode_ids: vec![Number::from(1)],
      source_title: "Test Source Title".to_owned(),
      languages: vec![language()],
      quality: quality_wrapper(),
      date: DateTime::from(DateTime::parse_from_rfc3339("2024-02-10T07:28:45Z").unwrap()),
      protocol: "usenet".to_owned(),
      indexer: "NZBgeek (Prowlarr)".to_owned(),
      message: "test message".to_owned(),
    }
  }

  fn download_record() -> DownloadRecord {
    DownloadRecord {
      title: "Test Download Title".to_owned(),
      status: DownloadStatus::Downloading,
      id: 1,
      episode_id: 1,
      size: 3543348019f64,
      sizeleft: 1771674009f64,
      output_path: Some(HorizontallyScrollableText::from(
        "/nfs/tv/Test show/season 1/",
      )),
      indexer: "kickass torrents".to_owned(),
      download_client: Some("transmission".to_owned()),
    }
  }

  fn downloads_response() -> DownloadsResponse {
    DownloadsResponse {
      records: vec![download_record()],
    }
  }

  fn episode() -> Episode {
    Episode {
      id: 1,
      series_id: 1,
      tvdb_id: 1234,
      episode_file_id: 1,
      season_number: 1,
      episode_number: 1,
      title: "Something cool".to_owned(),
      air_date_utc: Some(DateTime::from(
        DateTime::parse_from_rfc3339("2024-02-10T07:28:45Z").unwrap(),
      )),
      overview: Some("Okay so this one time at band camp...".to_owned()),
      has_file: true,
      monitored: true,
      episode_file: Some(episode_file()),
    }
  }

  fn episode_file() -> EpisodeFile {
    EpisodeFile {
      id: 1,
      relative_path: "/season 1/episode 1.mkv".to_owned(),
      path: "/nfs/tv/series/season 1/episode 1.mkv".to_owned(),
      size: 3543348019,
      quality: quality_wrapper(),
      languages: vec![language()],
      date_added: DateTime::from(DateTime::parse_from_rfc3339("2024-02-10T07:28:45Z").unwrap()),
      media_info: Some(media_info()),
    }
  }

  fn genres() -> Vec<String> {
    vec!["cool".to_owned(), "family".to_owned(), "fun".to_owned()]
  }

  fn history_data() -> SonarrHistoryData {
    SonarrHistoryData {
      dropped_path: Some("/nfs/nzbget/completed/series/Coolness/something.cool.mkv".to_owned()),
      imported_path: Some(
        "/nfs/tv/Coolness/Season 1/Coolness - S01E01 - Something Cool Bluray-1080p.mkv".to_owned(),
      ),
      ..SonarrHistoryData::default()
    }
  }

  fn history_item() -> SonarrHistoryItem {
    SonarrHistoryItem {
      id: 1,
      source_title: "Test source".into(),
      episode_id: 1,
      quality: quality_wrapper(),
      languages: vec![language()],
      date: DateTime::from(DateTime::parse_from_rfc3339("2024-02-10T07:28:45Z").unwrap()),
      event_type: SonarrHistoryEventType::Grabbed,
      data: history_data(),
    }
  }

  fn indexer() -> Indexer {
    Indexer {
      enable_rss: true,
      enable_automatic_search: true,
      enable_interactive_search: true,
      supports_rss: true,
      supports_search: true,
      protocol: "torrent".to_owned(),
      priority: 25,
      download_client_id: 0,
      name: Some("Test Indexer".to_owned()),
      implementation_name: Some("Torznab".to_owned()),
      implementation: Some("Torznab".to_owned()),
      config_contract: Some("TorznabSettings".to_owned()),
      tags: vec![Number::from(1)],
      id: 1,
      fields: Some(vec![
        IndexerField {
          name: Some("baseUrl".to_owned()),
          value: Some(json!("https://test.com")),
        },
        IndexerField {
          name: Some("apiKey".to_owned()),
          value: Some(json!("")),
        },
        IndexerField {
          name: Some("seedCriteria.seedRatio".to_owned()),
          value: Some(json!("1.2")),
        },
      ]),
    }
  }

  fn indexer_settings() -> IndexerSettings {
    IndexerSettings {
      id: 1,
      minimum_age: 1,
      retention: 1,
      maximum_size: 12345,
      rss_sync_interval: 60,
    }
  }

  fn language() -> Language {
    Language {
      id: 1,
      name: "English".to_owned(),
    }
  }

  fn media_info() -> MediaInfo {
    MediaInfo {
      audio_bitrate: 0,
      audio_channels: Number::from_f64(7.1).unwrap(),
      audio_codec: Some("AAC".to_owned()),
      audio_languages: Some("eng".to_owned()),
      audio_stream_count: 1,
      video_bit_depth: 10,
      video_bitrate: 0,
      video_codec: "x265".to_owned(),
      video_fps: Number::from_f64(23.976).unwrap(),
      resolution: "1920x1080".to_owned(),
      run_time: "23:51".to_owned(),
      scan_type: "Progressive".to_owned(),
      subtitles: Some("English".to_owned()),
    }
  }
  fn quality() -> Quality {
    Quality {
      name: "Bluray-1080p".to_owned(),
    }
  }

  fn quality_wrapper() -> QualityWrapper {
    QualityWrapper { quality: quality() }
  }

  fn rating() -> Rating {
    Rating {
      votes: 406744,
      value: 8.4,
    }
  }

  fn season() -> Season {
    Season {
      title: None,
      season_number: 1,
      monitored: true,
      statistics: season_statistics(),
    }
  }

  fn season_statistics() -> SeasonStatistics {
    SeasonStatistics {
      previous_airing: Some(DateTime::from(
        DateTime::parse_from_rfc3339("2022-10-24T01:00:00Z").unwrap(),
      )),
      next_airing: None,
      episode_file_count: 10,
      episode_count: 10,
      total_episode_count: 10,
      size_on_disk: 36708563419,
      percent_of_episodes: 100.0,
    }
  }

  fn series() -> Series {
    Series {
      title: "Test".to_owned().into(),
      status: SeriesStatus::Continuing,
      ended: false,
      overview: Some("Blah blah blah".to_owned()),
      network: Some("HBO".to_owned()),
      seasons: Some(vec![season()]),
      year: 2022,
      path: "/nfs/tv/Test".to_owned(),
      quality_profile_id: 6,
      language_profile_id: 1,
      season_folder: true,
      monitored: true,
      runtime: 63,
      tvdb_id: 371572,
      series_type: SeriesType::Standard,
      certification: Some("TV-MA".to_owned()),
      genres: vec!["cool".to_owned(), "family".to_owned(), "fun".to_owned()],
      tags: vec![Number::from(3)],
      ratings: rating(),
      statistics: Some(series_statistics()),
      id: 1,
    }
  }

  fn series_statistics() -> SeriesStatistics {
    SeriesStatistics {
      season_count: 2,
      episode_file_count: 18,
      episode_count: 18,
      total_episode_count: 50,
      size_on_disk: 63894022699,
      percent_of_episodes: 100.0,
    }
  }

  fn rejections() -> Vec<String> {
    vec![
      "Unknown quality profile".to_owned(),
      "Release is already mapped".to_owned(),
    ]
  }

  fn release() -> SonarrRelease {
    SonarrRelease {
      guid: "1234".to_owned(),
      protocol: "torrent".to_owned(),
      age: 1,
      title: HorizontallyScrollableText::from("Test Release"),
      indexer: "kickass torrents".to_owned(),
      indexer_id: 2,
      size: 1234,
      rejected: true,
      rejections: Some(rejections()),
      seeders: Some(Number::from(2)),
      leechers: Some(Number::from(1)),
      languages: Some(vec![language()]),
      quality: quality_wrapper(),
      full_season: false,
    }
  }

  fn root_folder() -> RootFolder {
    RootFolder {
      id: 1,
      path: "/nfs".to_owned(),
      accessible: true,
      free_space: 219902325555200,
      unmapped_folders: None,
    }
  }
}
