#[cfg(test)]
mod tests {
  use crate::models::servarr_data::sonarr::modals::SeasonDetailsModal;
  use crate::models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock;
  use crate::models::sonarr_models::{SonarrHistoryItem, SonarrRelease, SonarrSerdeable};
  use crate::network::NetworkResource;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::sonarr_network::SonarrEvent;
  use crate::network::sonarr_network::sonarr_network_test_utils::test_utils::{
    SERIES_JSON, season, series, sonarr_history_item, torrent_release,
  };
  use mockito::Matcher;
  use pretty_assertions::assert_eq;
  use serde_json::{Value, json};

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

    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(serde_json::from_str(SERIES_JSON).unwrap())
      .path("/1")
      .build_for(SonarrEvent::GetSeriesDetails(1))
      .await;
    let async_toggle_server = server
      .mock(
        "PUT",
        format!(
          "/api/v3{}/1",
          SonarrEvent::ToggleSeasonMonitoring(1, 1).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .create_async()
      .await;
    {
      let mut app_lock = app.lock().await;
      app_lock.data.sonarr_data.series.set_items(vec![series()]);
      app_lock.data.sonarr_data.seasons.set_items(vec![season()]);
    }
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    assert!(
      network
        .handle_sonarr_event(SonarrEvent::ToggleSeasonMonitoring(1, 1))
        .await
        .is_ok()
    );

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
      ..torrent_release()
    };
    let expected_raw_sonarr_releases = vec![
      SonarrRelease {
        full_season: true,
        ..torrent_release()
      },
      SonarrRelease {
        guid: "4567".to_owned(),
        ..torrent_release()
      },
    ];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(release_json)
      .query("seriesId=1&seasonNumber=1")
      .build_for(SonarrEvent::GetSeasonReleases(1, 1))
      .await;
    app
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .set_items(vec![series()]);
    app
      .lock()
      .await
      .data
      .sonarr_data
      .seasons
      .set_items(vec![season()]);
    app.lock().await.data.sonarr_data.season_details_modal = Some(SeasonDetailsModal::default());
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    let SonarrSerdeable::Releases(releases_vec) = network
      .handle_sonarr_event(SonarrEvent::GetSeasonReleases(1, 1))
      .await
      .unwrap()
    else {
      panic!("Expected Releases")
    };
    mock.assert_async().await;
    assert_eq!(
      app
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
      ..torrent_release()
    };
    let (mock, app, _server) = MockServarrApi::get()
      .returns(release_json)
      .query("seriesId=1&seasonNumber=1")
      .build_for(SonarrEvent::GetSeasonReleases(1, 1))
      .await;
    app
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .set_items(vec![series()]);
    app
      .lock()
      .await
      .data
      .sonarr_data
      .seasons
      .set_items(vec![season()]);
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    assert!(
      network
        .handle_sonarr_event(SonarrEvent::GetSeasonReleases(1, 1))
        .await
        .is_ok()
    );

    mock.assert_async().await;
    assert_eq!(
      app
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
        ..sonarr_history_item()
      },
      SonarrHistoryItem {
        id: 456,
        episode_id: 2001,
        source_title: "A Episode".into(),
        ..sonarr_history_item()
      },
    ];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(history_json)
      .query("seriesId=1&seasonNumber=1")
      .build_for(SonarrEvent::GetSeasonHistory(1, 1))
      .await;
    app.lock().await.data.sonarr_data.season_details_modal = Some(SeasonDetailsModal::default());
    app
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .set_items(vec![series()]);
    app
      .lock()
      .await
      .data
      .sonarr_data
      .seasons
      .set_items(vec![season()]);
    app
      .lock()
      .await
      .data
      .sonarr_data
      .season_details_modal
      .as_mut()
      .unwrap()
      .season_history
      .sort_asc = true;
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    let SonarrSerdeable::SonarrHistoryItems(history) = network
      .handle_sonarr_event(SonarrEvent::GetSeasonHistory(1, 1))
      .await
      .unwrap()
    else {
      panic!("Expected SonarrHistoryItems")
    };
    mock.assert_async().await;
    assert_eq!(
      app
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
      app
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
        ..sonarr_history_item()
      },
      SonarrHistoryItem {
        id: 456,
        episode_id: 2001,
        source_title: "A Episode".into(),
        ..sonarr_history_item()
      },
    ];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(history_json)
      .query("seriesId=1&seasonNumber=1")
      .build_for(SonarrEvent::GetSeasonHistory(1, 1))
      .await;
    app
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .set_items(vec![series()]);
    app
      .lock()
      .await
      .data
      .sonarr_data
      .seasons
      .set_items(vec![season()]);
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    let SonarrSerdeable::SonarrHistoryItems(history) = network
      .handle_sonarr_event(SonarrEvent::GetSeasonHistory(1, 1))
      .await
      .unwrap()
    else {
      panic!("Expected SonarrHistoryItems")
    };
    mock.assert_async().await;
    assert!(
      app
        .lock()
        .await
        .data
        .sonarr_data
        .season_details_modal
        .is_some()
    );
    assert_eq!(
      app
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
      !app
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

  #[tokio::test]
  async fn test_handle_get_sonarr_season_history_event_no_op_when_user_is_selecting_sort_option() {
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
    let (mock, app, _server) = MockServarrApi::get()
      .returns(history_json)
      .query("seriesId=1&seasonNumber=1")
      .build_for(SonarrEvent::GetSeasonHistory(1, 1))
      .await;
    app.lock().await.data.sonarr_data.season_details_modal = Some(SeasonDetailsModal::default());
    app
      .lock()
      .await
      .data
      .sonarr_data
      .season_details_modal
      .as_mut()
      .unwrap()
      .season_history
      .sort_asc = true;
    app.lock().await.server_tabs.next();
    app
      .lock()
      .await
      .push_navigation_stack(ActiveSonarrBlock::SeasonHistorySortPrompt.into());
    let mut network = test_network(&app);

    let SonarrSerdeable::SonarrHistoryItems(history) = network
      .handle_sonarr_event(SonarrEvent::GetSeasonHistory(1, 1))
      .await
      .unwrap()
    else {
      panic!("Expected SonarrHistoryItems")
    };
    mock.assert_async().await;
    assert_is_empty!(
      app
        .lock()
        .await
        .data
        .sonarr_data
        .season_details_modal
        .as_ref()
        .unwrap()
        .season_history
        .items
    );
    assert!(
      app
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

  #[tokio::test]
  async fn test_handle_trigger_automatic_season_search_event() {
    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(json!({
        "name": "SeasonSearch",
        "seriesId": 1,
        "seasonNumber": 1
      }))
      .returns(json!({}))
      .build_for(SonarrEvent::TriggerAutomaticSeasonSearch(1, 1))
      .await;
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    assert!(
      network
        .handle_sonarr_event(SonarrEvent::TriggerAutomaticSeasonSearch(1, 1))
        .await
        .is_ok()
    );

    mock.assert_async().await;
  }
}
