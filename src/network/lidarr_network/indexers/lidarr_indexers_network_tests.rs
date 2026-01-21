#[cfg(test)]
mod tests {
  use crate::models::HorizontallyScrollableText;
  use crate::models::lidarr_models::LidarrSerdeable;
  use crate::models::servarr_data::modals::IndexerTestResultModalItem;
  use crate::models::servarr_models::{EditIndexerParams, Indexer, IndexerTestResult};
  use crate::network::NetworkResource;
  use crate::network::lidarr_network::LidarrEvent;
  use crate::network::lidarr_network::lidarr_network_test_utils::test_utils::{
    indexer, indexer_settings,
  };
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use bimap::BiMap;
  use mockito::Matcher;
  use pretty_assertions::assert_eq;
  use serde_json::json;

  #[tokio::test]
  async fn test_handle_delete_lidarr_indexer_event() {
    let (mock, app, _server) = MockServarrApi::delete()
      .path("/1")
      .build_for(LidarrEvent::DeleteIndexer(1))
      .await;
    app
      .lock()
      .await
      .data
      .lidarr_data
      .indexers
      .set_items(vec![indexer()]);
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    assert!(
      network
        .handle_lidarr_event(LidarrEvent::DeleteIndexer(1))
        .await
        .is_ok()
    );

    mock.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_all_indexer_settings_event() {
    let indexer_settings_json = json!({
        "id": 1,
        "minimumAge": 1,
        "maximumSize": 12345,
        "retention": 1,
        "rssSyncInterval": 60
    });
    let (mock, app, _server) = MockServarrApi::put()
      .with_request_body(indexer_settings_json)
      .build_for(LidarrEvent::EditAllIndexerSettings(indexer_settings()))
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    assert_ok!(
      network
        .handle_lidarr_event(LidarrEvent::EditAllIndexerSettings(indexer_settings()))
        .await
    );

    mock.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_lidarr_indexer_event() {
    let expected_edit_indexer_params = EditIndexerParams {
      indexer_id: 1,
      name: Some("Test Update".to_owned()),
      enable_rss: Some(false),
      enable_automatic_search: Some(false),
      enable_interactive_search: Some(false),
      url: Some("https://localhost:9696/1/".to_owned()),
      api_key: Some("test1234".to_owned()),
      seed_ratio: Some("1.3".to_owned()),
      tag_input_string: Some("usenet, testing".to_owned()),
      priority: Some(0),
      ..EditIndexerParams::default()
    };
    let indexer_details_json = json!({
        "enableRss": true,
        "enableAutomaticSearch": true,
        "enableInteractiveSearch": true,
        "name": "Test Indexer",
        "priority": 1,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://test.com",
            },
            {
                "name": "apiKey",
                "value": "",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.2",
            },
        ],
        "tags": [1],
        "id": 1
    });
    let expected_indexer_edit_body_json = json!({
        "enableRss": false,
        "enableAutomaticSearch": false,
        "enableInteractiveSearch": false,
        "name": "Test Update",
        "priority": 0,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://localhost:9696/1/",
            },
            {
                "name": "apiKey",
                "value": "test1234",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.3",
            },
        ],
        "tags": [1, 2],
        "id": 1
    });
    let (mock_details_server, app, mut server) = MockServarrApi::get()
      .returns(indexer_details_json)
      .path("/1")
      .build_for(LidarrEvent::GetIndexers)
      .await;
    let mock_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v1{}/1?forceSave=true",
          LidarrEvent::EditIndexer(expected_edit_indexer_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_indexer_edit_body_json))
      .create_async()
      .await;
    app.lock().await.data.lidarr_data.tags_map =
      BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    assert_ok!(
      network
        .handle_lidarr_event(LidarrEvent::EditIndexer(expected_edit_indexer_params))
        .await
    );

    mock_details_server.assert_async().await;
    mock_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_lidarr_indexer_event_does_not_overwrite_tags_vec_if_tag_input_string_is_none()
   {
    let expected_edit_indexer_params = EditIndexerParams {
      indexer_id: 1,
      name: Some("Test Update".to_owned()),
      enable_rss: Some(false),
      enable_automatic_search: Some(false),
      enable_interactive_search: Some(false),
      url: Some("https://localhost:9696/1/".to_owned()),
      api_key: Some("test1234".to_owned()),
      seed_ratio: Some("1.3".to_owned()),
      tags: Some(vec![1, 2]),
      priority: Some(0),
      ..EditIndexerParams::default()
    };
    let indexer_details_json = json!({
        "enableRss": true,
        "enableAutomaticSearch": true,
        "enableInteractiveSearch": true,
        "name": "Test Indexer",
        "priority": 1,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://test.com",
            },
            {
                "name": "apiKey",
                "value": "",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.2",
            },
        ],
        "tags": [1],
        "id": 1
    });
    let expected_indexer_edit_body_json = json!({
        "enableRss": false,
        "enableAutomaticSearch": false,
        "enableInteractiveSearch": false,
        "name": "Test Update",
        "priority": 0,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://localhost:9696/1/",
            },
            {
                "name": "apiKey",
                "value": "test1234",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.3",
            },
        ],
        "tags": [1, 2],
        "id": 1
    });
    let (mock_details_server, app, mut server) = MockServarrApi::get()
      .returns(indexer_details_json)
      .path("/1")
      .build_for(LidarrEvent::GetIndexers)
      .await;
    let mock_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v1{}/1?forceSave=true",
          LidarrEvent::EditIndexer(expected_edit_indexer_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_indexer_edit_body_json))
      .create_async()
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    assert_ok!(
      network
        .handle_lidarr_event(LidarrEvent::EditIndexer(expected_edit_indexer_params))
        .await
    );

    mock_details_server.assert_async().await;
    mock_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_lidarr_indexer_event_does_not_add_seed_ratio_when_seed_ratio_field_is_none_in_details()
   {
    let expected_edit_indexer_params = EditIndexerParams {
      indexer_id: 1,
      name: Some("Test Update".to_owned()),
      enable_rss: Some(false),
      enable_automatic_search: Some(false),
      enable_interactive_search: Some(false),
      url: Some("https://localhost:9696/1/".to_owned()),
      api_key: Some("test1234".to_owned()),
      seed_ratio: Some("1.3".to_owned()),
      tag_input_string: Some("usenet, testing".to_owned()),
      priority: Some(0),
      ..EditIndexerParams::default()
    };
    let indexer_details_json = json!({
        "enableRss": true,
        "enableAutomaticSearch": true,
        "enableInteractiveSearch": true,
        "name": "Test Indexer",
        "priority": 1,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://test.com",
            },
            {
                "name": "apiKey",
                "value": "",
            },
        ],
        "tags": [1],
        "id": 1
    });
    let expected_indexer_edit_body_json = json!({
        "enableRss": false,
        "enableAutomaticSearch": false,
        "enableInteractiveSearch": false,
        "name": "Test Update",
        "priority": 0,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://localhost:9696/1/",
            },
            {
                "name": "apiKey",
                "value": "test1234",
            },
        ],
        "tags": [1, 2],
        "id": 1
    });

    let (mock_details_server, app, mut server) = MockServarrApi::get()
      .returns(indexer_details_json)
      .path("/1")
      .build_for(LidarrEvent::GetIndexers)
      .await;
    let mock_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v1{}/1?forceSave=true",
          LidarrEvent::EditIndexer(expected_edit_indexer_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_indexer_edit_body_json))
      .create_async()
      .await;
    app.lock().await.data.lidarr_data.tags_map =
      BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    assert_ok!(
      network
        .handle_lidarr_event(LidarrEvent::EditIndexer(expected_edit_indexer_params))
        .await
    );

    mock_details_server.assert_async().await;
    mock_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_lidarr_indexer_event_populates_the_seed_ratio_value_when_seed_ratio_field_is_present_in_details()
   {
    let expected_edit_indexer_params = EditIndexerParams {
      indexer_id: 1,
      name: Some("Test Update".to_owned()),
      enable_rss: Some(false),
      enable_automatic_search: Some(false),
      enable_interactive_search: Some(false),
      url: Some("https://localhost:9696/1/".to_owned()),
      api_key: Some("test1234".to_owned()),
      seed_ratio: Some("1.3".to_owned()),
      tag_input_string: Some("usenet, testing".to_owned()),
      priority: Some(0),
      ..EditIndexerParams::default()
    };
    let indexer_details_json = json!({
        "enableRss": true,
        "enableAutomaticSearch": true,
        "enableInteractiveSearch": true,
        "name": "Test Indexer",
        "priority": 1,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://test.com",
            },
            {
                "name": "apiKey",
                "value": "",
            },
            {
                "name": "seedCriteria.seedRatio",
            },
        ],
        "tags": [1],
        "id": 1
    });
    let expected_indexer_edit_body_json = json!({
        "enableRss": false,
        "enableAutomaticSearch": false,
        "enableInteractiveSearch": false,
        "name": "Test Update",
        "priority": 0,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://localhost:9696/1/",
            },
            {
                "name": "apiKey",
                "value": "test1234",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.3",
            },
        ],
        "tags": [1, 2],
        "id": 1
    });

    let (mock_details_server, app, mut server) = MockServarrApi::get()
      .returns(indexer_details_json)
      .path("/1")
      .build_for(LidarrEvent::GetIndexers)
      .await;
    let mock_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v1{}/1?forceSave=true",
          LidarrEvent::EditIndexer(expected_edit_indexer_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_indexer_edit_body_json))
      .create_async()
      .await;
    app.lock().await.data.lidarr_data.tags_map =
      BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    assert_ok!(
      network
        .handle_lidarr_event(LidarrEvent::EditIndexer(expected_edit_indexer_params))
        .await
    );

    mock_details_server.assert_async().await;
    mock_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_lidarr_indexer_event_defaults_to_previous_values() {
    let indexer_details_json = json!({
        "enableRss": true,
        "enableAutomaticSearch": true,
        "enableInteractiveSearch": true,
        "name": "Test Indexer",
        "priority": 1,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://test.com",
            },
            {
                "name": "apiKey",
                "value": "",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.2",
            },
        ],
        "tags": [1],
        "id": 1
    });
    let edit_indexer_params = EditIndexerParams {
      indexer_id: 1,
      ..EditIndexerParams::default()
    };
    let (mock_details_server, app, mut server) = MockServarrApi::get()
      .returns(indexer_details_json.clone())
      .path("/1")
      .build_for(LidarrEvent::GetIndexers)
      .await;
    let mock_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v1{}/1?forceSave=true",
          LidarrEvent::EditIndexer(edit_indexer_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(indexer_details_json))
      .create_async()
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    assert_ok!(
      network
        .handle_lidarr_event(LidarrEvent::EditIndexer(edit_indexer_params))
        .await
    );

    mock_details_server.assert_async().await;
    mock_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_lidarr_indexer_event_clears_tags_when_clear_tags_is_true() {
    let indexer_details_json = json!({
        "enableRss": true,
        "enableAutomaticSearch": true,
        "enableInteractiveSearch": true,
        "name": "Test Indexer",
        "priority": 1,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://test.com",
            },
            {
                "name": "apiKey",
                "value": "",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.2",
            },
        ],
        "tags": [1, 2],
        "id": 1
    });
    let expected_edit_indexer_body = json!({
        "enableRss": true,
        "enableAutomaticSearch": true,
        "enableInteractiveSearch": true,
        "name": "Test Indexer",
        "priority": 1,
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://test.com",
            },
            {
                "name": "apiKey",
                "value": "",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.2",
            },
        ],
        "tags": [],
        "id": 1
    });
    let edit_indexer_params = EditIndexerParams {
      indexer_id: 1,
      clear_tags: true,
      ..EditIndexerParams::default()
    };

    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(indexer_details_json)
      .path("/1")
      .build_for(LidarrEvent::GetIndexers)
      .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v1{}/1?forceSave=true",
          LidarrEvent::EditIndexer(edit_indexer_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_edit_indexer_body))
      .create_async()
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    assert_ok!(
      network
        .handle_lidarr_event(LidarrEvent::EditIndexer(edit_indexer_params))
        .await
    );

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_get_lidarr_indexers_event() {
    let indexers_response_json = json!([{
        "enableRss": true,
        "enableAutomaticSearch": true,
        "enableInteractiveSearch": true,
        "supportsRss": true,
        "supportsSearch": true,
        "protocol": "torrent",
        "priority": 25,
        "downloadClientId": 0,
        "name": "Test Indexer",
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://test.com",
            },
            {
                "name": "apiKey",
                "value": "",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.2",
            },
        ],
        "implementationName": "Torznab",
        "implementation": "Torznab",
        "configContract": "TorznabSettings",
        "tags": [1],
        "id": 1
    }]);
    let response: Vec<Indexer> = serde_json::from_value(indexers_response_json.clone()).unwrap();
    let (async_server, app, _server) = MockServarrApi::get()
      .returns(indexers_response_json)
      .build_for(LidarrEvent::GetIndexers)
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let LidarrSerdeable::Indexers(indexers) = network
      .handle_lidarr_event(LidarrEvent::GetIndexers)
      .await
      .unwrap()
    else {
      panic!("Expected Indexers")
    };

    async_server.assert_async().await;
    assert_eq!(
      app.lock().await.data.lidarr_data.indexers.items,
      vec![indexer()]
    );
    assert_eq!(indexers, response);
  }

  #[tokio::test]
  async fn test_handle_test_lidarr_indexer_event_error() {
    let indexer_details_json = json!({
        "enableRss": true,
        "enableAutomaticSearch": true,
        "enableInteractiveSearch": true,
        "name": "Test Indexer",
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://test.com",
            },
            {
                "name": "apiKey",
                "value": "",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.2",
            },
        ],
        "tags": [1],
        "id": 1
    });
    let response_json = json!([
    {
        "isWarning": false,
        "propertyName": "",
        "errorMessage": "test failure",
        "severity": "error"
    }]);
    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(indexer_details_json.clone())
      .path("/1")
      .build_for(LidarrEvent::GetIndexers)
      .await;
    let async_test_server = server
      .mock(
        "POST",
        format!("/api/v1{}", LidarrEvent::TestIndexer(1).resource()).as_str(),
      )
      .with_status(400)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(indexer_details_json.clone()))
      .with_body(response_json.to_string())
      .create_async()
      .await;
    app
      .lock()
      .await
      .data
      .lidarr_data
      .indexers
      .set_items(vec![indexer()]);
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let LidarrSerdeable::Value(value) = network
      .handle_lidarr_event(LidarrEvent::TestIndexer(1))
      .await
      .unwrap()
    else {
      panic!("Expected Value")
    };

    async_details_server.assert_async().await;
    async_test_server.assert_async().await;
    assert_eq!(
      app.lock().await.data.lidarr_data.indexer_test_errors,
      Some("\"test failure\"".to_owned())
    );
    assert_eq!(value, response_json);
  }

  #[tokio::test]
  async fn test_handle_test_lidarr_indexer_event_success() {
    let indexer_details_json = json!({
        "enableRss": true,
        "enableAutomaticSearch": true,
        "enableInteractiveSearch": true,
        "name": "Test Indexer",
        "fields": [
            {
                "name": "baseUrl",
                "value": "https://test.com",
            },
            {
                "name": "apiKey",
                "value": "",
            },
            {
                "name": "seedCriteria.seedRatio",
                "value": "1.2",
            },
        ],
        "tags": [1],
        "id": 1
    });
    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(indexer_details_json.clone())
      .path("/1")
      .build_for(LidarrEvent::GetIndexers)
      .await;
    let async_test_server = server
      .mock(
        "POST",
        format!("/api/v1{}", LidarrEvent::TestIndexer(1).resource()).as_str(),
      )
      .with_status(200)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(indexer_details_json.clone()))
      .with_body("{}")
      .create_async()
      .await;
    app
      .lock()
      .await
      .data
      .lidarr_data
      .indexers
      .set_items(vec![indexer()]);
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let LidarrSerdeable::Value(value) = network
      .handle_lidarr_event(LidarrEvent::TestIndexer(1))
      .await
      .unwrap()
    else {
      panic!("Expected Value")
    };
    async_details_server.assert_async().await;
    async_test_server.assert_async().await;
    assert_eq!(
      app.lock().await.data.lidarr_data.indexer_test_errors,
      Some(String::new())
    );
    assert_eq!(value, json!({}));
  }

  #[tokio::test]
  async fn test_handle_test_all_lidarr_indexers_event() {
    let indexers = vec![
      Indexer {
        id: 1,
        name: Some("Test 1".to_owned()),
        ..Indexer::default()
      },
      Indexer {
        id: 2,
        name: Some("Test 2".to_owned()),
        ..Indexer::default()
      },
    ];
    let indexer_test_results_modal_items = vec![
      IndexerTestResultModalItem {
        name: "Test 1".to_owned(),
        is_valid: true,
        validation_failures: HorizontallyScrollableText::default(),
      },
      IndexerTestResultModalItem {
        name: "Test 2".to_owned(),
        is_valid: false,
        validation_failures: "Failure for field 'test field 1': test error message, Failure for field 'test field 2': test error message 2".into(),
      },
    ];
    let response_json = json!([
    {
      "id": 1,
      "isValid": true,
      "validationFailures": []
    },
    {
      "id": 2,
      "isValid": false,
      "validationFailures": [
          {
              "propertyName": "test field 1",
              "errorMessage": "test error message",
              "severity": "error"
          },
          {
              "propertyName": "test field 2",
              "errorMessage": "test error message 2",
              "severity": "error"
          },
      ]
    }]);
    let response: Vec<IndexerTestResult> = serde_json::from_value(response_json.clone()).unwrap();
    let (async_server, app, _server) = MockServarrApi::post()
      .returns(response_json)
      .status(400)
      .build_for(LidarrEvent::TestAllIndexers)
      .await;
    app
      .lock()
      .await
      .data
      .lidarr_data
      .indexers
      .set_items(indexers);
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let LidarrSerdeable::IndexerTestResults(results) = network
      .handle_lidarr_event(LidarrEvent::TestAllIndexers)
      .await
      .unwrap()
    else {
      panic!("Expected IndexerTestResults")
    };
    async_server.assert_async().await;
    assert_some!(&app.lock().await.data.lidarr_data.indexer_test_all_results);
    assert_eq!(
      app
        .lock()
        .await
        .data
        .lidarr_data
        .indexer_test_all_results
        .as_ref()
        .unwrap()
        .items,
      indexer_test_results_modal_items
    );
    assert_eq!(results, response);
  }

  #[tokio::test]
  async fn test_handle_test_all_lidarr_indexers_event_sets_empty_table_on_api_error() {
    let (async_server, app, _server) = MockServarrApi::post()
      .status(500)
      .build_for(LidarrEvent::TestAllIndexers)
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let result = network
      .handle_lidarr_event(LidarrEvent::TestAllIndexers)
      .await;

    async_server.assert_async().await;
    assert_err!(result);
    let app = app.lock().await;
    assert_some!(&app.data.lidarr_data.indexer_test_all_results);
    assert_is_empty!(
      app
        .data
        .lidarr_data
        .indexer_test_all_results
        .as_ref()
        .unwrap()
        .items
    );
  }
}
