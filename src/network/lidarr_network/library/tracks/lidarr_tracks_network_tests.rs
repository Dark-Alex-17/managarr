#[cfg(test)]
mod tests {
  use crate::models::lidarr_models::LidarrSerdeable;
  use crate::models::servarr_data::lidarr::modals::AlbumDetailsModal;
  use crate::network::lidarr_network::LidarrEvent;
  use crate::network::lidarr_network::lidarr_network_test_utils::test_utils::{track, track_file};
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use pretty_assertions::assert_eq;
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
}
