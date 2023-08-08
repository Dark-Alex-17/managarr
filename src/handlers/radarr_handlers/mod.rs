use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::radarr::{
  ActiveRadarrBlock, ADD_MOVIE_BLOCKS, COLLECTION_DETAILS_BLOCKS, EDIT_COLLECTION_BLOCKS,
  EDIT_MOVIE_BLOCKS, FILTER_BLOCKS, MOVIE_DETAILS_BLOCKS, SEARCH_BLOCKS,
};
use crate::handlers::radarr_handlers::add_movie_handler::AddMovieHandler;
use crate::handlers::radarr_handlers::collection_details_handler::CollectionDetailsHandler;
use crate::handlers::radarr_handlers::edit_collection_handler::EditCollectionHandler;
use crate::handlers::radarr_handlers::edit_movie_handler::EditMovieHandler;
use crate::handlers::radarr_handlers::movie_details_handler::MovieDetailsHandler;
use crate::handlers::{handle_clear_errors, handle_prompt_toggle, KeyEventHandler};
use crate::models::Scrollable;
use crate::network::radarr_network::RadarrEvent;
use crate::utils::strip_non_alphanumeric_characters;
use crate::{handle_text_box_keys, handle_text_box_left_right_keys, App, Key};

mod add_movie_handler;
mod collection_details_handler;
mod edit_collection_handler;
mod edit_movie_handler;
mod movie_details_handler;

pub(super) struct RadarrHandler<'a> {
  key: &'a Key,
  app: &'a mut App,
  active_radarr_block: &'a ActiveRadarrBlock,
  context: &'a Option<ActiveRadarrBlock>,
}

impl<'a> KeyEventHandler<'a, ActiveRadarrBlock> for RadarrHandler<'a> {
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
      _ if EDIT_COLLECTION_BLOCKS.contains(self.active_radarr_block) => {
        EditCollectionHandler::with(self.key, self.app, self.active_radarr_block, self.context)
          .handle()
      }
      _ => self.handle_key_event(),
    }
  }

  fn with(
    key: &'a Key,
    app: &'a mut App,
    active_block: &'a ActiveRadarrBlock,
    context: &'a Option<ActiveRadarrBlock>,
  ) -> RadarrHandler<'a> {
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
      ActiveRadarrBlock::SearchMovie | ActiveRadarrBlock::SearchCollection => {
        self.app.data.radarr_data.search.scroll_home()
      }
      ActiveRadarrBlock::FilterMovies | ActiveRadarrBlock::FilterCollections => {
        self.app.data.radarr_data.filter.scroll_home()
      }
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
      ActiveRadarrBlock::SearchMovie | ActiveRadarrBlock::SearchCollection => {
        self.app.data.radarr_data.search.reset_offset()
      }
      ActiveRadarrBlock::FilterMovies | ActiveRadarrBlock::FilterCollections => {
        self.app.data.radarr_data.filter.reset_offset()
      }
      _ => (),
    }
  }

  fn handle_delete(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Movies => self
        .app
        .push_navigation_stack(ActiveRadarrBlock::DeleteMoviePrompt.into()),
      ActiveRadarrBlock::Downloads => self
        .app
        .push_navigation_stack(ActiveRadarrBlock::DeleteDownloadPrompt.into()),
      _ => (),
    }
  }

  fn handle_left_right_action(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Movies | ActiveRadarrBlock::Downloads | ActiveRadarrBlock::Collections => {
        match self.key {
          _ if *self.key == DEFAULT_KEYBINDINGS.left.key => {
            self.app.data.radarr_data.main_tabs.previous();
            self.app.pop_and_push_navigation_stack(
              *self.app.data.radarr_data.main_tabs.get_active_route(),
            );
          }
          _ if *self.key == DEFAULT_KEYBINDINGS.right.key => {
            self.app.data.radarr_data.main_tabs.next();
            self.app.pop_and_push_navigation_stack(
              *self.app.data.radarr_data.main_tabs.get_active_route(),
            );
          }
          _ => (),
        }
      }
      ActiveRadarrBlock::DeleteMoviePrompt
      | ActiveRadarrBlock::DeleteDownloadPrompt
      | ActiveRadarrBlock::UpdateAllMoviesPrompt
      | ActiveRadarrBlock::UpdateAllCollectionsPrompt
      | ActiveRadarrBlock::UpdateDownloadsPrompt => handle_prompt_toggle(self.app, self.key),
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
      ActiveRadarrBlock::DeleteMoviePrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::DeleteMovie);
        }

        self.app.pop_navigation_stack();
      }
      ActiveRadarrBlock::DeleteDownloadPrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::DeleteDownload);
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
      ActiveRadarrBlock::DeleteMoviePrompt
      | ActiveRadarrBlock::DeleteDownloadPrompt
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
          self.app.data.radarr_data.selected_block = ActiveRadarrBlock::EditMovieToggleMonitored;
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
            ActiveRadarrBlock::EditCollectionToggleMonitored;
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

