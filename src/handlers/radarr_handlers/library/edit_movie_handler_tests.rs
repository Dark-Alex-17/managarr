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
  use crate::models::servarr_data::radarr::modals::EditMovieModal;
  use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, EDIT_MOVIE_BLOCKS};

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use strum::IntoEnumIterator;

    use crate::models::servarr_data::radarr::modals::EditMovieModal;
    use crate::models::servarr_data::radarr::radarr_data::EDIT_MOVIE_SELECTION_BLOCKS;
    use crate::models::BlockSelectionState;

    use super::*;

    #[rstest]
    fn test_edit_movie_select_minimum_availability_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let minimum_availability_vec = Vec::from_iter(MinimumAvailability::iter());
      let mut app = App::default();
      app.data.radarr_data.edit_movie_modal = Some(EditMovieModal::default());
      app
        .data
        .radarr_data
        .edit_movie_modal
        .as_mut()
        .unwrap()
        .minimum_availability_list
        .set_items(minimum_availability_vec.clone());

      if key == Key::Up {
        for i in (0..minimum_availability_vec.len()).rev() {
          EditMovieHandler::with(
            key,
            &mut app,
            ActiveRadarrBlock::EditMovieSelectMinimumAvailability,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .radarr_data
              .edit_movie_modal
              .as_ref()
              .unwrap()
              .minimum_availability_list
              .current_selection(),
            &minimum_availability_vec[i]
          );
        }
      } else {
        for i in 0..minimum_availability_vec.len() {
          EditMovieHandler::with(
            key,
            &mut app,
            ActiveRadarrBlock::EditMovieSelectMinimumAvailability,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .radarr_data
              .edit_movie_modal
              .as_ref()
              .unwrap()
              .minimum_availability_list
              .current_selection(),
            &minimum_availability_vec[(i + 1) % minimum_availability_vec.len()]
          );
        }
      }
    }

    #[rstest]
    fn test_edit_movie_select_quality_profile_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::default();
      app.data.radarr_data.edit_movie_modal = Some(EditMovieModal::default());
      app
        .data
        .radarr_data
        .edit_movie_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .set_items(vec!["Test 1".to_owned(), "Test 2".to_owned()]);

      EditMovieHandler::with(
        key,
        &mut app,
        ActiveRadarrBlock::EditMovieSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 2"
      );

      EditMovieHandler::with(
        key,
        &mut app,
        ActiveRadarrBlock::EditMovieSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[rstest]
    fn test_edit_movie_prompt_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::default();
      app.data.radarr_data.edit_movie_modal = Some(EditMovieModal::default());
      app.data.radarr_data.selected_block = BlockSelectionState::new(EDIT_MOVIE_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.down();

      EditMovieHandler::with(key, &mut app, ActiveRadarrBlock::EditMoviePrompt, None).handle();

      if key == Key::Up {
        assert_eq!(
          app.data.radarr_data.selected_block.get_active_block(),
          ActiveRadarrBlock::EditMovieToggleMonitored
        );
      } else {
        assert_eq!(
          app.data.radarr_data.selected_block.get_active_block(),
          ActiveRadarrBlock::EditMovieSelectQualityProfile
        );
      }
    }

    #[rstest]
    fn test_edit_movie_prompt_scroll_no_op_when_not_ready(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::default();
      app.is_loading = true;
      app.data.radarr_data.edit_movie_modal = Some(EditMovieModal::default());
      app.data.radarr_data.selected_block = BlockSelectionState::new(EDIT_MOVIE_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.down();

      EditMovieHandler::with(key, &mut app, ActiveRadarrBlock::EditMoviePrompt, None).handle();

      assert_eq!(
        app.data.radarr_data.selected_block.get_active_block(),
        ActiveRadarrBlock::EditMovieSelectMinimumAvailability
      );
    }
  }

  mod test_handle_home_end {
    use std::sync::atomic::Ordering;

    use strum::IntoEnumIterator;

    use crate::models::servarr_data::radarr::modals::EditMovieModal;

    use super::*;

    #[test]
    fn test_edit_movie_select_minimum_availability_home_end() {
      let minimum_availability_vec = Vec::from_iter(MinimumAvailability::iter());
      let mut app = App::default();
      app.data.radarr_data.edit_movie_modal = Some(EditMovieModal::default());
      app
        .data
        .radarr_data
        .edit_movie_modal
        .as_mut()
        .unwrap()
        .minimum_availability_list
        .set_items(minimum_availability_vec.clone());

      EditMovieHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveRadarrBlock::EditMovieSelectMinimumAvailability,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .minimum_availability_list
          .current_selection(),
        &minimum_availability_vec[minimum_availability_vec.len() - 1]
      );

      EditMovieHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveRadarrBlock::EditMovieSelectMinimumAvailability,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .minimum_availability_list
          .current_selection(),
        &minimum_availability_vec[0]
      );
    }

    #[test]
    fn test_edit_movie_select_quality_profile_scroll() {
      let mut app = App::default();
      app.data.radarr_data.edit_movie_modal = Some(EditMovieModal::default());
      app
        .data
        .radarr_data
        .edit_movie_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .set_items(vec![
          "Test 1".to_owned(),
          "Test 2".to_owned(),
          "Test 3".to_owned(),
        ]);

      EditMovieHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveRadarrBlock::EditMovieSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 3"
      );

      EditMovieHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveRadarrBlock::EditMovieSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[test]
    fn test_edit_movie_path_input_home_end_keys() {
      let mut app = App::default();
      app.data.radarr_data.edit_movie_modal = Some(EditMovieModal {
        path: "Test".into(),
        ..EditMovieModal::default()
      });

      EditMovieHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveRadarrBlock::EditMoviePathInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .path
          .offset
          .load(Ordering::SeqCst),
        4
      );

      EditMovieHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveRadarrBlock::EditMoviePathInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .path
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }

    #[test]
    fn test_edit_movie_tags_input_home_end_keys() {
      let mut app = App::default();
      app.data.radarr_data.edit_movie_modal = Some(EditMovieModal {
        tags: "Test".into(),
        ..EditMovieModal::default()
      });

      EditMovieHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveRadarrBlock::EditMovieTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .tags
          .offset
          .load(Ordering::SeqCst),
        4
      );

      EditMovieHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveRadarrBlock::EditMovieTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .tags
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }
  }

  mod test_handle_left_right_action {
    use std::sync::atomic::Ordering;

    use crate::models::servarr_data::radarr::modals::EditMovieModal;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_left_right_prompt_toggle(#[values(Key::Left, Key::Right)] key: Key) {
      let mut app = App::default();

      EditMovieHandler::with(key, &mut app, ActiveRadarrBlock::EditMoviePrompt, None).handle();

      assert!(app.data.radarr_data.prompt_confirm);

      EditMovieHandler::with(key, &mut app, ActiveRadarrBlock::EditMoviePrompt, None).handle();

      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[test]
    fn test_edit_movie_path_input_left_right_keys() {
      let mut app = App::default();
      app.data.radarr_data.edit_movie_modal = Some(EditMovieModal {
        path: "Test".into(),
        ..EditMovieModal::default()
      });

      EditMovieHandler::with(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveRadarrBlock::EditMoviePathInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .path
          .offset
          .load(Ordering::SeqCst),
        1
      );

      EditMovieHandler::with(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveRadarrBlock::EditMoviePathInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .path
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }

    #[test]
    fn test_edit_movie_tags_input_left_right_keys() {
      let mut app = App::default();
      app.data.radarr_data.edit_movie_modal = Some(EditMovieModal {
        tags: "Test".into(),
        ..EditMovieModal::default()
      });

      EditMovieHandler::with(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveRadarrBlock::EditMovieTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .tags
          .offset
          .load(Ordering::SeqCst),
        1
      );

      EditMovieHandler::with(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveRadarrBlock::EditMovieTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .tags
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }
  }

  mod test_handle_submit {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::models::servarr_data::radarr::modals::EditMovieModal;
    use crate::models::servarr_data::radarr::radarr_data::EDIT_MOVIE_SELECTION_BLOCKS;
    use crate::models::{BlockSelectionState, Route};
    use crate::network::radarr_network::RadarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_edit_movie_path_input_submit() {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.data.radarr_data.edit_movie_modal = Some(EditMovieModal {
        path: "Test Path".into(),
        ..EditMovieModal::default()
      });
      app.push_navigation_stack(ActiveRadarrBlock::EditMoviePrompt.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditMoviePathInput.into());

      EditMovieHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::EditMoviePathInput,
        None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert!(!app
        .data
        .radarr_data
        .edit_movie_modal
        .as_ref()
        .unwrap()
        .path
        .text
        .is_empty());
      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::EditMoviePrompt.into()
      );
    }

    #[test]
    fn test_edit_movie_tags_input_submit() {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.data.radarr_data.edit_movie_modal = Some(EditMovieModal {
        tags: "Test Tags".into(),
        ..EditMovieModal::default()
      });
      app.push_navigation_stack(ActiveRadarrBlock::EditMoviePrompt.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditMoviePathInput.into());

      EditMovieHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::EditMovieTagsInput,
        None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert!(!app
        .data
        .radarr_data
        .edit_movie_modal
        .as_mut()
        .unwrap()
        .tags
        .text
        .is_empty());
      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::EditMoviePrompt.into()
      );
    }

    #[test]
    fn test_edit_movie_prompt_prompt_decline_submit() {
      let mut app = App::default();
      app.data.radarr_data.edit_movie_modal = Some(EditMovieModal::default());
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditMoviePrompt.into());
      app.data.radarr_data.selected_block = BlockSelectionState::new(EDIT_MOVIE_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(0, EDIT_MOVIE_SELECTION_BLOCKS.len() - 1);

      EditMovieHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::EditMoviePrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Movies.into());
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
    }

    #[test]
    fn test_edit_movie_confirm_prompt_prompt_confirmation_submit() {
      let mut app = App::default();
      app.data.radarr_data.edit_movie_modal = Some(EditMovieModal::default());
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditMoviePrompt.into());
      app.data.radarr_data.prompt_confirm = true;
      app.data.radarr_data.selected_block = BlockSelectionState::new(EDIT_MOVIE_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(0, EDIT_MOVIE_SELECTION_BLOCKS.len() - 1);

      EditMovieHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::EditMoviePrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Movies.into());
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::EditMovie(None))
      );
      assert!(app.data.radarr_data.edit_movie_modal.is_some());
      assert!(app.should_refresh);
    }

    #[test]
    fn test_edit_movie_confirm_prompt_prompt_confirmation_submit_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.data.radarr_data.edit_movie_modal = Some(EditMovieModal::default());
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditMoviePrompt.into());
      app.data.radarr_data.prompt_confirm = true;

      EditMovieHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::EditMoviePrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::EditMoviePrompt.into()
      );
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
      assert!(!app.should_refresh);
    }

    #[test]
    fn test_edit_movie_toggle_monitored_submit() {
      let current_route = Route::from((
        ActiveRadarrBlock::EditMoviePrompt,
        Some(ActiveRadarrBlock::Movies),
      ));
      let mut app = App::default();
      app.data.radarr_data.edit_movie_modal = Some(EditMovieModal::default());
      app.data.radarr_data.selected_block = BlockSelectionState::new(EDIT_MOVIE_SELECTION_BLOCKS);
      app.push_navigation_stack(current_route);

      EditMovieHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::EditMoviePrompt,
        Some(ActiveRadarrBlock::Movies),
      )
      .handle();

      assert_eq!(app.get_current_route(), current_route);
      assert_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .monitored,
        Some(true)
      );

      EditMovieHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::EditMoviePrompt,
        Some(ActiveRadarrBlock::Movies),
      )
      .handle();

      assert_eq!(app.get_current_route(), current_route);
      assert_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .monitored,
        Some(false)
      );
    }

    #[rstest]
    #[case(ActiveRadarrBlock::EditMovieSelectMinimumAvailability, 1)]
    #[case(ActiveRadarrBlock::EditMovieSelectQualityProfile, 2)]
    #[case(ActiveRadarrBlock::EditMoviePathInput, 3)]
    #[case(ActiveRadarrBlock::EditMovieTagsInput, 4)]
    fn test_edit_movie_prompt_selected_block_submit(
      #[case] selected_block: ActiveRadarrBlock,
      #[case] y_index: usize,
    ) {
      let mut app = App::default();
      app.data.radarr_data.edit_movie_modal = Some(EditMovieModal::default());
      app.push_navigation_stack(
        (
          ActiveRadarrBlock::EditMoviePrompt,
          Some(ActiveRadarrBlock::Movies),
        )
          .into(),
      );
      app.data.radarr_data.selected_block = BlockSelectionState::new(EDIT_MOVIE_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.set_index(0, y_index);

      EditMovieHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::EditMoviePrompt,
        Some(ActiveRadarrBlock::Movies),
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        (selected_block, Some(ActiveRadarrBlock::Movies)).into()
      );
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);

      if selected_block == ActiveRadarrBlock::EditMoviePathInput
        || selected_block == ActiveRadarrBlock::EditMovieTagsInput
      {
        assert!(app.should_ignore_quit_key);
      }
    }

    #[rstest]
    fn test_edit_movie_prompt_selected_block_submit_no_op_when_not_ready(
      #[values(1, 2, 3, 4)] y_index: usize,
    ) {
      let mut app = App::default();
      app.is_loading = true;
      app.data.radarr_data.edit_movie_modal = Some(EditMovieModal::default());
      app.push_navigation_stack(
        (
          ActiveRadarrBlock::EditMoviePrompt,
          Some(ActiveRadarrBlock::Movies),
        )
          .into(),
      );
      app.data.radarr_data.selected_block = BlockSelectionState::new(EDIT_MOVIE_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.set_index(0, y_index);

      EditMovieHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::EditMoviePrompt,
        Some(ActiveRadarrBlock::Movies),
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        (
          ActiveRadarrBlock::EditMoviePrompt,
          Some(ActiveRadarrBlock::Movies),
        )
          .into()
      );
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
      assert!(!app.should_ignore_quit_key);
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
      app.data.radarr_data.edit_movie_modal = Some(EditMovieModal::default());
      app.push_navigation_stack(ActiveRadarrBlock::EditMoviePrompt.into());
      app.push_navigation_stack(active_radarr_block.into());

      EditMovieHandler::with(
        SUBMIT_KEY,
        &mut app,
        active_radarr_block,
        Some(ActiveRadarrBlock::Movies),
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::EditMoviePrompt.into()
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

    use crate::models::servarr_data::radarr::modals::EditMovieModal;
    use crate::models::servarr_data::radarr::radarr_data::radarr_test_utils::utils::create_test_radarr_data;

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

      EditMovieHandler::with(ESC_KEY, &mut app, active_radarr_block, None).handle();

      assert!(!app.should_ignore_quit_key);
      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::EditMoviePrompt.into()
      );
    }

    #[test]
    fn test_edit_movie_prompt_esc() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::EditMoviePrompt.into());
      app.data.radarr_data = create_test_radarr_data();
      app.data.radarr_data.edit_movie_modal = Some(EditMovieModal::default());

      EditMovieHandler::with(ESC_KEY, &mut app, ActiveRadarrBlock::EditMoviePrompt, None).handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Movies.into());

      assert!(app.data.radarr_data.edit_movie_modal.is_none());
      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[rstest]
    fn test_edit_movie_esc(
      #[values(
        ActiveRadarrBlock::EditMovieSelectMinimumAvailability,
        ActiveRadarrBlock::EditMovieSelectQualityProfile
      )]
      active_radarr_block: ActiveRadarrBlock,
      #[values(true, false)] is_ready: bool,
    ) {
      let mut app = App::default();
      app.is_loading = is_ready;
      app.data.radarr_data = create_test_radarr_data();
      app.push_navigation_stack(active_radarr_block.into());

      EditMovieHandler::with(ESC_KEY, &mut app, active_radarr_block, None).handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Movies.into());
    }
  }

  mod test_handle_key_char {
    use super::*;
    use crate::{
      models::{
        servarr_data::radarr::{modals::EditMovieModal, radarr_data::EDIT_MOVIE_SELECTION_BLOCKS},
        BlockSelectionState,
      },
      network::radarr_network::RadarrEvent,
    };

    #[test]
    fn test_edit_movie_path_input_backspace() {
      let mut app = App::default();
      app.data.radarr_data.edit_movie_modal = Some(EditMovieModal {
        path: "Test".into(),
        ..EditMovieModal::default()
      });

      EditMovieHandler::with(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveRadarrBlock::EditMoviePathInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .path
          .text,
        "Tes"
      );
    }

    #[test]
    fn test_edit_movie_tags_input_backspace() {
      let mut app = App::default();
      app.data.radarr_data.edit_movie_modal = Some(EditMovieModal {
        tags: "Test".into(),
        ..EditMovieModal::default()
      });

      EditMovieHandler::with(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveRadarrBlock::EditMovieTagsInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .tags
          .text,
        "Tes"
      );
    }

    #[test]
    fn test_edit_movie_path_input_char_key() {
      let mut app = App::default();
      app.data.radarr_data.edit_movie_modal = Some(EditMovieModal::default());

      EditMovieHandler::with(
        Key::Char('h'),
        &mut app,
        ActiveRadarrBlock::EditMoviePathInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .path
          .text,
        "h"
      );
    }

    #[test]
    fn test_edit_movie_tags_input_char_key() {
      let mut app = App::default();
      app.data.radarr_data.edit_movie_modal = Some(EditMovieModal::default());

      EditMovieHandler::with(
        Key::Char('h'),
        &mut app,
        ActiveRadarrBlock::EditMovieTagsInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .edit_movie_modal
          .as_ref()
          .unwrap()
          .tags
          .text,
        "h"
      );
    }

    #[test]
    fn test_edit_movie_confirm_prompt_prompt_confirm() {
      let mut app = App::default();
      app.data.radarr_data.edit_movie_modal = Some(EditMovieModal::default());
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditMoviePrompt.into());
      app.data.radarr_data.selected_block = BlockSelectionState::new(EDIT_MOVIE_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(0, EDIT_MOVIE_SELECTION_BLOCKS.len() - 1);

      EditMovieHandler::with(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveRadarrBlock::EditMoviePrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Movies.into());
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::EditMovie(None))
      );
      assert!(app.data.radarr_data.edit_movie_modal.is_some());
      assert!(app.should_refresh);
    }
  }

  #[test]
  fn test_edit_movie_handler_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if EDIT_MOVIE_BLOCKS.contains(&active_radarr_block) {
        assert!(EditMovieHandler::accepts(active_radarr_block));
      } else {
        assert!(!EditMovieHandler::accepts(active_radarr_block));
      }
    });
  }

  #[test]
  fn test_edit_movie_handler_is_not_ready_when_loading() {
    let mut app = App::default();
    app.is_loading = true;

    let handler = EditMovieHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::EditMoviePrompt,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_edit_movie_handler_is_not_ready_when_edit_movie_modal_is_none() {
    let mut app = App::default();
    app.is_loading = false;

    let handler = EditMovieHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::EditMoviePrompt,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_edit_movie_handler_is_ready_when_edit_movie_modal_is_some() {
    let mut app = App::default();
    app.is_loading = false;
    app.data.radarr_data.edit_movie_modal = Some(EditMovieModal::default());

    let handler = EditMovieHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::EditMoviePrompt,
      None,
    );

    assert!(handler.is_ready());
  }
}
