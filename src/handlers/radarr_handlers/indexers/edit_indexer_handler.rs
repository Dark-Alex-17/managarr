use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::App;
use crate::event::Key;
use crate::handlers::{handle_prompt_toggle, KeyEventHandler};
use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, EDIT_INDEXER_BLOCKS};
use crate::network::radarr_network::RadarrEvent;
use crate::{handle_text_box_keys, handle_text_box_left_right_keys};

#[cfg(test)]
#[path = "edit_indexer_handler_tests.rs"]
mod edit_indexer_handler_tests;

pub(super) struct EditIndexerHandler<'a, 'b> {
  key: &'a Key,
  app: &'a mut App<'b>,
  active_radarr_block: &'a ActiveRadarrBlock,
  _context: &'a Option<ActiveRadarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for EditIndexerHandler<'a, 'b> {
  fn accepts(active_block: &'a ActiveRadarrBlock) -> bool {
    EDIT_INDEXER_BLOCKS.contains(active_block)
  }

  fn with(
    key: &'a Key,
    app: &'a mut App<'b>,
    active_block: &'a ActiveRadarrBlock,
    _context: &'a Option<ActiveRadarrBlock>,
  ) -> EditIndexerHandler<'a, 'b> {
    EditIndexerHandler {
      key,
      app,
      active_radarr_block: active_block,
      _context,
    }
  }

  fn get_key(&self) -> &Key {
    self.key
  }

  fn handle_scroll_up(&mut self) {
    if self.active_radarr_block == &ActiveRadarrBlock::EditIndexerPrompt {
      self.app.data.radarr_data.selected_block.previous();
    }
  }

  fn handle_scroll_down(&mut self) {
    if self.active_radarr_block == &ActiveRadarrBlock::EditIndexerPrompt {
      self.app.data.radarr_data.selected_block.next();
    }
  }

