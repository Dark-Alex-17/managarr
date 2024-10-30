use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::App;
use crate::event::Key;
use crate::handlers::{handle_prompt_toggle, KeyEventHandler};
use crate::models::servarr_data::radarr::radarr_data::{
  ActiveRadarrBlock, INDEXER_SETTINGS_BLOCKS,
};
use crate::network::radarr_network::RadarrEvent;
use crate::{handle_text_box_keys, handle_text_box_left_right_keys};

#[cfg(test)]
#[path = "edit_indexer_settings_handler_tests.rs"]
mod edit_indexer_settings_handler_tests;

pub(super) struct IndexerSettingsHandler<'a, 'b> {
  key: &'a Key,
  app: &'a mut App<'b>,
  active_radarr_block: &'a ActiveRadarrBlock,
  _context: &'a Option<ActiveRadarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for IndexerSettingsHandler<'a, 'b> {
  fn accepts(active_block: &'a ActiveRadarrBlock) -> bool {
    INDEXER_SETTINGS_BLOCKS.contains(active_block)
  }

  fn with(
    key: &'a Key,
    app: &'a mut App<'b>,
    active_block: &'a ActiveRadarrBlock,
    _context: &'a Option<ActiveRadarrBlock>,
  ) -> IndexerSettingsHandler<'a, 'b> {
    IndexerSettingsHandler {
      key,
      app,
      active_radarr_block: active_block,
      _context,
    }
  }

  fn get_key(&self) -> &Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    !self.app.is_loading && self.app.data.radarr_data.indexer_settings.is_some()
  }

  fn handle_scroll_up(&mut self) {
    let indexer_settings = self.app.data.radarr_data.indexer_settings.as_mut().unwrap();
    match self.active_radarr_block {
      ActiveRadarrBlock::AllIndexerSettingsPrompt => {
        self.app.data.radarr_data.selected_block.previous();
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
        self.app.data.radarr_data.selected_block.next()
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
    if self.active_radarr_block == &ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput {
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
    if self.active_radarr_block == &ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput {
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
        if self.app.data.radarr_data.selected_block.get_active_block()
          == &ActiveRadarrBlock::IndexerSettingsConfirmPrompt
        {
          handle_prompt_toggle(self.app, self.key);
        } else {
          let len = self.app.data.radarr_data.selected_block.blocks.len();
          let idx = self.app.data.radarr_data.selected_block.index;
          self.app.data.radarr_data.selected_block.index = (idx + 5) % len;
        }
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
            let radarr_data = &mut self.app.data.radarr_data;
            if radarr_data.prompt_confirm {
              radarr_data.prompt_confirm_action = Some(RadarrEvent::EditAllIndexerSettings(None));
              self.app.should_refresh = true;
            } else {
              radarr_data.indexer_settings = None;
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
                *self.app.data.radarr_data.selected_block.get_active_block(),
                None,
              )
                .into(),
            )
          }
          ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput => {
            self.app.push_navigation_stack(
              ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput.into(),
            );
            self.app.should_ignore_quit_key = true;
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
        self.app.should_ignore_quit_key = false;
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
        self.app.should_ignore_quit_key = false;
      }
      _ => self.app.pop_navigation_stack(),
    }
  }

  fn handle_char_key_event(&mut self) {
    if self.active_radarr_block == &ActiveRadarrBlock::IndexerSettingsWhitelistedSubtitleTagsInput {
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
  }
}
