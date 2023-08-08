#[cfg(test)]
mod tests {
  use pretty_assertions::assert_str_eq;
  use strum::IntoEnumIterator;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::radarr::{ActiveRadarrBlock, ADD_MOVIE_BLOCKS};
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::radarr_handlers::library::add_movie_handler::AddMovieHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::radarr_models::{
    AddMovieSearchResult, MinimumAvailability, Monitor, RootFolder,
  };
  use crate::models::HorizontallyScrollableText;

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use strum::IntoEnumIterator;

    use crate::app::radarr::ADD_MOVIE_SELECTION_BLOCKS;
    use crate::models::BlockSelectionState;
    use crate::{simple_stateful_iterable_vec, test_enum_scroll, test_iterable_scroll};

    use super::*;

    test_iterable_scroll!(
      test_add_movie_search_results_scroll,
      AddMovieHandler,
      add_searched_movies,
      simple_stateful_iterable_vec!(AddMovieSearchResult, HorizontallyScrollableText),
      ActiveRadarrBlock::AddMovieSearchResults,
      None,
      title,
      to_string
    );

    test_enum_scroll!(
      test_add_movie_select_monitor_scroll,
      AddMovieHandler,
      Monitor,
      monitor_list,
      ActiveRadarrBlock::AddMovieSelectMonitor,
      None
    );

    test_enum_scroll!(
      test_add_movie_select_minimum_availability_scroll,
      AddMovieHandler,
      MinimumAvailability,
      minimum_availability_list,
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
      None
    );

    test_iterable_scroll!(
      test_add_movie_select_quality_profile_scroll,
      AddMovieHandler,
      quality_profile_list,
      ActiveRadarrBlock::AddMovieSelectQualityProfile,
      None
    );

    test_iterable_scroll!(
      test_add_movie_select_root_folder_scroll,
      AddMovieHandler,
      root_folder_list,
      simple_stateful_iterable_vec!(RootFolder, String, path),
      ActiveRadarrBlock::AddMovieSelectRootFolder,
      None,
      path
    );

