use add_root_folder_handler::AddRootFolderHandler;

use crate::app::App;
use crate::event::Key;
use crate::handlers::lidarr_handlers::handle_change_tab_left_right_keys;
use crate::handlers::table_handler::{TableHandlingConfig, handle_table};
use crate::handlers::{KeyEventHandler, handle_clear_errors, handle_prompt_toggle};
use crate::matches_key;
use crate::models::servarr_data::lidarr::lidarr_data::{
  ADD_ROOT_FOLDER_BLOCKS, ADD_ROOT_FOLDER_SELECTION_BLOCKS, ActiveLidarrBlock, ROOT_FOLDERS_BLOCKS,
};
use crate::models::servarr_data::lidarr::modals::AddRootFolderModal;
use crate::models::{BlockSelectionState, Route};
use crate::network::lidarr_network::LidarrEvent;

mod add_root_folder_handler;

#[cfg(test)]
#[path = "root_folders_handler_tests.rs"]
mod root_folders_handler_tests;

pub(super) struct RootFoldersHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_lidarr_block: ActiveLidarrBlock,
  context: Option<ActiveLidarrBlock>,
}

impl RootFoldersHandler<'_, '_> {
  fn extract_root_folder_id(&self) -> i64 {
    self
      .app
      .data
      .lidarr_data
      .root_folders
      .current_selection()
      .id
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveLidarrBlock> for RootFoldersHandler<'a, 'b> {
  fn handle(&mut self) {
    let root_folders_table_handling_config =
      TableHandlingConfig::new(ActiveLidarrBlock::RootFolders.into());

    if AddRootFolderHandler::accepts(self.active_lidarr_block) {
      return AddRootFolderHandler::new(self.key, self.app, self.active_lidarr_block, self.context)
        .handle();
    }

    if !handle_table(
      self,
      |app| &mut app.data.lidarr_data.root_folders,
      root_folders_table_handling_config,
    ) {
      self.handle_key_event();
    }
  }

  fn accepts(active_block: ActiveLidarrBlock) -> bool {
    ROOT_FOLDERS_BLOCKS.contains(&active_block) || ADD_ROOT_FOLDER_BLOCKS.contains(&active_block)
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveLidarrBlock,
    context: Option<ActiveLidarrBlock>,
  ) -> RootFoldersHandler<'a, 'b> {
    RootFoldersHandler {
      key,
      app,
      active_lidarr_block: active_block,
      context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn is_ready(&self) -> bool {
    !self.app.is_loading && !self.app.data.lidarr_data.root_folders.is_empty()
  }

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {
    if self.active_lidarr_block == ActiveLidarrBlock::RootFolders {
      self
        .app
        .push_navigation_stack(ActiveLidarrBlock::DeleteRootFolderPrompt.into())
    }
  }

  fn handle_left_right_action(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::RootFolders => handle_change_tab_left_right_keys(self.app, self.key),
      ActiveLidarrBlock::DeleteRootFolderPrompt => handle_prompt_toggle(self.app, self.key),
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    if self.active_lidarr_block == ActiveLidarrBlock::DeleteRootFolderPrompt {
      if self.app.data.lidarr_data.prompt_confirm {
        self.app.data.lidarr_data.prompt_confirm_action =
          Some(LidarrEvent::DeleteRootFolder(self.extract_root_folder_id()));
      }

      self.app.pop_navigation_stack();
    }
  }

  fn handle_esc(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::DeleteRootFolderPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.lidarr_data.prompt_confirm = false;
      }
      _ => handle_clear_errors(self.app),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_lidarr_block {
      ActiveLidarrBlock::RootFolders => match self.key {
        _ if matches_key!(refresh, key) => {
          self.app.should_refresh = true;
        }
        _ if matches_key!(add, key) => {
          self.app.data.lidarr_data.add_root_folder_modal =
            Some(AddRootFolderModal::from(&self.app.data.lidarr_data));
          self.app.data.lidarr_data.selected_block =
            BlockSelectionState::new(ADD_ROOT_FOLDER_SELECTION_BLOCKS);
          self
            .app
            .push_navigation_stack(ActiveLidarrBlock::AddRootFolderPrompt.into());
        }
        _ => (),
      },
      ActiveLidarrBlock::DeleteRootFolderPrompt => {
        if matches_key!(confirm, key) {
          self.app.data.lidarr_data.prompt_confirm = true;
          self.app.data.lidarr_data.prompt_confirm_action =
            Some(LidarrEvent::DeleteRootFolder(self.extract_root_folder_id()));

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
