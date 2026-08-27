#[cfg(test)]
mod tests {
  use crate::models::readarr_models::{
    Book, BookFile, Edition, ReadarrHistoryItem, ReadarrSerdeable,
  };
  use crate::models::servarr_data::readarr::modals::BookDetailsModal;
  use crate::models::servarr_data::readarr::readarr_data::ActiveReadarrBlock;
  use crate::models::stateful_table::SortOption;
  use crate::network::NetworkResource;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::readarr_network::ReadarrEvent;
  use crate::network::readarr_network::readarr_network_test_utils::test_utils::{
    BOOK_FILE_JSON, BOOK_JSON, EDITION_JSON, readarr_history_item, stale_book, stale_book_file,
    stale_edition, stale_readarr_history_item,
  };
  use mockito::Matcher;
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use serde_json::{Value, json};

  #[tokio::test]
  async fn test_handle_get_book_details_event() {
    let expected_book: Book = serde_json::from_str(BOOK_JSON).unwrap();
    let stale_books = vec![stale_book()];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(serde_json::from_str(BOOK_JSON).unwrap())
      .path("/1")
      .build_for(ReadarrEvent::GetBookDetails(1))
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.data.readarr_data.books.set_items(stale_books.clone());
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetBookDetails(1))
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::Book(book) = result.unwrap() else {
      panic!("Expected Book")
    };

