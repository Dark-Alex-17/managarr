#[cfg(test)]
mod tests {
  use pretty_assertions::assert_str_eq;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::radarr::{
    ActiveRadarrBlock, ADD_MOVIE_BLOCKS, DELETE_MOVIE_BLOCKS, EDIT_MOVIE_BLOCKS, LIBRARY_BLOCKS,
    MOVIE_DETAILS_BLOCKS,
  };
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::radarr_handlers::library::LibraryHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::radarr_models::Movie;
  use crate::models::HorizontallyScrollableText;
  use crate::test_handler_delegation;

  mod test_handle_scroll_up_and_down {
    use crate::{simple_stateful_iterable_vec, test_iterable_scroll};

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

    test_iterable_scroll!(
      test_filtered_movies_scroll,
      LibraryHandler,
      filtered_movies,
      simple_stateful_iterable_vec!(Movie, HorizontallyScrollableText),
      ActiveRadarrBlock::Movies,
      None,
      title,
      to_string
    );
  }

  mod test_handle_home_end {
    use pretty_assertions::assert_eq;

    use crate::{
      extended_stateful_iterable_vec, test_iterable_home_and_end, test_text_box_home_end_keys,
    };

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

    test_iterable_home_and_end!(
      test_filtered_movies_home_end,
      LibraryHandler,
      filtered_movies,
      extended_stateful_iterable_vec!(Movie, HorizontallyScrollableText),
      ActiveRadarrBlock::Movies,
      None,
      title,
      to_string
    );

    #[test]
    fn test_movie_search_box_home_end_keys() {
      test_text_box_home_end_keys!(LibraryHandler, ActiveRadarrBlock::SearchMovie, search);
    }

    #[test]
    fn test_movie_filter_box_home_end_keys() {
      test_text_box_home_end_keys!(LibraryHandler, ActiveRadarrBlock::FilterMovies, filter);
    }
  }

  mod test_handle_delete {
    use pretty_assertions::assert_eq;

    use crate::app::radarr::DELETE_MOVIE_SELECTION_BLOCKS;
    use crate::assert_delete_prompt;

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

    use crate::test_text_box_left_right_keys;

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
      test_text_box_left_right_keys!(LibraryHandler, ActiveRadarrBlock::SearchMovie, search);
    }

    #[test]
    fn test_movie_filter_box_left_right_keys() {
      test_text_box_left_right_keys!(LibraryHandler, ActiveRadarrBlock::FilterMovies, filter);
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
      app
        .data
        .radarr_data
        .movies
        .set_items(extended_stateful_iterable_vec!(
          Movie,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.search = "Test 2".to_owned().into();

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

      LibraryHandler::with(
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

      LibraryHandler::with(
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
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;

    use crate::app::radarr::radarr_test_utils::utils::create_test_radarr_data;
    use crate::{assert_filter_reset, assert_search_reset};

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[test]
    fn test_search_movie_block_esc() {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::SearchMovie.into());
      app.data.radarr_data = create_test_radarr_data();

      LibraryHandler::with(&ESC_KEY, &mut app, &ActiveRadarrBlock::SearchMovie, &None).handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
      assert!(!app.should_ignore_quit_key);
      assert_search_reset!(app.data.radarr_data);
    }

    #[test]
    fn test_filter_movies_block_esc() {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::FilterMovies.into());
      app.data.radarr_data = create_test_radarr_data();

      LibraryHandler::with(&ESC_KEY, &mut app, &ActiveRadarrBlock::FilterMovies, &None).handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
      assert!(!app.should_ignore_quit_key);
      assert_filter_reset!(app.data.radarr_data);
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

      LibraryHandler::with(&ESC_KEY, &mut app, &ActiveRadarrBlock::Movies, &None).handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
      assert!(app.error.text.is_empty());
      assert_search_reset!(app.data.radarr_data);
      assert_filter_reset!(app.data.radarr_data);
    }
  }

  mod test_handle_key_char {
    use bimap::BiMap;
    use pretty_assertions::{assert_eq, assert_str_eq};
    use serde_json::Number;
    use strum::IntoEnumIterator;

    use crate::app::radarr::radarr_test_utils::utils::create_test_radarr_data;
    use crate::app::radarr::RadarrData;
    use crate::app::radarr::EDIT_MOVIE_SELECTION_BLOCKS;
    use crate::models::radarr_models::MinimumAvailability;
    use crate::models::HorizontallyScrollableText;
    use crate::models::StatefulTable;
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
      assert!(app.data.radarr_data.is_searching);
      assert!(app.should_ignore_quit_key);
    }

    #[test]
    fn test_filter_movies_key() {
      let mut app = App::default();

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
      assert!(app.data.radarr_data.is_filtering);
      assert!(app.should_ignore_quit_key);
    }

    #[test]
    fn test_movie_add() {
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
      app.data.radarr_data.search = "Test".to_owned().into();

      LibraryHandler::with(
        &DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        &ActiveRadarrBlock::SearchMovie,
        &None,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.search.text, "Tes");
    }

    #[test]
    fn test_filter_movies_box_backspace_key() {
      let mut app = App::default();
      app.data.radarr_data.filter = "Test".to_owned().into();

      LibraryHandler::with(
        &DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        &ActiveRadarrBlock::FilterMovies,
        &None,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.filter.text, "Tes");
    }

    #[test]
    fn test_search_movies_box_char_key() {
      let mut app = App::default();

      LibraryHandler::with(
        &Key::Char('h'),
        &mut app,
        &ActiveRadarrBlock::SearchMovie,
        &None,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.search.text, "h");
    }

    #[test]
    fn test_filter_movies_box_char_key() {
      let mut app = App::default();

      LibraryHandler::with(
        &Key::Char('h'),
        &mut app,
        &ActiveRadarrBlock::FilterMovies,
        &None,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.filter.text, "h");
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
