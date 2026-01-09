#[cfg(test)]
mod tests {
	use mockito::Matcher;
	use pretty_assertions::assert_eq;
	use serde_json::{json, Value};
	use crate::models::lidarr_models::{Album, LidarrSerdeable};
	use crate::network::lidarr_network::lidarr_network_test_utils::test_utils::{ALBUM_JSON};
	use crate::network::lidarr_network::LidarrEvent;
	use crate::network::network_tests::test_utils::{test_network, MockServarrApi};

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
}
