#[cfg(test)]
mod tests {
  use crate::models::radarr_models::BlocklistItem;
  use crate::models::radarr_models::BlocklistItemMovie;
  use crate::models::radarr_models::BlocklistResponse;
  use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
  use crate::models::stateful_table::SortOption;
  use crate::network::network_tests::test_utils::mock_servarr_api;
  use crate::network::radarr_network::radarr_network_test_utils::test_utils::blocklist_item;
  use crate::network::radarr_network::RadarrEvent;
  use crate::network::radarr_network::RadarrSerdeable;
  use crate::network::{Network, RequestMethod};
  use pretty_assertions::assert_eq;
  use reqwest::Client;
  use rstest::rstest;
  use serde_json::json;
  use tokio_util::sync::CancellationToken;

  #[tokio::test]
  async fn test_handle_clear_radarr_blocklist_event() {
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
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Delete,
      Some(expected_request_json),
      None,
      None,
      RadarrEvent::ClearBlocklist,
      None,
      None,
    )
    .await;
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .blocklist
      .set_items(blocklist_items);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::ClearBlocklist)
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_delete_radarr_blocklist_item_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Delete,
      None,
      None,
      None,
      RadarrEvent::DeleteBlocklistItem(1),
      Some("/1"),
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::DeleteBlocklistItem(1))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[rstest]
  #[tokio::test]
  async fn test_handle_get_radarr_blocklist_event(#[values(true, false)] use_custom_sorting: bool) {
    let blocklist_json = json!({"records": [{
        "id": 123,
        "movieId": 1007,
        "sourceTitle": "z movie",
        "languages": [{"id": 1, "name": "English"}],
        "quality": {"quality": {"name": "HD - 1080p"}},
        "customFormats": [{"id": 1, "name": "English"}],
        "date": "2024-02-10T07:28:45Z",
        "protocol": "usenet",
        "indexer": "DrunkenSlug (Prowlarr)",
        "message": "test message",
        "movie": {
          "id": 1007,
          "title": "z movie",
          "tmdbId": 1234,
          "originalLanguage": {"id": 1, "name": "English"},
          "sizeOnDisk": 3543348019i64,
          "status": "Downloaded",
          "overview": "Blah blah blah",
          "path": "/nfs/movies",
          "studio": "21st Century Alex",
          "genres": ["cool", "family", "fun"],
          "year": 2023,
          "monitored": true,
          "hasFile": true,
          "runtime": 120,
          "qualityProfileId": 2222,
          "minimumAvailability": "announced",
          "certification": "R",
          "tags": [1],
          "ratings": {
            "imdb": {"value": 9.9},
            "tmdb": {"value": 9.9},
            "rottenTomatoes": {"value": 9.9}
          },
        },
      }, {
        "id": 456,
        "movieId": 2001,
        "sourceTitle": "A Movie",
        "languages": [{"id": 1, "name": "English"}],
        "quality": {"quality": {"name": "HD - 1080p"}},
        "customFormats": [{"id": 1, "name": "English"}],
        "date": "2024-02-10T07:28:45Z",
        "protocol": "usenet",
        "indexer": "DrunkenSlug (Prowlarr)",
        "message": "test message",
        "movie": {
          "id": 2001,
          "title": "A Movie",
          "tmdbId": 1234,
          "originalLanguage": {"id": 1, "name": "English"},
          "sizeOnDisk": 3543348019i64,
          "status": "Downloaded",
          "overview": "Blah blah blah",
          "path": "/nfs/movies",
          "studio": "21st Century Alex",
          "genres": ["cool", "family", "fun"],
          "year": 2023,
          "monitored": true,
          "hasFile": true,
          "runtime": 120,
          "qualityProfileId": 2222,
          "minimumAvailability": "announced",
          "certification": "R",
          "tags": [1],
          "ratings": {
            "imdb": {"value": 9.9},
            "tmdb": {"value": 9.9},
            "rottenTomatoes": {"value": 9.9}
          },
        },
    }]});
    let response: BlocklistResponse = serde_json::from_value(blocklist_json.clone()).unwrap();
    let mut expected_blocklist = vec![
      BlocklistItem {
        id: 123,
        movie_id: 1007,
        source_title: "z movie".into(),
        movie: BlocklistItemMovie {
          title: "z movie".into(),
        },
        ..blocklist_item()
      },
      BlocklistItem {
        id: 456,
        movie_id: 2001,
        source_title: "A Movie".into(),
        movie: BlocklistItemMovie {
          title: "A Movie".into(),
        },
        ..blocklist_item()
      },
    ];
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(blocklist_json),
      None,
      RadarrEvent::GetBlocklist,
      None,
      None,
    )
    .await;
    app_arc.lock().await.data.radarr_data.blocklist.sort_asc = true;
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
      app_arc
        .lock()
        .await
        .data
        .radarr_data
        .blocklist
        .sorting(vec![blocklist_sort_option]);
    }
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::BlocklistResponse(blocklist) = network
      .handle_radarr_event(RadarrEvent::GetBlocklist)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.radarr_data.blocklist.items,
        expected_blocklist
      );
      assert!(app_arc.lock().await.data.radarr_data.blocklist.sort_asc);
      assert_eq!(blocklist, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_blocklist_event_no_op_when_user_is_selecting_sort_options() {
    let blocklist_json = json!({"records": [{
        "id": 123,
        "movieId": 1007,
        "sourceTitle": "z movie",
        "languages": [{"id": 1, "name": "English"}],
        "quality": {"quality": {"name": "HD - 1080p"}},
        "customFormats": [{"id": 1, "name": "English"}],
        "date": "2024-02-10T07:28:45Z",
        "protocol": "usenet",
        "indexer": "DrunkenSlug (Prowlarr)",
        "message": "test message",
        "movie": {
          "id": 1007,
          "title": "z movie",
          "tmdbId": 1234,
          "originalLanguage": {"id": 1, "name": "English"},
          "sizeOnDisk": 3543348019i64,
          "status": "Downloaded",
          "overview": "Blah blah blah",
          "path": "/nfs/movies",
          "studio": "21st Century Alex",
          "genres": ["cool", "family", "fun"],
          "year": 2023,
          "monitored": true,
          "hasFile": true,
          "runtime": 120,
          "qualityProfileId": 2222,
          "minimumAvailability": "announced",
          "certification": "R",
          "tags": [1],
          "ratings": {
            "imdb": {"value": 9.9},
            "tmdb": {"value": 9.9},
            "rottenTomatoes": {"value": 9.9}
          },
        },
      }, {
        "id": 456,
        "movieId": 2001,
        "sourceTitle": "A Movie",
        "languages": [{"id": 1, "name": "English"}],
        "quality": {"quality": {"name": "HD - 1080p"}},
        "customFormats": [{"id": 1, "name": "English"}],
        "date": "2024-02-10T07:28:45Z",
        "protocol": "usenet",
        "indexer": "DrunkenSlug (Prowlarr)",
        "message": "test message",
        "movie": {
          "id": 2001,
          "title": "A Movie",
          "tmdbId": 1234,
          "originalLanguage": {"id": 1, "name": "English"},
          "sizeOnDisk": 3543348019i64,
          "status": "Downloaded",
          "overview": "Blah blah blah",
          "path": "/nfs/movies",
          "studio": "21st Century Alex",
          "genres": ["cool", "family", "fun"],
          "year": 2023,
          "monitored": true,
          "hasFile": true,
          "runtime": 120,
          "qualityProfileId": 2222,
          "minimumAvailability": "announced",
          "certification": "R",
          "tags": [1],
          "ratings": {
            "imdb": {"value": 9.9},
            "tmdb": {"value": 9.9},
            "rottenTomatoes": {"value": 9.9}
          },
        },
    }]});
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(blocklist_json),
      None,
      RadarrEvent::GetBlocklist,
      None,
      None,
    )
    .await;
    app_arc.lock().await.data.radarr_data.blocklist.sort_asc = true;
    app_arc
      .lock()
      .await
      .push_navigation_stack(ActiveRadarrBlock::BlocklistSortPrompt.into());
    let cmp_fn = |a: &BlocklistItem, b: &BlocklistItem| {
      a.source_title
        .to_lowercase()
        .cmp(&b.source_title.to_lowercase())
    };
    let blocklist_sort_option = SortOption {
      name: "Source Title",
      cmp_fn: Some(cmp_fn),
    };
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .blocklist
      .sorting(vec![blocklist_sort_option]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::GetBlocklist)
      .await
      .is_ok());

    async_server.assert_async().await;
    assert!(app_arc
      .lock()
      .await
      .data
      .radarr_data
      .blocklist
      .items
      .is_empty());
    assert!(app_arc.lock().await.data.radarr_data.blocklist.sort_asc);
  }
}