  fn handle_home(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::EditIndexerNameInput => {
        self
          .app
          .data
          .radarr_data
          .edit_indexer_modal
          .as_mut()
          .unwrap()
          .name
          .scroll_home();
      }
      ActiveRadarrBlock::EditIndexerUrlInput => {
        self
          .app
          .data
          .radarr_data
          .edit_indexer_modal
          .as_mut()
          .unwrap()
          .url
          .scroll_home();
      }
      ActiveRadarrBlock::EditIndexerApiKeyInput => {
        self
          .app
          .data
          .radarr_data
          .edit_indexer_modal
          .as_mut()
          .unwrap()
          .api_key
          .scroll_home();
      }
      ActiveRadarrBlock::EditIndexerSeedRatioInput => {
        self
          .app
          .data
          .radarr_data
          .edit_indexer_modal
          .as_mut()
          .unwrap()
          .seed_ratio
          .scroll_home();
      }
      ActiveRadarrBlock::EditIndexerTagsInput => {
        self
          .app
          .data
          .radarr_data
          .edit_indexer_modal
          .as_mut()
          .unwrap()
          .tags
          .scroll_home();
      }
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::EditIndexerNameInput => {
        self
          .app
          .data
          .radarr_data
          .edit_indexer_modal
          .as_mut()
          .unwrap()
          .name
          .reset_offset();
      }
      ActiveRadarrBlock::EditIndexerUrlInput => {
        self
          .app
          .data
          .radarr_data
          .edit_indexer_modal
          .as_mut()
          .unwrap()
          .url
          .reset_offset();
      }
      ActiveRadarrBlock::EditIndexerApiKeyInput => {
        self
          .app
          .data
          .radarr_data
          .edit_indexer_modal
          .as_mut()
          .unwrap()
          .api_key
          .reset_offset();
      }
      ActiveRadarrBlock::EditIndexerSeedRatioInput => {
        self
          .app
          .data
          .radarr_data
          .edit_indexer_modal
          .as_mut()
          .unwrap()
          .seed_ratio
          .reset_offset();
      }
      ActiveRadarrBlock::EditIndexerTagsInput => {
        self
          .app
          .data
          .radarr_data
          .edit_indexer_modal
          .as_mut()
          .unwrap()
          .tags
          .reset_offset();
      }
      _ => (),
    }
  }

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::EditIndexerPrompt => {
        if self.app.data.radarr_data.selected_block.get_active_block()
          == &ActiveRadarrBlock::EditIndexerConfirmPrompt
        {
          handle_prompt_toggle(self.app, self.key);
        } else {
          let len = self.app.data.radarr_data.selected_block.blocks.len();
          let idx = self.app.data.radarr_data.selected_block.index;
          self.app.data.radarr_data.selected_block.index = (idx + 5) % len;
        }
      }
      ActiveRadarrBlock::EditIndexerNameInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .radarr_data
            .edit_indexer_modal
            .as_mut()
            .unwrap()
            .name
        );
      }
      ActiveRadarrBlock::EditIndexerUrlInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .radarr_data
            .edit_indexer_modal
            .as_mut()
            .unwrap()
            .url
        );
      }
      ActiveRadarrBlock::EditIndexerApiKeyInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .radarr_data
            .edit_indexer_modal
            .as_mut()
            .unwrap()
            .api_key
        );
      }
      ActiveRadarrBlock::EditIndexerSeedRatioInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .radarr_data
            .edit_indexer_modal
            .as_mut()
            .unwrap()
            .seed_ratio
        );
      }
      ActiveRadarrBlock::EditIndexerTagsInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .radarr_data
            .edit_indexer_modal
            .as_mut()
            .unwrap()
            .tags
        );
      }
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::EditIndexerPrompt => {
        let selected_block = *self.app.data.radarr_data.selected_block.get_active_block();
        match selected_block {
          ActiveRadarrBlock::EditIndexerConfirmPrompt => {
            let radarr_data = &mut self.app.data.radarr_data;
            if radarr_data.prompt_confirm {
              radarr_data.prompt_confirm_action = Some(RadarrEvent::EditIndexer);
              self.app.should_refresh = true;
            } else {
              radarr_data.edit_indexer_modal = None;
            }

            self.app.pop_navigation_stack();
          }
          ActiveRadarrBlock::EditIndexerNameInput
          | ActiveRadarrBlock::EditIndexerUrlInput
          | ActiveRadarrBlock::EditIndexerApiKeyInput
          | ActiveRadarrBlock::EditIndexerSeedRatioInput
          | ActiveRadarrBlock::EditIndexerTagsInput => {
            self.app.push_navigation_stack(selected_block.into());
            self.app.should_ignore_quit_key = true;
          }
          ActiveRadarrBlock::EditIndexerToggleEnableRss => {
            let indexer = self
              .app
              .data
              .radarr_data
              .edit_indexer_modal
              .as_mut()
              .unwrap();
            indexer.enable_rss = Some(!indexer.enable_rss.unwrap_or_default());
          }
          ActiveRadarrBlock::EditIndexerToggleEnableAutomaticSearch => {
            let indexer = self
              .app
              .data
              .radarr_data
              .edit_indexer_modal
              .as_mut()
              .unwrap();
            indexer.enable_automatic_search =
              Some(!indexer.enable_automatic_search.unwrap_or_default());
          }
          ActiveRadarrBlock::EditIndexerToggleEnableInteractiveSearch => {
            let indexer = self
              .app
              .data
              .radarr_data
              .edit_indexer_modal
              .as_mut()
              .unwrap();
            indexer.enable_interactive_search =
              Some(!indexer.enable_interactive_search.unwrap_or_default());
          }
          _ => (),
        }
      }
      ActiveRadarrBlock::EditIndexerNameInput
      | ActiveRadarrBlock::EditIndexerUrlInput
      | ActiveRadarrBlock::EditIndexerApiKeyInput
      | ActiveRadarrBlock::EditIndexerSeedRatioInput
      | ActiveRadarrBlock::EditIndexerTagsInput => {
        self.app.pop_navigation_stack();
        self.app.should_ignore_quit_key = false;
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::EditIndexerPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.prompt_confirm = false;
        self.app.data.radarr_data.edit_indexer_modal = None;
      }
      ActiveRadarrBlock::EditIndexerNameInput
      | ActiveRadarrBlock::EditIndexerUrlInput
      | ActiveRadarrBlock::EditIndexerApiKeyInput
      | ActiveRadarrBlock::EditIndexerSeedRatioInput
      | ActiveRadarrBlock::EditIndexerTagsInput => {
        self.app.pop_navigation_stack();
        self.app.should_ignore_quit_key = false;
      }
      _ => self.app.pop_navigation_stack(),
    }
  }

  fn handle_char_key_event(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::EditIndexerNameInput => {
        handle_text_box_keys!(
          self,
          self.key,
          self
            .app
            .data
            .radarr_data
            .edit_indexer_modal
            .as_mut()
            .unwrap()
            .name
        );
      }
      ActiveRadarrBlock::EditIndexerUrlInput => {
        handle_text_box_keys!(
          self,
          self.key,
          self
            .app
            .data
            .radarr_data
            .edit_indexer_modal
            .as_mut()
            .unwrap()
            .url
        );
      }
      ActiveRadarrBlock::EditIndexerApiKeyInput => {
        handle_text_box_keys!(
          self,
          self.key,
          self
            .app
            .data
            .radarr_data
            .edit_indexer_modal
            .as_mut()
            .unwrap()
            .api_key
        );
      }
      ActiveRadarrBlock::EditIndexerSeedRatioInput => {
        handle_text_box_keys!(
          self,
          self.key,
          self
            .app
            .data
            .radarr_data
            .edit_indexer_modal
            .as_mut()
            .unwrap()
            .seed_ratio
        );
      }
      ActiveRadarrBlock::EditIndexerTagsInput => {
        handle_text_box_keys!(
          self,
          self.key,
          self
            .app
            .data
            .radarr_data
            .edit_indexer_modal
            .as_mut()
            .unwrap()
            .tags
        );
      }
      _ => (),
    }
  }
}
