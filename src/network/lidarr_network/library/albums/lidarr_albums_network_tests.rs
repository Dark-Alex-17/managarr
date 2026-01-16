#[cfg(test)]
mod tests {
  use crate::models::lidarr_models::{
    Album, DeleteParams, LidarrHistoryItem, LidarrRelease, LidarrSerdeable,
  };
  use crate::models::servarr_data::lidarr::modals::AlbumDetailsModal;
  use crate::models::stateful_table::SortOption;
  use crate::network::lidarr_network::LidarrEvent;
  use crate::network::lidarr_network::lidarr_network_test_utils::test_utils::{
    ALBUM_JSON, lidarr_history_item, torrent_release,
  };
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use mockito::Matcher;
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use serde_json::{Value, json};

  #[tokio::test]
  async fn test_handle_get_albums_event() {
    let albums_json = json!([{
      "id": 1,
      "title": "Test Album",
      "foreignAlbumId": "test-foreign-album-id",
      "monitored": true,
      "anyReleaseOk": true,
      "profileId": 1,
      "duration": 180,
      "albumType": "Album",
      "genres": ["Classical"],
      "ratings": {"votes": 15, "value": 8.4},
      "releaseDate": "2023-01-01T00:00:00Z",
      "statistics": {
        "trackFileCount": 10,
        "trackCount": 10,
        "totalTrackCount": 10,
        "sizeOnDisk": 1024,
        "percentOfTracks": 99.9
      }
    }]);
    let response: Vec<Album> = serde_json::from_value(albums_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(albums_json)
      .query("artistId=1")
      .build_for(LidarrEvent::GetAlbums(1))
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let result = network.handle_lidarr_event(LidarrEvent::GetAlbums(1)).await;

    mock.assert_async().await;

    let LidarrSerdeable::Albums(albums) = result.unwrap() else {
      panic!("Expected Albums");
    };

    assert_eq!(albums, response);
    assert!(!app.lock().await.data.lidarr_data.albums.is_empty());
  }

  #[tokio::test]
  async fn test_handle_delete_album_event() {
    let delete_album_params = DeleteParams {
      id: 1,
      delete_files: true,
      add_import_list_exclusion: true,
    };
    let (async_server, app, _server) = MockServarrApi::delete()
      .path("/1")
      .query("deleteFiles=true&addImportListExclusion=true")
      .build_for(LidarrEvent::DeleteAlbum(delete_album_params.clone()))
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    assert!(
      network
        .handle_lidarr_event(LidarrEvent::DeleteAlbum(delete_album_params))
        .await
        .is_ok()
    );

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_toggle_album_monitoring_event() {
    let mut expected_body: Value = serde_json::from_str(ALBUM_JSON).unwrap();
    *expected_body.get_mut("monitored").unwrap() = json!(false);
    let (get_mock, app, mut server) = MockServarrApi::get()
      .returns(serde_json::from_str(ALBUM_JSON).unwrap())
      .path("/1")
      .build_for(LidarrEvent::GetAlbums(1))
      .await;
    let put_mock = server
      .mock("PUT", "/api/v1/album/1")
      .match_body(Matcher::Json(expected_body))
      .match_header("X-Api-Key", "test1234")
      .with_status(202)
      .create_async()
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    assert_ok!(
      network
        .handle_lidarr_event(LidarrEvent::ToggleAlbumMonitoring(1))
        .await
    );

    get_mock.assert_async().await;
    put_mock.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_get_album_details_event() {
    let expected_album: Album = serde_json::from_str(ALBUM_JSON).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(serde_json::from_str(ALBUM_JSON).unwrap())
      .path("/1")
      .build_for(LidarrEvent::GetAlbumDetails(1))
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let result = network
      .handle_lidarr_event(LidarrEvent::GetAlbumDetails(1))
      .await;

    mock.assert_async().await;

    let LidarrSerdeable::Album(album) = result.unwrap() else {
      panic!("Expected Album");
    };

    assert_eq!(album, expected_album);
  }

  #[rstest]
  #[tokio::test]
  async fn test_handle_get_lidarr_album_history_event(
    #[values(true, false)] use_custom_sorting: bool,
  ) {
    let history_json = json!([{
      "id": 123,
      "sourceTitle": "z album",
      "albumId": 1007,
      "artistId": 1007,
      "quality": { "quality": { "name": "Lossless" } },
      "date": "2023-01-01T00:00:00Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/music/Something/cool.mp3",
        "importedPath": "/nfs/music/Something/Album 1/Cool.mp3"
      }
    },
    {
      "id": 456,
      "sourceTitle": "An Album",
      "albumId": 2001,
      "artistId": 2001,
      "quality": { "quality": { "name": "Lossless" } },
      "date": "2023-01-01T00:00:00Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/music/Something/cool.mp3",
        "importedPath": "/nfs/music/Something/Album 1/Cool.mp3"
      }
    }]);
    let response: Vec<LidarrHistoryItem> = serde_json::from_value(history_json.clone()).unwrap();
    let mut expected_history_items = vec![
      LidarrHistoryItem {
        id: 123,
        artist_id: 1007,
        album_id: 1007,
        source_title: "z album".into(),
        ..lidarr_history_item()
      },
      LidarrHistoryItem {
        id: 456,
        artist_id: 2001,
        album_id: 2001,
        source_title: "An Album".into(),
        ..lidarr_history_item()
      },
    ];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(history_json)
      .query("artistId=1&albumId=1")
      .build_for(LidarrEvent::GetAlbumHistory(1, 1))
      .await;
    app.lock().await.data.lidarr_data.album_details_modal = Some(AlbumDetailsModal::default());
    if use_custom_sorting {
      let cmp_fn = |a: &LidarrHistoryItem, b: &LidarrHistoryItem| {
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
      app
        .lock()
        .await
        .data
        .lidarr_data
        .album_details_modal
        .as_mut()
        .unwrap()
        .album_history
        .sorting(vec![history_sort_option]);
    }
    app
      .lock()
      .await
      .data
      .lidarr_data
      .album_details_modal
      .as_mut()
      .unwrap()
      .album_history
      .sort_asc = true;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let LidarrSerdeable::LidarrHistoryItems(history) = network
      .handle_lidarr_event(LidarrEvent::GetAlbumHistory(1, 1))
      .await
      .unwrap()
    else {
      panic!("Expected LidarrHistoryItems")
    };
    mock.assert_async().await;
    assert_eq!(
      app
        .lock()
        .await
        .data
        .lidarr_data
        .album_details_modal
        .as_ref()
        .unwrap()
        .album_history
        .items,
      expected_history_items
    );
    assert!(
      app
        .lock()
        .await
        .data
        .lidarr_data
        .album_details_modal
        .as_ref()
        .unwrap()
        .album_history
        .sort_asc
    );
    assert_eq!(history, response);
  }

  #[tokio::test]
  async fn test_handle_get_lidarr_album_history_event_empty_album_details_modal() {
    let history_json = json!([{
      "id": 123,
      "sourceTitle": "z album",
      "albumId": 1007,
      "artistId": 1007,
      "quality": { "quality": { "name": "Lossless" } },
      "date": "2023-01-01T00:00:00Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/music/Something/cool.mp3",
        "importedPath": "/nfs/music/Something/Album 1/Cool.mp3"
      }
    },
    {
      "id": 456,
      "sourceTitle": "An Album",
      "albumId": 2001,
      "artistId": 2001,
      "quality": { "quality": { "name": "Lossless" } },
      "date": "2023-01-01T00:00:00Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/music/Something/cool.mp3",
        "importedPath": "/nfs/music/Something/Album 1/Cool.mp3"
      }
    }]);
    let response: Vec<LidarrHistoryItem> = serde_json::from_value(history_json.clone()).unwrap();
    let expected_history_items = vec![
      LidarrHistoryItem {
        id: 123,
        artist_id: 1007,
        album_id: 1007,
        source_title: "z album".into(),
        ..lidarr_history_item()
      },
      LidarrHistoryItem {
        id: 456,
        artist_id: 2001,
        album_id: 2001,
        source_title: "An Album".into(),
        ..lidarr_history_item()
      },
    ];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(history_json)
      .query("artistId=1&albumId=1")
      .build_for(LidarrEvent::GetAlbumHistory(1, 1))
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let LidarrSerdeable::LidarrHistoryItems(history) = network
      .handle_lidarr_event(LidarrEvent::GetAlbumHistory(1, 1))
      .await
      .unwrap()
    else {
      panic!("Expected LidarrHistoryItems")
    };
    mock.assert_async().await;
    let app = app.lock().await;
    assert_some!(&app.data.lidarr_data.album_details_modal);
    assert_eq!(
      app
        .data
        .lidarr_data
        .album_details_modal
        .as_ref()
        .unwrap()
        .album_history
        .items,
      expected_history_items
    );
    assert!(
      !app
        .data
        .lidarr_data
        .album_details_modal
        .as_ref()
        .unwrap()
        .album_history
        .sort_asc
    );
    assert_eq!(history, response);
  }

