use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::App;
use crate::event::Key;
use crate::handlers::{handle_prompt_toggle, KeyEventHandler};
use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, EDIT_INDEXER_BLOCKS};
use crate::network::sonarr_network::SonarrEvent;
use crate::{handle_prompt_left_right_keys, handle_text_box_keys, handle_text_box_left_right_keys};

#[cfg(test)]
#[path = "edit_indexer_handler_tests.rs"]
mod edit_indexer_handler_tests;

pub(super) struct EditIndexerHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_sonarr_block: ActiveSonarrBlock,
  _context: Option<ActiveSonarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveSonarrBlock> for EditIndexerHandler<'a, 'b> {
  fn accepts(active_block: ActiveSonarrBlock) -> bool {
    EDIT_INDEXER_BLOCKS.contains(&active_block)
  }

  fn with(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveSonarrBlock,
    _context: Option<ActiveSonarrBlock>,
  ) -> EditIndexerHandler<'a, 'b> {
    EditIndexerHandler {
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
    !self.app.is_loading && self.app.data.sonarr_data.edit_indexer_modal.is_some()
  }

  fn handle_scroll_up(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::EditIndexerPrompt => {
        self.app.data.sonarr_data.selected_block.up();
      }
      ActiveSonarrBlock::EditIndexerPriorityInput => {
        self
          .app
          .data
          .sonarr_data
          .edit_indexer_modal
          .as_mut()
          .unwrap()
          .priority += 1;
      }
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::EditIndexerPrompt => {
        self.app.data.sonarr_data.selected_block.down();
      }
      ActiveSonarrBlock::EditIndexerPriorityInput => {
        let edit_indexer_modal = self
          .app
          .data
          .sonarr_data
          .edit_indexer_modal
          .as_mut()
          .unwrap();
        if edit_indexer_modal.priority > 0 {
          edit_indexer_modal.priority -= 1;
        }
      }
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::EditIndexerNameInput => {
        self
          .app
          .data
          .sonarr_data
          .edit_indexer_modal
          .as_mut()
          .unwrap()
          .name
          .scroll_home();
      }
      ActiveSonarrBlock::EditIndexerUrlInput => {
        self
          .app
          .data
          .sonarr_data
          .edit_indexer_modal
          .as_mut()
          .unwrap()
          .url
          .scroll_home();
      }
      ActiveSonarrBlock::EditIndexerApiKeyInput => {
        self
          .app
          .data
          .sonarr_data
          .edit_indexer_modal
          .as_mut()
          .unwrap()
          .api_key
          .scroll_home();
      }
      ActiveSonarrBlock::EditIndexerSeedRatioInput => {
        self
          .app
          .data
          .sonarr_data
          .edit_indexer_modal
          .as_mut()
          .unwrap()
          .seed_ratio
          .scroll_home();
      }
      ActiveSonarrBlock::EditIndexerTagsInput => {
        self
          .app
          .data
          .sonarr_data
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
    match self.active_sonarr_block {
      ActiveSonarrBlock::EditIndexerNameInput => {
        self
          .app
          .data
          .sonarr_data
          .edit_indexer_modal
          .as_mut()
          .unwrap()
          .name
          .reset_offset();
      }
      ActiveSonarrBlock::EditIndexerUrlInput => {
        self
          .app
          .data
          .sonarr_data
          .edit_indexer_modal
          .as_mut()
          .unwrap()
          .url
          .reset_offset();
      }
      ActiveSonarrBlock::EditIndexerApiKeyInput => {
        self
          .app
          .data
          .sonarr_data
          .edit_indexer_modal
          .as_mut()
          .unwrap()
          .api_key
          .reset_offset();
      }
      ActiveSonarrBlock::EditIndexerSeedRatioInput => {
        self
          .app
          .data
          .sonarr_data
          .edit_indexer_modal
          .as_mut()
          .unwrap()
          .seed_ratio
          .reset_offset();
      }
      ActiveSonarrBlock::EditIndexerTagsInput => {
        self
          .app
          .data
          .sonarr_data
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
    match self.active_sonarr_block {
      ActiveSonarrBlock::EditIndexerPrompt => {
        handle_prompt_left_right_keys!(
          self,
          ActiveSonarrBlock::EditIndexerConfirmPrompt,
          sonarr_data
        );
      }
      ActiveSonarrBlock::EditIndexerNameInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .sonarr_data
            .edit_indexer_modal
            .as_mut()
            .unwrap()
            .name
        );
      }
      ActiveSonarrBlock::EditIndexerUrlInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .sonarr_data
            .edit_indexer_modal
            .as_mut()
            .unwrap()
            .url
        );
      }
      ActiveSonarrBlock::EditIndexerApiKeyInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .sonarr_data
            .edit_indexer_modal
            .as_mut()
            .unwrap()
            .api_key
        );
      }
      ActiveSonarrBlock::EditIndexerSeedRatioInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .sonarr_data
            .edit_indexer_modal
            .as_mut()
            .unwrap()
            .seed_ratio
        );
      }
      ActiveSonarrBlock::EditIndexerTagsInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .sonarr_data
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
    match self.active_sonarr_block {
      ActiveSonarrBlock::EditIndexerPrompt => {
        let selected_block = self.app.data.sonarr_data.selected_block.get_active_block();
        match selected_block {
          ActiveSonarrBlock::EditIndexerConfirmPrompt => {
            let sonarr_data = &mut self.app.data.sonarr_data;
            if sonarr_data.prompt_confirm {
              sonarr_data.prompt_confirm_action = Some(SonarrEvent::EditIndexer(None));
              self.app.should_refresh = true;
            } else {
              sonarr_data.edit_indexer_modal = None;
            }

            self.app.pop_navigation_stack();
          }
          ActiveSonarrBlock::EditIndexerNameInput
          | ActiveSonarrBlock::EditIndexerUrlInput
          | ActiveSonarrBlock::EditIndexerApiKeyInput
          | ActiveSonarrBlock::EditIndexerSeedRatioInput
          | ActiveSonarrBlock::EditIndexerTagsInput => {
            self.app.push_navigation_stack(selected_block.into());
            self.app.should_ignore_quit_key = true;
          }
          ActiveSonarrBlock::EditIndexerPriorityInput => self
            .app
            .push_navigation_stack(ActiveSonarrBlock::EditIndexerPriorityInput.into()),
          ActiveSonarrBlock::EditIndexerToggleEnableRss => {
            let indexer = self
              .app
              .data
              .sonarr_data
              .edit_indexer_modal
              .as_mut()
              .unwrap();
            indexer.enable_rss = Some(!indexer.enable_rss.unwrap_or_default());
          }
          ActiveSonarrBlock::EditIndexerToggleEnableAutomaticSearch => {
            let indexer = self
              .app
              .data
              .sonarr_data
              .edit_indexer_modal
              .as_mut()
              .unwrap();
            indexer.enable_automatic_search =
              Some(!indexer.enable_automatic_search.unwrap_or_default());
          }
          ActiveSonarrBlock::EditIndexerToggleEnableInteractiveSearch => {
            let indexer = self
              .app
              .data
              .sonarr_data
              .edit_indexer_modal
              .as_mut()
              .unwrap();
            indexer.enable_interactive_search =
              Some(!indexer.enable_interactive_search.unwrap_or_default());
          }
          _ => (),
        }
      }
      ActiveSonarrBlock::EditIndexerNameInput
      | ActiveSonarrBlock::EditIndexerUrlInput
      | ActiveSonarrBlock::EditIndexerApiKeyInput
      | ActiveSonarrBlock::EditIndexerSeedRatioInput
      | ActiveSonarrBlock::EditIndexerTagsInput => {
        self.app.pop_navigation_stack();
        self.app.should_ignore_quit_key = false;
      }
      ActiveSonarrBlock::EditIndexerPriorityInput => self.app.pop_navigation_stack(),
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::EditIndexerPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.sonarr_data.prompt_confirm = false;
        self.app.data.sonarr_data.edit_indexer_modal = None;
      }
      ActiveSonarrBlock::EditIndexerNameInput
      | ActiveSonarrBlock::EditIndexerUrlInput
      | ActiveSonarrBlock::EditIndexerApiKeyInput
      | ActiveSonarrBlock::EditIndexerSeedRatioInput
      | ActiveSonarrBlock::EditIndexerPriorityInput
      | ActiveSonarrBlock::EditIndexerTagsInput => {
        self.app.pop_navigation_stack();
        self.app.should_ignore_quit_key = false;
      }
      _ => self.app.pop_navigation_stack(),
    }
  }

  fn handle_char_key_event(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::EditIndexerNameInput => {
        handle_text_box_keys!(
          self,
          self.key,
          self
            .app
            .data
            .sonarr_data
            .edit_indexer_modal
            .as_mut()
            .unwrap()
            .name
        );
      }
      ActiveSonarrBlock::EditIndexerUrlInput => {
        handle_text_box_keys!(
          self,
          self.key,
          self
            .app
            .data
            .sonarr_data
            .edit_indexer_modal
            .as_mut()
            .unwrap()
            .url
        );
      }
      ActiveSonarrBlock::EditIndexerApiKeyInput => {
        handle_text_box_keys!(
          self,
          self.key,
          self
            .app
            .data
            .sonarr_data
            .edit_indexer_modal
            .as_mut()
            .unwrap()
            .api_key
        );
      }
      ActiveSonarrBlock::EditIndexerSeedRatioInput => {
        handle_text_box_keys!(
          self,
          self.key,
          self
            .app
            .data
            .sonarr_data
            .edit_indexer_modal
            .as_mut()
            .unwrap()
            .seed_ratio
        );
      }
      ActiveSonarrBlock::EditIndexerTagsInput => {
        handle_text_box_keys!(
          self,
          self.key,
          self
            .app
            .data
            .sonarr_data
            .edit_indexer_modal
            .as_mut()
            .unwrap()
            .tags
        );
      }
      ActiveSonarrBlock::EditIndexerPrompt => {
        if self.app.data.sonarr_data.selected_block.get_active_block()
          == ActiveSonarrBlock::EditIndexerConfirmPrompt
          && self.key == DEFAULT_KEYBINDINGS.confirm.key
        {
          self.app.data.sonarr_data.prompt_confirm = true;
          self.app.data.sonarr_data.prompt_confirm_action = Some(SonarrEvent::EditIndexer(None));
          self.app.should_refresh = true;

          self.app.pop_navigation_stack();
        }
      }
      _ => (),
    }
  }
}
