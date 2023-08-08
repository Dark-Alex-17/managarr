use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::radarr::ActiveRadarrBlock;
use crate::handlers::radarr_handlers::add_movie_handler::AddMovieHandler;
use crate::handlers::radarr_handlers::collection_details_handler::CollectionDetailsHandler;
use crate::handlers::radarr_handlers::movie_details_handler::MovieDetailsHandler;
use crate::handlers::{handle_clear_errors, handle_prompt_toggle, KeyEventHandler};
use crate::models::Scrollable;
use crate::network::radarr_network::RadarrEvent;
use crate::utils::strip_non_alphanumeric_characters;
use crate::{App, Key};

mod add_movie_handler;
mod collection_details_handler;
mod movie_details_handler;

pub(super) struct RadarrHandler<'a> {
  key: &'a Key,
  app: &'a mut App,
  active_radarr_block: &'a ActiveRadarrBlock,
}

impl<'a> KeyEventHandler<'a, ActiveRadarrBlock> for RadarrHandler<'a> {
  fn handle(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::MovieDetails
      | ActiveRadarrBlock::MovieHistory
      | ActiveRadarrBlock::FileInfo
      | ActiveRadarrBlock::Cast
      | ActiveRadarrBlock::Crew => {
        MovieDetailsHandler::with(self.key, self.app, self.active_radarr_block).handle()
      }
      ActiveRadarrBlock::CollectionDetails | ActiveRadarrBlock::ViewMovieOverview => {
        CollectionDetailsHandler::with(self.key, self.app, self.active_radarr_block).handle()
      }
      ActiveRadarrBlock::AddMovieSearchInput
      | ActiveRadarrBlock::AddMovieSearchResults
      | ActiveRadarrBlock::AddMoviePrompt => {
        AddMovieHandler::with(self.key, self.app, self.active_radarr_block).handle()
      }
      _ => self.handle_key_event(),
    }
  }

  fn with(
    key: &'a Key,
    app: &'a mut App,
    active_block: &'a ActiveRadarrBlock,
  ) -> RadarrHandler<'a> {
    RadarrHandler {
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
      ActiveRadarrBlock::Collections => {
        if !self
          .app
          .data
          .radarr_data
          .filtered_collections
          .items
          .is_empty()
        {
          self.app.data.radarr_data.filtered_collections.scroll_up();
        } else {
          self.app.data.radarr_data.collections.scroll_up()
        }
      }
      ActiveRadarrBlock::Movies => {
        if !self.app.data.radarr_data.filtered_movies.items.is_empty() {
          self.app.data.radarr_data.filtered_movies.scroll_up();
        } else {
          self.app.data.radarr_data.movies.scroll_up()
        }
      }
      ActiveRadarrBlock::Downloads => self.app.data.radarr_data.downloads.scroll_up(),
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Collections => {
        if !self
          .app
          .data
          .radarr_data
          .filtered_collections
          .items
          .is_empty()
        {
          self.app.data.radarr_data.filtered_collections.scroll_down();
        } else {
          self.app.data.radarr_data.collections.scroll_down()
        }
      }
      ActiveRadarrBlock::Movies => {
        if !self.app.data.radarr_data.filtered_movies.items.is_empty() {
          self.app.data.radarr_data.filtered_movies.scroll_down();
        } else {
          self.app.data.radarr_data.movies.scroll_down()
        }
      }
      ActiveRadarrBlock::Downloads => self.app.data.radarr_data.downloads.scroll_down(),
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Collections => {
        if !self
          .app
          .data
          .radarr_data
          .filtered_collections
          .items
          .is_empty()
        {
          self
            .app
            .data
            .radarr_data
            .filtered_collections
            .scroll_to_top();
        } else {
          self.app.data.radarr_data.collections.scroll_to_top()
        }
      }
      ActiveRadarrBlock::Movies => {
        if !self.app.data.radarr_data.filtered_movies.items.is_empty() {
          self.app.data.radarr_data.filtered_movies.scroll_to_top();
        } else {
          self.app.data.radarr_data.movies.scroll_to_top()
        }
      }
      ActiveRadarrBlock::Downloads => self.app.data.radarr_data.downloads.scroll_to_top(),
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Collections => {
        if !self
          .app
          .data
          .radarr_data
          .filtered_collections
          .items
          .is_empty()
        {
          self
            .app
            .data
            .radarr_data
            .filtered_collections
            .scroll_to_bottom();
        } else {
          self.app.data.radarr_data.collections.scroll_to_bottom()
        }
      }
      ActiveRadarrBlock::Movies => {
        if !self.app.data.radarr_data.filtered_movies.items.is_empty() {
          self.app.data.radarr_data.filtered_movies.scroll_to_bottom();
        } else {
          self.app.data.radarr_data.movies.scroll_to_bottom()
        }
      }
      ActiveRadarrBlock::Downloads => self.app.data.radarr_data.downloads.scroll_to_bottom(),
      _ => (),
    }
  }

  fn handle_delete(&mut self) {
    if *self.active_radarr_block == ActiveRadarrBlock::Movies {
      self
        .app
        .push_navigation_stack(ActiveRadarrBlock::DeleteMoviePrompt.into());
    }
  }

  fn handle_left_right_action(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Movies | ActiveRadarrBlock::Downloads | ActiveRadarrBlock::Collections => {
        match self.key {
          _ if *self.key == DEFAULT_KEYBINDINGS.left.key => {
            self.app.data.radarr_data.main_tabs.previous();
            self.app.pop_and_push_navigation_stack(
              self
                .app
                .data
                .radarr_data
                .main_tabs
                .get_active_route()
                .clone(),
            );
          }
          _ if *self.key == DEFAULT_KEYBINDINGS.right.key => {
            self.app.data.radarr_data.main_tabs.next();
            self.app.pop_and_push_navigation_stack(
              self
                .app
                .data
                .radarr_data
                .main_tabs
                .get_active_route()
                .clone(),
            );
          }
          _ => (),
        }
      }
      ActiveRadarrBlock::DeleteMoviePrompt => handle_prompt_toggle(self.app, self.key),
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Movies => self
        .app
        .push_navigation_stack(ActiveRadarrBlock::MovieDetails.into()),
      ActiveRadarrBlock::Collections => self
        .app
        .push_navigation_stack(ActiveRadarrBlock::CollectionDetails.into()),
      ActiveRadarrBlock::SearchMovie => {
        let selected_index = self
          .search_table(&self.app.data.radarr_data.movies.items.clone(), |movie| {
            &movie.title
          });
        self
          .app
          .data
          .radarr_data
          .movies
          .select_index(selected_index);
      }
      ActiveRadarrBlock::SearchCollection => {
        let selected_index = self.search_table(
          &self.app.data.radarr_data.collections.items.clone(),
          |movie| &movie.title,
        );
        self
          .app
          .data
          .radarr_data
          .collections
          .select_index(selected_index);
      }
      ActiveRadarrBlock::FilterMovies => {
        let filtered_movies = self
          .filter_table(&self.app.data.radarr_data.movies.items.clone(), |movie| {
            &movie.title
          });

        if !filtered_movies.is_empty() {
          self
            .app
            .data
            .radarr_data
            .filtered_movies
            .set_items(filtered_movies);
        }
      }
      ActiveRadarrBlock::FilterCollections => {
        let filtered_collections = self.filter_table(
          &self.app.data.radarr_data.collections.items.clone(),
          |collection| &collection.title,
        );

        if !filtered_collections.is_empty() {
          self
            .app
            .data
            .radarr_data
            .filtered_collections
            .set_items(filtered_collections);
        }
      }
      ActiveRadarrBlock::DeleteMoviePrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::DeleteMovie);
        }

        self.app.pop_navigation_stack();
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::SearchMovie
      | ActiveRadarrBlock::SearchCollection
      | ActiveRadarrBlock::FilterMovies
      | ActiveRadarrBlock::FilterCollections => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.reset_search();
        self.app.should_ignore_quit_key = false;
      }
      ActiveRadarrBlock::DeleteMoviePrompt => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.prompt_confirm = false;
      }
      _ => {
        self.app.data.radarr_data.reset_search();
        handle_clear_errors(self.app);
      }
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match *self.active_radarr_block {
      ActiveRadarrBlock::Movies => match self.key {
        _ if *key == DEFAULT_KEYBINDINGS.search.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::SearchMovie.into());
          self.app.data.radarr_data.is_searching = true;
          self.app.should_ignore_quit_key = true;
        }
        _ if *key == DEFAULT_KEYBINDINGS.filter.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::FilterMovies.into());
          self.app.data.radarr_data.is_searching = true;
          self.app.should_ignore_quit_key = true;
        }
        _ if *key == DEFAULT_KEYBINDINGS.add.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::AddMovieSearchInput.into());
          self.app.should_ignore_quit_key = true;
        }
        _ => (),
      },
      ActiveRadarrBlock::Collections => match self.key {
        _ if *key == DEFAULT_KEYBINDINGS.search.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::SearchCollection.into());
          self.app.data.radarr_data.is_searching = true;
          self.app.should_ignore_quit_key = true;
        }
        _ if *key == DEFAULT_KEYBINDINGS.filter.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::FilterCollections.into());
          self.app.data.radarr_data.is_searching = true;
          self.app.should_ignore_quit_key = true;
        }
        _ => (),
      },
      ActiveRadarrBlock::SearchMovie | ActiveRadarrBlock::SearchCollection => match self.key {
        _ if *key == DEFAULT_KEYBINDINGS.backspace.key => {
          self.app.data.radarr_data.search.pop();
        }
        Key::Char(character) => {
          self.app.data.radarr_data.search.push(*character);
        }
        _ => (),
      },
      ActiveRadarrBlock::FilterMovies | ActiveRadarrBlock::FilterCollections => match self.key {
        _ if *key == DEFAULT_KEYBINDINGS.backspace.key => {
          self.app.data.radarr_data.filter.pop();
        }
        Key::Char(character) => {
          self.app.data.radarr_data.filter.push(*character);
        }
        _ => (),
      },
      _ => (),
    }
  }
}

