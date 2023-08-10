use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::App;
use crate::event::Key;
use crate::handlers::radarr_handlers::handle_change_tab_left_right_keys;
use crate::handlers::radarr_handlers::library::add_movie_handler::AddMovieHandler;
use crate::handlers::radarr_handlers::library::delete_movie_handler::DeleteMovieHandler;
use crate::handlers::radarr_handlers::library::edit_movie_handler::EditMovieHandler;
use crate::handlers::radarr_handlers::library::movie_details_handler::MovieDetailsHandler;
use crate::handlers::{handle_clear_errors, handle_prompt_toggle, KeyEventHandler};

use crate::models::servarr_data::radarr::radarr_data::{
  ActiveRadarrBlock, DELETE_MOVIE_SELECTION_BLOCKS, EDIT_MOVIE_SELECTION_BLOCKS, LIBRARY_BLOCKS,
};
use crate::models::{BlockSelectionState, HorizontallyScrollableText, Scrollable};
use crate::network::radarr_network::RadarrEvent;
use crate::utils::strip_non_search_characters;
use crate::{filter_table, handle_text_box_keys, handle_text_box_left_right_keys, search_table};

mod add_movie_handler;
mod delete_movie_handler;
mod edit_movie_handler;
mod movie_details_handler;

#[cfg(test)]
#[path = "library_handler_tests.rs"]
mod library_handler_tests;

pub(super) struct LibraryHandler<'a, 'b> {
  key: &'a Key,
  app: &'a mut App<'b>,
  active_radarr_block: &'a ActiveRadarrBlock,
  context: &'a Option<ActiveRadarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for LibraryHandler<'a, 'b> {
  fn handle(&mut self) {
    match self.active_radarr_block {
      _ if AddMovieHandler::accepts(self.active_radarr_block) => {
        AddMovieHandler::with(self.key, self.app, self.active_radarr_block, self.context).handle();
      }
      _ if DeleteMovieHandler::accepts(self.active_radarr_block) => {
        DeleteMovieHandler::with(self.key, self.app, self.active_radarr_block, self.context)
          .handle();
      }
      _ if EditMovieHandler::accepts(self.active_radarr_block) => {
        EditMovieHandler::with(self.key, self.app, self.active_radarr_block, self.context).handle();
      }
      _ if MovieDetailsHandler::accepts(self.active_radarr_block) => {
        MovieDetailsHandler::with(self.key, self.app, self.active_radarr_block, self.context)
          .handle();
      }
      _ => self.handle_key_event(),
    }
  }

  fn accepts(active_block: &'a ActiveRadarrBlock) -> bool {
    AddMovieHandler::accepts(active_block)
      || DeleteMovieHandler::accepts(active_block)
      || EditMovieHandler::accepts(active_block)
      || MovieDetailsHandler::accepts(active_block)
      || LIBRARY_BLOCKS.contains(active_block)
  }

  fn with(
    key: &'a Key,
    app: &'a mut App<'b>,
    active_block: &'a ActiveRadarrBlock,
    context: &'a Option<ActiveRadarrBlock>,
  ) -> LibraryHandler<'a, 'b> {
    LibraryHandler {
      key,
      app,
      active_radarr_block: active_block,
      context,
    }
  }

  fn get_key(&self) -> &Key {
    self.key
  }

  fn handle_scroll_up(&mut self) {
    if self.active_radarr_block == &ActiveRadarrBlock::Movies {
      if !self.app.data.radarr_data.filtered_movies.items.is_empty() {
        self.app.data.radarr_data.filtered_movies.scroll_up();
      } else {
        self.app.data.radarr_data.movies.scroll_up()
      }
    }
  }

  fn handle_scroll_down(&mut self) {
    if self.active_radarr_block == &ActiveRadarrBlock::Movies {
      if !self.app.data.radarr_data.filtered_movies.items.is_empty() {
        self.app.data.radarr_data.filtered_movies.scroll_down();
      } else {
        self.app.data.radarr_data.movies.scroll_down()
      }
    }
  }