  #[tokio::test]
  async fn test_handle_get_album_releases_event() {
    let release_json = json!([
      {
        "guid": "1234",
        "protocol": "torrent",
        "age": 1,
        "title": "Test Release",
        "indexer": "kickass torrents",
        "indexerId": 2,
        "artistName": "Alex",
        "albumTitle": "Something",
        "size": 1234,
        "rejected": true,
        "rejections": [ "Unknown quality profile", "Release is already mapped" ],
        "seeders": 2,
        "leechers": 1,
        "quality": { "quality": { "name": "Lossless" }},
        "discography": true
      },
      {
        "guid": "4567",
        "protocol": "torrent",
        "age": 1,
        "title": "Test Release",
        "indexer": "kickass torrents",
        "indexerId": 2,
        "artistName": "Alex",
        "albumTitle": "Something",
        "size": 1234,
        "rejected": true,
        "rejections": [ "Unknown quality profile", "Release is already mapped" ],
        "seeders": 2,
        "leechers": 1,
        "quality": { "quality": { "name": "Lossless" }},
      }
    ]);
    let expected_filtered_lidarr_release = LidarrRelease {
      guid: "4567".to_owned(),
      ..torrent_release()
    };
    let expected_raw_lidarr_releases = vec![
      LidarrRelease {
        discography: true,
        ..torrent_release()
      },
      LidarrRelease {
        guid: "4567".to_owned(),
        ..torrent_release()
      },
    ];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(release_json)
      .query("artistId=1&albumId=1")
      .build_for(LidarrEvent::GetAlbumReleases(1, 1))
      .await;
    app.lock().await.data.lidarr_data.album_details_modal = Some(AlbumDetailsModal::default());
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let LidarrSerdeable::Releases(releases_vec) = network
      .handle_lidarr_event(LidarrEvent::GetAlbumReleases(1, 1))
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
        .lidarr_data
        .album_details_modal
        .as_ref()
        .unwrap()
        .album_releases
        .items,
      vec![expected_filtered_lidarr_release]
    );
    assert_eq!(releases_vec, expected_raw_lidarr_releases);
  }

  #[tokio::test]
  async fn test_handle_get_album_releases_event_empty_album_details_modal() {
    let release_json = json!([
      {
        "guid": "1234",
        "protocol": "torrent",
        "age": 1,
        "title": "Test Release",
        "indexer": "kickass torrents",
        "indexerId": 2,
        "artistName": "Alex",
        "albumTitle": "Something",
        "size": 1234,
        "rejected": true,
        "rejections": [ "Unknown quality profile", "Release is already mapped" ],
        "seeders": 2,
        "leechers": 1,
        "quality": { "quality": { "name": "Lossless" }},
        "discography": true
      },
      {
        "guid": "4567",
        "protocol": "torrent",
        "age": 1,
        "title": "Test Release",
        "indexer": "kickass torrents",
        "indexerId": 2,
        "artistName": "Alex",
        "albumTitle": "Something",
        "size": 1234,
        "rejected": true,
        "rejections": [ "Unknown quality profile", "Release is already mapped" ],
        "seeders": 2,
        "leechers": 1,
        "quality": { "quality": { "name": "Lossless" }},
      }
    ]);
    let expected_lidarr_release = LidarrRelease {
      guid: "4567".to_owned(),
      ..torrent_release()
    };
    let (mock, app, _server) = MockServarrApi::get()
      .returns(release_json)
      .query("artistId=1&albumId=1")
      .build_for(LidarrEvent::GetAlbumReleases(1, 1))
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    assert_ok!(
      network
        .handle_lidarr_event(LidarrEvent::GetAlbumReleases(1, 1))
        .await
    );

    mock.assert_async().await;
    assert_eq!(
      app
        .lock()
        .await
        .data
        .lidarr_data
        .album_details_modal
        .as_ref()
        .unwrap()
        .album_releases
        .items,
      vec![expected_lidarr_release]
    );
  }

  #[tokio::test]
  async fn test_handle_trigger_automatic_album_search_event() {
    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(json!({
        "name": "AlbumSearch",
        "albumIds": [1]
      }))
      .returns(json!({}))
      .build_for(LidarrEvent::TriggerAutomaticAlbumSearch(1))
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    assert!(
      network
        .handle_lidarr_event(LidarrEvent::TriggerAutomaticAlbumSearch(1))
        .await
        .is_ok()
    );

    mock.assert_async().await;
  }
}
