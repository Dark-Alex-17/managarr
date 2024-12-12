use add_series_handler::AddSeriesHandler;
mod edit_series_handler;
use delete_series_handler::DeleteSeriesHandler;
use edit_series_handler::EditSeriesHandler;

use crate::{
  app::App,
  event::Key,
  handle_table_events,
  handlers::{handle_clear_errors, handle_prompt_toggle, KeyEventHandler},
  models::{
    servarr_data::sonarr::sonarr_data::{
      ActiveSonarrBlock, DELETE_SERIES_SELECTION_BLOCKS, EDIT_SERIES_SELECTION_BLOCKS,
      LIBRARY_BLOCKS,
    },
    sonarr_models::Series,
    stateful_table::SortOption,
    BlockSelectionState, HorizontallyScrollableText,
  },
  network::sonarr_network::SonarrEvent,
};

use super::handle_change_tab_left_right_keys;
use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::handlers::sonarr_handlers::library::season_details_handler::SeasonDetailsHandler;
use crate::handlers::sonarr_handlers::library::series_details_handler::SeriesDetailsHandler;
use crate::handlers::table_handler::TableHandlingConfig;

mod add_series_handler;
mod delete_series_handler;

#[cfg(test)]
#[path = "library_handler_tests.rs"]
mod library_handler_tests;
mod series_details_handler;
mod season_details_handler;

pub(super) struct LibraryHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_sonarr_block: ActiveSonarrBlock,
  context: Option<ActiveSonarrBlock>,
}

impl<'a, 'b> LibraryHandler<'a, 'b> {
  handle_table_events!(self, series, self.app.data.sonarr_data.series, Series);
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveSonarrBlock> for LibraryHandler<'a, 'b> {
  fn handle(&mut self) {
    let series_table_handling_config = TableHandlingConfig::new(ActiveSonarrBlock::Series.into())
      .sorting_block(ActiveSonarrBlock::SeriesSortPrompt.into())
      .sort_by_fn(|a: &Series, b: &Series| a.id.cmp(&b.id))
      .sort_options(series_sorting_options())
      .searching_block(ActiveSonarrBlock::SearchSeries.into())
      .search_error_block(ActiveSonarrBlock::SearchSeriesError.into())
      .search_field_fn(|series| &series.title.text)
      .filtering_block(ActiveSonarrBlock::FilterSeries.into())
      .filter_error_block(ActiveSonarrBlock::FilterSeriesError.into())
      .filter_field_fn(|series| &series.title.text);

    if !self.handle_series_table_events(series_table_handling_config) {
      match self.active_sonarr_block {
        _ if AddSeriesHandler::accepts(self.active_sonarr_block) => {
          AddSeriesHandler::with(self.key, self.app, self.active_sonarr_block, self.context)
            .handle();
        }
        _ if DeleteSeriesHandler::accepts(self.active_sonarr_block) => {
          DeleteSeriesHandler::with(self.key, self.app, self.active_sonarr_block, self.context)
            .handle();
        }
        _ if EditSeriesHandler::accepts(self.active_sonarr_block) => {
          EditSeriesHandler::with(self.key, self.app, self.active_sonarr_block, self.context)
            .handle();
        }
        _ if SeriesDetailsHandler::accepts(self.active_sonarr_block) => {
          SeriesDetailsHandler::with(self.key, self.app, self.active_sonarr_block, self.context)
            .handle();
        }
        _ if SeasonDetailsHandler::accepts(self.active_sonarr_block) => {
          SeasonDetailsHandler::with(self.key, self.app, self.active_sonarr_block, self.context)
            .handle();
        }
        _ => self.handle_key_event(),
      }
    }
  }

  fn accepts(active_block: ActiveSonarrBlock) -> bool {
    AddSeriesHandler::accepts(active_block)
      || DeleteSeriesHandler::accepts(active_block)
      || EditSeriesHandler::accepts(active_block)
      || SeriesDetailsHandler::accepts(active_block)
      || SeasonDetailsHandler::accepts(active_block)
      || LIBRARY_BLOCKS.contains(&active_block)
  }

  fn with(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveSonarrBlock,
    context: Option<ActiveSonarrBlock>,
  ) -> LibraryHandler<'a, 'b> {
    LibraryHandler {
      key,
      app,
      active_sonarr_block: active_block,
      context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    !self.app.is_loading && !self.app.data.sonarr_data.series.is_empty()
  }

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

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
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::Series => self
        .app
        .push_navigation_stack(ActiveSonarrBlock::SeriesDetails.into()),
      ActiveSonarrBlock::UpdateAllSeriesPrompt => {
        if self.app.data.sonarr_data.prompt_confirm {
          self.app.data.sonarr_data.prompt_confirm_action = Some(SonarrEvent::UpdateAllSeries);
        }

        self.app.pop_navigation_stack();
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::UpdateAllSeriesPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.sonarr_data.prompt_confirm = false;
      }
      _ => {
        handle_clear_errors(self.app);
      }
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_sonarr_block {
      ActiveSonarrBlock::Series => match self.key {
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
        _ => (),
      },
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
      name: "Status",
      cmp_fn: Some(|a, b| {
        a.status
          .to_string()
          .to_lowercase()
          .cmp(&b.status.to_string().to_lowercase())
      }),
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