  fn handle_home(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Movies => {
        if !self.app.data.radarr_data.filtered_movies.items.is_empty() {
          self.app.data.radarr_data.filtered_movies.scroll_to_top();
        } else {
          self.app.data.radarr_data.movies.scroll_to_top()
        }
      }
      ActiveRadarrBlock::SearchMovie => {
        self
          .app
          .data
          .radarr_data
          .search
          .as_mut()
          .unwrap()
          .scroll_home();
      }
      ActiveRadarrBlock::FilterMovies => {
        self
          .app
          .data
          .radarr_data
          .filter
          .as_mut()
          .unwrap()
          .scroll_home();
      }
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Movies => {
        if !self.app.data.radarr_data.filtered_movies.items.is_empty() {
          self.app.data.radarr_data.filtered_movies.scroll_to_bottom();
        } else {
          self.app.data.radarr_data.movies.scroll_to_bottom()
        }
      }
      ActiveRadarrBlock::SearchMovie => self
        .app
        .data
        .radarr_data
        .search
        .as_mut()
        .unwrap()
        .reset_offset(),
      ActiveRadarrBlock::FilterMovies => self
        .app
        .data
        .radarr_data
        .filter
        .as_mut()
        .unwrap()
        .reset_offset(),
      _ => (),
    }
  }

  fn handle_delete(&mut self) {
    if self.active_radarr_block == &ActiveRadarrBlock::Movies {
      self
        .app
        .push_navigation_stack(ActiveRadarrBlock::DeleteMoviePrompt.into());
      self.app.data.radarr_data.selected_block =
        BlockSelectionState::new(&DELETE_MOVIE_SELECTION_BLOCKS);
    }
  }

  fn handle_left_right_action(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Movies => handle_change_tab_left_right_keys(self.app, self.key),
      ActiveRadarrBlock::UpdateAllMoviesPrompt => handle_prompt_toggle(self.app, self.key),
      ActiveRadarrBlock::SearchMovie => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self.app.data.radarr_data.search.as_mut().unwrap()
        )
      }
      ActiveRadarrBlock::FilterMovies => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self.app.data.radarr_data.filter.as_mut().unwrap()
        )
      }
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Movies => self
        .app
        .push_navigation_stack(ActiveRadarrBlock::MovieDetails.into()),
      ActiveRadarrBlock::SearchMovie => {
        if self.app.data.radarr_data.filtered_movies.items.is_empty() {
          search_table!(self.app, movies, ActiveRadarrBlock::SearchMovieError);
        } else {
          search_table!(
            self.app,
            filtered_movies,
            ActiveRadarrBlock::SearchMovieError
          );
        }
      }
      ActiveRadarrBlock::FilterMovies => {
        filter_table!(
          self.app,
          movies,
          filtered_movies,
          ActiveRadarrBlock::FilterMoviesError
        );
      }
      ActiveRadarrBlock::UpdateAllMoviesPrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::UpdateAllMovies);
        }

        self.app.pop_navigation_stack();
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::FilterMovies | ActiveRadarrBlock::FilterMoviesError => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.reset_filter();
        self.app.should_ignore_quit_key = false;
      }
      ActiveRadarrBlock::SearchMovie | ActiveRadarrBlock::SearchMovieError => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.reset_search();
        self.app.should_ignore_quit_key = false;
      }
      ActiveRadarrBlock::UpdateAllMoviesPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.prompt_confirm = false;
      }
      _ => {
        self.app.data.radarr_data.reset_search();
        self.app.data.radarr_data.reset_filter();
        handle_clear_errors(self.app);
      }
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_radarr_block {
      ActiveRadarrBlock::Movies => match self.key {
        _ if *key == DEFAULT_KEYBINDINGS.search.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::SearchMovie.into());
          self.app.data.radarr_data.search = Some(HorizontallyScrollableText::default());
          self.app.data.radarr_data.is_searching = true;
          self.app.should_ignore_quit_key = true;
        }
        _ if *key == DEFAULT_KEYBINDINGS.filter.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::FilterMovies.into());
          self.app.data.radarr_data.reset_filter();
          self.app.data.radarr_data.filter = Some(HorizontallyScrollableText::default());
          self.app.data.radarr_data.is_filtering = true;
          self.app.should_ignore_quit_key = true;
        }
        _ if *key == DEFAULT_KEYBINDINGS.edit.key => {
          self.app.push_navigation_stack(
            (
              ActiveRadarrBlock::EditMoviePrompt,
              Some(ActiveRadarrBlock::Movies),
            )
              .into(),
          );
          self.app.data.radarr_data.edit_movie_modal = Some((&self.app.data.radarr_data).into());
          self.app.data.radarr_data.selected_block =
            BlockSelectionState::new(&EDIT_MOVIE_SELECTION_BLOCKS);
        }
        _ if *key == DEFAULT_KEYBINDINGS.add.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::AddMovieSearchInput.into());
          self.app.data.radarr_data.search = Some(HorizontallyScrollableText::default());
          self.app.should_ignore_quit_key = true;
        }
        _ if *key == DEFAULT_KEYBINDINGS.update.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::UpdateAllMoviesPrompt.into());
        }
        _ if *key == DEFAULT_KEYBINDINGS.refresh.key => {
          self.app.should_refresh = true;
        }
        _ => (),
      },
      ActiveRadarrBlock::SearchMovie => {
        handle_text_box_keys!(
          self,
          key,
          self.app.data.radarr_data.search.as_mut().unwrap()
        )
      }
      ActiveRadarrBlock::FilterMovies => {
        handle_text_box_keys!(
          self,
          key,
          self.app.data.radarr_data.filter.as_mut().unwrap()
        )
      }
      _ => (),
    }
  }
}
