use crate::app::App;
use crate::event::Key;
use crate::handlers::{KeyEventHandler, handle_prompt_toggle};
use crate::models::readarr_models::AddReadarrRootFolderBody;
use crate::models::servarr_data::readarr::modals::AddReadarrRootFolderModal;
use crate::models::servarr_data::readarr::readarr_data::{
  ADD_ROOT_FOLDER_BLOCKS, ActiveReadarrBlock,
};
use crate::models::{Route, Scrollable};
use crate::network::readarr_network::ReadarrEvent;
use crate::{handle_text_box_keys, handle_text_box_left_right_keys, matches_key};

#[cfg(test)]
#[path = "add_root_folder_handler_tests.rs"]
mod add_root_folder_handler_tests;

pub(super) struct AddRootFolderHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_readarr_block: ActiveReadarrBlock,
  context: Option<ActiveReadarrBlock>,
}

impl AddRootFolderHandler<'_, '_> {
  fn build_add_root_folder_body(&mut self) -> AddReadarrRootFolderBody {
    let add_root_folder_modal = self
      .app
      .data
      .readarr_data
      .add_root_folder_modal
      .take()
      .expect("AddReadarrRootFolderModal is None");

    let tags = add_root_folder_modal.tags.text.clone();

    let AddReadarrRootFolderModal {
      name,
      path,
      monitor_list,
      monitor_new_items_list,
      quality_profile_list,
      metadata_profile_list,
      ..
    } = add_root_folder_modal;

    let quality_profile = quality_profile_list.current_selection();
    let quality_profile_id = *self
      .app
      .data
      .readarr_data
      .quality_profile_map
      .iter()
      .filter(|(_, value)| *value == quality_profile)
      .map(|(key, _)| key)
      .next()
      .unwrap();

    let metadata_profile = metadata_profile_list.current_selection();
    let metadata_profile_id = *self
      .app
      .data
      .readarr_data
      .metadata_profile_map
      .iter()
      .filter(|(_, value)| *value == metadata_profile)
      .map(|(key, _)| key)
      .next()
      .unwrap();

    AddReadarrRootFolderBody {
      name: name.text,
      path: path.text,
      default_quality_profile_id: quality_profile_id,
      default_metadata_profile_id: metadata_profile_id,
      default_monitor_option: *monitor_list.current_selection(),
      default_new_item_monitor_option: *monitor_new_items_list.current_selection(),
      default_tags: Vec::new(),
      tag_input_string: Some(tags),
    }
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveReadarrBlock> for AddRootFolderHandler<'a, 'b> {
  fn accepts(active_block: ActiveReadarrBlock) -> bool {
    ADD_ROOT_FOLDER_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveReadarrBlock,
    context: Option<ActiveReadarrBlock>,
  ) -> AddRootFolderHandler<'a, 'b> {
    AddRootFolderHandler {
      key,
      app,
      active_readarr_block: active_block,
      context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    !self.app.is_loading && self.app.data.readarr_data.add_root_folder_modal.is_some()
  }

  fn handle_scroll_up(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::AddRootFolderSelectMonitor => self
        .app
        .data
        .readarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .scroll_up(),
      ActiveReadarrBlock::AddRootFolderSelectMonitorNewItems => self
        .app
        .data
        .readarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .monitor_new_items_list
        .scroll_up(),
      ActiveReadarrBlock::AddRootFolderSelectQualityProfile => self
        .app
        .data
        .readarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_up(),
      ActiveReadarrBlock::AddRootFolderSelectMetadataProfile => self
        .app
        .data
        .readarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .scroll_up(),
      ActiveReadarrBlock::AddRootFolderPrompt => self.app.data.readarr_data.selected_block.up(),
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::AddRootFolderSelectMonitor => self
        .app
        .data
        .readarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .scroll_down(),
      ActiveReadarrBlock::AddRootFolderSelectMonitorNewItems => self
        .app
        .data
        .readarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .monitor_new_items_list
        .scroll_down(),
      ActiveReadarrBlock::AddRootFolderSelectQualityProfile => self
        .app
        .data
        .readarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_down(),
      ActiveReadarrBlock::AddRootFolderSelectMetadataProfile => self
        .app
        .data
        .readarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .scroll_down(),
      ActiveReadarrBlock::AddRootFolderPrompt => self.app.data.readarr_data.selected_block.down(),
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::AddRootFolderSelectMonitor => self
        .app
        .data
        .readarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .scroll_to_top(),
      ActiveReadarrBlock::AddRootFolderSelectMonitorNewItems => self
        .app
        .data
        .readarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .monitor_new_items_list
        .scroll_to_top(),
      ActiveReadarrBlock::AddRootFolderSelectQualityProfile => self
        .app
        .data
        .readarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_to_top(),
      ActiveReadarrBlock::AddRootFolderSelectMetadataProfile => self
        .app
        .data
        .readarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .scroll_to_top(),
      ActiveReadarrBlock::AddRootFolderNameInput => self
        .app
        .data
        .readarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .name
        .scroll_home(),
      ActiveReadarrBlock::AddRootFolderPathInput => self
        .app
        .data
        .readarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .path
        .scroll_home(),
      ActiveReadarrBlock::AddRootFolderTagsInput => self
        .app
        .data
        .readarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .tags
        .scroll_home(),
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::AddRootFolderSelectMonitor => self
        .app
        .data
        .readarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .scroll_to_bottom(),
      ActiveReadarrBlock::AddRootFolderSelectMonitorNewItems => self
        .app
        .data
        .readarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .monitor_new_items_list
        .scroll_to_bottom(),
      ActiveReadarrBlock::AddRootFolderSelectQualityProfile => self
        .app
        .data
        .readarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_to_bottom(),
      ActiveReadarrBlock::AddRootFolderSelectMetadataProfile => self
        .app
        .data
        .readarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .scroll_to_bottom(),
      ActiveReadarrBlock::AddRootFolderNameInput => self
        .app
        .data
        .readarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .name
        .reset_offset(),
      ActiveReadarrBlock::AddRootFolderPathInput => self
        .app
        .data
        .readarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .path
        .reset_offset(),
      ActiveReadarrBlock::AddRootFolderTagsInput => self
        .app
        .data
        .readarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .tags
        .reset_offset(),
      _ => (),
    }
  }

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::AddRootFolderPrompt => handle_prompt_toggle(self.app, self.key),
      ActiveReadarrBlock::AddRootFolderNameInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .readarr_data
            .add_root_folder_modal
            .as_mut()
            .unwrap()
            .name
        )
      }
      ActiveReadarrBlock::AddRootFolderPathInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .readarr_data
            .add_root_folder_modal
            .as_mut()
            .unwrap()
            .path
        )
      }
      ActiveReadarrBlock::AddRootFolderTagsInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .readarr_data
            .add_root_folder_modal
            .as_mut()
            .unwrap()
            .tags
        )
      }
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::AddRootFolderPrompt => {
        match self.app.data.readarr_data.selected_block.get_active_block() {
          ActiveReadarrBlock::AddRootFolderConfirmPrompt => {
            if self.app.data.readarr_data.prompt_confirm {
              self.app.data.readarr_data.prompt_confirm_action = Some(ReadarrEvent::AddRootFolder(
                self.build_add_root_folder_body(),
              ));
              self.app.should_refresh = true;
            }

            self.app.pop_navigation_stack();
          }
          ActiveReadarrBlock::AddRootFolderSelectMonitor
          | ActiveReadarrBlock::AddRootFolderSelectMonitorNewItems
          | ActiveReadarrBlock::AddRootFolderSelectQualityProfile
          | ActiveReadarrBlock::AddRootFolderSelectMetadataProfile => {
            self.app.push_navigation_stack(
              (
                self.app.data.readarr_data.selected_block.get_active_block(),
                self.context,
              )
                .into(),
            )
          }
          ActiveReadarrBlock::AddRootFolderNameInput
          | ActiveReadarrBlock::AddRootFolderPathInput
          | ActiveReadarrBlock::AddRootFolderTagsInput => {
            self.app.push_navigation_stack(
              (
                self.app.data.readarr_data.selected_block.get_active_block(),
                self.context,
              )
                .into(),
            );
            self.app.ignore_special_keys_for_textbox_input = true;
          }
          _ => (),
        }
      }
      ActiveReadarrBlock::AddRootFolderSelectMonitor
      | ActiveReadarrBlock::AddRootFolderSelectMonitorNewItems
      | ActiveReadarrBlock::AddRootFolderSelectQualityProfile
      | ActiveReadarrBlock::AddRootFolderSelectMetadataProfile => self.app.pop_navigation_stack(),
      ActiveReadarrBlock::AddRootFolderNameInput
      | ActiveReadarrBlock::AddRootFolderPathInput
      | ActiveReadarrBlock::AddRootFolderTagsInput => {
        self.app.pop_navigation_stack();
        self.app.ignore_special_keys_for_textbox_input = false;
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::AddRootFolderNameInput
      | ActiveReadarrBlock::AddRootFolderPathInput
      | ActiveReadarrBlock::AddRootFolderTagsInput => {
        self.app.pop_navigation_stack();
        self.app.ignore_special_keys_for_textbox_input = false;
      }
      ActiveReadarrBlock::AddRootFolderPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.readarr_data.add_root_folder_modal = None;
        self.app.data.readarr_data.prompt_confirm = false;
      }
      ActiveReadarrBlock::AddRootFolderSelectMonitor
      | ActiveReadarrBlock::AddRootFolderSelectMonitorNewItems
      | ActiveReadarrBlock::AddRootFolderSelectQualityProfile
      | ActiveReadarrBlock::AddRootFolderSelectMetadataProfile => self.app.pop_navigation_stack(),
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_readarr_block {
      ActiveReadarrBlock::AddRootFolderNameInput => {
        handle_text_box_keys!(
          self,
          key,
          self
            .app
            .data
            .readarr_data
            .add_root_folder_modal
            .as_mut()
            .unwrap()
            .name
        )
      }
      ActiveReadarrBlock::AddRootFolderPathInput => {
        handle_text_box_keys!(
          self,
          key,
          self
            .app
            .data
            .readarr_data
            .add_root_folder_modal
            .as_mut()
            .unwrap()
            .path
        )
      }
      ActiveReadarrBlock::AddRootFolderTagsInput => {
        handle_text_box_keys!(
          self,
          key,
          self
            .app
            .data
            .readarr_data
            .add_root_folder_modal
            .as_mut()
            .unwrap()
            .tags
        )
      }
      ActiveReadarrBlock::AddRootFolderPrompt
        if self.app.data.readarr_data.selected_block.get_active_block()
          == ActiveReadarrBlock::AddRootFolderConfirmPrompt
          && matches_key!(confirm, key) =>
      {
        self.app.data.readarr_data.prompt_confirm = true;
        self.app.data.readarr_data.prompt_confirm_action = Some(ReadarrEvent::AddRootFolder(
          self.build_add_root_folder_body(),
        ));
        self.app.should_refresh = true;

        self.app.pop_navigation_stack();
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
