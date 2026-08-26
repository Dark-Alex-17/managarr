#[cfg(test)]
mod tests {
  use crate::models::readarr_models::{Author, ReadarrSerdeable};
  use crate::models::servarr_data::readarr::readarr_data::ActiveReadarrBlock;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::readarr_network::ReadarrEvent;
  use crate::network::readarr_network::readarr_network_test_utils::test_utils::{
    AUTHOR_JSON, stale_author,
  };
  use pretty_assertions::assert_eq;
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
}
