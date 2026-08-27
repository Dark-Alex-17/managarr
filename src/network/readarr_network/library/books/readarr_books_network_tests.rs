#[cfg(test)]
mod tests {
  use crate::models::readarr_models::{Book, Edition, ReadarrSerdeable};
  use crate::models::servarr_data::readarr::modals::BookDetailsModal;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::readarr_network::ReadarrEvent;
  use crate::network::readarr_network::readarr_network_test_utils::test_utils::{
    BOOK_JSON, EDITION_JSON, stale_book, stale_edition,
  };
  use pretty_assertions::assert_eq;
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
}
