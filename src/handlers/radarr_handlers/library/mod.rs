use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::App;
use crate::event::Key;
use crate::handlers::radarr_handlers::handle_change_tab_left_right_keys;
use crate::handlers::radarr_handlers::library::add_movie_handler::AddMovieHandler;
use crate::handlers::radarr_handlers::library::delete_movie_handler::DeleteMovieHandler;
use crate::handlers::radarr_handlers::library::edit_movie_handler::EditMovieHandler;
use crate::handlers::radarr_handlers::library::movie_details_handler::MovieDetailsHandler;
use crate::handlers::{handle_clear_errors, handle_prompt_toggle, KeyEventHandler};

use crate::handle_table_events;
use crate::handlers::table_handler::TableHandlingProps;
use crate::models::radarr_models::Movie;
use crate::models::servarr_data::radarr::radarr_data::{
  ActiveRadarrBlock, DELETE_MOVIE_SELECTION_BLOCKS, EDIT_MOVIE_SELECTION_BLOCKS, LIBRARY_BLOCKS,
};
use crate::models::stateful_table::SortOption;
use crate::models::{BlockSelectionState, HorizontallyScrollableText, Scrollable};
use crate::network::radarr_network::RadarrEvent;

mod add_movie_handler;
mod delete_movie_handler;
mod edit_movie_handler;
mod movie_details_handler;

#[cfg(test)]
#[path = "library_handler_tests.rs"]
mod library_handler_tests;

pub(super) struct LibraryHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_radarr_block: ActiveRadarrBlock,
  context: Option<ActiveRadarrBlock>,
}

impl<'a, 'b> LibraryHandler<'a, 'b> {
  handle_table_events!(self, movies, self.app.data.radarr_data.movies, Movie);
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for LibraryHandler<'a, 'b> {
  fn handle(&mut self) {
    let movie_table_handling_props = TableHandlingProps::new(ActiveRadarrBlock::Movies.into())
      .sorting_block(ActiveRadarrBlock::MoviesSortPrompt.into())
      .sort_by_fn(|a: &Movie, b: &Movie| a.id.cmp(&b.id))
      .sort_options(movies_sorting_options())
      .searching_block(ActiveRadarrBlock::SearchMovie.into())
      .search_error_block(ActiveRadarrBlock::SearchMovieError.into())
      .search_field_fn(|movie| &movie.title.text)
      .filtering_block(ActiveRadarrBlock::FilterMovies.into())
      .filter_error_block(ActiveRadarrBlock::FilterMoviesError.into())
      .filter_field_fn(|movie| &movie.title.text);

    if !self.handle_movies_table_events(movie_table_handling_props) {
      match self.active_radarr_block {
        _ if AddMovieHandler::accepts(self.active_radarr_block) => {
          AddMovieHandler::with(self.key, self.app, self.active_radarr_block, self.context)
            .handle();
        }
        _ if DeleteMovieHandler::accepts(self.active_radarr_block) => {
          DeleteMovieHandler::with(self.key, self.app, self.active_radarr_block, self.context)
            .handle();
        }
        _ if EditMovieHandler::accepts(self.active_radarr_block) => {
          EditMovieHandler::with(self.key, self.app, self.active_radarr_block, self.context)
            .handle();
        }
        _ if MovieDetailsHandler::accepts(self.active_radarr_block) => {
          MovieDetailsHandler::with(self.key, self.app, self.active_radarr_block, self.context)
            .handle();
        }
        _ => self.handle_key_event(),
      }
    }
  }

  fn accepts(active_block: ActiveRadarrBlock) -> bool {
    AddMovieHandler::accepts(active_block)
      || DeleteMovieHandler::accepts(active_block)
      || EditMovieHandler::accepts(active_block)
      || MovieDetailsHandler::accepts(active_block)
      || LIBRARY_BLOCKS.contains(&active_block)
  }

  fn with(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveRadarrBlock,
    context: Option<ActiveRadarrBlock>,
  ) -> LibraryHandler<'a, 'b> {
    LibraryHandler {
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
    !self.app.is_loading && !self.app.data.radarr_data.movies.is_empty()
  }

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {
    if self.active_radarr_block == ActiveRadarrBlock::Movies {
      self
        .app
        .push_navigation_stack(ActiveRadarrBlock::DeleteMoviePrompt.into());
      self.app.data.radarr_data.selected_block =
        BlockSelectionState::new(DELETE_MOVIE_SELECTION_BLOCKS);
    }
  }

  fn handle_left_right_action(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Movies => handle_change_tab_left_right_keys(self.app, self.key),
      ActiveRadarrBlock::UpdateAllMoviesPrompt => handle_prompt_toggle(self.app, self.key),
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Movies => self
        .app
        .push_navigation_stack(ActiveRadarrBlock::MovieDetails.into()),
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
      ActiveRadarrBlock::UpdateAllMoviesPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.prompt_confirm = false;
      }
      _ => {
        handle_clear_errors(self.app);
      }
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_radarr_block {
      ActiveRadarrBlock::Movies => match self.key {
        _ if key == DEFAULT_KEYBINDINGS.edit.key => {
          self.app.push_navigation_stack(
            (
              ActiveRadarrBlock::EditMoviePrompt,
              Some(ActiveRadarrBlock::Movies),
            )
              .into(),
          );
          self.app.data.radarr_data.edit_movie_modal = Some((&self.app.data.radarr_data).into());
          self.app.data.radarr_data.selected_block =
            BlockSelectionState::new(EDIT_MOVIE_SELECTION_BLOCKS);
        }
        _ if key == DEFAULT_KEYBINDINGS.add.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::AddMovieSearchInput.into());
          self.app.data.radarr_data.add_movie_search = Some(HorizontallyScrollableText::default());
          self.app.should_ignore_quit_key = true;
        }
        _ if key == DEFAULT_KEYBINDINGS.update.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::UpdateAllMoviesPrompt.into());
        }
        _ if key == DEFAULT_KEYBINDINGS.refresh.key => {
          self.app.should_refresh = true;
        }
        _ => (),
      },
      ActiveRadarrBlock::UpdateAllMoviesPrompt => {
        if key == DEFAULT_KEYBINDINGS.confirm.key {
          self.app.data.radarr_data.prompt_confirm = true;
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::UpdateAllMovies);

          self.app.pop_navigation_stack();
        }
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
