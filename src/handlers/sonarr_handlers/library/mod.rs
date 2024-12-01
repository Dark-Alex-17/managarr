use crate::{
  app::App,
  event::Key,
  handle_text_box_keys, handle_text_box_left_right_keys,
  handlers::{handle_clear_errors, handle_prompt_toggle, KeyEventHandler},
  models::{
    servarr_data::sonarr::sonarr_data::{
      ActiveSonarrBlock, DELETE_SERIES_SELECTION_BLOCKS, EDIT_SERIES_SELECTION_BLOCKS,
      SERIES_BLOCKS,
    },
    sonarr_models::Series,
    stateful_table::SortOption,
    BlockSelectionState, HorizontallyScrollableText, Scrollable,
  },
  network::sonarr_network::SonarrEvent,
};

use super::handle_change_tab_left_right_keys;
use crate::app::key_binding::DEFAULT_KEYBINDINGS;

#[cfg(test)]
#[path = "library_handler_tests.rs"]
mod library_handler_tests;

pub(super) struct LibraryHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_sonarr_block: ActiveSonarrBlock,
  _context: Option<ActiveSonarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveSonarrBlock> for LibraryHandler<'a, 'b> {
  fn handle(&mut self) {
    match self.active_sonarr_block {
      _ => self.handle_key_event(),
    }
  }

  fn accepts(active_block: ActiveSonarrBlock) -> bool {
    SERIES_BLOCKS.contains(&active_block)
  }

  fn with(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveSonarrBlock,
    _context: Option<ActiveSonarrBlock>,
  ) -> LibraryHandler<'a, 'b> {
    LibraryHandler {
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
    !self.app.is_loading && !self.app.data.sonarr_data.series.is_empty()
  }

  fn handle_scroll_up(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::Series => self.app.data.sonarr_data.series.scroll_up(),
      ActiveSonarrBlock::SeriesSortPrompt => self
        .app
        .data
        .sonarr_data
        .series
        .sort
        .as_mut()
        .unwrap()
        .scroll_up(),
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::Series => self.app.data.sonarr_data.series.scroll_down(),
      ActiveSonarrBlock::SeriesSortPrompt => self
        .app
        .data
        .sonarr_data
        .series
        .sort
        .as_mut()
        .unwrap()
        .scroll_down(),
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::Series => self.app.data.sonarr_data.series.scroll_to_top(),
      ActiveSonarrBlock::SearchSeries => {
        self
          .app
          .data
          .sonarr_data
          .series
          .search
          .as_mut()
          .unwrap()
          .scroll_home();
      }
      ActiveSonarrBlock::FilterSeries => {
        self
          .app
          .data
          .sonarr_data
          .series
          .filter
          .as_mut()
          .unwrap()
          .scroll_home();
      }
      ActiveSonarrBlock::SeriesSortPrompt => self
        .app
        .data
        .sonarr_data
        .series
        .sort
        .as_mut()
        .unwrap()
        .scroll_to_top(),
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::Series => self.app.data.sonarr_data.series.scroll_to_bottom(),
      ActiveSonarrBlock::SearchSeries => self
        .app
        .data
        .sonarr_data
        .series
        .search
        .as_mut()
        .unwrap()
        .reset_offset(),
      ActiveSonarrBlock::FilterSeries => self
        .app
        .data
        .sonarr_data
        .series
        .filter
        .as_mut()
        .unwrap()
        .reset_offset(),
      ActiveSonarrBlock::SeriesSortPrompt => self
        .app
        .data
        .sonarr_data
        .series
        .sort
        .as_mut()
        .unwrap()
        .scroll_to_bottom(),
      _ => (),
    }
  }

  fn handle_delete(&mut self) {
    if self.active_sonarr_block == ActiveSonarrBlock::Series {
      self
        .app
        .push_navigation_stack(ActiveSonarrBlock::DeleteSeriesPrompt.into());
      self.app.data.sonarr_data.selected_block =
        BlockSelectionState::new(DELETE_SERIES_SELECTION_BLOCKS);
    }
  }

  fn handle_left_right_action(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::Series => handle_change_tab_left_right_keys(self.app, self.key),
      ActiveSonarrBlock::UpdateAllSeriesPrompt => handle_prompt_toggle(self.app, self.key),
      ActiveSonarrBlock::SearchSeries => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self.app.data.sonarr_data.series.search.as_mut().unwrap()
        )
      }
      ActiveSonarrBlock::FilterSeries => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self.app.data.sonarr_data.series.filter.as_mut().unwrap()
        )
      }
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::Series => self
        .app
        .push_navigation_stack(ActiveSonarrBlock::SeriesDetails.into()),
      ActiveSonarrBlock::SearchSeries => {
        self.app.pop_navigation_stack();
        self.app.should_ignore_quit_key = false;

        if self.app.data.sonarr_data.series.search.is_some() {
          let has_match = self
            .app
            .data
            .sonarr_data
            .series
            .apply_search(|series| &series.title.text);

          if !has_match {
            self
              .app
              .push_navigation_stack(ActiveSonarrBlock::SearchSeriesError.into());
          }
        }
      }
      ActiveSonarrBlock::FilterSeries => {
        self.app.pop_navigation_stack();
        self.app.should_ignore_quit_key = false;

        if self.app.data.sonarr_data.series.filter.is_some() {
          let has_matches = self
            .app
            .data
            .sonarr_data
            .series
            .apply_filter(|series| &series.title.text);

          if !has_matches {
            self
              .app
              .push_navigation_stack(ActiveSonarrBlock::FilterSeriesError.into());
          }
        }
      }
      ActiveSonarrBlock::UpdateAllSeriesPrompt => {
        if self.app.data.sonarr_data.prompt_confirm {
          self.app.data.sonarr_data.prompt_confirm_action = Some(SonarrEvent::UpdateAllSeries);
        }

        self.app.pop_navigation_stack();
      }
      ActiveSonarrBlock::SeriesSortPrompt => {
        self
          .app
          .data
          .sonarr_data
          .series
          .items
          .sort_by(|a, b| a.id.cmp(&b.id));
        self.app.data.sonarr_data.series.apply_sorting();

        self.app.pop_navigation_stack();
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::FilterSeries | ActiveSonarrBlock::FilterSeriesError => {
        self.app.pop_navigation_stack();
        self.app.data.sonarr_data.series.reset_filter();
        self.app.should_ignore_quit_key = false;
      }
      ActiveSonarrBlock::SearchSeries | ActiveSonarrBlock::SearchSeriesError => {
        self.app.pop_navigation_stack();
        self.app.data.sonarr_data.series.reset_search();
        self.app.should_ignore_quit_key = false;
      }
      ActiveSonarrBlock::UpdateAllSeriesPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.sonarr_data.prompt_confirm = false;
      }
      ActiveSonarrBlock::SeriesSortPrompt => {
        self.app.pop_navigation_stack();
      }
      _ => {
        self.app.data.sonarr_data.series.reset_search();
        self.app.data.sonarr_data.series.reset_filter();
        handle_clear_errors(self.app);
      }
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_sonarr_block {
      ActiveSonarrBlock::Series => match self.key {
        _ if key == DEFAULT_KEYBINDINGS.search.key => {
          self
            .app
            .push_navigation_stack(ActiveSonarrBlock::SearchSeries.into());
          self.app.data.sonarr_data.series.search = Some(HorizontallyScrollableText::default());
          self.app.should_ignore_quit_key = true;
        }
        _ if key == DEFAULT_KEYBINDINGS.filter.key => {
          self
            .app
            .push_navigation_stack(ActiveSonarrBlock::FilterSeries.into());
          self.app.data.sonarr_data.series.reset_filter();
          self.app.data.sonarr_data.series.filter = Some(HorizontallyScrollableText::default());
          self.app.should_ignore_quit_key = true;
        }
        _ if key == DEFAULT_KEYBINDINGS.edit.key => {
          self.app.push_navigation_stack(
            (
              ActiveSonarrBlock::EditSeriesPrompt,
              Some(ActiveSonarrBlock::Series),
            )
              .into(),
          );
          self.app.data.sonarr_data.edit_series_modal = Some((&self.app.data.sonarr_data).into());
          self.app.data.sonarr_data.selected_block =
            BlockSelectionState::new(EDIT_SERIES_SELECTION_BLOCKS);
        }
        _ if key == DEFAULT_KEYBINDINGS.add.key => {
          self
            .app
            .push_navigation_stack(ActiveSonarrBlock::AddSeriesSearchInput.into());
          self.app.data.sonarr_data.add_series_search = Some(HorizontallyScrollableText::default());
          self.app.should_ignore_quit_key = true;
        }
        _ if key == DEFAULT_KEYBINDINGS.update.key => {
          self
            .app
            .push_navigation_stack(ActiveSonarrBlock::UpdateAllSeriesPrompt.into());
        }
        _ if key == DEFAULT_KEYBINDINGS.refresh.key => {
          self.app.should_refresh = true;
        }
        _ if key == DEFAULT_KEYBINDINGS.sort.key => {
          self
            .app
            .data
            .sonarr_data
            .series
            .sorting(series_sorting_options());
          self
            .app
            .push_navigation_stack(ActiveSonarrBlock::SeriesSortPrompt.into());
        }
        _ => (),
      },
      ActiveSonarrBlock::SearchSeries => {
        handle_text_box_keys!(
          self,
          key,
          self.app.data.sonarr_data.series.search.as_mut().unwrap()
        )
      }
      ActiveSonarrBlock::FilterSeries => {
        handle_text_box_keys!(
          self,
          key,
          self.app.data.sonarr_data.series.filter.as_mut().unwrap()
        )
      }
      ActiveSonarrBlock::UpdateAllSeriesPrompt => {
        if key == DEFAULT_KEYBINDINGS.confirm.key {
          self.app.data.sonarr_data.prompt_confirm = true;
          self.app.data.sonarr_data.prompt_confirm_action = Some(SonarrEvent::UpdateAllSeries);

          self.app.pop_navigation_stack();
        }
      }
      _ => (),
    }
  }
}

