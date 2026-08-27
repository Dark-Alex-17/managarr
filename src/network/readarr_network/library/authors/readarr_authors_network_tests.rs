#[cfg(test)]
mod tests {
  use crate::models::readarr_models::{
    AddAuthorBody, AddAuthorSearchResult, Author, DeleteParams, EditAuthorParams,
    NewItemMonitorType, ReadarrSerdeable,
  };
  use crate::models::servarr_data::readarr::readarr_data::ActiveReadarrBlock;
  use crate::models::stateful_table::StatefulTable;
  use crate::network::NetworkResource;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::readarr_network::ReadarrEvent;
  use crate::network::readarr_network::readarr_network_test_utils::test_utils::{
    ADD_AUTHOR_SEARCH_RESULT_JSON, AUTHOR_JSON, add_author_body, stale_add_author_search_result,
    stale_author,
  };
  use bimap::BiMap;
  use mockito::Matcher;
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use serde_json::{Value, json};

  #[tokio::test]
  async fn test_handle_get_author_details_event() {
    let expected_author: Author = serde_json::from_str(AUTHOR_JSON).unwrap();
    let stale_authors = vec![stale_author()];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(serde_json::from_str(AUTHOR_JSON).unwrap())
      .path("/1")
      .build_for(ReadarrEvent::GetAuthorDetails(1))
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app
        .data
        .readarr_data
        .authors
        .set_items(stale_authors.clone());
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetAuthorDetails(1))
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::Author(author) = result.unwrap() else {
      panic!("Expected Author")
    };

