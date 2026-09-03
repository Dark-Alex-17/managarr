use crate::app::App;
use crate::event::Key;
use crate::handlers::readarr_handlers::history::history_sorting_options;
use crate::handlers::table_handler::{TableHandlingConfig, handle_table};
use crate::handlers::{KeyEventHandler, handle_prompt_toggle};
use crate::matches_key;
use crate::models::readarr_models::{Edition, Ratings, ReadarrHistoryItem, ReadarrRelease};
use crate::models::servarr_data::readarr::modals::EditionDetailsModal;
use crate::models::servarr_data::readarr::readarr_data::{ActiveReadarrBlock, BOOK_DETAILS_BLOCKS};
use crate::models::servarr_models::ReleaseDownloadBody;
use crate::models::{Route, ScrollableText};
use crate::network::readarr_network::ReadarrEvent;
use indoc::formatdoc;

use super::author_details_handler::releases_sorting_options;
use super::edition_details_handler::EditionDetailsHandler;

#[cfg(test)]
#[path = "book_details_handler_tests.rs"]
mod book_details_handler_tests;

pub(in crate::handlers::readarr_handlers) struct BookDetailsHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_readarr_block: ActiveReadarrBlock,
  _context: Option<ActiveReadarrBlock>,
}

impl BookDetailsHandler<'_, '_> {
  fn extract_book_id(&self) -> i64 {
    self.app.data.readarr_data.books.current_selection().id
  }

  fn extract_book_file_id(&self) -> i64 {
    self
      .app
      .data
      .readarr_data
      .book_details_modal
      .as_ref()
      .expect("Book details modal is undefined")
      .book_files
      .current_selection()
      .id
  }

  fn book_files_are_populated(&self) -> bool {
    self
      .app
      .data
      .readarr_data
      .book_details_modal
      .as_ref()
      .is_some_and(|modal| !modal.book_files.is_empty())
  }

  fn editions_are_populated(&self) -> bool {
    self
      .app
      .data
      .readarr_data
      .book_details_modal
      .as_ref()
      .is_some_and(|modal| !modal.editions.is_empty())
  }