    assert_eq!(book, expected_book);
    assert_eq!(app.lock().await.data.readarr_data.books.items, stale_books);
  }

  #[tokio::test]
  async fn test_handle_get_book_details_event_failure() {
    let (mock, app, _server) = MockServarrApi::get()
      .returns(serde_json::from_str(BOOK_JSON).unwrap())
      .status(500)
      .path("/1")
      .build_for(ReadarrEvent::GetBookDetails(1))
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetBookDetails(1))
      .await;

    mock.assert_async().await;

    assert_err!(result);
  }

  #[tokio::test]
  async fn test_handle_get_book_editions_event() {
    let editions_json = json!([
      {
        "id": 3,
        "bookId": 1,
        "foreignEditionId": "test-foreign-edition-id-3",
        "monitored": false,
        "isEbook": true,
        "title": "Third Edition",
        "language": "eng",
        "overview": "the third edition",
        "format": "Kindle Edition",
        "publisher": "Third Publisher",
        "pageCount": 320,
        "releaseDate": "2023-03-01T00:00:00Z",
        "isbn13": "9780000000003",
        "asin": "B000000003",
        "ratings": { "votes": 15, "value": 8.4, "popularity": 1.2 }
      },
      {
        "id": 1,
        "bookId": 1,
        "foreignEditionId": "test-foreign-edition-id-1",
        "monitored": true,
        "isEbook": false,
        "title": "First Edition",
        "language": "eng",
        "overview": "the first edition",
        "format": "Hardcover",
        "publisher": "First Publisher",
        "pageCount": 662,
        "releaseDate": "2023-01-01T00:00:00Z",
        "isbn13": "9780000000001",
        "asin": "B000000001",
        "ratings": { "votes": 15, "value": 8.4, "popularity": 1.2 }
      },
      {
        "id": 2,
        "bookId": 1,
        "foreignEditionId": "test-foreign-edition-id-2",
        "monitored": true,
        "isEbook": false,
        "title": "Second Edition",
        "language": "eng",
        "overview": "the second edition",
        "format": "Paperback",
        "publisher": "Second Publisher",
        "pageCount": 128,
        "releaseDate": "2023-02-01T00:00:00Z",
        "isbn13": "9780000000002",
        "asin": "B000000002",
        "ratings": { "votes": 15, "value": 8.4, "popularity": 1.2 }
      }
    ]);
    let response: Vec<Edition> = serde_json::from_value(editions_json.clone()).unwrap();
    let mut sorted_editions = response.clone();
    sorted_editions.sort_by_key(|edition| edition.id);
    let (mock, app, _server) = MockServarrApi::get()
      .returns(editions_json)
      .query("bookId=1")
      .build_for(ReadarrEvent::GetBookEditions(1))
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.data.readarr_data.book_details_modal = Some(BookDetailsModal::default());
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetBookEditions(1))
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::Editions(editions) = result.unwrap() else {
      panic!("Expected Editions")
    };

    assert_eq!(editions, response);
    assert_eq!(
      app
        .lock()
        .await
        .data
        .readarr_data
        .book_details_modal
        .as_ref()
        .unwrap()
        .editions
        .items,
      sorted_editions
    );
  }

  #[tokio::test]
  async fn test_handle_get_book_editions_event_empty_book_details_modal() {
    let expected_editions: Vec<Edition> = vec![serde_json::from_str(EDITION_JSON).unwrap()];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!(
        [serde_json::from_str::<Value>(EDITION_JSON).unwrap()]
      ))
      .query("bookId=1")
      .build_for(ReadarrEvent::GetBookEditions(1))
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetBookEditions(1))
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::Editions(editions) = result.unwrap() else {
      panic!("Expected Editions")
    };

    assert_eq!(editions, expected_editions);

    let app = app.lock().await;

    assert_some!(&app.data.readarr_data.book_details_modal);
    assert_eq!(
      app
        .data
        .readarr_data
        .book_details_modal
        .as_ref()
        .unwrap()
        .editions
        .items,
      expected_editions
    );
  }

  #[tokio::test]
  async fn test_handle_get_book_editions_event_failure() {
    let stale_editions = vec![stale_edition()];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!(
        [serde_json::from_str::<Value>(EDITION_JSON).unwrap()]
      ))
      .status(500)
      .query("bookId=1")
      .build_for(ReadarrEvent::GetBookEditions(1))
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      let mut book_details_modal = BookDetailsModal::default();
      book_details_modal
        .editions
        .set_items(stale_editions.clone());
      app.data.readarr_data.book_details_modal = Some(book_details_modal);
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetBookEditions(1))
      .await;

    mock.assert_async().await;

    assert_err!(result);
    assert_eq!(
      app
        .lock()
        .await
        .data
        .readarr_data
        .book_details_modal
        .as_ref()
        .unwrap()
        .editions
        .items,
      stale_editions
    );
  }

  #[tokio::test]
  async fn test_handle_get_book_files_event() {
    let expected_book_files: Vec<BookFile> = vec![serde_json::from_str(BOOK_FILE_JSON).unwrap()];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!([
        serde_json::from_str::<Value>(BOOK_FILE_JSON).unwrap()
      ]))
      .query("bookId=1")
      .build_for(ReadarrEvent::GetBookFiles(1))
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.data.readarr_data.book_details_modal = Some(BookDetailsModal::default());
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetBookFiles(1))
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::BookFiles(book_files) = result.unwrap() else {
      panic!("Expected BookFiles")
    };

    assert_eq!(book_files, expected_book_files);
    assert_eq!(
      app
        .lock()
        .await
        .data
        .readarr_data
        .book_details_modal
        .as_ref()
        .unwrap()
        .book_files
        .items,
      expected_book_files
    );
  }

  #[tokio::test]
  async fn test_handle_get_book_files_event_empty_book_details_modal() {
    let expected_book_files: Vec<BookFile> = vec![serde_json::from_str(BOOK_FILE_JSON).unwrap()];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!([
        serde_json::from_str::<Value>(BOOK_FILE_JSON).unwrap()
      ]))
      .query("bookId=1")
      .build_for(ReadarrEvent::GetBookFiles(1))
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetBookFiles(1))
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::BookFiles(book_files) = result.unwrap() else {
      panic!("Expected BookFiles")
    };

    assert_eq!(book_files, expected_book_files);

    let app = app.lock().await;

    assert_some!(&app.data.readarr_data.book_details_modal);
    assert_eq!(
      app
        .data
        .readarr_data
        .book_details_modal
        .as_ref()
        .unwrap()
        .book_files
        .items,
      expected_book_files
    );
  }

  #[tokio::test]
  async fn test_handle_get_book_files_event_failure() {
    let stale_book_files = vec![stale_book_file()];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!([
        serde_json::from_str::<Value>(BOOK_FILE_JSON).unwrap()
      ]))
      .status(500)
      .query("bookId=1")
      .build_for(ReadarrEvent::GetBookFiles(1))
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      let mut book_details_modal = BookDetailsModal::default();
      book_details_modal
        .book_files
        .set_items(stale_book_files.clone());
      app.data.readarr_data.book_details_modal = Some(book_details_modal);
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetBookFiles(1))
      .await;

    mock.assert_async().await;

    assert_err!(result);
    assert_eq!(
      app
        .lock()
        .await
        .data
        .readarr_data
        .book_details_modal
        .as_ref()
        .unwrap()
        .book_files
        .items,
      stale_book_files
    );
  }

  #[tokio::test]
  async fn test_handle_get_books_event() {
    let books_json = json!([
      {
        "id": 3,
        "title": "Third Book",
        "authorId": 1,
        "foreignBookId": "test-foreign-book-id-3",
        "monitored": true,
        "anyEditionOk": true,
        "pageCount": 320,
        "ratings": { "votes": 15, "value": 8.4, "popularity": 1.2 },
        "releaseDate": "2023-03-01T00:00:00Z",
        "statistics": {
          "bookFileCount": 2,
          "totalBookCount": 3,
          "sizeOnDisk": 12345,
          "percentOfBooks": 66.6
        },
        "grabbed": false
      },
      {
        "id": 1,
        "title": "First Book",
        "authorId": 1,
        "foreignBookId": "test-foreign-book-id-1",
        "monitored": true,
        "anyEditionOk": true,
        "pageCount": 662,
        "ratings": { "votes": 15, "value": 8.4, "popularity": 1.2 },
        "releaseDate": "2023-01-01T00:00:00Z",
        "statistics": {
          "bookFileCount": 2,
          "totalBookCount": 3,
          "sizeOnDisk": 12345,
          "percentOfBooks": 66.6
        },
        "grabbed": false
      },
      {
        "id": 2,
        "title": "Second Book",
        "authorId": 1,
        "foreignBookId": "test-foreign-book-id-2",
        "monitored": false,
        "anyEditionOk": true,
        "pageCount": 128,
        "ratings": { "votes": 15, "value": 8.4, "popularity": 1.2 },
        "releaseDate": "2023-02-01T00:00:00Z",
        "statistics": {
          "bookFileCount": 2,
          "totalBookCount": 3,
          "sizeOnDisk": 12345,
          "percentOfBooks": 66.6
        },
        "grabbed": false
      }
    ]);
    let response: Vec<Book> = serde_json::from_value(books_json.clone()).unwrap();
    let mut sorted_books = response.clone();
    sorted_books.sort_by_key(|book| book.id);
    let (mock, app, _server) = MockServarrApi::get()
      .returns(books_json)
      .query("authorId=1")
      .build_for(ReadarrEvent::GetBooks(1))
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetBooks(1))
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::Books(books) = result.unwrap() else {
      panic!("Expected Books")
    };

    assert_eq!(books, response);
    assert_eq!(app.lock().await.data.readarr_data.books.items, sorted_books);
  }

  #[tokio::test]
  async fn test_handle_get_books_event_failure() {
    let stale_books = vec![stale_book()];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!([serde_json::from_str::<Value>(BOOK_JSON).unwrap()]))
      .status(500)
      .query("authorId=1")
      .build_for(ReadarrEvent::GetBooks(1))
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.data.readarr_data.books.set_items(stale_books.clone());
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetBooks(1))
      .await;

    mock.assert_async().await;

    assert_err!(result);
    assert_eq!(app.lock().await.data.readarr_data.books.items, stale_books);
  }

  #[rstest]
  #[tokio::test]
  async fn test_handle_get_readarr_book_history_event(
    #[values(true, false)] use_custom_sorting: bool,
  ) {
    let history_json = json!([{
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
    }]);
    let response: Vec<ReadarrHistoryItem> = serde_json::from_value(history_json.clone()).unwrap();
    let mut expected_history_items = vec![
      ReadarrHistoryItem {
        id: 123,
        author_id: 1007,
        book_id: 1007,
        source_title: "z book".into(),
        ..readarr_history_item()
      },
      ReadarrHistoryItem {
        id: 456,
        author_id: 2001,
        book_id: 2001,
        source_title: "An Anthology".into(),
        ..readarr_history_item()
      },
    ];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(history_json)
      .query("authorId=1&bookId=2")
      .build_for(ReadarrEvent::GetBookHistory(1, 2))
      .await;
    app.lock().await.data.readarr_data.book_details_modal = Some(BookDetailsModal::default());
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
        .book_details_modal
        .as_mut()
        .unwrap()
        .book_history
        .sorting(vec![history_sort_option]);
    }
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app
        .data
        .readarr_data
        .book_details_modal
        .as_mut()
        .unwrap()
        .book_history
        .sort_asc = true;
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetBookHistory(1, 2))
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::ReadarrHistoryItems(history_items) = result.unwrap() else {
      panic!("Expected ReadarrHistoryItems")
    };

    assert_eq!(history_items, response);
    let app = app.lock().await;
    let book_details_modal = app.data.readarr_data.book_details_modal.as_ref().unwrap();
    assert_eq!(
      book_details_modal.book_history.items,
      expected_history_items
    );
    assert!(book_details_modal.book_history.sort_asc);
  }

  #[tokio::test]
  async fn test_handle_get_readarr_book_history_event_no_op_when_user_is_selecting_sort_options() {
    let history_json = json!([{
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
    }]);
    let response: Vec<ReadarrHistoryItem> = serde_json::from_value(history_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(history_json)
      .query("authorId=1&bookId=2")
      .build_for(ReadarrEvent::GetBookHistory(1, 2))
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      let mut book_details_modal = BookDetailsModal::default();
      book_details_modal.book_history.sort_asc = true;
      app.data.readarr_data.book_details_modal = Some(book_details_modal);
      app.push_navigation_stack(ActiveReadarrBlock::BookHistorySortPrompt.into());
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetBookHistory(1, 2))
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::ReadarrHistoryItems(history_items) = result.unwrap() else {
      panic!("Expected ReadarrHistoryItems")
    };

    assert_eq!(history_items, response);
    let app = app.lock().await;
    let book_details_modal = app.data.readarr_data.book_details_modal.as_ref().unwrap();
    assert_is_empty!(book_details_modal.book_history);
    assert!(book_details_modal.book_history.sort_asc);
  }

  #[tokio::test]
  async fn test_handle_get_readarr_book_history_event_empty_book_details_modal() {
    let history_json = json!([{
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
    }]);
    let expected_history_items = vec![ReadarrHistoryItem {
      id: 123,
      author_id: 1007,
      book_id: 1007,
      source_title: "z book".into(),
      ..readarr_history_item()
    }];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(history_json)
      .query("authorId=1&bookId=2")
      .build_for(ReadarrEvent::GetBookHistory(1, 2))
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetBookHistory(1, 2))
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::ReadarrHistoryItems(history_items) = result.unwrap() else {
      panic!("Expected ReadarrHistoryItems")
    };

    assert_eq!(history_items, expected_history_items);

    let app = app.lock().await;

    assert_some!(&app.data.readarr_data.book_details_modal);
    assert_eq!(
      app
        .data
        .readarr_data
        .book_details_modal
        .as_ref()
        .unwrap()
        .book_history
        .items,
      expected_history_items
    );
  }

  #[tokio::test]
  async fn test_handle_get_readarr_book_history_event_failure() {
    let history_json = json!([{
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
    }]);
    let stale_history_items = vec![stale_readarr_history_item()];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(history_json)
      .status(500)
      .query("authorId=1&bookId=2")
      .build_for(ReadarrEvent::GetBookHistory(1, 2))
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      let mut book_details_modal = BookDetailsModal::default();
      book_details_modal
        .book_history
        .set_items(stale_history_items.clone());
      app.data.readarr_data.book_details_modal = Some(book_details_modal);
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetBookHistory(1, 2))
      .await;

    mock.assert_async().await;

    assert_err!(result);
    assert_eq!(
      app
        .lock()
        .await
        .data
        .readarr_data
        .book_details_modal
        .as_ref()
        .unwrap()
        .book_history
        .items,
      stale_history_items
    );
  }

  #[rstest]
  #[case(true, false)]
  #[case(false, true)]
  #[tokio::test]
  async fn test_handle_toggle_book_monitoring_event(
    #[case] initial_monitored: bool,
    #[case] expected_monitored: bool,
  ) {
    let mut book_json: Value = serde_json::from_str(BOOK_JSON).unwrap();
    *book_json.get_mut("monitored").unwrap() = json!(initial_monitored);
    let mut expected_body = book_json.clone();
    *expected_body.get_mut("monitored").unwrap() = json!(expected_monitored);
    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(book_json)
      .path("/1")
      .build_for(ReadarrEvent::GetBookDetails(1))
      .await;
    let async_toggle_server = server
      .mock(
        "PUT",
        format!(
          "/api/v1{}/1",
          ReadarrEvent::ToggleBookMonitoring(1).resource()
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
      .handle_readarr_event(ReadarrEvent::ToggleBookMonitoring(1))
      .await;

    async_details_server.assert_async().await;
    async_toggle_server.assert_async().await;
    assert_ok!(result);
  }

  #[tokio::test]
  async fn test_handle_toggle_book_monitoring_event_failure() {
    let (async_details_server, app, mut server) = MockServarrApi::get()
      .returns(serde_json::from_str(BOOK_JSON).unwrap())
      .status(404)
      .path("/1")
      .build_for(ReadarrEvent::GetBookDetails(1))
      .await;
    let async_toggle_server = server
      .mock(
        "PUT",
        format!(
          "/api/v1{}/1",
          ReadarrEvent::ToggleBookMonitoring(1).resource()
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
      .handle_readarr_event(ReadarrEvent::ToggleBookMonitoring(1))
      .await;

    async_details_server.assert_async().await;
    async_toggle_server.assert_async().await;
    assert_err!(result);
  }

  #[tokio::test]
  async fn test_handle_trigger_automatic_book_search_event() {
    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(json!({
        "name": "BookSearch",
        "bookIds": [1]
      }))
      .returns(json!({}))
      .build_for(ReadarrEvent::TriggerAutomaticBookSearch(1))
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::TriggerAutomaticBookSearch(1))
      .await;

    mock.assert_async().await;
    assert_ok!(result);
  }

  #[tokio::test]
  async fn test_handle_trigger_automatic_book_search_event_failure() {
    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(json!({
        "name": "BookSearch",
        "bookIds": [1]
      }))
      .returns(json!({"name": "BookSearch", "status": "queued"}))
      .status(500)
      .build_for(ReadarrEvent::TriggerAutomaticBookSearch(1))
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::TriggerAutomaticBookSearch(1))
      .await;

    mock.assert_async().await;
    assert_err!(result);
  }
}
