#[cfg(test)]
mod tests {
  use crate::models::readarr_models::ReadarrSerdeable;
  use crate::models::servarr_models::{EditIndexerParams, Indexer};
  use crate::network::NetworkResource;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::readarr_network::ReadarrEvent;
  use crate::network::readarr_network::readarr_network_test_utils::test_utils::{
    INDEXER_JSON, indexer, stale_indexer,
  };
  use bimap::BiMap;
  use mockito::Matcher;
  use pretty_assertions::assert_eq;
  use serde_json::{Value, json};

  #[tokio::test]
  async fn test_handle_delete_readarr_indexer_event() {
    let (mock, app, _server) = MockServarrApi::delete()
      .path("/8")
      .build_for(ReadarrEvent::DeleteIndexer(8))
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::DeleteIndexer(8))
      .await;

    mock.assert_async().await;
    assert_ok!(result);
  }

  #[tokio::test]
  async fn test_handle_delete_readarr_indexer_event_failure() {
    let (mock, app, _server) = MockServarrApi::delete()
      .path("/8")
      .status(500)
      .build_for(ReadarrEvent::DeleteIndexer(8))
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::DeleteIndexer(8))
      .await;

    mock.assert_async().await;
    assert_err!(result);
  }

  #[tokio::test]
  async fn test_handle_get_readarr_indexers_event() {
    let indexers_response_json = json!([serde_json::from_str::<Value>(INDEXER_JSON).unwrap()]);
    let response: Vec<Indexer> = serde_json::from_value(indexers_response_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(indexers_response_json)
      .build_for(ReadarrEvent::GetIndexers)
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app
        .data
        .readarr_data
        .indexers
        .set_items(vec![stale_indexer()]);
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetIndexers)
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::Indexers(indexers) = result.unwrap() else {
      panic!("Expected Indexers")
    };

    assert_eq!(indexers, response);
    assert_eq!(
      app.lock().await.data.readarr_data.indexers.items,
      vec![indexer()]
    );
  }

  #[tokio::test]
  async fn test_handle_get_readarr_indexers_event_failure() {
    let stale_indexers = vec![stale_indexer()];
    let indexers_response_json = json!([serde_json::from_str::<Value>(INDEXER_JSON).unwrap()]);
    let (mock, app, _server) = MockServarrApi::get()
      .returns(indexers_response_json)
      .status(500)
      .build_for(ReadarrEvent::GetIndexers)
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app
        .data
        .readarr_data
        .indexers
        .set_items(stale_indexers.clone());
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetIndexers)
      .await;

    mock.assert_async().await;

    assert_err!(result);
    assert_eq!(
      app.lock().await.data.readarr_data.indexers.items,
      stale_indexers
    );
  }

  #[tokio::test]
  async fn test_handle_edit_readarr_indexer_event() {
    let mut expected_body: Value = serde_json::from_str(INDEXER_JSON).unwrap();
    *expected_body.get_mut("name").unwrap() = json!("Test Update");
    *expected_body.get_mut("priority").unwrap() = json!(3);
    *expected_body.get_mut("enableRss").unwrap() = json!(false);
    *expected_body.get_mut("enableAutomaticSearch").unwrap() = json!(false);
    *expected_body.get_mut("enableInteractiveSearch").unwrap() = json!(false);
    *expected_body["fields"][0].get_mut("value").unwrap() = json!("https://localhost:9696/8/");
    *expected_body["fields"][1].get_mut("value").unwrap() = json!("test1234");
    *expected_body["fields"][2].get_mut("value").unwrap() = json!("1.3");
    *expected_body.get_mut("tags").unwrap() = json!([1, 2]);
    let edit_indexer_params = EditIndexerParams {
      indexer_id: 8,
      name: Some("Test Update".to_owned()),
      enable_rss: Some(false),
      enable_automatic_search: Some(false),
      enable_interactive_search: Some(false),
      url: Some("https://localhost:9696/8/".to_owned()),
      api_key: Some("test1234".to_owned()),
      seed_ratio: Some("1.3".to_owned()),
      tag_input_string: Some("usenet, testing".to_owned()),
      priority: Some(3),
      ..EditIndexerParams::default()
    };
    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(serde_json::from_str(INDEXER_JSON).unwrap())
      .path("/8")
      .build_for(ReadarrEvent::GetIndexers)
      .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v1{}/8?forceSave=true",
          ReadarrEvent::EditIndexer(edit_indexer_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .create_async()
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.data.readarr_data.tags_map =
        BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::EditIndexer(edit_indexer_params))
      .await;

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
    assert_ok!(result);
  }

  #[tokio::test]
  async fn test_handle_edit_readarr_indexer_event_defaults_to_previous_values() {
    let expected_body: Value = serde_json::from_str(INDEXER_JSON).unwrap();
    let edit_indexer_params = EditIndexerParams {
      indexer_id: 8,
      ..EditIndexerParams::default()
    };
    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(serde_json::from_str(INDEXER_JSON).unwrap())
      .path("/8")
      .build_for(ReadarrEvent::GetIndexers)
      .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v1{}/8?forceSave=true",
          ReadarrEvent::EditIndexer(edit_indexer_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .create_async()
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::EditIndexer(edit_indexer_params))
      .await;

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
    assert_ok!(result);
  }

  #[tokio::test]
  async fn test_handle_edit_readarr_indexer_event_does_not_overwrite_tags_vec_if_tag_input_string_is_none()
   {
    let mut expected_body: Value = serde_json::from_str(INDEXER_JSON).unwrap();
    *expected_body.get_mut("tags").unwrap() = json!([2, 3]);
    let edit_indexer_params = EditIndexerParams {
      indexer_id: 8,
      tags: Some(vec![2, 3]),
      ..EditIndexerParams::default()
    };
    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(serde_json::from_str(INDEXER_JSON).unwrap())
      .path("/8")
      .build_for(ReadarrEvent::GetIndexers)
      .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v1{}/8?forceSave=true",
          ReadarrEvent::EditIndexer(edit_indexer_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .create_async()
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::EditIndexer(edit_indexer_params))
      .await;

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
    assert_ok!(result);
  }

  #[tokio::test]
  async fn test_handle_edit_readarr_indexer_event_clears_tags_when_clear_tags_is_true() {
    let mut expected_body: Value = serde_json::from_str(INDEXER_JSON).unwrap();
    *expected_body.get_mut("tags").unwrap() = json!([]);
    let edit_indexer_params = EditIndexerParams {
      indexer_id: 8,
      clear_tags: true,
      ..EditIndexerParams::default()
    };
    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(serde_json::from_str(INDEXER_JSON).unwrap())
      .path("/8")
      .build_for(ReadarrEvent::GetIndexers)
      .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v1{}/8?forceSave=true",
          ReadarrEvent::EditIndexer(edit_indexer_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .create_async()
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::EditIndexer(edit_indexer_params))
      .await;

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
    assert_ok!(result);
  }

  #[tokio::test]
  async fn test_handle_edit_readarr_indexer_event_does_not_add_seed_ratio_when_seed_ratio_field_is_absent_in_details()
   {
    let mut details_body: Value = serde_json::from_str(INDEXER_JSON).unwrap();
    details_body["fields"].as_array_mut().unwrap().pop();
    let expected_body = details_body.clone();
    let edit_indexer_params = EditIndexerParams {
      indexer_id: 8,
      seed_ratio: Some("1.3".to_owned()),
      ..EditIndexerParams::default()
    };
    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(details_body)
      .path("/8")
      .build_for(ReadarrEvent::GetIndexers)
      .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v1{}/8?forceSave=true",
          ReadarrEvent::EditIndexer(edit_indexer_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .create_async()
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::EditIndexer(edit_indexer_params))
      .await;

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
    assert_ok!(result);
  }

  #[tokio::test]
  async fn test_handle_edit_readarr_indexer_event_populates_the_seed_ratio_value_when_it_is_missing_in_details()
   {
    let mut details_body: Value = serde_json::from_str(INDEXER_JSON).unwrap();
    details_body["fields"][2]
      .as_object_mut()
      .unwrap()
      .remove("value");
    let mut expected_body = details_body.clone();
    expected_body["fields"][2]
      .as_object_mut()
      .unwrap()
      .insert("value".to_owned(), json!("1.3"));
    let edit_indexer_params = EditIndexerParams {
      indexer_id: 8,
      seed_ratio: Some("1.3".to_owned()),
      ..EditIndexerParams::default()
    };
    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(details_body)
      .path("/8")
      .build_for(ReadarrEvent::GetIndexers)
      .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v1{}/8?forceSave=true",
          ReadarrEvent::EditIndexer(edit_indexer_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .create_async()
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::EditIndexer(edit_indexer_params))
      .await;

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
    assert_ok!(result);
  }

  #[tokio::test]
  async fn test_handle_edit_readarr_indexer_event_failure() {
    let edit_indexer_params = EditIndexerParams {
      indexer_id: 8,
      name: Some("Test Update".to_owned()),
      ..EditIndexerParams::default()
    };
    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(serde_json::from_str(INDEXER_JSON).unwrap())
      .path("/8")
      .build_for(ReadarrEvent::GetIndexers)
      .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v1{}/8?forceSave=true",
          ReadarrEvent::EditIndexer(edit_indexer_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(500)
      .match_header("X-Api-Key", "test1234")
      .with_body(INDEXER_JSON)
      .create_async()
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::EditIndexer(edit_indexer_params))
      .await;

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
    assert_err!(result);
  }

  #[tokio::test]
  async fn test_handle_test_readarr_indexer_event() {
    let expected_body: Value = serde_json::from_str(INDEXER_JSON).unwrap();
    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(serde_json::from_str(INDEXER_JSON).unwrap())
      .path("/8")
      .build_for(ReadarrEvent::GetIndexers)
      .await;
    let async_test_server = server
      .mock(
        "POST",
        format!("/api/v1{}", ReadarrEvent::TestIndexer(8).resource()).as_str(),
      )
      .with_status(200)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .with_body("{}")
      .create_async()
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.data.readarr_data.indexer_test_errors = Some("stale error".to_owned());
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::TestIndexer(8))
      .await;

    async_details_server.assert_async().await;
    async_test_server.assert_async().await;
    assert_ok!(&result);
    let app = app.lock().await;
    assert_some_eq_x!(&app.data.readarr_data.indexer_test_errors, &String::new());
  }

  #[tokio::test]
  async fn test_handle_test_readarr_indexer_event_captures_validation_errors() {
    let expected_body: Value = serde_json::from_str(INDEXER_JSON).unwrap();
    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(serde_json::from_str(INDEXER_JSON).unwrap())
      .path("/8")
      .build_for(ReadarrEvent::GetIndexers)
      .await;
    let async_test_server = server
      .mock(
        "POST",
        format!("/api/v1{}", ReadarrEvent::TestIndexer(8).resource()).as_str(),
      )
      .with_status(400)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .with_body(json!([{ "errorMessage": "Unable to connect to indexer" }]).to_string())
      .create_async()
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.data.readarr_data.indexer_test_errors = Some("stale error".to_owned());
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::TestIndexer(8))
      .await;

    async_details_server.assert_async().await;
    async_test_server.assert_async().await;
    assert_ok!(&result);
    let app = app.lock().await;
    assert_some_eq_x!(
      &app.data.readarr_data.indexer_test_errors,
      &"\"Unable to connect to indexer\"".to_owned()
    );
  }

  #[tokio::test]
  async fn test_handle_test_readarr_indexer_event_falls_back_to_a_generic_error_message() {
    let expected_body: Value = serde_json::from_str(INDEXER_JSON).unwrap();
    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(serde_json::from_str(INDEXER_JSON).unwrap())
      .path("/8")
      .build_for(ReadarrEvent::GetIndexers)
      .await;
    let async_test_server = server
      .mock(
        "POST",
        format!("/api/v1{}", ReadarrEvent::TestIndexer(8).resource()).as_str(),
      )
      .with_status(400)
      .match_header("X-Api-Key", "test1234")
      .match_body(Matcher::Json(expected_body))
      .with_body("[]")
      .create_async()
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.data.readarr_data.indexer_test_errors = Some("stale error".to_owned());
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::TestIndexer(8))
      .await;

    async_details_server.assert_async().await;
    async_test_server.assert_async().await;
    assert_ok!(&result);
    let app = app.lock().await;
    assert_some_eq_x!(
      &app.data.readarr_data.indexer_test_errors,
      &"Unknown indexer test error".to_owned()
    );
  }

  #[tokio::test]
  async fn test_handle_test_readarr_indexer_event_failure() {
    let (async_details_server, app, _server) = MockServarrApi::get()
      .returns(serde_json::from_str(INDEXER_JSON).unwrap())
      .path("/8")
      .status(500)
      .build_for(ReadarrEvent::GetIndexers)
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.data.readarr_data.indexer_test_errors = Some("stale error".to_owned());
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::TestIndexer(8))
      .await;

    async_details_server.assert_async().await;
    assert_err!(result);
    let app = app.lock().await;
    assert_some_eq_x!(
      &app.data.readarr_data.indexer_test_errors,
      &"stale error".to_owned()
    );
  }
}
