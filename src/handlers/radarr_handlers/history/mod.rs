use crate::app::App;
use crate::event::Key;
use crate::handlers::radarr_handlers::handle_change_tab_left_right_keys;
use crate::handlers::table_handler::{TableHandlingConfig, handle_table};
use crate::handlers::{KeyEventHandler, handle_clear_errors};
use crate::matches_key;
use crate::models::Route;
use crate::models::radarr_models::RadarrHistoryItem;
use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, HISTORY_BLOCKS};
use crate::models::servarr_models::Language;
use crate::models::stateful_table::SortOption;

#[cfg(test)]
#[path = "history_handler_tests.rs"]
mod history_handler_tests;

pub(super) struct HistoryHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_radarr_block: ActiveRadarrBlock,
  _context: Option<ActiveRadarrBlock>,
}

impl HistoryHandler<'_, '_> {}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for HistoryHandler<'a, 'b> {
  fn handle(&mut self) {
    let history_table_handling_config = TableHandlingConfig::new(ActiveRadarrBlock::History.into())
      .sorting_block(ActiveRadarrBlock::HistorySortPrompt.into())
      .sort_options(history_sorting_options())
      .searching_block(ActiveRadarrBlock::SearchHistory.into())
      .search_error_block(ActiveRadarrBlock::SearchHistoryError.into())
      .search_field_fn(|history| &history.source_title.text)
      .filtering_block(ActiveRadarrBlock::FilterHistory.into())
      .filter_error_block(ActiveRadarrBlock::FilterHistoryError.into())
      .filter_field_fn(|history| &history.source_title.text);

    if !handle_table(
      self,
      |app| &mut app.data.radarr_data.history,
      history_table_handling_config,
    ) {
      self.handle_key_event();
    }
  }

  fn accepts(active_block: ActiveRadarrBlock) -> bool {
    HISTORY_BLOCKS.contains(&active_block)
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveRadarrBlock,
    context: Option<ActiveRadarrBlock>,
  ) -> Self {
    HistoryHandler {
      key,
      app,
      active_radarr_block: active_block,
      _context: context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn is_ready(&self) -> bool {
    !self.app.is_loading && !self.app.data.radarr_data.history.is_empty()
  }

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    if self.active_radarr_block == ActiveRadarrBlock::History {
      handle_change_tab_left_right_keys(self.app, self.key)
    }
  }

  fn handle_submit(&mut self) {
    if self.active_radarr_block == ActiveRadarrBlock::History {
      self
        .app
        .push_navigation_stack(ActiveRadarrBlock::HistoryItemDetails.into());
    }
  }

  fn handle_esc(&mut self) {
    if self.active_radarr_block == ActiveRadarrBlock::HistoryItemDetails {
      self.app.pop_navigation_stack();
    } else {
      handle_clear_errors(self.app);
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    if self.active_radarr_block == ActiveRadarrBlock::History {
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

pub(in crate::handlers::radarr_handlers) fn history_sorting_options()
-> Vec<SortOption<RadarrHistoryItem>> {
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
      name: "Language",
      cmp_fn: Some(|a, b| {
        let default_language = Language {
          id: 1,
          name: "_".to_owned(),
        };
        let language_a = a.languages.first().unwrap_or(&default_language);
        let language_b = b.languages.first().unwrap_or(&default_language);

        language_a.cmp(language_b)
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
