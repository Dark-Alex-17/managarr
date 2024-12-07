use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::App;
use crate::event::Key;
use crate::handle_text_box_keys;
use crate::handlers::sonarr_handlers::history::history_sorting_options;
use crate::handlers::{handle_prompt_toggle, KeyEventHandler};
use crate::models::servarr_data::sonarr::sonarr_data::{
  ActiveSonarrBlock, EDIT_SERIES_SELECTION_BLOCKS, SERIES_DETAILS_BLOCKS,
};
use crate::models::{BlockSelectionState, HorizontallyScrollableText, Scrollable};
use crate::network::sonarr_network::SonarrEvent;

#[cfg(test)]
#[path = "series_details_handler_tests.rs"]
mod series_details_handler_tests;

pub(super) struct SeriesDetailsHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_sonarr_block: ActiveSonarrBlock,
  _context: Option<ActiveSonarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveSonarrBlock> for SeriesDetailsHandler<'a, 'b> {
  fn accepts(active_block: ActiveSonarrBlock) -> bool {
    SERIES_DETAILS_BLOCKS.contains(&active_block)
  }

  fn with(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveSonarrBlock,
    _context: Option<ActiveSonarrBlock>,
  ) -> SeriesDetailsHandler<'a, 'b> {
    SeriesDetailsHandler {
      key,
      app,
      active_sonarr_block: active_block,
      _context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    if self.active_sonarr_block == ActiveSonarrBlock::SeriesHistory {
      !self.app.is_loading && self.app.data.sonarr_data.series_history.is_some()
    } else {
      !self.app.is_loading
    }
  }

  fn handle_scroll_up(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::SeriesDetails => self.app.data.sonarr_data.seasons.scroll_up(),
      ActiveSonarrBlock::SeriesHistory => self
        .app
        .data
        .sonarr_data
        .series_history
        .as_mut()
        .unwrap()
        .scroll_up(),
      ActiveSonarrBlock::SeriesHistorySortPrompt => self
        .app
        .data
        .sonarr_data
        .series_history
        .as_mut()
        .unwrap()
        .sort
        .as_mut()
        .unwrap()
        .scroll_up(),
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::SeriesDetails => self.app.data.sonarr_data.seasons.scroll_down(),
      ActiveSonarrBlock::SeriesHistory => self
        .app
        .data
        .sonarr_data
        .series_history
        .as_mut()
        .unwrap()
        .scroll_down(),
      ActiveSonarrBlock::SeriesHistorySortPrompt => self
        .app
        .data
        .sonarr_data
        .series_history
        .as_mut()
        .unwrap()
        .sort
        .as_mut()
        .unwrap()
        .scroll_down(),
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::SeriesDetails => self.app.data.sonarr_data.seasons.scroll_to_top(),
      ActiveSonarrBlock::SearchSeason => self
        .app
        .data
        .sonarr_data
        .seasons
        .search
        .as_mut()
        .unwrap()
        .scroll_home(),
      ActiveSonarrBlock::SearchSeriesHistory => self
        .app
        .data
        .sonarr_data
        .series_history
        .as_mut()
        .unwrap()
        .search
        .as_mut()
        .unwrap()
        .scroll_home(),
      ActiveSonarrBlock::FilterSeriesHistory => self
        .app
        .data
        .sonarr_data
        .series_history
        .as_mut()
        .unwrap()
        .filter
        .as_mut()
        .unwrap()
        .scroll_home(),
      ActiveSonarrBlock::SeriesHistory => self
        .app
        .data
        .sonarr_data
        .series_history
        .as_mut()
        .unwrap()
        .scroll_to_top(),
      ActiveSonarrBlock::SeriesHistorySortPrompt => self
        .app
        .data
        .sonarr_data
        .series_history
        .as_mut()
        .unwrap()
        .sort
        .as_mut()
        .unwrap()
        .scroll_to_top(),
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::SeriesDetails => self.app.data.sonarr_data.seasons.scroll_to_bottom(),
      ActiveSonarrBlock::SearchSeason => self
        .app
        .data
        .sonarr_data
        .seasons
        .search
        .as_mut()
        .unwrap()
        .reset_offset(),
      ActiveSonarrBlock::SearchSeriesHistory => self
        .app
        .data
        .sonarr_data
        .series_history
        .as_mut()
        .unwrap()
        .search
        .as_mut()
        .unwrap()
        .reset_offset(),
      ActiveSonarrBlock::FilterSeriesHistory => self
        .app
        .data
        .sonarr_data
        .series_history
        .as_mut()
        .unwrap()
        .filter
        .as_mut()
        .unwrap()
        .reset_offset(),
      ActiveSonarrBlock::SeriesHistory => self
        .app
        .data
        .sonarr_data
        .series_history
        .as_mut()
        .unwrap()
        .scroll_to_bottom(),
      ActiveSonarrBlock::SeriesHistorySortPrompt => self
        .app
        .data
        .sonarr_data
        .series_history
        .as_mut()
        .unwrap()
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
      ActiveSonarrBlock::SeriesDetails | ActiveSonarrBlock::SeriesHistory => match self.key {
        _ if self.key == DEFAULT_KEYBINDINGS.left.key => {
          self.app.data.sonarr_data.series_info_tabs.previous();
          self.app.pop_and_push_navigation_stack(
            self
              .app
              .data
              .sonarr_data
              .series_info_tabs
              .get_active_route(),
          );
        }
        _ if self.key == DEFAULT_KEYBINDINGS.right.key => {
          self.app.data.sonarr_data.series_info_tabs.next();
          self.app.pop_and_push_navigation_stack(
            self
              .app
              .data
              .sonarr_data
              .series_info_tabs
              .get_active_route(),
          );
        }
        _ => (),
      },
      ActiveSonarrBlock::UpdateAndScanSeriesPrompt
      | ActiveSonarrBlock::AutomaticallySearchSeriesPrompt => {
        handle_prompt_toggle(self.app, self.key)
      }
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::SeriesDetails if !self.app.data.sonarr_data.seasons.is_empty() => {
        self
          .app
          .push_navigation_stack(ActiveSonarrBlock::SeasonDetails.into());
      }
      ActiveSonarrBlock::SeriesHistory
        if !self
          .app
          .data
          .sonarr_data
          .series_history
          .as_ref()
          .expect("Series history should be Some")
          .is_empty() =>
      {
        self
          .app
          .push_navigation_stack(ActiveSonarrBlock::SeriesHistoryDetails.into());
      }
      ActiveSonarrBlock::AutomaticallySearchSeriesPrompt => {
        if self.app.data.sonarr_data.prompt_confirm {
          self.app.data.sonarr_data.prompt_confirm_action =
            Some(SonarrEvent::TriggerAutomaticSeriesSearch(None));
        }

        self.app.pop_navigation_stack();
      }
      ActiveSonarrBlock::UpdateAndScanSeriesPrompt => {
        if self.app.data.sonarr_data.prompt_confirm {
          self.app.data.sonarr_data.prompt_confirm_action =
            Some(SonarrEvent::UpdateAndScanSeries(None));
        }

        self.app.pop_navigation_stack();
      }
      ActiveSonarrBlock::SearchSeason => {
        self.app.pop_navigation_stack();
        self.app.should_ignore_quit_key = false;

        if self.app.data.sonarr_data.seasons.search.is_some() {
          let has_match = self.app.data.sonarr_data.seasons.apply_search(|season| {
            season
              .title
              .as_ref()
              .expect("Season was not populated with title in handlers")
          });

          if !has_match {
            self
              .app
              .push_navigation_stack(ActiveSonarrBlock::SearchSeasonError.into());
          }
        }
      }
      ActiveSonarrBlock::SearchSeriesHistory => {
        self.app.pop_navigation_stack();
        self.app.should_ignore_quit_key = false;

        if self
          .app
          .data
          .sonarr_data
          .series_history
          .as_ref()
          .unwrap()
          .search
          .is_some()
        {
          let has_match = self
            .app
            .data
            .sonarr_data
            .series_history
            .as_mut()
            .unwrap()
            .apply_search(|history_item| &history_item.source_title.text);

          if !has_match {
            self
              .app
              .push_navigation_stack(ActiveSonarrBlock::SearchSeriesHistoryError.into());
          }
        }
      }
      ActiveSonarrBlock::FilterSeriesHistory => {
        self.app.pop_navigation_stack();
        self.app.should_ignore_quit_key = false;

        if self
          .app
          .data
          .sonarr_data
          .series_history
          .as_ref()
          .unwrap()
          .filter
          .is_some()
        {
          let has_matches = self
            .app
            .data
            .sonarr_data
            .series_history
            .as_mut()
            .unwrap()
            .apply_filter(|history_item| &history_item.source_title.text);

          if !has_matches {
            self
              .app
              .push_navigation_stack(ActiveSonarrBlock::FilterSeriesHistoryError.into());
          }
        }
      }
      ActiveSonarrBlock::SeriesHistorySortPrompt => {
        self
          .app
          .data
          .sonarr_data
          .series_history
          .as_mut()
          .unwrap()
          .items
          .sort_by(|a, b| a.id.cmp(&b.id));
        self
          .app
          .data
          .sonarr_data
          .series_history
          .as_mut()
          .unwrap()
          .apply_sorting();

        self.app.pop_navigation_stack();
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::SearchSeason | ActiveSonarrBlock::SearchSeasonError => {
        self.app.pop_navigation_stack();
        self.app.data.sonarr_data.seasons.reset_search();
        self.app.should_ignore_quit_key = false;
      }
      ActiveSonarrBlock::SearchSeriesHistory | ActiveSonarrBlock::SearchSeriesHistoryError => {
        self.app.pop_navigation_stack();
        self
          .app
          .data
          .sonarr_data
          .series_history
          .as_mut()
          .unwrap()
          .reset_search();
        self.app.should_ignore_quit_key = false;
      }
      ActiveSonarrBlock::FilterSeriesHistory | ActiveSonarrBlock::FilterSeriesHistoryError => {
        self.app.pop_navigation_stack();
        self
          .app
          .data
          .sonarr_data
          .series_history
          .as_mut()
          .unwrap()
          .reset_filter();
        self.app.should_ignore_quit_key = false;
      }
      ActiveSonarrBlock::UpdateAndScanSeriesPrompt
      | ActiveSonarrBlock::AutomaticallySearchSeriesPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.sonarr_data.prompt_confirm = false;
      }
      ActiveSonarrBlock::SeriesHistoryDetails | ActiveSonarrBlock::SeriesHistorySortPrompt => {
        self.app.pop_navigation_stack();
      }
      ActiveSonarrBlock::SeriesHistory => {
        if self.app.data.sonarr_data.series_history.as_ref().expect("Series history is not populated").filtered_items.is_some() {
          self.app.data.sonarr_data.series_history.as_mut().expect("Series history is not populated").reset_filter();
        } else {
          self.app.pop_navigation_stack();
          self.app.data.sonarr_data.reset_series_info_tabs();
        }
      }
      ActiveSonarrBlock::SeriesDetails => {
        self.app.pop_navigation_stack();
        self.app.data.sonarr_data.reset_series_info_tabs();
      }
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_sonarr_block {
      ActiveSonarrBlock::SeriesDetails => match self.key {
        _ if key == DEFAULT_KEYBINDINGS.refresh.key => self
          .app
          .pop_and_push_navigation_stack(self.active_sonarr_block.into()),
        _ if key == DEFAULT_KEYBINDINGS.search.key => {
          self
            .app
            .push_navigation_stack(ActiveSonarrBlock::SearchSeason.into());
          self.app.data.sonarr_data.seasons.search = Some(HorizontallyScrollableText::default());
          self.app.should_ignore_quit_key = true;
        }
        _ if key == DEFAULT_KEYBINDINGS.auto_search.key => {
          self
            .app
            .push_navigation_stack(ActiveSonarrBlock::AutomaticallySearchSeriesPrompt.into());
        }
        _ if key == DEFAULT_KEYBINDINGS.update.key => {
          self
            .app
            .push_navigation_stack(ActiveSonarrBlock::UpdateAndScanSeriesPrompt.into());
        }
        _ if key == DEFAULT_KEYBINDINGS.edit.key => {
          self.app.push_navigation_stack(
            (
              ActiveSonarrBlock::EditSeriesPrompt,
              Some(self.active_sonarr_block),
            )
              .into(),
          );
          self.app.data.sonarr_data.edit_series_modal = Some((&self.app.data.sonarr_data).into());
          self.app.data.sonarr_data.selected_block =
            BlockSelectionState::new(EDIT_SERIES_SELECTION_BLOCKS);
        }
        _ => (),
      },
      ActiveSonarrBlock::SeriesHistory => match self.key {
        _ if key == DEFAULT_KEYBINDINGS.refresh.key => self
          .app
          .pop_and_push_navigation_stack(self.active_sonarr_block.into()),
        _ if key == DEFAULT_KEYBINDINGS.auto_search.key => {
          self
            .app
            .push_navigation_stack(ActiveSonarrBlock::AutomaticallySearchSeriesPrompt.into());
        }
        _ if key == DEFAULT_KEYBINDINGS.search.key => {
          self
            .app
            .push_navigation_stack(ActiveSonarrBlock::SearchSeriesHistory.into());
          self
            .app
            .data
            .sonarr_data
            .series_history
            .as_mut()
            .expect("Series history should be populated")
            .search = Some(HorizontallyScrollableText::default());
          self.app.should_ignore_quit_key = true;
        }
        _ if key == DEFAULT_KEYBINDINGS.filter.key => {
          self
            .app
            .push_navigation_stack(ActiveSonarrBlock::FilterSeriesHistory.into());
          self
            .app
            .data
            .sonarr_data
            .series_history
            .as_mut()
            .expect("Series history should be populated")
            .reset_filter();
          self
            .app
            .data
            .sonarr_data
            .series_history
            .as_mut()
            .expect("Series history should be populated")
            .filter = Some(HorizontallyScrollableText::default());
          self.app.should_ignore_quit_key = true;
        }
        _ if key == DEFAULT_KEYBINDINGS.sort.key => {
          self
            .app
            .data
            .sonarr_data
            .series_history
            .as_mut()
            .expect("Series history should be populated")
            .sorting(history_sorting_options());
          self
            .app
            .push_navigation_stack(ActiveSonarrBlock::SeriesHistorySortPrompt.into());
        }
        _ if key == DEFAULT_KEYBINDINGS.edit.key => {
          self.app.push_navigation_stack(
            (
              ActiveSonarrBlock::EditSeriesPrompt,
              Some(self.active_sonarr_block),
            )
              .into(),
          );
          self.app.data.sonarr_data.edit_series_modal = Some((&self.app.data.sonarr_data).into());
          self.app.data.sonarr_data.selected_block =
            BlockSelectionState::new(EDIT_SERIES_SELECTION_BLOCKS);
        }
        _ if key == DEFAULT_KEYBINDINGS.update.key => {
          self
            .app
            .push_navigation_stack(ActiveSonarrBlock::UpdateAndScanSeriesPrompt.into());
        }
        _ => (),
      },
      ActiveSonarrBlock::SearchSeason => {
        handle_text_box_keys!(
          self,
          key,
          self.app.data.sonarr_data.seasons.search.as_mut().unwrap()
        )
      }
      ActiveSonarrBlock::SearchSeriesHistory => {
        handle_text_box_keys!(
          self,
          key,
          self.app.data.sonarr_data.series_history.as_mut().expect("Series history should be populated").search.as_mut().unwrap()
        )
      }
      ActiveSonarrBlock::FilterSeriesHistory => {
        handle_text_box_keys!(
          self,
          key,
          self.app.data.sonarr_data.series_history.as_mut().expect("Series history should be populated").filter.as_mut().unwrap()
        )
      }
      ActiveSonarrBlock::AutomaticallySearchSeriesPrompt => {
        if key == DEFAULT_KEYBINDINGS.confirm.key {
          self.app.data.sonarr_data.prompt_confirm = true;
          self.app.data.sonarr_data.prompt_confirm_action =
            Some(SonarrEvent::TriggerAutomaticSeriesSearch(None));

          self.app.pop_navigation_stack();
        }
      }
      ActiveSonarrBlock::UpdateAndScanSeriesPrompt => {
        if self.app.data.sonarr_data.prompt_confirm {
          self.app.data.sonarr_data.prompt_confirm_action =
            Some(SonarrEvent::UpdateAndScanSeries(None));
        }

        self.app.pop_navigation_stack();
      }
      _ => (),
    }
  }
}
