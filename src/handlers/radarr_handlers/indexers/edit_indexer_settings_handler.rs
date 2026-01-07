use crate::app::App;
use crate::event::Key;
use crate::handlers::{KeyEventHandler, handle_prompt_toggle};
use crate::models::radarr_models::IndexerSettings;
use crate::models::servarr_data::radarr::radarr_data::{
  ActiveRadarrBlock, INDEXER_SETTINGS_BLOCKS,
};
use crate::network::radarr_network::RadarrEvent;
use crate::{
  handle_prompt_left_right_keys, handle_text_box_keys, handle_text_box_left_right_keys, matches_key,
};
use crate::models::Route;

#[cfg(test)]
#[path = "edit_indexer_settings_handler_tests.rs"]
mod edit_indexer_settings_handler_tests;

pub(super) struct IndexerSettingsHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_radarr_block: ActiveRadarrBlock,
  _context: Option<ActiveRadarrBlock>,
}

impl IndexerSettingsHandler<'_, '_> {
  fn build_edit_indexer_settings_body(&mut self) -> IndexerSettings {
    self
      .app
      .data
      .radarr_data
      .indexer_settings
      .take()
      .expect("Indexer settings not found")
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for IndexerSettingsHandler<'a, 'b> {
  fn accepts(active_block: ActiveRadarrBlock) -> bool {
    INDEXER_SETTINGS_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveRadarrBlock,
    _context: Option<ActiveRadarrBlock>,
  ) -> IndexerSettingsHandler<'a, 'b> {
    IndexerSettingsHandler {
      key,
      app,
      active_radarr_block: active_block,
      _context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    !self.app.is_loading && self.app.data.radarr_data.indexer_settings.is_some()
  }

  fn handle_scroll_up(&mut self) {
    let indexer_settings = self.app.data.radarr_data.indexer_settings.as_mut().unwrap();
    match self.active_radarr_block {
      ActiveRadarrBlock::AllIndexerSettingsPrompt => {
        self.app.data.radarr_data.selected_block.up();
      }
      ActiveRadarrBlock::IndexerSettingsMinimumAgeInput => {
        indexer_settings.minimum_age += 1;
      }
      ActiveRadarrBlock::IndexerSettingsRetentionInput => {
        indexer_settings.retention += 1;
      }
      ActiveRadarrBlock::IndexerSettingsMaximumSizeInput => {
        indexer_settings.maximum_size += 1;
      }
      ActiveRadarrBlock::IndexerSettingsAvailabilityDelayInput => {
        indexer_settings.availability_delay += 1;
      }
      ActiveRadarrBlock::IndexerSettingsRssSyncIntervalInput => {
        indexer_settings.rss_sync_interval += 1;
      }
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    let indexer_settings = self.app.data.radarr_data.indexer_settings.as_mut().unwrap();
    match self.active_radarr_block {
      ActiveRadarrBlock::AllIndexerSettingsPrompt => {
        self.app.data.radarr_data.selected_block.down()
      }
      ActiveRadarrBlock::IndexerSettingsMinimumAgeInput => {
        if indexer_settings.minimum_age > 0 {
          indexer_settings.minimum_age -= 1;
        }
      }
      ActiveRadarrBlock::IndexerSettingsRetentionInput => {
        if indexer_settings.retention > 0 {
          indexer_settings.retention -= 1;
        }
      }
      ActiveRadarrBlock::IndexerSettingsMaximumSizeInput => {
        if indexer_settings.maximum_size > 0 {
          indexer_settings.maximum_size -= 1;
        }
      }
      ActiveRadarrBlock::IndexerSettingsAvailabilityDelayInput => {
        indexer_settings.availability_delay -= 1;
      }
      ActiveRadarrBlock::IndexerSettingsRssSyncIntervalInput => {
        if indexer_settings.rss_sync_interval > 0 {
          indexer_settings.rss_sync_interval -= 1;
        }
      }
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    if self.active_radarr_block == ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput {
      self
        .app
        .data
        .radarr_data
        .indexer_settings
        .as_mut()
        .unwrap()
        .whitelisted_hardcoded_subs
        .scroll_home();
    }
  }

  fn handle_end(&mut self) {
    if self.active_radarr_block == ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput {
      self
        .app
        .data
        .radarr_data
        .indexer_settings
        .as_mut()
        .unwrap()
        .whitelisted_hardcoded_subs
        .reset_offset();
    }
  }

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::AllIndexerSettingsPrompt => {
        handle_prompt_left_right_keys!(
          self,
          ActiveRadarrBlock::IndexerSettingsConfirmPrompt,
          radarr_data
        );
      }
      ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .radarr_data
            .indexer_settings
            .as_mut()
            .unwrap()
            .whitelisted_hardcoded_subs
        )
      }
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::AllIndexerSettingsPrompt => {
        match self.app.data.radarr_data.selected_block.get_active_block() {
          ActiveRadarrBlock::IndexerSettingsConfirmPrompt => {
            if self.app.data.radarr_data.prompt_confirm {
              self.app.data.radarr_data.prompt_confirm_action = Some(
                RadarrEvent::EditAllIndexerSettings(self.build_edit_indexer_settings_body()),
              );
              self.app.should_refresh = true;
            } else {
              self.app.data.radarr_data.indexer_settings = None;
            }

            self.app.pop_navigation_stack();
          }
          ActiveRadarrBlock::IndexerSettingsMinimumAgeInput
          | ActiveRadarrBlock::IndexerSettingsRetentionInput
          | ActiveRadarrBlock::IndexerSettingsMaximumSizeInput
          | ActiveRadarrBlock::IndexerSettingsAvailabilityDelayInput
          | ActiveRadarrBlock::IndexerSettingsRssSyncIntervalInput => {
            self.app.push_navigation_stack(
              (
                self.app.data.radarr_data.selected_block.get_active_block(),
                None,
              )
                .into(),
            )
          }
          ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput => {
            self.app.push_navigation_stack(
              ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput.into(),
            );
            self.app.ignore_special_keys_for_textbox_input = true;
          }
          ActiveRadarrBlock::IndexerSettingsTogglePreferIndexerFlags => {
            let indexer_settings = self.app.data.radarr_data.indexer_settings.as_mut().unwrap();

            indexer_settings.prefer_indexer_flags = !indexer_settings.prefer_indexer_flags;
          }
          ActiveRadarrBlock::IndexerSettingsToggleAllowHardcodedSubs => {
            let indexer_settings = self.app.data.radarr_data.indexer_settings.as_mut().unwrap();

            indexer_settings.allow_hardcoded_subs = !indexer_settings.allow_hardcoded_subs;
          }
          _ => (),
        }
      }
      ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput => {
        self.app.pop_navigation_stack();
        self.app.ignore_special_keys_for_textbox_input = false;
      }
      ActiveRadarrBlock::IndexerSettingsMinimumAgeInput
      | ActiveRadarrBlock::IndexerSettingsRetentionInput
      | ActiveRadarrBlock::IndexerSettingsMaximumSizeInput
      | ActiveRadarrBlock::IndexerSettingsAvailabilityDelayInput
      | ActiveRadarrBlock::IndexerSettingsRssSyncIntervalInput => self.app.pop_navigation_stack(),
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::AllIndexerSettingsPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.prompt_confirm = false;
        self.app.data.radarr_data.indexer_settings = None;
      }
      ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput => {
        self.app.pop_navigation_stack();
        self.app.ignore_special_keys_for_textbox_input = false;
      }
      _ => self.app.pop_navigation_stack(),
    }
  }

  fn handle_char_key_event(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput => {
        handle_text_box_keys!(
          self,
          self.key,
          self
            .app
            .data
            .radarr_data
            .indexer_settings
            .as_mut()
            .unwrap()
            .whitelisted_hardcoded_subs
        )
      }
      ActiveRadarrBlock::AllIndexerSettingsPrompt => {
        if self.app.data.radarr_data.selected_block.get_active_block()
          == ActiveRadarrBlock::IndexerSettingsConfirmPrompt
          && matches_key!(confirm, self.key)
        {
          self.app.data.radarr_data.prompt_confirm = true;
          self.app.data.radarr_data.prompt_confirm_action = Some(
            RadarrEvent::EditAllIndexerSettings(self.build_edit_indexer_settings_body()),
          );
          self.app.should_refresh = true;

          self.app.pop_navigation_stack();
        }
      }
      _ => (),
    }
  }

  fn app_mut(&mut self) -> &mut App<'b> {
    self.app
  }

  fn current_route(&self) -> Route {
    self.app.get_current_route()
  }
}