impl RadarrHandler<'_> {
  fn search_table<T, F>(&mut self, rows: &[T], field_selection_fn: F) -> Option<usize>
  where
    F: Fn(&T) -> &str,
  {
    let search_string = self.app.data.radarr_data.search.drain().to_lowercase();
    let search_index = rows.iter().position(|item| {
      strip_non_alphanumeric_characters(field_selection_fn(item)).contains(&search_string)
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
    let filter = strip_non_alphanumeric_characters(&self.app.data.radarr_data.filter.drain());
    let filter_matches: Vec<T> = rows
      .iter()
      .filter(|&item| strip_non_alphanumeric_characters(field_selection_fn(item)).contains(&filter))
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

#[cfg(test)]
#[macro_use]
mod radarr_handler_test_utils {
  #[macro_export]
  macro_rules! test_edit_movie_key {
    ($handler:ident, $block:expr, $context:expr) => {
      let mut app = App::default();
      let mut radarr_data = RadarrData {
        edit_path: HorizontallyScrollableText::default(),
        edit_tags: HorizontallyScrollableText::default(),
        edit_monitored: None,
        edit_search_on_add: None,
        quality_profile_map: BiMap::from_iter([
          (2222, "HD - 1080p".to_owned()),
          (1111, "Any".to_owned()),
        ]),
        tags_map: BiMap::from_iter([(1, "test".to_owned())]),
        filtered_movies: StatefulTable::default(),
        ..create_test_radarr_data()
      };
      radarr_data.movies.set_items(vec![Movie {
        path: "/nfs/movies/Test".to_owned().into(),
        monitored: true,
        quality_profile_id: Number::from(2222),
        minimum_availability: MinimumAvailability::Released,
        tags: vec![Number::from(1)],
        ..Movie::default()
      }]);
      app.data.radarr_data = radarr_data;

      $handler::with(&DEFAULT_KEYBINDINGS.edit.key, &mut app, &$block, &None).handle();

      assert_eq!(
        app.get_current_route(),
        &(ActiveRadarrBlock::EditMoviePrompt, Some($context)).into()
      );
      assert_eq!(
        app.data.radarr_data.selected_block,
        ActiveRadarrBlock::EditMovieToggleMonitored
      );
      assert_eq!(
        app.data.radarr_data.minimum_availability_list.items,
        Vec::from_iter(MinimumAvailability::iter())
      );
      assert_eq!(
        app
          .data
          .radarr_data
          .minimum_availability_list
          .current_selection(),
        &MinimumAvailability::Released
      );
      assert_eq!(
        app.data.radarr_data.quality_profile_list.items,
        vec!["Any".to_owned(), "HD - 1080p".to_owned()]
      );
      assert_str_eq!(
        app
          .data
          .radarr_data
          .quality_profile_list
          .current_selection(),
        "HD - 1080p"
      );
      assert_str_eq!(app.data.radarr_data.edit_path.text, "/nfs/movies/Test");
      assert_str_eq!(app.data.radarr_data.edit_tags.text, "test");
      assert_eq!(app.data.radarr_data.edit_monitored, Some(true));
    };
  }

  #[macro_export]
  macro_rules! test_edit_collection_key {
    ($handler:ident, $block:expr, $context:expr) => {
      let mut app = App::default();
      let mut radarr_data = RadarrData {
        edit_path: HorizontallyScrollableText::default(),
        edit_tags: HorizontallyScrollableText::default(),
        edit_monitored: None,
        edit_search_on_add: None,
        quality_profile_map: BiMap::from_iter([
          (2222, "HD - 1080p".to_owned()),
          (1111, "Any".to_owned()),
        ]),
        filtered_collections: StatefulTable::default(),
        ..create_test_radarr_data()
      };
      radarr_data.collections.set_items(vec![Collection {
        root_folder_path: "/nfs/movies/Test".to_owned().into(),
        monitored: true,
        search_on_add: true,
        quality_profile_id: Number::from(2222),
        minimum_availability: MinimumAvailability::Released,
        ..Collection::default()
      }]);
      app.data.radarr_data = radarr_data;

      $handler::with(&DEFAULT_KEYBINDINGS.edit.key, &mut app, &$block, &None).handle();

      assert_eq!(
        app.get_current_route(),
        &(ActiveRadarrBlock::EditCollectionPrompt, Some($context)).into()
      );
      assert_eq!(
        app.data.radarr_data.selected_block,
        ActiveRadarrBlock::EditCollectionToggleMonitored
      );
      assert_eq!(
        app.data.radarr_data.minimum_availability_list.items,
        Vec::from_iter(MinimumAvailability::iter())
      );
      assert_eq!(
        app
          .data
          .radarr_data
          .minimum_availability_list
          .current_selection(),
        &MinimumAvailability::Released
      );
      assert_eq!(
        app.data.radarr_data.quality_profile_list.items,
        vec!["Any".to_owned(), "HD - 1080p".to_owned()]
      );
      assert_str_eq!(
        app
          .data
          .radarr_data
          .quality_profile_list
          .current_selection(),
        "HD - 1080p"
      );
      assert_str_eq!(app.data.radarr_data.edit_path.text, "/nfs/movies/Test");
      assert_eq!(app.data.radarr_data.edit_monitored, Some(true));
      assert_eq!(app.data.radarr_data.edit_search_on_add, Some(true));
    };
  }
}

#[cfg(test)]
mod tests {
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::radarr::ActiveRadarrBlock;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::radarr_handlers::RadarrHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::radarr_models::{Collection, Movie};
  use crate::models::HorizontallyScrollableText;
  use crate::{extended_stateful_iterable_vec, test_handler_delegation};

  mod test_handle_scroll_up_and_down {
    use rstest::rstest;

    use crate::models::radarr_models::DownloadRecord;
    use crate::{simple_stateful_iterable_vec, test_iterable_scroll};

    use super::*;

    test_iterable_scroll!(
      test_collections_scroll,
      RadarrHandler,
      collections,
      simple_stateful_iterable_vec!(Collection, HorizontallyScrollableText),
      ActiveRadarrBlock::Collections,
      None,
      title,
      to_string
    );

    test_iterable_scroll!(
      test_filtered_collections_scroll,
      RadarrHandler,
      filtered_collections,
      simple_stateful_iterable_vec!(Collection, HorizontallyScrollableText),
      ActiveRadarrBlock::Collections,
      None,
      title,
      to_string
    );

    test_iterable_scroll!(
      test_movies_scroll,
      RadarrHandler,
      movies,
      simple_stateful_iterable_vec!(Movie, HorizontallyScrollableText),
      ActiveRadarrBlock::Movies,
      None,
      title,
      to_string
    );

    test_iterable_scroll!(
      test_filtered_movies_scroll,
      RadarrHandler,
      filtered_movies,
      simple_stateful_iterable_vec!(Movie, HorizontallyScrollableText),
      ActiveRadarrBlock::Movies,
      None,
      title,
      to_string
    );

    test_iterable_scroll!(
      test_downloads_scroll,
      RadarrHandler,
      downloads,
      DownloadRecord,
      ActiveRadarrBlock::Downloads,
      None,
      title
    );
  }

  mod test_handle_home_end {
    use pretty_assertions::assert_eq;

    use crate::models::radarr_models::DownloadRecord;
    use crate::{
      extended_stateful_iterable_vec, test_iterable_home_and_end, test_text_box_home_end_keys,
    };

    use super::*;

    test_iterable_home_and_end!(
      test_collections_home_end,
      RadarrHandler,
      collections,
      extended_stateful_iterable_vec!(Collection, HorizontallyScrollableText),
      ActiveRadarrBlock::Collections,
      None,
      title,
      to_string
    );

    test_iterable_home_and_end!(
      test_filtered_collections_home_end,
      RadarrHandler,
      filtered_collections,
      extended_stateful_iterable_vec!(Collection, HorizontallyScrollableText),
      ActiveRadarrBlock::Collections,
      None,
      title,
      to_string
    );

    test_iterable_home_and_end!(
      test_movies_home_end,
      RadarrHandler,
      movies,
      extended_stateful_iterable_vec!(Movie, HorizontallyScrollableText),
      ActiveRadarrBlock::Movies,
      None,
      title,
      to_string
    );

    test_iterable_home_and_end!(
      test_filtered_movies_home_end,
      RadarrHandler,
      filtered_movies,
      extended_stateful_iterable_vec!(Movie, HorizontallyScrollableText),
      ActiveRadarrBlock::Movies,
      None,
      title,
      to_string
    );

    test_iterable_home_and_end!(
      test_downloads_home_end,
      RadarrHandler,
      downloads,
      DownloadRecord,
      ActiveRadarrBlock::Downloads,
      None,
      title
    );

    #[rstest]
    fn test_search_boxes_home_end_keys(
      #[values(ActiveRadarrBlock::SearchMovie, ActiveRadarrBlock::SearchCollection)]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      test_text_box_home_end_keys!(RadarrHandler, active_radarr_block, search);
    }

    #[rstest]
    fn test_filter_boxes_home_end_keys(
      #[values(ActiveRadarrBlock::FilterMovies, ActiveRadarrBlock::FilterCollections)]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      test_text_box_home_end_keys!(RadarrHandler, active_radarr_block, filter);
    }
  }

  mod test_handle_delete {
    use pretty_assertions::assert_eq;

    use super::*;

    const DELETE_KEY: Key = DEFAULT_KEYBINDINGS.delete.key;

    #[test]
    fn test_movies_delete() {
      let mut app = App::default();

      RadarrHandler::with(&DELETE_KEY, &mut app, &ActiveRadarrBlock::Movies, &None).handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::DeleteMoviePrompt.into()
      );
    }

    #[test]
    fn test_downloads_delete() {
      let mut app = App::default();

      RadarrHandler::with(&DELETE_KEY, &mut app, &ActiveRadarrBlock::Downloads, &None).handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::DeleteDownloadPrompt.into()
      );
    }
  }

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::test_text_box_left_right_keys;

    use super::*;

    #[rstest]
    #[case(ActiveRadarrBlock::Movies, 0, ActiveRadarrBlock::Collections)]
    #[case(ActiveRadarrBlock::Downloads, 1, ActiveRadarrBlock::Movies)]
    #[case(ActiveRadarrBlock::Collections, 2, ActiveRadarrBlock::Downloads)]
    fn test_movies_downloads_collections_left(
      #[case] active_radarr_block: ActiveRadarrBlock,
      #[case] index: usize,
      #[case] expected_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.data.radarr_data.main_tabs.set_index(index);

      RadarrHandler::with(
        &DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        &active_radarr_block,
        &None,
      )
      .handle();

      assert_eq!(
        app.data.radarr_data.main_tabs.get_active_route(),
        &expected_radarr_block.into()
      );
      assert_eq!(app.get_current_route(), &expected_radarr_block.into());
    }

    #[rstest]
    #[case(ActiveRadarrBlock::Movies, 0, ActiveRadarrBlock::Downloads)]
    #[case(ActiveRadarrBlock::Downloads, 1, ActiveRadarrBlock::Collections)]
    #[case(ActiveRadarrBlock::Collections, 2, ActiveRadarrBlock::Movies)]
    fn test_movie_downloads_collections_right(
      #[case] active_radarr_block: ActiveRadarrBlock,
      #[case] index: usize,
      #[case] expected_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.data.radarr_data.main_tabs.set_index(index);

      RadarrHandler::with(
        &DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        &active_radarr_block,
        &None,
      )
      .handle();

      assert_eq!(
        app.data.radarr_data.main_tabs.get_active_route(),
        &expected_radarr_block.into()
      );
      assert_eq!(app.get_current_route(), &expected_radarr_block.into());
    }

    #[rstest]
    fn test_left_right_prompt_toggle(
      #[values(
        ActiveRadarrBlock::DeleteMoviePrompt,
        ActiveRadarrBlock::DeleteDownloadPrompt,
        ActiveRadarrBlock::UpdateAllMoviesPrompt,
        ActiveRadarrBlock::UpdateAllCollectionsPrompt,
        ActiveRadarrBlock::UpdateDownloadsPrompt
      )]
      active_radarr_block: ActiveRadarrBlock,
      #[values(DEFAULT_KEYBINDINGS.left.key, DEFAULT_KEYBINDINGS.right.key)] key: Key,
    ) {
      let mut app = App::default();

      RadarrHandler::with(&key, &mut app, &active_radarr_block, &None).handle();

      assert!(app.data.radarr_data.prompt_confirm);

      RadarrHandler::with(&key, &mut app, &active_radarr_block, &None).handle();

      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[rstest]
    fn test_search_boxes_left_right_keys(
      #[values(ActiveRadarrBlock::SearchMovie, ActiveRadarrBlock::SearchCollection)]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      test_text_box_left_right_keys!(RadarrHandler, active_radarr_block, search);
    }

    #[rstest]
    fn test_filter_boxes_left_right_keys(
      #[values(ActiveRadarrBlock::FilterMovies, ActiveRadarrBlock::FilterCollections)]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      test_text_box_left_right_keys!(RadarrHandler, active_radarr_block, filter);
    }
  }

  mod test_handle_submit {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::network::radarr_network::RadarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[rstest]
    #[case(ActiveRadarrBlock::Movies, ActiveRadarrBlock::MovieDetails)]
    #[case(ActiveRadarrBlock::Collections, ActiveRadarrBlock::CollectionDetails)]
    fn test_movies_collections_details_submit(
      #[case] active_radarr_block: ActiveRadarrBlock,
      #[case] expected_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();

      RadarrHandler::with(&SUBMIT_KEY, &mut app, &active_radarr_block, &None).handle();

      assert_eq!(app.get_current_route(), &expected_radarr_block.into());
    }

    #[test]
    fn test_search_movie_submit() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .movies
        .set_items(extended_stateful_iterable_vec!(
          Movie,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.search = "Test 2".to_owned().into();

      RadarrHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::SearchMovie,
        &None,
      )
      .handle();

      assert_str_eq!(
        app.data.radarr_data.movies.current_selection().title.text,
        "Test 2"
      );
    }

    #[test]
    fn test_search_filtered_movies_submit() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .filtered_movies
        .set_items(extended_stateful_iterable_vec!(
          Movie,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.search = "Test 2".to_owned().into();

      RadarrHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::SearchMovie,
        &None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .filtered_movies
          .current_selection()
          .title
          .text,
        "Test 2"
      );
    }

    #[test]
    fn test_search_collections_submit() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .collections
        .set_items(extended_stateful_iterable_vec!(
          Collection,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.search = "Test 2".to_owned().into();

      RadarrHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::SearchCollection,
        &None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .collections
          .current_selection()
          .title
          .text,
        "Test 2"
      );
    }

    #[test]
    fn test_search_filtered_collections_submit() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .filtered_collections
        .set_items(extended_stateful_iterable_vec!(
          Collection,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.search = "Test 2".to_owned().into();

      RadarrHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::SearchCollection,
        &None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .filtered_collections
          .current_selection()
          .title
          .text,
        "Test 2"
      );
    }

    #[test]
    fn test_filter_movies_submit() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .movies
        .set_items(extended_stateful_iterable_vec!(
          Movie,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.filter = "Test".to_owned().into();

      RadarrHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::FilterMovies,
        &None,
      )
      .handle();

      assert_eq!(app.data.radarr_data.filtered_movies.items.len(), 3);
      assert_str_eq!(
        app
          .data
          .radarr_data
          .filtered_movies
          .current_selection()
          .title
          .text,
        "Test 1"
      );
    }

    #[test]
    fn test_filter_collections_submit() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .collections
        .set_items(extended_stateful_iterable_vec!(
          Collection,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.filter = "Test".to_owned().into();

      RadarrHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::FilterCollections,
        &None,
      )
      .handle();

      assert_eq!(app.data.radarr_data.filtered_collections.items.len(), 3);
      assert_str_eq!(
        app
          .data
          .radarr_data
          .filtered_collections
          .current_selection()
          .title
          .text,
        "Test 1"
      );
    }

    #[rstest]
    #[case(
      ActiveRadarrBlock::Movies,
      ActiveRadarrBlock::DeleteMoviePrompt,
      RadarrEvent::DeleteMovie
    )]
    #[case(
      ActiveRadarrBlock::Downloads,
      ActiveRadarrBlock::DeleteDownloadPrompt,
      RadarrEvent::DeleteDownload
    )]
    #[case(
      ActiveRadarrBlock::Movies,
      ActiveRadarrBlock::UpdateAllMoviesPrompt,
      RadarrEvent::UpdateAllMovies
    )]
    #[case(
      ActiveRadarrBlock::Downloads,
      ActiveRadarrBlock::UpdateDownloadsPrompt,
      RadarrEvent::UpdateDownloads
    )]
    #[case(
      ActiveRadarrBlock::Collections,
      ActiveRadarrBlock::UpdateAllCollectionsPrompt,
      RadarrEvent::UpdateCollections
    )]
    fn test_prompt_confirm_submit(
      #[case] base_route: ActiveRadarrBlock,
      #[case] prompt_block: ActiveRadarrBlock,
      #[case] expected_action: RadarrEvent,
    ) {
      let mut app = App::default();
      app.data.radarr_data.prompt_confirm = true;
      app.push_navigation_stack(base_route.into());
      app.push_navigation_stack(prompt_block.into());

      RadarrHandler::with(&SUBMIT_KEY, &mut app, &prompt_block, &None).handle();

      assert!(app.data.radarr_data.prompt_confirm);
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(expected_action)
      );
      assert_eq!(app.get_current_route(), &base_route.into());
    }

    #[rstest]
    #[case(ActiveRadarrBlock::Movies, ActiveRadarrBlock::DeleteMoviePrompt)]
    #[case(ActiveRadarrBlock::Downloads, ActiveRadarrBlock::DeleteDownloadPrompt)]
    #[case(ActiveRadarrBlock::Movies, ActiveRadarrBlock::UpdateAllMoviesPrompt)]
    #[case(ActiveRadarrBlock::Downloads, ActiveRadarrBlock::UpdateDownloadsPrompt)]
    #[case(
      ActiveRadarrBlock::Collections,
      ActiveRadarrBlock::UpdateAllCollectionsPrompt
    )]
    fn test_prompt_decline_submit(
      #[case] base_route: ActiveRadarrBlock,
      #[case] prompt_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(base_route.into());
      app.push_navigation_stack(prompt_block.into());

      RadarrHandler::with(&SUBMIT_KEY, &mut app, &prompt_block, &None).handle();

      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
      assert_eq!(app.get_current_route(), &base_route.into());
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::app::radarr::radarr_test_utils::create_test_radarr_data;
    use crate::{assert_filter_reset, assert_search_reset};

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    #[case(ActiveRadarrBlock::Movies, ActiveRadarrBlock::SearchMovie)]
    #[case(ActiveRadarrBlock::Collections, ActiveRadarrBlock::SearchCollection)]
    fn test_search_blocks_esc(
      #[case] base_block: ActiveRadarrBlock,
      #[case] search_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(base_block.into());
      app.push_navigation_stack(search_block.into());
      app.data.radarr_data = create_test_radarr_data();

      RadarrHandler::with(&ESC_KEY, &mut app, &search_block, &None).handle();

      assert_eq!(app.get_current_route(), &base_block.into());
      assert!(!app.should_ignore_quit_key);
      assert_search_reset!(app.data.radarr_data);
    }

    #[rstest]
    #[case(ActiveRadarrBlock::Movies, ActiveRadarrBlock::FilterMovies)]
    #[case(ActiveRadarrBlock::Collections, ActiveRadarrBlock::FilterCollections)]
    fn test_filter_blocks_esc(
      #[case] base_block: ActiveRadarrBlock,
      #[case] filter_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(base_block.into());
      app.push_navigation_stack(filter_block.into());
      app.data.radarr_data = create_test_radarr_data();

      RadarrHandler::with(&ESC_KEY, &mut app, &filter_block, &None).handle();

      assert_eq!(app.get_current_route(), &base_block.into());
      assert!(!app.should_ignore_quit_key);
      assert_filter_reset!(app.data.radarr_data);
    }

    #[rstest]
    #[case(ActiveRadarrBlock::Movies, ActiveRadarrBlock::DeleteMoviePrompt)]
    #[case(ActiveRadarrBlock::Movies, ActiveRadarrBlock::UpdateAllMoviesPrompt)]
    #[case(ActiveRadarrBlock::Downloads, ActiveRadarrBlock::DeleteDownloadPrompt)]
    #[case(ActiveRadarrBlock::Downloads, ActiveRadarrBlock::UpdateDownloadsPrompt)]
    #[case(
      ActiveRadarrBlock::Collections,
      ActiveRadarrBlock::UpdateAllCollectionsPrompt
    )]
    fn test_prompt_blocks_esc(
      #[case] base_block: ActiveRadarrBlock,
      #[case] prompt_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(base_block.into());
      app.push_navigation_stack(prompt_block.into());
      app.data.radarr_data.prompt_confirm = true;

      RadarrHandler::with(&ESC_KEY, &mut app, &prompt_block, &None).handle();

      assert_eq!(app.get_current_route(), &base_block.into());
      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[test]
    fn test_default_esc() {
      let mut app = App::default();
      app.error = "test error".to_owned().into();
      app.push_navigation_stack(ActiveRadarrBlock::Downloads.into());
      app.push_navigation_stack(ActiveRadarrBlock::Downloads.into());
      app.data.radarr_data = create_test_radarr_data();

      RadarrHandler::with(&ESC_KEY, &mut app, &ActiveRadarrBlock::Downloads, &None).handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Downloads.into()
      );
      assert!(app.error.text.is_empty());
      assert_search_reset!(app.data.radarr_data);
      assert_filter_reset!(app.data.radarr_data);
    }
  }

  mod test_handle_key_char {
    use bimap::BiMap;
    use pretty_assertions::{assert_eq, assert_str_eq};
    use rstest::rstest;
    use serde_json::Number;
    use strum::IntoEnumIterator;

    use crate::app::key_binding::DEFAULT_KEYBINDINGS;
    use crate::app::radarr::radarr_test_utils::create_test_radarr_data;
    use crate::app::radarr::RadarrData;
    use crate::models::radarr_models::MinimumAvailability;
    use crate::models::HorizontallyScrollableText;
    use crate::models::StatefulTable;

    use super::*;

    #[rstest]
    #[case(ActiveRadarrBlock::Movies, ActiveRadarrBlock::SearchMovie)]
    #[case(ActiveRadarrBlock::Collections, ActiveRadarrBlock::SearchCollection)]
    fn test_search_key(
      #[case] active_radarr_block: ActiveRadarrBlock,
      #[case] expected_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();

      RadarrHandler::with(
        &DEFAULT_KEYBINDINGS.search.key,
        &mut app,
        &active_radarr_block,
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &expected_radarr_block.into());
      assert!(app.data.radarr_data.is_searching);
      assert!(app.should_ignore_quit_key);
    }

    #[rstest]
    #[case(ActiveRadarrBlock::Movies, ActiveRadarrBlock::FilterMovies)]
    #[case(ActiveRadarrBlock::Collections, ActiveRadarrBlock::FilterCollections)]
    fn test_filter_key(
      #[case] active_radarr_block: ActiveRadarrBlock,
      #[case] expected_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();

      RadarrHandler::with(
        &DEFAULT_KEYBINDINGS.filter.key,
        &mut app,
        &active_radarr_block,
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &expected_radarr_block.into());
      assert!(app.data.radarr_data.is_filtering);
      assert!(app.should_ignore_quit_key);
    }

    #[test]
    fn test_movie_add() {
      let mut app = App::default();

      RadarrHandler::with(
        &DEFAULT_KEYBINDINGS.add.key,
        &mut app,
        &ActiveRadarrBlock::Movies,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMovieSearchInput.into()
      );
      assert!(app.should_ignore_quit_key);
    }

    #[test]
    fn test_movie_edit_key() {
      test_edit_movie_key!(
        RadarrHandler,
        ActiveRadarrBlock::Movies,
        ActiveRadarrBlock::Movies
      );
    }

    #[test]
    fn test_collection_edit_key() {
      test_edit_collection_key!(
        RadarrHandler,
        ActiveRadarrBlock::Collections,
        ActiveRadarrBlock::Collections
      );
    }

    #[rstest]
    #[case(ActiveRadarrBlock::Movies, ActiveRadarrBlock::UpdateAllMoviesPrompt)]
    #[case(ActiveRadarrBlock::Downloads, ActiveRadarrBlock::UpdateDownloadsPrompt)]
    #[case(
      ActiveRadarrBlock::Collections,
      ActiveRadarrBlock::UpdateAllCollectionsPrompt
    )]
    fn test_update_key(
      #[case] active_radarr_block: ActiveRadarrBlock,
      #[case] expected_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();

      RadarrHandler::with(
        &DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        &active_radarr_block,
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &expected_radarr_block.into());
    }

    #[rstest]
    fn test_refresh_key(
      #[values(
        ActiveRadarrBlock::Movies,
        ActiveRadarrBlock::Collections,
        ActiveRadarrBlock::Downloads
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(active_radarr_block.into());

      RadarrHandler::with(
        &DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        &active_radarr_block,
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &active_radarr_block.into());
      assert!(app.should_refresh);
    }

    #[rstest]
    fn test_search_boxes_backspace_key(
      #[values(ActiveRadarrBlock::SearchMovie, ActiveRadarrBlock::SearchCollection)]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.data.radarr_data.search = "Test".to_owned().into();

      RadarrHandler::with(
        &DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        &active_radarr_block,
        &None,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.search.text, "Tes");
    }

    #[rstest]
    fn test_filter_boxes_backspace_key(
      #[values(ActiveRadarrBlock::FilterMovies, ActiveRadarrBlock::FilterCollections)]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.data.radarr_data.filter = "Test".to_owned().into();

      RadarrHandler::with(
        &DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        &active_radarr_block,
        &None,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.filter.text, "Tes");
    }

    #[rstest]
    fn test_search_boxes_char_key(
      #[values(ActiveRadarrBlock::SearchMovie, ActiveRadarrBlock::SearchCollection)]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();

      RadarrHandler::with(&Key::Char('h'), &mut app, &active_radarr_block, &None).handle();

      assert_str_eq!(app.data.radarr_data.search.text, "h");
    }

    #[rstest]
    fn test_filter_boxes_char_key(
      #[values(ActiveRadarrBlock::FilterMovies, ActiveRadarrBlock::FilterCollections)]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();

      RadarrHandler::with(&Key::Char('h'), &mut app, &active_radarr_block, &None).handle();

      assert_str_eq!(app.data.radarr_data.filter.text, "h");
    }
  }

  #[test]
  fn test_search_table() {
    let mut app = App::default();
    app
      .data
      .radarr_data
      .movies
      .set_items(extended_stateful_iterable_vec!(
        Movie,
        HorizontallyScrollableText
      ));
    app.data.radarr_data.search = "Test 2".to_owned().into();
    app.data.radarr_data.is_searching = true;
    app.should_ignore_quit_key = true;
    app.push_navigation_stack(ActiveRadarrBlock::SearchMovie.into());

    let movies = &app.data.radarr_data.movies.items.clone();

    let index = RadarrHandler::with(
      &DEFAULT_KEYBINDINGS.submit.key,
      &mut app,
      &ActiveRadarrBlock::SearchMovie,
      &None,
    )
    .search_table(movies, |movie| &movie.title.text);

    assert_eq!(index, Some(1));
    assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
    assert!(!app.data.radarr_data.is_searching);
    assert!(!app.should_ignore_quit_key);
    assert!(app.data.radarr_data.search.text.is_empty());
  }

  #[test]
  fn test_search_table_no_search_hits() {
    let mut app = App::default();
    app
      .data
      .radarr_data
      .movies
      .set_items(extended_stateful_iterable_vec!(
        Movie,
        HorizontallyScrollableText
      ));
    app.data.radarr_data.search = "Test 5".to_owned().into();
    app.data.radarr_data.is_searching = true;
    app.should_ignore_quit_key = true;
    app.push_navigation_stack(ActiveRadarrBlock::SearchMovie.into());

    let movies = &app.data.radarr_data.movies.items.clone();

    let index = RadarrHandler::with(
      &DEFAULT_KEYBINDINGS.submit.key,
      &mut app,
      &ActiveRadarrBlock::SearchMovie,
      &None,
    )
    .search_table(movies, |movie| &movie.title.text);

    assert_eq!(index, None);
    assert_eq!(
      app.get_current_route(),
      &ActiveRadarrBlock::SearchMovie.into()
    );
    assert!(!app.data.radarr_data.is_searching);
    assert!(!app.should_ignore_quit_key);
    assert!(app.data.radarr_data.search.text.is_empty());
  }

  #[test]
  fn test_filter_table() {
    let mut app = App::default();
    app
      .data
      .radarr_data
      .movies
      .set_items(extended_stateful_iterable_vec!(
        Movie,
        HorizontallyScrollableText
      ));
    app.data.radarr_data.filter = "Test 2".to_owned().into();
    app.data.radarr_data.is_searching = true;
    app.should_ignore_quit_key = true;
    app.push_navigation_stack(ActiveRadarrBlock::FilterMovies.into());

    let movies = &app.data.radarr_data.movies.items.clone();

    let filter_matches = RadarrHandler::with(
      &DEFAULT_KEYBINDINGS.submit.key,
      &mut app,
      &ActiveRadarrBlock::FilterMovies,
      &None,
    )
    .filter_table(movies, |movie| &movie.title.text);

    assert_eq!(filter_matches.len(), 1);
    assert_str_eq!(filter_matches[0].title.text, "Test 2");
    assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
    assert!(!app.data.radarr_data.is_filtering);
    assert!(!app.should_ignore_quit_key);
    assert!(app.data.radarr_data.filter.text.is_empty());
  }

  #[test]
  fn test_filter_table_no_filter_matches() {
    let mut app = App::default();
    app
      .data
      .radarr_data
      .movies
      .set_items(extended_stateful_iterable_vec!(
        Movie,
        HorizontallyScrollableText
      ));
    app.data.radarr_data.filter = "Test 5".to_owned().into();
    app.data.radarr_data.is_filtering = true;
    app.should_ignore_quit_key = true;
    app.push_navigation_stack(ActiveRadarrBlock::FilterMovies.into());

    let movies = &app.data.radarr_data.movies.items.clone();

    let filter_matches = RadarrHandler::with(
      &DEFAULT_KEYBINDINGS.submit.key,
      &mut app,
      &ActiveRadarrBlock::FilterMovies,
      &None,
    )
    .filter_table(movies, |movie| &movie.title.text);

    assert!(filter_matches.is_empty());
    assert_eq!(
      app.get_current_route(),
      &ActiveRadarrBlock::FilterMovies.into()
    );
    assert!(!app.data.radarr_data.is_searching);
    assert!(!app.should_ignore_quit_key);
    assert!(app.data.radarr_data.filter.text.is_empty());
  }

  #[rstest]
  fn test_delegates_add_movie_blocks_to_add_movie_handler(
    #[values(
      ActiveRadarrBlock::AddMovieSearchInput,
      ActiveRadarrBlock::AddMovieSearchResults,
      ActiveRadarrBlock::AddMoviePrompt,
      ActiveRadarrBlock::AddMovieSelectMonitor,
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
      ActiveRadarrBlock::AddMovieSelectQualityProfile,
      ActiveRadarrBlock::AddMovieAlreadyInLibrary,
      ActiveRadarrBlock::AddMovieTagsInput
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(ActiveRadarrBlock::Movies, active_radarr_block);
  }

  #[rstest]
  fn test_delegate_collection_details_blocks_to_collection_details_handler(
    #[values(
      ActiveRadarrBlock::CollectionDetails,
      ActiveRadarrBlock::ViewMovieOverview
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(ActiveRadarrBlock::Collections, active_radarr_block);
  }

  #[rstest]
  fn test_delegate_movie_details_blocks_to_movie_details_handler(
    #[values(
      ActiveRadarrBlock::MovieDetails,
      ActiveRadarrBlock::MovieHistory,
      ActiveRadarrBlock::FileInfo,
      ActiveRadarrBlock::Cast,
      ActiveRadarrBlock::Crew,
      ActiveRadarrBlock::AutomaticallySearchMoviePrompt,
      ActiveRadarrBlock::UpdateAndScanPrompt,
      ActiveRadarrBlock::ManualSearch,
      ActiveRadarrBlock::ManualSearchConfirmPrompt
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(ActiveRadarrBlock::Movies, active_radarr_block);
  }

  #[rstest]
  fn test_delegate_edit_movie_blocks_to_edit_movie_handler(
    #[values(
      ActiveRadarrBlock::EditMoviePrompt,
      ActiveRadarrBlock::EditMoviePathInput,
      ActiveRadarrBlock::EditMovieSelectMinimumAvailability,
      ActiveRadarrBlock::EditMovieSelectQualityProfile,
      ActiveRadarrBlock::EditMovieTagsInput,
      ActiveRadarrBlock::EditMovieToggleMonitored
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(ActiveRadarrBlock::Movies, active_radarr_block);
  }

  #[rstest]
  fn test_delegate_edit_collection_blocks_to_edit_collection_handler(
    #[values(
      ActiveRadarrBlock::EditCollectionPrompt,
      ActiveRadarrBlock::EditCollectionRootFolderPathInput,
      ActiveRadarrBlock::EditCollectionSelectMinimumAvailability,
      ActiveRadarrBlock::EditCollectionSelectQualityProfile,
      ActiveRadarrBlock::EditCollectionToggleSearchOnAdd,
      ActiveRadarrBlock::EditCollectionToggleMonitored
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(ActiveRadarrBlock::Collections, active_radarr_block);
  }
}
