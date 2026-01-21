use crate::app::App;
use crate::event::Key;
use crate::handlers::{KeyEventHandler, handle_prompt_toggle};
use crate::models::Route;
use crate::models::servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, EDIT_INDEXER_BLOCKS};
use crate::models::servarr_data::modals::EditIndexerModal;
use crate::models::servarr_models::EditIndexerParams;
use crate::network::lidarr_network::LidarrEvent;
use crate::{
  handle_prompt_left_right_keys, handle_text_box_keys, handle_text_box_left_right_keys, matches_key,
};

#[cfg(test)]
#[path = "edit_indexer_handler_tests.rs"]
mod edit_indexer_handler_tests;

pub(super) struct EditIndexerHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_lidarr_block: ActiveLidarrBlock,
  _context: Option<ActiveLidarrBlock>,
}

impl EditIndexerHandler<'_, '_> {
  fn build_edit_indexer_params(&mut self) -> EditIndexerParams {
    let edit_indexer_modal = self
      .app
      .data
      .lidarr_data
      .edit_indexer_modal
      .take()
      .expect("EditIndexerModal is None");
    let indexer_id = self.app.data.lidarr_data.indexers.current_selection().id;
    let tags = edit_indexer_modal.tags.text;
    let EditIndexerModal {
      name,
      enable_rss,
      enable_automatic_search,
      enable_interactive_search,
      url,
      api_key,
      seed_ratio,
      priority,
      ..
    } = edit_indexer_modal;

    EditIndexerParams {
      indexer_id,
      name: Some(name.text),
      enable_rss,
      enable_automatic_search,
      enable_interactive_search,
      url: Some(url.text),
      api_key: Some(api_key.text),
      seed_ratio: Some(seed_ratio.text),
      tags: None,
      tag_input_string: Some(tags),
      priority: Some(priority),
      clear_tags: false,
    }
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveLidarrBlock> for EditIndexerHandler<'a, 'b> {
  fn accepts(active_block: ActiveLidarrBlock) -> bool {
    EDIT_INDEXER_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveLidarrBlock,
    _context: Option<ActiveLidarrBlock>,
  ) -> EditIndexerHandler<'a, 'b> {
    EditIndexerHandler {
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
    !self.app.is_loading && self.app.data.lidarr_data.edit_indexer_modal.is_some()
  }

  fn handle_scroll_up(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::EditIndexerPrompt => {
        self.app.data.lidarr_data.selected_block.up();
      }
      ActiveLidarrBlock::EditIndexerPriorityInput => {
        self
          .app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_mut()
          .unwrap()
          .priority += 1;
      }
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::EditIndexerPrompt => {
        self.app.data.lidarr_data.selected_block.down();
      }
      ActiveLidarrBlock::EditIndexerPriorityInput => {
        let edit_indexer_modal = self
          .app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_mut()
          .unwrap();
        if edit_indexer_modal.priority > 1 {
          edit_indexer_modal.priority -= 1;
        }
      }
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::EditIndexerNameInput => {
        self
          .app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_mut()
          .unwrap()
          .name
          .scroll_home();
      }
      ActiveLidarrBlock::EditIndexerUrlInput => {
        self
          .app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_mut()
          .unwrap()
          .url
          .scroll_home();
      }
      ActiveLidarrBlock::EditIndexerApiKeyInput => {
        self
          .app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_mut()
          .unwrap()
          .api_key
          .scroll_home();
      }
      ActiveLidarrBlock::EditIndexerSeedRatioInput => {
        self
          .app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_mut()
          .unwrap()
          .seed_ratio
          .scroll_home();
      }
      ActiveLidarrBlock::EditIndexerTagsInput => {
        self
          .app
          .data
          .lidarr_data
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
    match self.active_lidarr_block {
      ActiveLidarrBlock::EditIndexerNameInput => {
        self
          .app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_mut()
          .unwrap()
          .name
          .reset_offset();
      }
      ActiveLidarrBlock::EditIndexerUrlInput => {
        self
          .app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_mut()
          .unwrap()
          .url
          .reset_offset();
      }
      ActiveLidarrBlock::EditIndexerApiKeyInput => {
        self
          .app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_mut()
          .unwrap()
          .api_key
          .reset_offset();
      }
      ActiveLidarrBlock::EditIndexerSeedRatioInput => {
        self
          .app
          .data
          .lidarr_data
          .edit_indexer_modal
          .as_mut()
          .unwrap()
          .seed_ratio
          .reset_offset();
      }
      ActiveLidarrBlock::EditIndexerTagsInput => {
        self
          .app
          .data
          .lidarr_data
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
    match self.active_lidarr_block {
      ActiveLidarrBlock::EditIndexerPrompt => {
        handle_prompt_left_right_keys!(
          self,
          ActiveLidarrBlock::EditIndexerConfirmPrompt,
          lidarr_data
        );
      }
      ActiveLidarrBlock::EditIndexerNameInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .lidarr_data
            .edit_indexer_modal
            .as_mut()
            .unwrap()
            .name
        );
      }
      ActiveLidarrBlock::EditIndexerUrlInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .lidarr_data
            .edit_indexer_modal
            .as_mut()
            .unwrap()
            .url
        );
      }
      ActiveLidarrBlock::EditIndexerApiKeyInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .lidarr_data
            .edit_indexer_modal
            .as_mut()
            .unwrap()
            .api_key
        );
      }
      ActiveLidarrBlock::EditIndexerSeedRatioInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .lidarr_data
            .edit_indexer_modal
            .as_mut()
            .unwrap()
            .seed_ratio
        );
      }
      ActiveLidarrBlock::EditIndexerTagsInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .lidarr_data
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
    match self.active_lidarr_block {
      ActiveLidarrBlock::EditIndexerPrompt => {
        let selected_block = self.app.data.lidarr_data.selected_block.get_active_block();
        match selected_block {
          ActiveLidarrBlock::EditIndexerConfirmPrompt => {
            if self.app.data.lidarr_data.prompt_confirm {
              self.app.data.lidarr_data.prompt_confirm_action =
                Some(LidarrEvent::EditIndexer(self.build_edit_indexer_params()));
              self.app.should_refresh = true;
            } else {
              self.app.data.lidarr_data.edit_indexer_modal = None;
            }

            self.app.pop_navigation_stack();
          }
          ActiveLidarrBlock::EditIndexerNameInput
          | ActiveLidarrBlock::EditIndexerUrlInput
          | ActiveLidarrBlock::EditIndexerApiKeyInput
          | ActiveLidarrBlock::EditIndexerSeedRatioInput
          | ActiveLidarrBlock::EditIndexerTagsInput => {
            self.app.push_navigation_stack(selected_block.into());
            self.app.ignore_special_keys_for_textbox_input = true;
          }
          ActiveLidarrBlock::EditIndexerPriorityInput => self
            .app
            .push_navigation_stack(ActiveLidarrBlock::EditIndexerPriorityInput.into()),
          ActiveLidarrBlock::EditIndexerToggleEnableRss => {
            let indexer = self
              .app
              .data
              .lidarr_data
              .edit_indexer_modal
              .as_mut()
              .unwrap();
            indexer.enable_rss = Some(!indexer.enable_rss.unwrap_or_default());
          }
          ActiveLidarrBlock::EditIndexerToggleEnableAutomaticSearch => {
            let indexer = self
              .app
              .data
              .lidarr_data
              .edit_indexer_modal
              .as_mut()
              .unwrap();
            indexer.enable_automatic_search =
              Some(!indexer.enable_automatic_search.unwrap_or_default());
          }
          ActiveLidarrBlock::EditIndexerToggleEnableInteractiveSearch => {
            let indexer = self
              .app
              .data
              .lidarr_data
              .edit_indexer_modal
              .as_mut()
              .unwrap();
            indexer.enable_interactive_search =
              Some(!indexer.enable_interactive_search.unwrap_or_default());
          }
          _ => (),
        }
      }
      ActiveLidarrBlock::EditIndexerNameInput
      | ActiveLidarrBlock::EditIndexerUrlInput
      | ActiveLidarrBlock::EditIndexerApiKeyInput
      | ActiveLidarrBlock::EditIndexerSeedRatioInput
      | ActiveLidarrBlock::EditIndexerTagsInput => {
        self.app.pop_navigation_stack();
        self.app.ignore_special_keys_for_textbox_input = false;
      }
      ActiveLidarrBlock::EditIndexerPriorityInput => self.app.pop_navigation_stack(),
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::EditIndexerPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.lidarr_data.prompt_confirm = false;
        self.app.data.lidarr_data.edit_indexer_modal = None;
      }
      ActiveLidarrBlock::EditIndexerNameInput
      | ActiveLidarrBlock::EditIndexerUrlInput
      | ActiveLidarrBlock::EditIndexerApiKeyInput
      | ActiveLidarrBlock::EditIndexerSeedRatioInput
      | ActiveLidarrBlock::EditIndexerPriorityInput
      | ActiveLidarrBlock::EditIndexerTagsInput => {
        self.app.pop_navigation_stack();
        self.app.ignore_special_keys_for_textbox_input = false;
      }
      _ => self.app.pop_navigation_stack(),
    }
  }

  fn handle_char_key_event(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::EditIndexerNameInput => {
        handle_text_box_keys!(
          self,
          self.key,
          self
            .app
            .data
            .lidarr_data
            .edit_indexer_modal
            .as_mut()
            .unwrap()
            .name
        );
      }
      ActiveLidarrBlock::EditIndexerUrlInput => {
        handle_text_box_keys!(
          self,
          self.key,
          self
            .app
            .data
            .lidarr_data
            .edit_indexer_modal
            .as_mut()
            .unwrap()
            .url
        );
      }
      ActiveLidarrBlock::EditIndexerApiKeyInput => {
        handle_text_box_keys!(
          self,
          self.key,
          self
            .app
            .data
            .lidarr_data
            .edit_indexer_modal
            .as_mut()
            .unwrap()
            .api_key
        );
      }
      ActiveLidarrBlock::EditIndexerSeedRatioInput => {
        handle_text_box_keys!(
          self,
          self.key,
          self
            .app
            .data
            .lidarr_data
            .edit_indexer_modal
            .as_mut()
            .unwrap()
            .seed_ratio
        );
      }
      ActiveLidarrBlock::EditIndexerTagsInput => {
        handle_text_box_keys!(
          self,
          self.key,
          self
            .app
            .data
            .lidarr_data
            .edit_indexer_modal
            .as_mut()
            .unwrap()
            .tags
        );
      }
      ActiveLidarrBlock::EditIndexerPrompt => {
        if self.app.data.lidarr_data.selected_block.get_active_block()
          == ActiveLidarrBlock::EditIndexerConfirmPrompt
          && matches_key!(confirm, self.key)
        {
          self.app.data.lidarr_data.prompt_confirm = true;
          self.app.data.lidarr_data.prompt_confirm_action =
            Some(LidarrEvent::EditIndexer(self.build_edit_indexer_params()));
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
