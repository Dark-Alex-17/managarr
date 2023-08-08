use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::radarr::ActiveRadarrBlock;
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
      ActiveRadarrBlock::MovieDetails
      | ActiveRadarrBlock::MovieHistory
      | ActiveRadarrBlock::FileInfo
      | ActiveRadarrBlock::Cast
      | ActiveRadarrBlock::Crew
      | ActiveRadarrBlock::AutomaticallySearchMoviePrompt
      | ActiveRadarrBlock::RefreshAndScanPrompt
      | ActiveRadarrBlock::ManualSearch
      | ActiveRadarrBlock::ManualSearchConfirmPrompt => {
        MovieDetailsHandler::with(self.key, self.app, self.active_radarr_block).handle()
      }
      ActiveRadarrBlock::CollectionDetails | ActiveRadarrBlock::ViewMovieOverview => {
        CollectionDetailsHandler::with(self.key, self.app, self.active_radarr_block).handle()
      }
      ActiveRadarrBlock::AddMovieSearchInput
      | ActiveRadarrBlock::AddMovieSearchResults
      | ActiveRadarrBlock::AddMoviePrompt
      | ActiveRadarrBlock::AddMovieSelectMinimumAvailability
      | ActiveRadarrBlock::AddMovieSelectMonitor
      | ActiveRadarrBlock::AddMovieSelectQualityProfile => {
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
          |collection| &collection.title,
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
      ActiveRadarrBlock::SearchMovie
      | ActiveRadarrBlock::SearchCollection
      | ActiveRadarrBlock::FilterMovies
      | ActiveRadarrBlock::FilterCollections => {
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
          self.app.data.radarr_data.is_searching = true;
          self.app.should_ignore_quit_key = true;
        }
        _ if *key == DEFAULT_KEYBINDINGS.refresh.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::RefreshAllCollectionsPrompt.into());
        }
        _ => (),
      },
      ActiveRadarrBlock::SearchMovie | ActiveRadarrBlock::SearchCollection => {
        handle_text_box_keys!(self, key, self.app.data.radarr_data.search)
      }
      ActiveRadarrBlock::FilterMovies | ActiveRadarrBlock::FilterCollections => {
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

#[cfg(test)]
mod tests {
  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::app::radarr::ActiveRadarrBlock;
    use crate::app::App;
    use crate::event::Key;
    use crate::handlers::radarr_handlers::RadarrHandler;
    use crate::handlers::KeyEventHandler;
    use crate::models::radarr_models::{Collection, DownloadRecord, Movie};
    use crate::simple_stateful_iterable_vec;

    #[rstest]
    fn test_collections_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .collections
        .set_items(simple_stateful_iterable_vec!(Collection));

      RadarrHandler::with(&key, &mut app, &ActiveRadarrBlock::Collections).handle();

      assert_eq!(
        app.data.radarr_data.collections.current_selection().title,
        "Test 2".to_owned()
      );

      RadarrHandler::with(&key, &mut app, &ActiveRadarrBlock::Collections).handle();

      assert_eq!(
        app.data.radarr_data.collections.current_selection().title,
        "Test 1".to_owned()
      );
    }

    #[rstest]
    fn test_filtered_collections_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .filtered_collections
        .set_items(simple_stateful_iterable_vec!(Collection));

      RadarrHandler::with(&key, &mut app, &ActiveRadarrBlock::Collections).handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .filtered_collections
          .current_selection()
          .title,
        "Test 2".to_owned()
      );

      RadarrHandler::with(&key, &mut app, &ActiveRadarrBlock::Collections).handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .filtered_collections
          .current_selection()
          .title,
        "Test 1".to_owned()
      );
    }

    #[rstest]
    fn test_movies_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .movies
        .set_items(simple_stateful_iterable_vec!(Movie));

      RadarrHandler::with(&key, &mut app, &ActiveRadarrBlock::Movies).handle();

      assert_eq!(
        app.data.radarr_data.movies.current_selection().title,
        "Test 2".to_owned()
      );

      RadarrHandler::with(&key, &mut app, &ActiveRadarrBlock::Movies).handle();

      assert_eq!(
        app.data.radarr_data.movies.current_selection().title,
        "Test 1".to_owned()
      );
    }

    #[rstest]
    fn test_filtered_movies_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .filtered_movies
        .set_items(simple_stateful_iterable_vec!(Movie));

      RadarrHandler::with(&key, &mut app, &ActiveRadarrBlock::Movies).handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .filtered_movies
          .current_selection()
          .title,
        "Test 2".to_owned()
      );

      RadarrHandler::with(&key, &mut app, &ActiveRadarrBlock::Movies).handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .filtered_movies
          .current_selection()
          .title,
        "Test 1".to_owned()
      );
    }

    #[rstest]
    fn test_downloads_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .downloads
        .set_items(simple_stateful_iterable_vec!(DownloadRecord));

      RadarrHandler::with(&key, &mut app, &ActiveRadarrBlock::Downloads).handle();

      assert_eq!(
        app.data.radarr_data.downloads.current_selection().title,
        "Test 2".to_owned()
      );

      RadarrHandler::with(&key, &mut app, &ActiveRadarrBlock::Downloads).handle();

      assert_eq!(
        app.data.radarr_data.downloads.current_selection().title,
        "Test 1".to_owned()
      );
    }
  }

  mod test_handle_home_end {
    use pretty_assertions::assert_eq;

    use crate::app::radarr::ActiveRadarrBlock;
    use crate::app::App;
    use crate::event::Key;
    use crate::extended_stateful_iterable_vec;
    use crate::handlers::radarr_handlers::RadarrHandler;
    use crate::handlers::KeyEventHandler;
    use crate::models::radarr_models::{Collection, DownloadRecord, Movie};

    #[test]
    fn test_collections_home_end() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .collections
        .set_items(extended_stateful_iterable_vec!(Collection));

      RadarrHandler::with(&Key::End, &mut app, &ActiveRadarrBlock::Collections).handle();

      assert_eq!(
        app.data.radarr_data.collections.current_selection().title,
        "Test 3".to_owned()
      );

      RadarrHandler::with(&Key::Home, &mut app, &ActiveRadarrBlock::Collections).handle();

      assert_eq!(
        app.data.radarr_data.collections.current_selection().title,
        "Test 1".to_owned()
      );
    }

    #[test]
    fn test_filtered_collections_home_end() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .filtered_collections
        .set_items(extended_stateful_iterable_vec!(Collection));

      RadarrHandler::with(&Key::End, &mut app, &ActiveRadarrBlock::Collections).handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .filtered_collections
          .current_selection()
          .title,
        "Test 3".to_owned()
      );

      RadarrHandler::with(&Key::Home, &mut app, &ActiveRadarrBlock::Collections).handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .filtered_collections
          .current_selection()
          .title,
        "Test 1".to_owned()
      );
    }

    #[test]
    fn test_movies_home_end() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .movies
        .set_items(extended_stateful_iterable_vec!(Movie));

      RadarrHandler::with(&Key::End, &mut app, &ActiveRadarrBlock::Movies).handle();

      assert_eq!(
        app.data.radarr_data.movies.current_selection().title,
        "Test 3".to_owned()
      );

      RadarrHandler::with(&Key::Home, &mut app, &ActiveRadarrBlock::Movies).handle();

      assert_eq!(
        app.data.radarr_data.movies.current_selection().title,
        "Test 1".to_owned()
      );
    }

    #[test]
    fn test_filtered_movies_home_end() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .filtered_movies
        .set_items(extended_stateful_iterable_vec!(Movie));

      RadarrHandler::with(&Key::End, &mut app, &ActiveRadarrBlock::Movies).handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .filtered_movies
          .current_selection()
          .title,
        "Test 3".to_owned()
      );

      RadarrHandler::with(&Key::Home, &mut app, &ActiveRadarrBlock::Movies).handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .filtered_movies
          .current_selection()
          .title,
        "Test 1".to_owned()
      );
    }

    #[test]
    fn test_downloads_home_end() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .downloads
        .set_items(extended_stateful_iterable_vec!(DownloadRecord));

      RadarrHandler::with(&Key::End, &mut app, &ActiveRadarrBlock::Downloads).handle();

      assert_eq!(
        app.data.radarr_data.downloads.current_selection().title,
        "Test 3".to_owned()
      );

      RadarrHandler::with(&Key::Home, &mut app, &ActiveRadarrBlock::Downloads).handle();

      assert_eq!(
        app.data.radarr_data.downloads.current_selection().title,
        "Test 1".to_owned()
      );
    }
  }
}
