#[cfg(test)]
mod tests {
  use crate::models::radarr_models::{
    Collection, EditCollectionParams, MinimumAvailability, RadarrSerdeable,
  };
  use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
  use crate::models::stateful_table::SortOption;
  use crate::network::network_tests::test_utils::mock_servarr_api;
  use crate::network::radarr_network::radarr_network_test_utils::test_utils::collection;
  use crate::network::radarr_network::RadarrEvent;
  use crate::network::{Network, NetworkResource, RequestMethod};
  use mockito::Matcher;
  use pretty_assertions::assert_eq;
  use reqwest::Client;
  use rstest::rstest;
  use serde_json::json;
  use tokio_util::sync::CancellationToken;

  #[tokio::test]
  async fn test_handle_edit_collection_event() {
    let detailed_collection_body = json!({
      "id": 123,
      "title": "Test Collection",
      "rootFolderPath": "/nfs/movies",
      "searchOnAdd": true,
      "monitored": true,
      "minimumAvailability": "released",
      "overview": "Collection blah blah blah",
      "qualityProfileId": 2222,
      "movies": [
        {
          "title": "Test",
          "overview": "Collection blah blah blah",
          "year": 2023,
          "runtime": 120,
          "tmdbId": 1234,
          "genres": ["cool", "family", "fun"],
          "ratings": {
            "imdb": {
              "value": 9.9
            },
            "tmdb": {
              "value": 9.9
            },
            "rottenTomatoes": {
              "value": 9.9
            }
          }
        }
      ]
    });
    let mut expected_body = detailed_collection_body.clone();
    *expected_body.get_mut("monitored").unwrap() = json!(false);
    *expected_body.get_mut("minimumAvailability").unwrap() = json!("announced");
    *expected_body.get_mut("qualityProfileId").unwrap() = json!(1111);
    *expected_body.get_mut("rootFolderPath").unwrap() = json!("/nfs/Test Path");
    *expected_body.get_mut("searchOnAdd").unwrap() = json!(false);
    let edit_collection_params = EditCollectionParams {
      collection_id: 123,
      monitored: Some(false),
      minimum_availability: Some(MinimumAvailability::Announced),
      quality_profile_id: Some(1111),
      root_folder_path: Some("/nfs/Test Path".to_owned()),
      search_on_add: Some(false),
    };

    let (async_details_server, app_arc, mut server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(detailed_collection_body),
      None,
      RadarrEvent::GetCollections,
      Some("/123"),
      None,
    )
    .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v3{}/123",
          RadarrEvent::EditCollection(edit_collection_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .create_async()
      .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::EditCollection(edit_collection_params))
      .await
      .is_ok());

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_collection_event_defaults_to_previous_values_when_no_params_are_provided(
  ) {
    let detailed_collection_body = json!({
      "id": 123,
      "title": "Test Collection",
      "rootFolderPath": "/nfs/movies",
      "searchOnAdd": true,
      "monitored": true,
      "minimumAvailability": "released",
      "overview": "Collection blah blah blah",
      "qualityProfileId": 2222,
      "movies": [
        {
          "title": "Test",
          "overview": "Collection blah blah blah",
          "year": 2023,
          "runtime": 120,
          "tmdbId": 1234,
          "genres": ["cool", "family", "fun"],
          "ratings": {
            "imdb": {
              "value": 9.9
            },
            "tmdb": {
              "value": 9.9
            },
            "rottenTomatoes": {
              "value": 9.9
            }
          }
        }
      ]
    });
    let mut expected_body = detailed_collection_body.clone();
    *expected_body.get_mut("monitored").unwrap() = json!(true);
    *expected_body.get_mut("minimumAvailability").unwrap() = json!("released");
    *expected_body.get_mut("qualityProfileId").unwrap() = json!(2222);
    *expected_body.get_mut("rootFolderPath").unwrap() = json!("/nfs/movies");
    *expected_body.get_mut("searchOnAdd").unwrap() = json!(true);

    let (async_details_server, app_arc, mut server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(detailed_collection_body),
      None,
      RadarrEvent::GetCollections,
      Some("/123"),
      None,
    )
    .await;
    let edit_collection_params = EditCollectionParams {
      collection_id: 123,
      ..EditCollectionParams::default()
    };
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v3{}/123",
          RadarrEvent::EditCollection(edit_collection_params).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .create_async()
      .await;
    let edit_collection_params = EditCollectionParams {
      collection_id: 123,
      ..EditCollectionParams::default()
    };
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::EditCollection(edit_collection_params))
      .await
      .is_ok());

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[rstest]
  #[tokio::test]
  async fn test_handle_get_collections_event(#[values(true, false)] use_custom_sorting: bool) {
    let collections_json = json!([{
      "id": 123,
      "title": "z Collection",
      "rootFolderPath": "/nfs/movies",
      "searchOnAdd": true,
      "monitored": true,
      "minimumAvailability": "released",
      "overview": "Collection blah blah blah",
      "qualityProfileId": 2222,
      "movies": [{
        "title": "Test",
        "overview": "Collection blah blah blah",
        "year": 2023,
        "runtime": 120,
        "tmdbId": 1234,
        "genres": ["cool", "family", "fun"],
        "ratings": {
          "imdb": {
            "value": 9.9
          },
          "tmdb": {
            "value": 9.9
          },
          "rottenTomatoes": {
            "value": 9.9
          }
        }
      }],
    },
    {
      "id": 456,
      "title": "A Collection",
      "rootFolderPath": "/nfs/movies",
      "searchOnAdd": true,
      "monitored": true,
      "minimumAvailability": "released",
      "overview": "Collection blah blah blah",
      "qualityProfileId": 2222,
      "movies": [{
        "title": "Test",
        "overview": "Collection blah blah blah",
        "year": 2023,
        "runtime": 120,
        "tmdbId": 1234,
        "genres": ["cool", "family", "fun"],
        "ratings": {
          "imdb": {
            "value": 9.9
          },
          "tmdb": {
            "value": 9.9
          },
          "rottenTomatoes": {
            "value": 9.9
          }
        }
      }],
    }]);
    let response: Vec<Collection> = serde_json::from_value(collections_json.clone()).unwrap();
    let mut expected_collections = vec![
      Collection {
        id: 123,
        title: "z Collection".into(),
        ..collection()
      },
      Collection {
        id: 456,
        title: "A Collection".into(),
        ..collection()
      },
    ];
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(collections_json),
      None,
      RadarrEvent::GetCollections,
      None,
      None,
    )
    .await;
    app_arc.lock().await.data.radarr_data.collections.sort_asc = true;
    if use_custom_sorting {
      let cmp_fn = |a: &Collection, b: &Collection| {
        a.title
          .text
          .to_lowercase()
          .cmp(&b.title.text.to_lowercase())
      };
      expected_collections.sort_by(cmp_fn);

      let collection_sort_option = SortOption {
        name: "Collection",
        cmp_fn: Some(cmp_fn),
      };
      app_arc
        .lock()
        .await
        .data
        .radarr_data
        .collections
        .sorting(vec![collection_sort_option]);
    }
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::Collections(collections) = network
      .handle_radarr_event(RadarrEvent::GetCollections)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.radarr_data.collections.items,
        expected_collections
      );
      assert!(app_arc.lock().await.data.radarr_data.collections.sort_asc);
      assert_eq!(collections, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_collections_event_no_op_when_user_is_selecting_sort_options() {
    let collections_json = json!([{
      "id": 123,
      "title": "z Collection",
      "rootFolderPath": "/nfs/movies",
      "searchOnAdd": true,
      "monitored": true,
      "minimumAvailability": "released",
      "overview": "Collection blah blah blah",
      "qualityProfileId": 2222,
      "movies": [{
        "title": "Test",
        "overview": "Collection blah blah blah",
        "year": 2023,
        "runtime": 120,
        "tmdbId": 1234,
        "genres": ["cool", "family", "fun"],
        "ratings": {
          "imdb": {
            "value": 9.9
          },
          "tmdb": {
            "value": 9.9
          },
          "rottenTomatoes": {
            "value": 9.9
          }
        }
      }],
    },
    {
      "id": 456,
      "title": "A Collection",
      "rootFolderPath": "/nfs/movies",
      "searchOnAdd": true,
      "monitored": true,
      "minimumAvailability": "released",
      "overview": "Collection blah blah blah",
      "qualityProfileId": 2222,
      "movies": [{
        "title": "Test",
        "overview": "Collection blah blah blah",
        "year": 2023,
        "runtime": 120,
        "tmdbId": 1234,
        "genres": ["cool", "family", "fun"],
        "ratings": {
          "imdb": {
            "value": 9.9
          },
          "tmdb": {
            "value": 9.9
          },
          "rottenTomatoes": {
            "value": 9.9
          }
        }
      }],
    }]);
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(collections_json),
      None,
      RadarrEvent::GetCollections,
      None,
      None,
    )
    .await;
    app_arc.lock().await.data.radarr_data.collections.sort_asc = true;
    app_arc
      .lock()
      .await
      .push_navigation_stack(ActiveRadarrBlock::CollectionsSortPrompt.into());
    let cmp_fn = |a: &Collection, b: &Collection| {
      a.title
        .text
        .to_lowercase()
        .cmp(&b.title.text.to_lowercase())
    };
    let collection_sort_option = SortOption {
      name: "Collection",
      cmp_fn: Some(cmp_fn),
    };
    app_arc
      .lock()
      .await
      .data
      .radarr_data
      .collections
      .sorting(vec![collection_sort_option]);
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::GetCollections)
      .await
      .is_ok());

    async_server.assert_async().await;
    assert!(app_arc
      .lock()
      .await
      .data
      .radarr_data
      .collections
      .items
      .is_empty());
    assert!(app_arc.lock().await.data.radarr_data.collections.sort_asc);
  }

  #[tokio::test]
  async fn test_handle_update_collections_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "name": "RefreshCollections"
      })),
      Some(json!({})),
      None,
      RadarrEvent::UpdateCollections,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::UpdateCollections)
      .await
      .is_ok());

    async_server.assert_async().await;
  }
}
