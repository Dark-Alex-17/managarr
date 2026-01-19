#[cfg(test)]
mod tests {
  use crate::models::lidarr_models::{LidarrHistoryItem, LidarrSerdeable, Track, TrackFile};
  use crate::models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock;
  use crate::models::servarr_data::lidarr::modals::{AlbumDetailsModal, TrackDetailsModal};
  use crate::models::stateful_table::SortOption;
  use crate::network::lidarr_network::LidarrEvent;
  use crate::network::lidarr_network::lidarr_network_test_utils::test_utils::{
    lidarr_history_item, track, track_file,
  };
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use indoc::formatdoc;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use serde_json::json;

  #[tokio::test]
  async fn test_handle_delete_lidarr_track_file_event() {
    let (async_server, app_arc, _server) = MockServarrApi::delete()
      .path("/1")
      .build_for(LidarrEvent::DeleteTrackFile(1))
      .await;
    app_arc.lock().await.data.lidarr_data.album_details_modal = Some(AlbumDetailsModal::default());
    app_arc.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app_arc);

    assert!(
      network
        .handle_lidarr_event(LidarrEvent::DeleteTrackFile(1))
        .await
        .is_ok()
    );

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_get_track_details_event() {
    let response = track();
    let (async_server, app_arc, _server) = MockServarrApi::get()
      .returns(serde_json::to_value(track()).unwrap())
      .path("/1")
      .build_for(LidarrEvent::GetTrackDetails(1))
      .await;
    app_arc.lock().await.data.lidarr_data.album_details_modal = Some(AlbumDetailsModal {
      track_details_modal: Some(TrackDetailsModal::default()),
      ..AlbumDetailsModal::default()
    });
    app_arc.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app_arc);

    let result = network
      .handle_lidarr_event(LidarrEvent::GetTrackDetails(1))
      .await;

