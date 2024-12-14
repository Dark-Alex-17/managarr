use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::App;
use crate::event::Key;
use crate::handle_table_events;
use crate::handlers::sonarr_handlers::history::history_sorting_options;
use crate::handlers::table_handler::TableHandlingConfig;
use crate::handlers::{handle_prompt_toggle, KeyEventHandler};
use crate::models::servarr_data::sonarr::sonarr_data::{
  ActiveSonarrBlock, EDIT_SERIES_SELECTION_BLOCKS, SERIES_DETAILS_BLOCKS,
};
use crate::models::sonarr_models::{Season, SonarrHistoryItem};
use crate::models::BlockSelectionState;
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

impl<'a, 'b> SeriesDetailsHandler<'a, 'b> {
  handle_table_events!(self, season, self.app.data.sonarr_data.seasons, Season);
  handle_table_events!(
    self,
    series_history,
    self
      .app
      .data
      .sonarr_data
      .series_history
      .as_mut()
      .expect("Series history is undefined"),
    SonarrHistoryItem
  );
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveSonarrBlock> for SeriesDetailsHandler<'a, 'b> {
  fn handle(&mut self) {
    let season_table_handling_config =
      TableHandlingConfig::new(ActiveSonarrBlock::SeriesDetails.into())
        .searching_block(ActiveSonarrBlock::SearchSeason.into())
        .search_error_block(ActiveSonarrBlock::SearchSeasonError.into())
        .search_field_fn(|season: &Season| {
          season
            .title
            .as_ref()
            .expect("Season was not populated with title in handlers")
        });
    let series_history_table_handling_config =
      TableHandlingConfig::new(ActiveSonarrBlock::SeriesHistory.into())
        .sorting_block(ActiveSonarrBlock::SeriesHistorySortPrompt.into())
        .sort_options(history_sorting_options())
        .sort_by_fn(|a: &SonarrHistoryItem, b: &SonarrHistoryItem| a.id.cmp(&b.id))
        .searching_block(ActiveSonarrBlock::SearchSeriesHistory.into())
        .search_error_block(ActiveSonarrBlock::SearchSeriesHistoryError.into())
        .search_field_fn(|history_item: &SonarrHistoryItem| &history_item.source_title.text)
        .filtering_block(ActiveSonarrBlock::FilterSeriesHistory.into())
        .filter_error_block(ActiveSonarrBlock::FilterSeriesHistoryError.into())
        .filter_field_fn(|history_item: &SonarrHistoryItem| &history_item.source_title.text);

    if !self.handle_season_table_events(season_table_handling_config)
      && !self.handle_series_history_table_events(series_history_table_handling_config)
    {
      self.handle_key_event();
    }
  }

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

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

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
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::UpdateAndScanSeriesPrompt
      | ActiveSonarrBlock::AutomaticallySearchSeriesPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.sonarr_data.prompt_confirm = false;
      }
      ActiveSonarrBlock::SeriesHistoryDetails => {
        self.app.pop_navigation_stack();
      }
      ActiveSonarrBlock::SeriesHistory => {
        if self
          .app
          .data
          .sonarr_data
          .series_history
          .as_ref()
          .expect("Series history is not populated")
          .filtered_items
          .is_some()
        {
          self
            .app
            .data
            .sonarr_data
            .series_history
            .as_mut()
            .expect("Series history is not populated")
            .reset_filter();
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
        _ if key == DEFAULT_KEYBINDINGS.toggle_monitoring.key => {
          self.app.data.sonarr_data.prompt_confirm = true;
          self.app.data.sonarr_data.prompt_confirm_action =
            Some(SonarrEvent::ToggleSeasonMonitoring(None));

          self
            .app
            .pop_and_push_navigation_stack(self.active_sonarr_block.into());
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