  fn build_edition_details_modal(&mut self) {
    let Edition {
      title,
      overview,
      format,
      publisher,
      page_count,
      release_date,
      isbn13,
      asin,
      ratings,
      language,
      monitored,
      ..
    } = self
      .app
      .data
      .readarr_data
      .book_details_modal
      .as_ref()
      .expect("Book details modal is undefined")
      .editions
      .current_selection()
      .clone();
    let format = format.unwrap_or_default();
    let language = language.unwrap_or_default();
    let publisher = publisher.unwrap_or_default();
    let page_count = page_count
      .map(|count| count.to_string())
      .unwrap_or_default();
    let release_date = release_date
      .map(|date| date.to_string())
      .unwrap_or_default();
    let isbn13 = isbn13.unwrap_or_default();
    let asin = asin.unwrap_or_default();
    let ratings = ratings
      .map(|Ratings { votes, value, .. }| format!("{value} ({votes} votes)"))
      .unwrap_or_default();
    let overview = overview.unwrap_or_default();
    let details = formatdoc!(
      "
        Title: {title}
        Format: {format}
        Language: {language}
        Publisher: {publisher}
        Page Count: {page_count}
        Release Date: {release_date}
        ISBN13: {isbn13}
        ASIN: {asin}
        Ratings: {ratings}
        Monitored: {monitored}
        Overview: {overview}
      "
    );

    self
      .app
      .data
      .readarr_data
      .book_details_modal
      .as_mut()
      .expect("Book details modal is undefined")
      .edition_details_modal = Some(EditionDetailsModal {
      edition_details: ScrollableText::with_string(details),
    });
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveReadarrBlock> for BookDetailsHandler<'a, 'b> {
  fn handle(&mut self) {
    if EditionDetailsHandler::accepts(self.active_readarr_block) {
      return EditionDetailsHandler::new(
        self.key,
        self.app,
        self.active_readarr_block,
        self._context,
      )
      .handle();
    }

    let editions_table_handling_config =
      TableHandlingConfig::new(ActiveReadarrBlock::BookDetails.into())
        .searching_block(ActiveReadarrBlock::SearchEditions.into())
        .search_error_block(ActiveReadarrBlock::SearchEditionsError.into())
        .search_field_fn(|edition: &Edition| &edition.title);

    let book_history_table_handling_config =
      TableHandlingConfig::new(ActiveReadarrBlock::BookHistory.into())
        .sorting_block(ActiveReadarrBlock::BookHistorySortPrompt.into())
        .sort_options(history_sorting_options())
        .searching_block(ActiveReadarrBlock::SearchBookHistory.into())
        .search_error_block(ActiveReadarrBlock::SearchBookHistoryError.into())
        .search_field_fn(|history_item: &ReadarrHistoryItem| &history_item.source_title.text)
        .filtering_block(ActiveReadarrBlock::FilterBookHistory.into())
        .filter_error_block(ActiveReadarrBlock::FilterBookHistoryError.into())
        .filter_field_fn(|history_item: &ReadarrHistoryItem| &history_item.source_title.text);

    let book_releases_table_handling_config =
      TableHandlingConfig::new(ActiveReadarrBlock::ManualBookSearch.into())
        .sorting_block(ActiveReadarrBlock::ManualBookSearchSortPrompt.into())
        .sort_options(releases_sorting_options());

    if !handle_table(
      self,
      |app| {
        &mut app
          .data
          .readarr_data
          .book_details_modal
          .as_mut()
          .expect("Book details modal is undefined")
          .editions
      },
      editions_table_handling_config,
    ) && !handle_table(
      self,
      |app| {
        &mut app
          .data
          .readarr_data
          .book_details_modal
          .as_mut()
          .expect("Book details modal is undefined")
          .book_history
      },
      book_history_table_handling_config,
    ) && !handle_table(
      self,
      |app| {
        &mut app
          .data
          .readarr_data
          .book_details_modal
          .as_mut()
          .expect("Book details modal is undefined")
          .book_releases
      },
      book_releases_table_handling_config,
    ) {
      self.handle_key_event();
    }
  }

  fn accepts(active_block: ActiveReadarrBlock) -> bool {
    EditionDetailsHandler::accepts(active_block) || BOOK_DETAILS_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveReadarrBlock,
    _context: Option<ActiveReadarrBlock>,
  ) -> BookDetailsHandler<'a, 'b> {
    BookDetailsHandler {
      key,
      app,
      active_readarr_block: active_block,
      _context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    if self.app.is_loading {
      return false;
    }

    let Some(book_details_modal) = &self.app.data.readarr_data.book_details_modal else {
      return false;
    };

    match self.active_readarr_block {
      ActiveReadarrBlock::BookDetails => !book_details_modal.editions.is_empty(),
      ActiveReadarrBlock::BookHistory => !book_details_modal.book_history.is_empty(),
      ActiveReadarrBlock::BookFileInfo => !book_details_modal.book_files.is_empty(),
      ActiveReadarrBlock::ManualBookSearch => !book_details_modal.book_releases.is_empty(),
      _ => true,
    }
  }

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {
    if self.active_readarr_block == ActiveReadarrBlock::BookDetails
      && self.book_files_are_populated()
    {
      self
        .app
        .push_navigation_stack(ActiveReadarrBlock::DeleteBookFilePrompt.into());
    }
  }

  fn handle_left_right_action(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::BookDetails
      | ActiveReadarrBlock::BookHistory
      | ActiveReadarrBlock::BookFileInfo
      | ActiveReadarrBlock::ManualBookSearch => match self.key {
        _ if matches_key!(left, self.key) => {
          self
            .app
            .data
            .readarr_data
            .book_details_modal
            .as_mut()
            .expect("Book details modal is undefined")
            .book_details_tabs
            .previous();
          self.app.pop_and_push_navigation_stack(
            self
              .app
              .data
              .readarr_data
              .book_details_modal
              .as_ref()
              .expect("Book details modal is undefined")
              .book_details_tabs
              .get_active_route(),
          );
        }
        _ if matches_key!(right, self.key) => {
          self
            .app
            .data
            .readarr_data
            .book_details_modal
            .as_mut()
            .expect("Book details modal is undefined")
            .book_details_tabs
            .next();
          self.app.pop_and_push_navigation_stack(
            self
              .app
              .data
              .readarr_data
              .book_details_modal
              .as_ref()
              .expect("Book details modal is undefined")
              .book_details_tabs
              .get_active_route(),
          );
        }
        _ => (),
      },
      ActiveReadarrBlock::AutomaticallySearchBookPrompt
      | ActiveReadarrBlock::ManualBookSearchConfirmPrompt
      | ActiveReadarrBlock::DeleteBookFilePrompt => {
        handle_prompt_toggle(self.app, self.key);
      }
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::BookDetails if self.editions_are_populated() => {
        self.build_edition_details_modal();
        self
          .app
          .push_navigation_stack(ActiveReadarrBlock::BookEditionDetails.into());
      }
      ActiveReadarrBlock::BookHistory => self
        .app
        .push_navigation_stack(ActiveReadarrBlock::BookHistoryDetails.into()),
      ActiveReadarrBlock::ManualBookSearch => {
        self
          .app
          .push_navigation_stack(ActiveReadarrBlock::ManualBookSearchConfirmPrompt.into());
      }
      ActiveReadarrBlock::ManualBookSearchConfirmPrompt => {
        if self.app.data.readarr_data.prompt_confirm {
          let ReadarrRelease {
            guid, indexer_id, ..
          } = self
            .app
            .data
            .readarr_data
            .book_details_modal
            .as_ref()
            .expect("Book details modal is undefined")
            .book_releases
            .current_selection()
            .clone();
          let params = ReleaseDownloadBody { guid, indexer_id };
          self.app.data.readarr_data.prompt_confirm_action =
            Some(ReadarrEvent::DownloadRelease(params));
        }

        self.app.pop_navigation_stack();
      }
      ActiveReadarrBlock::AutomaticallySearchBookPrompt => {
        if self.app.data.readarr_data.prompt_confirm {
          self.app.data.readarr_data.prompt_confirm_action = Some(
            ReadarrEvent::TriggerAutomaticBookSearch(self.extract_book_id()),
          );
        }

        self.app.pop_navigation_stack();
      }
      ActiveReadarrBlock::DeleteBookFilePrompt => {
        if self.app.data.readarr_data.prompt_confirm {
          self.app.data.readarr_data.prompt_confirm_action =
            Some(ReadarrEvent::DeleteBookFile(self.extract_book_file_id()));
        }

        self.app.pop_navigation_stack();
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::BookDetails
      | ActiveReadarrBlock::BookFileInfo
      | ActiveReadarrBlock::ManualBookSearch => {
        self.app.pop_navigation_stack();
        self.app.data.readarr_data.book_details_modal = None;
      }
      ActiveReadarrBlock::BookHistoryDetails => {
        self.app.pop_navigation_stack();
      }
      ActiveReadarrBlock::BookHistory => {
        if self
          .app
          .data
          .readarr_data
          .book_details_modal
          .as_ref()
          .expect("Book details modal is undefined")
          .book_history
          .filtered_items
          .is_some()
        {
          self
            .app
            .data
            .readarr_data
            .book_details_modal
            .as_mut()
            .expect("Book details modal is undefined")
            .book_history
            .reset_filter();
        } else {
          self.app.pop_navigation_stack();
          self.app.data.readarr_data.book_details_modal = None;
        }
      }
      ActiveReadarrBlock::AutomaticallySearchBookPrompt
      | ActiveReadarrBlock::ManualBookSearchConfirmPrompt
      | ActiveReadarrBlock::DeleteBookFilePrompt => {
        self.app.pop_navigation_stack();
        self.app.data.readarr_data.prompt_confirm = false;
      }
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_readarr_block {
      ActiveReadarrBlock::BookDetails
      | ActiveReadarrBlock::BookHistory
      | ActiveReadarrBlock::BookFileInfo
      | ActiveReadarrBlock::ManualBookSearch => match key {
        _ if matches_key!(refresh, key) => {
          self
            .app
            .pop_and_push_navigation_stack(self.active_readarr_block.into());
        }
        _ if matches_key!(auto_search, key) => {
          self
            .app
            .push_navigation_stack(ActiveReadarrBlock::AutomaticallySearchBookPrompt.into());
        }
        _ => (),
      },
      ActiveReadarrBlock::AutomaticallySearchBookPrompt if matches_key!(confirm, key) => {
        self.app.data.readarr_data.prompt_confirm = true;
        self.app.data.readarr_data.prompt_confirm_action = Some(
          ReadarrEvent::TriggerAutomaticBookSearch(self.extract_book_id()),
        );

        self.app.pop_navigation_stack();
      }
      ActiveReadarrBlock::DeleteBookFilePrompt if matches_key!(confirm, key) => {
        self.app.data.readarr_data.prompt_confirm = true;
        self.app.data.readarr_data.prompt_confirm_action =
          Some(ReadarrEvent::DeleteBookFile(self.extract_book_file_id()));

        self.app.pop_navigation_stack();
      }
      ActiveReadarrBlock::ManualBookSearchConfirmPrompt if matches_key!(confirm, key) => {
        self.app.data.readarr_data.prompt_confirm = true;
        let ReadarrRelease {
          guid, indexer_id, ..
        } = self
          .app
          .data
          .readarr_data
          .book_details_modal
          .as_ref()
          .expect("Book details modal is undefined")
          .book_releases
          .current_selection()
          .clone();
        let params = ReleaseDownloadBody { guid, indexer_id };
        self.app.data.readarr_data.prompt_confirm_action =
          Some(ReadarrEvent::DownloadRelease(params));

        self.app.pop_navigation_stack();
      }
      _ => (),
    }
  }

  fn app_mut(&mut self) -> &mut App<'b> {
    self.app
  }

  fn current_route(&self) -> Route {
    self.app.get_current_route()
  }
}
