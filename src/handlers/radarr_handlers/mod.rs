use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::radarr::ActiveRadarrBlock;
use crate::handlers::radarr_handlers::collection_details_handler::CollectionDetailsHandler;
use crate::handlers::radarr_handlers::movie_details_handler::MovieDetailsHandler;
use crate::handlers::{handle_clear_errors, KeyEventHandler};
use crate::models::Scrollable;
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
      ActiveRadarrBlock::Collections => self.app.data.radarr_data.collections.scroll_up(),
      ActiveRadarrBlock::CollectionDetails => {
        self.app.data.radarr_data.collection_movies.scroll_up()
      }
      ActiveRadarrBlock::Movies => self.app.data.radarr_data.movies.scroll_up(),
      ActiveRadarrBlock::Downloads => self.app.data.radarr_data.downloads.scroll_up(),
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Collections => self.app.data.radarr_data.collections.scroll_down(),
      ActiveRadarrBlock::CollectionDetails => {
        self.app.data.radarr_data.collection_movies.scroll_down()
      }
      ActiveRadarrBlock::Movies => self.app.data.radarr_data.movies.scroll_down(),
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
          .position(|movie| movie.title.to_lowercase() == search_string);

        self.app.data.radarr_data.is_searching = false;
        self.app.data.radarr_data.movies.select_index(movie_index);

        if movie_index.is_some() {
          self.app.pop_navigation_stack();
        }
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::SearchMovie => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.is_searching = false;
        self.app.data.radarr_data.search = String::default();
      }
      _ => handle_clear_errors(self.app),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match *self.active_radarr_block {
      ActiveRadarrBlock::Movies => match key {
        _ if *key == DEFAULT_KEYBINDINGS.search.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::SearchMovie.into());
          self.app.data.radarr_data.is_searching = true;
        }
        _ => (),
      },
      ActiveRadarrBlock::SearchMovie => match key {
        _ if *key == DEFAULT_KEYBINDINGS.backspace.key => {
          self.app.data.radarr_data.search.pop();
        }
        Key::Char(character) => self.app.data.radarr_data.search.push(*character),
        _ => (),
      },
      _ => {}
    }
  }
}
