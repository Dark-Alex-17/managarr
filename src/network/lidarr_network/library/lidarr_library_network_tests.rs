#[cfg(test)]
mod tests {
  use crate::models::lidarr_models::{Artist, LidarrSerdeable};
  use crate::network::lidarr_network::LidarrEvent;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use pretty_assertions::assert_eq;
  use serde_json::json;

  #[tokio::test]
  async fn test_handle_list_artists_event() {
    let artists_json = json!([{
      "id": 1,
      "mbId": "test-mb-id",
      "artistName": "Test Artist",
      "foreignArtistId": "test-foreign-id",
      "status": "continuing",
      "path": "/music/test-artist",
      "qualityProfileId": 1,
      "metadataProfileId": 1,
      "monitored": true,
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
}
