use anyhow::Result;
use log::info;
use serde_json::Value;

use crate::models::Route;
use crate::models::readarr_models::{AddAuthorBody, AddAuthorSearchResult, Author};
use crate::models::servarr_data::readarr::readarr_data::ActiveReadarrBlock;
use crate::models::stateful_table::StatefulTable;
use crate::network::readarr_network::ReadarrEvent;
use crate::network::{Network, RequestMethod};
use urlencoding::encode;

#[cfg(test)]
#[path = "readarr_authors_network_tests.rs"]
mod readarr_authors_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::readarr_network) async fn add_author(
    &mut self,
    mut add_author_body: AddAuthorBody,
  ) -> Result<Value> {
    info!("Adding Readarr author: {}", add_author_body.author_name);
    if let Some(tag_input_str) = add_author_body.tag_input_string.as_ref() {
      let tag_ids_vec = self
        .extract_and_add_readarr_tag_ids_vec(tag_input_str)
        .await;
      add_author_body.tags = tag_ids_vec;
    }
    let event = ReadarrEvent::AddAuthor(AddAuthorBody::default());

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Post,
        Some(add_author_body),
        None,
        None,
      )
      .await;

    self
      .handle_request::<AddAuthorBody, Value>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::readarr_network) async fn get_author_details(
    &mut self,
    author_id: i64,
  ) -> Result<Author> {
    info!("Fetching details for Readarr author with ID: {author_id}");
    let event = ReadarrEvent::GetAuthorDetails(author_id);

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{author_id}")),
        None,
      )
      .await;

    self
      .handle_request::<(), Author>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::readarr_network) async fn list_authors(&mut self) -> Result<Vec<Author>> {
    info!("Fetching Readarr authors");
    let event = ReadarrEvent::ListAuthors;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<Author>>(request_props, |mut authors_vec, mut app| {
        if !matches!(
          app.get_current_route(),
          Route::Readarr(ActiveReadarrBlock::AuthorsSortPrompt, _)
        ) {
          authors_vec.sort_by_key(|a| a.id);
          app.data.readarr_data.authors.set_items(authors_vec);
          app.data.readarr_data.authors.apply_sorting_toggle(false);
        }
      })
      .await
  }

  pub(in crate::network::readarr_network) async fn search_author(
    &mut self,
    query: String,
  ) -> Result<Vec<AddAuthorSearchResult>> {
    info!("Searching for author: {query}");
    let event = ReadarrEvent::SearchNewAuthor(String::new());

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("term={}", encode(&query))),
      )
      .await;

    let result = self
      .handle_request::<(), Vec<AddAuthorSearchResult>>(request_props, |author_vec, mut app| {
        if author_vec.is_empty() {
          app.pop_and_push_navigation_stack(ActiveReadarrBlock::AddAuthorEmptySearchResults.into());
        } else if let Some(add_searched_authors) =
          app.data.readarr_data.add_searched_authors.as_mut()
        {
          add_searched_authors.set_items(author_vec);
        } else {
          let mut add_searched_authors = StatefulTable::default();
          add_searched_authors.set_items(author_vec);
          app.data.readarr_data.add_searched_authors = Some(add_searched_authors);
        }
      })
      .await;

    if result.is_err() {
      self.app.lock().await.data.readarr_data.add_searched_authors = Some(StatefulTable::default());
    }

    result
  }
}
