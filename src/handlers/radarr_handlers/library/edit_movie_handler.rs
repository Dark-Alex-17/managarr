use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::App;
use crate::event::Key;
use crate::handlers::{handle_prompt_toggle, KeyEventHandler};
use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, EDIT_MOVIE_BLOCKS};
use crate::models::Scrollable;
use crate::network::radarr_network::RadarrEvent;
use crate::{handle_text_box_keys, handle_text_box_left_right_keys};

#[cfg(test)]
#[path = "edit_movie_handler_tests.rs"]
mod edit_movie_handler_tests;

pub(super) struct EditMovieHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_radarr_block: ActiveRadarrBlock,
  context: Option<ActiveRadarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for EditMovieHandler<'a, 'b> {
  fn accepts(active_block: ActiveRadarrBlock) -> bool {
    EDIT_MOVIE_BLOCKS.contains(&active_block)
  }

  fn with(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveRadarrBlock,
    context: Option<ActiveRadarrBlock>,
  ) -> EditMovieHandler<'a, 'b> {
    EditMovieHandler {
      key,
      app,
      active_radarr_block: active_block,
      context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    !self.app.is_loading && self.app.data.radarr_data.edit_movie_modal.is_some()
  }

  fn handle_scroll_up(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::EditMovieSelectMinimumAvailability => self
        .app
        .data
        .radarr_data
        .edit_movie_modal
        .as_mut()
        .unwrap()
        .minimum_availability_list
        .scroll_up(),
      ActiveRadarrBlock::EditMovieSelectQualityProfile => self
        .app
        .data
        .radarr_data
        .edit_movie_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_up(),
      ActiveRadarrBlock::EditMoviePrompt => self.app.data.radarr_data.selected_block.up(),
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::EditMovieSelectMinimumAvailability => self
        .app
        .data
        .radarr_data
        .edit_movie_modal
        .as_mut()
        .unwrap()
        .minimum_availability_list
        .scroll_down(),
      ActiveRadarrBlock::EditMovieSelectQualityProfile => self
        .app
        .data
        .radarr_data
        .edit_movie_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_down(),
      ActiveRadarrBlock::EditMoviePrompt => self.app.data.radarr_data.selected_block.down(),
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::EditMovieSelectMinimumAvailability => self
        .app
        .data
        .radarr_data
        .edit_movie_modal
        .as_mut()
        .unwrap()
        .minimum_availability_list
        .scroll_to_top(),
      ActiveRadarrBlock::EditMovieSelectQualityProfile => self
        .app
        .data
        .radarr_data
        .edit_movie_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_to_top(),
      ActiveRadarrBlock::EditMoviePathInput => self
        .app
        .data
        .radarr_data
        .edit_movie_modal
        .as_mut()
        .unwrap()
        .path
        .scroll_home(),
      ActiveRadarrBlock::EditMovieTagsInput => self
        .app
        .data
        .radarr_data
        .edit_movie_modal
        .as_mut()
        .unwrap()
        .tags
        .scroll_home(),
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::EditMovieSelectMinimumAvailability => self
        .app
        .data
        .radarr_data
        .edit_movie_modal
        .as_mut()
        .unwrap()
        .minimum_availability_list
        .scroll_to_bottom(),
      ActiveRadarrBlock::EditMovieSelectQualityProfile => self
        .app
        .data
        .radarr_data
        .edit_movie_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_to_bottom(),
      ActiveRadarrBlock::EditMoviePathInput => self
        .app
        .data
        .radarr_data
        .edit_movie_modal
        .as_mut()
        .unwrap()
        .path
        .reset_offset(),
      ActiveRadarrBlock::EditMovieTagsInput => self
        .app
        .data
        .radarr_data
        .edit_movie_modal
        .as_mut()
        .unwrap()
        .tags
        .reset_offset(),
      _ => (),
    }
  }

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::EditMoviePrompt => handle_prompt_toggle(self.app, self.key),
      ActiveRadarrBlock::EditMoviePathInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .radarr_data
            .edit_movie_modal
            .as_mut()
            .unwrap()
            .path
        )
      }
      ActiveRadarrBlock::EditMovieTagsInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .radarr_data
            .edit_movie_modal
            .as_mut()
            .unwrap()
            .tags
        )
      }
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::EditMoviePrompt => {
        match self.app.data.radarr_data.selected_block.get_active_block() {
          ActiveRadarrBlock::EditMovieConfirmPrompt => {
            if self.app.data.radarr_data.prompt_confirm {
              self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::EditMovie(None));
              self.app.should_refresh = true;
            }

            self.app.pop_navigation_stack();
          }
          ActiveRadarrBlock::EditMovieSelectMinimumAvailability
          | ActiveRadarrBlock::EditMovieSelectQualityProfile => self.app.push_navigation_stack(
            (
              self.app.data.radarr_data.selected_block.get_active_block(),
              self.context,
            )
              .into(),
          ),
          ActiveRadarrBlock::EditMoviePathInput | ActiveRadarrBlock::EditMovieTagsInput => {
            self.app.push_navigation_stack(
              (
                self.app.data.radarr_data.selected_block.get_active_block(),
                self.context,
              )
                .into(),
            );
            self.app.should_ignore_quit_key = true;
          }
          ActiveRadarrBlock::EditMovieToggleMonitored => {
            self
              .app
              .data
              .radarr_data
              .edit_movie_modal
              .as_mut()
              .unwrap()
              .monitored = Some(
              !self
                .app
                .data
                .radarr_data
                .edit_movie_modal
                .as_mut()
                .unwrap()
                .monitored
                .unwrap_or_default(),
            )
          }
          _ => (),
        }
      }
      ActiveRadarrBlock::EditMovieSelectMinimumAvailability
      | ActiveRadarrBlock::EditMovieSelectQualityProfile => self.app.pop_navigation_stack(),
      ActiveRadarrBlock::EditMoviePathInput | ActiveRadarrBlock::EditMovieTagsInput => {
        self.app.pop_navigation_stack();
        self.app.should_ignore_quit_key = false;
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::EditMovieTagsInput | ActiveRadarrBlock::EditMoviePathInput => {
        self.app.pop_navigation_stack();
        self.app.should_ignore_quit_key = false;
      }
      ActiveRadarrBlock::EditMoviePrompt => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.edit_movie_modal = None;
        self.app.data.radarr_data.prompt_confirm = false;
      }
      ActiveRadarrBlock::EditMovieSelectMinimumAvailability
      | ActiveRadarrBlock::EditMovieSelectQualityProfile => self.app.pop_navigation_stack(),
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_radarr_block {
      ActiveRadarrBlock::EditMoviePathInput => {
        handle_text_box_keys!(
          self,
          key,
          self
            .app
            .data
            .radarr_data
            .edit_movie_modal
            .as_mut()
            .unwrap()
            .path
        )
      }
      ActiveRadarrBlock::EditMovieTagsInput => {
        handle_text_box_keys!(
          self,
          key,
          self
            .app
            .data
            .radarr_data
            .edit_movie_modal
            .as_mut()
            .unwrap()
            .tags
        )
      }
      ActiveRadarrBlock::EditMoviePrompt => {
        if self.app.data.radarr_data.selected_block.get_active_block()
          == ActiveRadarrBlock::EditMovieConfirmPrompt
          && key == DEFAULT_KEYBINDINGS.confirm.key
        {
          self.app.data.radarr_data.prompt_confirm = true;
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::EditMovie(None));
          self.app.should_refresh = true;

          self.app.pop_navigation_stack();
        }
      }
      _ => (),
    }
  }
}
