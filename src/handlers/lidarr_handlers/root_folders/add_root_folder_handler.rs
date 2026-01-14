use crate::app::App;
use crate::event::Key;
use crate::handlers::{KeyEventHandler, handle_prompt_toggle};
use crate::models::lidarr_models::AddLidarrRootFolderBody;
use crate::models::servarr_data::lidarr::lidarr_data::{ADD_ROOT_FOLDER_BLOCKS, ActiveLidarrBlock};
use crate::models::servarr_data::lidarr::modals::AddRootFolderModal;
use crate::models::{Route, Scrollable};
use crate::network::lidarr_network::LidarrEvent;
use crate::{handle_text_box_keys, handle_text_box_left_right_keys, matches_key};

#[cfg(test)]
#[path = "add_root_folder_handler_tests.rs"]
mod add_root_folder_handler_tests;

pub(super) struct AddRootFolderHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_lidarr_block: ActiveLidarrBlock,
  context: Option<ActiveLidarrBlock>,
}

impl AddRootFolderHandler<'_, '_> {
  fn build_add_root_folder_body(&mut self) -> AddLidarrRootFolderBody {
    let add_root_folder_modal = self
      .app
      .data
      .lidarr_data
      .add_root_folder_modal
      .take()
      .expect("AddRootFolderModal is None");

    let tags = add_root_folder_modal.tags.text.clone();

    let AddRootFolderModal {
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
      .lidarr_data
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
      .lidarr_data
      .metadata_profile_map
      .iter()
      .filter(|(_, value)| *value == metadata_profile)
      .map(|(key, _)| key)
      .next()
      .unwrap();

    AddLidarrRootFolderBody {
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

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveLidarrBlock> for AddRootFolderHandler<'a, 'b> {
  fn accepts(active_block: ActiveLidarrBlock) -> bool {
    ADD_ROOT_FOLDER_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveLidarrBlock,
    context: Option<ActiveLidarrBlock>,
  ) -> AddRootFolderHandler<'a, 'b> {
    AddRootFolderHandler {
      key,
      app,
      active_lidarr_block: active_block,
      context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    !self.app.is_loading && self.app.data.lidarr_data.add_root_folder_modal.is_some()
  }

  fn handle_scroll_up(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::AddRootFolderSelectMonitor => self
        .app
        .data
        .lidarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .scroll_up(),
      ActiveLidarrBlock::AddRootFolderSelectMonitorNewItems => self
        .app
        .data
        .lidarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .monitor_new_items_list
        .scroll_up(),
      ActiveLidarrBlock::AddRootFolderSelectQualityProfile => self
        .app
        .data
        .lidarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_up(),
      ActiveLidarrBlock::AddRootFolderSelectMetadataProfile => self
        .app
        .data
        .lidarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .scroll_up(),
      ActiveLidarrBlock::AddRootFolderPrompt => self.app.data.lidarr_data.selected_block.up(),
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::AddRootFolderSelectMonitor => self
        .app
        .data
        .lidarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .scroll_down(),
      ActiveLidarrBlock::AddRootFolderSelectMonitorNewItems => self
        .app
        .data
        .lidarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .monitor_new_items_list
        .scroll_down(),
      ActiveLidarrBlock::AddRootFolderSelectQualityProfile => self
        .app
        .data
        .lidarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_down(),
      ActiveLidarrBlock::AddRootFolderSelectMetadataProfile => self
        .app
        .data
        .lidarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .scroll_down(),
      ActiveLidarrBlock::AddRootFolderPrompt => self.app.data.lidarr_data.selected_block.down(),
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::AddRootFolderSelectMonitor => self
        .app
        .data
        .lidarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .scroll_to_top(),
      ActiveLidarrBlock::AddRootFolderSelectMonitorNewItems => self
        .app
        .data
        .lidarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .monitor_new_items_list
        .scroll_to_top(),
      ActiveLidarrBlock::AddRootFolderSelectQualityProfile => self
        .app
        .data
        .lidarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_to_top(),
      ActiveLidarrBlock::AddRootFolderSelectMetadataProfile => self
        .app
        .data
        .lidarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .scroll_to_top(),
      ActiveLidarrBlock::AddRootFolderNameInput => self
        .app
        .data
        .lidarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .name
        .scroll_home(),
      ActiveLidarrBlock::AddRootFolderPathInput => self
        .app
        .data
        .lidarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .path
        .scroll_home(),
      ActiveLidarrBlock::AddRootFolderTagsInput => self
        .app
        .data
        .lidarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .tags
        .scroll_home(),
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::AddRootFolderSelectMonitor => self
        .app
        .data
        .lidarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .scroll_to_bottom(),
      ActiveLidarrBlock::AddRootFolderSelectMonitorNewItems => self
        .app
        .data
        .lidarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .monitor_new_items_list
        .scroll_to_bottom(),
      ActiveLidarrBlock::AddRootFolderSelectQualityProfile => self
        .app
        .data
        .lidarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_to_bottom(),
      ActiveLidarrBlock::AddRootFolderSelectMetadataProfile => self
        .app
        .data
        .lidarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .scroll_to_bottom(),
      ActiveLidarrBlock::AddRootFolderNameInput => self
        .app
        .data
        .lidarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .name
        .reset_offset(),
      ActiveLidarrBlock::AddRootFolderPathInput => self
        .app
        .data
        .lidarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .path
        .reset_offset(),
      ActiveLidarrBlock::AddRootFolderTagsInput => self
        .app
        .data
        .lidarr_data
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
    match self.active_lidarr_block {
      ActiveLidarrBlock::AddRootFolderPrompt => handle_prompt_toggle(self.app, self.key),
      ActiveLidarrBlock::AddRootFolderNameInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .lidarr_data
            .add_root_folder_modal
            .as_mut()
            .unwrap()
            .name
        )
      }
      ActiveLidarrBlock::AddRootFolderPathInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .lidarr_data
            .add_root_folder_modal
            .as_mut()
            .unwrap()
            .path
        )
      }
      ActiveLidarrBlock::AddRootFolderTagsInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .lidarr_data
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
    match self.active_lidarr_block {
      ActiveLidarrBlock::AddRootFolderPrompt => {
        match self.app.data.lidarr_data.selected_block.get_active_block() {
          ActiveLidarrBlock::AddRootFolderConfirmPrompt => {
            if self.app.data.lidarr_data.prompt_confirm {
              self.app.data.lidarr_data.prompt_confirm_action = Some(LidarrEvent::AddRootFolder(
                self.build_add_root_folder_body(),
              ));
              self.app.should_refresh = true;
            }

            self.app.pop_navigation_stack();
          }
          ActiveLidarrBlock::AddRootFolderSelectMonitor
          | ActiveLidarrBlock::AddRootFolderSelectMonitorNewItems
          | ActiveLidarrBlock::AddRootFolderSelectQualityProfile
          | ActiveLidarrBlock::AddRootFolderSelectMetadataProfile => {
            self.app.push_navigation_stack(
              (
                self.app.data.lidarr_data.selected_block.get_active_block(),
                self.context,
              )
                .into(),
            )
          }
          ActiveLidarrBlock::AddRootFolderNameInput
          | ActiveLidarrBlock::AddRootFolderPathInput
          | ActiveLidarrBlock::AddRootFolderTagsInput => {
            self.app.push_navigation_stack(
              (
                self.app.data.lidarr_data.selected_block.get_active_block(),
                self.context,
              )
                .into(),
            );
            self.app.ignore_special_keys_for_textbox_input = true;
          }
          _ => (),
        }
      }
      ActiveLidarrBlock::AddRootFolderSelectMonitor
      | ActiveLidarrBlock::AddRootFolderSelectMonitorNewItems
      | ActiveLidarrBlock::AddRootFolderSelectQualityProfile
      | ActiveLidarrBlock::AddRootFolderSelectMetadataProfile => self.app.pop_navigation_stack(),
      ActiveLidarrBlock::AddRootFolderNameInput
      | ActiveLidarrBlock::AddRootFolderPathInput
      | ActiveLidarrBlock::AddRootFolderTagsInput => {
        self.app.pop_navigation_stack();
        self.app.ignore_special_keys_for_textbox_input = false;
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::AddRootFolderNameInput
      | ActiveLidarrBlock::AddRootFolderPathInput
      | ActiveLidarrBlock::AddRootFolderTagsInput => {
        self.app.pop_navigation_stack();
        self.app.ignore_special_keys_for_textbox_input = false;
      }
      ActiveLidarrBlock::AddRootFolderPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.lidarr_data.add_root_folder_modal = None;
        self.app.data.lidarr_data.prompt_confirm = false;
      }
      ActiveLidarrBlock::AddRootFolderSelectMonitor
      | ActiveLidarrBlock::AddRootFolderSelectMonitorNewItems
      | ActiveLidarrBlock::AddRootFolderSelectQualityProfile
      | ActiveLidarrBlock::AddRootFolderSelectMetadataProfile => self.app.pop_navigation_stack(),
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_lidarr_block {
      ActiveLidarrBlock::AddRootFolderNameInput => {
        handle_text_box_keys!(
          self,
          key,
          self
            .app
            .data
            .lidarr_data
            .add_root_folder_modal
            .as_mut()
            .unwrap()
            .name
        )
      }
      ActiveLidarrBlock::AddRootFolderPathInput => {
        handle_text_box_keys!(
          self,
          key,
          self
            .app
            .data
            .lidarr_data
            .add_root_folder_modal
            .as_mut()
            .unwrap()
            .path
        )
      }
      ActiveLidarrBlock::AddRootFolderTagsInput => {
        handle_text_box_keys!(
          self,
          key,
          self
            .app
            .data
            .lidarr_data
            .add_root_folder_modal
            .as_mut()
            .unwrap()
            .tags
        )
      }
      ActiveLidarrBlock::AddRootFolderPrompt => {
        if self.app.data.lidarr_data.selected_block.get_active_block()
          == ActiveLidarrBlock::AddRootFolderConfirmPrompt
          && matches_key!(confirm, key)
        {
          self.app.data.lidarr_data.prompt_confirm = true;
          self.app.data.lidarr_data.prompt_confirm_action = Some(LidarrEvent::AddRootFolder(
            self.build_add_root_folder_body(),
          ));
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
