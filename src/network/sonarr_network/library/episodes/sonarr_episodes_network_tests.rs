#[cfg(test)]
mod tests {
  use crate::models::servarr_data::sonarr::modals::{EpisodeDetailsModal, SeasonDetailsModal};
  use crate::models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock;
  use crate::models::sonarr_models::{
    DownloadRecord, DownloadStatus, Episode, MonitorEpisodeBody, Season, Series, SonarrHistoryItem,
    SonarrHistoryWrapper, SonarrRelease, SonarrSerdeable,
  };
  use crate::models::stateful_table::SortOption;
  use crate::network::NetworkResource;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::sonarr_network::SonarrEvent;
  use crate::network::sonarr_network::library::episodes::get_episode_status;
  use crate::network::sonarr_network::sonarr_network_test_utils::test_utils::{
    EPISODE_JSON, episode, episode_file, sonarr_history_item, torrent_release,
  };
  use indoc::formatdoc;
  use mockito::Matcher;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use serde_json::{Number, json};
  use std::slice;

  #[tokio::test]
  async fn test_handle_delete_sonarr_episode_file_event() {
    let (async_server, app_arc, _server) = MockServarrApi::delete()
      .path("/1")
      .build_for(SonarrEvent::DeleteEpisodeFile(1))
      .await;
    app_arc.lock().await.data.sonarr_data.season_details_modal =
      Some(SeasonDetailsModal::default());
    app_arc.lock().await.server_tabs.next();
    let mut network = test_network(&app_arc);

    assert!(
      network
        .handle_sonarr_event(SonarrEvent::DeleteEpisodeFile(1))
        .await
        .is_ok()
    );

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
    let (mock, app, _server) = MockServarrApi::get()
      .query("seriesId=1")
      .returns(json!([episode_1, episode_2, episode_3]))
      .build_for(SonarrEvent::GetEpisodes(1))
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
    app.lock().await.data.sonarr_data.season_details_modal = Some(season_details_modal);
    app
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .set_items(vec![Series {
        id: 1,
        ..Series::default()
      }]);
    app
      .lock()
      .await
      .data
      .sonarr_data
      .seasons
      .set_items(vec![Season {
        season_number: 1,
        ..Season::default()
      }]);
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    let result = network
      .handle_sonarr_event(SonarrEvent::GetEpisodes(1))
      .await;

    mock.assert_async().await;

    let SonarrSerdeable::Episodes(episodes) = result.unwrap() else {
      panic!("Expected Episodes variant")
    };
    assert_eq!(
      app
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
      app
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
    let (mock, app, _server) = MockServarrApi::get()
      .query("seriesId=1")
      .returns(json!([episode_1, episode_2, episode_3]))
      .build_for(SonarrEvent::GetEpisodes(1))
      .await;
    let mut season_details_modal = SeasonDetailsModal::default();
    season_details_modal.episodes.sort_asc = true;
    app.lock().await.data.sonarr_data.season_details_modal = Some(season_details_modal);
    app
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .set_items(vec![Series {
        id: 1,
        ..Series::default()
      }]);
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    let result = network
      .handle_sonarr_event(SonarrEvent::GetEpisodes(1))
      .await;

    mock.assert_async().await;

    let SonarrSerdeable::Episodes(episodes) = result.unwrap() else {
      panic!("Expected Episodes variant")
    };
    assert_eq!(
      app
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
      app
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

  #[tokio::test]
  async fn test_handle_get_episodes_event_empty_season_details_modal() {
    let (async_server, app_arc, _server) = MockServarrApi::get()
      .returns(json!([episode()]))
      .query("seriesId=1")
      .build_for(SonarrEvent::GetEpisodes(1))
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
    app_arc.lock().await.server_tabs.next();
    let mut network = test_network(&app_arc);

    let SonarrSerdeable::Episodes(episodes) = network
      .handle_sonarr_event(SonarrEvent::GetEpisodes(1))
      .await
      .unwrap()
    else {
      panic!("Expected Episodes")
    };
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
    let (async_server, app_arc, _server) = MockServarrApi::get()
      .returns(episodes_json)
      .query("seriesId=1")
      .build_for(SonarrEvent::GetEpisodes(1))
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
    app_arc.lock().await.server_tabs.next();
    let mut network = test_network(&app_arc);

    let SonarrSerdeable::Episodes(episodes) = network
      .handle_sonarr_event(SonarrEvent::GetEpisodes(1))
      .await
      .unwrap()
    else {
      panic!("Expected Episodes")
    };
    async_server.assert_async().await;
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
        .is_empty()
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

  #[tokio::test]
  async fn test_handle_get_episode_files_event() {
    let (async_server, app_arc, _server) = MockServarrApi::get()
      .returns(json!([episode_file()]))
      .query("seriesId=1")
      .build_for(SonarrEvent::GetEpisodeFiles(1))
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
    app_arc.lock().await.server_tabs.next();
    let mut network = test_network(&app_arc);

    let SonarrSerdeable::EpisodeFiles(episode_files) = network
      .handle_sonarr_event(SonarrEvent::GetEpisodeFiles(1))
      .await
      .unwrap()
    else {
      panic!("Expected EpisodeFiles")
    };
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

  #[tokio::test]
  async fn test_handle_get_episode_files_event_empty_season_details_modal() {
    let (async_server, app_arc, _server) = MockServarrApi::get()
      .returns(json!([episode_file()]))
      .query("seriesId=1")
      .build_for(SonarrEvent::GetEpisodeFiles(1))
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
    app_arc.lock().await.server_tabs.next();
    let mut network = test_network(&app_arc);

    let SonarrSerdeable::EpisodeFiles(episode_files) = network
      .handle_sonarr_event(SonarrEvent::GetEpisodeFiles(1))
      .await
      .unwrap()
    else {
      panic!("Expected EpisodeFiles")
    };
    async_server.assert_async().await;
    assert!(
      app_arc
        .lock()
        .await
        .data
        .sonarr_data
        .season_details_modal
        .is_some()
    );
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
        ..sonarr_history_item()
      },
      SonarrHistoryItem {
        id: 456,
        episode_id: 2001,
        source_title: "A Episode".into(),
        ..sonarr_history_item()
      },
    ];
    let (async_server, app_arc, _server) = MockServarrApi::get()
      .returns(history_json)
      .query("episodeId=1&pageSize=1000&sortDirection=descending&sortKey=date")
      .build_for(SonarrEvent::GetEpisodeHistory(1))
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
    app_arc.lock().await.server_tabs.next();
    let mut network = test_network(&app_arc);

    let SonarrSerdeable::SonarrHistoryWrapper(history) = network
      .handle_sonarr_event(SonarrEvent::GetEpisodeHistory(1))
      .await
      .unwrap()
    else {
      panic!("Expected SonarrHistoryWrapper")
    };
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
        ..sonarr_history_item()
      },
      SonarrHistoryItem {
        id: 456,
        episode_id: 2001,
        source_title: "A Episode".into(),
        ..sonarr_history_item()
      },
    ];
    let (async_server, app_arc, _server) = MockServarrApi::get()
      .returns(history_json)
      .query("episodeId=1&pageSize=1000&sortDirection=descending&sortKey=date")
      .build_for(SonarrEvent::GetEpisodeHistory(1))
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
    app_arc.lock().await.server_tabs.next();
    let mut network = test_network(&app_arc);

    let SonarrSerdeable::SonarrHistoryWrapper(history) = network
      .handle_sonarr_event(SonarrEvent::GetEpisodeHistory(1))
      .await
      .unwrap()
    else {
      panic!("Expected SonarrHistoryWrapper")
    };
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
        ..sonarr_history_item()
      },
      SonarrHistoryItem {
        id: 456,
        episode_id: 2001,
        source_title: "A Episode".into(),
        ..sonarr_history_item()
      },
    ];
    let (async_server, app_arc, _server) = MockServarrApi::get()
      .returns(history_json)
      .query("episodeId=1&pageSize=1000&sortDirection=descending&sortKey=date")
      .build_for(SonarrEvent::GetEpisodeHistory(1))
      .await;
    app_arc.lock().await.server_tabs.next();
    let mut network = test_network(&app_arc);

