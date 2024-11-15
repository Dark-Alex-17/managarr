#[cfg(test)]
mod test {
  use std::sync::Arc;

  use chrono::{DateTime, Utc};
  use managarr_tree_widget::TreeItem;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use reqwest::Client;
  use rstest::rstest;
  use serde_json::json;
  use serde_json::{Number, Value};
  use tokio::sync::Mutex;
  use tokio_util::sync::CancellationToken;

  use crate::app::App;
  use crate::models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock;
  use crate::models::sonarr_models::{BlocklistItem, Episode, Language, LogResponse};
  use crate::models::sonarr_models::{BlocklistResponse, Quality};
  use crate::models::sonarr_models::{QualityWrapper, SystemStatus};
  use crate::models::stateful_table::StatefulTable;
  use crate::models::HorizontallyScrollableText;
  use crate::models::{sonarr_models::SonarrSerdeable, stateful_table::SortOption};

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

  #[rstest]
  fn test_resource_series(#[values(SonarrEvent::ListSeries)] event: SonarrEvent) {
    assert_str_eq!(event.resource(), "/series");
  }

  #[rstest]
  #[case(SonarrEvent::ClearBlocklist, "/blocklist/bulk")]
  #[case(SonarrEvent::DeleteBlocklistItem(None), "/blocklist")]
  #[case(SonarrEvent::HealthCheck, "/health")]
  #[case(SonarrEvent::GetBlocklist, "/blocklist?page=1&pageSize=10000")]
  #[case(SonarrEvent::GetEpisodes(None), "/episode")]
  #[case(SonarrEvent::GetLogs(Some(500)), "/log")]
  #[case(SonarrEvent::GetStatus, "/system/status")]
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
      SonarrEvent::DeleteBlocklistItem(None),
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
      .handle_sonarr_event(SonarrEvent::DeleteBlocklistItem(None))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[rstest]
  #[tokio::test]
  async fn test_handle_get_sonarr_blocklist_event(#[values(true, false)] use_custom_sorting: bool) {
    let blocklist_json = json!({"records": [{
        "seriesId": 1007,
        "episodeIds": [42020],
        "sourceTitle": "z series",
        "language": { "id": 1, "name": "English" },
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
        "language": { "id": 1, "name": "English" },
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

  #[tokio::test]
  async fn test_handle_get_episodes_event() {
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
    let marker_episode_1 = Episode {
      title: Some("Season 1".to_owned()),
      ..Episode::default()
    };
    let marker_episode_2 = Episode {
      title: Some("Season 2".to_owned()),
      ..Episode::default()
    };
    let episode_1 = episode();
    let episode_2 = Episode {
      id: 2,
      episode_file_id: 2,
      season_number: 2,
      episode_number: 2,
      ..episode()
    };
    let expected_episodes = vec![episode_2.clone(), episode_1.clone()];
    let expected_tree = vec![
      TreeItem::new(
        marker_episode_1,
        vec![TreeItem::new_leaf(episode_1.clone())],
      )
      .unwrap(),
      TreeItem::new(
        marker_episode_2,
        vec![TreeItem::new_leaf(episode_2.clone())],
      )
      .unwrap(),
    ];
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(episodes_json),
      None,
      SonarrEvent::GetEpisodes(None),
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
      .handle_sonarr_event(SonarrEvent::GetEpisodes(None))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.sonarr_data.episodes.items,
        expected_tree
      );
      assert_eq!(episodes, expected_episodes);
    }
  }

  #[tokio::test]
  async fn test_handle_get_episodes_event_uses_provided_series_id() {
    let episodes_json = json!([
      {
          "id": 2,
          "seriesId": 2,
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
          "seriesId": 2,
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
    let marker_episode_1 = Episode {
      title: Some("Season 1".to_owned()),
      ..Episode::default()
    };
    let marker_episode_2 = Episode {
      title: Some("Season 2".to_owned()),
      ..Episode::default()
    };
    let episode_1 = Episode {
      series_id: 2,
      ..episode()
    };
    let episode_2 = Episode {
      id: 2,
      episode_file_id: 2,
      season_number: 2,
      episode_number: 2,
      series_id: 2,
      ..episode()
    };
    let expected_episodes = vec![episode_2.clone(), episode_1.clone()];
    let expected_tree = vec![
      TreeItem::new(
        marker_episode_1,
        vec![TreeItem::new_leaf(episode_1.clone())],
      )
      .unwrap(),
      TreeItem::new(
        marker_episode_2,
        vec![TreeItem::new_leaf(episode_2.clone())],
      )
      .unwrap(),
    ];
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(episodes_json),
      None,
      SonarrEvent::GetEpisodes(None),
      None,
      Some("seriesId=2"),
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::Episodes(episodes) = network
      .handle_sonarr_event(SonarrEvent::GetEpisodes(Some(2)))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.sonarr_data.episodes.items,
        expected_tree
      );
      assert_eq!(episodes, expected_episodes);
    }
  }

  #[tokio::test]
  async fn test_handle_get_sonarr_logs_event() {
    let expected_logs = vec![
      HorizontallyScrollableText::from(
        "2023-05-20 21:29:16 UTC|FATAL|RadarrError|Some.Big.Bad.Exception|test exception",
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
              "logger": "RadarrError",
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
      SonarrEvent::GetLogs(None),
      None,
      Some("pageSize=500&sortDirection=descending&sortKey=time"),
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::LogResponse(logs) = network
      .handle_sonarr_event(SonarrEvent::GetLogs(None))
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
  async fn test_handle_get_sonarr_logs_event_uses_provided_events() {
    let expected_logs = vec![
      HorizontallyScrollableText::from(
        "2023-05-20 21:29:16 UTC|FATAL|RadarrError|Some.Big.Bad.Exception|test exception",
      ),
      HorizontallyScrollableText::from("2023-05-20 21:29:16 UTC|INFO|TestLogger|test message"),
    ];
    let logs_response_json = json!({
      "page": 1,
      "pageSize": 1000,
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
              "logger": "RadarrError",
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
      SonarrEvent::GetLogs(Some(1000)),
      None,
      Some("pageSize=1000&sortDirection=descending&sortKey=time"),
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::LogResponse(logs) = network
      .handle_sonarr_event(SonarrEvent::GetLogs(Some(1000)))
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

  #[rstest]
  #[tokio::test]
  async fn test_handle_get_series_event(#[values(true, false)] use_custom_sorting: bool) {
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
    let date_time = DateTime::from(DateTime::parse_from_rfc3339("2023-02-25T20:16:43Z").unwrap())
      as DateTime<Utc>;

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

  fn blocklist_item() -> BlocklistItem {
    BlocklistItem {
      id: 1,
      series_id: 1,
      episode_ids: vec![Number::from(1)],
      source_title: "Test Source Title".to_owned(),
      language: language(),
      quality: quality_wrapper(),
      date: DateTime::from(DateTime::parse_from_rfc3339("2024-02-10T07:28:45Z").unwrap()),
      protocol: "usenet".to_owned(),
      indexer: "NZBgeek (Prowlarr)".to_owned(),
      message: "test message".to_owned(),
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
      title: Some("Something cool".to_owned()),
      air_date_utc: Some(DateTime::from(
        DateTime::parse_from_rfc3339("2024-02-10T07:28:45Z").unwrap(),
      )),
      overview: Some("Okay so this one time at band camp...".to_owned()),
      has_file: true,
      monitored: true,
    }
  }

  fn language() -> Language {
    Language {
      name: "English".to_owned(),
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
      overview: "Blah blah blah".into(),
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
}
