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
  use crate::handlers::lidarr_handlers::library::edit_artist_handler::EditArtistHandler;
  use crate::models::lidarr_models::{Artist, EditArtistParams, NewItemMonitorType};
  use crate::models::servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, EDIT_ARTIST_BLOCKS};
  use crate::models::servarr_data::lidarr::modals::EditArtistModal;
  use crate::network::lidarr_network::LidarrEvent;

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use strum::IntoEnumIterator;

    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::lidarr::lidarr_data::EDIT_ARTIST_SELECTION_BLOCKS;

    use super::*;

    #[rstest]
    fn test_edit_artist_select_monitor_new_items_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let monitor_type_vec = Vec::from_iter(NewItemMonitorType::iter());
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal::default());
      app
        .data
        .lidarr_data
        .edit_artist_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .set_items(monitor_type_vec.clone());

      if key == Key::Up {
        for i in (0..monitor_type_vec.len()).rev() {
          EditArtistHandler::new(
            key,
            &mut app,
            ActiveLidarrBlock::EditArtistSelectMonitorNewItems,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .lidarr_data
              .edit_artist_modal
              .as_ref()
              .unwrap()
              .monitor_list
              .current_selection(),
            &monitor_type_vec[i]
          );
        }
      } else {
        for i in 0..monitor_type_vec.len() {
          EditArtistHandler::new(
            key,
            &mut app,
            ActiveLidarrBlock::EditArtistSelectMonitorNewItems,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .lidarr_data
              .edit_artist_modal
              .as_ref()
              .unwrap()
              .monitor_list
              .current_selection(),
            &monitor_type_vec[(i + 1) % monitor_type_vec.len()]
          );
        }
      }
    }

    #[rstest]
    fn test_edit_artist_select_quality_profile_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal::default());
      app
        .data
        .lidarr_data
        .edit_artist_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .set_items(vec!["Test 1".to_owned(), "Test 2".to_owned()]);

      EditArtistHandler::new(
        key,
        &mut app,
        ActiveLidarrBlock::EditArtistSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .edit_artist_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 2"
      );

      EditArtistHandler::new(
        key,
        &mut app,
        ActiveLidarrBlock::EditArtistSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .edit_artist_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[rstest]
    fn test_edit_artist_select_metadata_profile_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal::default());
      app
        .data
        .lidarr_data
        .edit_artist_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .set_items(vec!["Test 1".to_owned(), "Test 2".to_owned()]);

      EditArtistHandler::new(
        key,
        &mut app,
        ActiveLidarrBlock::EditArtistSelectMetadataProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .edit_artist_modal
          .as_ref()
          .unwrap()
          .metadata_profile_list
          .current_selection(),
        "Test 2"
      );

      EditArtistHandler::new(
        key,
        &mut app,
        ActiveLidarrBlock::EditArtistSelectMetadataProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .edit_artist_modal
          .as_ref()
          .unwrap()
          .metadata_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[rstest]
    fn test_edit_artist_prompt_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal::default());
      app.data.lidarr_data.selected_block = BlockSelectionState::new(EDIT_ARTIST_SELECTION_BLOCKS);
      app.data.lidarr_data.selected_block.down();

      EditArtistHandler::new(key, &mut app, ActiveLidarrBlock::EditArtistPrompt, None).handle();

      if key == Key::Up {
        assert_eq!(
          app.data.lidarr_data.selected_block.get_active_block(),
          ActiveLidarrBlock::EditArtistToggleMonitored
        );
      } else {
        assert_eq!(
          app.data.lidarr_data.selected_block.get_active_block(),
          ActiveLidarrBlock::EditArtistSelectQualityProfile
        );
      }
    }

    #[rstest]
    fn test_edit_artist_prompt_scroll_no_op_when_not_ready(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.is_loading = true;
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal::default());
      app.data.lidarr_data.selected_block = BlockSelectionState::new(EDIT_ARTIST_SELECTION_BLOCKS);
      app.data.lidarr_data.selected_block.down();

      EditArtistHandler::new(key, &mut app, ActiveLidarrBlock::EditArtistPrompt, None).handle();

      assert_eq!(
        app.data.lidarr_data.selected_block.get_active_block(),
        ActiveLidarrBlock::EditArtistSelectMonitorNewItems
      );
    }
  }

  mod test_handle_home_end {
    use pretty_assertions::assert_eq;
    use std::sync::atomic::Ordering;

    use strum::IntoEnumIterator;

    use crate::models::servarr_data::lidarr::modals::EditArtistModal;

    use super::*;

    #[test]
    fn test_edit_artist_select_monitor_new_items_home_end() {
      let monitor_type_vec = Vec::from_iter(NewItemMonitorType::iter());
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal::default());
      app
        .data
        .lidarr_data
        .edit_artist_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .set_items(monitor_type_vec.clone());

      EditArtistHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveLidarrBlock::EditArtistSelectMonitorNewItems,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .edit_artist_modal
          .as_ref()
          .unwrap()
          .monitor_list
          .current_selection(),
        &monitor_type_vec[monitor_type_vec.len() - 1]
      );

      EditArtistHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveLidarrBlock::EditArtistSelectMonitorNewItems,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .edit_artist_modal
          .as_ref()
          .unwrap()
          .monitor_list
          .current_selection(),
        &monitor_type_vec[0]
      );
    }

    #[test]
    fn test_edit_artist_select_quality_profile_home_end() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal::default());
      app
        .data
        .lidarr_data
        .edit_artist_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .set_items(vec![
          "Test 1".to_owned(),
          "Test 2".to_owned(),
          "Test 3".to_owned(),
        ]);

      EditArtistHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveLidarrBlock::EditArtistSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .edit_artist_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 3"
      );

      EditArtistHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveLidarrBlock::EditArtistSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .edit_artist_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[test]
    fn test_edit_artist_select_metadata_profile_home_end() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal::default());
      app
        .data
        .lidarr_data
        .edit_artist_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .set_items(vec![
          "Test 1".to_owned(),
          "Test 2".to_owned(),
          "Test 3".to_owned(),
        ]);

      EditArtistHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveLidarrBlock::EditArtistSelectMetadataProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .edit_artist_modal
          .as_ref()
          .unwrap()
          .metadata_profile_list
          .current_selection(),
        "Test 3"
      );

      EditArtistHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveLidarrBlock::EditArtistSelectMetadataProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .edit_artist_modal
          .as_ref()
          .unwrap()
          .metadata_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[test]
    fn test_edit_artist_path_input_home_end_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal {
        path: "Test".into(),
        ..EditArtistModal::default()
      });

      EditArtistHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveLidarrBlock::EditArtistPathInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .edit_artist_modal
          .as_ref()
          .unwrap()
          .path
          .offset
          .load(Ordering::SeqCst),
        4
      );

      EditArtistHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveLidarrBlock::EditArtistPathInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .edit_artist_modal
          .as_ref()
          .unwrap()
          .path
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }

    #[test]
    fn test_edit_artist_tags_input_home_end_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal {
        tags: "Test".into(),
        ..EditArtistModal::default()
      });

      EditArtistHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveLidarrBlock::EditArtistTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .edit_artist_modal
          .as_ref()
          .unwrap()
          .tags
          .offset
          .load(Ordering::SeqCst),
        4
      );

      EditArtistHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveLidarrBlock::EditArtistTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .edit_artist_modal
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

    use crate::models::servarr_data::lidarr::modals::EditArtistModal;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_left_right_prompt_toggle(#[values(Key::Left, Key::Right)] key: Key) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal::default());

      EditArtistHandler::new(key, &mut app, ActiveLidarrBlock::EditArtistPrompt, None).handle();

      assert!(app.data.lidarr_data.prompt_confirm);

      EditArtistHandler::new(key, &mut app, ActiveLidarrBlock::EditArtistPrompt, None).handle();

      assert!(!app.data.lidarr_data.prompt_confirm);
    }

    #[test]
    fn test_edit_artist_path_input_left_right_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal {
        path: "Test".into(),
        ..EditArtistModal::default()
      });

      EditArtistHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveLidarrBlock::EditArtistPathInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .edit_artist_modal
          .as_ref()
          .unwrap()
          .path
          .offset
          .load(Ordering::SeqCst),
        1
      );

      EditArtistHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveLidarrBlock::EditArtistPathInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .edit_artist_modal
          .as_ref()
          .unwrap()
          .path
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }

    #[test]
    fn test_edit_artist_tags_input_left_right_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal {
        tags: "Test".into(),
        ..EditArtistModal::default()
      });

      EditArtistHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveLidarrBlock::EditArtistTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .edit_artist_modal
          .as_ref()
          .unwrap()
          .tags
          .offset
          .load(Ordering::SeqCst),
        1
      );

      EditArtistHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveLidarrBlock::EditArtistTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .edit_artist_modal
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
    use crate::models::servarr_data::lidarr::lidarr_data::EDIT_ARTIST_SELECTION_BLOCKS;
    use crate::models::{BlockSelectionState, Route};
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_edit_artist_path_input_submit() {
      let mut app = App::test_default();
      app.ignore_special_keys_for_textbox_input = true;
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal {
        path: "Test Path".into(),
        ..EditArtistModal::default()
      });
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::EditArtistPrompt.into());
      app.push_navigation_stack(ActiveLidarrBlock::EditArtistPathInput.into());

      EditArtistHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::EditArtistPathInput,
        None,
      )
      .handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert!(
        !app
          .data
          .lidarr_data
          .edit_artist_modal
          .as_ref()
          .unwrap()
          .path
          .text
          .is_empty()
      );
      assert_navigation_popped!(app, ActiveLidarrBlock::EditArtistPrompt.into());
    }

    #[test]
    fn test_edit_artist_tags_input_submit() {
      let mut app = App::test_default();
      app.ignore_special_keys_for_textbox_input = true;
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal {
        tags: "Test Tags".into(),
        ..EditArtistModal::default()
      });
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::EditArtistPrompt.into());
      app.push_navigation_stack(ActiveLidarrBlock::EditArtistTagsInput.into());

      EditArtistHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::EditArtistTagsInput,
        None,
      )
      .handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert!(
        !app
          .data
          .lidarr_data
          .edit_artist_modal
          .as_mut()
          .unwrap()
          .tags
          .text
          .is_empty()
      );
      assert_navigation_popped!(app, ActiveLidarrBlock::EditArtistPrompt.into());
    }

    #[test]
    fn test_edit_artist_prompt_prompt_decline_submit() {
      let mut app = App::test_default();
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal::default());
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::EditArtistPrompt.into());
      app.data.lidarr_data.selected_block = BlockSelectionState::new(EDIT_ARTIST_SELECTION_BLOCKS);
      app
        .data
        .lidarr_data
        .selected_block
        .set_index(0, EDIT_ARTIST_SELECTION_BLOCKS.len() - 1);

      EditArtistHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::EditArtistPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::Artists.into());
      assert_none!(app.data.lidarr_data.prompt_confirm_action);
    }

    #[test]
    fn test_edit_artist_confirm_prompt_prompt_confirmation_submit() {
      let mut app = App::test_default();
      let mut edit_artist = EditArtistModal {
        tags: "usenet, testing".to_owned().into(),
        path: "/nfs/Test Path".to_owned().into(),
        monitored: Some(false),
        ..EditArtistModal::default()
      };
      edit_artist
        .quality_profile_list
        .set_items(vec!["Lossless".to_owned(), "HD - 1080p".to_owned()]);
      edit_artist
        .metadata_profile_list
        .set_items(vec!["Standard".to_owned(), "Full".to_owned()]);
      edit_artist
        .monitor_list
        .set_items(Vec::from_iter(NewItemMonitorType::iter()));
      app.data.lidarr_data.edit_artist_modal = Some(edit_artist);
      app.data.lidarr_data.artists.set_items(vec![Artist {
        monitored: false,
        ..Artist::default()
      }]);
      app.data.lidarr_data.quality_profile_map = BiMap::from_iter([
        (1111, "Lossless".to_owned()),
        (2222, "HD - 1080p".to_owned()),
      ]);
      app.data.lidarr_data.metadata_profile_map =
        BiMap::from_iter([(1111, "Standard".to_owned()), (2222, "Full".to_owned())]);
      let expected_edit_artist_params = EditArtistParams {
        artist_id: 0,
        monitored: Some(false),
        monitor_new_items: Some(NewItemMonitorType::All),
        quality_profile_id: Some(1111),
        metadata_profile_id: Some(1111),
        root_folder_path: Some("/nfs/Test Path".to_owned()),
        tag_input_string: Some("usenet, testing".to_owned()),
        ..EditArtistParams::default()
      };
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::EditArtistPrompt.into());
      app.data.lidarr_data.prompt_confirm = true;
      app.data.lidarr_data.selected_block = BlockSelectionState::new(EDIT_ARTIST_SELECTION_BLOCKS);
      app
        .data
        .lidarr_data
        .selected_block
        .set_index(0, EDIT_ARTIST_SELECTION_BLOCKS.len() - 1);

      EditArtistHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::EditArtistPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::Artists.into());
      assert_eq!(
        app.data.lidarr_data.prompt_confirm_action,
        Some(LidarrEvent::EditArtist(expected_edit_artist_params))
      );
      assert_modal_absent!(app.data.lidarr_data.edit_artist_modal);
      assert!(app.should_refresh);
    }

    #[test]
    fn test_edit_artist_confirm_prompt_prompt_confirmation_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal::default());
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::EditArtistPrompt.into());
      app.data.lidarr_data.prompt_confirm = true;

      EditArtistHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::EditArtistPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::EditArtistPrompt.into()
      );
      assert_none!(app.data.lidarr_data.prompt_confirm_action);
      assert!(!app.should_refresh);
    }

    #[test]
    fn test_edit_artist_toggle_monitored_submit() {
      let current_route = Route::from((
        ActiveLidarrBlock::EditArtistPrompt,
        Some(ActiveLidarrBlock::Artists),
      ));
      let mut app = App::test_default();
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal::default());
      app.data.lidarr_data.selected_block = BlockSelectionState::new(EDIT_ARTIST_SELECTION_BLOCKS);
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(current_route);

      EditArtistHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::EditArtistPrompt,
        Some(ActiveLidarrBlock::Artists),
      )
      .handle();

      assert_eq!(app.get_current_route(), current_route);
      assert_some_eq_x!(
        app
          .data
          .lidarr_data
          .edit_artist_modal
          .as_ref()
          .unwrap()
          .monitored,
        true
      );

      EditArtistHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::EditArtistPrompt,
        Some(ActiveLidarrBlock::Artists),
      )
      .handle();

      assert_eq!(app.get_current_route(), current_route);
      assert_some_eq_x!(
        app
          .data
          .lidarr_data
          .edit_artist_modal
          .as_ref()
          .unwrap()
          .monitored,
        false
      );
    }

    #[rstest]
    #[case(ActiveLidarrBlock::EditArtistSelectQualityProfile, 2)]
    #[case(ActiveLidarrBlock::EditArtistSelectMetadataProfile, 3)]
    #[case(ActiveLidarrBlock::EditArtistPathInput, 4)]
    #[case(ActiveLidarrBlock::EditArtistTagsInput, 5)]
    fn test_edit_artist_prompt_selected_block_submit(
      #[case] selected_block: ActiveLidarrBlock,
      #[case] y_index: usize,
    ) {
      let mut app = App::test_default();
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal::default());
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(
        (
          ActiveLidarrBlock::EditArtistPrompt,
          Some(ActiveLidarrBlock::Artists),
        )
          .into(),
      );
      app.data.lidarr_data.selected_block = BlockSelectionState::new(EDIT_ARTIST_SELECTION_BLOCKS);
      app.data.lidarr_data.selected_block.set_index(0, y_index);

      EditArtistHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::EditArtistPrompt,
        Some(ActiveLidarrBlock::Artists),
      )
      .handle();

      assert_navigation_pushed!(
        app,
        (selected_block, Some(ActiveLidarrBlock::Artists)).into()
      );
      assert_none!(app.data.lidarr_data.prompt_confirm_action);

      if selected_block == ActiveLidarrBlock::EditArtistPathInput
        || selected_block == ActiveLidarrBlock::EditArtistTagsInput
      {
        assert!(app.ignore_special_keys_for_textbox_input);
      }
    }

    #[rstest]
    fn test_edit_artist_prompt_selected_block_submit_no_op_when_not_ready(
      #[values(1, 2, 3, 4)] y_index: usize,
    ) {
      let mut app = App::test_default();
      app.is_loading = true;
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal::default());
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(
        (
          ActiveLidarrBlock::EditArtistPrompt,
          Some(ActiveLidarrBlock::Artists),
        )
          .into(),
      );
      app.data.lidarr_data.selected_block = BlockSelectionState::new(EDIT_ARTIST_SELECTION_BLOCKS);
      app.data.lidarr_data.selected_block.set_index(0, y_index);

      EditArtistHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::EditArtistPrompt,
        Some(ActiveLidarrBlock::Artists),
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        (
          ActiveLidarrBlock::EditArtistPrompt,
          Some(ActiveLidarrBlock::Artists),
        )
          .into()
      );
      assert_none!(app.data.lidarr_data.prompt_confirm_action);
      assert!(!app.ignore_special_keys_for_textbox_input);
    }

    #[rstest]
    fn test_edit_artist_prompt_selecting_preferences_blocks_submit(
      #[values(
        ActiveLidarrBlock::EditArtistSelectMonitorNewItems,
        ActiveLidarrBlock::EditArtistSelectQualityProfile,
        ActiveLidarrBlock::EditArtistSelectMetadataProfile,
        ActiveLidarrBlock::EditArtistPathInput,
        ActiveLidarrBlock::EditArtistTagsInput
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default();
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal::default());
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::EditArtistPrompt.into());
      app.push_navigation_stack(active_lidarr_block.into());

      EditArtistHandler::new(
        SUBMIT_KEY,
        &mut app,
        active_lidarr_block,
        Some(ActiveLidarrBlock::Artists),
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::EditArtistPrompt.into());

      if active_lidarr_block == ActiveLidarrBlock::EditArtistPathInput
        || active_lidarr_block == ActiveLidarrBlock::EditArtistTagsInput
      {
        assert!(!app.ignore_special_keys_for_textbox_input);
      }
    }
  }

  mod test_handle_esc {
    use crate::assert_navigation_popped;
    use crate::models::servarr_data::lidarr::modals::EditArtistModal;
    use rstest::rstest;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_edit_artist_input_esc(
      #[values(
        ActiveLidarrBlock::EditArtistTagsInput,
        ActiveLidarrBlock::EditArtistPathInput
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default();
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal::default());
      app.ignore_special_keys_for_textbox_input = true;
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::EditArtistPrompt.into());
      app.push_navigation_stack(active_lidarr_block.into());

      EditArtistHandler::new(ESC_KEY, &mut app, active_lidarr_block, None).handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert_navigation_popped!(app, ActiveLidarrBlock::EditArtistPrompt.into());
    }

    #[test]
    fn test_edit_artist_prompt_esc() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::EditArtistPrompt.into());
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal::default());

      EditArtistHandler::new(ESC_KEY, &mut app, ActiveLidarrBlock::EditArtistPrompt, None).handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::Artists.into());

      assert_modal_absent!(app.data.lidarr_data.edit_artist_modal);
      assert!(!app.data.lidarr_data.prompt_confirm);
    }

    #[rstest]
    fn test_edit_artist_esc(
      #[values(
        ActiveLidarrBlock::EditArtistSelectMonitorNewItems,
        ActiveLidarrBlock::EditArtistSelectQualityProfile,
        ActiveLidarrBlock::EditArtistSelectMetadataProfile
      )]
      active_lidarr_block: ActiveLidarrBlock,
      #[values(true, false)] is_ready: bool,
    ) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal::default());
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(active_lidarr_block.into());

      EditArtistHandler::new(ESC_KEY, &mut app, active_lidarr_block, None).handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::Artists.into());
    }
  }

  mod test_handle_key_char {
    use super::*;
    use crate::{
      assert_navigation_popped,
      models::{
        BlockSelectionState,
        servarr_data::lidarr::{
          lidarr_data::EDIT_ARTIST_SELECTION_BLOCKS, modals::EditArtistModal,
        },
      },
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn test_edit_artist_path_input_backspace() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal {
        path: "Test".into(),
        ..EditArtistModal::default()
      });

      EditArtistHandler::new(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveLidarrBlock::EditArtistPathInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .edit_artist_modal
          .as_ref()
          .unwrap()
          .path
          .text,
        "Tes"
      );
    }

    #[test]
    fn test_edit_artist_tags_input_backspace() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal {
        tags: "Test".into(),
        ..EditArtistModal::default()
      });

      EditArtistHandler::new(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveLidarrBlock::EditArtistTagsInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .edit_artist_modal
          .as_ref()
          .unwrap()
          .tags
          .text,
        "Tes"
      );
    }

    #[test]
    fn test_edit_artist_path_input_char_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal::default());

      EditArtistHandler::new(
        Key::Char('a'),
        &mut app,
        ActiveLidarrBlock::EditArtistPathInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .edit_artist_modal
          .as_ref()
          .unwrap()
          .path
          .text,
        "a"
      );
    }

    #[test]
    fn test_edit_artist_tags_input_char_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal::default());

      EditArtistHandler::new(
        Key::Char('a'),
        &mut app,
        ActiveLidarrBlock::EditArtistTagsInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .edit_artist_modal
          .as_ref()
          .unwrap()
          .tags
          .text,
        "a"
      );
    }

    #[test]
    fn test_edit_artist_confirm_prompt_prompt_confirm() {
      let mut app = App::test_default();
      let mut edit_artist = EditArtistModal {
        tags: "usenet, testing".to_owned().into(),
        path: "/nfs/Test Path".to_owned().into(),
        monitored: Some(false),
        ..EditArtistModal::default()
      };
      edit_artist
        .quality_profile_list
        .set_items(vec!["Lossless".to_owned(), "HD - 1080p".to_owned()]);
      edit_artist
        .metadata_profile_list
        .set_items(vec!["Standard".to_owned(), "Full".to_owned()]);
      edit_artist
        .monitor_list
        .set_items(Vec::from_iter(NewItemMonitorType::iter()));
      app.data.lidarr_data.edit_artist_modal = Some(edit_artist);
      app.data.lidarr_data.artists.set_items(vec![Artist {
        monitored: false,
        ..Artist::default()
      }]);
      app.data.lidarr_data.quality_profile_map = BiMap::from_iter([
        (1111, "Lossless".to_owned()),
        (2222, "HD - 1080p".to_owned()),
      ]);
      app.data.lidarr_data.metadata_profile_map =
        BiMap::from_iter([(1111, "Standard".to_owned()), (2222, "Full".to_owned())]);
      let expected_edit_artist_params = EditArtistParams {
        artist_id: 0,
        monitored: Some(false),
        monitor_new_items: Some(NewItemMonitorType::All),
        quality_profile_id: Some(1111),
        metadata_profile_id: Some(1111),
        root_folder_path: Some("/nfs/Test Path".to_owned()),
        tag_input_string: Some("usenet, testing".to_owned()),
        ..EditArtistParams::default()
      };
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::EditArtistPrompt.into());
      app.data.lidarr_data.selected_block = BlockSelectionState::new(EDIT_ARTIST_SELECTION_BLOCKS);
      app
        .data
        .lidarr_data
        .selected_block
        .set_index(0, EDIT_ARTIST_SELECTION_BLOCKS.len() - 1);

      EditArtistHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveLidarrBlock::EditArtistPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::Artists.into());
      assert_eq!(
        app.data.lidarr_data.prompt_confirm_action,
        Some(LidarrEvent::EditArtist(expected_edit_artist_params))
      );
      assert_modal_absent!(app.data.lidarr_data.edit_artist_modal);
      assert!(app.should_refresh);
    }
  }

  #[test]
  fn test_edit_artist_handler_accepts() {
    ActiveLidarrBlock::iter().for_each(|active_lidarr_block| {
      if EDIT_ARTIST_BLOCKS.contains(&active_lidarr_block) {
        assert!(EditArtistHandler::accepts(active_lidarr_block));
      } else {
        assert!(!EditArtistHandler::accepts(active_lidarr_block));
      }
    });
  }

  #[rstest]
  fn test_edit_artist_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = EditArtistHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::default(),
      None,
    );

    assert_eq!(
      handler.ignore_special_keys(),
      ignore_special_keys_for_textbox_input
    );
  }

  #[test]
  fn test_build_edit_artist_params() {
    let mut app = App::test_default();
    let mut edit_artist = EditArtistModal {
      tags: "usenet, testing".to_owned().into(),
      path: "/nfs/Test Path".to_owned().into(),
      monitored: Some(false),
      ..EditArtistModal::default()
    };
    edit_artist
      .quality_profile_list
      .set_items(vec!["Lossless".to_owned(), "HD - 1080p".to_owned()]);
    edit_artist
      .metadata_profile_list
      .set_items(vec!["Standard".to_owned(), "Full".to_owned()]);
    edit_artist
      .monitor_list
      .set_items(Vec::from_iter(NewItemMonitorType::iter()));
    app.data.lidarr_data.edit_artist_modal = Some(edit_artist);
    app.data.lidarr_data.artists.set_items(vec![Artist {
      monitored: false,
      ..Artist::default()
    }]);
    app.data.lidarr_data.quality_profile_map = BiMap::from_iter([
      (1111, "Lossless".to_owned()),
      (2222, "HD - 1080p".to_owned()),
    ]);
    app.data.lidarr_data.metadata_profile_map =
      BiMap::from_iter([(1111, "Standard".to_owned()), (2222, "Full".to_owned())]);
    let expected_edit_artist_params = EditArtistParams {
      artist_id: 0,
      monitored: Some(false),
      monitor_new_items: Some(NewItemMonitorType::All),
      quality_profile_id: Some(1111),
      metadata_profile_id: Some(1111),
      root_folder_path: Some("/nfs/Test Path".to_owned()),
      tag_input_string: Some("usenet, testing".to_owned()),
      ..EditArtistParams::default()
    };

    let edit_artist_params = EditArtistHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::EditArtistPrompt,
      None,
    )
    .build_edit_artist_params();

    assert_eq!(edit_artist_params, expected_edit_artist_params);
    assert_modal_absent!(app.data.lidarr_data.edit_artist_modal);
  }

  #[test]
  fn test_edit_artist_handler_is_not_ready_when_loading() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
    app.is_loading = true;

    let handler = EditArtistHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::EditArtistPrompt,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_edit_artist_handler_is_not_ready_when_edit_artist_modal_is_none() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
    app.is_loading = false;

    let handler = EditArtistHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::EditArtistPrompt,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_edit_artist_handler_is_ready_when_edit_artist_modal_is_some() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
    app.is_loading = false;
    app.data.lidarr_data.edit_artist_modal = Some(EditArtistModal::default());

    let handler = EditArtistHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::EditArtistPrompt,
      None,
    );

    assert!(handler.is_ready());
  }
}
