#[cfg(test)]
mod tests {
  use pretty_assertions::assert_str_eq;
  use strum::IntoEnumIterator;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::radarr_handlers::library::edit_movie_handler::EditMovieHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::radarr_models::MinimumAvailability;
  use crate::models::servarr_data::radarr_data::{ActiveRadarrBlock, EDIT_MOVIE_BLOCKS};

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use strum::IntoEnumIterator;

    use crate::models::servarr_data::radarr_data::EDIT_MOVIE_SELECTION_BLOCKS;
    use crate::models::BlockSelectionState;
    use crate::{test_enum_scroll, test_iterable_scroll};

    use super::*;

    test_enum_scroll!(
      test_edit_movie_select_minimum_availability_scroll,
      EditMovieHandler,
      MinimumAvailability,
      minimum_availability_list,
      ActiveRadarrBlock::EditMovieSelectMinimumAvailability,
      None
    );

    test_iterable_scroll!(
      test_edit_movie_select_quality_profile_scroll,
      EditMovieHandler,
      quality_profile_list,
      ActiveRadarrBlock::EditMovieSelectQualityProfile,
      None
    );

    #[rstest]
    fn test_edit_movie_prompt_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::default();
      app.data.radarr_data.selected_block = BlockSelectionState::new(&EDIT_MOVIE_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.next();

      EditMovieHandler::with(&key, &mut app, &ActiveRadarrBlock::EditMoviePrompt, &None).handle();

      if key == Key::Up {
        assert_eq!(
          app.data.radarr_data.selected_block.get_active_block(),
          &ActiveRadarrBlock::EditMovieToggleMonitored
        );
      } else {
        assert_eq!(
          app.data.radarr_data.selected_block.get_active_block(),
          &ActiveRadarrBlock::EditMovieSelectQualityProfile
        );
      }
    }
  }

  mod test_handle_home_end {
    use strum::IntoEnumIterator;

    use crate::{test_enum_home_and_end, test_iterable_home_and_end, test_text_box_home_end_keys};

    use super::*;

    test_enum_home_and_end!(
      test_edit_movie_select_minimum_availability_home_end,
      EditMovieHandler,
      MinimumAvailability,
      minimum_availability_list,
      ActiveRadarrBlock::EditMovieSelectMinimumAvailability,
      None
    );

    test_iterable_home_and_end!(
      test_edit_movie_select_quality_profile_scroll,
      EditMovieHandler,
      quality_profile_list,
      ActiveRadarrBlock::EditMovieSelectQualityProfile,
      None
    );

    #[test]
    fn test_edit_movie_path_input_home_end_keys() {
      test_text_box_home_end_keys!(
        EditMovieHandler,
        ActiveRadarrBlock::EditMoviePathInput,
        edit_path
      );
    }

    #[test]
    fn test_edit_movie_tags_input_home_end_keys() {
      test_text_box_home_end_keys!(
        EditMovieHandler,
        ActiveRadarrBlock::EditMovieTagsInput,
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

      EditMovieHandler::with(&key, &mut app, &ActiveRadarrBlock::EditMoviePrompt, &None).handle();

      assert!(app.data.radarr_data.prompt_confirm);

      EditMovieHandler::with(&key, &mut app, &ActiveRadarrBlock::EditMoviePrompt, &None).handle();

      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[test]
    fn test_edit_movie_path_input_left_right_keys() {
      test_text_box_left_right_keys!(
        EditMovieHandler,
        ActiveRadarrBlock::EditMoviePathInput,
        edit_path
      );
    }

    #[test]
    fn test_edit_movie_tags_input_left_right_keys() {
      test_text_box_left_right_keys!(
        EditMovieHandler,
        ActiveRadarrBlock::EditMovieTagsInput,
        edit_tags
      );
    }
  }

  mod test_handle_submit {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::models::servarr_data::radarr_data::{
      EDIT_COLLECTION_SELECTION_BLOCKS, EDIT_MOVIE_SELECTION_BLOCKS,
    };
    use crate::models::{BlockSelectionState, Route};
    use crate::network::radarr_network::RadarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_edit_movie_path_input_submit() {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.data.radarr_data.edit_path = "Test Path".to_owned().into();
      app.push_navigation_stack(ActiveRadarrBlock::EditMoviePrompt.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditMoviePathInput.into());

      EditMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::EditMoviePathInput,
        &None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert!(!app.data.radarr_data.edit_path.text.is_empty());
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::EditMoviePrompt.into()
      );
    }

    #[test]
    fn test_edit_movie_tags_input_submit() {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.data.radarr_data.edit_tags = "Test Tags".to_owned().into();
      app.push_navigation_stack(ActiveRadarrBlock::EditMoviePrompt.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditMoviePathInput.into());

      EditMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::EditMovieTagsInput,
        &None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert!(!app.data.radarr_data.edit_tags.text.is_empty());
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::EditMoviePrompt.into()
      );
    }

    #[test]
    fn test_edit_movie_prompt_prompt_decline_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditMoviePrompt.into());
      app.data.radarr_data.selected_block = BlockSelectionState::new(&EDIT_MOVIE_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(EDIT_COLLECTION_SELECTION_BLOCKS.len() - 1);

      EditMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::EditMoviePrompt,
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
    }

    #[test]
    fn test_edit_movie_confirm_prompt_prompt_confirmation_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditMoviePrompt.into());
      app.data.radarr_data.prompt_confirm = true;
      app.data.radarr_data.selected_block = BlockSelectionState::new(&EDIT_MOVIE_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(EDIT_COLLECTION_SELECTION_BLOCKS.len() - 1);

      EditMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::EditMoviePrompt,
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::EditMovie)
      );
      assert!(app.should_refresh);
    }

    #[test]
    fn test_edit_movie_toggle_monitored_submit() {
      let current_route = Route::from((
        ActiveRadarrBlock::EditMoviePrompt,
        Some(ActiveRadarrBlock::Movies),
      ));
      let mut app = App::default();
      app.data.radarr_data.selected_block = BlockSelectionState::new(&EDIT_MOVIE_SELECTION_BLOCKS);
      app.push_navigation_stack(current_route);

      EditMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::EditMoviePrompt,
        &Some(ActiveRadarrBlock::Movies),
      )
      .handle();

      assert_eq!(app.get_current_route(), &current_route);
      assert_eq!(app.data.radarr_data.edit_monitored, Some(true));

      EditMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::EditMoviePrompt,
        &Some(ActiveRadarrBlock::Movies),
      )
      .handle();

      assert_eq!(app.get_current_route(), &current_route);
      assert_eq!(app.data.radarr_data.edit_monitored, Some(false));
    }

    #[rstest]
    #[case(ActiveRadarrBlock::EditMovieSelectMinimumAvailability, 1)]
    #[case(ActiveRadarrBlock::EditMovieSelectQualityProfile, 2)]
    #[case(ActiveRadarrBlock::EditMoviePathInput, 3)]
    #[case(ActiveRadarrBlock::EditMovieTagsInput, 4)]
    fn test_edit_movie_prompt_selected_block_submit(
      #[case] selected_block: ActiveRadarrBlock,
      #[case] index: usize,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(
        (
          ActiveRadarrBlock::EditMoviePrompt,
          Some(ActiveRadarrBlock::Movies),
        )
          .into(),
      );
      app.data.radarr_data.selected_block = BlockSelectionState::new(&EDIT_MOVIE_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.set_index(index);

      EditMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::EditMoviePrompt,
        &Some(ActiveRadarrBlock::Movies),
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &(selected_block, Some(ActiveRadarrBlock::Movies)).into()
      );
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);

      if selected_block == ActiveRadarrBlock::EditMoviePathInput
        || selected_block == ActiveRadarrBlock::EditMovieTagsInput
      {
        assert!(app.should_ignore_quit_key);
      }
    }

    #[rstest]
    fn test_edit_movie_prompt_selecting_preferences_blocks_submit(
      #[values(
        ActiveRadarrBlock::EditMovieSelectMinimumAvailability,
        ActiveRadarrBlock::EditMovieSelectQualityProfile,
        ActiveRadarrBlock::EditMoviePathInput,
        ActiveRadarrBlock::EditMovieTagsInput
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::EditMoviePrompt.into());
      app.push_navigation_stack(active_radarr_block.into());

      EditMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &active_radarr_block,
        &Some(ActiveRadarrBlock::Movies),
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::EditMoviePrompt.into()
      );

      if active_radarr_block == ActiveRadarrBlock::EditMoviePathInput
        || active_radarr_block == ActiveRadarrBlock::EditMovieTagsInput
      {
        assert!(!app.should_ignore_quit_key);
      }
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::models::servarr_data::radarr_data::radarr_test_utils::utils::create_test_radarr_data;
    use crate::{assert_edit_media_reset, assert_preferences_selections_reset};

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_edit_movie_input_esc(
      #[values(
        ActiveRadarrBlock::EditMovieTagsInput,
        ActiveRadarrBlock::EditMoviePathInput
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.data.radarr_data = create_test_radarr_data();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveRadarrBlock::EditMoviePrompt.into());
      app.push_navigation_stack(active_radarr_block.into());

      EditMovieHandler::with(&ESC_KEY, &mut app, &active_radarr_block, &None).handle();

      assert!(!app.should_ignore_quit_key);
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::EditMoviePrompt.into()
      );
    }

    #[test]
    fn test_edit_movie_prompt_esc() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::EditMoviePrompt.into());
      app.data.radarr_data = create_test_radarr_data();

      EditMovieHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::EditMoviePrompt,
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
      let radarr_data = &app.data.radarr_data;

      assert_preferences_selections_reset!(radarr_data);
      assert_edit_media_reset!(radarr_data);
      assert!(!radarr_data.prompt_confirm);
    }

    #[rstest]
    fn test_edit_movie_esc(
      #[values(
        ActiveRadarrBlock::EditMovieSelectMinimumAvailability,
        ActiveRadarrBlock::EditMovieSelectQualityProfile
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.data.radarr_data = create_test_radarr_data();
      app.push_navigation_stack(active_radarr_block.into());

      EditMovieHandler::with(&ESC_KEY, &mut app, &active_radarr_block, &None).handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
    }
  }

  mod test_handle_key_char {
    use super::*;

    #[test]
    fn test_edit_movie_path_input_backspace() {
      let mut app = App::default();
      app.data.radarr_data.edit_path = "Test".to_owned().into();

      EditMovieHandler::with(
        &DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        &ActiveRadarrBlock::EditMoviePathInput,
        &None,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.edit_path.text, "Tes");
    }

    #[test]
    fn test_edit_movie_tags_input_backspace() {
      let mut app = App::default();
      app.data.radarr_data.edit_tags = "Test".to_owned().into();

      EditMovieHandler::with(
        &DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        &ActiveRadarrBlock::EditMovieTagsInput,
        &None,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.edit_tags.text, "Tes");
    }

    #[test]
    fn test_edit_movie_path_input_char_key() {
      let mut app = App::default();

      EditMovieHandler::with(
        &Key::Char('h'),
        &mut app,
        &ActiveRadarrBlock::EditMoviePathInput,
        &None,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.edit_path.text, "h");
    }

    #[test]
    fn test_edit_movie_tags_input_char_key() {
      let mut app = App::default();

      EditMovieHandler::with(
        &Key::Char('h'),
        &mut app,
        &ActiveRadarrBlock::EditMovieTagsInput,
        &None,
      )
      .handle();

      assert_str_eq!(app.data.radarr_data.edit_tags.text, "h");
    }
  }

  #[test]
  fn test_edit_movie_handler_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if EDIT_MOVIE_BLOCKS.contains(&active_radarr_block) {
        assert!(EditMovieHandler::accepts(&active_radarr_block));
      } else {
        assert!(!EditMovieHandler::accepts(&active_radarr_block));
      }
    });
  }
}
