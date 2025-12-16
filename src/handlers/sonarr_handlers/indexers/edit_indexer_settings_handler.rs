use crate::app::App;
use crate::event::Key;
use crate::handlers::{KeyEventHandler, handle_prompt_toggle};
use crate::models::servarr_data::sonarr::sonarr_data::{
  ActiveSonarrBlock, INDEXER_SETTINGS_BLOCKS,
};
use crate::models::sonarr_models::IndexerSettings;
use crate::network::sonarr_network::SonarrEvent;
use crate::{handle_prompt_left_right_keys, matches_key};

#[cfg(test)]
#[path = "edit_indexer_settings_handler_tests.rs"]
mod edit_indexer_settings_handler_tests;

pub(super) struct IndexerSettingsHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_sonarr_block: ActiveSonarrBlock,
  _context: Option<ActiveSonarrBlock>,
}

impl IndexerSettingsHandler<'_, '_> {
  fn build_edit_indexer_settings_params(&mut self) -> IndexerSettings {
    self
      .app
      .data
      .sonarr_data
      .indexer_settings
      .take()
      .expect("IndexerSettings is None")
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveSonarrBlock> for IndexerSettingsHandler<'a, 'b> {
  fn accepts(active_block: ActiveSonarrBlock) -> bool {
    INDEXER_SETTINGS_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveSonarrBlock,
    _context: Option<ActiveSonarrBlock>,
  ) -> IndexerSettingsHandler<'a, 'b> {
    IndexerSettingsHandler {
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
    !self.app.is_loading && self.app.data.sonarr_data.indexer_settings.is_some()
  }

  fn handle_scroll_up(&mut self) {
    let indexer_settings = self.app.data.sonarr_data.indexer_settings.as_mut().unwrap();
    match self.active_sonarr_block {
      ActiveSonarrBlock::AllIndexerSettingsPrompt => {
        self.app.data.sonarr_data.selected_block.up();
      }
      ActiveSonarrBlock::IndexerSettingsMinimumAgeInput => {
        indexer_settings.minimum_age += 1;
      }
      ActiveSonarrBlock::IndexerSettingsRetentionInput => {
        indexer_settings.retention += 1;
      }
      ActiveSonarrBlock::IndexerSettingsMaximumSizeInput => {
        indexer_settings.maximum_size += 1;
      }
      ActiveSonarrBlock::IndexerSettingsRssSyncIntervalInput => {
        indexer_settings.rss_sync_interval += 1;
      }
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    let indexer_settings = self.app.data.sonarr_data.indexer_settings.as_mut().unwrap();
    match self.active_sonarr_block {
      ActiveSonarrBlock::AllIndexerSettingsPrompt => {
        self.app.data.sonarr_data.selected_block.down()
      }
      ActiveSonarrBlock::IndexerSettingsMinimumAgeInput => {
        if indexer_settings.minimum_age > 0 {
          indexer_settings.minimum_age -= 1;
        }
      }
      ActiveSonarrBlock::IndexerSettingsRetentionInput => {
        if indexer_settings.retention > 0 {
          indexer_settings.retention -= 1;
        }
      }
      ActiveSonarrBlock::IndexerSettingsMaximumSizeInput => {
        if indexer_settings.maximum_size > 0 {
          indexer_settings.maximum_size -= 1;
        }
      }
      ActiveSonarrBlock::IndexerSettingsRssSyncIntervalInput => {
        if indexer_settings.rss_sync_interval > 0 {
          indexer_settings.rss_sync_interval -= 1;
        }
      }
      _ => (),
    }
  }

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    if self.active_sonarr_block == ActiveSonarrBlock::AllIndexerSettingsPrompt {
      handle_prompt_left_right_keys!(
        self,
        ActiveSonarrBlock::IndexerSettingsConfirmPrompt,
        sonarr_data
      );
    }
  }

  fn handle_submit(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::AllIndexerSettingsPrompt => {
        match self.app.data.sonarr_data.selected_block.get_active_block() {
          ActiveSonarrBlock::IndexerSettingsConfirmPrompt => {
            if self.app.data.sonarr_data.prompt_confirm {
              self.app.data.sonarr_data.prompt_confirm_action = Some(
                SonarrEvent::EditAllIndexerSettings(self.build_edit_indexer_settings_params()),
              );
              self.app.should_refresh = true;
            } else {
              self.app.data.sonarr_data.indexer_settings = None;
            }

            self.app.pop_navigation_stack();
          }
          ActiveSonarrBlock::IndexerSettingsMinimumAgeInput
          | ActiveSonarrBlock::IndexerSettingsRetentionInput
          | ActiveSonarrBlock::IndexerSettingsMaximumSizeInput
          | ActiveSonarrBlock::IndexerSettingsRssSyncIntervalInput => {
            self.app.push_navigation_stack(
              (
                self.app.data.sonarr_data.selected_block.get_active_block(),
                None,
              )
                .into(),
            )
          }

          _ => (),
        }
      }

      ActiveSonarrBlock::IndexerSettingsMinimumAgeInput
      | ActiveSonarrBlock::IndexerSettingsRetentionInput
      | ActiveSonarrBlock::IndexerSettingsMaximumSizeInput
      | ActiveSonarrBlock::IndexerSettingsRssSyncIntervalInput => self.app.pop_navigation_stack(),
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::AllIndexerSettingsPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.sonarr_data.prompt_confirm = false;
        self.app.data.sonarr_data.indexer_settings = None;
      }
      _ => self.app.pop_navigation_stack(),
    }
  }

  fn handle_char_key_event(&mut self) {
    if self.active_sonarr_block == ActiveSonarrBlock::AllIndexerSettingsPrompt
      && self.app.data.sonarr_data.selected_block.get_active_block()
        == ActiveSonarrBlock::IndexerSettingsConfirmPrompt
      && matches_key!(confirm, self.key)
    {
      self.app.data.sonarr_data.prompt_confirm = true;
      self.app.data.sonarr_data.prompt_confirm_action = Some(SonarrEvent::EditAllIndexerSettings(
        self.build_edit_indexer_settings_params(),
      ));
      self.app.should_refresh = true;

      self.app.pop_navigation_stack();
    }
  }

  fn app_mut(&mut self) -> &mut App<'b> {
    self.app
  }

  fn current_route(&self) -> crate::models::Route {
    self.app.get_current_route()
  }
}
