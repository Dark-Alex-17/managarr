use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::App;
use crate::event::Key;
use crate::handlers::{handle_prompt_toggle, KeyEventHandler};
use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, EDIT_COLLECTION_BLOCKS};
use crate::models::Scrollable;
use crate::network::radarr_network::RadarrEvent;
use crate::{handle_text_box_keys, handle_text_box_left_right_keys};

#[cfg(test)]
#[path = "edit_collection_handler_tests.rs"]
mod edit_collection_handler_tests;

pub(super) struct EditCollectionHandler<'a, 'b> {
  key: &'a Key,
  app: &'a mut App<'b>,
  active_radarr_block: &'a ActiveRadarrBlock,
  context: &'a Option<ActiveRadarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for EditCollectionHandler<'a, 'b> {
  fn accepts(active_block: &'a ActiveRadarrBlock) -> bool {
    EDIT_COLLECTION_BLOCKS.contains(active_block)
  }

  fn with(
    key: &'a Key,
    app: &'a mut App<'b>,
    active_block: &'a ActiveRadarrBlock,
    context: &'a Option<ActiveRadarrBlock>,
  ) -> EditCollectionHandler<'a, 'b> {
    EditCollectionHandler {
      key,
      app,
      active_radarr_block: active_block,
      context,
    }
  }

  fn get_key(&self) -> &Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    !self.app.is_loading && self.app.data.radarr_data.edit_collection_modal.is_some()
  }

  fn handle_scroll_up(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::EditCollectionSelectMinimumAvailability => self
        .app
        .data
        .radarr_data
        .edit_collection_modal
        .as_mut()
        .unwrap()
        .minimum_availability_list
        .scroll_up(),
      ActiveRadarrBlock::EditCollectionSelectQualityProfile => self
        .app
        .data
        .radarr_data
        .edit_collection_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_up(),
      ActiveRadarrBlock::EditCollectionPrompt => {
        self.app.data.radarr_data.selected_block.previous()
      }
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::EditCollectionSelectMinimumAvailability => self
        .app
        .data
        .radarr_data
        .edit_collection_modal
        .as_mut()
        .unwrap()
        .minimum_availability_list
        .scroll_down(),
      ActiveRadarrBlock::EditCollectionSelectQualityProfile => self
        .app
        .data
        .radarr_data
        .edit_collection_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_down(),
      ActiveRadarrBlock::EditCollectionPrompt => self.app.data.radarr_data.selected_block.next(),
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::EditCollectionSelectMinimumAvailability => self
        .app
        .data
        .radarr_data
        .edit_collection_modal
        .as_mut()
        .unwrap()
        .minimum_availability_list
        .scroll_to_top(),
      ActiveRadarrBlock::EditCollectionSelectQualityProfile => self
        .app
        .data
        .radarr_data
        .edit_collection_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_to_top(),
      ActiveRadarrBlock::EditCollectionRootFolderPathInput => self
        .app
        .data
        .radarr_data
        .edit_collection_modal
        .as_mut()
        .unwrap()
        .path
        .scroll_home(),
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::EditCollectionSelectMinimumAvailability => self
        .app
        .data
        .radarr_data
        .edit_collection_modal
        .as_mut()
        .unwrap()
        .minimum_availability_list
        .scroll_to_bottom(),
      ActiveRadarrBlock::EditCollectionSelectQualityProfile => self
        .app
        .data
        .radarr_data
        .edit_collection_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_to_bottom(),
      ActiveRadarrBlock::EditCollectionRootFolderPathInput => self
        .app
        .data
        .radarr_data
        .edit_collection_modal
        .as_mut()
        .unwrap()
        .path
        .reset_offset(),
      _ => (),
    }
  }

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::EditCollectionPrompt => handle_prompt_toggle(self.app, self.key),
      ActiveRadarrBlock::EditCollectionRootFolderPathInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .radarr_data
            .edit_collection_modal
            .as_mut()
            .unwrap()
            .path
        )
      }
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::EditCollectionPrompt => {
        match self.app.data.radarr_data.selected_block.get_active_block() {
          ActiveRadarrBlock::EditCollectionConfirmPrompt => {
            if self.app.data.radarr_data.prompt_confirm {
              self.app.data.radarr_data.prompt_confirm_action =
                Some(RadarrEvent::EditCollection(None));
              self.app.should_refresh = true;
            }

            self.app.pop_navigation_stack();
          }
          ActiveRadarrBlock::EditCollectionSelectMinimumAvailability
          | ActiveRadarrBlock::EditCollectionSelectQualityProfile => {
            self.app.push_navigation_stack(
              (
                *self.app.data.radarr_data.selected_block.get_active_block(),
                *self.context,
              )
                .into(),
            )
          }
          ActiveRadarrBlock::EditCollectionRootFolderPathInput => {
            self.app.push_navigation_stack(
              (
                *self.app.data.radarr_data.selected_block.get_active_block(),
                *self.context,
              )
                .into(),
            );
            self.app.should_ignore_quit_key = true;
          }
          ActiveRadarrBlock::EditCollectionToggleMonitored => {
            self
              .app
              .data
              .radarr_data
              .edit_collection_modal
              .as_mut()
              .unwrap()
              .monitored = Some(
              !self
                .app
                .data
                .radarr_data
                .edit_collection_modal
                .as_mut()
                .unwrap()
                .monitored
                .unwrap_or_default(),
            )
          }
          ActiveRadarrBlock::EditCollectionToggleSearchOnAdd => {
            self
              .app
              .data
              .radarr_data
              .edit_collection_modal
              .as_mut()
              .unwrap()
              .search_on_add = Some(
              !self
                .app
                .data
                .radarr_data
                .edit_collection_modal
                .as_mut()
                .unwrap()
                .search_on_add
                .unwrap_or_default(),
            )
          }
          _ => (),
        }
      }
      ActiveRadarrBlock::EditCollectionSelectMinimumAvailability
      | ActiveRadarrBlock::EditCollectionSelectQualityProfile => self.app.pop_navigation_stack(),
      ActiveRadarrBlock::EditCollectionRootFolderPathInput => {
        self.app.pop_navigation_stack();
        self.app.should_ignore_quit_key = false;
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::EditCollectionRootFolderPathInput => {
        self.app.pop_navigation_stack();
        self.app.should_ignore_quit_key = false;
      }
      ActiveRadarrBlock::EditCollectionPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.edit_collection_modal = None;
        self.app.data.radarr_data.prompt_confirm = false;
      }
      ActiveRadarrBlock::EditCollectionSelectMinimumAvailability
      | ActiveRadarrBlock::EditCollectionSelectQualityProfile => self.app.pop_navigation_stack(),
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_radarr_block {
      ActiveRadarrBlock::EditCollectionRootFolderPathInput => {
        handle_text_box_keys!(
          self,
          key,
          self
            .app
            .data
            .radarr_data
            .edit_collection_modal
            .as_mut()
            .unwrap()
            .path
        )
      }
      ActiveRadarrBlock::EditCollectionPrompt => {
        if self.app.data.radarr_data.selected_block.get_active_block()
          == &ActiveRadarrBlock::EditCollectionConfirmPrompt
          && *key == DEFAULT_KEYBINDINGS.confirm.key
        {
          self.app.data.radarr_data.prompt_confirm = true;
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::EditCollection(None));
          self.app.should_refresh = true;

          self.app.pop_navigation_stack();
        }
      }
      _ => (),
    }
  }
}
