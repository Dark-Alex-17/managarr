use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::radarr::ActiveRadarrBlock;
use crate::handlers::{handle_prompt_toggle, KeyEventHandler};
use crate::models::radarr_models::{MinimumAvailability, Monitor};
use crate::models::{Scrollable, StatefulTable};
use crate::network::radarr_network::RadarrEvent;
use crate::{App, Key};

pub(super) struct AddMovieHandler<'a> {
  key: &'a Key,
  app: &'a mut App,
  active_radarr_block: &'a ActiveRadarrBlock,
}

impl<'a> KeyEventHandler<'a, ActiveRadarrBlock> for AddMovieHandler<'a> {
  fn with(
    key: &'a Key,
    app: &'a mut App,
    active_block: &'a ActiveRadarrBlock,
  ) -> AddMovieHandler<'a> {
    AddMovieHandler {
      key,
      app,
      active_radarr_block: active_block,
    }
  }

  fn get_key(&self) -> &Key {
    self.key
  }

  fn handle_scroll_up(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::AddMovieSearchResults => {
        self.app.data.radarr_data.add_searched_movies.scroll_up()
      }
      ActiveRadarrBlock::AddMovieSelectMonitor => {
        self.app.data.radarr_data.add_movie_monitor_list.scroll_up()
      }
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability => self
        .app
        .data
        .radarr_data
        .add_movie_minimum_availability_list
        .scroll_up(),
      ActiveRadarrBlock::AddMovieSelectQualityProfile => self
        .app
        .data
        .radarr_data
        .add_movie_quality_profile_list
        .scroll_up(),
      ActiveRadarrBlock::AddMoviePrompt => {
        self.app.data.radarr_data.selected_block = self
          .app
          .data
          .radarr_data
          .selected_block
          .clone()
          .previous_add_prompt_block()
      }
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::AddMovieSearchResults => {
        self.app.data.radarr_data.add_searched_movies.scroll_down()
      }
      ActiveRadarrBlock::AddMovieSelectMonitor => self
        .app
        .data
        .radarr_data
        .add_movie_monitor_list
        .scroll_down(),
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability => self
        .app
        .data
        .radarr_data
        .add_movie_minimum_availability_list
        .scroll_down(),
      ActiveRadarrBlock::AddMovieSelectQualityProfile => self
        .app
        .data
        .radarr_data
        .add_movie_quality_profile_list
        .scroll_down(),
      ActiveRadarrBlock::AddMoviePrompt => {
        self.app.data.radarr_data.selected_block = self
          .app
          .data
          .radarr_data
          .selected_block
          .next_add_prompt_block()
      }
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::AddMovieSearchResults => self
        .app
        .data
        .radarr_data
        .add_searched_movies
        .scroll_to_top(),
      ActiveRadarrBlock::AddMovieSelectMonitor => self
        .app
        .data
        .radarr_data
        .add_movie_monitor_list
        .scroll_to_top(),
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability => self
        .app
        .data
        .radarr_data
        .add_movie_minimum_availability_list
        .scroll_to_top(),
      ActiveRadarrBlock::AddMovieSelectQualityProfile => self
        .app
        .data
        .radarr_data
        .add_movie_quality_profile_list
        .scroll_to_top(),
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::AddMovieSearchResults => self
        .app
        .data
        .radarr_data
        .add_searched_movies
        .scroll_to_bottom(),
      ActiveRadarrBlock::AddMovieSelectMonitor => self
        .app
        .data
        .radarr_data
        .add_movie_monitor_list
        .scroll_to_bottom(),
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability => self
        .app
        .data
        .radarr_data
        .add_movie_minimum_availability_list
        .scroll_to_bottom(),
      ActiveRadarrBlock::AddMovieSelectQualityProfile => self
        .app
        .data
        .radarr_data
        .add_movie_quality_profile_list
        .scroll_to_bottom(),
      _ => (),
    }
  }

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    if let ActiveRadarrBlock::AddMoviePrompt = self.active_radarr_block {
      handle_prompt_toggle(self.app, self.key)
    }
  }

  fn handle_submit(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::AddMovieSearchInput => {
        self
          .app
          .push_navigation_stack(ActiveRadarrBlock::AddMovieSearchResults.into());
        self.app.should_ignore_quit_key = false;
      }
      ActiveRadarrBlock::AddMovieSearchResults => {
        self
          .app
          .push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());
        self
          .app
          .data
          .radarr_data
          .add_movie_monitor_list
          .set_items(Monitor::vec());
        self
          .app
          .data
          .radarr_data
          .add_movie_minimum_availability_list
          .set_items(MinimumAvailability::vec());
        let mut quality_profile_names: Vec<String> = self
          .app
          .data
          .radarr_data
          .quality_profile_map
          .values()
          .cloned()
          .collect();
        quality_profile_names.sort();
        self
          .app
          .data
          .radarr_data
          .add_movie_quality_profile_list
          .set_items(quality_profile_names);
      }
      ActiveRadarrBlock::AddMoviePrompt => match self.app.data.radarr_data.selected_block {
        ActiveRadarrBlock::AddMovieConfirmPrompt => {
          if self.app.data.radarr_data.prompt_confirm {
            self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::AddMovie);
            self.app.pop_navigation_stack();
          } else {
            self.app.pop_navigation_stack();
          }
        }
        ActiveRadarrBlock::AddMovieSelectMonitor => self
          .app
          .push_navigation_stack(ActiveRadarrBlock::AddMovieSelectMonitor.into()),
        ActiveRadarrBlock::AddMovieSelectMinimumAvailability => self
          .app
          .push_navigation_stack(ActiveRadarrBlock::AddMovieSelectMinimumAvailability.into()),
        ActiveRadarrBlock::AddMovieSelectQualityProfile => self
          .app
          .push_navigation_stack(ActiveRadarrBlock::AddMovieSelectQualityProfile.into()),
        _ => (),
      },
      ActiveRadarrBlock::AddMovieSelectMonitor
      | ActiveRadarrBlock::AddMovieSelectMinimumAvailability
      | ActiveRadarrBlock::AddMovieSelectQualityProfile => self.app.pop_navigation_stack(),
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::AddMovieSearchInput => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.reset_search();
        self.app.should_ignore_quit_key = false;
      }
      ActiveRadarrBlock::AddMovieSearchResults => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.add_searched_movies = StatefulTable::default();
        self.app.should_ignore_quit_key = true;
      }
      ActiveRadarrBlock::AddMoviePrompt => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.reset_add_movie_selections();
        self.app.data.radarr_data.prompt_confirm = false;
      }
      ActiveRadarrBlock::AddMovieSelectMonitor
      | ActiveRadarrBlock::AddMovieSelectMinimumAvailability
      | ActiveRadarrBlock::AddMovieSelectQualityProfile => self.app.pop_navigation_stack(),
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    if self.active_radarr_block == &ActiveRadarrBlock::AddMovieSearchInput {
      match self.key {
        _ if *key == DEFAULT_KEYBINDINGS.backspace.key => {
          self.app.data.radarr_data.search.pop();
        }
        Key::Char(character) => {
          self.app.data.radarr_data.search.push(*character);
        }
        _ => (),
      }
    }
  }
}
