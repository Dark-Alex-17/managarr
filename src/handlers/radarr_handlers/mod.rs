use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::radarr::{
  ActiveRadarrBlock, ADD_MOVIE_BLOCKS, COLLECTION_DETAILS_BLOCKS, DELETE_MOVIE_BLOCKS,
  DELETE_MOVIE_SELECTION_BLOCKS, EDIT_COLLECTION_BLOCKS, EDIT_COLLECTION_SELECTION_BLOCKS,
  EDIT_MOVIE_BLOCKS, EDIT_MOVIE_SELECTION_BLOCKS, FILTER_BLOCKS, MOVIE_DETAILS_BLOCKS,
  SEARCH_BLOCKS,
};
use crate::handlers::radarr_handlers::add_movie_handler::AddMovieHandler;
use crate::handlers::radarr_handlers::collection_details_handler::CollectionDetailsHandler;
use crate::handlers::radarr_handlers::delete_movie_handler::DeleteMovieHandler;
use crate::handlers::radarr_handlers::edit_collection_handler::EditCollectionHandler;
use crate::handlers::radarr_handlers::edit_movie_handler::EditMovieHandler;
use crate::handlers::radarr_handlers::movie_details_handler::MovieDetailsHandler;
use crate::handlers::{handle_clear_errors, handle_prompt_toggle, KeyEventHandler};
use crate::models::{BlockSelectionState, HorizontallyScrollableText, Scrollable};
use crate::network::radarr_network::RadarrEvent;
use crate::utils::strip_non_search_characters;
use crate::{handle_text_box_keys, handle_text_box_left_right_keys, App, Key};

mod add_movie_handler;
mod collection_details_handler;
mod delete_movie_handler;
mod edit_collection_handler;
mod edit_movie_handler;
mod movie_details_handler;

#[cfg(test)]
#[path = "radarr_handler_tests.rs"]
mod radarr_handler_tests;

#[cfg(test)]
#[path = "radarr_handler_test_utils.rs"]
mod radarr_handler_test_utils;

pub(super) struct RadarrHandler<'a, 'b> {
  key: &'a Key,
  app: &'a mut App<'b>,
  active_radarr_block: &'a ActiveRadarrBlock,
  context: &'a Option<ActiveRadarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for RadarrHandler<'a, 'b> {
  fn handle(&mut self) {
    match self.active_radarr_block {
      _ if MOVIE_DETAILS_BLOCKS.contains(self.active_radarr_block) => {
        MovieDetailsHandler::with(self.key, self.app, self.active_radarr_block, self.context)
          .handle()
      }
      _ if COLLECTION_DETAILS_BLOCKS.contains(self.active_radarr_block) => {
        CollectionDetailsHandler::with(self.key, self.app, self.active_radarr_block, self.context)
          .handle()
      }
      _ if ADD_MOVIE_BLOCKS.contains(self.active_radarr_block) => {
        AddMovieHandler::with(self.key, self.app, self.active_radarr_block, self.context).handle()
      }
      _ if EDIT_MOVIE_BLOCKS.contains(self.active_radarr_block) => {
        EditMovieHandler::with(self.key, self.app, self.active_radarr_block, self.context).handle()
      }
      _ if DELETE_MOVIE_BLOCKS.contains(self.active_radarr_block) => {
        DeleteMovieHandler::with(self.key, self.app, self.active_radarr_block, self.context)
          .handle()
      }
      _ if EDIT_COLLECTION_BLOCKS.contains(self.active_radarr_block) => {
        EditCollectionHandler::with(self.key, self.app, self.active_radarr_block, self.context)
          .handle()
      }
      _ => self.handle_key_event(),
    }
  }

  fn with(
    key: &'a Key,
    app: &'a mut App<'b>,
    active_block: &'a ActiveRadarrBlock,
    context: &'a Option<ActiveRadarrBlock>,
  ) -> RadarrHandler<'a, 'b> {
    RadarrHandler {
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
      ActiveRadarrBlock::RootFolders => self.app.data.radarr_data.root_folders.scroll_up(),
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
      ActiveRadarrBlock::RootFolders => self.app.data.radarr_data.root_folders.scroll_down(),
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
      ActiveRadarrBlock::RootFolders => self.app.data.radarr_data.root_folders.scroll_to_top(),
      ActiveRadarrBlock::SearchMovie | ActiveRadarrBlock::SearchCollection => {
        self.app.data.radarr_data.search.scroll_home()
      }
      ActiveRadarrBlock::FilterMovies | ActiveRadarrBlock::FilterCollections => {
        self.app.data.radarr_data.filter.scroll_home()
      }
      ActiveRadarrBlock::AddRootFolderPrompt => self.app.data.radarr_data.edit_path.scroll_home(),
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
      ActiveRadarrBlock::RootFolders => self.app.data.radarr_data.root_folders.scroll_to_bottom(),
      ActiveRadarrBlock::SearchMovie | ActiveRadarrBlock::SearchCollection => {
        self.app.data.radarr_data.search.reset_offset()
      }
      ActiveRadarrBlock::FilterMovies | ActiveRadarrBlock::FilterCollections => {
        self.app.data.radarr_data.filter.reset_offset()
      }
      ActiveRadarrBlock::AddRootFolderPrompt => self.app.data.radarr_data.edit_path.reset_offset(),
      _ => (),
    }
  }

