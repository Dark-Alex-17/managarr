use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::App;
use crate::event::Key;
use crate::handlers::radarr_handlers::handle_change_tab_left_right_keys;
use crate::handlers::radarr_handlers::library::add_movie_handler::AddMovieHandler;
use crate::handlers::radarr_handlers::library::delete_movie_handler::DeleteMovieHandler;
use crate::handlers::radarr_handlers::library::edit_movie_handler::EditMovieHandler;
use crate::handlers::radarr_handlers::library::movie_details_handler::MovieDetailsHandler;
use crate::handlers::{handle_clear_errors, handle_prompt_toggle, KeyEventHandler};

use crate::models::radarr_models::Movie;
use crate::models::servarr_data::radarr::radarr_data::{
  ActiveRadarrBlock, DELETE_MOVIE_SELECTION_BLOCKS, EDIT_MOVIE_SELECTION_BLOCKS, LIBRARY_BLOCKS,
};
use crate::models::stateful_table::SortOption;
use crate::models::{BlockSelectionState, HorizontallyScrollableText, Scrollable};
use crate::network::radarr_network::RadarrEvent;
use crate::{handle_text_box_keys, handle_text_box_left_right_keys};

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
    match self.active_radarr_block {
      ActiveRadarrBlock::Movies => self.app.data.radarr_data.movies.scroll_up(),
      ActiveRadarrBlock::MoviesSortPrompt => self
        .app
        .data
        .radarr_data
        .movies
        .sort
        .as_mut()
        .unwrap()
        .scroll_up(),
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Movies => self.app.data.radarr_data.movies.scroll_down(),
      ActiveRadarrBlock::MoviesSortPrompt => self
        .app
        .data
        .radarr_data
        .movies
        .sort
        .as_mut()
        .unwrap()
        .scroll_down(),
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Movies => self.app.data.radarr_data.movies.scroll_to_top(),
      ActiveRadarrBlock::SearchMovie => {
        self
          .app
          .data
          .radarr_data
          .movies
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
          .movies
          .filter
          .as_mut()
          .unwrap()
          .scroll_home();
      }
      ActiveRadarrBlock::MoviesSortPrompt => self
        .app
        .data
        .radarr_data
        .movies
        .sort
        .as_mut()
        .unwrap()
        .scroll_to_top(),
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Movies => self.app.data.radarr_data.movies.scroll_to_bottom(),
      ActiveRadarrBlock::SearchMovie => self
        .app
        .data
        .radarr_data
        .movies
        .search
        .as_mut()
        .unwrap()
        .reset_offset(),
      ActiveRadarrBlock::FilterMovies => self
        .app
        .data
        .radarr_data
        .movies
        .filter
        .as_mut()
        .unwrap()
        .reset_offset(),
      ActiveRadarrBlock::MoviesSortPrompt => self
        .app
        .data
        .radarr_data
        .movies
        .sort
        .as_mut()
        .unwrap()
        .scroll_to_bottom(),
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
          self.app.data.radarr_data.movies.search.as_mut().unwrap()
        )
      }
      ActiveRadarrBlock::FilterMovies => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self.app.data.radarr_data.movies.filter.as_mut().unwrap()
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
        self.app.pop_navigation_stack();
        self.app.should_ignore_quit_key = false;

        if self.app.data.radarr_data.movies.search.is_some() {
          let has_match = self
            .app
            .data
            .radarr_data
            .movies
            .apply_search(|movie| &movie.title.text);

          if !has_match {
            self
              .app
              .push_navigation_stack(ActiveRadarrBlock::SearchMovieError.into());
          }
        }
      }
      ActiveRadarrBlock::FilterMovies => {
        self.app.pop_navigation_stack();
        self.app.should_ignore_quit_key = false;

        if self.app.data.radarr_data.movies.filter.is_some() {
          let has_matches = self
            .app
            .data
            .radarr_data
            .movies
            .apply_filter(|movie| &movie.title.text);

          if !has_matches {
            self
              .app
              .push_navigation_stack(ActiveRadarrBlock::FilterMoviesError.into());
          }
        }
      }
      ActiveRadarrBlock::UpdateAllMoviesPrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::UpdateAllMovies);
        }

        self.app.pop_navigation_stack();
      }
      ActiveRadarrBlock::MoviesSortPrompt => {
        self
          .app
          .data
          .radarr_data
          .movies
          .items
          .sort_by(|a, b| a.id.cmp(&b.id));
        self.app.data.radarr_data.movies.apply_sorting();

        self.app.pop_navigation_stack();
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::FilterMovies | ActiveRadarrBlock::FilterMoviesError => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.movies.reset_filter();
        self.app.should_ignore_quit_key = false;
      }
      ActiveRadarrBlock::SearchMovie | ActiveRadarrBlock::SearchMovieError => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.movies.reset_search();
        self.app.should_ignore_quit_key = false;
      }
      ActiveRadarrBlock::UpdateAllMoviesPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.prompt_confirm = false;
      }
      ActiveRadarrBlock::MoviesSortPrompt => {
        self.app.pop_navigation_stack();
      }
      _ => {
        self.app.data.radarr_data.movies.reset_search();
        self.app.data.radarr_data.movies.reset_filter();
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
          self.app.data.radarr_data.movies.search = Some(HorizontallyScrollableText::default());
          self.app.should_ignore_quit_key = true;
        }
        _ if *key == DEFAULT_KEYBINDINGS.filter.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::FilterMovies.into());
          self.app.data.radarr_data.movies.reset_filter();
          self.app.data.radarr_data.movies.filter = Some(HorizontallyScrollableText::default());
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
          self.app.data.radarr_data.add_movie_search = Some(HorizontallyScrollableText::default());
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
        _ if *key == DEFAULT_KEYBINDINGS.sort.key => {
          self
            .app
            .data
            .radarr_data
            .movies
            .sorting(movies_sorting_options());
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::MoviesSortPrompt.into());
        }
        _ => (),
      },
      ActiveRadarrBlock::SearchMovie => {
        handle_text_box_keys!(
          self,
          key,
          self.app.data.radarr_data.movies.search.as_mut().unwrap()
        )
      }
      ActiveRadarrBlock::FilterMovies => {
        handle_text_box_keys!(
          self,
          key,
          self.app.data.radarr_data.movies.filter.as_mut().unwrap()
        )
      }
      _ => (),
    }
  }
}

