use super::author_overview_handler::AuthorOverviewHandler;
use crate::app::App;
use crate::event::Key;
use crate::handlers::readarr_handlers::history::history_sorting_options;
use crate::handlers::table_handler::{TableHandlingConfig, handle_table};
use crate::handlers::{KeyEventHandler, handle_prompt_toggle};
use crate::matches_key;
use crate::models::readarr_models::{Book, ReadarrHistoryItem, ReadarrRelease};
use crate::models::servarr_data::readarr::modals::AuthorOverviewModal;
use crate::models::servarr_data::readarr::readarr_data::{
  AUTHOR_DETAILS_BLOCKS, ActiveReadarrBlock, DELETE_BOOK_SELECTION_BLOCKS,
  EDIT_AUTHOR_SELECTION_BLOCKS,
};
use crate::models::servarr_models::ReleaseDownloadBody;
use crate::models::stateful_table::SortOption;
use crate::models::{BlockSelectionState, Route, ScrollableText};
use crate::network::readarr_network::ReadarrEvent;
use serde_json::Number;

#[cfg(test)]
#[path = "author_details_handler_tests.rs"]
mod author_details_handler_tests;

pub(in crate::handlers::readarr_handlers) struct AuthorDetailsHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_readarr_block: ActiveReadarrBlock,
  _context: Option<ActiveReadarrBlock>,
}

impl AuthorDetailsHandler<'_, '_> {
  fn extract_author_id(&self) -> i64 {
    self.app.data.readarr_data.authors.current_selection().id
  }

  fn extract_book_id(&self) -> i64 {
    self.app.data.readarr_data.books.current_selection().id
  }

