use crate::models::sonarr_models::DeleteSeriesParams;
use crate::network::sonarr_network::SonarrEvent;
use crate::{
  app::{key_binding::DEFAULT_KEYBINDINGS, App},
  event::Key,
  handlers::{handle_prompt_toggle, KeyEventHandler},
  models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, DELETE_SERIES_BLOCKS},
};

#[cfg(test)]
#[path = "delete_series_handler_tests.rs"]
mod delete_series_handler_tests;

pub(super) struct DeleteSeriesHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_sonarr_block: ActiveSonarrBlock,
  _context: Option<ActiveSonarrBlock>,
}

impl DeleteSeriesHandler<'_, '_> {
  fn build_delete_series_params(&mut self) -> DeleteSeriesParams {
    let id = self.app.data.sonarr_data.series.current_selection().id;
    let delete_series_files = self.app.data.sonarr_data.delete_series_files;
    let add_list_exclusion = self.app.data.sonarr_data.add_list_exclusion;
    self.app.data.sonarr_data.reset_delete_series_preferences();

    DeleteSeriesParams {
      id,
      delete_series_files,
      add_list_exclusion,
    }
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveSonarrBlock> for DeleteSeriesHandler<'a, 'b> {
  fn accepts(active_block: ActiveSonarrBlock) -> bool {
    DELETE_SERIES_BLOCKS.contains(&active_block)
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveSonarrBlock,
    _context: Option<ActiveSonarrBlock>,
  ) -> Self {
    DeleteSeriesHandler {
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
    if self.active_sonarr_block == ActiveSonarrBlock::DeleteSeriesPrompt {
      self.app.data.sonarr_data.selected_block.up();
    }
  }

  fn handle_scroll_down(&mut self) {
    if self.active_sonarr_block == ActiveSonarrBlock::DeleteSeriesPrompt {
      self.app.data.sonarr_data.selected_block.down();
    }
  }

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    if self.active_sonarr_block == ActiveSonarrBlock::DeleteSeriesPrompt {
      handle_prompt_toggle(self.app, self.key);
    }
  }

  fn handle_submit(&mut self) {
    if self.active_sonarr_block == ActiveSonarrBlock::DeleteSeriesPrompt {
      match self.app.data.sonarr_data.selected_block.get_active_block() {
        ActiveSonarrBlock::DeleteSeriesConfirmPrompt => {
          if self.app.data.sonarr_data.prompt_confirm {
            self.app.data.sonarr_data.prompt_confirm_action =
              Some(SonarrEvent::DeleteSeries(self.build_delete_series_params()));
            self.app.should_refresh = true;
          } else {
            self.app.data.sonarr_data.reset_delete_series_preferences();
          }

          self.app.pop_navigation_stack();
        }
        ActiveSonarrBlock::DeleteSeriesToggleDeleteFile => {
          self.app.data.sonarr_data.delete_series_files =
            !self.app.data.sonarr_data.delete_series_files;
        }
        ActiveSonarrBlock::DeleteSeriesToggleAddListExclusion => {
          self.app.data.sonarr_data.add_list_exclusion =
            !self.app.data.sonarr_data.add_list_exclusion;
        }
        _ => (),
      }
    }
  }

  fn handle_esc(&mut self) {
    if self.active_sonarr_block == ActiveSonarrBlock::DeleteSeriesPrompt {
      self.app.pop_navigation_stack();
      self.app.data.sonarr_data.reset_delete_series_preferences();
      self.app.data.sonarr_data.prompt_confirm = false;
    }
  }

  fn handle_char_key_event(&mut self) {
    if self.active_sonarr_block == ActiveSonarrBlock::DeleteSeriesPrompt
      && self.app.data.sonarr_data.selected_block.get_active_block()
        == ActiveSonarrBlock::DeleteSeriesConfirmPrompt
      && self.key == DEFAULT_KEYBINDINGS.confirm.key
    {
      self.app.data.sonarr_data.prompt_confirm = true;
      self.app.data.sonarr_data.prompt_confirm_action =
        Some(SonarrEvent::DeleteSeries(self.build_delete_series_params()));
      self.app.should_refresh = true;

      self.app.pop_navigation_stack();
    }
  }
}
