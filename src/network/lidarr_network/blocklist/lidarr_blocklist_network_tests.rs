#[cfg(test)]
mod tests {
  use crate::models::lidarr_models::{Artist, BlocklistItem, BlocklistResponse, LidarrSerdeable};
  use crate::models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock;
  use crate::models::stateful_table::SortOption;
  use crate::network::lidarr_network::LidarrEvent;
  use crate::network::lidarr_network::lidarr_network_test_utils::test_utils::{
    artist, blocklist_item,
  };
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use serde_json::{Number, json};

  #[tokio::test]
  async fn test_handle_clear_lidarr_blocklist_event() {
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
    let (mock, app, _server) = MockServarrApi::delete()
      .with_request_body(expected_request_json)
      .build_for(LidarrEvent::ClearBlocklist)
      .await;
    app
      .lock()
      .await
      .data
      .lidarr_data
      .blocklist
      .set_items(blocklist_items);
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    assert!(
      network
        .handle_lidarr_event(LidarrEvent::ClearBlocklist)
        .await
        .is_ok()
    );

    mock.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_delete_lidarr_blocklist_item_event() {
    let (mock, app, _server) = MockServarrApi::delete()
      .path("/1")
      .build_for(LidarrEvent::DeleteBlocklistItem(1))
      .await;
    app
      .lock()
      .await
      .data
      .lidarr_data
      .blocklist
      .set_items(vec![blocklist_item()]);
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    assert!(
      network
        .handle_lidarr_event(LidarrEvent::DeleteBlocklistItem(1))
        .await
        .is_ok()
    );

    mock.assert_async().await;
  }

  #[rstest]
  #[tokio::test]
  async fn test_handle_get_lidarr_blocklist_event(#[values(true, false)] use_custom_sorting: bool) {
    let blocklist_json = json!({"records": [{
        "artistId": 1007,
        "albumIds": [42020],
        "sourceTitle": "z artist",
        "quality": { "quality": { "name": "Lossless" }},
        "date": "2023-05-20T21:29:16Z",
        "protocol": "usenet",
        "indexer": "NZBgeek (Prowlarr)",
        "message": "test message",
        "id": 123,
        "artist": {
          "id": 1,
          "artistName": "Alex",
          "foreignArtistId": "test-foreign-id",
          "status": "continuing",
          "overview": "some interesting description of the artist",
          "artistType": "Person",
          "disambiguation": "American pianist",
          "path": "/nfs/music/test-artist",
          "members": [{"name": "alex", "instrument": "piano"}],
          "qualityProfileId": 1,
          "metadataProfileId": 1,
          "monitored": true,
          "monitorNewItems": "all",
          "genres": ["soundtrack"],
          "tags": [1],
          "added": "2023-01-01T00:00:00Z",
          "ratings": { "votes": 15, "value": 8.4 },
          "statistics": {
            "albumCount": 1,
            "trackFileCount": 15,
            "trackCount": 15,
            "totalTrackCount": 15,
            "sizeOnDisk": 12345,
            "percentOfTracks": 99.9
          }
        }
    },
    {
        "artistId": 2001,
        "artistTitle": "Test Artist",
        "albumIds": [42018],
        "sourceTitle": "A Artist",
        "quality": { "quality": { "name": "Lossless" }},
        "date": "2023-05-20T21:29:16Z",
        "protocol": "usenet",
        "indexer": "NZBgeek (Prowlarr)",
        "message": "test message",
        "id": 456,
        "artist": {
          "id": 1,
          "artistName": "Alex",
          "foreignArtistId": "test-foreign-id",
          "status": "continuing",
          "overview": "some interesting description of the artist",
          "artistType": "Person",
          "disambiguation": "American pianist",
          "path": "/nfs/music/test-artist",
          "members": [{"name": "alex", "instrument": "piano"}],
          "qualityProfileId": 1,
          "metadataProfileId": 1,
          "monitored": true,
          "monitorNewItems": "all",
          "genres": ["soundtrack"],
          "tags": [1],
          "added": "2023-01-01T00:00:00Z",
          "ratings": { "votes": 15, "value": 8.4 },
          "statistics": {
            "albumCount": 1,
            "trackFileCount": 15,
            "trackCount": 15,
            "totalTrackCount": 15,
            "sizeOnDisk": 12345,
            "percentOfTracks": 99.9
          }
        }
    }]});
    let response: BlocklistResponse = serde_json::from_value(blocklist_json.clone()).unwrap();
    let mut expected_blocklist = vec![
      BlocklistItem {
        id: 123,
        artist_id: 1007,
        source_title: "z artist".into(),
        album_ids: Some(vec![Number::from(42020)]),
        ..blocklist_item()
      },
      BlocklistItem {
        id: 456,
        artist_id: 2001,
        source_title: "A Artist".into(),
        album_ids: Some(vec![Number::from(42018)]),
        ..blocklist_item()
      },
    ];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(blocklist_json)
      .build_for(LidarrEvent::GetBlocklist)
      .await;
    app
      .lock()
      .await
      .data
      .lidarr_data
      .artists
      .set_items(vec![Artist {
        id: 1007,
        artist_name: "Z Artist".into(),
        ..artist()
      }]);
    app.lock().await.data.lidarr_data.blocklist.sort_asc = true;
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
      app
        .lock()
        .await
        .data
        .lidarr_data
        .blocklist
        .sorting(vec![blocklist_sort_option]);
    }
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let LidarrSerdeable::BlocklistResponse(blocklist) = network
      .handle_lidarr_event(LidarrEvent::GetBlocklist)
      .await
      .unwrap()
    else {
      panic!("Expected BlocklistResponse")
    };
    mock.assert_async().await;
    assert_eq!(
      app.lock().await.data.lidarr_data.blocklist.items,
      expected_blocklist
    );
    assert!(app.lock().await.data.lidarr_data.blocklist.sort_asc);
    assert_eq!(blocklist, response);
  }

