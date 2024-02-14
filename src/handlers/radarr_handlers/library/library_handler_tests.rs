#[cfg(test)]
mod tests {
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use std::cmp::Ordering;
  use strum::IntoEnumIterator;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::radarr_handlers::library::{movies_sorting_options, LibraryHandler};
  use crate::handlers::KeyEventHandler;
  use crate::models::radarr_models::{Language, Movie};
  use crate::models::servarr_data::radarr::radarr_data::{
    ActiveRadarrBlock, ADD_MOVIE_BLOCKS, DELETE_MOVIE_BLOCKS, EDIT_MOVIE_BLOCKS, LIBRARY_BLOCKS,
    MOVIE_DETAILS_BLOCKS,
  };
  use crate::models::stateful_table::SortOption;
  use crate::models::HorizontallyScrollableText;
  use crate::test_handler_delegation;

  mod test_handle_scroll_up_and_down {
    use crate::{simple_stateful_iterable_vec, test_iterable_scroll};
    use pretty_assertions::assert_eq;

    use super::*;

    test_iterable_scroll!(
      test_movies_scroll,
      LibraryHandler,
      movies,
      simple_stateful_iterable_vec!(Movie, HorizontallyScrollableText),
      ActiveRadarrBlock::Movies,
      None,
      title,
      to_string
    );

    #[rstest]
    fn test_movies_sort_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let movie_field_vec = sort_options();
      let mut app = App::default();
      app.data.radarr_data.movies.sorting(sort_options());

