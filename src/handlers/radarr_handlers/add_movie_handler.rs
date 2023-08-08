use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::radarr::ActiveRadarrBlock;
use crate::handlers::{handle_prompt_toggle, KeyEventHandler};
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
    if self.active_radarr_block == &ActiveRadarrBlock::AddMovieSearchResults {
      self.app.data.radarr_data.add_searched_movies.scroll_up()
    }
  }

  fn handle_scroll_down(&mut self) {
    if self.active_radarr_block == &ActiveRadarrBlock::AddMovieSearchResults {
      self.app.data.radarr_data.add_searched_movies.scroll_down()
    }
  }

  fn handle_home(&mut self) {
    if self.active_radarr_block == &ActiveRadarrBlock::AddMovieSearchResults {
      self
        .app
        .data
        .radarr_data
        .add_searched_movies
        .scroll_to_top()
    }
  }

  fn handle_end(&mut self) {
    if self.active_radarr_block == &ActiveRadarrBlock::AddMovieSearchResults {
      self
        .app
        .data
        .radarr_data
        .add_searched_movies
        .scroll_to_bottom()
    }
  }

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    if *self.active_radarr_block == ActiveRadarrBlock::AddMoviePrompt {
      handle_prompt_toggle(self.app, self.key);
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
      }
      ActiveRadarrBlock::AddMoviePrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::AddMovie);
          self.app.pop_navigation_stack();
        } else {
          self.app.pop_navigation_stack();
        }
      }
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
        self.app.data.radarr_data.prompt_confirm = false;
      }
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