    async_server.assert_async().await;
    assert_ok!(&result);
    let LidarrSerdeable::Track(track) = result.unwrap() else {
      panic!("Expected Track")
    };
    assert_eq!(track, response);
    let app = app_arc.lock().await;
    assert_eq!(
      app
        .data
        .lidarr_data
        .album_details_modal
        .as_ref()
        .unwrap()
        .track_details_modal
        .as_ref()
        .unwrap()
        .track_details_tabs
        .get_active_route(),
      ActiveLidarrBlock::TrackDetails.into()
    );
    let track_details = &app
      .data
      .lidarr_data
      .album_details_modal
      .as_ref()
      .unwrap()
      .track_details_modal
      .as_ref()
      .unwrap()
      .track_details;
    assert_str_eq!(
      track_details.get_text(),
      formatdoc!(
        "
        Title: Test title
        Track Number: 1
        Duration: 3:20
        Explicit: false
        Quality: Lossless
        File Path: /music/P!nk/TRUSTFALL/01 - When I Get There.flac
        File Size: 37.40 MB
        Date Added: 2023-05-20 21:29:16 UTC
        Codec: FLAC
        Channels: 2
        Bits: 24bit
        Bit Rate: 1563 kbps
        Sample Rate: 44.1kHz
      "
      )
    )
  }

  #[tokio::test]
  async fn test_handle_get_track_details_event_empty_media_info() {
    let expected_track = Track {
      track_file: Some(TrackFile {
        media_info: None,
        ..track_file()
      }),
      ..track()
    };
    let response = expected_track.clone();
    let (async_server, app_arc, _server) = MockServarrApi::get()
      .returns(serde_json::to_value(expected_track.clone()).unwrap())
      .path("/1")
      .build_for(LidarrEvent::GetTrackDetails(1))
      .await;
    app_arc.lock().await.data.lidarr_data.album_details_modal = Some(AlbumDetailsModal {
      track_details_modal: Some(TrackDetailsModal::default()),
      ..AlbumDetailsModal::default()
    });
    app_arc.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app_arc);

    let result = network
      .handle_lidarr_event(LidarrEvent::GetTrackDetails(1))
      .await;

    async_server.assert_async().await;
    assert_ok!(&result);
    let LidarrSerdeable::Track(track) = result.unwrap() else {
      panic!("Expected Track")
    };
    assert_eq!(track, response);
    let app = app_arc.lock().await;
    assert_eq!(
      app
        .data
        .lidarr_data
        .album_details_modal
        .as_ref()
        .unwrap()
        .track_details_modal
        .as_ref()
        .unwrap()
        .track_details_tabs
        .get_active_route(),
      ActiveLidarrBlock::TrackDetails.into()
    );
    let track_details = &app
      .data
      .lidarr_data
      .album_details_modal
      .as_ref()
      .unwrap()
      .track_details_modal
      .as_ref()
      .unwrap()
      .track_details;
    assert_str_eq!(
      track_details.get_text(),
      formatdoc!(
        "
        Title: Test title
        Track Number: 1
        Duration: 3:20
        Explicit: false
        Quality: Lossless
        File Path: /music/P!nk/TRUSTFALL/01 - When I Get There.flac
        File Size: 37.40 MB
        Date Added: 2023-05-20 21:29:16 UTC
      "
      )
    )
  }

  #[tokio::test]
  async fn test_handle_get_track_details_event_empty_track_file() {
    let expected_track = Track {
      track_file: None,
      ..track()
    };
    let response = expected_track.clone();
    let (async_server, app_arc, _server) = MockServarrApi::get()
      .returns(serde_json::to_value(expected_track.clone()).unwrap())
      .path("/1")
      .build_for(LidarrEvent::GetTrackDetails(1))
      .await;
    app_arc.lock().await.data.lidarr_data.album_details_modal = Some(AlbumDetailsModal {
      track_details_modal: Some(TrackDetailsModal::default()),
      ..AlbumDetailsModal::default()
    });
    app_arc.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app_arc);

    let result = network
      .handle_lidarr_event(LidarrEvent::GetTrackDetails(1))
      .await;

    async_server.assert_async().await;
    assert_ok!(&result);
    let LidarrSerdeable::Track(track) = result.unwrap() else {
      panic!("Expected Track")
    };
    assert_eq!(track, response);
    let app = app_arc.lock().await;
    assert_eq!(
      app
        .data
        .lidarr_data
        .album_details_modal
        .as_ref()
        .unwrap()
        .track_details_modal
        .as_ref()
        .unwrap()
        .track_details_tabs
        .get_active_route(),
      ActiveLidarrBlock::TrackDetails.into()
    );
    let track_details = &app
      .data
      .lidarr_data
      .album_details_modal
      .as_ref()
      .unwrap()
      .track_details_modal
      .as_ref()
      .unwrap()
      .track_details;
    assert_str_eq!(
      track_details.get_text(),
      formatdoc!(
        "
        Title: Test title
        Track Number: 1
        Duration: 3:20
        Explicit: false
      "
      )
    )
  }

  #[tokio::test]
  async fn test_handle_get_track_details_event_album_details_modal_not_required_in_cli_mode() {
    let response = track();
    let (async_server, app_arc, _server) = MockServarrApi::get()
      .returns(serde_json::to_value(track()).unwrap())
      .path("/1")
      .build_for(LidarrEvent::GetTrackDetails(1))
      .await;
    app_arc.lock().await.cli_mode = true;
    app_arc.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app_arc);

    let result = network
      .handle_lidarr_event(LidarrEvent::GetTrackDetails(1))
      .await;

    async_server.assert_async().await;
    assert_ok!(&result);
    let LidarrSerdeable::Track(track) = result.unwrap() else {
      panic!("Expected Track")
    };
    assert_eq!(track, response);
    let app = app_arc.lock().await;
    assert_some!(&app.data.lidarr_data.album_details_modal);
    assert_some!(
      &app
        .data
        .lidarr_data
        .album_details_modal
        .as_ref()
        .unwrap()
        .track_details_modal
    );
  }

  #[tokio::test]
  #[should_panic(expected = "Album details modal is empty")]
  async fn test_handle_get_track_details_event_requires_album_details_modal_to_be_some_when_in_tui_mode()
   {
    let (_async_server, app_arc, _server) = MockServarrApi::get()
      .returns(serde_json::to_value(track()).unwrap())
      .path("/1")
      .build_for(LidarrEvent::GetTrackDetails(1))
      .await;
    app_arc.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app_arc);

    network
      .handle_lidarr_event(LidarrEvent::GetTrackDetails(1))
      .await
      .unwrap();
  }

  #[tokio::test]
  async fn test_handle_get_tracks_event() {
    let expected_tracks = vec![track()];
    let (mock, app, _server) = MockServarrApi::get()
      .query("artistId=1&albumId=1")
      .returns(json!([track()]))
      .build_for(LidarrEvent::GetTracks(1, 1))
      .await;
    app.lock().await.data.lidarr_data.album_details_modal = Some(AlbumDetailsModal::default());
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let result = network
      .handle_lidarr_event(LidarrEvent::GetTracks(1, 1))
      .await;

    mock.assert_async().await;

    let LidarrSerdeable::Tracks(tracks) = result.unwrap() else {
      panic!("Expected Tracks variant")
    };
    assert_eq!(
      app
        .lock()
        .await
        .data
        .lidarr_data
        .album_details_modal
        .as_ref()
        .unwrap()
        .tracks
        .items,
      expected_tracks
    );
    assert_eq!(tracks, expected_tracks);
  }

  #[tokio::test]
  async fn test_handle_get_tracks_event_empty_album_details_modal() {
    let expected_tracks = vec![track()];
    let (mock, app, _server) = MockServarrApi::get()
      .query("artistId=1&albumId=1")
      .returns(json!([track()]))
      .build_for(LidarrEvent::GetTracks(1, 1))
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let result = network
      .handle_lidarr_event(LidarrEvent::GetTracks(1, 1))
      .await;

    mock.assert_async().await;

    let LidarrSerdeable::Tracks(tracks) = result.unwrap() else {
      panic!("Expected Tracks variant")
    };

    assert_eq!(
      app
        .lock()
        .await
        .data
        .lidarr_data
        .album_details_modal
        .as_ref()
        .unwrap()
        .tracks
        .items,
      expected_tracks
    );
    assert_eq!(tracks, expected_tracks);
  }

  #[tokio::test]
  async fn test_handle_get_track_files_event() {
    let (async_server, app_arc, _server) = MockServarrApi::get()
      .returns(json!([track_file()]))
      .query("albumId=1")
      .build_for(LidarrEvent::GetTrackFiles(1))
      .await;
    app_arc.lock().await.data.lidarr_data.album_details_modal = Some(AlbumDetailsModal::default());
    app_arc.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app_arc);

    let LidarrSerdeable::TrackFiles(track_files) = network
      .handle_lidarr_event(LidarrEvent::GetTrackFiles(1))
      .await
      .unwrap()
    else {
      panic!("Expected TrackFiles")
    };
    async_server.assert_async().await;
    assert_eq!(
      app_arc
        .lock()
        .await
        .data
        .lidarr_data
        .album_details_modal
        .as_ref()
        .unwrap()
        .track_files
        .items,
      vec![track_file()]
    );
    assert_eq!(track_files, vec![track_file()]);
  }

  #[tokio::test]
  async fn test_handle_get_track_files_event_empty_album_details_modal() {
    let (async_server, app_arc, _server) = MockServarrApi::get()
      .returns(json!([track_file()]))
      .query("albumId=1")
      .build_for(LidarrEvent::GetTrackFiles(1))
      .await;
    app_arc.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app_arc);

    let LidarrSerdeable::TrackFiles(track_files) = network
      .handle_lidarr_event(LidarrEvent::GetTrackFiles(1))
      .await
      .unwrap()
    else {
      panic!("Expected TrackFiles")
    };
    async_server.assert_async().await;
    let app = app_arc.lock().await;
    assert_some!(&app.data.lidarr_data.album_details_modal);
    assert_eq!(
      app
        .data
        .lidarr_data
        .album_details_modal
        .as_ref()
        .unwrap()
        .track_files
        .items,
      vec![track_file()]
    );
    assert_eq!(track_files, vec![track_file()]);
  }

  #[rstest]
  #[tokio::test]
  async fn test_handle_get_lidarr_track_history_event(
    #[values(true, false)] use_custom_sorting: bool,
  ) {
    let history_json = json!([{
      "id": 123,
      "sourceTitle": "z track",
      "albumId": 1007,
      "artistId": 1007,
      "trackId": 1007,
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
      "sourceTitle": "A Track",
      "albumId": 2001,
      "artistId": 2001,
      "trackId": 2001,
      "quality": { "quality": { "name": "Lossless" } },
      "date": "2023-01-01T00:00:00Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/music/Something/cool.mp3",
        "importedPath": "/nfs/music/Something/Album 1/Cool.mp3"
      }
    }]);
    let response: Vec<LidarrHistoryItem> = serde_json::from_value(history_json.clone()).unwrap();
    let mut expected_history_items = vec![LidarrHistoryItem {
      id: 456,
      artist_id: 2001,
      album_id: 2001,
      track_id: 2001,
      source_title: "A Track".into(),
      ..lidarr_history_item()
    }];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(history_json)
      .query("artistId=2001&albumId=2001")
      .build_for(LidarrEvent::GetTrackHistory(2001, 2001, 2001))
      .await;
    let album_details_modal = AlbumDetailsModal {
      track_details_modal: Some(TrackDetailsModal::default()),
      ..AlbumDetailsModal::default()
    };
    app.lock().await.data.lidarr_data.album_details_modal = Some(album_details_modal);
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
        .track_details_modal
        .as_mut()
        .unwrap()
        .track_history
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
      .track_details_modal
      .as_mut()
      .unwrap()
      .track_history
      .sort_asc = true;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let LidarrSerdeable::LidarrHistoryItems(history) = network
      .handle_lidarr_event(LidarrEvent::GetTrackHistory(2001, 2001, 2001))
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
        .track_details_modal
        .as_ref()
        .unwrap()
        .track_history
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
        .track_details_modal
        .as_ref()
        .unwrap()
        .track_history
        .sort_asc
    );
    assert_eq!(history, response);
  }

  #[tokio::test]
  async fn test_handle_get_lidarr_track_history_event_empty_track_details_modal() {
    let history_json = json!([{
      "id": 123,
      "sourceTitle": "z track",
      "albumId": 1007,
      "artistId": 1007,
      "trackId": 1007,
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
      "sourceTitle": "A Track",
      "albumId": 2001,
      "artistId": 2001,
      "trackId": 2001,
      "quality": { "quality": { "name": "Lossless" } },
      "date": "2023-01-01T00:00:00Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/music/Something/cool.mp3",
        "importedPath": "/nfs/music/Something/Album 1/Cool.mp3"
      }
    }]);
    let response: Vec<LidarrHistoryItem> = serde_json::from_value(history_json.clone()).unwrap();
    let expected_history_items = vec![LidarrHistoryItem {
      id: 456,
      artist_id: 2001,
      album_id: 2001,
      track_id: 2001,
      source_title: "A Track".into(),
      ..lidarr_history_item()
    }];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(history_json)
      .query("artistId=2001&albumId=2001")
      .build_for(LidarrEvent::GetTrackHistory(2001, 2001, 2001))
      .await;
    app.lock().await.data.lidarr_data.album_details_modal = Some(AlbumDetailsModal::default());
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let LidarrSerdeable::LidarrHistoryItems(history) = network
      .handle_lidarr_event(LidarrEvent::GetTrackHistory(2001, 2001, 2001))
      .await
      .unwrap()
    else {
      panic!("Expected LidarrHistoryItems")
    };
    mock.assert_async().await;
    let app = app.lock().await;
    assert_some!(&app.data.lidarr_data.album_details_modal);
    assert_some!(
      &app
        .data
        .lidarr_data
        .album_details_modal
        .as_ref()
        .unwrap()
        .track_details_modal
    );
    assert_eq!(
      app
        .data
        .lidarr_data
        .album_details_modal
        .as_ref()
        .unwrap()
        .track_details_modal
        .as_ref()
        .unwrap()
        .track_history
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
        .track_details_modal
        .as_ref()
        .unwrap()
        .track_history
        .sort_asc
    );
    assert_eq!(history, response);
  }

  #[tokio::test]
  async fn test_handle_get_lidarr_track_history_event_empty_album_details_modal() {
    let history_json = json!([{
      "id": 123,
      "sourceTitle": "z track",
      "albumId": 1007,
      "artistId": 1007,
      "trackId": 1007,
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
      "sourceTitle": "A Track",
      "albumId": 2001,
      "artistId": 2001,
      "trackId": 2001,
      "quality": { "quality": { "name": "Lossless" } },
      "date": "2023-01-01T00:00:00Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/music/Something/cool.mp3",
        "importedPath": "/nfs/music/Something/Album 1/Cool.mp3"
      }
    }]);
    let response: Vec<LidarrHistoryItem> = serde_json::from_value(history_json.clone()).unwrap();
    let expected_history_items = vec![LidarrHistoryItem {
      id: 456,
      artist_id: 2001,
      album_id: 2001,
      track_id: 2001,
      source_title: "A Track".into(),
      ..lidarr_history_item()
    }];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(history_json)
      .query("artistId=2001&albumId=2001")
      .build_for(LidarrEvent::GetTrackHistory(2001, 2001, 2001))
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let LidarrSerdeable::LidarrHistoryItems(history) = network
      .handle_lidarr_event(LidarrEvent::GetTrackHistory(2001, 2001, 2001))
      .await
      .unwrap()
    else {
      panic!("Expected LidarrHistoryItems")
    };
    mock.assert_async().await;
    let app = app.lock().await;
    assert_some!(&app.data.lidarr_data.album_details_modal);
    assert_some!(
      &app
        .data
        .lidarr_data
        .album_details_modal
        .as_ref()
        .unwrap()
        .track_details_modal
    );
    assert_eq!(
      app
        .data
        .lidarr_data
        .album_details_modal
        .as_ref()
        .unwrap()
        .track_details_modal
        .as_ref()
        .unwrap()
        .track_history
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
        .track_details_modal
        .as_ref()
        .unwrap()
        .track_history
        .sort_asc
    );
    assert_eq!(history, response);
  }

  #[tokio::test]
  async fn test_handle_get_lidarr_track_history_event_no_op_when_user_is_selecting_sort_options() {
    let history_json = json!([{
      "id": 123,
      "sourceTitle": "z track",
      "albumId": 1007,
      "artistId": 1007,
      "trackId": 1007,
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
      "sourceTitle": "A Track",
      "albumId": 2001,
      "artistId": 2001,
      "trackId": 2001,
      "quality": { "quality": { "name": "Lossless" } },
      "date": "2023-01-01T00:00:00Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/music/Something/cool.mp3",
        "importedPath": "/nfs/music/Something/Album 1/Cool.mp3"
      }
    }]);
    let response: Vec<LidarrHistoryItem> = serde_json::from_value(history_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(history_json)
      .query("artistId=2001&albumId=2001")
      .build_for(LidarrEvent::GetTrackHistory(2001, 2001, 2001))
      .await;
    let album_details_modal = AlbumDetailsModal {
      track_details_modal: Some(TrackDetailsModal::default()),
      ..AlbumDetailsModal::default()
    };
    app.lock().await.data.lidarr_data.album_details_modal = Some(album_details_modal);
    app.lock().await.server_tabs.set_index(2);
    app
      .lock()
      .await
      .push_navigation_stack(ActiveLidarrBlock::TrackHistorySortPrompt.into());
    let mut network = test_network(&app);

    let LidarrSerdeable::LidarrHistoryItems(history) = network
      .handle_lidarr_event(LidarrEvent::GetTrackHistory(2001, 2001, 2001))
      .await
      .unwrap()
    else {
      panic!("Expected LidarrHistoryItems")
    };
    mock.assert_async().await;
    let app = app.lock().await;
    assert_some!(&app.data.lidarr_data.album_details_modal);
    assert_some!(
      &app
        .data
        .lidarr_data
        .album_details_modal
        .as_ref()
        .unwrap()
        .track_details_modal
    );
    assert_is_empty!(
      app
        .data
        .lidarr_data
        .album_details_modal
        .as_ref()
        .unwrap()
        .track_details_modal
        .as_ref()
        .unwrap()
        .track_history
        .items,
    );
    assert!(
      !app
        .data
        .lidarr_data
        .album_details_modal
        .as_ref()
        .unwrap()
        .track_details_modal
        .as_ref()
        .unwrap()
        .track_history
        .sort_asc
    );
    assert_eq!(history, response);
  }
}
