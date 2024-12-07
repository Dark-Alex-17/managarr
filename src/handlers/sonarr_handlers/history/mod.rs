use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::App;
use crate::event::Key;
use crate::handlers::sonarr_handlers::handle_change_tab_left_right_keys;
use crate::handlers::{handle_clear_errors, KeyEventHandler};
use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, HISTORY_BLOCKS};
use crate::models::sonarr_models::SonarrHistoryItem;
use crate::models::stateful_table::SortOption;
use crate::models::{HorizontallyScrollableText, Scrollable};
use crate::{handle_text_box_keys, handle_text_box_left_right_keys};

#[cfg(test)]
#[path = "history_handler_tests.rs"]
mod history_handler_tests;

pub(super) struct HistoryHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_sonarr_block: ActiveSonarrBlock,
  _context: Option<ActiveSonarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveSonarrBlock> for HistoryHandler<'a, 'b> {
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

  fn handle_scroll_up(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::History => self.app.data.sonarr_data.history.scroll_up(),
      ActiveSonarrBlock::HistorySortPrompt => self
        .app
        .data
        .sonarr_data
        .history
        .sort
        .as_mut()
        .unwrap()
        .scroll_up(),
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::History => self.app.data.sonarr_data.history.scroll_down(),
      ActiveSonarrBlock::HistorySortPrompt => self
        .app
        .data
        .sonarr_data
        .history
        .sort
        .as_mut()
        .unwrap()
        .scroll_down(),
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::History => self.app.data.sonarr_data.history.scroll_to_top(),
      ActiveSonarrBlock::SearchHistory => {
        self
          .app
          .data
          .sonarr_data
          .history
          .search
          .as_mut()
          .unwrap()
          .scroll_home();
      }
      ActiveSonarrBlock::FilterHistory => {
        self
          .app
          .data
          .sonarr_data
          .history
          .filter
          .as_mut()
          .unwrap()
          .scroll_home();
      }
      ActiveSonarrBlock::HistorySortPrompt => self
        .app
        .data
        .sonarr_data
        .history
        .sort
        .as_mut()
        .unwrap()
        .scroll_to_top(),
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::History => self.app.data.sonarr_data.history.scroll_to_bottom(),
      ActiveSonarrBlock::SearchHistory => self
        .app
        .data
        .sonarr_data
        .history
        .search
        .as_mut()
        .unwrap()
        .reset_offset(),
      ActiveSonarrBlock::FilterHistory => self
        .app
        .data
        .sonarr_data
        .history
        .filter
        .as_mut()
        .unwrap()
        .reset_offset(),
      ActiveSonarrBlock::HistorySortPrompt => self
        .app
        .data
        .sonarr_data
        .history
        .sort
        .as_mut()
        .unwrap()
        .scroll_to_bottom(),
      _ => (),
    }
  }

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::History => handle_change_tab_left_right_keys(self.app, self.key),
      ActiveSonarrBlock::SearchHistory => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self.app.data.sonarr_data.history.search.as_mut().unwrap()
        )
      }
      ActiveSonarrBlock::FilterHistory => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self.app.data.sonarr_data.history.filter.as_mut().unwrap()
        )
      }
      _ => {}
    }
  }

  fn handle_submit(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::SearchHistory => {
        self.app.pop_navigation_stack();
        self.app.should_ignore_quit_key = false;

        if self.app.data.sonarr_data.history.search.is_some() {
          let has_match = self
            .app
            .data
            .sonarr_data
            .history
            .apply_search(|history| &history.source_title.text);

          if !has_match {
            self
              .app
              .push_navigation_stack(ActiveSonarrBlock::SearchHistoryError.into());
          }
        }
      }
      ActiveSonarrBlock::FilterHistory => {
        self.app.pop_navigation_stack();
        self.app.should_ignore_quit_key = false;

        if self.app.data.sonarr_data.history.filter.is_some() {
          let has_matches = self
            .app
            .data
            .sonarr_data
            .history
            .apply_filter(|history| &history.source_title.text);

          if !has_matches {
            self
              .app
              .push_navigation_stack(ActiveSonarrBlock::FilterHistoryError.into());
          }
        }
      }
      ActiveSonarrBlock::HistorySortPrompt => {
        self
          .app
          .data
          .sonarr_data
          .history
          .items
          .sort_by(|a, b| a.id.cmp(&b.id));
        self.app.data.sonarr_data.history.apply_sorting();

        self.app.pop_navigation_stack();
      }
      ActiveSonarrBlock::History => {
        self
          .app
          .push_navigation_stack(ActiveSonarrBlock::HistoryItemDetails.into());
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::FilterHistory | ActiveSonarrBlock::FilterHistoryError => {
        self.app.pop_navigation_stack();
        self.app.data.sonarr_data.history.reset_filter();
        self.app.should_ignore_quit_key = false;
      }
      ActiveSonarrBlock::SearchHistory | ActiveSonarrBlock::SearchHistoryError => {
        self.app.pop_navigation_stack();
        self.app.data.sonarr_data.history.reset_search();
        self.app.should_ignore_quit_key = false;
      }
      ActiveSonarrBlock::HistoryItemDetails | ActiveSonarrBlock::HistorySortPrompt => {
        self.app.pop_navigation_stack();
      }
      _ => {
        self.app.data.sonarr_data.history.reset_search();
        self.app.data.sonarr_data.history.reset_filter();
        handle_clear_errors(self.app);
      }
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_sonarr_block {
      ActiveSonarrBlock::History => match self.key {
        _ if key == DEFAULT_KEYBINDINGS.search.key => {
          self
            .app
            .push_navigation_stack(ActiveSonarrBlock::SearchHistory.into());
          self.app.data.sonarr_data.history.search = Some(HorizontallyScrollableText::default());
          self.app.should_ignore_quit_key = true;
        }
        _ if key == DEFAULT_KEYBINDINGS.filter.key => {
          self
            .app
            .push_navigation_stack(ActiveSonarrBlock::FilterHistory.into());
          self.app.data.sonarr_data.history.reset_filter();
          self.app.data.sonarr_data.history.filter = Some(HorizontallyScrollableText::default());
          self.app.should_ignore_quit_key = true;
        }
        _ if key == DEFAULT_KEYBINDINGS.refresh.key => {
          self.app.should_refresh = true;
        }
        _ if key == DEFAULT_KEYBINDINGS.sort.key => {
          self
            .app
            .data
            .sonarr_data
            .history
            .sorting(history_sorting_options());
          self
            .app
            .push_navigation_stack(ActiveSonarrBlock::HistorySortPrompt.into());
        }
        _ => (),
      },
      ActiveSonarrBlock::SearchHistory => {
        handle_text_box_keys!(
          self,
          key,
          self.app.data.sonarr_data.history.search.as_mut().unwrap()
        )
      }
      ActiveSonarrBlock::FilterHistory => {
        handle_text_box_keys!(
          self,
          key,
          self.app.data.sonarr_data.history.filter.as_mut().unwrap()
        )
      }
      _ => (),
    }
  }
}

pub(in crate::handlers::sonarr_handlers) fn history_sorting_options() -> Vec<SortOption<SonarrHistoryItem>> {
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
        a.language
          .name
          .to_lowercase()
          .cmp(&b.language.name.to_lowercase())
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
