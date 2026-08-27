use anyhow::Result;
use log::info;

use crate::models::readarr_models::Book;
use crate::network::readarr_network::ReadarrEvent;
use crate::network::{Network, RequestMethod};

#[cfg(test)]
#[path = "readarr_books_network_tests.rs"]
mod readarr_books_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::readarr_network) async fn get_book_details(
    &mut self,
    book_id: i64,
  ) -> Result<Book> {
    info!("Fetching details for Readarr book with ID: {book_id}");
    let event = ReadarrEvent::GetBookDetails(book_id);

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{book_id}")),
        None,
      )
      .await;

    self
      .handle_request::<(), Book>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::readarr_network) async fn get_books(
    &mut self,
    author_id: i64,
  ) -> Result<Vec<Book>> {
    info!("Fetching books for Readarr author with ID: {author_id}");
    let event = ReadarrEvent::GetBooks(author_id);

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("authorId={author_id}")),
      )
      .await;

    self
      .handle_request::<(), Vec<Book>>(request_props, |mut books_vec, mut app| {
        books_vec.sort_by_key(|a| a.id);
        app.data.readarr_data.books.set_items(books_vec);
      })
      .await
  }
}
