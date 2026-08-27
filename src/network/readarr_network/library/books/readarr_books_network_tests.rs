#[cfg(test)]
mod tests {
  use crate::models::readarr_models::{Book, ReadarrSerdeable};
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::readarr_network::ReadarrEvent;
  use crate::network::readarr_network::readarr_network_test_utils::test_utils::{
    BOOK_JSON, stale_book,
  };
  use pretty_assertions::assert_eq;
  use serde_json::{Value, json};

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