  fn handle_delete(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Movies => {
        self
          .app
          .push_navigation_stack(ActiveRadarrBlock::DeleteMoviePrompt.into());
        self.app.data.radarr_data.selected_block =
          BlockSelectionState::new(&DELETE_MOVIE_SELECTION_BLOCKS);
      }
      ActiveRadarrBlock::Downloads => self
        .app
        .push_navigation_stack(ActiveRadarrBlock::DeleteDownloadPrompt.into()),
      ActiveRadarrBlock::RootFolders => self
        .app
        .push_navigation_stack(ActiveRadarrBlock::DeleteRootFolderPrompt.into()),
      _ => (),
    }
  }

  fn handle_left_right_action(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Movies
      | ActiveRadarrBlock::Downloads
      | ActiveRadarrBlock::Collections
      | ActiveRadarrBlock::RootFolders
      | ActiveRadarrBlock::System => match self.key {
        _ if *self.key == DEFAULT_KEYBINDINGS.left.key => {
          self.app.data.radarr_data.main_tabs.previous();
          self
            .app
            .pop_and_push_navigation_stack(*self.app.data.radarr_data.main_tabs.get_active_route());
        }
        _ if *self.key == DEFAULT_KEYBINDINGS.right.key => {
          self.app.data.radarr_data.main_tabs.next();
          self
            .app
            .pop_and_push_navigation_stack(*self.app.data.radarr_data.main_tabs.get_active_route());
        }
        _ => (),
      },
      ActiveRadarrBlock::DeleteDownloadPrompt
      | ActiveRadarrBlock::DeleteRootFolderPrompt
      | ActiveRadarrBlock::UpdateAllMoviesPrompt
      | ActiveRadarrBlock::UpdateAllCollectionsPrompt
      | ActiveRadarrBlock::UpdateDownloadsPrompt => handle_prompt_toggle(self.app, self.key),
      ActiveRadarrBlock::AddRootFolderPrompt => {
        handle_text_box_left_right_keys!(self, self.key, self.app.data.radarr_data.edit_path)
      }
      ActiveRadarrBlock::SearchMovie | ActiveRadarrBlock::SearchCollection => {
        handle_text_box_left_right_keys!(self, self.key, self.app.data.radarr_data.search)
      }
      ActiveRadarrBlock::FilterMovies | ActiveRadarrBlock::FilterCollections => {
        handle_text_box_left_right_keys!(self, self.key, self.app.data.radarr_data.filter)
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
        if self.app.data.radarr_data.filtered_movies.items.is_empty() {
          let selected_index = self
            .search_table(&self.app.data.radarr_data.movies.items.clone(), |movie| {
              &movie.title.text
            });
          self
            .app
            .data
            .radarr_data
            .movies
            .select_index(selected_index);
        } else {
          let selected_index = self.search_table(
            &self.app.data.radarr_data.filtered_movies.items.clone(),
            |movie| &movie.title.text,
          );
          self
            .app
            .data
            .radarr_data
            .filtered_movies
            .select_index(selected_index);
        };
      }
      ActiveRadarrBlock::SearchCollection => {
        if self
          .app
          .data
          .radarr_data
          .filtered_collections
          .items
          .is_empty()
        {
          let selected_index = self.search_table(
            &self.app.data.radarr_data.collections.items.clone(),
            |collection| &collection.title.text,
          );
          self
            .app
            .data
            .radarr_data
            .collections
            .select_index(selected_index);
        } else {
          let selected_index = self.search_table(
            &self.app.data.radarr_data.filtered_collections.items.clone(),
            |collection| &collection.title.text,
          );
          self
            .app
            .data
            .radarr_data
            .filtered_collections
            .select_index(selected_index);
        }
      }
      ActiveRadarrBlock::FilterMovies => {
        let filtered_movies = self
          .filter_table(&self.app.data.radarr_data.movies.items.clone(), |movie| {
            &movie.title.text
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
          |collection| &collection.title.text,
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
      ActiveRadarrBlock::DeleteDownloadPrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::DeleteDownload);
        }

        self.app.pop_navigation_stack();
      }
      ActiveRadarrBlock::DeleteRootFolderPrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::DeleteRootFolder);
        }

        self.app.pop_navigation_stack();
      }
      ActiveRadarrBlock::UpdateAllMoviesPrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::UpdateAllMovies);
        }

        self.app.pop_navigation_stack();
      }
      ActiveRadarrBlock::UpdateDownloadsPrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::UpdateDownloads);
        }

        self.app.pop_navigation_stack();
      }
      ActiveRadarrBlock::UpdateAllCollectionsPrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::UpdateCollections);
        }

        self.app.pop_navigation_stack();
      }
      ActiveRadarrBlock::AddRootFolderPrompt => {
        self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::AddRootFolder);
        self.app.data.radarr_data.prompt_confirm = true;
        self.app.should_ignore_quit_key = false;
        self.app.pop_navigation_stack();
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_radarr_block {
      _ if FILTER_BLOCKS.contains(self.active_radarr_block) => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.reset_filter();
        self.app.should_ignore_quit_key = false;
      }
      _ if SEARCH_BLOCKS.contains(self.active_radarr_block) => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.reset_search();
        self.app.should_ignore_quit_key = false;
      }
      ActiveRadarrBlock::AddRootFolderPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.edit_path = HorizontallyScrollableText::default();
        self.app.data.radarr_data.prompt_confirm = false;
        self.app.should_ignore_quit_key = false;
      }
      ActiveRadarrBlock::DeleteDownloadPrompt
      | ActiveRadarrBlock::DeleteRootFolderPrompt
      | ActiveRadarrBlock::UpdateAllMoviesPrompt
      | ActiveRadarrBlock::UpdateAllCollectionsPrompt
      | ActiveRadarrBlock::UpdateDownloadsPrompt => {
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
          self.app.data.radarr_data.populate_edit_movie_fields();
          self.app.data.radarr_data.selected_block =
            BlockSelectionState::new(&EDIT_MOVIE_SELECTION_BLOCKS);
        }
        _ if *key == DEFAULT_KEYBINDINGS.add.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::AddMovieSearchInput.into());
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
      ActiveRadarrBlock::Downloads => match self.key {
        _ if *key == DEFAULT_KEYBINDINGS.update.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::UpdateDownloadsPrompt.into());
        }
        _ if *key == DEFAULT_KEYBINDINGS.refresh.key => {
          self.app.should_refresh = true;
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
          self.app.data.radarr_data.is_filtering = true;
          self.app.should_ignore_quit_key = true;
        }
        _ if *key == DEFAULT_KEYBINDINGS.edit.key => {
          self.app.push_navigation_stack(
            (
              ActiveRadarrBlock::EditCollectionPrompt,
              Some(ActiveRadarrBlock::Collections),
            )
              .into(),
          );
          self.app.data.radarr_data.populate_edit_collection_fields();
          self.app.data.radarr_data.selected_block =
            BlockSelectionState::new(&EDIT_COLLECTION_SELECTION_BLOCKS);
        }
        _ if *key == DEFAULT_KEYBINDINGS.update.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::UpdateAllCollectionsPrompt.into());
        }
        _ if *key == DEFAULT_KEYBINDINGS.refresh.key => {
          self.app.should_refresh = true;
        }
        _ => (),
      },
      ActiveRadarrBlock::RootFolders => match self.key {
        _ if *key == DEFAULT_KEYBINDINGS.refresh.key => {
          self.app.should_refresh = true;
        }
        _ if *key == DEFAULT_KEYBINDINGS.add.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::AddRootFolderPrompt.into());
          self.app.should_ignore_quit_key = true;
        }
        _ => (),
      },
      ActiveRadarrBlock::AddRootFolderPrompt => {
        handle_text_box_keys!(self, key, self.app.data.radarr_data.edit_path)
      }
      _ if SEARCH_BLOCKS.contains(self.active_radarr_block) => {
        handle_text_box_keys!(self, key, self.app.data.radarr_data.search)
      }
      _ if FILTER_BLOCKS.contains(self.active_radarr_block) => {
        handle_text_box_keys!(self, key, self.app.data.radarr_data.filter)
      }
      _ => (),
    }
  }
}

