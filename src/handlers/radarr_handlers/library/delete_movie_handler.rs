use crate::app::App;
use crate::event::Key;
use crate::handlers::{KeyEventHandler, handle_prompt_toggle};
use crate::matches_key;
use crate::models::Route;
use crate::models::radarr_models::DeleteMovieParams;
use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, DELETE_MOVIE_BLOCKS};
use crate::network::radarr_network::RadarrEvent;

#[cfg(test)]
#[path = "delete_movie_handler_tests.rs"]
mod delete_movie_handler_tests;

pub(super) struct DeleteMovieHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_radarr_block: ActiveRadarrBlock,
  _context: Option<ActiveRadarrBlock>,
}

impl DeleteMovieHandler<'_, '_> {
  fn build_delete_movie_params(&mut self) -> DeleteMovieParams {
    let id = self.app.data.radarr_data.movies.current_selection().id;
    let delete_movie_files = self.app.data.radarr_data.delete_movie_files;
    let add_list_exclusion = self.app.data.radarr_data.add_list_exclusion;
    self.app.data.radarr_data.reset_delete_movie_preferences();

    DeleteMovieParams {
      id,
      delete_movie_files,
      add_list_exclusion,
    }
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for DeleteMovieHandler<'a, 'b> {
  fn accepts(active_block: ActiveRadarrBlock) -> bool {
    DELETE_MOVIE_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveRadarrBlock,
    _context: Option<ActiveRadarrBlock>,
  ) -> Self {
    DeleteMovieHandler {
      key,
      app,
      active_radarr_block: active_block,
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
    if self.active_radarr_block == ActiveRadarrBlock::DeleteMoviePrompt {
      self.app.data.radarr_data.selected_block.up();
    }
  }

  fn handle_scroll_down(&mut self) {
    if self.active_radarr_block == ActiveRadarrBlock::DeleteMoviePrompt {
      self.app.data.radarr_data.selected_block.down();
    }
  }

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    if self.active_radarr_block == ActiveRadarrBlock::DeleteMoviePrompt {
      handle_prompt_toggle(self.app, self.key);
    }
  }

  fn handle_submit(&mut self) {
    if self.active_radarr_block == ActiveRadarrBlock::DeleteMoviePrompt {
      match self.app.data.radarr_data.selected_block.get_active_block() {
        ActiveRadarrBlock::DeleteMovieConfirmPrompt => {
          if self.app.data.radarr_data.prompt_confirm {
            self.app.data.radarr_data.prompt_confirm_action =
              Some(RadarrEvent::DeleteMovie(self.build_delete_movie_params()));
            self.app.should_refresh = true;
          } else {
            self.app.data.radarr_data.reset_delete_movie_preferences();
          }

          self.app.pop_navigation_stack();
        }
        ActiveRadarrBlock::DeleteMovieToggleDeleteFile => {
          self.app.data.radarr_data.delete_movie_files =
            !self.app.data.radarr_data.delete_movie_files;
        }
        ActiveRadarrBlock::DeleteMovieToggleAddListExclusion => {
          self.app.data.radarr_data.add_list_exclusion =
            !self.app.data.radarr_data.add_list_exclusion;
        }
        _ => (),
      }
    }
  }

  fn handle_esc(&mut self) {
    if self.active_radarr_block == ActiveRadarrBlock::DeleteMoviePrompt {
      self.app.pop_navigation_stack();
      self.app.data.radarr_data.reset_delete_movie_preferences();
      self.app.data.radarr_data.prompt_confirm = false;
    }
  }

  fn handle_char_key_event(&mut self) {
    if self.active_radarr_block == ActiveRadarrBlock::DeleteMoviePrompt
      && self.app.data.radarr_data.selected_block.get_active_block()
        == ActiveRadarrBlock::DeleteMovieConfirmPrompt
      && matches_key!(confirm, self.key)
    {
      self.app.data.radarr_data.prompt_confirm = true;
      self.app.data.radarr_data.prompt_confirm_action =
        Some(RadarrEvent::DeleteMovie(self.build_delete_movie_params()));
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
