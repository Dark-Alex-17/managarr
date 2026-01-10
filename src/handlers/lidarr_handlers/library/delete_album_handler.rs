use crate::models::Route;
use crate::models::lidarr_models::DeleteParams;
use crate::models::servarr_data::lidarr::lidarr_data::DELETE_ALBUM_BLOCKS;
use crate::network::lidarr_network::LidarrEvent;
use crate::{
  app::App,
  event::Key,
  handlers::{KeyEventHandler, handle_prompt_toggle},
  matches_key,
  models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock,
};

#[cfg(test)]
#[path = "delete_album_handler_tests.rs"]
mod delete_album_handler_tests;

pub(in crate::handlers::lidarr_handlers) struct DeleteAlbumHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_lidarr_block: ActiveLidarrBlock,
  _context: Option<ActiveLidarrBlock>,
}

impl DeleteAlbumHandler<'_, '_> {
  fn build_delete_album_params(&mut self) -> DeleteParams {
    let id = self.app.data.lidarr_data.albums.current_selection().id;
    let delete_files = self.app.data.lidarr_data.delete_files;
    let add_import_list_exclusion = self.app.data.lidarr_data.add_import_list_exclusion;
    self.app.data.lidarr_data.reset_delete_preferences();

    DeleteParams {
      id,
      delete_files,
      add_import_list_exclusion,
    }
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveLidarrBlock> for DeleteAlbumHandler<'a, 'b> {
  fn accepts(active_block: ActiveLidarrBlock) -> bool {
    DELETE_ALBUM_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveLidarrBlock,
    _context: Option<ActiveLidarrBlock>,
  ) -> Self {
    DeleteAlbumHandler {
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
    !self.app.is_loading
  }

  fn handle_scroll_up(&mut self) {
    if self.active_lidarr_block == ActiveLidarrBlock::DeleteAlbumPrompt {
      self.app.data.lidarr_data.selected_block.up();
    }
  }

  fn handle_scroll_down(&mut self) {
    if self.active_lidarr_block == ActiveLidarrBlock::DeleteAlbumPrompt {
      self.app.data.lidarr_data.selected_block.down();
    }
  }

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    if self.active_lidarr_block == ActiveLidarrBlock::DeleteAlbumPrompt {
      handle_prompt_toggle(self.app, self.key);
    }
  }

  fn handle_submit(&mut self) {
    if self.active_lidarr_block == ActiveLidarrBlock::DeleteAlbumPrompt {
      match self.app.data.lidarr_data.selected_block.get_active_block() {
        ActiveLidarrBlock::DeleteAlbumConfirmPrompt => {
          if self.app.data.lidarr_data.prompt_confirm {
            self.app.data.lidarr_data.prompt_confirm_action =
              Some(LidarrEvent::DeleteAlbum(self.build_delete_album_params()));
            self.app.should_refresh = true;
          } else {
            self.app.data.lidarr_data.reset_delete_preferences();
          }

          self.app.pop_navigation_stack();
        }
        ActiveLidarrBlock::DeleteAlbumToggleDeleteFile => {
          self.app.data.lidarr_data.delete_files = !self.app.data.lidarr_data.delete_files;
        }
        ActiveLidarrBlock::DeleteAlbumToggleAddListExclusion => {
          self.app.data.lidarr_data.add_import_list_exclusion =
            !self.app.data.lidarr_data.add_import_list_exclusion;
        }
        _ => (),
      }
    }
  }

  fn handle_esc(&mut self) {
    if self.active_lidarr_block == ActiveLidarrBlock::DeleteAlbumPrompt {
      self.app.pop_navigation_stack();
      self.app.data.lidarr_data.reset_delete_preferences();
      self.app.data.lidarr_data.prompt_confirm = false;
    }
  }

  fn handle_char_key_event(&mut self) {
    if self.active_lidarr_block == ActiveLidarrBlock::DeleteAlbumPrompt
      && self.app.data.lidarr_data.selected_block.get_active_block()
        == ActiveLidarrBlock::DeleteAlbumConfirmPrompt
      && matches_key!(confirm, self.key)
    {
      self.app.data.lidarr_data.prompt_confirm = true;
      self.app.data.lidarr_data.prompt_confirm_action =
        Some(LidarrEvent::DeleteAlbum(self.build_delete_album_params()));
      self.app.should_refresh = true;

      self.app.pop_navigation_stack();
    }
  }

  fn app_mut(&mut self) -> &mut App<'b> {
    self.app
  }

  fn current_route(&self) -> Route {
    self.app.get_current_route()
  }
}