fn series_sorting_options() -> Vec<SortOption<Series>> {
  vec![
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
      name: "Year",
      cmp_fn: Some(|a, b| a.year.cmp(&b.year)),
    },
    SortOption {
      name: "Network",
      cmp_fn: Some(|a, b| {
        a.network
          .as_ref()
          .unwrap_or(&String::new())
          .to_lowercase()
          .cmp(&b.network.as_ref().unwrap_or(&String::new()).to_lowercase())
      }),
    },
    SortOption {
      name: "Runtime",
      cmp_fn: Some(|a, b| a.runtime.cmp(&b.runtime)),
    },
    SortOption {
      name: "Rating",
      cmp_fn: Some(|a, b| {
        a.certification
          .as_ref()
          .unwrap_or(&String::new())
          .to_lowercase()
          .cmp(
            &b.certification
              .as_ref()
              .unwrap_or(&String::new())
              .to_lowercase(),
          )
      }),
    },
    SortOption {
      name: "Type",
      cmp_fn: Some(|a, b| a.series_type.to_string().cmp(&b.series_type.to_string())),
    },
    SortOption {
      name: "Quality",
      cmp_fn: Some(|a, b| a.quality_profile_id.cmp(&b.quality_profile_id)),
    },
    SortOption {
      name: "Language",
      cmp_fn: Some(|a, b| a.language_profile_id.cmp(&b.language_profile_id)),
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
