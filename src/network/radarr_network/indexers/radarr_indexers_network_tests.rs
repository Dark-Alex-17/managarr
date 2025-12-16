#[cfg(test)]
mod tests {
  use crate::models::HorizontallyScrollableText;
  use crate::models::radarr_models::{IndexerSettings, RadarrSerdeable};
  use crate::models::servarr_data::modals::IndexerTestResultModalItem;
  use crate::models::servarr_models::{EditIndexerParams, Indexer, IndexerTestResult};
  use crate::network::NetworkResource;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::radarr_network::RadarrEvent;
  use crate::network::radarr_network::radarr_network_test_utils::test_utils::{
    indexer, indexer_settings,
  };
  use bimap::BiMap;
  use mockito::Matcher;
  use pretty_assertions::assert_eq;
  use serde_json::json;

  #[tokio::test]
  async fn test_handle_delete_radarr_indexer_event() {
    let (async_server, app, _server) = MockServarrApi::delete()
      .path("/1")
      .build_for(RadarrEvent::DeleteIndexer(1))
      .await;
    let mut network = test_network(&app);

    assert!(
      network
        .handle_radarr_event(RadarrEvent::DeleteIndexer(1))
        .await
        .is_ok()
    );

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_all_radarr_indexer_settings_event() {
    let indexer_settings_json = json!({
        "minimumAge": 0,
        "maximumSize": 0,
        "retention": 0,
        "rssSyncInterval": 60,
        "preferIndexerFlags": false,
        "availabilityDelay": 0,
        "allowHardcodedSubs": true,
        "whitelistedHardcodedSubs": "",
        "id": 1
    });
    let (async_server, app, _server) = MockServarrApi::put()
      .with_request_body(indexer_settings_json)
      .build_for(RadarrEvent::EditAllIndexerSettings(indexer_settings()))
      .await;
    let mut network = test_network(&app);

    assert!(
      network
        .handle_radarr_event(RadarrEvent::EditAllIndexerSettings(indexer_settings()))
        .await
        .is_ok()
    );

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_radarr_indexer_event() {
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
    let edit_indexer_params = EditIndexerParams {
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
    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(indexer_details_json)
      .path("/1")
      .build_for(RadarrEvent::GetIndexers)
      .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v3{}/1?forceSave=true",
          RadarrEvent::EditIndexer(edit_indexer_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_indexer_edit_body_json))
      .create_async()
      .await;
    app.lock().await.data.radarr_data.tags_map =
      BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
    let mut network = test_network(&app);

    assert!(
      network
        .handle_radarr_event(RadarrEvent::EditIndexer(edit_indexer_params))
        .await
        .is_ok()
    );

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_radarr_indexer_event_does_not_overwrite_tags_vec_if_tag_input_string_is_none()
   {
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
    let edit_indexer_params = EditIndexerParams {
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
    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(indexer_details_json)
      .path("/1")
      .build_for(RadarrEvent::GetIndexers)
      .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v3{}/1?forceSave=true",
          RadarrEvent::EditIndexer(edit_indexer_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_indexer_edit_body_json))
      .create_async()
      .await;
    app.lock().await.data.radarr_data.tags_map =
      BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
    let mut network = test_network(&app);

    assert!(
      network
        .handle_radarr_event(RadarrEvent::EditIndexer(edit_indexer_params))
        .await
        .is_ok()
    );

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_radarr_indexer_event_does_not_add_seed_ratio_when_seed_ratio_field_is_none_in_details()
   {
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
    let edit_indexer_params = EditIndexerParams {
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
    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(indexer_details_json)
      .path("/1")
      .build_for(RadarrEvent::GetIndexers)
      .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v3{}/1?forceSave=true",
          RadarrEvent::EditIndexer(edit_indexer_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_indexer_edit_body_json))
      .create_async()
      .await;
    app.lock().await.data.radarr_data.tags_map =
      BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
    let mut network = test_network(&app);

    assert!(
      network
        .handle_radarr_event(RadarrEvent::EditIndexer(edit_indexer_params))
        .await
        .is_ok()
    );

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_radarr_indexer_event_populates_the_seed_ratio_value_when_seed_ratio_field_is_present_in_details()
   {
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
    let edit_indexer_params = EditIndexerParams {
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
    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(indexer_details_json)
      .path("/1")
      .build_for(RadarrEvent::GetIndexers)
      .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v3{}/1?forceSave=true",
          RadarrEvent::EditIndexer(edit_indexer_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_indexer_edit_body_json))
      .create_async()
      .await;
    app.lock().await.data.radarr_data.tags_map =
      BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
    let mut network = test_network(&app);

    assert!(
      network
        .handle_radarr_event(RadarrEvent::EditIndexer(edit_indexer_params))
        .await
        .is_ok()
    );

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_radarr_indexer_event_defaults_to_previous_values() {
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

    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(indexer_details_json.clone())
      .path("/1")
      .build_for(RadarrEvent::GetIndexers)
      .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v3{}/1?forceSave=true",
          RadarrEvent::EditIndexer(edit_indexer_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(indexer_details_json))
      .create_async()
      .await;
    let mut network = test_network(&app);

    assert!(
      network
        .handle_radarr_event(RadarrEvent::EditIndexer(edit_indexer_params))
        .await
        .is_ok()
    );

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_edit_radarr_indexer_event_clears_tags_when_clear_tags_is_true() {
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
      .build_for(RadarrEvent::GetIndexers)
      .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v3{}/1?forceSave=true",
          RadarrEvent::EditIndexer(edit_indexer_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_edit_indexer_body))
      .create_async()
      .await;
    let mut network = test_network(&app);

    assert!(
      network
        .handle_radarr_event(RadarrEvent::EditIndexer(edit_indexer_params))
        .await
        .is_ok()
    );

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_get_radarr_indexers_event() {
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
      .build_for(RadarrEvent::GetIndexers)
      .await;
    let mut network = test_network(&app);

    let RadarrSerdeable::Indexers(indexers) = network
      .handle_radarr_event(RadarrEvent::GetIndexers)
      .await
      .unwrap()
    else {
      panic!("Expected Indexers")
    };
    async_server.assert_async().await;
    assert_eq!(
      app.lock().await.data.radarr_data.indexers.items,
      vec![indexer()]
    );
    assert_eq!(indexers, response);
  }

  #[tokio::test]
  async fn test_handle_get_all_indexer_settings_event() {
    let indexer_settings_response_json = json!({
        "minimumAge": 0,
        "maximumSize": 0,
        "retention": 0,
        "rssSyncInterval": 60,
        "preferIndexerFlags": false,
        "availabilityDelay": 0,
        "allowHardcodedSubs": true,
        "whitelistedHardcodedSubs": "",
        "id": 1
    });
    let response: IndexerSettings =
      serde_json::from_value(indexer_settings_response_json.clone()).unwrap();
    let (async_server, app, _server) = MockServarrApi::get()
      .returns(indexer_settings_response_json)
      .build_for(RadarrEvent::GetAllIndexerSettings)
      .await;
    let mut network = test_network(&app);

    let RadarrSerdeable::IndexerSettings(settings) = network
      .handle_radarr_event(RadarrEvent::GetAllIndexerSettings)
      .await
      .unwrap()
    else {
      panic!("Expected IndexerSettings")
    };
    async_server.assert_async().await;
    assert_eq!(
      app.lock().await.data.radarr_data.indexer_settings,
      Some(indexer_settings())
    );
    assert_eq!(settings, response);
  }

  #[tokio::test]
  async fn test_handle_get_all_indexer_settings_event_no_op_if_already_present() {
    let indexer_settings_response_json = json!({
        "minimumAge": 0,
        "maximumSize": 0,
        "retention": 0,
        "rssSyncInterval": 60,
        "preferIndexerFlags": false,
        "availabilityDelay": 0,
        "allowHardcodedSubs": true,
        "whitelistedHardcodedSubs": "",
        "id": 1
    });
    let (async_server, app, _server) = MockServarrApi::get()
      .returns(indexer_settings_response_json)
      .build_for(RadarrEvent::GetAllIndexerSettings)
      .await;
    app.lock().await.data.radarr_data.indexer_settings = Some(IndexerSettings::default());
    let mut network = test_network(&app);

    assert!(
      network
        .handle_radarr_event(RadarrEvent::GetAllIndexerSettings)
        .await
        .is_ok()
    );

    async_server.assert_async().await;
    assert_eq!(
      app.lock().await.data.radarr_data.indexer_settings,
      Some(IndexerSettings::default())
    );
  }

  #[tokio::test]
  async fn test_handle_test_radarr_indexer_event_error() {
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
      .build_for(RadarrEvent::GetIndexers)
      .await;
    let async_test_server = server
      .mock(
        "POST",
        format!("/api/v3{}", RadarrEvent::TestIndexer(1).resource()).as_str(),
      )
      .with_status(400)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(indexer_details_json.clone()))
      .with_body(response_json.to_string())
      .create_async()
      .await;
    let mut network = test_network(&app);

    let RadarrSerdeable::Value(value) = network
      .handle_radarr_event(RadarrEvent::TestIndexer(1))
      .await
      .unwrap()
    else {
      panic!("Expected Value")
    };
    async_details_server.assert_async().await;
    async_test_server.assert_async().await;
    assert_eq!(
      app.lock().await.data.radarr_data.indexer_test_errors,
      Some("\"test failure\"".to_owned())
    );
    assert_eq!(value, response_json);
  }

  #[tokio::test]
  async fn test_handle_test_radarr_indexer_event_success() {
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
      .build_for(RadarrEvent::GetIndexers)
      .await;
    let async_test_server = server
      .mock(
        "POST",
        format!("/api/v3{}", RadarrEvent::TestIndexer(1).resource()).as_str(),
      )
      .with_status(200)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(indexer_details_json.clone()))
      .with_body("{}")
      .create_async()
      .await;
    let mut network = test_network(&app);

    let RadarrSerdeable::Value(value) = network
      .handle_radarr_event(RadarrEvent::TestIndexer(1))
      .await
      .unwrap()
    else {
      panic!("Expected Value")
    };
    async_details_server.assert_async().await;
    async_test_server.assert_async().await;
    assert_eq!(
      app.lock().await.data.radarr_data.indexer_test_errors,
      Some(String::new())
    );
    assert_eq!(value, json!({}));
  }

  #[tokio::test]
  async fn test_handle_test_all_radarr_indexers_event() {
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
      .build_for(RadarrEvent::TestAllIndexers)
      .await;
    app
      .lock()
      .await
      .data
      .radarr_data
      .indexers
      .set_items(indexers);
    let mut network = test_network(&app);

    let RadarrSerdeable::IndexerTestResults(results) = network
      .handle_radarr_event(RadarrEvent::TestAllIndexers)
      .await
      .unwrap()
    else {
      panic!("Expected IndexerTestResults")
    };
    async_server.assert_async().await;
    assert!(
      app
        .lock()
        .await
        .data
        .radarr_data
        .indexer_test_all_results
        .is_some()
    );
    assert_eq!(
      app
        .lock()
        .await
        .data
        .radarr_data
        .indexer_test_all_results
        .as_ref()
        .unwrap()
        .items,
      indexer_test_results_modal_items
    );
    assert_eq!(results, response);
  }
}
