#[cfg(test)]
mod tests {
  use crate::models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock;
  use crate::models::sonarr_models::{
    AddSeriesBody, AddSeriesOptions, DeleteSeriesParams, EditSeriesParams, Series, SeriesMonitor,
    SeriesType, SonarrHistoryItem, SonarrSerdeable,
  };
  use crate::models::stateful_table::{SortOption, StatefulTable};
  use crate::network::NetworkResource;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::sonarr_network::SonarrEvent;
  use crate::network::sonarr_network::sonarr_network_test_utils::test_utils::{
    SERIES_JSON, add_series_search_result, history_item, season, series,
  };
  use bimap::BiMap;
  use mockito::Matcher;
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use serde_json::{Value, json};

  #[tokio::test]
  async fn test_handle_add_sonarr_series_event() {
    let expected_add_series_body = AddSeriesBody {
      tvdb_id: 1234,
      title: "Test".to_owned(),
      monitored: true,
      root_folder_path: "/nfs2".to_owned(),
      quality_profile_id: 2222,
      language_profile_id: 2222,
      series_type: SeriesType::Standard,
      season_folder: true,
      tags: Vec::new(),
      tag_input_string: Some("usenet, testing".to_owned()),
      add_options: AddSeriesOptions {
        monitor: SeriesMonitor::All,
        search_for_cutoff_unmet_episodes: true,
        search_for_missing_episodes: true,
      },
    };
    let (async_server, app, _server) = MockServarrApi::post()
      .with_request_body(json!({
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
      }))
      .returns(json!({}))
      .build_for(SonarrEvent::AddSeries(expected_add_series_body.clone()))
      .await;
    app.lock().await.data.sonarr_data.tags_map =
      BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    assert!(
      network
        .handle_sonarr_event(SonarrEvent::AddSeries(expected_add_series_body))
        .await
        .is_ok()
    );

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_add_sonarr_series_event_does_not_overwrite_tags_vec_when_tag_input_string_is_none()
   {
    let expected_add_series_body = AddSeriesBody {
      tvdb_id: 1234,
      title: "Test".to_owned(),
      monitored: true,
      root_folder_path: "/nfs2".to_owned(),
      quality_profile_id: 2222,
      language_profile_id: 2222,
      series_type: SeriesType::Standard,
      season_folder: true,
      tags: vec![1, 2],
      tag_input_string: None,
      add_options: AddSeriesOptions {
        monitor: SeriesMonitor::All,
        search_for_cutoff_unmet_episodes: true,
        search_for_missing_episodes: true,
      },
    };
    let (async_server, app, _server) = MockServarrApi::post()
      .with_request_body(json!({
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
      }))
      .returns(json!({}))
      .build_for(SonarrEvent::AddSeries(expected_add_series_body.clone()))
      .await;
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    assert!(
      network
        .handle_sonarr_event(SonarrEvent::AddSeries(expected_add_series_body))
        .await
        .is_ok()
    );

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_delete_series_event() {
    let delete_series_params = DeleteSeriesParams {
      id: 1,
      delete_series_files: true,
      add_list_exclusion: true,
    };
    let (async_server, app, _server) = MockServarrApi::delete()
      .path("/1")
      .query("deleteFiles=true&addImportExclusion=true")
      .build_for(SonarrEvent::DeleteSeries(delete_series_params.clone()))
      .await;
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    assert!(
      network
        .handle_sonarr_event(SonarrEvent::DeleteSeries(delete_series_params))
        .await
        .is_ok()
    );

    async_server.assert_async().await;
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

    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(serde_json::from_str(SERIES_JSON).unwrap())
      .path("/1")
      .build_for(SonarrEvent::GetSeriesDetails(1))
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
    app.lock().await.data.sonarr_data.tags_map =
      BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    assert!(
      network
        .handle_sonarr_event(SonarrEvent::EditSeries(edit_series_params))
        .await
        .is_ok()
    );

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_series_event_does_not_overwrite_tag_ids_vec_when_tag_input_string_is_none()
   {
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

    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(serde_json::from_str(SERIES_JSON).unwrap())
      .path("/1")
      .build_for(SonarrEvent::GetSeriesDetails(1))
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
    app.lock().await.data.sonarr_data.tags_map =
      BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    assert!(
      network
        .handle_sonarr_event(SonarrEvent::EditSeries(edit_series_params))
        .await
        .is_ok()
    );

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
    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(serde_json::from_str(SERIES_JSON).unwrap())
      .path("/1")
      .build_for(SonarrEvent::GetSeriesDetails(1))
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
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    assert!(
      network
        .handle_sonarr_event(SonarrEvent::EditSeries(edit_series_params))
        .await
        .is_ok()
    );

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_series_event_returns_empty_tags_vec_when_clear_tags_is_true() {
    let mut expected_body: Value = serde_json::from_str(SERIES_JSON).unwrap();
    *expected_body.get_mut("tags").unwrap() = json!([]);

    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(serde_json::from_str(SERIES_JSON).unwrap())
      .path("/1")
      .build_for(SonarrEvent::GetSeriesDetails(1))
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
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    assert!(
      network
        .handle_sonarr_event(SonarrEvent::EditSeries(edit_series_params))
        .await
        .is_ok()
    );

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_toggle_series_monitoring_event() {
    let mut expected_body: Value = serde_json::from_str(SERIES_JSON).unwrap();
    *expected_body.get_mut("monitored").unwrap() = json!(false);

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
          SonarrEvent::ToggleSeriesMonitoring(1).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .create_async()
      .await;
    {
      let mut app = app.lock().await;
      app.data.sonarr_data.series.set_items(vec![series()]);
      app.data.sonarr_data.seasons.set_items(vec![season()]);
    }
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    assert!(
      network
        .handle_sonarr_event(SonarrEvent::ToggleSeriesMonitoring(1))
        .await
        .is_ok()
    );

    async_details_server.assert_async().await;
    async_toggle_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_get_series_details_event() {
    let expected_series: Series = serde_json::from_str(SERIES_JSON).unwrap();
    let (async_server, app, _server) = MockServarrApi::get()
      .returns(serde_json::from_str(SERIES_JSON).unwrap())
      .path("/1")
      .build_for(SonarrEvent::GetSeriesDetails(1))
      .await;
    app
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .set_items(vec![series()]);
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    let SonarrSerdeable::Series(series) = network
      .handle_sonarr_event(SonarrEvent::GetSeriesDetails(1))
      .await
      .unwrap()
    else {
      panic!("Expected Series")
    };
    async_server.assert_async().await;
    assert_eq!(series, expected_series);
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
    let (async_server, app, _server) = MockServarrApi::get()
      .returns(history_json)
      .query("seriesId=1")
      .build_for(SonarrEvent::GetSeriesHistory(1))
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
    app
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .set_items(vec![series()]);
    app.lock().await.data.sonarr_data.series_history = Some(series_history_table);
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    let SonarrSerdeable::SonarrHistoryItems(history_items) = network
      .handle_sonarr_event(SonarrEvent::GetSeriesHistory(1))
      .await
      .unwrap()
    else {
      panic!("Expected SonarrHistoryItems")
    };
    async_server.assert_async().await;
    assert!(app.lock().await.data.sonarr_data.series_history.is_some());
    assert_eq!(
      app
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
      app
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
    let (async_server, app, _server) = MockServarrApi::get()
      .returns(history_json)
      .query("seriesId=1")
      .build_for(SonarrEvent::GetSeriesHistory(1))
      .await;
    app
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .set_items(vec![series()]);
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    let SonarrSerdeable::SonarrHistoryItems(history_items) = network
      .handle_sonarr_event(SonarrEvent::GetSeriesHistory(1))
      .await
      .unwrap()
    else {
      panic!("Expected SonarrHistoryItems")
    };
    async_server.assert_async().await;
    assert!(app.lock().await.data.sonarr_data.series_history.is_some());
    assert_eq!(
      app
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
      !app
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
    let (async_server, app, _server) = MockServarrApi::get()
      .returns(history_json)
      .query("seriesId=1")
      .build_for(SonarrEvent::GetSeriesHistory(1))
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
    app.lock().await.data.sonarr_data.series_history = Some(series_history_table);
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
      .push_navigation_stack(ActiveSonarrBlock::SeriesHistorySortPrompt.into());
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    let SonarrSerdeable::SonarrHistoryItems(history_items) = network
      .handle_sonarr_event(SonarrEvent::GetSeriesHistory(1))
      .await
      .unwrap()
    else {
      panic!("Expected SonarrHistoryItems")
    };
    async_server.assert_async().await;
    assert!(app.lock().await.data.sonarr_data.series_history.is_some());
    assert!(
      app
        .lock()
        .await
        .data
        .sonarr_data
        .series_history
        .as_ref()
        .unwrap()
        .is_empty()
    );
    assert!(
      app
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
    let (async_server, app, _server) = MockServarrApi::get()
      .returns(json!([series_1, series_2]))
      .build_for(SonarrEvent::ListSeries)
      .await;
    app.lock().await.data.sonarr_data.series.sort_asc = true;
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
      app
        .lock()
        .await
        .data
        .sonarr_data
        .series
        .sorting(vec![title_sort_option]);
    }
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    let SonarrSerdeable::SeriesVec(series) = network
      .handle_sonarr_event(SonarrEvent::ListSeries)
      .await
      .unwrap()
    else {
      panic!("Expected SeriesVec")
    };
    async_server.assert_async().await;
    assert_eq!(
      app.lock().await.data.sonarr_data.series.items,
      expected_sorted_series
    );
    assert!(app.lock().await.data.sonarr_data.series.sort_asc);
    assert_eq!(series, expected_series);
  }

  #[tokio::test]
  async fn test_handle_list_series_event_no_op_while_user_is_selecting_sort_options() {
    let mut series_1: Value = serde_json::from_str(SERIES_JSON).unwrap();
    let mut series_2: Value = serde_json::from_str(SERIES_JSON).unwrap();
    *series_1.get_mut("id").unwrap() = json!(1);
    *series_1.get_mut("title").unwrap() = json!("z test");
    *series_2.get_mut("id").unwrap() = json!(2);
    *series_2.get_mut("title").unwrap() = json!("A test");
    let (async_server, app, _server) = MockServarrApi::get()
      .returns(json!([series_1, series_2]))
      .build_for(SonarrEvent::ListSeries)
      .await;
    app
      .lock()
      .await
      .push_navigation_stack(ActiveSonarrBlock::SeriesSortPrompt.into());
    app.lock().await.data.sonarr_data.series.sort_asc = true;
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
    app
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .sorting(vec![title_sort_option]);
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    assert!(
      network
        .handle_sonarr_event(SonarrEvent::ListSeries)
        .await
        .is_ok()
    );

    async_server.assert_async().await;
    assert!(app.lock().await.data.sonarr_data.series.items.is_empty());
    assert!(app.lock().await.data.sonarr_data.series.sort_asc);
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
    let (async_server, app, _server) = MockServarrApi::get()
      .returns(add_series_search_result_json)
      .query("term=test%20term")
      .build_for(SonarrEvent::SearchNewSeries("test term".into()))
      .await;
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    let SonarrSerdeable::AddSeriesSearchResults(add_series_search_results) = network
      .handle_sonarr_event(SonarrEvent::SearchNewSeries("test term".into()))
      .await
      .unwrap()
    else {
      panic!("Expected AddSeriesSearchResults")
    };
    async_server.assert_async().await;
    assert!(
      app
        .lock()
        .await
        .data
        .sonarr_data
        .add_searched_series
        .is_some()
    );
    assert_eq!(
      app
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

  #[tokio::test]
  async fn test_handle_search_new_series_event_no_results() {
    let (async_server, app, _server) = MockServarrApi::get()
      .returns(json!([]))
      .query("term=test%20term")
      .build_for(SonarrEvent::SearchNewSeries("test term".into()))
      .await;
    app.lock().await.data.sonarr_data.add_series_search = Some("test term".into());
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    assert!(
      network
        .handle_sonarr_event(SonarrEvent::SearchNewSeries("test term".into()))
        .await
        .is_ok()
    );

    async_server.assert_async().await;
    assert!(
      app
        .lock()
        .await
        .data
        .sonarr_data
        .add_searched_series
        .is_none()
    );
    assert_eq!(
      app.lock().await.get_current_route(),
      ActiveSonarrBlock::AddSeriesEmptySearchResults.into()
    );
  }

  #[tokio::test]
  async fn test_handle_search_new_series_event_sets_empty_table_on_api_error() {
    let (async_server, app, _server) = MockServarrApi::get()
      .status(500)
      .query("term=test%20term")
      .build_for(SonarrEvent::SearchNewSeries("test term".into()))
      .await;
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    let result = network
      .handle_sonarr_event(SonarrEvent::SearchNewSeries("test term".into()))
      .await;

    async_server.assert_async().await;
    assert_err!(result);
    let app = app.lock().await;
    assert_some!(&app.data.sonarr_data.add_searched_series);
    assert_is_empty!(app.data.sonarr_data.add_searched_series.as_ref().unwrap());
  }

  #[tokio::test]
  async fn test_handle_trigger_automatic_series_search_event() {
    let (async_server, app, _server) = MockServarrApi::post()
      .with_request_body(json!({
        "name": "SeriesSearch",
        "seriesId": 1
      }))
      .returns(json!({}))
      .build_for(SonarrEvent::TriggerAutomaticSeriesSearch(1))
      .await;
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    assert!(
      network
        .handle_sonarr_event(SonarrEvent::TriggerAutomaticSeriesSearch(1))
        .await
        .is_ok()
    );

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_update_all_series_event() {
    let (async_server, app, _server) = MockServarrApi::post()
      .with_request_body(json!({
        "name": "RefreshSeries",
      }))
      .returns(json!({}))
      .build_for(SonarrEvent::UpdateAllSeries)
      .await;
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    assert!(
      network
        .handle_sonarr_event(SonarrEvent::UpdateAllSeries)
        .await
        .is_ok()
    );

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_update_and_scan_series_event() {
    let (async_server, app, _server) = MockServarrApi::post()
      .with_request_body(json!({
        "name": "RefreshSeries",
        "seriesId": 1,
      }))
      .returns(json!({}))
      .build_for(SonarrEvent::UpdateAndScanSeries(1))
      .await;
    app
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .set_items(vec![series()]);
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    assert!(
      network
        .handle_sonarr_event(SonarrEvent::UpdateAndScanSeries(1))
        .await
        .is_ok()
    );

    async_server.assert_async().await;
  }
}
