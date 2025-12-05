#[cfg(test)]
mod tests {
  use bimap::BiMap;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::assert_modal_absent;
  use crate::assert_navigation_pushed;
  use crate::event::Key;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::sonarr_handlers::library::edit_series_handler::EditSeriesHandler;
  use crate::handlers::sonarr_handlers::sonarr_handler_test_utils::utils::series;
  use crate::models::servarr_data::sonarr::modals::EditSeriesModal;
  use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, EDIT_SERIES_BLOCKS};
  use crate::models::sonarr_models::{EditSeriesParams, Series, SeriesType};

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use strum::IntoEnumIterator;

    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::sonarr::modals::EditSeriesModal;
    use crate::models::servarr_data::sonarr::sonarr_data::EDIT_SERIES_SELECTION_BLOCKS;

    use super::*;
		
    #[rstest]
    fn test_edit_series_select_series_type_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let series_type_vec = Vec::from_iter(SeriesType::iter());
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.edit_series_modal = Some(EditSeriesModal::default());
      app
        .data
        .sonarr_data
        .edit_series_modal
        .as_mut()
        .unwrap()
        .series_type_list
        .set_items(series_type_vec.clone());

      if key == Key::Up {
        for i in (0..series_type_vec.len()).rev() {
          EditSeriesHandler::new(
            key,
            &mut app,
            ActiveSonarrBlock::EditSeriesSelectSeriesType,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .sonarr_data
              .edit_series_modal
              .as_ref()
              .unwrap()
              .series_type_list
              .current_selection(),
            &series_type_vec[i]
          );
        }
      } else {
        for i in 0..series_type_vec.len() {
          EditSeriesHandler::new(
            key,
            &mut app,
            ActiveSonarrBlock::EditSeriesSelectSeriesType,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .sonarr_data
              .edit_series_modal
              .as_ref()
              .unwrap()
              .series_type_list
              .current_selection(),
            &series_type_vec[(i + 1) % series_type_vec.len()]
          );
        }
      }
    }

    #[rstest]
    fn test_edit_series_select_quality_profile_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.edit_series_modal = Some(EditSeriesModal::default());
      app
        .data
        .sonarr_data
        .edit_series_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .set_items(vec!["Test 1".to_owned(), "Test 2".to_owned()]);

      EditSeriesHandler::new(
        key,
        &mut app,
        ActiveSonarrBlock::EditSeriesSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 2"
      );

      EditSeriesHandler::new(
        key,
        &mut app,
        ActiveSonarrBlock::EditSeriesSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[rstest]
    fn test_edit_series_select_language_profile_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.edit_series_modal = Some(EditSeriesModal::default());
      app
        .data
        .sonarr_data
        .edit_series_modal
        .as_mut()
        .unwrap()
        .language_profile_list
        .set_items(vec!["Test 1".to_owned(), "Test 2".to_owned()]);

      EditSeriesHandler::new(
        key,
        &mut app,
        ActiveSonarrBlock::EditSeriesSelectLanguageProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .language_profile_list
          .current_selection(),
        "Test 2"
      );

      EditSeriesHandler::new(
        key,
        &mut app,
        ActiveSonarrBlock::EditSeriesSelectLanguageProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .language_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[rstest]
    fn test_edit_series_prompt_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.edit_series_modal = Some(EditSeriesModal::default());
      app.data.sonarr_data.selected_block = BlockSelectionState::new(EDIT_SERIES_SELECTION_BLOCKS);
      app.data.sonarr_data.selected_block.down();

      EditSeriesHandler::new(key, &mut app, ActiveSonarrBlock::EditSeriesPrompt, None).handle();

      if key == Key::Up {
        assert_eq!(
          app.data.sonarr_data.selected_block.get_active_block(),
          ActiveSonarrBlock::EditSeriesToggleMonitored
        );
      } else {
        assert_eq!(
          app.data.sonarr_data.selected_block.get_active_block(),
          ActiveSonarrBlock::EditSeriesSelectQualityProfile
        );
      }
    }

    #[rstest]
    fn test_edit_series_prompt_scroll_no_op_when_not_ready(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.is_loading = true;
      app.data.sonarr_data.edit_series_modal = Some(EditSeriesModal::default());
      app.data.sonarr_data.selected_block = BlockSelectionState::new(EDIT_SERIES_SELECTION_BLOCKS);
      app.data.sonarr_data.selected_block.down();

      EditSeriesHandler::new(key, &mut app, ActiveSonarrBlock::EditSeriesPrompt, None).handle();

      assert_eq!(
        app.data.sonarr_data.selected_block.get_active_block(),
        ActiveSonarrBlock::EditSeriesToggleSeasonFolder
      );
    }
  }

  mod test_handle_home_end {
    use pretty_assertions::assert_eq;
    use std::sync::atomic::Ordering;

    use strum::IntoEnumIterator;

    use crate::models::servarr_data::sonarr::modals::EditSeriesModal;

    use super::*;

    #[test]
    fn test_edit_series_select_series_type_home_end() {
      let series_type_vec = Vec::from_iter(SeriesType::iter());
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.edit_series_modal = Some(EditSeriesModal::default());
      app
        .data
        .sonarr_data
        .edit_series_modal
        .as_mut()
        .unwrap()
        .series_type_list
        .set_items(series_type_vec.clone());

      EditSeriesHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::EditSeriesSelectSeriesType,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .series_type_list
          .current_selection(),
        &series_type_vec[series_type_vec.len() - 1]
      );

      EditSeriesHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::EditSeriesSelectSeriesType,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .series_type_list
          .current_selection(),
        &series_type_vec[0]
      );
    }

    #[test]
    fn test_edit_series_select_quality_profile_scroll() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.edit_series_modal = Some(EditSeriesModal::default());
      app
        .data
        .sonarr_data
        .edit_series_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .set_items(vec![
          "Test 1".to_owned(),
          "Test 2".to_owned(),
          "Test 3".to_owned(),
        ]);

      EditSeriesHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::EditSeriesSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 3"
      );

      EditSeriesHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::EditSeriesSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[test]
    fn test_edit_series_select_language_profile_scroll() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.edit_series_modal = Some(EditSeriesModal::default());
      app
        .data
        .sonarr_data
        .edit_series_modal
        .as_mut()
        .unwrap()
        .language_profile_list
        .set_items(vec![
          "Test 1".to_owned(),
          "Test 2".to_owned(),
          "Test 3".to_owned(),
        ]);

      EditSeriesHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::EditSeriesSelectLanguageProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .language_profile_list
          .current_selection(),
        "Test 3"
      );

      EditSeriesHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::EditSeriesSelectLanguageProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .language_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[test]
    fn test_edit_series_path_input_home_end_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.edit_series_modal = Some(EditSeriesModal {
        path: "Test".into(),
        ..EditSeriesModal::default()
      });

      EditSeriesHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::EditSeriesPathInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .path
          .offset
          .load(Ordering::SeqCst),
        4
      );

      EditSeriesHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::EditSeriesPathInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .path
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }

    #[test]
    fn test_edit_series_tags_input_home_end_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.edit_series_modal = Some(EditSeriesModal {
        tags: "Test".into(),
        ..EditSeriesModal::default()
      });

      EditSeriesHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::EditSeriesTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .tags
          .offset
          .load(Ordering::SeqCst),
        4
      );

      EditSeriesHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::EditSeriesTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
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
    use pretty_assertions::assert_eq;
    use std::sync::atomic::Ordering;

    use crate::models::servarr_data::sonarr::modals::EditSeriesModal;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_left_right_prompt_toggle(#[values(Key::Left, Key::Right)] key: Key) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());

      EditSeriesHandler::new(key, &mut app, ActiveSonarrBlock::EditSeriesPrompt, None).handle();

      assert!(app.data.sonarr_data.prompt_confirm);

      EditSeriesHandler::new(key, &mut app, ActiveSonarrBlock::EditSeriesPrompt, None).handle();

      assert!(!app.data.sonarr_data.prompt_confirm);
    }

    #[test]
    fn test_edit_series_path_input_left_right_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.edit_series_modal = Some(EditSeriesModal {
        path: "Test".into(),
        ..EditSeriesModal::default()
      });

      EditSeriesHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveSonarrBlock::EditSeriesPathInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .path
          .offset
          .load(Ordering::SeqCst),
        1
      );

      EditSeriesHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveSonarrBlock::EditSeriesPathInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .path
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }

    #[test]
    fn test_edit_series_tags_input_left_right_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.edit_series_modal = Some(EditSeriesModal {
        tags: "Test".into(),
        ..EditSeriesModal::default()
      });

      EditSeriesHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveSonarrBlock::EditSeriesTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .tags
          .offset
          .load(Ordering::SeqCst),
        1
      );

      EditSeriesHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveSonarrBlock::EditSeriesTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
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
    use crate::assert_navigation_popped;
    use crate::models::servarr_data::sonarr::modals::EditSeriesModal;
    use crate::models::servarr_data::sonarr::sonarr_data::EDIT_SERIES_SELECTION_BLOCKS;
    use crate::models::{BlockSelectionState, Route};
    use crate::network::sonarr_network::SonarrEvent;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_edit_series_path_input_submit() {
      let mut app = App::test_default();
      app.ignore_special_keys_for_textbox_input = true;
      app.data.sonarr_data.edit_series_modal = Some(EditSeriesModal {
        path: "Test Path".into(),
        ..EditSeriesModal::default()
      });
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::EditSeriesPrompt.into());
      app.push_navigation_stack(ActiveSonarrBlock::EditSeriesPathInput.into());

      EditSeriesHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::EditSeriesPathInput,
        None,
      )
      .handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert!(
        !app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .path
          .text
          .is_empty()
      );
      assert_navigation_popped!(app, ActiveSonarrBlock::EditSeriesPrompt.into());
    }

    #[test]
    fn test_edit_series_tags_input_submit() {
      let mut app = App::test_default();
      app.ignore_special_keys_for_textbox_input = true;
      app.data.sonarr_data.edit_series_modal = Some(EditSeriesModal {
        tags: "Test Tags".into(),
        ..EditSeriesModal::default()
      });
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::EditSeriesPrompt.into());
      app.push_navigation_stack(ActiveSonarrBlock::EditSeriesPathInput.into());

      EditSeriesHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::EditSeriesTagsInput,
        None,
      )
      .handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert!(
        !app
          .data
          .sonarr_data
          .edit_series_modal
          .as_mut()
          .unwrap()
          .tags
          .text
          .is_empty()
      );
      assert_navigation_popped!(app, ActiveSonarrBlock::EditSeriesPrompt.into());
    }

    #[test]
    fn test_edit_series_prompt_prompt_decline_submit() {
      let mut app = App::test_default();
      app.data.sonarr_data.edit_series_modal = Some(EditSeriesModal::default());
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::EditSeriesPrompt.into());
      app.data.sonarr_data.selected_block = BlockSelectionState::new(EDIT_SERIES_SELECTION_BLOCKS);
      app
        .data
        .sonarr_data
        .selected_block
        .set_index(0, EDIT_SERIES_SELECTION_BLOCKS.len() - 1);

      EditSeriesHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::EditSeriesPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveSonarrBlock::Series.into());
      assert_eq!(app.data.sonarr_data.prompt_confirm_action, None);
    }

    #[test]
    fn test_edit_series_confirm_prompt_prompt_confirmation_submit() {
      let mut app = App::test_default();
      let mut edit_series = EditSeriesModal {
        tags: "usenet, testing".to_owned().into(),
        path: "/nfs/Test Path".to_owned().into(),
        monitored: Some(false),
        use_season_folders: Some(false),
        ..EditSeriesModal::default()
      };
      edit_series
        .quality_profile_list
        .set_items(vec!["Any".to_owned(), "HD - 1080p".to_owned()]);
      edit_series
        .language_profile_list
        .set_items(vec!["Any".to_owned(), "English".to_owned()]);
      edit_series
        .series_type_list
        .set_items(Vec::from_iter(SeriesType::iter()));
      app.data.sonarr_data.edit_series_modal = Some(edit_series);
      app.data.sonarr_data.series.set_items(vec![Series {
        monitored: false,
        season_folder: false,
        ..series()
      }]);
      app.data.sonarr_data.quality_profile_map =
        BiMap::from_iter([(1111, "Any".to_owned()), (2222, "HD - 1080p".to_owned())]);
      app.data.sonarr_data.language_profiles_map =
        BiMap::from_iter([(1111, "Any".to_owned()), (2222, "English".to_owned())]);
      let expected_edit_series_params = EditSeriesParams {
        series_id: 1,
        monitored: Some(false),
        use_season_folders: Some(false),
        series_type: Some(SeriesType::Standard),
        quality_profile_id: Some(1111),
        language_profile_id: Some(1111),
        root_folder_path: Some("/nfs/Test Path".to_owned()),
        tag_input_string: Some("usenet, testing".to_owned()),
        ..EditSeriesParams::default()
      };
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::EditSeriesPrompt.into());
      app.data.sonarr_data.prompt_confirm = true;
      app.data.sonarr_data.selected_block = BlockSelectionState::new(EDIT_SERIES_SELECTION_BLOCKS);
      app
        .data
        .sonarr_data
        .selected_block
        .set_index(0, EDIT_SERIES_SELECTION_BLOCKS.len() - 1);

      EditSeriesHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::EditSeriesPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveSonarrBlock::Series.into());
      assert_eq!(
        app.data.sonarr_data.prompt_confirm_action,
        Some(SonarrEvent::EditSeries(expected_edit_series_params))
      );
      assert_modal_absent!(app.data.sonarr_data.edit_series_modal);
      assert!(app.should_refresh);
    }

    #[test]
    fn test_edit_series_confirm_prompt_prompt_confirmation_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.data.sonarr_data.edit_series_modal = Some(EditSeriesModal::default());
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::EditSeriesPrompt.into());
      app.data.sonarr_data.prompt_confirm = true;

      EditSeriesHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::EditSeriesPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::EditSeriesPrompt.into()
      );
      assert_eq!(app.data.sonarr_data.prompt_confirm_action, None);
      assert!(!app.should_refresh);
    }

    #[test]
    fn test_edit_series_toggle_monitored_submit() {
      let current_route = Route::from((
        ActiveSonarrBlock::EditSeriesPrompt,
        Some(ActiveSonarrBlock::Series),
      ));
      let mut app = App::test_default();
      app.data.sonarr_data.edit_series_modal = Some(EditSeriesModal::default());
      app.data.sonarr_data.selected_block = BlockSelectionState::new(EDIT_SERIES_SELECTION_BLOCKS);
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(current_route);

      EditSeriesHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::EditSeriesPrompt,
        Some(ActiveSonarrBlock::Series),
      )
      .handle();

      assert_eq!(app.get_current_route(), current_route);
      assert_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .monitored,
        Some(true)
      );

      EditSeriesHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::EditSeriesPrompt,
        Some(ActiveSonarrBlock::Series),
      )
      .handle();

      assert_eq!(app.get_current_route(), current_route);
      assert_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .monitored,
        Some(false)
      );
    }

    #[test]
    fn test_edit_series_toggle_use_season_folders_submit() {
      let current_route = Route::from((
        ActiveSonarrBlock::EditSeriesPrompt,
        Some(ActiveSonarrBlock::Series),
      ));
      let mut app = App::test_default();
      app.data.sonarr_data.edit_series_modal = Some(EditSeriesModal::default());
      app.data.sonarr_data.selected_block = BlockSelectionState::new(EDIT_SERIES_SELECTION_BLOCKS);
      app.data.sonarr_data.selected_block.set_index(0, 1);
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(current_route);

      EditSeriesHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::EditSeriesPrompt,
        Some(ActiveSonarrBlock::Series),
      )
      .handle();

      assert_eq!(app.get_current_route(), current_route);
      assert_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .use_season_folders,
        Some(true)
      );

      EditSeriesHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::EditSeriesPrompt,
        Some(ActiveSonarrBlock::Series),
      )
      .handle();

      assert_eq!(app.get_current_route(), current_route);
      assert_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .use_season_folders,
        Some(false)
      );
    }

    #[rstest]
    #[case(ActiveSonarrBlock::EditSeriesSelectQualityProfile, 2)]
    #[case(ActiveSonarrBlock::EditSeriesSelectLanguageProfile, 3)]
    #[case(ActiveSonarrBlock::EditSeriesSelectSeriesType, 4)]
    #[case(ActiveSonarrBlock::EditSeriesPathInput, 5)]
    #[case(ActiveSonarrBlock::EditSeriesTagsInput, 6)]
    fn test_edit_series_prompt_selected_block_submit(
      #[case] selected_block: ActiveSonarrBlock,
      #[case] y_index: usize,
    ) {
      let mut app = App::test_default();
      app.data.sonarr_data.edit_series_modal = Some(EditSeriesModal::default());
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(
        (
          ActiveSonarrBlock::EditSeriesPrompt,
          Some(ActiveSonarrBlock::Series),
        )
          .into(),
      );
      app.data.sonarr_data.selected_block = BlockSelectionState::new(EDIT_SERIES_SELECTION_BLOCKS);
      app.data.sonarr_data.selected_block.set_index(0, y_index);

      EditSeriesHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::EditSeriesPrompt,
        Some(ActiveSonarrBlock::Series),
      )
      .handle();

      assert_navigation_pushed!(
        app,
        (selected_block, Some(ActiveSonarrBlock::Series)).into()
      );
      assert_eq!(app.data.sonarr_data.prompt_confirm_action, None);

      if selected_block == ActiveSonarrBlock::EditSeriesPathInput
        || selected_block == ActiveSonarrBlock::EditSeriesTagsInput
      {
        assert!(app.ignore_special_keys_for_textbox_input);
      }
    }

    #[rstest]
    fn test_edit_series_prompt_selected_block_submit_no_op_when_not_ready(
      #[values(1, 2, 3, 4)] y_index: usize,
    ) {
      let mut app = App::test_default();
      app.is_loading = true;
      app.data.sonarr_data.edit_series_modal = Some(EditSeriesModal::default());
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(
        (
          ActiveSonarrBlock::EditSeriesPrompt,
          Some(ActiveSonarrBlock::Series),
        )
          .into(),
      );
      app.data.sonarr_data.selected_block = BlockSelectionState::new(EDIT_SERIES_SELECTION_BLOCKS);
      app.data.sonarr_data.selected_block.set_index(0, y_index);

      EditSeriesHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::EditSeriesPrompt,
        Some(ActiveSonarrBlock::Series),
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        (
          ActiveSonarrBlock::EditSeriesPrompt,
          Some(ActiveSonarrBlock::Series),
        )
          .into()
      );
      assert_eq!(app.data.sonarr_data.prompt_confirm_action, None);
      assert!(!app.ignore_special_keys_for_textbox_input);
    }

    #[rstest]
    fn test_edit_series_prompt_selecting_preferences_blocks_submit(
      #[values(
        ActiveSonarrBlock::EditSeriesSelectSeriesType,
        ActiveSonarrBlock::EditSeriesSelectQualityProfile,
        ActiveSonarrBlock::EditSeriesSelectLanguageProfile,
        ActiveSonarrBlock::EditSeriesPathInput,
        ActiveSonarrBlock::EditSeriesTagsInput
      )]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::test_default();
      app.data.sonarr_data.edit_series_modal = Some(EditSeriesModal::default());
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::EditSeriesPrompt.into());
      app.push_navigation_stack(active_sonarr_block.into());

      EditSeriesHandler::new(
        SUBMIT_KEY,
        &mut app,
        active_sonarr_block,
        Some(ActiveSonarrBlock::Series),
      )
      .handle();

      assert_navigation_popped!(app, ActiveSonarrBlock::EditSeriesPrompt.into());

      if active_sonarr_block == ActiveSonarrBlock::EditSeriesPathInput
        || active_sonarr_block == ActiveSonarrBlock::EditSeriesTagsInput
      {
        assert!(!app.ignore_special_keys_for_textbox_input);
      }
    }
  }

  mod test_handle_esc {
    use crate::assert_navigation_popped;
    use crate::models::servarr_data::sonarr::modals::EditSeriesModal;
    use crate::models::servarr_data::sonarr::sonarr_data::sonarr_test_utils::utils::create_test_sonarr_data;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_edit_series_input_esc(
      #[values(
        ActiveSonarrBlock::EditSeriesTagsInput,
        ActiveSonarrBlock::EditSeriesPathInput
      )]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::test_default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.ignore_special_keys_for_textbox_input = true;
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::EditSeriesPrompt.into());
      app.push_navigation_stack(active_sonarr_block.into());

      EditSeriesHandler::new(ESC_KEY, &mut app, active_sonarr_block, None).handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert_navigation_popped!(app, ActiveSonarrBlock::EditSeriesPrompt.into());
    }

    #[test]
    fn test_edit_series_prompt_esc() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::EditSeriesPrompt.into());
      app.data.sonarr_data = create_test_sonarr_data();
      app.data.sonarr_data.edit_series_modal = Some(EditSeriesModal::default());

      EditSeriesHandler::new(ESC_KEY, &mut app, ActiveSonarrBlock::EditSeriesPrompt, None).handle();

      assert_navigation_popped!(app, ActiveSonarrBlock::Series.into());

      assert_modal_absent!(app.data.sonarr_data.edit_series_modal);
      assert!(!app.data.sonarr_data.prompt_confirm);
    }

    #[rstest]
    fn test_edit_series_esc(
      #[values(
        ActiveSonarrBlock::EditSeriesSelectSeriesType,
        ActiveSonarrBlock::EditSeriesSelectQualityProfile,
        ActiveSonarrBlock::EditSeriesSelectLanguageProfile
      )]
      active_sonarr_block: ActiveSonarrBlock,
      #[values(true, false)] is_ready: bool,
    ) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.data.sonarr_data = create_test_sonarr_data();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(active_sonarr_block.into());

      EditSeriesHandler::new(ESC_KEY, &mut app, active_sonarr_block, None).handle();

      assert_navigation_popped!(app, ActiveSonarrBlock::Series.into());
    }
  }

  mod test_handle_key_char {
    use super::*;
    use crate::{
      assert_navigation_popped,
      models::{
        BlockSelectionState,
        servarr_data::sonarr::{
          modals::EditSeriesModal, sonarr_data::EDIT_SERIES_SELECTION_BLOCKS,
        },
      },
      network::sonarr_network::SonarrEvent,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn test_edit_series_path_input_backspace() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.edit_series_modal = Some(EditSeriesModal {
        path: "Test".into(),
        ..EditSeriesModal::default()
      });

      EditSeriesHandler::new(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveSonarrBlock::EditSeriesPathInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .path
          .text,
        "Tes"
      );
    }

    #[test]
    fn test_edit_series_tags_input_backspace() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.edit_series_modal = Some(EditSeriesModal {
        tags: "Test".into(),
        ..EditSeriesModal::default()
      });

      EditSeriesHandler::new(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveSonarrBlock::EditSeriesTagsInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .tags
          .text,
        "Tes"
      );
    }

    #[test]
    fn test_edit_series_path_input_char_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.edit_series_modal = Some(EditSeriesModal::default());

      EditSeriesHandler::new(
        Key::Char('a'),
        &mut app,
        ActiveSonarrBlock::EditSeriesPathInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .path
          .text,
        "a"
      );
    }

    #[test]
    fn test_edit_series_tags_input_char_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.edit_series_modal = Some(EditSeriesModal::default());

      EditSeriesHandler::new(
        Key::Char('a'),
        &mut app,
        ActiveSonarrBlock::EditSeriesTagsInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .edit_series_modal
          .as_ref()
          .unwrap()
          .tags
          .text,
        "a"
      );
    }

    #[test]
    fn test_edit_series_confirm_prompt_prompt_confirm() {
      let mut app = App::test_default();
      let mut edit_series = EditSeriesModal {
        tags: "usenet, testing".to_owned().into(),
        path: "/nfs/Test Path".to_owned().into(),
        monitored: Some(false),
        use_season_folders: Some(false),
        ..EditSeriesModal::default()
      };
      edit_series
        .quality_profile_list
        .set_items(vec!["Any".to_owned(), "HD - 1080p".to_owned()]);
      edit_series
        .language_profile_list
        .set_items(vec!["Any".to_owned(), "English".to_owned()]);
      edit_series
        .series_type_list
        .set_items(Vec::from_iter(SeriesType::iter()));
      app.data.sonarr_data.edit_series_modal = Some(edit_series);
      app.data.sonarr_data.series.set_items(vec![Series {
        monitored: false,
        season_folder: false,
        ..series()
      }]);
      app.data.sonarr_data.quality_profile_map =
        BiMap::from_iter([(1111, "Any".to_owned()), (2222, "HD - 1080p".to_owned())]);
      app.data.sonarr_data.language_profiles_map =
        BiMap::from_iter([(1111, "Any".to_owned()), (2222, "English".to_owned())]);
      let expected_edit_series_params = EditSeriesParams {
        series_id: 1,
        monitored: Some(false),
        use_season_folders: Some(false),
        series_type: Some(SeriesType::Standard),
        quality_profile_id: Some(1111),
        language_profile_id: Some(1111),
        root_folder_path: Some("/nfs/Test Path".to_owned()),
        tag_input_string: Some("usenet, testing".to_owned()),
        ..EditSeriesParams::default()
      };
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::EditSeriesPrompt.into());
      app.data.sonarr_data.selected_block = BlockSelectionState::new(EDIT_SERIES_SELECTION_BLOCKS);
      app
        .data
        .sonarr_data
        .selected_block
        .set_index(0, EDIT_SERIES_SELECTION_BLOCKS.len() - 1);

      EditSeriesHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveSonarrBlock::EditSeriesPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveSonarrBlock::Series.into());
      assert_eq!(
        app.data.sonarr_data.prompt_confirm_action,
        Some(SonarrEvent::EditSeries(expected_edit_series_params))
      );
      assert_modal_absent!(app.data.sonarr_data.edit_series_modal);
      assert!(app.should_refresh);
    }
  }

  #[test]
  fn test_edit_series_handler_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if EDIT_SERIES_BLOCKS.contains(&active_sonarr_block) {
        assert!(EditSeriesHandler::accepts(active_sonarr_block));
      } else {
        assert!(!EditSeriesHandler::accepts(active_sonarr_block));
      }
    });
  }

  #[rstest]
  fn test_edit_series_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = EditSeriesHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::default(),
      None,
    );

    assert_eq!(
      handler.ignore_special_keys(),
      ignore_special_keys_for_textbox_input
    );
  }

  #[test]
  fn test_build_edit_series_params() {
    let mut app = App::test_default();
    let mut edit_series = EditSeriesModal {
      tags: "usenet, testing".to_owned().into(),
      path: "/nfs/Test Path".to_owned().into(),
      monitored: Some(false),
      use_season_folders: Some(false),
      ..EditSeriesModal::default()
    };
    edit_series
      .quality_profile_list
      .set_items(vec!["Any".to_owned(), "HD - 1080p".to_owned()]);
    edit_series
      .language_profile_list
      .set_items(vec!["Any".to_owned(), "English".to_owned()]);
    edit_series
      .series_type_list
      .set_items(Vec::from_iter(SeriesType::iter()));
    app.data.sonarr_data.edit_series_modal = Some(edit_series);
    app.data.sonarr_data.series.set_items(vec![Series {
      monitored: false,
      season_folder: false,
      ..series()
    }]);
    app.data.sonarr_data.quality_profile_map =
      BiMap::from_iter([(1111, "Any".to_owned()), (2222, "HD - 1080p".to_owned())]);
    app.data.sonarr_data.language_profiles_map =
      BiMap::from_iter([(1111, "Any".to_owned()), (2222, "English".to_owned())]);
    let expected_edit_series_params = EditSeriesParams {
      series_id: 1,
      monitored: Some(false),
      use_season_folders: Some(false),
      series_type: Some(SeriesType::Standard),
      quality_profile_id: Some(1111),
      language_profile_id: Some(1111),
      root_folder_path: Some("/nfs/Test Path".to_owned()),
      tag_input_string: Some("usenet, testing".to_owned()),
      ..EditSeriesParams::default()
    };

    let edit_series_params = EditSeriesHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::EditSeriesPrompt,
      None,
    )
    .build_edit_series_params();

    assert_eq!(edit_series_params, expected_edit_series_params);
    assert_modal_absent!(app.data.sonarr_data.edit_series_modal);
  }

  #[test]
  fn test_edit_series_handler_is_not_ready_when_loading() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveSonarrBlock::Series.into());
    app.is_loading = true;

    let handler = EditSeriesHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::EditSeriesPrompt,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_edit_series_handler_is_not_ready_when_edit_series_modal_is_none() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveSonarrBlock::Series.into());
    app.is_loading = false;

    let handler = EditSeriesHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::EditSeriesPrompt,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_edit_series_handler_is_ready_when_edit_series_modal_is_some() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveSonarrBlock::Series.into());
    app.is_loading = false;
    app.data.sonarr_data.edit_series_modal = Some(EditSeriesModal::default());

    let handler = EditSeriesHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::EditSeriesPrompt,
      None,
    );

    assert!(handler.is_ready());
  }
}
