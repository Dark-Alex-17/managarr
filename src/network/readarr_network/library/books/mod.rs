use anyhow::Result;
use log::{debug, info, warn};
use serde_json::{Value, json};

use crate::models::Route;
use crate::models::readarr_models::{
  Book, BookFile, DeleteParams, Edition, ReadarrCommandBody, ReadarrHistoryItem,
};
use crate::models::servarr_data::readarr::readarr_data::ActiveReadarrBlock;
use crate::network::readarr_network::ReadarrEvent;
use crate::network::{Network, RequestMethod};

#[cfg(test)]
#[path = "readarr_books_network_tests.rs"]
mod readarr_books_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::readarr_network) async fn delete_book(
    &mut self,
    delete_book_params: DeleteParams,
  ) -> Result<()> {
    let event = ReadarrEvent::DeleteBook(DeleteParams::default());
    let DeleteParams {
      id,
      delete_files,
      add_import_list_exclusion,
    } = delete_book_params;

    info!(
      "Deleting Readarr book with ID: {id} with deleteFiles={delete_files} and addImportListExclusion={add_import_list_exclusion}"
    );

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Delete,
        None::<()>,
        Some(format!("/{id}")),
        Some(format!(
          "deleteFiles={delete_files}&addImportListExclusion={add_import_list_exclusion}"
        )),
      )
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }

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

  pub(in crate::network::readarr_network) async fn get_book_editions(
    &mut self,
    book_id: i64,
  ) -> Result<Vec<Edition>> {
    info!("Fetching editions for Readarr book with ID: {book_id}");
    let event = ReadarrEvent::GetBookEditions(book_id);

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("bookId={book_id}")),
      )
      .await;

    self
      .handle_request::<(), Vec<Edition>>(request_props, |mut editions_vec, mut app| {
        editions_vec.sort_by_key(|edition| edition.id);
        let book_details_modal = app
          .data
          .readarr_data
          .book_details_modal
          .get_or_insert_default();

        book_details_modal.editions.set_items(editions_vec);
      })
      .await
  }

  pub(in crate::network::readarr_network) async fn get_book_files(
    &mut self,
    book_id: i64,
  ) -> Result<Vec<BookFile>> {
    info!("Fetching book files for Readarr book with ID: {book_id}");
    let event = ReadarrEvent::GetBookFiles(book_id);

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("bookId={book_id}")),
      )
      .await;

    self
      .handle_request::<(), Vec<BookFile>>(request_props, |book_files_vec, mut app| {
        let book_details_modal = app
          .data
          .readarr_data
          .book_details_modal
          .get_or_insert_default();

        book_details_modal.book_files.set_items(book_files_vec);
      })
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

  pub(in crate::network::readarr_network) async fn get_readarr_book_history(
    &mut self,
    author_id: i64,
    book_id: i64,
  ) -> Result<Vec<ReadarrHistoryItem>> {
    info!("Fetching Readarr book history for book with ID: {book_id}");
    let event = ReadarrEvent::GetBookHistory(author_id, book_id);

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("authorId={author_id}&bookId={book_id}")),
      )
      .await;

    self
      .handle_request::<(), Vec<ReadarrHistoryItem>>(request_props, |mut history_vec, mut app| {
        let is_sorting = matches!(
          app.get_current_route(),
          Route::Readarr(ActiveReadarrBlock::BookHistorySortPrompt, _)
        );

        let book_details_modal = app
          .data
          .readarr_data
          .book_details_modal
          .get_or_insert_default();

        if !is_sorting {
          history_vec.sort_by_key(|item| item.id);
          book_details_modal.book_history.set_items(history_vec);
          book_details_modal.book_history.apply_sorting_toggle(false);
        }
      })
      .await
  }

  pub(in crate::network::readarr_network) async fn toggle_book_monitoring(
    &mut self,
    book_id: i64,
  ) -> Result<()> {
    let event = ReadarrEvent::ToggleBookMonitoring(book_id);

    let detail_event = ReadarrEvent::GetBookDetails(book_id);
    info!("Toggling book monitoring for book with ID: {book_id}");
    info!("Fetching book details for book with ID: {book_id}");

    let request_props = self
      .request_props_from(
        detail_event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{book_id}")),
        None,
      )
      .await;

    let mut response = String::new();

    self
      .handle_request::<(), Value>(request_props, |detailed_book_body, _| {
        response = detailed_book_body.to_string()
      })
      .await?;

    info!("Constructing toggle book monitoring body");

    match serde_json::from_str::<Value>(&response) {
      Ok(mut detailed_book_body) => {
        let monitored = detailed_book_body
          .get("monitored")
          .unwrap()
          .as_bool()
          .unwrap();

        *detailed_book_body.get_mut("monitored").unwrap() = json!(!monitored);

        debug!("Toggle book monitoring body: {detailed_book_body:?}");

        let request_props = self
          .request_props_from(
            event,
            RequestMethod::Put,
            Some(detailed_book_body),
            Some(format!("/{book_id}")),
            None,
          )
          .await;

        self
          .handle_request::<Value, ()>(request_props, |_, _| ())
          .await
      }
      Err(_) => {
        warn!("Request for detailed book body was interrupted");
        Ok(())
      }
    }
  }

  pub(in crate::network::readarr_network) async fn trigger_automatic_book_search(
    &mut self,
    book_id: i64,
  ) -> Result<Value> {
    let event = ReadarrEvent::TriggerAutomaticBookSearch(book_id);
    info!("Searching indexers for book with ID: {book_id}");
    let body = ReadarrCommandBody {
      name: "BookSearch".to_owned(),
      book_ids: Some(vec![book_id]),
      ..ReadarrCommandBody::default()
    };

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(body), None, None)
      .await;

    self
      .handle_request::<ReadarrCommandBody, Value>(request_props, |_, _| ())
      .await
  }
}
