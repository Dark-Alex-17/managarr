use crate::app::App;
use crate::event::Key;
use crate::handlers::{handle_prompt_toggle, KeyEventHandler};
use crate::models::servarr_data::radarr_data::{ActiveRadarrBlock, DELETE_MOVIE_BLOCKS};
use crate::network::radarr_network::RadarrEvent;

#[cfg(test)]
#[path = "delete_movie_handler_tests.rs"]
mod delete_movie_handler_tests;

pub(super) struct DeleteMovieHandler<'a, 'b> {
  key: &'a Key,
  app: &'a mut App<'b>,
  active_radarr_block: &'a ActiveRadarrBlock,
  _context: &'a Option<ActiveRadarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for DeleteMovieHandler<'a, 'b> {
  fn accepts(active_block: &'a ActiveRadarrBlock) -> bool {
    DELETE_MOVIE_BLOCKS.contains(active_block)
  }

  fn with(
    key: &'a Key,
    app: &'a mut App<'b>,
    active_block: &'a ActiveRadarrBlock,
    _context: &'a Option<ActiveRadarrBlock>,
  ) -> Self {
    DeleteMovieHandler {
      key,
      app,
      active_radarr_block: active_block,
      _context,
    }
  }

  fn get_key(&self) -> &Key {
    self.key
  }

  fn handle_scroll_up(&mut self) {
    if *self.active_radarr_block == ActiveRadarrBlock::DeleteMoviePrompt {
      self.app.data.radarr_data.selected_block.previous();
    }
  }

  fn handle_scroll_down(&mut self) {
    if *self.active_radarr_block == ActiveRadarrBlock::DeleteMoviePrompt {
      self.app.data.radarr_data.selected_block.next();
    }
  }

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    if *self.active_radarr_block == ActiveRadarrBlock::DeleteMoviePrompt {
      handle_prompt_toggle(self.app, self.key);
    }
  }

  fn handle_submit(&mut self) {
    if self.active_radarr_block == &ActiveRadarrBlock::DeleteMoviePrompt {
      match self.app.data.radarr_data.selected_block.get_active_block() {
        ActiveRadarrBlock::DeleteMovieConfirmPrompt => {
          if self.app.data.radarr_data.prompt_confirm {
            self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::DeleteMovie);
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
    if *self.active_radarr_block == ActiveRadarrBlock::DeleteMoviePrompt {
      self.app.pop_navigation_stack();
      self.app.data.radarr_data.reset_delete_movie_preferences();
      self.app.data.radarr_data.prompt_confirm = false;
    }
  }

  fn handle_char_key_event(&mut self) {}
}
