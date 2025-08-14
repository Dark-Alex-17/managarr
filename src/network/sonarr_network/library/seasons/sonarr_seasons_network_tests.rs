#[cfg(test)]
mod tests {
  use crate::models::servarr_data::sonarr::modals::SeasonDetailsModal;
  use crate::models::sonarr_models::{SonarrHistoryItem, SonarrRelease, SonarrSerdeable};
  use crate::network::network_tests::test_utils::mock_servarr_api;
  use crate::network::sonarr_network::sonarr_network_test_utils::test_utils::{
    history_item, release, season, series, SERIES_JSON,
  };
  use crate::network::sonarr_network::SonarrEvent;
  use crate::network::{Network, NetworkResource, RequestMethod};
  use mockito::Matcher;
  use pretty_assertions::assert_eq;
  use reqwest::Client;
  use serde_json::{json, Value};
  use tokio_util::sync::CancellationToken;

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
    app_arc.lock().await.server_tabs.next();
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::ToggleSeasonMonitoring((1, 1)))
      .await
      .is_ok());

    async_details_server.assert_async().await;
    async_toggle_server.assert_async().await;
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
    app_arc.lock().await.server_tabs.next();
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
    app_arc.lock().await.server_tabs.next();
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
    app_arc.lock().await.server_tabs.next();
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
    app_arc.lock().await.server_tabs.next();
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
    app_arc.lock().await.server_tabs.next();
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::TriggerAutomaticSeasonSearch((1, 1)))
      .await
      .is_ok());

    async_server.assert_async().await;
  }
}
