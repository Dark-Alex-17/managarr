use crate::models::lidarr_models::DeleteArtistParams;
use crate::network::lidarr_network::LidarrEvent;
use crate::{
  app::App,
  event::Key,
  handlers::{KeyEventHandler, handle_prompt_toggle},
  matches_key,
  models::servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, DELETE_ARTIST_BLOCKS},
};
use crate::models::Route;

#[cfg(test)]
#[path = "delete_artist_handler_tests.rs"]
mod delete_artist_handler_tests;

pub(in crate::handlers::lidarr_handlers) struct DeleteArtistHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_lidarr_block: ActiveLidarrBlock,
  _context: Option<ActiveLidarrBlock>,
}

impl DeleteArtistHandler<'_, '_> {
  fn build_delete_artist_params(&mut self) -> DeleteArtistParams {
    let id = self.app.data.lidarr_data.artists.current_selection().id;
    let delete_files = self.app.data.lidarr_data.delete_artist_files;
    let add_import_list_exclusion = self.app.data.lidarr_data.add_import_list_exclusion;
    self.app.data.lidarr_data.reset_delete_artist_preferences();

    DeleteArtistParams {
      id,
      delete_files,
      add_import_list_exclusion,
    }
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveLidarrBlock> for DeleteArtistHandler<'a, 'b> {
  fn accepts(active_block: ActiveLidarrBlock) -> bool {
    DELETE_ARTIST_BLOCKS.contains(&active_block)
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
    DeleteArtistHandler {
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
    if self.active_lidarr_block == ActiveLidarrBlock::DeleteArtistPrompt {
      self.app.data.lidarr_data.selected_block.up();
    }
  }

  fn handle_scroll_down(&mut self) {
    if self.active_lidarr_block == ActiveLidarrBlock::DeleteArtistPrompt {
      self.app.data.lidarr_data.selected_block.down();
    }
  }

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    if self.active_lidarr_block == ActiveLidarrBlock::DeleteArtistPrompt {
      handle_prompt_toggle(self.app, self.key);
    }
  }

  fn handle_submit(&mut self) {
    if self.active_lidarr_block == ActiveLidarrBlock::DeleteArtistPrompt {
      match self.app.data.lidarr_data.selected_block.get_active_block() {
        ActiveLidarrBlock::DeleteArtistConfirmPrompt => {
          if self.app.data.lidarr_data.prompt_confirm {
            self.app.data.lidarr_data.prompt_confirm_action =
              Some(LidarrEvent::DeleteArtist(self.build_delete_artist_params()));
            self.app.should_refresh = true;
          } else {
            self.app.data.lidarr_data.reset_delete_artist_preferences();
          }

          self.app.pop_navigation_stack();
        }
        ActiveLidarrBlock::DeleteArtistToggleDeleteFile => {
          self.app.data.lidarr_data.delete_artist_files =
            !self.app.data.lidarr_data.delete_artist_files;
        }
        ActiveLidarrBlock::DeleteArtistToggleAddListExclusion => {
          self.app.data.lidarr_data.add_import_list_exclusion =
            !self.app.data.lidarr_data.add_import_list_exclusion;
        }
        _ => (),
      }
    }
  }

  fn handle_esc(&mut self) {
    if self.active_lidarr_block == ActiveLidarrBlock::DeleteArtistPrompt {
      self.app.pop_navigation_stack();
      self.app.data.lidarr_data.reset_delete_artist_preferences();
      self.app.data.lidarr_data.prompt_confirm = false;
    }
  }

  fn handle_char_key_event(&mut self) {
    if self.active_lidarr_block == ActiveLidarrBlock::DeleteArtistPrompt
      && self.app.data.lidarr_data.selected_block.get_active_block()
        == ActiveLidarrBlock::DeleteArtistConfirmPrompt
      && matches_key!(confirm, self.key)
    {
      self.app.data.lidarr_data.prompt_confirm = true;
      self.app.data.lidarr_data.prompt_confirm_action =
        Some(LidarrEvent::DeleteArtist(self.build_delete_artist_params()));
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
