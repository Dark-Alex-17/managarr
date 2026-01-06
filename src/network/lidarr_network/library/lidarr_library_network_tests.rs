#[cfg(test)]
mod tests {
  use crate::models::lidarr_models::{Artist, DeleteArtistParams, LidarrSerdeable};
  use crate::network::lidarr_network::LidarrEvent;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use mockito::Matcher;
  use pretty_assertions::assert_eq;
  use serde_json::json;

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
    let artist_json = json!({
      "id": 1,
      "artistName": "Test Artist",
      "foreignArtistId": "test-foreign-id",
      "status": "continuing",
      "overview": "some interesting description of the artist",
      "artistType": "Person",
      "disambiguation": "American pianist",
      "path": "/music/test-artist",
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
    });
    let response: Artist = serde_json::from_value(artist_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(artist_json)
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

    assert_eq!(artist, response);
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
}
