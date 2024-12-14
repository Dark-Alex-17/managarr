use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::App;
use crate::event::Key;
use crate::handle_table_events;
use crate::handlers::sonarr_handlers::handle_change_tab_left_right_keys;
use crate::handlers::table_handler::TableHandlingConfig;
use crate::handlers::{handle_clear_errors, KeyEventHandler};
use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, HISTORY_BLOCKS};
use crate::models::servarr_models::Language;
use crate::models::sonarr_models::SonarrHistoryItem;
use crate::models::stateful_table::SortOption;

#[cfg(test)]
#[path = "history_handler_tests.rs"]
mod history_handler_tests;

pub(super) struct HistoryHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_sonarr_block: ActiveSonarrBlock,
  _context: Option<ActiveSonarrBlock>,
}

impl<'a, 'b> HistoryHandler<'a, 'b> {
  handle_table_events!(
    self,
    history,
    self.app.data.sonarr_data.history,
    SonarrHistoryItem
  );
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveSonarrBlock> for HistoryHandler<'a, 'b> {
  fn handle(&mut self) {
    let history_table_handling_config = TableHandlingConfig::new(ActiveSonarrBlock::History.into())
      .sorting_block(ActiveSonarrBlock::HistorySortPrompt.into())
      .sort_by_fn(|a: &SonarrHistoryItem, b: &SonarrHistoryItem| a.id.cmp(&b.id))
      .sort_options(history_sorting_options())
      .searching_block(ActiveSonarrBlock::SearchHistory.into())
      .search_error_block(ActiveSonarrBlock::SearchHistoryError.into())
      .search_field_fn(|history| &history.source_title.text)
      .filtering_block(ActiveSonarrBlock::FilterHistory.into())
      .filter_error_block(ActiveSonarrBlock::FilterHistoryError.into())
      .filter_field_fn(|history| &history.source_title.text);

    if !self.handle_history_table_events(history_table_handling_config) {
      self.handle_key_event();
    }
  }

  fn accepts(active_block: ActiveSonarrBlock) -> bool {
    HISTORY_BLOCKS.contains(&active_block)
  }

  fn with(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveSonarrBlock,
    context: Option<ActiveSonarrBlock>,
  ) -> Self {
    HistoryHandler {
      key,
      app,
      active_sonarr_block: active_block,
      _context: context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    !self.app.is_loading && !self.app.data.sonarr_data.history.is_empty()
  }

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    if self.active_sonarr_block == ActiveSonarrBlock::History {
      handle_change_tab_left_right_keys(self.app, self.key)
    }
  }

  fn handle_submit(&mut self) {
    if self.active_sonarr_block == ActiveSonarrBlock::History {
      self
        .app
        .push_navigation_stack(ActiveSonarrBlock::HistoryItemDetails.into());
    }
  }

  fn handle_esc(&mut self) {
    if self.active_sonarr_block == ActiveSonarrBlock::HistoryItemDetails {
      self.app.pop_navigation_stack();
    } else {
      handle_clear_errors(self.app);
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    if self.active_sonarr_block == ActiveSonarrBlock::History {
      match self.key {
        _ if key == DEFAULT_KEYBINDINGS.refresh.key => {
          self.app.should_refresh = true;
        }
        _ => (),
      }
    }
  }
}

pub(in crate::handlers::sonarr_handlers) fn history_sorting_options(
) -> Vec<SortOption<SonarrHistoryItem>> {
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
        let language_a = &a.languages.first().unwrap_or(&default_language);
        let language_b = &b.languages.first().unwrap_or(&default_language);

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
