use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::handlers::table_handler::TableHandlingConfig;
use crate::handlers::{handle_prompt_toggle, KeyEventHandler};
use crate::models::servarr_data::sonarr::sonarr_data::{
  ActiveSonarrBlock, ADD_SERIES_BLOCKS, ADD_SERIES_SELECTION_BLOCKS,
};
use crate::models::sonarr_models::AddSeriesSearchResult;
use crate::models::{BlockSelectionState, Scrollable};
use crate::network::sonarr_network::SonarrEvent;
use crate::{handle_table_events, handle_text_box_keys, handle_text_box_left_right_keys, App, Key};

#[cfg(test)]
#[path = "add_series_handler_tests.rs"]
mod add_series_handler_tests;

pub(super) struct AddSeriesHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_sonarr_block: ActiveSonarrBlock,
  _context: Option<ActiveSonarrBlock>,
}

impl<'a, 'b> AddSeriesHandler<'a, 'b> {
  handle_table_events!(
    self,
    add_searched_series,
    self
      .app
      .data
      .sonarr_data
      .add_searched_series
      .as_mut()
      .unwrap(),
    AddSeriesSearchResult
  );
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveSonarrBlock> for AddSeriesHandler<'a, 'b> {
  fn handle(&mut self) {
    let add_series_table_handling_config =
      TableHandlingConfig::new(ActiveSonarrBlock::AddSeriesSearchResults.into());

    if !self.handle_add_searched_series_table_events(add_series_table_handling_config) {
      self.handle_key_event();
    }
  }

  fn accepts(active_block: ActiveSonarrBlock) -> bool {
    ADD_SERIES_BLOCKS.contains(&active_block)
  }

  fn with(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveSonarrBlock,
    _context: Option<ActiveSonarrBlock>,
  ) -> AddSeriesHandler<'a, 'b> {
    AddSeriesHandler {
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
    !self.app.is_loading
  }

  fn handle_scroll_up(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::AddSeriesSelectMonitor => self
        .app
        .data
        .sonarr_data
        .add_series_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .scroll_up(),
      ActiveSonarrBlock::AddSeriesSelectSeriesType => self
        .app
        .data
        .sonarr_data
        .add_series_modal
        .as_mut()
        .unwrap()
        .series_type_list
        .scroll_up(),
      ActiveSonarrBlock::AddSeriesSelectQualityProfile => self
        .app
        .data
        .sonarr_data
        .add_series_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_up(),
      ActiveSonarrBlock::AddSeriesSelectLanguageProfile => self
        .app
        .data
        .sonarr_data
        .add_series_modal
        .as_mut()
        .unwrap()
        .language_profile_list
        .scroll_up(),
      ActiveSonarrBlock::AddSeriesSelectRootFolder => self
        .app
        .data
        .sonarr_data
        .add_series_modal
        .as_mut()
        .unwrap()
        .root_folder_list
        .scroll_up(),
      ActiveSonarrBlock::AddSeriesPrompt => self.app.data.sonarr_data.selected_block.up(),
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::AddSeriesSelectMonitor => self
        .app
        .data
        .sonarr_data
        .add_series_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .scroll_down(),
      ActiveSonarrBlock::AddSeriesSelectSeriesType => self
        .app
        .data
        .sonarr_data
        .add_series_modal
        .as_mut()
        .unwrap()
        .series_type_list
        .scroll_down(),
      ActiveSonarrBlock::AddSeriesSelectQualityProfile => self
        .app
        .data
        .sonarr_data
        .add_series_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_down(),
      ActiveSonarrBlock::AddSeriesSelectLanguageProfile => self
        .app
        .data
        .sonarr_data
        .add_series_modal
        .as_mut()
        .unwrap()
        .language_profile_list
        .scroll_down(),
      ActiveSonarrBlock::AddSeriesSelectRootFolder => self
        .app
        .data
        .sonarr_data
        .add_series_modal
        .as_mut()
        .unwrap()
        .root_folder_list
        .scroll_down(),
      ActiveSonarrBlock::AddSeriesPrompt => self.app.data.sonarr_data.selected_block.down(),
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::AddSeriesSelectMonitor => self
        .app
        .data
        .sonarr_data
        .add_series_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .scroll_to_top(),
      ActiveSonarrBlock::AddSeriesSelectSeriesType => self
        .app
        .data
        .sonarr_data
        .add_series_modal
        .as_mut()
        .unwrap()
        .series_type_list
        .scroll_to_top(),
      ActiveSonarrBlock::AddSeriesSelectQualityProfile => self
        .app
        .data
        .sonarr_data
        .add_series_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_to_top(),
      ActiveSonarrBlock::AddSeriesSelectLanguageProfile => self
        .app
        .data
        .sonarr_data
        .add_series_modal
        .as_mut()
        .unwrap()
        .language_profile_list
        .scroll_to_top(),
      ActiveSonarrBlock::AddSeriesSelectRootFolder => self
        .app
        .data
        .sonarr_data
        .add_series_modal
        .as_mut()
        .unwrap()
        .root_folder_list
        .scroll_to_top(),
      ActiveSonarrBlock::AddSeriesSearchInput => self
        .app
        .data
        .sonarr_data
        .add_series_search
        .as_mut()
        .unwrap()
        .scroll_home(),
      ActiveSonarrBlock::AddSeriesTagsInput => self
        .app
        .data
        .sonarr_data
        .add_series_modal
        .as_mut()
        .unwrap()
        .tags
        .scroll_home(),
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::AddSeriesSelectMonitor => self
        .app
        .data
        .sonarr_data
        .add_series_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .scroll_to_bottom(),
      ActiveSonarrBlock::AddSeriesSelectSeriesType => self
        .app
        .data
        .sonarr_data
        .add_series_modal
        .as_mut()
        .unwrap()
        .series_type_list
        .scroll_to_bottom(),
      ActiveSonarrBlock::AddSeriesSelectQualityProfile => self
        .app
        .data
        .sonarr_data
        .add_series_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_to_bottom(),
      ActiveSonarrBlock::AddSeriesSelectLanguageProfile => self
        .app
        .data
        .sonarr_data
        .add_series_modal
        .as_mut()
        .unwrap()
        .language_profile_list
        .scroll_to_bottom(),
      ActiveSonarrBlock::AddSeriesSelectRootFolder => self
        .app
        .data
        .sonarr_data
        .add_series_modal
        .as_mut()
        .unwrap()
        .root_folder_list
        .scroll_to_bottom(),
      ActiveSonarrBlock::AddSeriesSearchInput => self
        .app
        .data
        .sonarr_data
        .add_series_search
        .as_mut()
        .unwrap()
        .reset_offset(),
      ActiveSonarrBlock::AddSeriesTagsInput => self
        .app
        .data
        .sonarr_data
        .add_series_modal
        .as_mut()
        .unwrap()
        .tags
        .reset_offset(),
      _ => (),
    }
  }

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::AddSeriesPrompt => handle_prompt_toggle(self.app, self.key),
      ActiveSonarrBlock::AddSeriesSearchInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .sonarr_data
            .add_series_search
            .as_mut()
            .unwrap()
        )
      }
      ActiveSonarrBlock::AddSeriesTagsInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .sonarr_data
            .add_series_modal
            .as_mut()
            .unwrap()
            .tags
        )
      }
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_sonarr_block {
      _ if self.active_sonarr_block == ActiveSonarrBlock::AddSeriesSearchInput
        && !self
          .app
          .data
          .sonarr_data
          .add_series_search
          .as_mut()
          .unwrap()
          .text
          .is_empty() =>
      {
        self
          .app
          .push_navigation_stack(ActiveSonarrBlock::AddSeriesSearchResults.into());
        self.app.should_ignore_quit_key = false;
      }
      _ if self.active_sonarr_block == ActiveSonarrBlock::AddSeriesSearchResults
        && self.app.data.sonarr_data.add_searched_series.is_some() =>
      {
        let tvdb_id = self
          .app
          .data
          .sonarr_data
          .add_searched_series
          .as_ref()
          .unwrap()
          .current_selection()
          .tvdb_id;

        if self
          .app
          .data
          .sonarr_data
          .series
          .items
          .iter()
          .any(|series| series.tvdb_id == tvdb_id)
        {
          self
            .app
            .push_navigation_stack(ActiveSonarrBlock::AddSeriesAlreadyInLibrary.into());
        } else {
          self
            .app
            .push_navigation_stack(ActiveSonarrBlock::AddSeriesPrompt.into());
          self.app.data.sonarr_data.add_series_modal = Some((&self.app.data.sonarr_data).into());
          self.app.data.sonarr_data.selected_block =
            BlockSelectionState::new(ADD_SERIES_SELECTION_BLOCKS);
        }
      }
      ActiveSonarrBlock::AddSeriesPrompt => {
        match self.app.data.sonarr_data.selected_block.get_active_block() {
          ActiveSonarrBlock::AddSeriesConfirmPrompt => {
            if self.app.data.sonarr_data.prompt_confirm {
              self.app.data.sonarr_data.prompt_confirm_action = Some(SonarrEvent::AddSeries(None));
            }

            self.app.pop_navigation_stack();
          }
          ActiveSonarrBlock::AddSeriesSelectMonitor
          | ActiveSonarrBlock::AddSeriesSelectSeriesType
          | ActiveSonarrBlock::AddSeriesSelectQualityProfile
          | ActiveSonarrBlock::AddSeriesSelectLanguageProfile
          | ActiveSonarrBlock::AddSeriesSelectRootFolder => self.app.push_navigation_stack(
            self
              .app
              .data
              .sonarr_data
              .selected_block
              .get_active_block()
              .into(),
          ),
          ActiveSonarrBlock::AddSeriesTagsInput => {
            self.app.push_navigation_stack(
              self
                .app
                .data
                .sonarr_data
                .selected_block
                .get_active_block()
                .into(),
            );
            self.app.should_ignore_quit_key = true;
          }
          ActiveSonarrBlock::AddSeriesToggleUseSeasonFolder => {
            self
              .app
              .data
              .sonarr_data
              .add_series_modal
              .as_mut()
              .unwrap()
              .use_season_folder = !self
              .app
              .data
              .sonarr_data
              .add_series_modal
              .as_mut()
              .unwrap()
              .use_season_folder;
          }
          _ => (),
        }
      }
      ActiveSonarrBlock::AddSeriesSelectMonitor
      | ActiveSonarrBlock::AddSeriesSelectSeriesType
      | ActiveSonarrBlock::AddSeriesSelectQualityProfile
      | ActiveSonarrBlock::AddSeriesSelectLanguageProfile
      | ActiveSonarrBlock::AddSeriesSelectRootFolder => self.app.pop_navigation_stack(),
      ActiveSonarrBlock::AddSeriesTagsInput => {
        self.app.pop_navigation_stack();
        self.app.should_ignore_quit_key = false;
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::AddSeriesSearchInput => {
        self.app.pop_navigation_stack();
        self.app.data.sonarr_data.add_series_search = None;
        self.app.should_ignore_quit_key = false;
      }
      ActiveSonarrBlock::AddSeriesSearchResults
      | ActiveSonarrBlock::AddSeriesEmptySearchResults => {
        self.app.pop_navigation_stack();
        self.app.data.sonarr_data.add_searched_series = None;
        self.app.should_ignore_quit_key = true;
      }
      ActiveSonarrBlock::AddSeriesPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.sonarr_data.add_series_modal = None;
        self.app.data.sonarr_data.prompt_confirm = false;
      }
      ActiveSonarrBlock::AddSeriesSelectMonitor
      | ActiveSonarrBlock::AddSeriesSelectSeriesType
      | ActiveSonarrBlock::AddSeriesSelectQualityProfile
      | ActiveSonarrBlock::AddSeriesSelectLanguageProfile
      | ActiveSonarrBlock::AddSeriesAlreadyInLibrary
      | ActiveSonarrBlock::AddSeriesSelectRootFolder => self.app.pop_navigation_stack(),
      ActiveSonarrBlock::AddSeriesTagsInput => {
        self.app.pop_navigation_stack();
        self.app.should_ignore_quit_key = false;
      }
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_sonarr_block {
      ActiveSonarrBlock::AddSeriesSearchInput => {
        handle_text_box_keys!(
          self,
          key,
          self
            .app
            .data
            .sonarr_data
            .add_series_search
            .as_mut()
            .unwrap()
        )
      }
      ActiveSonarrBlock::AddSeriesTagsInput => {
        handle_text_box_keys!(
          self,
          key,
          self
            .app
            .data
            .sonarr_data
            .add_series_modal
            .as_mut()
            .unwrap()
            .tags
        )
      }
      ActiveSonarrBlock::AddSeriesPrompt => {
        if self.app.data.sonarr_data.selected_block.get_active_block()
          == ActiveSonarrBlock::AddSeriesConfirmPrompt
          && key == DEFAULT_KEYBINDINGS.confirm.key
        {
          self.app.data.sonarr_data.prompt_confirm = true;
          self.app.data.sonarr_data.prompt_confirm_action = Some(SonarrEvent::AddSeries(None));
          self.app.pop_navigation_stack();
        }
      }
      _ => (),
    }
  }
}
