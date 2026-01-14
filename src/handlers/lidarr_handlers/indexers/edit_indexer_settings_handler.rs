use crate::app::App;
use crate::event::Key;
use crate::handlers::{KeyEventHandler, handle_prompt_toggle};
use crate::models::Route;
use crate::models::servarr_data::lidarr::lidarr_data::{
  ActiveLidarrBlock, INDEXER_SETTINGS_BLOCKS,
};
use crate::models::servarr_models::IndexerSettings;
use crate::network::lidarr_network::LidarrEvent;
use crate::{handle_prompt_left_right_keys, matches_key};

#[cfg(test)]
#[path = "edit_indexer_settings_handler_tests.rs"]
mod edit_indexer_settings_handler_tests;

pub(super) struct IndexerSettingsHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_lidarr_block: ActiveLidarrBlock,
  _context: Option<ActiveLidarrBlock>,
}

impl IndexerSettingsHandler<'_, '_> {
  fn build_edit_indexer_settings_params(&mut self) -> IndexerSettings {
    self
      .app
      .data
      .lidarr_data
      .indexer_settings
      .take()
      .expect("IndexerSettings is None")
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveLidarrBlock> for IndexerSettingsHandler<'a, 'b> {
  fn accepts(active_block: ActiveLidarrBlock) -> bool {
    INDEXER_SETTINGS_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveLidarrBlock,
    _context: Option<ActiveLidarrBlock>,
  ) -> IndexerSettingsHandler<'a, 'b> {
    IndexerSettingsHandler {
      key,
      app,
      active_lidarr_block: active_block,
      _context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    !self.app.is_loading && self.app.data.lidarr_data.indexer_settings.is_some()
  }

  fn handle_scroll_up(&mut self) {
    let indexer_settings = self.app.data.lidarr_data.indexer_settings.as_mut().unwrap();
    match self.active_lidarr_block {
      ActiveLidarrBlock::AllIndexerSettingsPrompt => {
        self.app.data.lidarr_data.selected_block.up();
      }
      ActiveLidarrBlock::IndexerSettingsMinimumAgeInput => {
        indexer_settings.minimum_age += 1;
      }
      ActiveLidarrBlock::IndexerSettingsRetentionInput => {
        indexer_settings.retention += 1;
      }
      ActiveLidarrBlock::IndexerSettingsMaximumSizeInput => {
        indexer_settings.maximum_size += 1;
      }
      ActiveLidarrBlock::IndexerSettingsRssSyncIntervalInput => {
        indexer_settings.rss_sync_interval += 1;
      }
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    let indexer_settings = self.app.data.lidarr_data.indexer_settings.as_mut().unwrap();
    match self.active_lidarr_block {
      ActiveLidarrBlock::AllIndexerSettingsPrompt => {
        self.app.data.lidarr_data.selected_block.down()
      }
      ActiveLidarrBlock::IndexerSettingsMinimumAgeInput => {
        if indexer_settings.minimum_age > 0 {
          indexer_settings.minimum_age -= 1;
        }
      }
      ActiveLidarrBlock::IndexerSettingsRetentionInput => {
        if indexer_settings.retention > 0 {
          indexer_settings.retention -= 1;
        }
      }
      ActiveLidarrBlock::IndexerSettingsMaximumSizeInput => {
        if indexer_settings.maximum_size > 0 {
          indexer_settings.maximum_size -= 1;
        }
      }
      ActiveLidarrBlock::IndexerSettingsRssSyncIntervalInput => {
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
    if self.active_lidarr_block == ActiveLidarrBlock::AllIndexerSettingsPrompt {
      handle_prompt_left_right_keys!(
        self,
        ActiveLidarrBlock::IndexerSettingsConfirmPrompt,
        lidarr_data
      );
    }
  }

  fn handle_submit(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::AllIndexerSettingsPrompt => {
        match self.app.data.lidarr_data.selected_block.get_active_block() {
          ActiveLidarrBlock::IndexerSettingsConfirmPrompt => {
            if self.app.data.lidarr_data.prompt_confirm {
              self.app.data.lidarr_data.prompt_confirm_action = Some(
                LidarrEvent::EditAllIndexerSettings(self.build_edit_indexer_settings_params()),
              );
              self.app.should_refresh = true;
            } else {
              self.app.data.lidarr_data.indexer_settings = None;
            }

            self.app.pop_navigation_stack();
          }
          ActiveLidarrBlock::IndexerSettingsMinimumAgeInput
          | ActiveLidarrBlock::IndexerSettingsRetentionInput
          | ActiveLidarrBlock::IndexerSettingsMaximumSizeInput
          | ActiveLidarrBlock::IndexerSettingsRssSyncIntervalInput => {
            self.app.push_navigation_stack(
              (
                self.app.data.lidarr_data.selected_block.get_active_block(),
                None,
              )
                .into(),
            )
          }

          _ => (),
        }
      }

      ActiveLidarrBlock::IndexerSettingsMinimumAgeInput
      | ActiveLidarrBlock::IndexerSettingsRetentionInput
      | ActiveLidarrBlock::IndexerSettingsMaximumSizeInput
      | ActiveLidarrBlock::IndexerSettingsRssSyncIntervalInput => self.app.pop_navigation_stack(),
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::AllIndexerSettingsPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.lidarr_data.prompt_confirm = false;
        self.app.data.lidarr_data.indexer_settings = None;
      }
      _ => self.app.pop_navigation_stack(),
    }
  }

  fn handle_char_key_event(&mut self) {
    if self.active_lidarr_block == ActiveLidarrBlock::AllIndexerSettingsPrompt
      && self.app.data.lidarr_data.selected_block.get_active_block()
        == ActiveLidarrBlock::IndexerSettingsConfirmPrompt
      && matches_key!(confirm, self.key)
    {
      self.app.data.lidarr_data.prompt_confirm = true;
      self.app.data.lidarr_data.prompt_confirm_action = Some(LidarrEvent::EditAllIndexerSettings(
        self.build_edit_indexer_settings_params(),
      ));
      self.app.should_refresh = true;

      self.app.pop_navigation_stack();
    }
  }

  fn app_mut(&mut self) -> &mut App<'b> {
    self.app
  }

  fn current_route(&self) -> Route {
    self.app.get_current_route()
  }
}