fn movies_sorting_options() -> Vec<SortOption<Movie>> {
  vec![
    SortOption {
      name: "Title",
      cmp_fn: Some(|a, b| {
        a.title
          .text
          .to_lowercase()
          .cmp(&b.title.text.to_lowercase())
      }),
    },
    SortOption {
      name: "Year",
      cmp_fn: Some(|a, b| a.year.cmp(&b.year)),
    },
    SortOption {
      name: "Studio",
      cmp_fn: Some(|a, b| a.studio.to_lowercase().cmp(&b.studio.to_lowercase())),
    },
    SortOption {
      name: "Runtime",
      cmp_fn: Some(|a, b| a.runtime.cmp(&b.runtime)),
    },
    SortOption {
      name: "Rating",
      cmp_fn: Some(|a, b| {
        a.certification
          .as_ref()
          .unwrap_or(&String::new())
          .to_lowercase()
          .cmp(
            &b.certification
              .as_ref()
              .unwrap_or(&String::new())
              .to_lowercase(),
          )
      }),
    },
    SortOption {
      name: "Language",
      cmp_fn: Some(|a, b| {
        a.original_language
          .name
          .to_lowercase()
          .cmp(&b.original_language.name.to_lowercase())
      }),
    },
    SortOption {
      name: "Size",
      cmp_fn: Some(|a, b| a.size_on_disk.cmp(&b.size_on_disk)),
    },
    SortOption {
      name: "Quality",
      cmp_fn: Some(|a, b| a.quality_profile_id.cmp(&b.quality_profile_id)),
    },
    SortOption {
      name: "Monitored",
      cmp_fn: Some(|a, b| a.monitored.cmp(&b.monitored)),
    },
    SortOption {
      name: "Tags",
      cmp_fn: Some(|a, b| {
        let a_str = a
          .tags
          .iter()
          .map(|tag| tag.as_i64().unwrap().to_string())
          .collect::<Vec<String>>()
          .join(",");
        let b_str = b
          .tags
          .iter()
          .map(|tag| tag.as_i64().unwrap().to_string())
          .collect::<Vec<String>>()
          .join(",");

        a_str.cmp(&b_str)
      }),
    },
  ]
}
