#[cfg(test)]
mod tests {
  use crate::models::lidarr_models::{
    AddArtistBody, AddArtistOptions, AddArtistSearchResult, Artist, DeleteArtistParams,
    EditArtistParams, LidarrSerdeable, MonitorType, NewItemMonitorType,
  };
  use crate::models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock;
  use crate::network::NetworkResource;
  use crate::network::lidarr_network::LidarrEvent;
  use crate::network::lidarr_network::lidarr_network_test_utils::test_utils::{
    ADD_ARTIST_SEARCH_RESULT_JSON, ARTIST_JSON,
  };
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use bimap::BiMap;
  use mockito::Matcher;
  use pretty_assertions::assert_eq;
  use serde_json::{Value, json};

  #[tokio::test]
  async fn test_handle_list_artists_event() {
    let artists_json = json!([{
      "id": 1,
      "artistName": "Test Artist",
      "foreignArtistId": "test-foreign-id",
      "status": "continuing",
      "path": "/music/test-artist",
      "qualityProfileId": 1,
      "metadataProfileId": 1,
      "monitored": true,
      "monitorNewItems": "all",
      "genres": [],
      "tags": [],
      "added": "2023-01-01T00:00:00Z"
    }]);
    let response: Vec<Artist> = serde_json::from_value(artists_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(artists_json)
      .build_for(LidarrEvent::ListArtists)
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let result = network.handle_lidarr_event(LidarrEvent::ListArtists).await;

    mock.assert_async().await;

    let LidarrSerdeable::Artists(artists) = result.unwrap() else {
      panic!("Expected Artists");
    };

    assert_eq!(artists, response);
    assert!(!app.lock().await.data.lidarr_data.artists.is_empty());
  }

  #[tokio::test]
  async fn test_handle_delete_artist_event() {
    let delete_artist_params = DeleteArtistParams {
      id: 1,
      delete_files: true,
      add_import_list_exclusion: true,
    };
    let (async_server, app, _server) = MockServarrApi::delete()
      .path("/1")
      .query("deleteFiles=true&addImportListExclusion=true")
      .build_for(LidarrEvent::DeleteArtist(delete_artist_params.clone()))
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    assert!(
      network
        .handle_lidarr_event(LidarrEvent::DeleteArtist(delete_artist_params))
        .await
        .is_ok()
    );

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_get_artist_details_event() {
    let expected_artist: Artist = serde_json::from_str(ARTIST_JSON).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(serde_json::from_str(ARTIST_JSON).unwrap())
      .path("/1")
      .build_for(LidarrEvent::GetArtistDetails(1))
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let result = network
      .handle_lidarr_event(LidarrEvent::GetArtistDetails(1))
      .await;

    mock.assert_async().await;

    let LidarrSerdeable::Artist(artist) = result.unwrap() else {
      panic!("Expected Artist");
    };

    assert_eq!(artist, expected_artist);
  }

  #[tokio::test]
  async fn test_handle_toggle_artist_monitoring_event() {
    let artist_json = json!({
      "id": 1,
      "artistName": "Test Artist",
      "foreignArtistId": "test-foreign-id",
      "status": "continuing",
      "path": "/music/test-artist",
      "qualityProfileId": 1,
      "metadataProfileId": 1,
      "monitored": true,
      "monitorNewItems": "all",
      "genres": [],
      "tags": [],
      "added": "2023-01-01T00:00:00Z"
    });
    let mut expected_body = artist_json.clone();
    *expected_body.get_mut("monitored").unwrap() = json!(false);
    let (get_mock, app, mut server) = MockServarrApi::get()
      .returns(artist_json)
      .path("/1")
      .build_for(LidarrEvent::GetArtistDetails(1))
      .await;
    let put_mock = server
      .mock("PUT", "/api/v1/artist/1")
      .match_body(Matcher::Json(expected_body))
      .match_header("X-Api-Key", "test1234")
      .with_status(202)
      .create_async()
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    assert!(
      network
        .handle_lidarr_event(LidarrEvent::ToggleArtistMonitoring(1))
        .await
        .is_ok()
    );

    get_mock.assert_async().await;
    put_mock.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_update_all_artists_event() {
    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(json!({
        "name": "RefreshArtist"
      }))
      .returns(json!({}))
      .build_for(LidarrEvent::UpdateAllArtists)
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    assert!(
      network
        .handle_lidarr_event(LidarrEvent::UpdateAllArtists)
        .await
        .is_ok()
    );

    mock.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_artist_event() {
    let mut expected_body: Value = serde_json::from_str(ARTIST_JSON).unwrap();
    *expected_body.get_mut("monitored").unwrap() = json!(false);
    *expected_body.get_mut("monitorNewItems").unwrap() = json!("none");
    *expected_body.get_mut("qualityProfileId").unwrap() = json!(1111);
    *expected_body.get_mut("metadataProfileId").unwrap() = json!(2222);
    *expected_body.get_mut("path").unwrap() = json!("/nfs/Test Path");
    *expected_body.get_mut("tags").unwrap() = json!([1, 2]);
    let edit_artist_params = EditArtistParams {
      artist_id: 1,
      monitored: Some(false),
      monitor_new_items: Some(NewItemMonitorType::None),
      quality_profile_id: Some(1111),
      metadata_profile_id: Some(2222),
      root_folder_path: Some("/nfs/Test Path".to_owned()),
      tag_input_string: Some("usenet, testing".to_owned()),
      ..EditArtistParams::default()
    };

    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(serde_json::from_str(ARTIST_JSON).unwrap())
      .path("/1")
      .build_for(LidarrEvent::GetArtistDetails(1))
      .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v1{}/1",
          LidarrEvent::EditArtist(edit_artist_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .create_async()
      .await;
    app.lock().await.data.lidarr_data.tags_map =
      BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    assert!(
      network
        .handle_lidarr_event(LidarrEvent::EditArtist(edit_artist_params))
        .await
        .is_ok()
    );

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_artist_event_does_not_overwrite_tag_ids_vec_when_tag_input_string_is_none()
   {
    let mut expected_body: Value = serde_json::from_str(ARTIST_JSON).unwrap();
    *expected_body.get_mut("monitored").unwrap() = json!(false);
    *expected_body.get_mut("monitorNewItems").unwrap() = json!("none");
    *expected_body.get_mut("qualityProfileId").unwrap() = json!(1111);
    *expected_body.get_mut("metadataProfileId").unwrap() = json!(2222);
    *expected_body.get_mut("path").unwrap() = json!("/nfs/Test Path");
    *expected_body.get_mut("tags").unwrap() = json!([1, 2]);
    let edit_artist_params = EditArtistParams {
      artist_id: 1,
      monitored: Some(false),
      monitor_new_items: Some(NewItemMonitorType::None),
      quality_profile_id: Some(1111),
      metadata_profile_id: Some(2222),
      root_folder_path: Some("/nfs/Test Path".to_owned()),
      tags: Some(vec![1, 2]),
      ..EditArtistParams::default()
    };

    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(serde_json::from_str(ARTIST_JSON).unwrap())
      .path("/1")
      .build_for(LidarrEvent::GetArtistDetails(1))
      .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v1{}/1",
          LidarrEvent::EditArtist(edit_artist_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .create_async()
      .await;
    app.lock().await.data.lidarr_data.tags_map =
      BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    assert!(
      network
        .handle_lidarr_event(LidarrEvent::EditArtist(edit_artist_params))
        .await
        .is_ok()
    );

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_artist_event_defaults_to_previous_values() {
    let edit_artist_params = EditArtistParams {
      artist_id: 1,
      ..EditArtistParams::default()
    };
    let expected_body: Value = serde_json::from_str(ARTIST_JSON).unwrap();
    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(serde_json::from_str(ARTIST_JSON).unwrap())
      .path("/1")
      .build_for(LidarrEvent::GetArtistDetails(1))
      .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v1{}/1",
          LidarrEvent::EditArtist(edit_artist_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .create_async()
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    assert!(
      network
        .handle_lidarr_event(LidarrEvent::EditArtist(edit_artist_params))
        .await
        .is_ok()
    );

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_artist_event_returns_empty_tags_vec_when_clear_tags_is_true() {
    let mut expected_body: Value = serde_json::from_str(ARTIST_JSON).unwrap();
    *expected_body.get_mut("tags").unwrap() = json!([]);

    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(serde_json::from_str(ARTIST_JSON).unwrap())
      .path("/1")
      .build_for(LidarrEvent::GetArtistDetails(1))
      .await;
    let edit_artist_params = EditArtistParams {
      artist_id: 1,
      clear_tags: true,
      ..EditArtistParams::default()
    };
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v1{}/1",
          LidarrEvent::EditArtist(edit_artist_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .create_async()
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    assert!(
      network
        .handle_lidarr_event(LidarrEvent::EditArtist(edit_artist_params))
        .await
        .is_ok()
    );

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_search_new_artist_event() {
    let search_results_json =
      json!([serde_json::from_str::<Value>(ADD_ARTIST_SEARCH_RESULT_JSON).unwrap()]);
    let expected_results: Vec<AddArtistSearchResult> =
      serde_json::from_value(search_results_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(search_results_json)
      .query("term=test%20artist")
      .build_for(LidarrEvent::SearchNewArtist("test artist".to_owned()))
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let result = network
      .handle_lidarr_event(LidarrEvent::SearchNewArtist("test artist".to_owned()))
      .await;

    mock.assert_async().await;

    let LidarrSerdeable::AddArtistSearchResults(search_results) = result.unwrap() else {
      panic!("Expected AddArtistSearchResults");
    };

    assert_eq!(search_results, expected_results);
    assert_some!(&app.lock().await.data.lidarr_data.add_searched_artists);
  }

  #[tokio::test]
  async fn test_handle_search_new_artist_event_navigates_to_empty_results_when_empty() {
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!([]))
      .query("term=nonexistent")
      .build_for(LidarrEvent::SearchNewArtist("nonexistent".to_owned()))
      .await;
    app.lock().await.server_tabs.set_index(2);
    app
      .lock()
      .await
      .push_navigation_stack(ActiveLidarrBlock::AddArtistSearchResults.into());
    let mut network = test_network(&app);

    let result = network
      .handle_lidarr_event(LidarrEvent::SearchNewArtist("nonexistent".to_owned()))
      .await;

    mock.assert_async().await;

    assert_ok!(result);
    let app = app.lock().await;
    assert_none!(&app.data.lidarr_data.add_searched_artists);
    assert_eq!(
      app.get_current_route(),
      ActiveLidarrBlock::AddArtistEmptySearchResults.into()
    );
  }

  #[tokio::test]
  async fn test_handle_search_new_artist_event_sets_empty_table_on_api_error() {
    let (mock, app, _server) = MockServarrApi::get()
      .status(500)
      .query("term=nonexistent")
      .build_for(LidarrEvent::SearchNewArtist("nonexistent".to_owned()))
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let result = network
      .handle_lidarr_event(LidarrEvent::SearchNewArtist("nonexistent".to_owned()))
      .await;

    mock.assert_async().await;

    assert_err!(result);
    let app = app.lock().await;
    assert_some!(&app.data.lidarr_data.add_searched_artists);
    assert_is_empty!(app.data.lidarr_data.add_searched_artists.as_ref().unwrap());
  }

  #[tokio::test]
  async fn test_handle_add_artist_event() {
    let add_artist_body = AddArtistBody {
      foreign_artist_id: "test-foreign-id".to_owned(),
      artist_name: "Test Artist".to_owned(),
      monitored: true,
      root_folder_path: "/music".to_owned(),
      quality_profile_id: 1,
      metadata_profile_id: 1,
      tags: Vec::default(),
      tag_input_string: Some("usenet, testing".to_owned()),
      add_options: AddArtistOptions {
        monitor: MonitorType::All,
        monitor_new_items: NewItemMonitorType::All,
        search_for_missing_albums: true,
      },
    };
    let expected_body = json!({
      "foreignArtistId": "test-foreign-id",
      "artistName": "Test Artist",
      "monitored": true,
      "rootFolderPath": "/music",
      "qualityProfileId": 1,
      "metadataProfileId": 1,
      "tags": [1, 2],
      "addOptions": {
        "monitor": "all",
        "monitorNewItems": "all",
        "searchForMissingAlbums": true
      }
    });
    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(expected_body)
      .returns(json!({"id": 1}))
      .build_for(LidarrEvent::AddArtist(AddArtistBody::default()))
      .await;
    app.lock().await.data.lidarr_data.tags_map =
      BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    assert!(
      network
        .handle_lidarr_event(LidarrEvent::AddArtist(add_artist_body))
        .await
        .is_ok()
    );

    mock.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_add_artist_event_does_not_overwrite_tags_vec_when_tag_input_string_is_none()
  {
    let add_artist_body = AddArtistBody {
      foreign_artist_id: "test-foreign-id".to_owned(),
      artist_name: "Test Artist".to_owned(),
      monitored: true,
      root_folder_path: "/music".to_owned(),
      quality_profile_id: 1,
      metadata_profile_id: 1,
      tags: vec![1, 2],
      tag_input_string: None,
      add_options: AddArtistOptions {
        monitor: MonitorType::All,
        monitor_new_items: NewItemMonitorType::All,
        search_for_missing_albums: true,
      },
    };
    let expected_body = json!({
      "foreignArtistId": "test-foreign-id",
      "artistName": "Test Artist",
      "monitored": true,
      "rootFolderPath": "/music",
      "qualityProfileId": 1,
      "metadataProfileId": 1,
      "tags": [1, 2],
      "addOptions": {
        "monitor": "all",
        "monitorNewItems": "all",
        "searchForMissingAlbums": true
      }
    });
    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(expected_body)
      .returns(json!({"id": 1}))
      .build_for(LidarrEvent::AddArtist(add_artist_body.clone()))
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    assert!(
      network
        .handle_lidarr_event(LidarrEvent::AddArtist(add_artist_body))
        .await
        .is_ok()
    );

    mock.assert_async().await;
  }
}
