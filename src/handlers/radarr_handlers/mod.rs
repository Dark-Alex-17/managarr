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
  use pretty_assertions::assert_eq;

  use crate::app::radarr::ActiveRadarrBlock;
  use crate::app::App;
  use crate::event::Key;
  use crate::extended_stateful_iterable_vec;
  use crate::handlers::radarr_handlers::RadarrHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::radarr_models::{Collection, Movie};

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::models::radarr_models::DownloadRecord;
    use crate::simple_stateful_iterable_vec;

    use super::*;

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

    use crate::extended_stateful_iterable_vec;
    use crate::models::radarr_models::DownloadRecord;

    use super::*;

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

  mod test_delete {
    use pretty_assertions::assert_eq;

    use super::*;

    const DELETE_KEY: Key = Key::Delete;

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

  mod test_left_right_action {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(ActiveRadarrBlock::Movies, 0, ActiveRadarrBlock::Collections)]
    #[case(ActiveRadarrBlock::Downloads, 1, ActiveRadarrBlock::Movies)]
    #[case(ActiveRadarrBlock::Collections, 2, ActiveRadarrBlock::Downloads)]
    fn test_left_movies_downloads_collections(
      #[case] active_radarr_block: ActiveRadarrBlock,
      #[case] index: usize,
      #[case] expected_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.data.radarr_data.main_tabs.set_index(index);

      RadarrHandler::with(&Key::Left, &mut app, &active_radarr_block).handle();

      assert_eq!(
        app.data.radarr_data.main_tabs.get_active_route(),
        &expected_radarr_block.clone().into()
      );
      assert_eq!(app.get_current_route(), &expected_radarr_block.into());
    }

    #[rstest]
    #[case(ActiveRadarrBlock::Movies, 0, ActiveRadarrBlock::Downloads)]
    #[case(ActiveRadarrBlock::Downloads, 1, ActiveRadarrBlock::Collections)]
    #[case(ActiveRadarrBlock::Collections, 2, ActiveRadarrBlock::Movies)]
    fn test_right_movie_downloads_collections(
      #[case] active_radarr_block: ActiveRadarrBlock,
      #[case] index: usize,
      #[case] expected_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.data.radarr_data.main_tabs.set_index(index);

      RadarrHandler::with(&Key::Right, &mut app, &active_radarr_block).handle();

      assert_eq!(
        app.data.radarr_data.main_tabs.get_active_route(),
        &expected_radarr_block.clone().into()
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
      #[values(Key::Left, Key::Right)] key: Key,
    ) {
      let mut app = App::default();

      RadarrHandler::with(&key, &mut app, &active_radarr_block).handle();

      assert!(app.data.radarr_data.prompt_confirm);

      RadarrHandler::with(&key, &mut app, &active_radarr_block).handle();

      assert!(!app.data.radarr_data.prompt_confirm);
    }
  }

  mod test_submit {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::network::radarr_network::RadarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = Key::Enter;

    #[rstest]
    #[case(ActiveRadarrBlock::Movies, ActiveRadarrBlock::MovieDetails)]
    #[case(ActiveRadarrBlock::Collections, ActiveRadarrBlock::CollectionDetails)]
    fn test_submit_movies_collections_details(
      #[case] active_radarr_block: ActiveRadarrBlock,
      #[case] expected_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();

      RadarrHandler::with(&SUBMIT_KEY, &mut app, &active_radarr_block).handle();

      assert_eq!(app.get_current_route(), &expected_radarr_block.into());
    }

    #[test]
    fn test_submit_search_movie() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .movies
        .set_items(extended_stateful_iterable_vec!(Movie));
      app.data.radarr_data.search = "Test 2".to_owned();

      RadarrHandler::with(&SUBMIT_KEY, &mut app, &ActiveRadarrBlock::SearchMovie).handle();

      assert_eq!(
        app.data.radarr_data.movies.current_selection().title,
        "Test 2".to_owned()
      );
    }

    #[test]
    fn test_submit_search_collections() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .collections
        .set_items(extended_stateful_iterable_vec!(Collection));
      app.data.radarr_data.search = "Test 2".to_owned();

      RadarrHandler::with(&SUBMIT_KEY, &mut app, &ActiveRadarrBlock::SearchCollection).handle();

      assert_eq!(
        app.data.radarr_data.collections.current_selection().title,
        "Test 2".to_owned()
      );
    }

    #[test]
    fn test_submit_filter_movies() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .movies
        .set_items(extended_stateful_iterable_vec!(Movie));
      app.data.radarr_data.filter = "Test".to_owned();

      RadarrHandler::with(&SUBMIT_KEY, &mut app, &ActiveRadarrBlock::FilterMovies).handle();

      assert_eq!(app.data.radarr_data.filtered_movies.items.len(), 3);
      assert_eq!(
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
    fn test_submit_filter_collections() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .collections
        .set_items(extended_stateful_iterable_vec!(Collection));
      app.data.radarr_data.filter = "Test".to_owned();

      RadarrHandler::with(&SUBMIT_KEY, &mut app, &ActiveRadarrBlock::FilterCollections).handle();

      assert_eq!(app.data.radarr_data.filtered_collections.items.len(), 3);
      assert_eq!(
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
    fn test_submit_prompt_confirm(
      #[case] base_route: ActiveRadarrBlock,
      #[case] prompt_block: ActiveRadarrBlock,
      #[case] expected_action: RadarrEvent,
    ) {
      let mut app = App::default();
      app.data.radarr_data.prompt_confirm = true;
      app.push_navigation_stack(base_route.clone().into());
      app.push_navigation_stack(prompt_block.clone().into());

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
    fn test_submit_prompt_decline(
      #[case] base_route: ActiveRadarrBlock,
      #[case] prompt_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(base_route.clone().into());
      app.push_navigation_stack(prompt_block.clone().into());

      RadarrHandler::with(&SUBMIT_KEY, &mut app, &prompt_block).handle();

      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
      assert_eq!(app.get_current_route(), &base_route.into());
    }
  }

  mod test_esc {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::app::radarr::RadarrData;
    use crate::models::radarr_models::AddMovieSearchResult;

    use super::*;

    const ESC_KEY: Key = Key::Esc;

    #[rstest]
    #[case(ActiveRadarrBlock::Movies, ActiveRadarrBlock::SearchMovie)]
    #[case(ActiveRadarrBlock::Movies, ActiveRadarrBlock::FilterMovies)]
    #[case(ActiveRadarrBlock::Collections, ActiveRadarrBlock::SearchCollection)]
    #[case(ActiveRadarrBlock::Collections, ActiveRadarrBlock::FilterCollections)]
    fn test_esc_search_and_filter_blocks(
      #[case] base_block: ActiveRadarrBlock,
      #[case] search_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(base_block.clone().into());
      app.push_navigation_stack(search_block.clone().into());
      let mut radarr_data = RadarrData {
        is_searching: true,
        search: "test search".to_owned(),
        filter: "test filter".to_owned(),
        ..RadarrData::default()
      };
      radarr_data
        .filtered_movies
        .set_items(vec![Movie::default()]);
      radarr_data
        .filtered_collections
        .set_items(vec![Collection::default()]);
      radarr_data
        .add_searched_movies
        .set_items(vec![AddMovieSearchResult::default()]);
      app.data.radarr_data = radarr_data;

      RadarrHandler::with(&ESC_KEY, &mut app, &search_block).handle();

      assert_eq!(app.get_current_route(), &base_block.into());
      assert!(!app.should_ignore_quit_key);
      assert!(!app.data.radarr_data.is_searching);
      assert!(app.data.radarr_data.search.is_empty());
      assert!(app.data.radarr_data.filter.is_empty());
      assert!(app.data.radarr_data.filtered_movies.items.is_empty());
      assert!(app.data.radarr_data.filtered_collections.items.is_empty());
      assert!(app.data.radarr_data.add_searched_movies.items.is_empty());
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
    fn test_esc_prompt_blocks(
      #[case] base_block: ActiveRadarrBlock,
      #[case] prompt_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(base_block.clone().into());
      app.push_navigation_stack(prompt_block.clone().into());
      app.data.radarr_data.prompt_confirm = true;

      RadarrHandler::with(&ESC_KEY, &mut app, &prompt_block).handle();

      assert_eq!(app.get_current_route(), &base_block.into());
      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[test]
    fn test_esc_default() {
      let mut app = App::default();
      app.error = "test error".to_owned().into();
      app.push_navigation_stack(ActiveRadarrBlock::Downloads.into());
      app.push_navigation_stack(ActiveRadarrBlock::Downloads.into());
      let mut radarr_data = RadarrData {
        is_searching: true,
        search: "test search".to_owned(),
        filter: "test filter".to_owned(),
        ..RadarrData::default()
      };
      radarr_data
        .filtered_movies
        .set_items(vec![Movie::default()]);
      radarr_data
        .filtered_collections
        .set_items(vec![Collection::default()]);
      radarr_data
        .add_searched_movies
        .set_items(vec![AddMovieSearchResult::default()]);
      app.data.radarr_data = radarr_data;

      RadarrHandler::with(&ESC_KEY, &mut app, &ActiveRadarrBlock::Downloads).handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Downloads.into()
      );
      assert!(app.error.text.is_empty());
      assert!(!app.data.radarr_data.is_searching);
      assert!(app.data.radarr_data.search.is_empty());
      assert!(app.data.radarr_data.filter.is_empty());
      assert!(app.data.radarr_data.filtered_movies.items.is_empty());
      assert!(app.data.radarr_data.filtered_collections.items.is_empty());
      assert!(app.data.radarr_data.add_searched_movies.items.is_empty());
    }
  }

  mod test_key_char {
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
      assert!(app.data.radarr_data.is_searching);
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
    fn test_backspace_key_search_boxes(
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

      assert_eq!(app.data.radarr_data.search, "Tes".to_owned());
    }

    #[rstest]
    fn test_backspace_key_filter_boxes(
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

      assert_eq!(app.data.radarr_data.filter, "Tes".to_owned());
    }

    #[rstest]
    fn test_char_key_search_boxes(
      #[values(ActiveRadarrBlock::SearchMovie, ActiveRadarrBlock::SearchCollection)]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();

      RadarrHandler::with(&Key::Char('h'), &mut app, &active_radarr_block).handle();

      assert_eq!(app.data.radarr_data.search, "h".to_owned());
    }

    #[rstest]
    fn test_char_key_filter_boxes(
      #[values(ActiveRadarrBlock::FilterMovies, ActiveRadarrBlock::FilterCollections)]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();

      RadarrHandler::with(&Key::Char('h'), &mut app, &active_radarr_block).handle();

      assert_eq!(app.data.radarr_data.filter, "h".to_owned());
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

    let index = RadarrHandler::with(&Key::Enter, &mut app, &ActiveRadarrBlock::SearchMovie)
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

    let index = RadarrHandler::with(&Key::Enter, &mut app, &ActiveRadarrBlock::SearchMovie)
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

    let filter_matches =
      RadarrHandler::with(&Key::Enter, &mut app, &ActiveRadarrBlock::FilterMovies)
        .filter_table(movies, |movie| &movie.title);

    assert_eq!(filter_matches.len(), 1);
    assert_eq!(filter_matches[0].title, "Test 2");
    assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
    assert!(!app.data.radarr_data.is_searching);
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
    app.data.radarr_data.is_searching = true;
    app.should_ignore_quit_key = true;
    app.push_navigation_stack(ActiveRadarrBlock::FilterMovies.into());

    let movies = &app.data.radarr_data.movies.items.clone();

    let filter_matches =
      RadarrHandler::with(&Key::Enter, &mut app, &ActiveRadarrBlock::FilterMovies)
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
}