impl<'a, 'b> RadarrHandler<'a, 'b> {
  fn search_table<T, F>(&mut self, rows: &[T], field_selection_fn: F) -> Option<usize>
  where
    F: Fn(&T) -> &str,
  {
    let search_string = self.app.data.radarr_data.search.drain().to_lowercase();
    let search_index = rows.iter().position(|item| {
      strip_non_search_characters(field_selection_fn(item)).contains(&search_string)
    });

    self.app.data.radarr_data.is_searching = false;
    self.app.should_ignore_quit_key = false;

    if search_index.is_some() {
      self.app.pop_navigation_stack();
    }

    search_index
  }

  fn filter_table<T, F>(&mut self, rows: &[T], field_selection_fn: F) -> Vec<T>
  where
    F: Fn(&T) -> &str,
    T: Clone,
  {
    let filter = strip_non_search_characters(&self.app.data.radarr_data.filter.drain());
    let filter_matches: Vec<T> = rows
      .iter()
      .filter(|&item| strip_non_search_characters(field_selection_fn(item)).contains(&filter))
      .cloned()
      .collect();

    self.app.data.radarr_data.is_filtering = false;
    self.app.should_ignore_quit_key = false;

    if !filter_matches.is_empty() {
      self.app.pop_navigation_stack();
    }

    filter_matches
  }
}
