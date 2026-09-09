use crate::{
  app::App,
  event::Key,
  handlers::{KeyEventHandler, handle_clear_errors, handle_prompt_toggle},
  matches_key,
  models::{
    BlockSelectionState, HorizontallyScrollableText,
    readarr_models::Author,
    servarr_data::readarr::readarr_data::{
      ActiveReadarrBlock, DELETE_AUTHOR_SELECTION_BLOCKS, EDIT_AUTHOR_SELECTION_BLOCKS,
      LIBRARY_BLOCKS,
    },
    stateful_table::SortOption,
  },
  network::readarr_network::ReadarrEvent,
};

use super::handle_change_tab_left_right_keys;
use crate::handlers::table_handler::{TableHandlingConfig, handle_table};
use crate::models::Route;

mod add_author_handler;
mod author_details_handler;
mod author_overview_handler;
mod book_details_handler;
mod delete_author_handler;
mod delete_book_handler;
mod edit_author_handler;
mod edition_details_handler;

pub(in crate::handlers::readarr_handlers) use {
  add_author_handler::AddAuthorHandler, author_details_handler::AuthorDetailsHandler,
  book_details_handler::BookDetailsHandler, delete_author_handler::DeleteAuthorHandler,
  delete_book_handler::DeleteBookHandler, edit_author_handler::EditAuthorHandler,
};

#[cfg(test)]
#[path = "library_handler_tests.rs"]
mod library_handler_tests;

pub(super) struct LibraryHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_readarr_block: ActiveReadarrBlock,
  context: Option<ActiveReadarrBlock>,
}