impl RadarrHandler<'_> {
  fn search_table<T, F>(&mut self, rows: &[T], field_selection_fn: F) -> Option<usize>
  where
    F: Fn(&T) -> &str,
  {
    let search_string = self
      .app
      .data
      .radarr_data
      .search
      .drain(..)
      .collect::<String>()
      .to_lowercase();
    let collection_index = rows.iter().position(|item| {
      strip_non_alphanumeric_characters(field_selection_fn(item)).contains(&search_string)
    });

    self.app.data.radarr_data.is_searching = false;
    self.app.should_ignore_quit_key = false;

    if collection_index.is_some() {
      self.app.pop_navigation_stack();
    }

    collection_index
  }

  fn filter_table<T, F>(&mut self, rows: &[T], field_selection_fn: F) -> Vec<T>
  where
    F: Fn(&T) -> &str,
    T: Clone,
  {
    let filter = strip_non_alphanumeric_characters(
      &self
        .app
        .data
        .radarr_data
        .filter
        .drain(..)
        .collect::<String>(),
    );
    let filter_matches: Vec<T> = rows
      .iter()
      .filter(|&item| strip_non_alphanumeric_characters(field_selection_fn(item)).contains(&filter))
      .cloned()
      .collect();

    self.app.data.radarr_data.is_searching = false;
    self.app.should_ignore_quit_key = false;

    if !filter_matches.is_empty() {
      self.app.pop_navigation_stack();
    }

    filter_matches
  }
}