    let SonarrSerdeable::SonarrHistoryWrapper(history) = network
      .handle_sonarr_event(SonarrEvent::GetEpisodeHistory(1))
      .await
      .unwrap()
    else {
      panic!("Expected SonarrHistoryWrapper")
    };
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

  #[tokio::test]
  async fn test_handle_get_episode_details_event() {
    let response: Episode = serde_json::from_str(EPISODE_JSON).unwrap();
    let (async_server, app_arc, _server) = MockServarrApi::get()
      .returns(serde_json::from_str(EPISODE_JSON).unwrap())
      .path("/1")
      .build_for(SonarrEvent::GetEpisodeDetails(1))
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
    app_arc.lock().await.server_tabs.next();
    let mut network = test_network(&app_arc);

    let SonarrSerdeable::Episode(episode) = network
      .handle_sonarr_event(SonarrEvent::GetEpisodeDetails(1))
      .await
      .unwrap()
    else {
      panic!("Expected Episode")
    };
    async_server.assert_async().await;
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
        .is_some()
    );
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

  #[tokio::test]
  async fn test_handle_get_episode_details_event_empty_episode_details_modal() {
    let response: Episode = serde_json::from_str(EPISODE_JSON).unwrap();
    let (async_server, app_arc, _server) = MockServarrApi::get()
      .returns(serde_json::from_str(EPISODE_JSON).unwrap())
      .path("/1")
      .build_for(SonarrEvent::GetEpisodeDetails(1))
      .await;
    let mut season_details_modal = SeasonDetailsModal::default();
    season_details_modal.episodes.set_items(vec![episode()]);
    app_arc.lock().await.data.sonarr_data.season_details_modal = Some(season_details_modal);
    app_arc
      .lock()
      .await
      .push_navigation_stack(ActiveSonarrBlock::EpisodeDetails.into());
    app_arc.lock().await.server_tabs.next();
    let mut network = test_network(&app_arc);

    let SonarrSerdeable::Episode(episode) = network
      .handle_sonarr_event(SonarrEvent::GetEpisodeDetails(1))
      .await
      .unwrap()
    else {
      panic!("Expected Episode")
    };
    async_server.assert_async().await;
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
        .is_some()
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

  #[tokio::test]
  async fn test_handle_get_episode_details_event_season_details_modal_not_required_in_cli_mode() {
    let response: Episode = serde_json::from_str(EPISODE_JSON).unwrap();
    let (async_server, app_arc, _server) = MockServarrApi::get()
      .returns(serde_json::from_str(EPISODE_JSON).unwrap())
      .path("/1")
      .build_for(SonarrEvent::GetEpisodeDetails(1))
      .await;
    app_arc.lock().await.cli_mode = true;
    app_arc.lock().await.server_tabs.next();
    let mut network = test_network(&app_arc);

    let SonarrSerdeable::Episode(episode) = network
      .handle_sonarr_event(SonarrEvent::GetEpisodeDetails(1))
      .await
      .unwrap()
    else {
      panic!("Expected Episode")
    };
    async_server.assert_async().await;
    assert_eq!(episode, response);
  }

  #[tokio::test]
  #[should_panic(expected = "Season details modal is empty")]
  async fn test_handle_get_episode_details_event_requires_season_details_modal_to_be_some_when_in_tui_mode()
   {
    let (_async_server, app_arc, _server) = MockServarrApi::get()
      .returns(serde_json::from_str(EPISODE_JSON).unwrap())
      .path("/1")
      .build_for(SonarrEvent::GetEpisodeDetails(1))
      .await;
    app_arc.lock().await.server_tabs.next();
    let mut network = test_network(&app_arc);

    network
      .handle_sonarr_event(SonarrEvent::GetEpisodeDetails(1))
      .await
      .unwrap();
  }

  #[tokio::test]
  async fn test_handle_get_episode_releases_event() {
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
      guid: "4567".to_owned(),
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
    let (async_server, app_arc, _server) = MockServarrApi::get()
      .returns(release_json)
      .query("episodeId=1")
      .build_for(SonarrEvent::GetEpisodeReleases(1))
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
    app_arc.lock().await.server_tabs.next();
    let mut network = test_network(&app_arc);

    let SonarrSerdeable::Releases(releases_vec) = network
      .handle_sonarr_event(SonarrEvent::GetEpisodeReleases(1))
      .await
      .unwrap()
    else {
      panic!("Expected Releases")
    };
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
      vec![expected_filtered_sonarr_release]
    );
    assert_eq!(releases_vec, expected_raw_sonarr_releases);
  }

