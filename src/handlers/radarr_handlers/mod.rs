use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::radarr::{
  ActiveRadarrBlock, ADD_MOVIE_BLOCKS, COLLECTION_DETAILS_BLOCKS, FILTER_BLOCKS,
  MOVIE_DETAILS_BLOCKS, SEARCH_BLOCKS,
};
use crate::handlers::radarr_handlers::add_movie_handler::AddMovieHandler;
use crate::handlers::radarr_handlers::collection_details_handler::CollectionDetailsHandler;
use crate::handlers::radarr_handlers::movie_details_handler::MovieDetailsHandler;
use crate::handlers::{handle_clear_errors, handle_prompt_toggle, KeyEventHandler};
use crate::models::Scrollable;
use crate::network::radarr_network::RadarrEvent;
use crate::utils::strip_non_alphanumeric_characters;
use crate::{handle_text_box_keys, App, Key};

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
      _ if MOVIE_DETAILS_BLOCKS.contains(self.active_radarr_block) => {
        MovieDetailsHandler::with(self.key, self.app, self.active_radarr_block).handle()
      }
      _ if COLLECTION_DETAILS_BLOCKS.contains(self.active_radarr_block) => {
        CollectionDetailsHandler::with(self.key, self.app, self.active_radarr_block).handle()
      }
      _ if ADD_MOVIE_BLOCKS.contains(self.active_radarr_block) => {
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
      | ActiveRadarrBlock::RefreshAllMoviesPrompt
      | ActiveRadarrBlock::RefreshAllCollectionsPrompt
      | ActiveRadarrBlock::RefreshDownloadsPrompt => handle_prompt_toggle(self.app, self.key),
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
              &movie.title
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
            |movie| &movie.title,
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
            |collection| &collection.title,
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
            |collection| &collection.title,
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
      ActiveRadarrBlock::DeleteDownloadPrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::DeleteDownload);
        }

        self.app.pop_navigation_stack();
      }
      ActiveRadarrBlock::RefreshAllMoviesPrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::UpdateAllMovies);
        }

        self.app.pop_navigation_stack();
      }
      ActiveRadarrBlock::RefreshDownloadsPrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::RefreshDownloads);
        }

        self.app.pop_navigation_stack();
      }
      ActiveRadarrBlock::RefreshAllCollectionsPrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::RefreshCollections);
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
      | ActiveRadarrBlock::RefreshAllMoviesPrompt
      | ActiveRadarrBlock::RefreshAllCollectionsPrompt
      | ActiveRadarrBlock::RefreshDownloadsPrompt => {
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
        _ if *key == DEFAULT_KEYBINDINGS.add.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::AddMovieSearchInput.into());
          self.app.should_ignore_quit_key = true;
        }
        _ if *key == DEFAULT_KEYBINDINGS.refresh.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::RefreshAllMoviesPrompt.into());
        }
        _ => (),
      },
      ActiveRadarrBlock::Downloads => match self.key {
        _ if *key == DEFAULT_KEYBINDINGS.refresh.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::RefreshDownloadsPrompt.into());
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
        _ if *key == DEFAULT_KEYBINDINGS.refresh.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::RefreshAllCollectionsPrompt.into());
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
    let search_string = self
      .app
      .data
      .radarr_data
      .search
      .drain(..)
      .collect::<String>()
      .to_lowercase();
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

    self.app.data.radarr_data.is_filtering = false;
    self.app.should_ignore_quit_key = false;

    if !filter_matches.is_empty() {
      self.app.pop_navigation_stack();
    }

    filter_matches
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
      Collection,
      ActiveRadarrBlock::Collections,
      title
    );

    test_iterable_scroll!(
      test_filtered_collections_scroll,
      RadarrHandler,
      filtered_collections,
      Collection,
      ActiveRadarrBlock::Collections,
      title
    );

    test_iterable_scroll!(
      test_movies_scroll,
      RadarrHandler,
      movies,
      Movie,
      ActiveRadarrBlock::Movies,
      title
    );

    test_iterable_scroll!(
      test_filtered_movies_scroll,
      RadarrHandler,
      filtered_movies,
      Movie,
      ActiveRadarrBlock::Movies,
      title
    );

    test_iterable_scroll!(
      test_downloads_scroll,
      RadarrHandler,
      downloads,
      DownloadRecord,
      ActiveRadarrBlock::Downloads,
      title
    );
  }

  mod test_handle_home_end {
    use crate::models::radarr_models::DownloadRecord;
    use crate::{extended_stateful_iterable_vec, test_iterable_home_and_end};

    use super::*;

    test_iterable_home_and_end!(
      test_collections_home_end,
      RadarrHandler,
      collections,
      Collection,
      ActiveRadarrBlock::Collections,
      title
    );

    test_iterable_home_and_end!(
      test_filtered_collections_home_end,
      RadarrHandler,
      filtered_collections,
      Collection,
      ActiveRadarrBlock::Collections,
      title
    );

    test_iterable_home_and_end!(
      test_movies_home_end,
      RadarrHandler,
      movies,
      Movie,
      ActiveRadarrBlock::Movies,
      title
    );

    test_iterable_home_and_end!(
      test_filtered_movies_home_end,
      RadarrHandler,
      filtered_movies,
      Movie,
      ActiveRadarrBlock::Movies,
      title
    );

    test_iterable_home_and_end!(
      test_downloads_home_end,
      RadarrHandler,
      downloads,
      DownloadRecord,
      ActiveRadarrBlock::Downloads,
      title
    );
  }

  mod test_handle_delete {
    use pretty_assertions::assert_eq;

    use super::*;

    const DELETE_KEY: Key = DEFAULT_KEYBINDINGS.delete.key;

    #[test]
    fn test_movies_delete() {
      let mut app = App::default();

      RadarrHandler::with(&DELETE_KEY, &mut app, &ActiveRadarrBlock::Movies).handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::DeleteMoviePrompt.into()
      );
    }

    #[test]
    fn test_downloads_delete() {
      let mut app = App::default();

      RadarrHandler::with(&DELETE_KEY, &mut app, &ActiveRadarrBlock::Downloads).handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::DeleteDownloadPrompt.into()
      );
    }
  }

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

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
        ActiveRadarrBlock::RefreshAllMoviesPrompt,
        ActiveRadarrBlock::RefreshAllCollectionsPrompt,
        ActiveRadarrBlock::RefreshDownloadsPrompt
      )]
      active_radarr_block: ActiveRadarrBlock,
      #[values(DEFAULT_KEYBINDINGS.left.key, DEFAULT_KEYBINDINGS.right.key)] key: Key,
    ) {
      let mut app = App::default();

      RadarrHandler::with(&key, &mut app, &active_radarr_block).handle();

      assert!(app.data.radarr_data.prompt_confirm);

      RadarrHandler::with(&key, &mut app, &active_radarr_block).handle();

      assert!(!app.data.radarr_data.prompt_confirm);
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

      RadarrHandler::with(&SUBMIT_KEY, &mut app, &active_radarr_block).handle();

      assert_eq!(app.get_current_route(), &expected_radarr_block.into());
    }

    #[test]
    fn test_search_movie_submit() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .movies
        .set_items(extended_stateful_iterable_vec!(Movie));
      app.data.radarr_data.search = "Test 2".to_owned();

      RadarrHandler::with(&SUBMIT_KEY, &mut app, &ActiveRadarrBlock::SearchMovie).handle();

      assert_str_eq!(
        app.data.radarr_data.movies.current_selection().title,
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
        .set_items(extended_stateful_iterable_vec!(Movie));
      app.data.radarr_data.search = "Test 2".to_owned();

      RadarrHandler::with(&SUBMIT_KEY, &mut app, &ActiveRadarrBlock::SearchMovie).handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .filtered_movies
          .current_selection()
          .title,
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
        .set_items(extended_stateful_iterable_vec!(Collection));
      app.data.radarr_data.search = "Test 2".to_owned();

      RadarrHandler::with(&SUBMIT_KEY, &mut app, &ActiveRadarrBlock::SearchCollection).handle();

      assert_str_eq!(
        app.data.radarr_data.collections.current_selection().title,
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
        .set_items(extended_stateful_iterable_vec!(Collection));
      app.data.radarr_data.search = "Test 2".to_owned();

      RadarrHandler::with(&SUBMIT_KEY, &mut app, &ActiveRadarrBlock::SearchCollection).handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .filtered_collections
          .current_selection()
          .title,
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
        .set_items(extended_stateful_iterable_vec!(Movie));
      app.data.radarr_data.filter = "Test".to_owned();

      RadarrHandler::with(&SUBMIT_KEY, &mut app, &ActiveRadarrBlock::FilterMovies).handle();

      assert_eq!(app.data.radarr_data.filtered_movies.items.len(), 3);
      assert_str_eq!(
        app
          .data
          .radarr_data
          .filtered_movies
          .current_selection()
          .title,
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
        .set_items(extended_stateful_iterable_vec!(Collection));
      app.data.radarr_data.filter = "Test".to_owned();

      RadarrHandler::with(&SUBMIT_KEY, &mut app, &ActiveRadarrBlock::FilterCollections).handle();

      assert_eq!(app.data.radarr_data.filtered_collections.items.len(), 3);
      assert_str_eq!(
        app
          .data
          .radarr_data
          .filtered_collections
          .current_selection()
          .title,
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
      ActiveRadarrBlock::RefreshAllMoviesPrompt,
      RadarrEvent::UpdateAllMovies
    )]
    #[case(
      ActiveRadarrBlock::Downloads,
      ActiveRadarrBlock::RefreshDownloadsPrompt,
      RadarrEvent::RefreshDownloads
    )]
    #[case(
      ActiveRadarrBlock::Collections,
      ActiveRadarrBlock::RefreshAllCollectionsPrompt,
      RadarrEvent::RefreshCollections
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

      RadarrHandler::with(&SUBMIT_KEY, &mut app, &prompt_block).handle();

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
    #[case(ActiveRadarrBlock::Movies, ActiveRadarrBlock::RefreshAllMoviesPrompt)]
    #[case(
      ActiveRadarrBlock::Downloads,
      ActiveRadarrBlock::RefreshDownloadsPrompt
    )]
    #[case(
      ActiveRadarrBlock::Collections,
      ActiveRadarrBlock::RefreshAllCollectionsPrompt
    )]
    fn test_prompt_decline_submit(
      #[case] base_route: ActiveRadarrBlock,
      #[case] prompt_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(base_route.into());
      app.push_navigation_stack(prompt_block.into());

      RadarrHandler::with(&SUBMIT_KEY, &mut app, &prompt_block).handle();

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

      RadarrHandler::with(&ESC_KEY, &mut app, &search_block).handle();

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

      RadarrHandler::with(&ESC_KEY, &mut app, &filter_block).handle();

      assert_eq!(app.get_current_route(), &base_block.into());
      assert!(!app.should_ignore_quit_key);
      assert_filter_reset!(app.data.radarr_data);
    }

    #[rstest]
    #[case(ActiveRadarrBlock::Movies, ActiveRadarrBlock::DeleteMoviePrompt)]
    #[case(ActiveRadarrBlock::Movies, ActiveRadarrBlock::RefreshAllMoviesPrompt)]
    #[case(ActiveRadarrBlock::Downloads, ActiveRadarrBlock::DeleteDownloadPrompt)]
    #[case(
      ActiveRadarrBlock::Downloads,
      ActiveRadarrBlock::RefreshDownloadsPrompt
    )]
    #[case(
      ActiveRadarrBlock::Collections,
      ActiveRadarrBlock::RefreshAllCollectionsPrompt
    )]
    fn test_prompt_blocks_esc(
      #[case] base_block: ActiveRadarrBlock,
      #[case] prompt_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(base_block.into());
      app.push_navigation_stack(prompt_block.into());
      app.data.radarr_data.prompt_confirm = true;

      RadarrHandler::with(&ESC_KEY, &mut app, &prompt_block).handle();

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

      RadarrHandler::with(&ESC_KEY, &mut app, &ActiveRadarrBlock::Downloads).handle();

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
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::app::key_binding::DEFAULT_KEYBINDINGS;

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
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMovieSearchInput.into()
      );
      assert!(app.should_ignore_quit_key);
    }

    #[rstest]
    #[case(ActiveRadarrBlock::Movies, ActiveRadarrBlock::RefreshAllMoviesPrompt)]
    #[case(
      ActiveRadarrBlock::Downloads,
      ActiveRadarrBlock::RefreshDownloadsPrompt
    )]
    #[case(
      ActiveRadarrBlock::Collections,
      ActiveRadarrBlock::RefreshAllCollectionsPrompt
    )]
    fn test_refresh_key(
      #[case] active_radarr_block: ActiveRadarrBlock,
      #[case] expected_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();

      RadarrHandler::with(
        &DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        &active_radarr_block,
      )
      .handle();

      assert_eq!(app.get_current_route(), &expected_radarr_block.into());
    }

    #[rstest]
    fn test_search_boxes_backspace_key(
      #[values(ActiveRadarrBlock::SearchMovie, ActiveRadarrBlock::SearchCollection)]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.data.radarr_data.search = "Test".to_owned();

      RadarrHandler::with(
        &DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        &active_radarr_block,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.search, "Tes");
    }

    #[rstest]
    fn test_filter_boxes_backspace_key(
      #[values(ActiveRadarrBlock::FilterMovies, ActiveRadarrBlock::FilterCollections)]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.data.radarr_data.filter = "Test".to_owned();

      RadarrHandler::with(
        &DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        &active_radarr_block,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.filter, "Tes");
    }

    #[rstest]
    fn test_search_boxes_char_key(
      #[values(ActiveRadarrBlock::SearchMovie, ActiveRadarrBlock::SearchCollection)]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();

      RadarrHandler::with(&Key::Char('h'), &mut app, &active_radarr_block).handle();

      assert_str_eq!(app.data.radarr_data.search, "h");
    }

    #[rstest]
    fn test_filter_boxes_char_key(
      #[values(ActiveRadarrBlock::FilterMovies, ActiveRadarrBlock::FilterCollections)]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();

      RadarrHandler::with(&Key::Char('h'), &mut app, &active_radarr_block).handle();

      assert_str_eq!(app.data.radarr_data.filter, "h");
    }
  }

  #[test]
  fn test_search_table() {
    let mut app = App::default();
    app
      .data
      .radarr_data
      .movies
      .set_items(extended_stateful_iterable_vec!(Movie));
    app.data.radarr_data.search = "Test 2".to_owned();
    app.data.radarr_data.is_searching = true;
    app.should_ignore_quit_key = true;
    app.push_navigation_stack(ActiveRadarrBlock::SearchMovie.into());

    let movies = &app.data.radarr_data.movies.items.clone();

    let index = RadarrHandler::with(
      &DEFAULT_KEYBINDINGS.submit.key,
      &mut app,
      &ActiveRadarrBlock::SearchMovie,
    )
    .search_table(movies, |movie| &movie.title);

    assert_eq!(index, Some(1));
    assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
    assert!(!app.data.radarr_data.is_searching);
    assert!(!app.should_ignore_quit_key);
    assert!(app.data.radarr_data.search.is_empty());
  }

  #[test]
  fn test_search_table_no_search_hits() {
    let mut app = App::default();
    app
      .data
      .radarr_data
      .movies
      .set_items(extended_stateful_iterable_vec!(Movie));
    app.data.radarr_data.search = "Test 5".to_owned();
    app.data.radarr_data.is_searching = true;
    app.should_ignore_quit_key = true;
    app.push_navigation_stack(ActiveRadarrBlock::SearchMovie.into());

    let movies = &app.data.radarr_data.movies.items.clone();

    let index = RadarrHandler::with(
      &DEFAULT_KEYBINDINGS.submit.key,
      &mut app,
      &ActiveRadarrBlock::SearchMovie,
    )
    .search_table(movies, |movie| &movie.title);

    assert_eq!(index, None);
    assert_eq!(
      app.get_current_route(),
      &ActiveRadarrBlock::SearchMovie.into()
    );
    assert!(!app.data.radarr_data.is_searching);
    assert!(!app.should_ignore_quit_key);
    assert!(app.data.radarr_data.search.is_empty());
  }

  #[test]
  fn test_filter_table() {
    let mut app = App::default();
    app
      .data
      .radarr_data
      .movies
      .set_items(extended_stateful_iterable_vec!(Movie));
    app.data.radarr_data.filter = "Test 2".to_owned();
    app.data.radarr_data.is_searching = true;
    app.should_ignore_quit_key = true;
    app.push_navigation_stack(ActiveRadarrBlock::FilterMovies.into());

    let movies = &app.data.radarr_data.movies.items.clone();

    let filter_matches = RadarrHandler::with(
      &DEFAULT_KEYBINDINGS.submit.key,
      &mut app,
      &ActiveRadarrBlock::FilterMovies,
    )
    .filter_table(movies, |movie| &movie.title);

    assert_eq!(filter_matches.len(), 1);
    assert_str_eq!(filter_matches[0].title, "Test 2");
    assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
    assert!(!app.data.radarr_data.is_filtering);
    assert!(!app.should_ignore_quit_key);
    assert!(app.data.radarr_data.filter.is_empty());
  }

  #[test]
  fn test_filter_table_no_filter_matches() {
    let mut app = App::default();
    app
      .data
      .radarr_data
      .movies
      .set_items(extended_stateful_iterable_vec!(Movie));
    app.data.radarr_data.filter = "Test 5".to_owned();
    app.data.radarr_data.is_filtering = true;
    app.should_ignore_quit_key = true;
    app.push_navigation_stack(ActiveRadarrBlock::FilterMovies.into());

    let movies = &app.data.radarr_data.movies.items.clone();

    let filter_matches = RadarrHandler::with(
      &DEFAULT_KEYBINDINGS.submit.key,
      &mut app,
      &ActiveRadarrBlock::FilterMovies,
    )
    .filter_table(movies, |movie| &movie.title);

    assert!(filter_matches.is_empty());
    assert_eq!(
      app.get_current_route(),
      &ActiveRadarrBlock::FilterMovies.into()
    );
    assert!(!app.data.radarr_data.is_searching);
    assert!(!app.should_ignore_quit_key);
    assert!(app.data.radarr_data.filter.is_empty());
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
      ActiveRadarrBlock::AddMovieAlreadyInLibrary
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
      ActiveRadarrBlock::RefreshAndScanPrompt,
      ActiveRadarrBlock::ManualSearch,
      ActiveRadarrBlock::ManualSearchConfirmPrompt
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(ActiveRadarrBlock::Movies, active_radarr_block);
  }
}
