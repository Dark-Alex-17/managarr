use crate::app::App;
use crate::event::Key;
use crate::handlers::lidarr_handlers::handle_change_tab_left_right_keys;
use crate::handlers::table_handler::{TableHandlingConfig, handle_table};
use crate::handlers::{KeyEventHandler, handle_clear_errors, handle_prompt_toggle};
use crate::models::servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, ROOT_FOLDERS_BLOCKS};
use crate::models::servarr_models::AddRootFolderBody;
use crate::models::{HorizontallyScrollableText, Route};
use crate::network::lidarr_network::LidarrEvent;
use crate::{handle_text_box_keys, handle_text_box_left_right_keys, matches_key};

#[cfg(test)]
#[path = "root_folders_handler_tests.rs"]
mod root_folders_handler_tests;

pub(super) struct RootFoldersHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_lidarr_block: ActiveLidarrBlock,
  _context: Option<ActiveLidarrBlock>,
}

impl RootFoldersHandler<'_, '_> {
  fn build_add_root_folder_body(&mut self) -> AddRootFolderBody {
    let root_folder = self
      .app
      .data
      .lidarr_data
      .edit_root_folder
      .take()
      .expect("EditRootFolder is None")
      .text;
    AddRootFolderBody { path: root_folder }
  }

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

    if !handle_table(
      self,
      |app| &mut app.data.lidarr_data.root_folders,
      root_folders_table_handling_config,
    ) {
      self.handle_key_event();
    }
  }

  fn accepts(active_block: ActiveLidarrBlock) -> bool {
    ROOT_FOLDERS_BLOCKS.contains(&active_block)
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveLidarrBlock,
    _context: Option<ActiveLidarrBlock>,
  ) -> RootFoldersHandler<'a, 'b> {
    RootFoldersHandler {
      key,
      app,
      active_lidarr_block: active_block,
      _context,
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

  fn handle_home(&mut self) {
    if self.active_lidarr_block == ActiveLidarrBlock::AddRootFolderPrompt {
      self
        .app
        .data
        .lidarr_data
        .edit_root_folder
        .as_mut()
        .unwrap()
        .scroll_home()
    }
  }

  fn handle_end(&mut self) {
    if self.active_lidarr_block == ActiveLidarrBlock::AddRootFolderPrompt {
      self
        .app
        .data
        .lidarr_data
        .edit_root_folder
        .as_mut()
        .unwrap()
        .reset_offset()
    }
  }

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
      ActiveLidarrBlock::AddRootFolderPrompt => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self.app.data.lidarr_data.edit_root_folder.as_mut().unwrap()
        )
      }
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::DeleteRootFolderPrompt => {
        if self.app.data.lidarr_data.prompt_confirm {
          self.app.data.lidarr_data.prompt_confirm_action =
            Some(LidarrEvent::DeleteRootFolder(self.extract_root_folder_id()));
        }

        self.app.pop_navigation_stack();
      }
      _ if self.active_lidarr_block == ActiveLidarrBlock::AddRootFolderPrompt
        && !self
          .app
          .data
          .lidarr_data
          .edit_root_folder
          .as_ref()
          .unwrap()
          .text
          .is_empty() =>
      {
        self.app.data.lidarr_data.prompt_confirm_action = Some(LidarrEvent::AddRootFolder(
          self.build_add_root_folder_body(),
        ));
        self.app.data.lidarr_data.prompt_confirm = true;
        self.app.ignore_special_keys_for_textbox_input = false;
        self.app.pop_navigation_stack();
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::AddRootFolderPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.lidarr_data.edit_root_folder = None;
        self.app.data.lidarr_data.prompt_confirm = false;
        self.app.ignore_special_keys_for_textbox_input = false;
      }
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
          self
            .app
            .push_navigation_stack(ActiveLidarrBlock::AddRootFolderPrompt.into());
          self.app.data.lidarr_data.edit_root_folder = Some(HorizontallyScrollableText::default());
          self.app.ignore_special_keys_for_textbox_input = true;
        }
        _ => (),
      },
      ActiveLidarrBlock::AddRootFolderPrompt => {
        handle_text_box_keys!(
          self,
          key,
          self.app.data.lidarr_data.edit_root_folder.as_mut().unwrap()
        )
      }
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
