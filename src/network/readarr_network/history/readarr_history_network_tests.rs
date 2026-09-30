#[cfg(test)]
mod tests {
  use crate::models::readarr_models::{
    ReadarrHistoryItem, ReadarrHistoryWrapper, ReadarrSerdeable,
  };
  use crate::models::servarr_data::readarr::readarr_data::ActiveReadarrBlock;
  use crate::models::stateful_table::SortOption;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::readarr_network::ReadarrEvent;
  use crate::network::readarr_network::readarr_network_test_utils::test_utils::{
    readarr_history_item, stale_readarr_history_item,
  };
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use serde_json::json;

  #[rstest]
  #[tokio::test]
  async fn test_handle_get_readarr_history_event(#[values(true, false)] use_custom_sorting: bool) {
    let history_json = json!({"records": [{
      "id": 456,
      "authorId": 2001,
      "bookId": 2001,
      "sourceTitle": "An Anthology",
      "quality": { "quality": { "name": "AZW3" } },
      "date": "2023-01-01T00:00:00Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/books/Something/cool.azw3",
        "importedPath": "/nfs/books/Test Author/Book 1/Cool.azw3"
      }
    },
    {
      "id": 123,
      "authorId": 1007,
      "bookId": 1007,
      "sourceTitle": "z book",
      "quality": { "quality": { "name": "AZW3" } },
      "date": "2023-01-01T00:00:00Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/books/Something/cool.azw3",
        "importedPath": "/nfs/books/Test Author/Book 1/Cool.azw3"
      }
    }]});
    let anthology_history_item = ReadarrHistoryItem {
      id: 456,
      author_id: 2001,
      book_id: 2001,
      source_title: "An Anthology".into(),
      ..readarr_history_item()
    };
    let z_book_history_item = ReadarrHistoryItem {
      id: 123,
      author_id: 1007,
      book_id: 1007,
      source_title: "z book".into(),
      ..readarr_history_item()
    };
    let expected_response = ReadarrHistoryWrapper {
      records: vec![anthology_history_item.clone(), z_book_history_item.clone()],
    };
    let mut expected_history_items = vec![z_book_history_item, anthology_history_item];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(history_json)
      .query("pageSize=250&sortDirection=descending&sortKey=date")
      .build_for(ReadarrEvent::GetHistory(250))
      .await;
    if use_custom_sorting {
      let cmp_fn = |a: &ReadarrHistoryItem, b: &ReadarrHistoryItem| {
        a.source_title
          .text
          .to_lowercase()
          .cmp(&b.source_title.text.to_lowercase())
      };
      expected_history_items.sort_by(cmp_fn);

      let history_sort_option = SortOption {
        name: "Source Title",
        cmp_fn: Some(cmp_fn),
      };
      app
        .lock()
        .await
        .data
        .readarr_data
        .history
        .sorting(vec![history_sort_option]);
    }
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.data.readarr_data.history.sort_asc = true;
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetHistory(250))
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::ReadarrHistoryWrapper(history) = result.unwrap() else {
      panic!("Expected ReadarrHistoryWrapper")
    };

    assert_eq!(history, expected_response);
    let app = app.lock().await;
    assert_eq!(app.data.readarr_data.history.items, expected_history_items);
    assert!(app.data.readarr_data.history.sort_asc);
  }

  #[tokio::test]
  async fn test_handle_get_readarr_history_event_no_op_when_user_is_selecting_sort_options() {
    let history_json = json!({"records": [{
      "id": 456,
      "authorId": 2001,
      "bookId": 2001,
      "sourceTitle": "An Anthology",
      "quality": { "quality": { "name": "AZW3" } },
      "date": "2023-01-01T00:00:00Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/books/Something/cool.azw3",
        "importedPath": "/nfs/books/Test Author/Book 1/Cool.azw3"
      }
    },
    {
      "id": 123,
      "authorId": 1007,
      "bookId": 1007,
      "sourceTitle": "z book",
      "quality": { "quality": { "name": "AZW3" } },
      "date": "2023-01-01T00:00:00Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/books/Something/cool.azw3",
        "importedPath": "/nfs/books/Test Author/Book 1/Cool.azw3"
      }
    }]});
    let expected_response = ReadarrHistoryWrapper {
      records: vec![
        ReadarrHistoryItem {
          id: 456,
          author_id: 2001,
          book_id: 2001,
          source_title: "An Anthology".into(),
          ..readarr_history_item()
        },
        ReadarrHistoryItem {
          id: 123,
          author_id: 1007,
          book_id: 1007,
          source_title: "z book".into(),
          ..readarr_history_item()
        },
      ],
    };
    let (mock, app, _server) = MockServarrApi::get()
      .returns(history_json)
      .query("pageSize=250&sortDirection=descending&sortKey=date")
      .build_for(ReadarrEvent::GetHistory(250))
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.data.readarr_data.history.sort_asc = true;
      app.push_navigation_stack(ActiveReadarrBlock::HistorySortPrompt.into());
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetHistory(250))
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::ReadarrHistoryWrapper(history) = result.unwrap() else {
      panic!("Expected ReadarrHistoryWrapper")
    };

    assert_eq!(history, expected_response);
    let app = app.lock().await;
    assert_is_empty!(app.data.readarr_data.history);
    assert!(app.data.readarr_data.history.sort_asc);
  }

  #[tokio::test]
  async fn test_handle_get_readarr_history_event_empty_response() {
    let stale_history_items = vec![stale_readarr_history_item()];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!({"records": []}))
      .query("pageSize=250&sortDirection=descending&sortKey=date")
      .build_for(ReadarrEvent::GetHistory(250))
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app
        .data
        .readarr_data
        .history
        .set_items(stale_history_items.clone());
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetHistory(250))
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::ReadarrHistoryWrapper(history) = result.unwrap() else {
      panic!("Expected ReadarrHistoryWrapper")
    };

    assert_is_empty!(history.records);
    assert_is_empty!(app.lock().await.data.readarr_data.history);
  }

  #[tokio::test]
  async fn test_handle_get_readarr_history_event_failure() {
    let history_json = json!({"records": [{
      "id": 123,
      "authorId": 1007,
      "bookId": 1007,
      "sourceTitle": "z book",
      "quality": { "quality": { "name": "AZW3" } },
      "date": "2023-01-01T00:00:00Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/books/Something/cool.azw3",
        "importedPath": "/nfs/books/Test Author/Book 1/Cool.azw3"
      }
    }]});
    let stale_history_items = vec![stale_readarr_history_item()];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(history_json)
      .status(500)
      .query("pageSize=250&sortDirection=descending&sortKey=date")
      .build_for(ReadarrEvent::GetHistory(250))
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app
        .data
        .readarr_data
        .history
        .set_items(stale_history_items.clone());
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetHistory(250))
      .await;

    mock.assert_async().await;

    assert_err!(result);
    assert_eq!(
      app.lock().await.data.readarr_data.history.items,
      stale_history_items
    );
  }

  #[tokio::test]
  async fn test_handle_mark_readarr_history_item_as_failed_event() {
    let history_item_id = 1234i64;
    let (mock, app, _server) = MockServarrApi::post()
      .returns(json!({}))
      .path("/1234")
      .build_for(ReadarrEvent::MarkHistoryItemAsFailed(history_item_id))
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::MarkHistoryItemAsFailed(history_item_id))
      .await;

    mock.assert_async().await;

    assert_ok!(result);
  }

  #[tokio::test]
  async fn test_handle_mark_readarr_history_item_as_failed_event_failure() {
    let history_item_id = 1234i64;
    let (mock, app, _server) = MockServarrApi::post()
      .returns(json!({}))
      .status(500)
      .path("/1234")
      .build_for(ReadarrEvent::MarkHistoryItemAsFailed(history_item_id))
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::MarkHistoryItemAsFailed(history_item_id))
      .await;

    mock.assert_async().await;

    assert_err!(result);
  }
}