  #[tokio::test]
  async fn test_handle_get_episode_releases_event_empty_episode_details_modal() {
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
      guid: "4567".to_owned(),
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
    let (async_server, app_arc, _server) = MockServarrApi::get()
      .returns(release_json)
      .query("episodeId=1")
      .build_for(SonarrEvent::GetEpisodeReleases(1))
      .await;
    let mut season_details_modal = SeasonDetailsModal::default();
    season_details_modal.episodes.set_items(vec![episode()]);
    app_arc.lock().await.data.sonarr_data.season_details_modal = Some(season_details_modal);
    app_arc.lock().await.server_tabs.next();
    let mut network = test_network(&app_arc);

    let SonarrSerdeable::Releases(releases_vec) = network
      .handle_sonarr_event(SonarrEvent::GetEpisodeReleases(1))
      .await
      .unwrap()
    else {
      panic!("Expected Releases")
    };
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
      vec![expected_filtered_sonarr_release]
    );
    assert_eq!(releases_vec, expected_raw_sonarr_releases);
  }

  #[tokio::test]
  async fn test_handle_toggle_episode_monitoring_event() {
    let expected_body = MonitorEpisodeBody {
      episode_ids: vec![2],
      monitored: false,
    };
    let body = Episode { id: 2, ..episode() };

    let (async_details_server, app_arc, mut server) = MockServarrApi::get()
      .returns(json!(body))
      .path("/2")
      .build_for(SonarrEvent::GetEpisodeDetails(2))
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
    app_arc.lock().await.server_tabs.next();
    let mut network = test_network(&app_arc);

    assert!(
      network
        .handle_sonarr_event(SonarrEvent::ToggleEpisodeMonitoring(2))
        .await
        .is_ok()
    );

    async_details_server.assert_async().await;
    async_toggle_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_trigger_automatic_episode_search_event() {
    let body = json!({
      "name": "EpisodeSearch",
      "episodeIds": [ 1 ]
    });
    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(body)
      .returns(json!({}))
      .build_for(SonarrEvent::TriggerAutomaticEpisodeSearch(1))
      .await;
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    assert!(
      network
        .handle_sonarr_event(SonarrEvent::TriggerAutomaticEpisodeSearch(1))
        .await
        .is_ok()
    );

    mock.assert_async().await;
  }

  #[test]
  fn test_get_episode_status_downloaded() {
    assert_str_eq!(get_episode_status(true, &[], 0), "Downloaded");
  }

  #[test]
  fn test_get_episode_status_missing() {
    let download_record = DownloadRecord {
      episode_id: Some(Number::from(1i64)),
      ..DownloadRecord::default()
    };

    assert_str_eq!(
      get_episode_status(false, slice::from_ref(&download_record), 0),
      "Missing"
    );

    assert_str_eq!(get_episode_status(false, &[download_record], 1), "Missing");
  }

  #[test]
  fn test_get_episode_status_missing_if_episode_id_is_missing() {
    let download_record = DownloadRecord::default();

    assert_str_eq!(
      get_episode_status(false, slice::from_ref(&download_record), 0),
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
          episode_id: Some(Number::from(1i64)),
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
          episode_id: Some(Number::from(1i64)),
          status: DownloadStatus::Completed,
          ..DownloadRecord::default()
        }],
        1
      ),
      "Awaiting Import"
    );
  }
}