  #[tokio::test]
  async fn test_handle_get_lidarr_blocklist_event_no_op_when_user_is_selecting_sort_options() {
    let blocklist_json = json!({"records": [{
        "artistId": 1007,
        "albumIds": [42020],
        "sourceTitle": "z artist",
        "quality": { "quality": { "name": "Lossless" }},
        "date": "2023-05-20T21:29:16Z",
        "protocol": "usenet",
        "indexer": "NZBgeek (Prowlarr)",
        "message": "test message",
        "id": 123,
        "artist": {
          "id": 1,
          "artistName": "Alex",
          "foreignArtistId": "test-foreign-id",
          "status": "continuing",
          "overview": "some interesting description of the artist",
          "artistType": "Person",
          "disambiguation": "American pianist",
          "path": "/nfs/music/test-artist",
          "members": [{"name": "alex", "instrument": "piano"}],
          "qualityProfileId": 1,
          "metadataProfileId": 1,
          "monitored": true,
          "monitorNewItems": "all",
          "genres": ["soundtrack"],
          "tags": [1],
          "added": "2023-01-01T00:00:00Z",
          "ratings": { "votes": 15, "value": 8.4 },
          "statistics": {
            "albumCount": 1,
            "trackFileCount": 15,
            "trackCount": 15,
            "totalTrackCount": 15,
            "sizeOnDisk": 12345,
            "percentOfTracks": 99.9
          }
        }
    },
    {
        "artistId": 2001,
        "albumIds": [42018],
        "sourceTitle": "A Artist",
        "quality": { "quality": { "name": "Lossless" }},
        "date": "2023-05-20T21:29:16Z",
        "protocol": "usenet",
        "indexer": "NZBgeek (Prowlarr)",
        "message": "test message",
        "id": 456,
        "artist": {
          "id": 1,
          "artistName": "Alex",
          "foreignArtistId": "test-foreign-id",
          "status": "continuing",
          "overview": "some interesting description of the artist",
          "artistType": "Person",
          "disambiguation": "American pianist",
          "path": "/nfs/music/test-artist",
          "members": [{"name": "alex", "instrument": "piano"}],
          "qualityProfileId": 1,
          "metadataProfileId": 1,
          "monitored": true,
          "monitorNewItems": "all",
          "genres": ["soundtrack"],
          "tags": [1],
          "added": "2023-01-01T00:00:00Z",
          "ratings": { "votes": 15, "value": 8.4 },
          "statistics": {
            "albumCount": 1,
            "trackFileCount": 15,
            "trackCount": 15,
            "totalTrackCount": 15,
            "sizeOnDisk": 12345,
            "percentOfTracks": 99.9
          }
        }
    }]});
    let response: BlocklistResponse = serde_json::from_value(blocklist_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(blocklist_json)
      .build_for(LidarrEvent::GetBlocklist)
      .await;
    app.lock().await.data.lidarr_data.blocklist.sort_asc = true;
    app
      .lock()
      .await
      .push_navigation_stack(ActiveLidarrBlock::BlocklistSortPrompt.into());
    let cmp_fn = |a: &BlocklistItem, b: &BlocklistItem| {
      a.source_title
        .to_lowercase()
        .cmp(&b.source_title.to_lowercase())
    };
    let blocklist_sort_option = SortOption {
      name: "Source Title",
      cmp_fn: Some(cmp_fn),
    };
    app
      .lock()
      .await
      .data
      .lidarr_data
      .blocklist
      .sorting(vec![blocklist_sort_option]);
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let LidarrSerdeable::BlocklistResponse(blocklist) = network
      .handle_lidarr_event(LidarrEvent::GetBlocklist)
      .await
      .unwrap()
    else {
      panic!("Expected BlocklistResponse")
    };
    mock.assert_async().await;
    assert_is_empty!(app.lock().await.data.lidarr_data.blocklist);
    assert!(app.lock().await.data.lidarr_data.blocklist.sort_asc);
    assert_eq!(blocklist, response);
  }
}