    #[rstest]
    fn test_add_movie_prompt_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::default();
      app.data.radarr_data.selected_block = BlockSelectionState::new(&ADD_MOVIE_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.next();

      AddMovieHandler::with(&key, &mut app, &ActiveRadarrBlock::AddMoviePrompt, &None).handle();

      if key == Key::Up {
        assert_eq!(
          app.data.radarr_data.selected_block.get_active_block(),
          &ActiveRadarrBlock::AddMovieSelectRootFolder
        );
      } else {
        assert_eq!(
          app.data.radarr_data.selected_block.get_active_block(),
          &ActiveRadarrBlock::AddMovieSelectMinimumAvailability
        );
      }
    }
  }

  mod test_handle_home_end {
    use strum::IntoEnumIterator;

    use crate::{
      extended_stateful_iterable_vec, test_enum_home_and_end, test_iterable_home_and_end,
      test_text_box_home_end_keys,
    };

    use super::*;

    test_iterable_home_and_end!(
      test_add_movie_search_results_home_end,
      AddMovieHandler,
      add_searched_movies,
      extended_stateful_iterable_vec!(AddMovieSearchResult, HorizontallyScrollableText),
      ActiveRadarrBlock::AddMovieSearchResults,
      None,
      title,
      to_string
    );

    test_enum_home_and_end!(
      test_add_movie_select_monitor_home_end,
      AddMovieHandler,
      Monitor,
      monitor_list,
      ActiveRadarrBlock::AddMovieSelectMonitor,
      None
    );

    test_enum_home_and_end!(
      test_add_movie_select_minimum_availability_home_end,
      AddMovieHandler,
      MinimumAvailability,
      minimum_availability_list,
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
      None
    );

    test_iterable_home_and_end!(
      test_add_movie_select_quality_profile_home_end,
      AddMovieHandler,
      quality_profile_list,
      ActiveRadarrBlock::AddMovieSelectQualityProfile,
      None
    );

    test_iterable_home_and_end!(
      test_add_movie_select_root_folder_home_end,
      AddMovieHandler,
      root_folder_list,
      extended_stateful_iterable_vec!(RootFolder, String, path),
      ActiveRadarrBlock::AddMovieSelectRootFolder,
      None,
      path
    );

    #[test]
    fn test_add_movie_search_input_home_end_keys() {
      test_text_box_home_end_keys!(
        AddMovieHandler,
        ActiveRadarrBlock::AddMovieSearchInput,
        search
      );
    }

    #[test]
    fn test_add_movie_tags_input_home_end_keys() {
      test_text_box_home_end_keys!(
        AddMovieHandler,
        ActiveRadarrBlock::AddMovieTagsInput,
        edit_tags
      );
    }
  }

  mod test_handle_left_right_action {
    use rstest::rstest;

    use crate::test_text_box_left_right_keys;

    use super::*;

    #[rstest]
    fn test_left_right_prompt_toggle(#[values(Key::Left, Key::Right)] key: Key) {
      let mut app = App::default();

      AddMovieHandler::with(&key, &mut app, &ActiveRadarrBlock::AddMoviePrompt, &None).handle();

      assert!(app.data.radarr_data.prompt_confirm);

      AddMovieHandler::with(&key, &mut app, &ActiveRadarrBlock::AddMoviePrompt, &None).handle();

      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[test]
    fn test_add_movie_search_input_left_right_keys() {
      test_text_box_left_right_keys!(
        AddMovieHandler,
        ActiveRadarrBlock::AddMovieSearchInput,
        search
      );
    }

    #[test]
    fn test_add_movie_tags_input_left_right_keys() {
      test_text_box_left_right_keys!(
        AddMovieHandler,
        ActiveRadarrBlock::AddMovieTagsInput,
        edit_tags
      );
    }
  }

  mod test_handle_submit {
    use bimap::BiMap;
    use pretty_assertions::{assert_eq, assert_str_eq};
    use rstest::rstest;

    use crate::app::radarr::ADD_MOVIE_SELECTION_BLOCKS;
    use crate::models::radarr_models::Movie;
    use crate::models::BlockSelectionState;
    use crate::network::radarr_network::RadarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_add_movie_search_input_submit() {
      let mut app = App::default();
      app.should_ignore_quit_key = true;

      AddMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchInput,
        &None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMovieSearchResults.into()
      );
    }

    #[test]
    fn test_add_movie_search_results_submit() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .add_searched_movies
        .set_items(vec![AddMovieSearchResult::default()]);
      app.data.radarr_data.quality_profile_map =
        BiMap::from_iter([(1, "B - Test 2".to_owned()), (0, "A - Test 1".to_owned())]);

      AddMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchResults,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMoviePrompt.into()
      );
      assert_eq!(
        app.data.radarr_data.selected_block.get_active_block(),
        &ActiveRadarrBlock::AddMovieSelectRootFolder
      );
      assert!(!app.data.radarr_data.monitor_list.items.is_empty());
      assert!(!app
        .data
        .radarr_data
        .minimum_availability_list
        .items
        .is_empty());
      assert!(!app.data.radarr_data.quality_profile_list.items.is_empty());
      assert_str_eq!(
        app
          .data
          .radarr_data
          .quality_profile_list
          .current_selection(),
        "A - Test 1"
      );
    }

    #[test]
    fn test_add_movie_search_results_submit_does_nothing_on_empty_table() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieSearchResults.into());
      AddMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchResults,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMovieSearchResults.into()
      );
    }

    #[test]
    fn test_add_movie_search_results_submit_movie_already_in_library() {
      let mut app = App::default();
      app
        .data
        .radarr_data
        .add_searched_movies
        .set_items(vec![AddMovieSearchResult::default()]);
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);

      AddMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchResults,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMovieAlreadyInLibrary.into()
      );
    }

    #[test]
    fn test_add_movie_prompt_prompt_decline_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());
      app.data.radarr_data.selected_block = BlockSelectionState::new(&ADD_MOVIE_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(ADD_MOVIE_SELECTION_BLOCKS.len() - 1);

      AddMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMoviePrompt,
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
    }

    #[test]
    fn test_add_movie_confirm_prompt_prompt_confirmation_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());
      app.data.radarr_data.prompt_confirm = true;
      app.data.radarr_data.selected_block = BlockSelectionState::new(&ADD_MOVIE_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(ADD_MOVIE_SELECTION_BLOCKS.len() - 1);

      AddMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMoviePrompt,
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::AddMovie)
      );
    }

    #[rstest]
    #[case(ActiveRadarrBlock::AddMovieSelectRootFolder, 0)]
    #[case(ActiveRadarrBlock::AddMovieSelectMonitor, 1)]
    #[case(ActiveRadarrBlock::AddMovieSelectMinimumAvailability, 2)]
    #[case(ActiveRadarrBlock::AddMovieSelectQualityProfile, 3)]
    #[case(ActiveRadarrBlock::AddMovieTagsInput, 4)]
    fn test_add_movie_prompt_selected_block_submit(
      #[case] selected_block: ActiveRadarrBlock,
      #[case] index: usize,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(
        (
          ActiveRadarrBlock::AddMoviePrompt,
          Some(ActiveRadarrBlock::CollectionDetails),
        )
          .into(),
      );
      app.data.radarr_data.selected_block = BlockSelectionState::new(&ADD_MOVIE_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.set_index(index);

      AddMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMoviePrompt,
        &Some(ActiveRadarrBlock::CollectionDetails),
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &(selected_block, Some(ActiveRadarrBlock::CollectionDetails)).into()
      );
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);

      if selected_block == ActiveRadarrBlock::AddMovieTagsInput {
        assert!(app.should_ignore_quit_key);
      }
    }

    #[rstest]
    fn test_add_movie_prompt_selecting_preferences_blocks_submit(
      #[values(
        ActiveRadarrBlock::AddMovieSelectMonitor,
        ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
        ActiveRadarrBlock::AddMovieSelectQualityProfile,
        ActiveRadarrBlock::AddMovieSelectRootFolder,
        ActiveRadarrBlock::AddMovieTagsInput
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());
      app.push_navigation_stack(active_radarr_block.into());

      AddMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &active_radarr_block,
        &Some(ActiveRadarrBlock::CollectionDetails),
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMoviePrompt.into()
      );

      if active_radarr_block == ActiveRadarrBlock::AddMovieTagsInput {
        assert!(!app.should_ignore_quit_key);
      }
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::app::radarr::radarr_test_utils::utils::create_test_radarr_data;
    use crate::{
      assert_edit_media_reset, assert_preferences_selections_reset, assert_search_reset,
      simple_stateful_iterable_vec,
    };

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[test]
    fn test_add_movie_search_input_esc() {
      let mut app = App::default();
      app.data.radarr_data = create_test_radarr_data();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieSearchInput.into());

      AddMovieHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchInput,
        &None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
      assert_search_reset!(app.data.radarr_data);
    }

    #[test]
    fn test_add_movie_input_esc() {
      let mut app = App::default();
      app.data.radarr_data = create_test_radarr_data();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieTagsInput.into());

      AddMovieHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMovieTagsInput,
        &None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMoviePrompt.into()
      );
    }

    #[rstest]
    fn test_add_movie_search_results_esc(
      #[values(
        ActiveRadarrBlock::AddMovieSearchResults,
        ActiveRadarrBlock::AddMovieEmptySearchResults
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieSearchInput.into());
      app.push_navigation_stack(active_radarr_block.into());
      app
        .data
        .radarr_data
        .add_searched_movies
        .set_items(simple_stateful_iterable_vec!(
          AddMovieSearchResult,
          HorizontallyScrollableText
        ));

      AddMovieHandler::with(&ESC_KEY, &mut app, &active_radarr_block, &None).handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMovieSearchInput.into()
      );
      assert!(app.data.radarr_data.add_searched_movies.items.is_empty());
      assert!(app.should_ignore_quit_key);
    }

    #[test]
    fn test_add_movie_already_in_library_esc() {
      let mut app = App::default();
      app.data.radarr_data = create_test_radarr_data();
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieSearchResults.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieAlreadyInLibrary.into());

      AddMovieHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMovieAlreadyInLibrary,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMovieSearchResults.into()
      );
    }

    #[test]
    fn test_add_movie_prompt_esc() {
      let mut app = App::default();
      app.data.radarr_data = create_test_radarr_data();
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieSearchResults.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());

      AddMovieHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMoviePrompt,
        &None,
      )
      .handle();

      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMovieSearchResults.into()
      );
      assert_preferences_selections_reset!(app.data.radarr_data);
      assert_edit_media_reset!(app.data.radarr_data);
    }

    #[test]
    fn test_add_movie_tags_input_esc() {
      let mut app = App::default();
      app.data.radarr_data = create_test_radarr_data();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieTagsInput.into());

      AddMovieHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMovieTagsInput,
        &None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMoviePrompt.into()
      );
    }

    #[rstest]
    fn test_selecting_preferences_blocks_esc(
      #[values(
        ActiveRadarrBlock::AddMovieSelectMonitor,
        ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
        ActiveRadarrBlock::AddMovieSelectQualityProfile,
        ActiveRadarrBlock::AddMovieSelectRootFolder
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(
        (
          ActiveRadarrBlock::AddMoviePrompt,
          Some(ActiveRadarrBlock::CollectionDetails),
        )
          .into(),
      );
      app.push_navigation_stack(
        (
          active_radarr_block,
          Some(ActiveRadarrBlock::CollectionDetails),
        )
          .into(),
      );

      AddMovieHandler::with(
        &ESC_KEY,
        &mut app,
        &active_radarr_block,
        &Some(ActiveRadarrBlock::CollectionDetails),
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &(
          ActiveRadarrBlock::AddMoviePrompt,
          Some(ActiveRadarrBlock::CollectionDetails),
        )
          .into()
      );
    }
  }

  mod test_handle_key_char {
    use super::*;

    #[test]
    fn test_add_movie_search_input_backspace() {
      let mut app = App::default();
      app.data.radarr_data.search = "Test".to_owned().into();

      AddMovieHandler::with(
        &DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchInput,
        &None,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.search.text, "Tes");
    }

    #[test]
    fn test_add_movie_tags_input_backspace() {
      let mut app = App::default();
      app.data.radarr_data.edit_tags = "Test".to_owned().into();

      AddMovieHandler::with(
        &DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        &ActiveRadarrBlock::AddMovieTagsInput,
        &None,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.edit_tags.text, "Tes");
    }

    #[test]
    fn test_add_movie_search_input_char_key() {
      let mut app = App::default();

      AddMovieHandler::with(
        &Key::Char('h'),
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchInput,
        &None,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.search.text, "h");
    }

    #[test]
    fn test_add_movie_tags_input_char_key() {
      let mut app = App::default();

      AddMovieHandler::with(
        &Key::Char('h'),
        &mut app,
        &ActiveRadarrBlock::AddMovieTagsInput,
        &None,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.edit_tags.text, "h");
    }
  }

  #[test]
  fn test_add_movie_handler_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if ADD_MOVIE_BLOCKS.contains(&active_radarr_block) {
        assert!(AddMovieHandler::accepts(&active_radarr_block));
      } else {
        assert!(!AddMovieHandler::accepts(&active_radarr_block));
      }
    });
  }
}
