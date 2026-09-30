use crate::app::App;
use crate::event::Key;
use crate::handlers::readarr_handlers::handle_change_tab_left_right_keys;
use crate::handlers::table_handler::{TableHandlingConfig, handle_table};
use crate::handlers::{KeyEventHandler, handle_clear_errors};
use crate::matches_key;
use crate::models::Route;
use crate::models::readarr_models::ReadarrHistoryItem;
use crate::models::servarr_data::readarr::readarr_data::{ActiveReadarrBlock, HISTORY_BLOCKS};
use crate::models::stateful_table::SortOption;

#[cfg(test)]
#[path = "history_handler_tests.rs"]
mod history_handler_tests;

pub(super) struct HistoryHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_readarr_block: ActiveReadarrBlock,
  _context: Option<ActiveReadarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveReadarrBlock> for HistoryHandler<'a, 'b> {
  fn handle(&mut self) {
    let history_table_handling_config =
      TableHandlingConfig::new(ActiveReadarrBlock::History.into())
        .sorting_block(ActiveReadarrBlock::HistorySortPrompt.into())
        .sort_options(history_sorting_options())
        .searching_block(ActiveReadarrBlock::SearchHistory.into())
        .search_error_block(ActiveReadarrBlock::SearchHistoryError.into())
        .search_field_fn(|history| &history.source_title.text)
        .filtering_block(ActiveReadarrBlock::FilterHistory.into())
        .filter_error_block(ActiveReadarrBlock::FilterHistoryError.into())
        .filter_field_fn(|history| &history.source_title.text);

    if !handle_table(
      self,
      |app| &mut app.data.readarr_data.history,
      history_table_handling_config,
    ) {
      self.handle_key_event();
    }
  }

  fn accepts(active_block: ActiveReadarrBlock) -> bool {
    HISTORY_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveReadarrBlock,
    context: Option<ActiveReadarrBlock>,
  ) -> Self {
    HistoryHandler {
      key,
      app,
      active_readarr_block: active_block,
      _context: context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    !self.app.is_loading && !self.app.data.readarr_data.history.is_empty()
  }

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    if self.active_readarr_block == ActiveReadarrBlock::History {
      handle_change_tab_left_right_keys(self.app, self.key)
    }
  }

  fn handle_submit(&mut self) {
    if self.active_readarr_block == ActiveReadarrBlock::History {
      self
        .app
        .push_navigation_stack(ActiveReadarrBlock::HistoryItemDetails.into());
    }
  }

  fn handle_esc(&mut self) {
    if self.active_readarr_block == ActiveReadarrBlock::HistoryItemDetails {
      self.app.pop_navigation_stack();
    } else {
      handle_clear_errors(self.app);
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    if self.active_readarr_block == ActiveReadarrBlock::History {
      match self.key {
        _ if matches_key!(refresh, key) => {
          self.app.should_refresh = true;
        }
        _ => (),
      }
    }
  }

  fn app_mut(&mut self) -> &mut App<'b> {
    self.app
  }

  fn current_route(&self) -> Route {
    self.app.get_current_route()
  }
}

pub(in crate::handlers::readarr_handlers) fn history_sorting_options()
-> Vec<SortOption<ReadarrHistoryItem>> {
  vec![
    SortOption {
      name: "Source Title",
      cmp_fn: Some(|a, b| {
        a.source_title
          .text
          .to_lowercase()
          .cmp(&b.source_title.text.to_lowercase())
      }),
    },
    SortOption {
      name: "Event Type",
      cmp_fn: Some(|a, b| {
        a.event_type
          .to_string()
          .to_lowercase()
          .cmp(&b.event_type.to_string().to_lowercase())
      }),
    },
    SortOption {
      name: "Quality",
      cmp_fn: Some(|a, b| {
        a.quality
          .quality
          .name
          .to_lowercase()
          .cmp(&b.quality.quality.name.to_lowercase())
      }),
    },
    SortOption {
      name: "Date",
      cmp_fn: Some(|a, b| a.date.cmp(&b.date)),
    },
  ]
}
