use regex::Regex;

use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::radarr::ActiveRadarrBlock;
use crate::handlers::radarr_handlers::collection_details_handler::CollectionDetailsHandler;
use crate::handlers::radarr_handlers::movie_details_handler::MovieDetailsHandler;
use crate::handlers::{handle_clear_errors, KeyEventHandler};
use crate::models::radarr_models::Movie;
use crate::models::Scrollable;
use crate::utils::strip_non_alphanumeric_characters;
use crate::{App, Key};

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
        if !self.app.data.radarr_data.filter.is_empty() {
          self
            .app
            .data
            .radarr_data
            .collections
            .scroll_up_with_filter(|&collection| {
              strip_non_alphanumeric_characters(&collection.title)
                .starts_with(&self.app.data.radarr_data.filter)
            });
        } else {
          self.app.data.radarr_data.collections.scroll_up()
        }
      }
      ActiveRadarrBlock::CollectionDetails => {
        self.app.data.radarr_data.collection_movies.scroll_up()
      }
      ActiveRadarrBlock::Movies => {
        if !self.app.data.radarr_data.filter.is_empty() {
          self
            .app
            .data
            .radarr_data
            .movies
            .scroll_up_with_filter(|&movie| {
              strip_non_alphanumeric_characters(&movie.title)
                .starts_with(&self.app.data.radarr_data.filter)
            });
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
        if !self.app.data.radarr_data.filter.is_empty() {
          self
            .app
            .data
            .radarr_data
            .collections
            .scroll_down_with_filter(|&collection| {
              strip_non_alphanumeric_characters(&collection.title)
                .starts_with(&self.app.data.radarr_data.filter)
            });
        } else {
          self.app.data.radarr_data.collections.scroll_down()
        }
      }
      ActiveRadarrBlock::CollectionDetails => {
        self.app.data.radarr_data.collection_movies.scroll_down()
      }
      ActiveRadarrBlock::Movies => {
        if !self.app.data.radarr_data.filter.is_empty() {
          self
            .app
            .data
            .radarr_data
            .movies
            .scroll_down_with_filter(|&movie| {
              strip_non_alphanumeric_characters(&movie.title)
                .starts_with(&self.app.data.radarr_data.filter)
            });
        } else {
          self.app.data.radarr_data.movies.scroll_down()
        }
      }
      ActiveRadarrBlock::Downloads => self.app.data.radarr_data.downloads.scroll_down(),
      _ => (),
    }
  }

  fn handle_tab_action(&mut self) {
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
        let search_string = self
          .app
          .data
          .radarr_data
          .search
          .drain(..)
          .collect::<String>()
          .to_lowercase();
        let movie_index = self
          .app
          .data
          .radarr_data
          .movies
          .items
          .iter()
          .position(|movie| {
            strip_non_alphanumeric_characters(&movie.title).starts_with(&search_string)
          });

        self.app.data.radarr_data.is_searching = false;
        self.app.data.radarr_data.movies.select_index(movie_index);

        if movie_index.is_some() {
          self.app.pop_navigation_stack();
        }
      }
      ActiveRadarrBlock::SearchCollection => {
        let search_string = self
          .app
          .data
          .radarr_data
          .search
          .drain(..)
          .collect::<String>()
          .to_lowercase();
        let collection_index =
          self
            .app
            .data
            .radarr_data
            .collections
            .items
            .iter()
            .position(|collection| {
              strip_non_alphanumeric_characters(&collection.title).starts_with(&search_string)
            });

        self.app.data.radarr_data.is_searching = false;
        self
          .app
          .data
          .radarr_data
          .collections
          .select_index(collection_index);

        if collection_index.is_some() {
          self.app.pop_navigation_stack();
        }
      }
      ActiveRadarrBlock::FilterMovies => {
        self.app.data.radarr_data.filter =
          strip_non_alphanumeric_characters(&self.app.data.radarr_data.filter);
        let filter_string = &self.app.data.radarr_data.filter;
        let filter_matches = self
          .app
          .data
          .radarr_data
          .movies
          .items
          .iter()
          .filter(|&movie| {
            strip_non_alphanumeric_characters(&movie.title).starts_with(filter_string)
          })
          .count();

        self.app.data.radarr_data.is_searching = false;

        if filter_matches > 0 {
          self.app.pop_navigation_stack();
        }
      }
      ActiveRadarrBlock::FilterCollections => {
        self.app.data.radarr_data.filter =
          strip_non_alphanumeric_characters(&self.app.data.radarr_data.filter);
        let filter_string = &self.app.data.radarr_data.filter;
        let filter_matches = self
          .app
          .data
          .radarr_data
          .collections
          .items
          .iter()
          .filter(|&collection| {
            strip_non_alphanumeric_characters(&collection.title).starts_with(filter_string)
          })
          .count();

        self.app.data.radarr_data.is_searching = false;

        if filter_matches > 0 {
          self.app.pop_navigation_stack();
        }
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
        }
        _ if *key == DEFAULT_KEYBINDINGS.filter.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::FilterMovies.into());
          self.app.data.radarr_data.is_searching = true;
        }
        _ => (),
      },
      ActiveRadarrBlock::Collections => match self.key {
        _ if *key == DEFAULT_KEYBINDINGS.search.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::SearchCollection.into());
          self.app.data.radarr_data.is_searching = true;
        }
        _ if *key == DEFAULT_KEYBINDINGS.filter.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::FilterCollections.into());
          self.app.data.radarr_data.is_searching = true;
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