    assert_eq!(author, expected_author);
    assert_eq!(
      app.lock().await.data.readarr_data.authors.items,
      stale_authors
    );
  }

  #[tokio::test]
  async fn test_handle_get_author_details_event_failure() {
    let (mock, app, _server) = MockServarrApi::get()
      .returns(serde_json::from_str(AUTHOR_JSON).unwrap())
      .status(500)
      .path("/1")
      .build_for(ReadarrEvent::GetAuthorDetails(1))
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetAuthorDetails(1))
      .await;

    mock.assert_async().await;

    assert_err!(result);
  }

  #[tokio::test]
  async fn test_handle_list_authors_event() {
    let authors_json = json!([
      {
        "id": 2,
        "authorName": "Zeta Author",
        "foreignAuthorId": "foreign-author-2",
        "status": "continuing",
        "overview": "An overview of Zeta Author",
        "path": "/nfs/books/Zeta Author",
        "rootFolderPath": "/nfs/books/",
        "qualityProfileId": 1,
        "metadataProfileId": 1,
        "monitored": true,
        "monitorNewItems": "all",
        "genres": ["Fantasy"],
        "tags": [1]
      },
      {
        "id": 1,
        "authorName": "Alpha Author",
        "foreignAuthorId": "foreign-author-1",
        "status": "continuing",
        "overview": "An overview of Alpha Author",
        "path": "/nfs/books/Alpha Author",
        "rootFolderPath": "/nfs/books/",
        "qualityProfileId": 1,
        "metadataProfileId": 1,
        "monitored": true,
        "monitorNewItems": "all",
        "genres": ["Fantasy"],
        "tags": [1]
      }
    ]);
    let response: Vec<Author> = serde_json::from_value(authors_json.clone()).unwrap();
    let mut sorted_authors = response.clone();
    sorted_authors.sort_by_key(|author| author.id);
    let (mock, app, _server) = MockServarrApi::get()
      .returns(authors_json)
      .build_for(ReadarrEvent::ListAuthors)
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::ListAuthors)
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::Authors(authors) = result.unwrap() else {
      panic!("Expected Authors")
    };

    assert_eq!(authors, response);
    assert_eq!(
      app.lock().await.data.readarr_data.authors.items,
      sorted_authors
    );
  }

  #[tokio::test]
  async fn test_handle_list_authors_event_no_op_when_user_is_selecting_sort_options() {
    let authors_json = json!([serde_json::from_str::<Value>(AUTHOR_JSON).unwrap()]);
    let (mock, app, _server) = MockServarrApi::get()
      .returns(authors_json)
      .build_for(ReadarrEvent::ListAuthors)
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.push_navigation_stack(ActiveReadarrBlock::AuthorsSortPrompt.into());
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::ListAuthors)
      .await;

    mock.assert_async().await;

    assert_ok!(result);
    assert_is_empty!(app.lock().await.data.readarr_data.authors);
  }

  #[tokio::test]
  async fn test_handle_list_authors_event_failure() {
    let stale_authors = vec![stale_author()];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!({}))
      .status(500)
      .build_for(ReadarrEvent::ListAuthors)
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app
        .data
        .readarr_data
        .authors
        .set_items(stale_authors.clone());
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::ListAuthors)
      .await;

    mock.assert_async().await;

    assert_err!(result);
    assert_eq!(
      app.lock().await.data.readarr_data.authors.items,
      stale_authors
    );
  }

  #[tokio::test]
  async fn test_handle_search_new_author_event() {
    let search_results_json =
      json!([serde_json::from_str::<Value>(ADD_AUTHOR_SEARCH_RESULT_JSON).unwrap()]);
    let expected_results: Vec<AddAuthorSearchResult> =
      serde_json::from_value(search_results_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(search_results_json)
      .query("term=test%20%26%20author")
      .build_for(ReadarrEvent::SearchNewAuthor("test & author".to_owned()))
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::SearchNewAuthor("test & author".to_owned()))
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::AddAuthorSearchResults(search_results) = result.unwrap() else {
      panic!("Expected AddAuthorSearchResults")
    };

    assert_eq!(search_results, expected_results);
    assert_eq!(
      app
        .lock()
        .await
        .data
        .readarr_data
        .add_searched_authors
        .as_ref()
        .unwrap()
        .items,
      expected_results
    );
  }

  #[tokio::test]
  async fn test_handle_search_new_author_event_navigates_to_empty_results_when_empty() {
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!([]))
      .query("term=nonexistent")
      .build_for(ReadarrEvent::SearchNewAuthor("nonexistent".to_owned()))
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.push_navigation_stack(ActiveReadarrBlock::AddAuthorSearchResults.into());
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::SearchNewAuthor("nonexistent".to_owned()))
      .await;

    mock.assert_async().await;

    assert_ok!(result);
    let app = app.lock().await;
    assert_none!(&app.data.readarr_data.add_searched_authors);
    assert_eq!(
      app.get_current_route(),
      ActiveReadarrBlock::AddAuthorEmptySearchResults.into()
    );
  }

  #[tokio::test]
  async fn test_handle_search_new_author_event_sets_empty_table_on_api_error() {
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!([serde_json::from_str::<Value>(
        ADD_AUTHOR_SEARCH_RESULT_JSON
      )
      .unwrap()]))
      .status(500)
      .query("term=nonexistent")
      .build_for(ReadarrEvent::SearchNewAuthor("nonexistent".to_owned()))
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      let mut stale_table = StatefulTable::default();
      stale_table.set_items(vec![stale_add_author_search_result()]);
      app.data.readarr_data.add_searched_authors = Some(stale_table);
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::SearchNewAuthor("nonexistent".to_owned()))
      .await;

    mock.assert_async().await;

    assert_err!(result);
    let app = app.lock().await;
    assert_some!(&app.data.readarr_data.add_searched_authors);
    assert_is_empty!(app.data.readarr_data.add_searched_authors.as_ref().unwrap());
  }

  #[tokio::test]
  async fn test_handle_add_author_event() {
    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(json!({
        "foreignAuthorId": "test-foreign-id",
        "authorName": "Test Author",
        "monitored": true,
        "rootFolderPath": "/nfs/books",
        "qualityProfileId": 1,
        "metadataProfileId": 1,
        "tags": [1, 2],
        "addOptions": {
          "monitor": "all",
          "monitorNewItems": "all",
          "searchForMissingBooks": true
        }
      }))
      .returns(json!({"id": 1}))
      .build_for(ReadarrEvent::AddAuthor(AddAuthorBody::default()))
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.data.readarr_data.tags_map =
        BiMap::from_iter([(1, "usenet".to_owned()), (2, "testing".to_owned())]);
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::AddAuthor(add_author_body()))
      .await;

    mock.assert_async().await;
    assert_ok!(result);
  }

  #[tokio::test]
  async fn test_handle_add_author_event_does_not_overwrite_tags_vec_when_tag_input_string_is_none()
  {
    let add_author_body = AddAuthorBody {
      tags: vec![1, 2],
      tag_input_string: None,
      ..add_author_body()
    };
    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(json!({
        "foreignAuthorId": "test-foreign-id",
        "authorName": "Test Author",
        "monitored": true,
        "rootFolderPath": "/nfs/books",
        "qualityProfileId": 1,
        "metadataProfileId": 1,
        "tags": [1, 2],
        "addOptions": {
          "monitor": "all",
          "monitorNewItems": "all",
          "searchForMissingBooks": true
        }
      }))
      .returns(json!({"id": 1}))
      .build_for(ReadarrEvent::AddAuthor(add_author_body.clone()))
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::AddAuthor(add_author_body))
      .await;

    mock.assert_async().await;
    assert_ok!(result);
  }

  #[tokio::test]
  async fn test_handle_add_author_event_failure() {
    let add_author_body = AddAuthorBody {
      tag_input_string: None,
      ..add_author_body()
    };
    let (mock, app, _server) = MockServarrApi::post()
      .returns(json!({"id": 1}))
      .status(400)
      .build_for(ReadarrEvent::AddAuthor(add_author_body.clone()))
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::AddAuthor(add_author_body))
      .await;

    mock.assert_async().await;
    assert_err!(result);
  }

  #[tokio::test]
  async fn test_handle_edit_author_event() {
    let mut expected_body: Value = serde_json::from_str(AUTHOR_JSON).unwrap();
    *expected_body.get_mut("monitored").unwrap() = json!(false);
    *expected_body.get_mut("monitorNewItems").unwrap() = json!("none");
    *expected_body.get_mut("qualityProfileId").unwrap() = json!(1111);
    *expected_body.get_mut("metadataProfileId").unwrap() = json!(2222);
    *expected_body.get_mut("path").unwrap() = json!("/nfs/Test Path");
    *expected_body.get_mut("tags").unwrap() = json!([1, 2]);
    let edit_author_params = EditAuthorParams {
      author_id: 1,
      monitored: Some(false),
      monitor_new_items: Some(NewItemMonitorType::None),
      quality_profile_id: Some(1111),
      metadata_profile_id: Some(2222),
      root_folder_path: Some("/nfs/Test Path".to_owned()),
      tag_input_string: Some("usenet, testing".to_owned()),
      ..EditAuthorParams::default()
    };
    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(serde_json::from_str(AUTHOR_JSON).unwrap())
      .path("/1")
      .build_for(ReadarrEvent::GetAuthorDetails(1))
      .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v1{}/1",
          ReadarrEvent::EditAuthor(edit_author_params.clone()).resource()
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
      app
        .data
        .readarr_data
        .authors
        .set_items(vec![stale_author()]);
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::EditAuthor(edit_author_params))
      .await;

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
    assert_ok!(result);
    let app = app.lock().await;
    assert_eq!(app.data.readarr_data.authors.items, vec![stale_author()]);
  }

  #[tokio::test]
  async fn test_handle_edit_author_event_does_not_overwrite_tags_vec_when_tag_input_string_is_none()
  {
    let mut expected_body: Value = serde_json::from_str(AUTHOR_JSON).unwrap();
    *expected_body.get_mut("monitored").unwrap() = json!(false);
    *expected_body.get_mut("monitorNewItems").unwrap() = json!("none");
    *expected_body.get_mut("qualityProfileId").unwrap() = json!(1111);
    *expected_body.get_mut("metadataProfileId").unwrap() = json!(2222);
    *expected_body.get_mut("path").unwrap() = json!("/nfs/Test Path");
    *expected_body.get_mut("tags").unwrap() = json!([1, 2]);
    let edit_author_params = EditAuthorParams {
      author_id: 1,
      monitored: Some(false),
      monitor_new_items: Some(NewItemMonitorType::None),
      quality_profile_id: Some(1111),
      metadata_profile_id: Some(2222),
      root_folder_path: Some("/nfs/Test Path".to_owned()),
      tags: Some(vec![1, 2]),
      ..EditAuthorParams::default()
    };
    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(serde_json::from_str(AUTHOR_JSON).unwrap())
      .path("/1")
      .build_for(ReadarrEvent::GetAuthorDetails(1))
      .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v1{}/1",
          ReadarrEvent::EditAuthor(edit_author_params.clone()).resource()
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
      .handle_readarr_event(ReadarrEvent::EditAuthor(edit_author_params))
      .await;

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
    assert_ok!(result);
  }

  #[tokio::test]
  async fn test_handle_edit_author_event_defaults_to_previous_values() {
    let edit_author_params = EditAuthorParams {
      author_id: 1,
      ..EditAuthorParams::default()
    };
    let expected_body: Value = serde_json::from_str(AUTHOR_JSON).unwrap();
    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(serde_json::from_str(AUTHOR_JSON).unwrap())
      .path("/1")
      .build_for(ReadarrEvent::GetAuthorDetails(1))
      .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v1{}/1",
          ReadarrEvent::EditAuthor(edit_author_params.clone()).resource()
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
      .handle_readarr_event(ReadarrEvent::EditAuthor(edit_author_params))
      .await;

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
    assert_ok!(result);
  }

  #[tokio::test]
  async fn test_handle_edit_author_event_returns_empty_tags_vec_when_clear_tags_is_true() {
    let mut expected_body: Value = serde_json::from_str(AUTHOR_JSON).unwrap();
    *expected_body.get_mut("tags").unwrap() = json!([]);
    let edit_author_params = EditAuthorParams {
      author_id: 1,
      clear_tags: true,
      ..EditAuthorParams::default()
    };
    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(serde_json::from_str(AUTHOR_JSON).unwrap())
      .path("/1")
      .build_for(ReadarrEvent::GetAuthorDetails(1))
      .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v1{}/1",
          ReadarrEvent::EditAuthor(edit_author_params.clone()).resource()
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
      .handle_readarr_event(ReadarrEvent::EditAuthor(edit_author_params))
      .await;

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
    assert_ok!(result);
  }

  #[tokio::test]
  async fn test_handle_edit_author_event_failure() {
    let edit_author_params = EditAuthorParams {
      author_id: 1,
      monitored: Some(false),
      ..EditAuthorParams::default()
    };
    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(serde_json::from_str(AUTHOR_JSON).unwrap())
      .status(404)
      .path("/1")
      .build_for(ReadarrEvent::GetAuthorDetails(1))
      .await;
    let async_edit_server = server
      .mock(
        "PUT",
        format!(
          "/api/v1{}/1",
          ReadarrEvent::EditAuthor(edit_author_params.clone()).resource()
        )
        .as_str(),
      )
      .with_status(202)
      .match_header("X-Api-Key", "test1234")
      .expect(0)
      .create_async()
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::EditAuthor(edit_author_params))
      .await;

    async_details_server.assert_async().await;
    async_edit_server.assert_async().await;
    assert_err!(result);
  }

  #[rstest]
  #[case(true, false, "deleteFiles=true&addImportListExclusion=false")]
  #[case(false, true, "deleteFiles=false&addImportListExclusion=true")]
  #[tokio::test]
  async fn test_handle_delete_author_event(
    #[case] delete_files: bool,
    #[case] add_import_list_exclusion: bool,
    #[case] expected_query: &str,
  ) {
    let delete_author_params = DeleteParams {
      id: 1,
      delete_files,
      add_import_list_exclusion,
    };
    let (mock, app, _server) = MockServarrApi::delete()
      .path("/1")
      .query(expected_query)
      .build_for(ReadarrEvent::DeleteAuthor(delete_author_params.clone()))
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::DeleteAuthor(delete_author_params))
      .await;

    mock.assert_async().await;
    assert_ok!(result);
  }

  #[tokio::test]
  async fn test_handle_delete_author_event_failure() {
    let delete_author_params = DeleteParams {
      id: 1,
      delete_files: true,
      add_import_list_exclusion: false,
    };
    let (mock, app, _server) = MockServarrApi::delete()
      .path("/1")
      .query("deleteFiles=true&addImportListExclusion=false")
      .status(500)
      .build_for(ReadarrEvent::DeleteAuthor(delete_author_params.clone()))
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::DeleteAuthor(delete_author_params))
      .await;

    mock.assert_async().await;
    assert_err!(result);
  }
}