impl LibraryHandler<'_, '_> {
  fn extract_author_id(&self) -> i64 {
    self.app.data.readarr_data.authors.current_selection().id
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveReadarrBlock> for LibraryHandler<'a, 'b> {
  fn handle(&mut self) {
    let authors_table_handling_config =
      TableHandlingConfig::new(ActiveReadarrBlock::Authors.into())
        .sorting_block(ActiveReadarrBlock::AuthorsSortPrompt.into())
        .sort_options(authors_sorting_options())
        .searching_block(ActiveReadarrBlock::SearchAuthors.into())
        .search_error_block(ActiveReadarrBlock::SearchAuthorsError.into())
        .search_field_fn(|author| &author.author_name.text)
        .filtering_block(ActiveReadarrBlock::FilterAuthors.into())
        .filter_error_block(ActiveReadarrBlock::FilterAuthorsError.into())
        .filter_field_fn(|author| &author.author_name.text);

    if !handle_table(
      self,
      |app| &mut app.data.readarr_data.authors,
      authors_table_handling_config,
    ) {
      match self.active_readarr_block {
        _ if AddAuthorHandler::accepts(self.active_readarr_block) => {
          AddAuthorHandler::new(self.key, self.app, self.active_readarr_block, self.context)
            .handle();
        }
        _ if DeleteAuthorHandler::accepts(self.active_readarr_block) => {
          DeleteAuthorHandler::new(self.key, self.app, self.active_readarr_block, self.context)
            .handle();
        }
        _ if EditAuthorHandler::accepts(self.active_readarr_block) => {
          EditAuthorHandler::new(self.key, self.app, self.active_readarr_block, self.context)
            .handle();
        }
        _ if AuthorDetailsHandler::accepts(self.active_readarr_block) => {
          AuthorDetailsHandler::new(self.key, self.app, self.active_readarr_block, self.context)
            .handle();
        }
        _ if DeleteBookHandler::accepts(self.active_readarr_block) => {
          DeleteBookHandler::new(self.key, self.app, self.active_readarr_block, self.context)
            .handle();
        }
        _ if BookDetailsHandler::accepts(self.active_readarr_block) => {
          BookDetailsHandler::new(self.key, self.app, self.active_readarr_block, self.context)
            .handle();
        }
        _ => self.handle_key_event(),
      }
    }
  }

  fn accepts(active_block: ActiveReadarrBlock) -> bool {
    AddAuthorHandler::accepts(active_block)
      || DeleteAuthorHandler::accepts(active_block)
      || DeleteBookHandler::accepts(active_block)
      || EditAuthorHandler::accepts(active_block)
      || AuthorDetailsHandler::accepts(active_block)
      || BookDetailsHandler::accepts(active_block)
      || LIBRARY_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveReadarrBlock,
    context: Option<ActiveReadarrBlock>,
  ) -> LibraryHandler<'a, 'b> {
    LibraryHandler {
      key,
      app,
      active_readarr_block: active_block,
      context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    !self.app.is_loading && !self.app.data.readarr_data.authors.is_empty()
  }

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {
    if self.active_readarr_block == ActiveReadarrBlock::Authors {
      self
        .app
        .push_navigation_stack(ActiveReadarrBlock::DeleteAuthorPrompt.into());
      self.app.data.readarr_data.selected_block =
        BlockSelectionState::new(DELETE_AUTHOR_SELECTION_BLOCKS);
    }
  }

  fn handle_left_right_action(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::Authors => handle_change_tab_left_right_keys(self.app, self.key),
      ActiveReadarrBlock::UpdateAllAuthorsPrompt => handle_prompt_toggle(self.app, self.key),
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::Authors => {
        self
          .app
          .push_navigation_stack(ActiveReadarrBlock::AuthorDetails.into());
      }
      ActiveReadarrBlock::UpdateAllAuthorsPrompt => {
        if self.app.data.readarr_data.prompt_confirm {
          self.app.data.readarr_data.prompt_confirm_action = Some(ReadarrEvent::UpdateAllAuthors);
        }

        self.app.pop_navigation_stack();
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::UpdateAllAuthorsPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.readarr_data.prompt_confirm = false;
      }
      _ => {
        handle_clear_errors(self.app);
      }
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_readarr_block {
      ActiveReadarrBlock::Authors => match key {
        _ if matches_key!(add, key) => {
          self
            .app
            .push_navigation_stack(ActiveReadarrBlock::AddAuthorSearchInput.into());
          self.app.data.readarr_data.add_author_search =
            Some(HorizontallyScrollableText::default());
          self.app.ignore_special_keys_for_textbox_input = true;
        }
        _ if matches_key!(toggle_monitoring, key) => {
          self.app.data.readarr_data.prompt_confirm = true;
          self.app.data.readarr_data.prompt_confirm_action = Some(
            ReadarrEvent::ToggleAuthorMonitoring(self.extract_author_id()),
          );

          self
            .app
            .pop_and_push_navigation_stack(self.active_readarr_block.into());
        }
        _ if matches_key!(edit, key) => {
          self.app.data.readarr_data.edit_author_modal = Some((&self.app.data.readarr_data).into());
          self
            .app
            .push_navigation_stack(ActiveReadarrBlock::EditAuthorPrompt.into());
          self.app.data.readarr_data.selected_block =
            BlockSelectionState::new(EDIT_AUTHOR_SELECTION_BLOCKS);
        }
        _ if matches_key!(update, key) => {
          self
            .app
            .push_navigation_stack(ActiveReadarrBlock::UpdateAllAuthorsPrompt.into());
        }
        _ if matches_key!(refresh, key) => {
          self.app.should_refresh = true;
        }
        _ => (),
      },
      ActiveReadarrBlock::UpdateAllAuthorsPrompt => {
        if matches_key!(confirm, key) {
          self.app.data.readarr_data.prompt_confirm = true;
          self.app.data.readarr_data.prompt_confirm_action = Some(ReadarrEvent::UpdateAllAuthors);

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

fn authors_sorting_options() -> Vec<SortOption<Author>> {
  vec![
    SortOption {
      name: "Name",
      cmp_fn: Some(|a, b| {
        a.author_name
          .text
          .to_lowercase()
          .cmp(&b.author_name.text.to_lowercase())
      }),
    },
    SortOption {
      name: "Status",
      cmp_fn: Some(|a, b| {
        a.status
          .to_string()
          .to_lowercase()
          .cmp(&b.status.to_string().to_lowercase())
      }),
    },
    SortOption {
      name: "Quality Profile",
      cmp_fn: Some(|a, b| a.quality_profile_id.cmp(&b.quality_profile_id)),
    },
    SortOption {
      name: "Metadata Profile",
      cmp_fn: Some(|a, b| a.metadata_profile_id.cmp(&b.metadata_profile_id)),
    },
    SortOption {
      name: "Books",
      cmp_fn: Some(|a, b| {
        a.statistics
          .as_ref()
          .map_or(0, |stats| stats.book_count)
          .cmp(&b.statistics.as_ref().map_or(0, |stats| stats.book_count))
      }),
    },
    SortOption {
      name: "Size",
      cmp_fn: Some(|a, b| {
        a.statistics
          .as_ref()
          .map_or(0, |stats| stats.size_on_disk)
          .cmp(&b.statistics.as_ref().map_or(0, |stats| stats.size_on_disk))
      }),
    },
    SortOption {
      name: "Monitored",
      cmp_fn: Some(|a, b| a.monitored.cmp(&b.monitored)),
    },
    SortOption {
      name: "Tags",
      cmp_fn: Some(|a, b| {
        let a_str = a
          .tags
          .iter()
          .map(|tag| tag.as_i64().unwrap().to_string())
          .collect::<Vec<String>>()
          .join(",");
        let b_str = b
          .tags
          .iter()
          .map(|tag| tag.as_i64().unwrap().to_string())
          .collect::<Vec<String>>()
          .join(",");

        a_str.cmp(&b_str)
      }),
    },
  ]
}
