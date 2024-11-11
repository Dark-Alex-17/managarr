#[cfg(test)]
mod test {
  use chrono::{DateTime, Utc};
  use pretty_assertions::{assert_eq, assert_str_eq};
  use reqwest::Client;
  use rstest::rstest;
  use serde_json::json;
  use serde_json::{Number, Value};
  use tokio_util::sync::CancellationToken;

  use crate::models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock;
  use crate::models::sonarr_models::SystemStatus;
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
  #[case(SonarrEvent::HealthCheck, "/health")]
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