  fn build_author_overview_modal(&mut self) {
    let overview = self
      .app
      .data
      .readarr_data
      .authors
      .current_selection()
      .overview
      .clone()
      .unwrap_or_default();

    self.app.data.readarr_data.author_overview_modal = Some(AuthorOverviewModal {
      overview: ScrollableText::with_string(overview),
    });
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveReadarrBlock> for AuthorDetailsHandler<'a, 'b> {
  fn handle(&mut self) {
    if AuthorOverviewHandler::accepts(self.active_readarr_block) {
      return AuthorOverviewHandler::new(
        self.key,
        self.app,
        self.active_readarr_block,
        self._context,
      )
      .handle();
    }

    let books_table_handling_config =
      TableHandlingConfig::new(ActiveReadarrBlock::AuthorDetails.into())
        .searching_block(ActiveReadarrBlock::SearchBooks.into())
        .search_error_block(ActiveReadarrBlock::SearchBooksError.into())
        .search_field_fn(|book: &Book| &book.title.text);

    let author_history_table_handling_config =
      TableHandlingConfig::new(ActiveReadarrBlock::AuthorHistory.into())
        .sorting_block(ActiveReadarrBlock::AuthorHistorySortPrompt.into())
        .sort_options(history_sorting_options())
        .searching_block(ActiveReadarrBlock::SearchAuthorHistory.into())
        .search_error_block(ActiveReadarrBlock::SearchAuthorHistoryError.into())
        .search_field_fn(|history_item: &ReadarrHistoryItem| &history_item.source_title.text)
        .filtering_block(ActiveReadarrBlock::FilterAuthorHistory.into())
        .filter_error_block(ActiveReadarrBlock::FilterAuthorHistoryError.into())
        .filter_field_fn(|history_item: &ReadarrHistoryItem| &history_item.source_title.text);

    let author_releases_table_handling_config =
      TableHandlingConfig::new(ActiveReadarrBlock::ManualAuthorSearch.into())
        .sorting_block(ActiveReadarrBlock::ManualAuthorSearchSortPrompt.into())
        .sort_options(releases_sorting_options());

    if !handle_table(
      self,
      |app| &mut app.data.readarr_data.books,
      books_table_handling_config,
    ) && !handle_table(
      self,
      |app| &mut app.data.readarr_data.author_history,
      author_history_table_handling_config,
    ) && !handle_table(
      self,
      |app| &mut app.data.readarr_data.author_releases,
      author_releases_table_handling_config,
    ) {
      self.handle_key_event();
    }
  }

  fn accepts(active_block: ActiveReadarrBlock) -> bool {
    AuthorOverviewHandler::accepts(active_block) || AUTHOR_DETAILS_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveReadarrBlock,
    _context: Option<ActiveReadarrBlock>,
  ) -> AuthorDetailsHandler<'a, 'b> {
    AuthorDetailsHandler {
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

    match self.active_readarr_block {
      ActiveReadarrBlock::AuthorHistory => !self.app.data.readarr_data.author_history.is_empty(),
      ActiveReadarrBlock::ManualAuthorSearch => {
        !self.app.data.readarr_data.author_releases.is_empty()
      }
      _ => true,
    }
  }

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {
    if self.active_readarr_block == ActiveReadarrBlock::AuthorDetails {
      self
        .app
        .push_navigation_stack(ActiveReadarrBlock::DeleteBookPrompt.into());
      self.app.data.readarr_data.selected_block =
        BlockSelectionState::new(DELETE_BOOK_SELECTION_BLOCKS);
    }
  }

  fn handle_left_right_action(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::AuthorDetails
      | ActiveReadarrBlock::AuthorHistory
      | ActiveReadarrBlock::ManualAuthorSearch => match self.key {
        _ if matches_key!(left, self.key) => {
          self.app.data.readarr_data.author_info_tabs.previous();
          self.app.pop_and_push_navigation_stack(
            self
              .app
              .data
              .readarr_data
              .author_info_tabs
              .get_active_route(),
          );
        }
        _ if matches_key!(right, self.key) => {
          self.app.data.readarr_data.author_info_tabs.next();
          self.app.pop_and_push_navigation_stack(
            self
              .app
              .data
              .readarr_data
              .author_info_tabs
              .get_active_route(),
          );
        }
        _ => (),
      },
      ActiveReadarrBlock::UpdateAndScanAuthorPrompt
      | ActiveReadarrBlock::AutomaticallySearchAuthorPrompt
      | ActiveReadarrBlock::ManualAuthorSearchConfirmPrompt => {
        handle_prompt_toggle(self.app, self.key);
      }
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::AuthorDetails if !self.app.data.readarr_data.books.is_empty() => {
        self
          .app
          .push_navigation_stack(ActiveReadarrBlock::BookDetails.into());
      }
      ActiveReadarrBlock::AuthorHistory
        if !self.app.data.readarr_data.author_history.is_empty() =>
      {
        self
          .app
          .push_navigation_stack(ActiveReadarrBlock::AuthorHistoryDetails.into());
      }
      ActiveReadarrBlock::ManualAuthorSearch => {
        self
          .app
          .push_navigation_stack(ActiveReadarrBlock::ManualAuthorSearchConfirmPrompt.into());
      }
      ActiveReadarrBlock::ManualAuthorSearchConfirmPrompt => {
        if self.app.data.readarr_data.prompt_confirm {
          let ReadarrRelease {
            guid, indexer_id, ..
          } = self
            .app
            .data
            .readarr_data
            .author_releases
            .current_selection()
            .clone();
          let params = ReleaseDownloadBody { guid, indexer_id };
          self.app.data.readarr_data.prompt_confirm_action =
            Some(ReadarrEvent::DownloadRelease(params));
        }

        self.app.pop_navigation_stack();
      }
      ActiveReadarrBlock::AutomaticallySearchAuthorPrompt => {
        if self.app.data.readarr_data.prompt_confirm {
          self.app.data.readarr_data.prompt_confirm_action = Some(
            ReadarrEvent::TriggerAutomaticAuthorSearch(self.extract_author_id()),
          );
        }

        self.app.pop_navigation_stack();
      }
      ActiveReadarrBlock::UpdateAndScanAuthorPrompt => {
        if self.app.data.readarr_data.prompt_confirm {
          self.app.data.readarr_data.prompt_confirm_action =
            Some(ReadarrEvent::UpdateAndScanAuthor(self.extract_author_id()));
        }

        self.app.pop_navigation_stack();
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::UpdateAndScanAuthorPrompt
      | ActiveReadarrBlock::AutomaticallySearchAuthorPrompt
      | ActiveReadarrBlock::ManualAuthorSearchConfirmPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.readarr_data.prompt_confirm = false;
      }
      ActiveReadarrBlock::AuthorHistoryDetails => {
        self.app.pop_navigation_stack();
      }
      ActiveReadarrBlock::AuthorHistory => {
        if self
          .app
          .data
          .readarr_data
          .author_history
          .filtered_items
          .is_some()
        {
          self.app.data.readarr_data.author_history.reset_filter();
        } else {
          self.app.pop_navigation_stack();
          self.app.data.readarr_data.reset_author_info_tabs();
        }
      }
      ActiveReadarrBlock::AuthorDetails | ActiveReadarrBlock::ManualAuthorSearch => {
        self.app.pop_navigation_stack();
        self.app.data.readarr_data.reset_author_info_tabs();
      }
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_readarr_block {
      ActiveReadarrBlock::AuthorDetails => match self.key {
        _ if matches_key!(refresh, key) => self
          .app
          .pop_and_push_navigation_stack(self.active_readarr_block.into()),
        _ if matches_key!(auto_search, key) => {
          self
            .app
            .push_navigation_stack(ActiveReadarrBlock::AutomaticallySearchAuthorPrompt.into());
        }
        _ if matches_key!(update, key) => {
          self
            .app
            .push_navigation_stack(ActiveReadarrBlock::UpdateAndScanAuthorPrompt.into());
        }
        _ if matches_key!(edit, key) => {
          self.app.push_navigation_stack(
            (
              ActiveReadarrBlock::EditAuthorPrompt,
              Some(self.active_readarr_block),
            )
              .into(),
          );
          self.app.data.readarr_data.edit_author_modal = Some((&self.app.data.readarr_data).into());
          self.app.data.readarr_data.selected_block =
            BlockSelectionState::new(EDIT_AUTHOR_SELECTION_BLOCKS);
        }
        _ if matches_key!(view, key) => {
          self.build_author_overview_modal();
          self
            .app
            .push_navigation_stack(ActiveReadarrBlock::AuthorOverview.into());
        }
        _ if matches_key!(toggle_monitoring, key)
          && !self.app.data.readarr_data.books.is_empty() =>
        {
          self.app.data.readarr_data.prompt_confirm = true;
          self.app.data.readarr_data.prompt_confirm_action =
            Some(ReadarrEvent::ToggleBookMonitoring(self.extract_book_id()));

          self
            .app
            .pop_and_push_navigation_stack(self.active_readarr_block.into());
        }
        _ => (),
      },
      ActiveReadarrBlock::AuthorHistory | ActiveReadarrBlock::ManualAuthorSearch => {
        match self.key {
          _ if matches_key!(refresh, key) => self
            .app
            .pop_and_push_navigation_stack(self.active_readarr_block.into()),
          _ if matches_key!(auto_search, key) => {
            self
              .app
              .push_navigation_stack(ActiveReadarrBlock::AutomaticallySearchAuthorPrompt.into());
          }
          _ if matches_key!(edit, key) => {
            self.app.push_navigation_stack(
              (
                ActiveReadarrBlock::EditAuthorPrompt,
                Some(self.active_readarr_block),
              )
                .into(),
            );
            self.app.data.readarr_data.edit_author_modal =
              Some((&self.app.data.readarr_data).into());
            self.app.data.readarr_data.selected_block =
              BlockSelectionState::new(EDIT_AUTHOR_SELECTION_BLOCKS);
          }
          _ if matches_key!(update, key) => {
            self
              .app
              .push_navigation_stack(ActiveReadarrBlock::UpdateAndScanAuthorPrompt.into());
          }
          _ => (),
        }
      }
      ActiveReadarrBlock::AutomaticallySearchAuthorPrompt => {
        if matches_key!(confirm, key) {
          self.app.data.readarr_data.prompt_confirm = true;
          self.app.data.readarr_data.prompt_confirm_action = Some(
            ReadarrEvent::TriggerAutomaticAuthorSearch(self.extract_author_id()),
          );

          self.app.pop_navigation_stack();
        }
      }
      ActiveReadarrBlock::UpdateAndScanAuthorPrompt => {
        if matches_key!(confirm, key) {
          self.app.data.readarr_data.prompt_confirm = true;
          self.app.data.readarr_data.prompt_confirm_action =
            Some(ReadarrEvent::UpdateAndScanAuthor(self.extract_author_id()));

          self.app.pop_navigation_stack();
        }
      }
      ActiveReadarrBlock::ManualAuthorSearchConfirmPrompt => {
        if matches_key!(confirm, key) {
          self.app.data.readarr_data.prompt_confirm = true;
          let ReadarrRelease {
            guid, indexer_id, ..
          } = self
            .app
            .data
            .readarr_data
            .author_releases
            .current_selection()
            .clone();
          let params = ReleaseDownloadBody { guid, indexer_id };
          self.app.data.readarr_data.prompt_confirm_action =
            Some(ReadarrEvent::DownloadRelease(params));

          self.app.pop_navigation_stack();
        }
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

pub(in crate::handlers::readarr_handlers::library) fn releases_sorting_options()
-> Vec<SortOption<ReadarrRelease>> {
  vec![
    SortOption {
      name: "Source",
      cmp_fn: Some(|a, b| a.protocol.cmp(&b.protocol)),
    },
    SortOption {
      name: "Age",
      cmp_fn: Some(|a, b| a.age.cmp(&b.age)),
    },
    SortOption {
      name: "Rejected",
      cmp_fn: Some(|a, b| a.rejected.cmp(&b.rejected)),
    },
    SortOption {
      name: "Title",
      cmp_fn: Some(|a, b| {
        a.title
          .text
          .to_lowercase()
          .cmp(&b.title.text.to_lowercase())
      }),
    },
    SortOption {
      name: "Indexer",
      cmp_fn: Some(|a, b| a.indexer.to_lowercase().cmp(&b.indexer.to_lowercase())),
    },
    SortOption {
      name: "Size",
      cmp_fn: Some(|a, b| a.size.cmp(&b.size)),
    },
    SortOption {
      name: "Peers",
      cmp_fn: Some(|a, b| {
        let default_number = Number::from(i64::MAX);
        let seeder_a = a
          .seeders
          .as_ref()
          .unwrap_or(&default_number)
          .as_u64()
          .unwrap();
        let seeder_b = b
          .seeders
          .as_ref()
          .unwrap_or(&default_number)
          .as_u64()
          .unwrap();

        seeder_a.cmp(&seeder_b)
      }),
    },
    SortOption {
      name: "Quality",
      cmp_fn: Some(|a, b| a.quality.cmp(&b.quality)),
    },
  ]
}