      if key == Key::Up {
        for i in (0..movie_field_vec.len()).rev() {
          LibraryHandler::with(&key, &mut app, &ActiveRadarrBlock::MoviesSortPrompt, &None)
            .handle();

          assert_eq!(
            app
              .data
              .radarr_data
              .movies
              .sort
              .as_ref()
              .unwrap()
              .current_selection(),
            &movie_field_vec[i]
          );
        }
      } else {
        for i in 0..movie_field_vec.len() {
          LibraryHandler::with(&key, &mut app, &ActiveRadarrBlock::MoviesSortPrompt, &None)
            .handle();

          assert_eq!(
            app
              .data
              .radarr_data
              .movies
              .sort
              .as_ref()
              .unwrap()
              .current_selection(),
            &movie_field_vec[(i + 1) % movie_field_vec.len()]
          );
        }
      }
    }
  }

  mod test_handle_home_end {
    use pretty_assertions::assert_eq;

    use crate::{extended_stateful_iterable_vec, test_iterable_home_and_end};

    use super::*;

    test_iterable_home_and_end!(
      test_movies_home_end,
      LibraryHandler,
      movies,
      extended_stateful_iterable_vec!(Movie, HorizontallyScrollableText),
      ActiveRadarrBlock::Movies,
      None,
      title,
      to_string
    );

    #[test]
    fn test_movie_search_box_home_end_keys() {
      let mut app = App::default();
      app.data.radarr_data.movies.search = Some("Test".into());

      LibraryHandler::with(
        &DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        &ActiveRadarrBlock::SearchMovie,
        &None,
      )
      .handle();

      assert_eq!(
        *app
          .data
          .radarr_data
          .movies
          .search
          .as_ref()
          .unwrap()
          .offset
          .borrow(),
        4
      );

      LibraryHandler::with(
        &DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        &ActiveRadarrBlock::SearchMovie,
        &None,
      )
      .handle();

      assert_eq!(
        *app
          .data
          .radarr_data
          .movies
          .search
          .as_ref()
          .unwrap()
          .offset
          .borrow(),
        0
      );
    }

    #[test]
    fn test_movie_filter_box_home_end_keys() {
      let mut app = App::default();
      app.data.radarr_data.movies.filter = Some("Test".into());

      LibraryHandler::with(
        &DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        &ActiveRadarrBlock::FilterMovies,
        &None,
      )
      .handle();

      assert_eq!(
        *app
          .data
          .radarr_data
          .movies
          .filter
          .as_ref()
          .unwrap()
          .offset
          .borrow(),
        4
      );

      LibraryHandler::with(
        &DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        &ActiveRadarrBlock::FilterMovies,
        &None,
      )
      .handle();

      assert_eq!(
        *app
          .data
          .radarr_data
          .movies
          .filter
          .as_ref()
          .unwrap()
          .offset
          .borrow(),
        0
      );
    }

    #[test]
    fn test_movies_sort_home_end() {
      let movie_field_vec = sort_options();
      let mut app = App::default();
      app.data.radarr_data.movies.sorting(sort_options());

      LibraryHandler::with(
        &DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        &ActiveRadarrBlock::MoviesSortPrompt,
        &None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .movies
          .sort
          .as_ref()
          .unwrap()
          .current_selection(),
        &movie_field_vec[movie_field_vec.len() - 1]
      );

      LibraryHandler::with(
        &DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        &ActiveRadarrBlock::MoviesSortPrompt,
        &None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .movies
          .sort
          .as_ref()
          .unwrap()
          .current_selection(),
        &movie_field_vec[0]
      );
    }
  }

  mod test_handle_delete {
    use pretty_assertions::assert_eq;

    use crate::assert_delete_prompt;
    use crate::models::servarr_data::radarr::radarr_data::DELETE_MOVIE_SELECTION_BLOCKS;

    use super::*;

    const DELETE_KEY: Key = DEFAULT_KEYBINDINGS.delete.key;

    #[test]
    fn test_movies_delete() {
      let mut app = App::default();

      assert_delete_prompt!(
        LibraryHandler,
        app,
        ActiveRadarrBlock::Movies,
        ActiveRadarrBlock::DeleteMoviePrompt
      );
      assert_eq!(
        app.data.radarr_data.selected_block.blocks,
        &DELETE_MOVIE_SELECTION_BLOCKS
      );
    }
  }

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_movie_tab_left() {
      let mut app = App::default();
      app.data.radarr_data.main_tabs.set_index(0);

      LibraryHandler::with(
        &DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        &ActiveRadarrBlock::Movies,
        &None,
      )
      .handle();

      assert_eq!(
        app.data.radarr_data.main_tabs.get_active_route(),
        &ActiveRadarrBlock::System.into()
      );
      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::System.into());
    }

    #[test]
    fn test_movie_tab_right() {
      let mut app = App::default();
      app.data.radarr_data.main_tabs.set_index(0);

      LibraryHandler::with(
        &DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        &ActiveRadarrBlock::Movies,
        &None,
      )
      .handle();

      assert_eq!(
        app.data.radarr_data.main_tabs.get_active_route(),
        &ActiveRadarrBlock::Downloads.into()
      );
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Downloads.into()
      );
    }

    #[rstest]
    fn test_left_right_update_all_movies_prompt_toggle(
      #[values(DEFAULT_KEYBINDINGS.left.key, DEFAULT_KEYBINDINGS.right.key)] key: Key,
    ) {
      let mut app = App::default();

      LibraryHandler::with(
        &key,
        &mut app,
        &ActiveRadarrBlock::UpdateAllMoviesPrompt,
        &None,
      )
      .handle();

      assert!(app.data.radarr_data.prompt_confirm);

      LibraryHandler::with(
        &key,
        &mut app,
        &ActiveRadarrBlock::UpdateAllMoviesPrompt,
        &None,
      )
      .handle();

      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[test]
    fn test_movie_search_box_left_right_keys() {
      let mut app = App::default();
      app.data.radarr_data.movies.search = Some("Test".into());

      LibraryHandler::with(
        &DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        &ActiveRadarrBlock::SearchMovie,
        &None,
      )
      .handle();

      assert_eq!(
        *app
          .data
          .radarr_data
          .movies
          .search
          .as_ref()
          .unwrap()
          .offset
          .borrow(),
        1
      );

      LibraryHandler::with(
        &DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        &ActiveRadarrBlock::SearchMovie,
        &None,
      )
      .handle();

      assert_eq!(
        *app
          .data
          .radarr_data
          .movies
          .search
          .as_ref()
          .unwrap()
          .offset
          .borrow(),
        0
      );
    }

    #[test]
    fn test_movie_filter_box_left_right_keys() {
      let mut app = App::default();
      app.data.radarr_data.movies.filter = Some("Test".into());

      LibraryHandler::with(
        &DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        &ActiveRadarrBlock::FilterMovies,
        &None,
      )
      .handle();

      assert_eq!(
        *app
          .data
          .radarr_data
          .movies
          .filter
          .as_ref()
          .unwrap()
          .offset
          .borrow(),
        1
      );

      LibraryHandler::with(
        &DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        &ActiveRadarrBlock::FilterMovies,
        &None,
      )
      .handle();

      assert_eq!(
        *app
          .data
          .radarr_data
          .movies
          .filter
          .as_ref()
          .unwrap()
          .offset
          .borrow(),
        0
      );
    }
  }

  mod test_handle_submit {
    use pretty_assertions::assert_eq;

    use crate::extended_stateful_iterable_vec;
    use crate::network::radarr_network::RadarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_movie_details_submit() {
      let mut app = App::default();

      LibraryHandler::with(&SUBMIT_KEY, &mut app, &ActiveRadarrBlock::Movies, &None).handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::MovieDetails.into()
      );
    }

    #[test]
    fn test_search_movie_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::SearchMovie.into());
      app
        .data
        .radarr_data
        .movies
        .set_items(extended_stateful_iterable_vec!(
          Movie,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.movies.search = Some("Test 2".into());

      LibraryHandler::with(
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
      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
    }

    #[test]
    fn test_search_movie_submit_error_on_no_search_hits() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::SearchMovie.into());
      app
        .data
        .radarr_data
        .movies
        .set_items(extended_stateful_iterable_vec!(
          Movie,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.movies.search = Some("Test 5".into());

      LibraryHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::SearchMovie,
        &None,
      )
      .handle();

      assert_str_eq!(
        app.data.radarr_data.movies.current_selection().title.text,
        "Test 1"
      );
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::SearchMovieError.into()
      );
    }

    #[test]
    fn test_search_filtered_movies_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::SearchMovie.into());
      app
        .data
        .radarr_data
        .movies
        .set_filtered_items(extended_stateful_iterable_vec!(
          Movie,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.movies.search = Some("Test 2".into());

      LibraryHandler::with(
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
      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
    }

    #[test]
    fn test_filter_movies_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::FilterMovies.into());
      app
        .data
        .radarr_data
        .movies
        .set_items(extended_stateful_iterable_vec!(
          Movie,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.movies.filter = Some("Test".into());

      LibraryHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::FilterMovies,
        &None,
      )
      .handle();

      assert!(app.data.radarr_data.movies.filtered_items.is_some());
      assert!(!app.should_ignore_quit_key);
      assert_eq!(
        app
          .data
          .radarr_data
          .movies
          .filtered_items
          .as_ref()
          .unwrap()
          .len(),
        3
      );
      assert_str_eq!(
        app.data.radarr_data.movies.current_selection().title.text,
        "Test 1"
      );
      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
    }

    #[test]
    fn test_filter_movies_submit_error_on_no_filter_matches() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::FilterMovies.into());
      app
        .data
        .radarr_data
        .movies
        .set_items(extended_stateful_iterable_vec!(
          Movie,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.movies.filter = Some("Test 5".into());

      LibraryHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::FilterMovies,
        &None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert!(app.data.radarr_data.movies.filtered_items.is_none());
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::FilterMoviesError.into()
      );
    }

    #[test]
    fn test_update_all_movies_prompt_confirm_submit() {
      let mut app = App::default();
      app.data.radarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::UpdateAllMoviesPrompt.into());

      LibraryHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::UpdateAllMoviesPrompt,
        &None,
      )
      .handle();

      assert!(app.data.radarr_data.prompt_confirm);
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::UpdateAllMovies)
      );
      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
    }

    #[test]
    fn test_update_all_movies_prompt_decline_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::UpdateAllMoviesPrompt.into());

      LibraryHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::UpdateAllMoviesPrompt,
        &None,
      )
      .handle();

      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
    }

    #[test]
    fn test_movies_sort_prompt_submit() {
      let mut app = App::default();
      app.data.radarr_data.movies.sort_asc = true;
      app.data.radarr_data.movies.sorting(sort_options());
      app.data.radarr_data.movies.set_items(movies_vec());
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::MoviesSortPrompt.into());

      let mut expected_vec = movies_vec();
      expected_vec.reverse();

      LibraryHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::MoviesSortPrompt,
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
      assert_eq!(app.data.radarr_data.movies.items, expected_vec);
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;
    use ratatui::widgets::TableState;

    use crate::models::servarr_data::radarr::radarr_data::radarr_test_utils::utils::create_test_radarr_data;
    use crate::models::stateful_table::StatefulTable;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_search_movie_block_esc(
      #[values(ActiveRadarrBlock::SearchMovie, ActiveRadarrBlock::SearchMovieError)]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(active_radarr_block.into());
      app.data.radarr_data = create_test_radarr_data();
      app.data.radarr_data.movies.search = Some("Test".into());

      LibraryHandler::with(&ESC_KEY, &mut app, &active_radarr_block, &None).handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
      assert!(!app.should_ignore_quit_key);
      assert_eq!(app.data.radarr_data.movies.search, None);
    }

    #[rstest]
    fn test_filter_movies_block_esc(
      #[values(ActiveRadarrBlock::FilterMovies, ActiveRadarrBlock::FilterMoviesError)]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(active_radarr_block.into());
      app.data.radarr_data = create_test_radarr_data();
      app.data.radarr_data.movies = StatefulTable {
        filter: Some("Test".into()),
        filtered_items: Some(Vec::new()),
        filtered_state: Some(TableState::default()),
        ..StatefulTable::default()
      };

      LibraryHandler::with(&ESC_KEY, &mut app, &active_radarr_block, &None).handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
      assert!(!app.should_ignore_quit_key);
      assert_eq!(app.data.radarr_data.movies.filter, None);
      assert_eq!(app.data.radarr_data.movies.filtered_items, None);
      assert_eq!(app.data.radarr_data.movies.filtered_state, None);
    }

    #[test]
    fn test_update_all_movies_prompt_blocks_esc() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::UpdateAllMoviesPrompt.into());
      app.data.radarr_data.prompt_confirm = true;

      LibraryHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::UpdateAllMoviesPrompt,
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[test]
    fn test_default_esc() {
      let mut app = App::default();
      app.error = "test error".to_owned().into();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.data.radarr_data = create_test_radarr_data();
      app.data.radarr_data.movies = StatefulTable {
        search: Some("Test".into()),
        filter: Some("Test".into()),
        filtered_items: Some(Vec::new()),
        filtered_state: Some(TableState::default()),
        ..StatefulTable::default()
      };

      LibraryHandler::with(&ESC_KEY, &mut app, &ActiveRadarrBlock::Movies, &None).handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
      assert!(app.error.text.is_empty());
      assert_eq!(app.data.radarr_data.movies.search, None);
      assert_eq!(app.data.radarr_data.movies.filter, None);
      assert_eq!(app.data.radarr_data.movies.filtered_items, None);
      assert_eq!(app.data.radarr_data.movies.filtered_state, None);
    }
  }

  mod test_handle_key_char {
    use bimap::BiMap;
    use pretty_assertions::{assert_eq, assert_str_eq};
    use serde_json::Number;
    use strum::IntoEnumIterator;

    use crate::models::radarr_models::MinimumAvailability;
    use crate::models::servarr_data::radarr::radarr_data::radarr_test_utils::utils::create_test_radarr_data;
    use crate::models::servarr_data::radarr::radarr_data::{
      RadarrData, EDIT_MOVIE_SELECTION_BLOCKS,
    };

    use crate::models::stateful_table::StatefulTable;
    use crate::{assert_refresh_key, test_edit_movie_key};

    use super::*;

    #[test]
    fn test_search_movies_key() {
      let mut app = App::default();

      LibraryHandler::with(
        &DEFAULT_KEYBINDINGS.search.key,
        &mut app,
        &ActiveRadarrBlock::Movies,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::SearchMovie.into()
      );
      assert!(app.should_ignore_quit_key);
      assert_eq!(
        app.data.radarr_data.movies.search,
        Some(HorizontallyScrollableText::default())
      );
    }

    #[test]
    fn test_filter_movies_key() {
      let mut app = App::default();
      app.data.radarr_data.movies = StatefulTable::default();

      LibraryHandler::with(
        &DEFAULT_KEYBINDINGS.filter.key,
        &mut app,
        &ActiveRadarrBlock::Movies,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::FilterMovies.into()
      );
      assert!(app.should_ignore_quit_key);
      assert!(app.data.radarr_data.movies.filter.is_some());
    }

    #[test]
    fn test_filter_movies_key_resets_previous_filter() {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.data.radarr_data = create_test_radarr_data();
      app.data.radarr_data.movies = StatefulTable::default();
      app.data.radarr_data.movies.filter = Some("Test".into());

      LibraryHandler::with(
        &DEFAULT_KEYBINDINGS.filter.key,
        &mut app,
        &ActiveRadarrBlock::Movies,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::FilterMovies.into()
      );
      assert!(app.should_ignore_quit_key);
      assert_eq!(
        app.data.radarr_data.movies.filter,
        Some(HorizontallyScrollableText::default())
      );
      assert!(app.data.radarr_data.movies.filtered_items.is_none());
      assert!(app.data.radarr_data.movies.filtered_state.is_none());
    }

    #[test]
    fn test_movie_add_key() {
      let mut app = App::default();

      LibraryHandler::with(
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
      assert!(app.data.radarr_data.add_movie_search.is_some());
    }

    #[test]
    fn test_movie_edit_key() {
      test_edit_movie_key!(
        LibraryHandler,
        ActiveRadarrBlock::Movies,
        ActiveRadarrBlock::Movies
      );
    }

    #[test]
    fn test_update_all_movies_key() {
      let mut app = App::default();

      LibraryHandler::with(
        &DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        &ActiveRadarrBlock::Movies,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::UpdateAllMoviesPrompt.into()
      );
    }

    #[test]
    fn test_refresh_movies_key() {
      assert_refresh_key!(LibraryHandler, ActiveRadarrBlock::Movies);
    }

    #[test]
    fn test_search_movies_box_backspace_key() {
      let mut app = App::default();
      app.data.radarr_data.movies.search = Some("Test".into());

      LibraryHandler::with(
        &DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        &ActiveRadarrBlock::SearchMovie,
        &None,
      )
      .handle();

      assert_str_eq!(
        app.data.radarr_data.movies.search.as_ref().unwrap().text,
        "Tes"
      );
    }

    #[test]
    fn test_filter_movies_box_backspace_key() {
      let mut app = App::default();
      app.data.radarr_data.movies = StatefulTable::default();
      app.data.radarr_data.movies.filter = Some("Test".into());

      LibraryHandler::with(
        &DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        &ActiveRadarrBlock::FilterMovies,
        &None,
      )
      .handle();

      assert_str_eq!(
        app.data.radarr_data.movies.filter.as_ref().unwrap().text,
        "Tes"
      );
    }

    #[test]
    fn test_search_movies_box_char_key() {
      let mut app = App::default();
      app.data.radarr_data.movies.search = Some(HorizontallyScrollableText::default());

      LibraryHandler::with(
        &Key::Char('h'),
        &mut app,
        &ActiveRadarrBlock::SearchMovie,
        &None,
      )
      .handle();

      assert_str_eq!(
        app.data.radarr_data.movies.search.as_ref().unwrap().text,
        "h"
      );
    }

    #[test]
    fn test_filter_movies_box_char_key() {
      let mut app = App::default();
      app.data.radarr_data.movies = StatefulTable::default();
      app.data.radarr_data.movies.filter = Some(HorizontallyScrollableText::default());

      LibraryHandler::with(
        &Key::Char('h'),
        &mut app,
        &ActiveRadarrBlock::FilterMovies,
        &None,
      )
      .handle();

      assert_str_eq!(
        app.data.radarr_data.movies.filter.as_ref().unwrap().text,
        "h"
      );
    }

    #[test]
    fn test_sort_key() {
      let mut app = App::default();

      LibraryHandler::with(
        &DEFAULT_KEYBINDINGS.sort.key,
        &mut app,
        &ActiveRadarrBlock::Movies,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::MoviesSortPrompt.into()
      );
      assert_eq!(
        app.data.radarr_data.movies.sort.as_ref().unwrap().items,
        movies_sorting_options()
      );
      assert!(!app.data.radarr_data.movies.sort_asc);
    }
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
      ActiveRadarrBlock::AddMovieSelectRootFolder,
      ActiveRadarrBlock::AddMovieAlreadyInLibrary,
      ActiveRadarrBlock::AddMovieTagsInput
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(
      LibraryHandler,
      ActiveRadarrBlock::Movies,
      active_radarr_block
    );
  }

  #[rstest]
  fn test_delegates_movie_details_blocks_to_movie_details_handler(
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
    test_handler_delegation!(
      LibraryHandler,
      ActiveRadarrBlock::Movies,
      active_radarr_block
    );
  }

  #[rstest]
  fn test_delegates_edit_movie_blocks_to_edit_movie_handler(
    #[values(
      ActiveRadarrBlock::EditMoviePrompt,
      ActiveRadarrBlock::EditMoviePathInput,
      ActiveRadarrBlock::EditMovieSelectMinimumAvailability,
      ActiveRadarrBlock::EditMovieSelectQualityProfile,
      ActiveRadarrBlock::EditMovieTagsInput
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(
      LibraryHandler,
      ActiveRadarrBlock::Movies,
      active_radarr_block
    );
  }

  #[test]
  fn test_delegates_delete_movie_blocks_to_delete_movie_handler() {
    test_handler_delegation!(
      LibraryHandler,
      ActiveRadarrBlock::Movies,
      ActiveRadarrBlock::DeleteMoviePrompt
    );
  }

  #[test]
  fn test_movies_sorting_options_title() {
    let expected_cmp_fn: fn(&Movie, &Movie) -> Ordering = |a, b| {
      a.title
        .text
        .to_lowercase()
        .cmp(&b.title.text.to_lowercase())
    };
    let mut expected_movies_vec = movies_vec();
    expected_movies_vec.sort_by(expected_cmp_fn);

    let sort_option = movies_sorting_options()[0].clone();
    let mut sorted_movies_vec = movies_vec();
    sorted_movies_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_movies_vec, expected_movies_vec);
    assert_str_eq!(sort_option.name, "Title");
  }

  #[test]
  fn test_movies_sorting_options_year() {
    let expected_cmp_fn: fn(&Movie, &Movie) -> Ordering = |a, b| a.year.cmp(&b.year);
    let mut expected_movies_vec = movies_vec();
    expected_movies_vec.sort_by(expected_cmp_fn);

    let sort_option = movies_sorting_options()[1].clone();
    let mut sorted_movies_vec = movies_vec();
    sorted_movies_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_movies_vec, expected_movies_vec);
    assert_str_eq!(sort_option.name, "Year");
  }

  #[test]
  fn test_movies_sorting_options_studio() {
    let expected_cmp_fn: fn(&Movie, &Movie) -> Ordering =
      |a, b| a.studio.to_lowercase().cmp(&b.studio.to_lowercase());
    let mut expected_movies_vec = movies_vec();
    expected_movies_vec.sort_by(expected_cmp_fn);

    let sort_option = movies_sorting_options()[2].clone();
    let mut sorted_movies_vec = movies_vec();
    sorted_movies_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_movies_vec, expected_movies_vec);
    assert_str_eq!(sort_option.name, "Studio");
  }

  #[test]
  fn test_movies_sorting_options_runtime() {
    let expected_cmp_fn: fn(&Movie, &Movie) -> Ordering = |a, b| a.runtime.cmp(&b.runtime);
    let mut expected_movies_vec = movies_vec();
    expected_movies_vec.sort_by(expected_cmp_fn);

    let sort_option = movies_sorting_options()[3].clone();
    let mut sorted_movies_vec = movies_vec();
    sorted_movies_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_movies_vec, expected_movies_vec);
    assert_str_eq!(sort_option.name, "Runtime");
  }

  #[test]
  fn test_movies_sorting_options_rating() {
    let expected_cmp_fn: fn(&Movie, &Movie) -> Ordering = |a, b| {
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
    };
    let mut expected_movies_vec = movies_vec();
    expected_movies_vec.sort_by(expected_cmp_fn);

    let sort_option = movies_sorting_options()[4].clone();
    let mut sorted_movies_vec = movies_vec();
    sorted_movies_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_movies_vec, expected_movies_vec);
    assert_str_eq!(sort_option.name, "Rating");
  }

  #[test]
  fn test_movies_sorting_options_language() {
    let expected_cmp_fn: fn(&Movie, &Movie) -> Ordering = |a, b| {
      a.original_language
        .name
        .to_lowercase()
        .cmp(&b.original_language.name.to_lowercase())
    };
    let mut expected_movies_vec = movies_vec();
    expected_movies_vec.sort_by(expected_cmp_fn);

    let sort_option = movies_sorting_options()[5].clone();
    let mut sorted_movies_vec = movies_vec();
    sorted_movies_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_movies_vec, expected_movies_vec);
    assert_str_eq!(sort_option.name, "Language");
  }

  #[test]
  fn test_movies_sorting_options_size() {
    let expected_cmp_fn: fn(&Movie, &Movie) -> Ordering =
      |a, b| a.size_on_disk.cmp(&b.size_on_disk);
    let mut expected_movies_vec = movies_vec();
    expected_movies_vec.sort_by(expected_cmp_fn);

    let sort_option = movies_sorting_options()[6].clone();
    let mut sorted_movies_vec = movies_vec();
    sorted_movies_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_movies_vec, expected_movies_vec);
    assert_str_eq!(sort_option.name, "Size");
  }

  #[test]
  fn test_movies_sorting_options_quality() {
    let expected_cmp_fn: fn(&Movie, &Movie) -> Ordering =
      |a, b| a.quality_profile_id.cmp(&b.quality_profile_id);
    let mut expected_movies_vec = movies_vec();
    expected_movies_vec.sort_by(expected_cmp_fn);

    let sort_option = movies_sorting_options()[7].clone();
    let mut sorted_movies_vec = movies_vec();
    sorted_movies_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_movies_vec, expected_movies_vec);
    assert_str_eq!(sort_option.name, "Quality");
  }

  #[test]
  fn test_movies_sorting_options_monitored() {
    let expected_cmp_fn: fn(&Movie, &Movie) -> Ordering = |a, b| a.monitored.cmp(&b.monitored);
    let mut expected_movies_vec = movies_vec();
    expected_movies_vec.sort_by(expected_cmp_fn);

    let sort_option = movies_sorting_options()[8].clone();
    let mut sorted_movies_vec = movies_vec();
    sorted_movies_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_movies_vec, expected_movies_vec);
    assert_str_eq!(sort_option.name, "Monitored");
  }

  #[test]
  fn test_movies_sorting_options_tags() {
    let expected_cmp_fn: fn(&Movie, &Movie) -> Ordering = |a, b| {
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
    };
    let mut expected_movies_vec = movies_vec();
    expected_movies_vec.sort_by(expected_cmp_fn);

    let sort_option = movies_sorting_options()[9].clone();
    let mut sorted_movies_vec = movies_vec();
    sorted_movies_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_movies_vec, expected_movies_vec);
    assert_str_eq!(sort_option.name, "Tags");
  }

  fn movies_vec() -> Vec<Movie> {
    vec![
      Movie {
        title: "test 1".into(),
        original_language: Language {
          name: "English".to_owned(),
        },
        size_on_disk: 1024,
        studio: "Studio 1".to_owned(),
        year: 2024,
        monitored: false,
        runtime: 12.into(),
        quality_profile_id: 1,
        certification: Some("PG-13".to_owned()),
        tags: vec![1.into(), 2.into()],
        ..Movie::default()
      },
      Movie {
        title: "test 2".into(),
        original_language: Language {
          name: "Chinese".to_owned(),
        },
        size_on_disk: 2048,
        studio: "Studio 2".to_owned(),
        year: 1998,
        monitored: false,
        runtime: 60.into(),
        quality_profile_id: 2,
        certification: Some("R".to_owned()),
        tags: vec![1.into(), 3.into()],
        ..Movie::default()
      },
      Movie {
        title: "test 3".into(),
        original_language: Language {
          name: "Japanese".to_owned(),
        },
        size_on_disk: 512,
        studio: "studio 3".to_owned(),
        year: 1954,
        monitored: true,
        runtime: 120.into(),
        quality_profile_id: 3,
        certification: Some("G".to_owned()),
        tags: vec![2.into(), 3.into()],
        ..Movie::default()
      },
    ]
  }

  fn sort_options() -> Vec<SortOption<Movie>> {
    vec![SortOption {
      name: "Test 1",
      cmp_fn: Some(|a, b| {
        a.title
          .text
          .to_lowercase()
          .cmp(&b.title.text.to_lowercase())
      }),
    }]
  }

  #[test]
  fn test_library_handler_accepts() {
    let mut library_handler_blocks = Vec::new();
    library_handler_blocks.extend(LIBRARY_BLOCKS);
    library_handler_blocks.extend(ADD_MOVIE_BLOCKS);
    library_handler_blocks.extend(DELETE_MOVIE_BLOCKS);
    library_handler_blocks.extend(EDIT_MOVIE_BLOCKS);
    library_handler_blocks.extend(MOVIE_DETAILS_BLOCKS);

    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if library_handler_blocks.contains(&active_radarr_block) {
        assert!(LibraryHandler::accepts(&active_radarr_block));
      } else {
        assert!(!LibraryHandler::accepts(&active_radarr_block));
      }
    });
  }
}
