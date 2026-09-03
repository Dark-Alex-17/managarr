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
  use crate::handlers::readarr_handlers::library::edit_author_handler::EditAuthorHandler;
  use crate::models::readarr_models::{Author, EditAuthorParams, NewItemMonitorType};
  use crate::models::servarr_data::readarr::modals::EditAuthorModal;
  use crate::models::servarr_data::readarr::readarr_data::{
    ActiveReadarrBlock, EDIT_AUTHOR_BLOCKS,
  };
  use crate::network::readarr_network::ReadarrEvent;

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use strum::IntoEnumIterator;

    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::readarr::readarr_data::EDIT_AUTHOR_SELECTION_BLOCKS;

    use super::*;

    #[rstest]
    fn test_edit_author_select_monitor_new_items_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let monitor_type_vec = Vec::from_iter(NewItemMonitorType::iter());
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.edit_author_modal = Some(EditAuthorModal::default());
      app
        .data
        .readarr_data
        .edit_author_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .set_items(monitor_type_vec.clone());

      if key == Key::Up {
        for i in (0..monitor_type_vec.len()).rev() {
          EditAuthorHandler::new(
            key,
            &mut app,
            ActiveReadarrBlock::EditAuthorSelectMonitorNewItems,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .readarr_data
              .edit_author_modal
              .as_ref()
              .unwrap()
              .monitor_list
              .current_selection(),
            &monitor_type_vec[i]
          );
        }
      } else {
        for i in 0..monitor_type_vec.len() {
          EditAuthorHandler::new(
            key,
            &mut app,
            ActiveReadarrBlock::EditAuthorSelectMonitorNewItems,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .readarr_data
              .edit_author_modal
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
    fn test_edit_author_select_quality_profile_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.edit_author_modal = Some(EditAuthorModal::default());
      app
        .data
        .readarr_data
        .edit_author_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .set_items(vec!["Test 1".to_owned(), "Test 2".to_owned()]);

      EditAuthorHandler::new(
        key,
        &mut app,
        ActiveReadarrBlock::EditAuthorSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .edit_author_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 2"
      );

      EditAuthorHandler::new(
        key,
        &mut app,
        ActiveReadarrBlock::EditAuthorSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .edit_author_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[rstest]
    fn test_edit_author_select_metadata_profile_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.edit_author_modal = Some(EditAuthorModal::default());
      app
        .data
        .readarr_data
        .edit_author_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .set_items(vec!["Test 1".to_owned(), "Test 2".to_owned()]);

      EditAuthorHandler::new(
        key,
        &mut app,
        ActiveReadarrBlock::EditAuthorSelectMetadataProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .edit_author_modal
          .as_ref()
          .unwrap()
          .metadata_profile_list
          .current_selection(),
        "Test 2"
      );

      EditAuthorHandler::new(
        key,
        &mut app,
        ActiveReadarrBlock::EditAuthorSelectMetadataProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .edit_author_modal
          .as_ref()
          .unwrap()
          .metadata_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[rstest]
    fn test_edit_author_prompt_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.edit_author_modal = Some(EditAuthorModal::default());
      app.data.readarr_data.selected_block = BlockSelectionState::new(EDIT_AUTHOR_SELECTION_BLOCKS);
      app.data.readarr_data.selected_block.down();

      EditAuthorHandler::new(key, &mut app, ActiveReadarrBlock::EditAuthorPrompt, None).handle();

      if key == Key::Up {
        assert_eq!(
          app.data.readarr_data.selected_block.get_active_block(),
          ActiveReadarrBlock::EditAuthorToggleMonitored
        );
      } else {
        assert_eq!(
          app.data.readarr_data.selected_block.get_active_block(),
          ActiveReadarrBlock::EditAuthorSelectQualityProfile
        );
      }
    }

    #[rstest]
    fn test_edit_author_prompt_scroll_no_op_when_not_ready(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.is_loading = true;
      app.data.readarr_data.edit_author_modal = Some(EditAuthorModal::default());
      app.data.readarr_data.selected_block = BlockSelectionState::new(EDIT_AUTHOR_SELECTION_BLOCKS);
      app.data.readarr_data.selected_block.down();

      EditAuthorHandler::new(key, &mut app, ActiveReadarrBlock::EditAuthorPrompt, None).handle();

      assert_eq!(
        app.data.readarr_data.selected_block.get_active_block(),
        ActiveReadarrBlock::EditAuthorSelectMonitorNewItems
      );
    }
  }

  mod test_handle_home_end {
    use pretty_assertions::assert_eq;
    use std::sync::atomic::Ordering;

    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn test_edit_author_select_monitor_new_items_home_end() {
      let monitor_type_vec = Vec::from_iter(NewItemMonitorType::iter());
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.edit_author_modal = Some(EditAuthorModal::default());
      app
        .data
        .readarr_data
        .edit_author_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .set_items(monitor_type_vec.clone());

      EditAuthorHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveReadarrBlock::EditAuthorSelectMonitorNewItems,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
          .edit_author_modal
          .as_ref()
          .unwrap()
          .monitor_list
          .current_selection(),
        &monitor_type_vec[monitor_type_vec.len() - 1]
      );

      EditAuthorHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveReadarrBlock::EditAuthorSelectMonitorNewItems,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
          .edit_author_modal
          .as_ref()
          .unwrap()
          .monitor_list
          .current_selection(),
        &monitor_type_vec[0]
      );
    }

    #[test]
    fn test_edit_author_select_quality_profile_home_end() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.edit_author_modal = Some(EditAuthorModal::default());
      app
        .data
        .readarr_data
        .edit_author_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .set_items(vec![
          "Test 1".to_owned(),
          "Test 2".to_owned(),
          "Test 3".to_owned(),
        ]);

      EditAuthorHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveReadarrBlock::EditAuthorSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .edit_author_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 3"
      );

      EditAuthorHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveReadarrBlock::EditAuthorSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .edit_author_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[test]
    fn test_edit_author_select_metadata_profile_home_end() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.edit_author_modal = Some(EditAuthorModal::default());
      app
        .data
        .readarr_data
        .edit_author_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .set_items(vec![
          "Test 1".to_owned(),
          "Test 2".to_owned(),
          "Test 3".to_owned(),
        ]);

      EditAuthorHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveReadarrBlock::EditAuthorSelectMetadataProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .edit_author_modal
          .as_ref()
          .unwrap()
          .metadata_profile_list
          .current_selection(),
        "Test 3"
      );

      EditAuthorHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveReadarrBlock::EditAuthorSelectMetadataProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .edit_author_modal
          .as_ref()
          .unwrap()
          .metadata_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[test]
    fn test_edit_author_path_input_home_end_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.edit_author_modal = Some(EditAuthorModal {
        path: "Test".into(),
        ..EditAuthorModal::default()
      });

      EditAuthorHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveReadarrBlock::EditAuthorPathInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
          .edit_author_modal
          .as_ref()
          .unwrap()
          .path
          .offset
          .load(Ordering::SeqCst),
        4
      );

      EditAuthorHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveReadarrBlock::EditAuthorPathInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
          .edit_author_modal
          .as_ref()
          .unwrap()
          .path
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }

    #[test]
    fn test_edit_author_tags_input_home_end_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.edit_author_modal = Some(EditAuthorModal {
        tags: "Test".into(),
        ..EditAuthorModal::default()
      });

      EditAuthorHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveReadarrBlock::EditAuthorTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
          .edit_author_modal
          .as_ref()
          .unwrap()
          .tags
          .offset
          .load(Ordering::SeqCst),
        4
      );

      EditAuthorHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveReadarrBlock::EditAuthorTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
          .edit_author_modal
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

    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_left_right_prompt_toggle(#[values(Key::Left, Key::Right)] key: Key) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.edit_author_modal = Some(EditAuthorModal::default());

      EditAuthorHandler::new(key, &mut app, ActiveReadarrBlock::EditAuthorPrompt, None).handle();

      assert!(app.data.readarr_data.prompt_confirm);

      EditAuthorHandler::new(key, &mut app, ActiveReadarrBlock::EditAuthorPrompt, None).handle();

      assert!(!app.data.readarr_data.prompt_confirm);
    }

    #[test]
    fn test_edit_author_path_input_left_right_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.edit_author_modal = Some(EditAuthorModal {
        path: "Test".into(),
        ..EditAuthorModal::default()
      });

      EditAuthorHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveReadarrBlock::EditAuthorPathInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
          .edit_author_modal
          .as_ref()
          .unwrap()
          .path
          .offset
          .load(Ordering::SeqCst),
        1
      );

      EditAuthorHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveReadarrBlock::EditAuthorPathInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
          .edit_author_modal
          .as_ref()
          .unwrap()
          .path
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }

    #[test]
    fn test_edit_author_tags_input_left_right_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.edit_author_modal = Some(EditAuthorModal {
        tags: "Test".into(),
        ..EditAuthorModal::default()
      });

      EditAuthorHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveReadarrBlock::EditAuthorTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
          .edit_author_modal
          .as_ref()
          .unwrap()
          .tags
          .offset
          .load(Ordering::SeqCst),
        1
      );

      EditAuthorHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveReadarrBlock::EditAuthorTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
          .edit_author_modal
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
    use crate::models::servarr_data::readarr::readarr_data::EDIT_AUTHOR_SELECTION_BLOCKS;
    use crate::models::{BlockSelectionState, Route};
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_edit_author_path_input_submit() {
      let mut app = App::test_default();
      app.ignore_special_keys_for_textbox_input = true;
      app.data.readarr_data.edit_author_modal = Some(EditAuthorModal {
        path: "Test Path".into(),
        ..EditAuthorModal::default()
      });
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(ActiveReadarrBlock::EditAuthorPrompt.into());
      app.push_navigation_stack(ActiveReadarrBlock::EditAuthorPathInput.into());

      EditAuthorHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::EditAuthorPathInput,
        None,
      )
      .handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert!(
        !app
          .data
          .readarr_data
          .edit_author_modal
          .as_ref()
          .unwrap()
          .path
          .text
          .is_empty()
      );
      assert_navigation_popped!(app, ActiveReadarrBlock::EditAuthorPrompt.into());
    }

    #[test]
    fn test_edit_author_tags_input_submit() {
      let mut app = App::test_default();
      app.ignore_special_keys_for_textbox_input = true;
      app.data.readarr_data.edit_author_modal = Some(EditAuthorModal {
        tags: "Test Tags".into(),
        ..EditAuthorModal::default()
      });
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(ActiveReadarrBlock::EditAuthorPrompt.into());
      app.push_navigation_stack(ActiveReadarrBlock::EditAuthorTagsInput.into());

      EditAuthorHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::EditAuthorTagsInput,
        None,
      )
      .handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert!(
        !app
          .data
          .readarr_data
          .edit_author_modal
          .as_mut()
          .unwrap()
          .tags
          .text
          .is_empty()
      );
      assert_navigation_popped!(app, ActiveReadarrBlock::EditAuthorPrompt.into());
    }

    #[test]
    fn test_edit_author_prompt_prompt_decline_submit() {
      let mut app = App::test_default();
      app.data.readarr_data.edit_author_modal = Some(EditAuthorModal::default());
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(ActiveReadarrBlock::EditAuthorPrompt.into());
      app.data.readarr_data.selected_block = BlockSelectionState::new(EDIT_AUTHOR_SELECTION_BLOCKS);
      app
        .data
        .readarr_data
        .selected_block
        .set_index(0, EDIT_AUTHOR_SELECTION_BLOCKS.len() - 1);

      EditAuthorHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::EditAuthorPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::Authors.into());
      assert_none!(app.data.readarr_data.prompt_confirm_action);
    }

    #[test]
    fn test_edit_author_confirm_prompt_prompt_confirmation_submit() {
      let mut app = app_with_populated_edit_author_modal();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(ActiveReadarrBlock::EditAuthorPrompt.into());
      app.data.readarr_data.prompt_confirm = true;
      app.data.readarr_data.selected_block = BlockSelectionState::new(EDIT_AUTHOR_SELECTION_BLOCKS);
      app
        .data
        .readarr_data
        .selected_block
        .set_index(0, EDIT_AUTHOR_SELECTION_BLOCKS.len() - 1);

      EditAuthorHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::EditAuthorPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::Authors.into());
      assert_eq!(
        app.data.readarr_data.prompt_confirm_action,
        Some(ReadarrEvent::EditAuthor(expected_edit_author_params()))
      );
      assert_modal_absent!(app.data.readarr_data.edit_author_modal);
      assert!(app.should_refresh);
    }

    #[test]
    fn test_edit_author_confirm_prompt_prompt_confirmation_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.data.readarr_data.edit_author_modal = Some(EditAuthorModal::default());
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(ActiveReadarrBlock::EditAuthorPrompt.into());
      app.data.readarr_data.prompt_confirm = true;

      EditAuthorHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::EditAuthorPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveReadarrBlock::EditAuthorPrompt.into()
      );
      assert_none!(app.data.readarr_data.prompt_confirm_action);
      assert!(!app.should_refresh);
    }

    #[test]
    fn test_edit_author_toggle_monitored_submit() {
      let current_route = Route::from((
        ActiveReadarrBlock::EditAuthorPrompt,
        Some(ActiveReadarrBlock::AuthorDetails),
      ));
      let mut app = App::test_default();
      app.data.readarr_data.edit_author_modal = Some(EditAuthorModal::default());
      app.data.readarr_data.selected_block = BlockSelectionState::new(EDIT_AUTHOR_SELECTION_BLOCKS);
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(current_route);

      EditAuthorHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::EditAuthorPrompt,
        Some(ActiveReadarrBlock::AuthorDetails),
      )
      .handle();

      assert_eq!(app.get_current_route(), current_route);
      assert_some_eq_x!(
        app
          .data
          .readarr_data
          .edit_author_modal
          .as_ref()
          .unwrap()
          .monitored,
        true
      );

      EditAuthorHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::EditAuthorPrompt,
        Some(ActiveReadarrBlock::AuthorDetails),
      )
      .handle();

      assert_eq!(app.get_current_route(), current_route);
      assert_some_eq_x!(
        app
          .data
          .readarr_data
          .edit_author_modal
          .as_ref()
          .unwrap()
          .monitored,
        false
      );
    }

    #[rstest]
    #[case(ActiveReadarrBlock::EditAuthorSelectMonitorNewItems, 1)]
    #[case(ActiveReadarrBlock::EditAuthorSelectQualityProfile, 2)]
    #[case(ActiveReadarrBlock::EditAuthorSelectMetadataProfile, 3)]
    #[case(ActiveReadarrBlock::EditAuthorPathInput, 4)]
    #[case(ActiveReadarrBlock::EditAuthorTagsInput, 5)]
    fn test_edit_author_prompt_selected_block_submit(
      #[case] selected_block: ActiveReadarrBlock,
      #[case] y_index: usize,
    ) {
      let mut app = App::test_default();
      app.data.readarr_data.edit_author_modal = Some(EditAuthorModal::default());
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(
        (
          ActiveReadarrBlock::EditAuthorPrompt,
          Some(ActiveReadarrBlock::AuthorDetails),
        )
          .into(),
      );
      app.data.readarr_data.selected_block = BlockSelectionState::new(EDIT_AUTHOR_SELECTION_BLOCKS);
      app.data.readarr_data.selected_block.set_index(0, y_index);

      EditAuthorHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::EditAuthorPrompt,
        Some(ActiveReadarrBlock::AuthorDetails),
      )
      .handle();

      assert_navigation_pushed!(
        app,
        (selected_block, Some(ActiveReadarrBlock::AuthorDetails)).into()
      );
      assert_none!(app.data.readarr_data.prompt_confirm_action);

      if selected_block == ActiveReadarrBlock::EditAuthorPathInput
        || selected_block == ActiveReadarrBlock::EditAuthorTagsInput
      {
        assert!(app.ignore_special_keys_for_textbox_input);
      }
    }

    #[rstest]
    fn test_edit_author_prompt_selected_block_submit_no_op_when_not_ready(
      #[values(1, 2, 3, 4)] y_index: usize,
    ) {
      let mut app = App::test_default();
      app.is_loading = true;
      app.data.readarr_data.edit_author_modal = Some(EditAuthorModal::default());
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(
        (
          ActiveReadarrBlock::EditAuthorPrompt,
          Some(ActiveReadarrBlock::AuthorDetails),
        )
          .into(),
      );
      app.data.readarr_data.selected_block = BlockSelectionState::new(EDIT_AUTHOR_SELECTION_BLOCKS);
      app.data.readarr_data.selected_block.set_index(0, y_index);

      EditAuthorHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::EditAuthorPrompt,
        Some(ActiveReadarrBlock::AuthorDetails),
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        (
          ActiveReadarrBlock::EditAuthorPrompt,
          Some(ActiveReadarrBlock::AuthorDetails),
        )
          .into()
      );
      assert_none!(app.data.readarr_data.prompt_confirm_action);
      assert!(!app.ignore_special_keys_for_textbox_input);
    }

    #[rstest]
    fn test_edit_author_prompt_selecting_preferences_blocks_submit(
      #[values(
        ActiveReadarrBlock::EditAuthorSelectMonitorNewItems,
        ActiveReadarrBlock::EditAuthorSelectQualityProfile,
        ActiveReadarrBlock::EditAuthorSelectMetadataProfile,
        ActiveReadarrBlock::EditAuthorPathInput,
        ActiveReadarrBlock::EditAuthorTagsInput
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default();
      app.data.readarr_data.edit_author_modal = Some(EditAuthorModal::default());
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(ActiveReadarrBlock::EditAuthorPrompt.into());
      app.push_navigation_stack(active_readarr_block.into());

      EditAuthorHandler::new(
        SUBMIT_KEY,
        &mut app,
        active_readarr_block,
        Some(ActiveReadarrBlock::AuthorDetails),
      )
      .handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::EditAuthorPrompt.into());

      if active_readarr_block == ActiveReadarrBlock::EditAuthorPathInput
        || active_readarr_block == ActiveReadarrBlock::EditAuthorTagsInput
      {
        assert!(!app.ignore_special_keys_for_textbox_input);
      }
    }
  }

  mod test_handle_esc {
    use crate::assert_navigation_popped;
    use rstest::rstest;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_edit_author_input_esc(
      #[values(
        ActiveReadarrBlock::EditAuthorTagsInput,
        ActiveReadarrBlock::EditAuthorPathInput
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default();
      app.data.readarr_data.edit_author_modal = Some(EditAuthorModal::default());
      app.ignore_special_keys_for_textbox_input = true;
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(ActiveReadarrBlock::EditAuthorPrompt.into());
      app.push_navigation_stack(active_readarr_block.into());

      EditAuthorHandler::new(ESC_KEY, &mut app, active_readarr_block, None).handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert_navigation_popped!(app, ActiveReadarrBlock::EditAuthorPrompt.into());
    }

    #[test]
    fn test_edit_author_prompt_esc() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(ActiveReadarrBlock::EditAuthorPrompt.into());
      app.data.readarr_data.edit_author_modal = Some(EditAuthorModal::default());

      EditAuthorHandler::new(
        ESC_KEY,
        &mut app,
        ActiveReadarrBlock::EditAuthorPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::Authors.into());

      assert_modal_absent!(app.data.readarr_data.edit_author_modal);
      assert!(!app.data.readarr_data.prompt_confirm);
    }

    #[rstest]
    fn test_edit_author_esc(
      #[values(
        ActiveReadarrBlock::EditAuthorSelectMonitorNewItems,
        ActiveReadarrBlock::EditAuthorSelectQualityProfile,
        ActiveReadarrBlock::EditAuthorSelectMetadataProfile
      )]
      active_readarr_block: ActiveReadarrBlock,
      #[values(true, false)] is_loading: bool,
    ) {
      let mut app = App::test_default();
      app.is_loading = is_loading;
      app.data.readarr_data.edit_author_modal = Some(EditAuthorModal::default());
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(active_readarr_block.into());

      EditAuthorHandler::new(ESC_KEY, &mut app, active_readarr_block, None).handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::Authors.into());
    }
  }

  mod test_handle_key_char {
    use super::*;
    use crate::assert_navigation_popped;
    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::readarr::readarr_data::EDIT_AUTHOR_SELECTION_BLOCKS;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_edit_author_path_input_backspace() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.edit_author_modal = Some(EditAuthorModal {
        path: "Test".into(),
        ..EditAuthorModal::default()
      });

      EditAuthorHandler::new(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveReadarrBlock::EditAuthorPathInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .edit_author_modal
          .as_ref()
          .unwrap()
          .path
          .text,
        "Tes"
      );
    }

    #[test]
    fn test_edit_author_tags_input_backspace() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.edit_author_modal = Some(EditAuthorModal {
        tags: "Test".into(),
        ..EditAuthorModal::default()
      });

      EditAuthorHandler::new(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveReadarrBlock::EditAuthorTagsInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .edit_author_modal
          .as_ref()
          .unwrap()
          .tags
          .text,
        "Tes"
      );
    }

    #[test]
    fn test_edit_author_path_input_char_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.edit_author_modal = Some(EditAuthorModal::default());

      EditAuthorHandler::new(
        Key::Char('a'),
        &mut app,
        ActiveReadarrBlock::EditAuthorPathInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .edit_author_modal
          .as_ref()
          .unwrap()
          .path
          .text,
        "a"
      );
    }

    #[test]
    fn test_edit_author_tags_input_char_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.edit_author_modal = Some(EditAuthorModal::default());

      EditAuthorHandler::new(
        Key::Char('a'),
        &mut app,
        ActiveReadarrBlock::EditAuthorTagsInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .edit_author_modal
          .as_ref()
          .unwrap()
          .tags
          .text,
        "a"
      );
    }

    #[test]
    fn test_edit_author_confirm_prompt_prompt_confirm() {
      let mut app = app_with_populated_edit_author_modal();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(ActiveReadarrBlock::EditAuthorPrompt.into());
      app.data.readarr_data.selected_block = BlockSelectionState::new(EDIT_AUTHOR_SELECTION_BLOCKS);
      app
        .data
        .readarr_data
        .selected_block
        .set_index(0, EDIT_AUTHOR_SELECTION_BLOCKS.len() - 1);

      EditAuthorHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveReadarrBlock::EditAuthorPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::Authors.into());
      assert_eq!(
        app.data.readarr_data.prompt_confirm_action,
        Some(ReadarrEvent::EditAuthor(expected_edit_author_params()))
      );
      assert_modal_absent!(app.data.readarr_data.edit_author_modal);
      assert!(app.should_refresh);
    }
  }

  #[test]
  fn test_edit_author_handler_accepts() {
    ActiveReadarrBlock::iter().for_each(|active_readarr_block| {
      if EDIT_AUTHOR_BLOCKS.contains(&active_readarr_block) {
        assert!(EditAuthorHandler::accepts(active_readarr_block));
      } else {
        assert!(!EditAuthorHandler::accepts(active_readarr_block));
      }
    });
  }

  #[rstest]
  fn test_edit_author_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = EditAuthorHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::default(),
      None,
    );

    assert_eq!(
      handler.ignore_special_keys(),
      ignore_special_keys_for_textbox_input
    );
  }

  #[test]
  fn test_build_edit_author_params() {
    let mut app = app_with_populated_edit_author_modal();

    let edit_author_params = EditAuthorHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::EditAuthorPrompt,
      None,
    )
    .build_edit_author_params();

    assert_eq!(edit_author_params, expected_edit_author_params());
    assert_modal_absent!(app.data.readarr_data.edit_author_modal);
  }

  #[test]
  fn test_edit_author_handler_is_not_ready_when_loading() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
    app.is_loading = true;

    let handler = EditAuthorHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::EditAuthorPrompt,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_edit_author_handler_is_not_ready_when_edit_author_modal_is_none() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
    app.is_loading = false;

    let handler = EditAuthorHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::EditAuthorPrompt,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_edit_author_handler_is_ready_when_edit_author_modal_is_some() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
    app.is_loading = false;
    app.data.readarr_data.edit_author_modal = Some(EditAuthorModal::default());

    let handler = EditAuthorHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::EditAuthorPrompt,
      None,
    );

    assert!(handler.is_ready());
  }

  fn app_with_populated_edit_author_modal() -> App<'static> {
    let mut app = App::test_default();
    let mut edit_author_modal = EditAuthorModal {
      tags: "usenet, testing".to_owned().into(),
      path: "/nfs/Test Path".to_owned().into(),
      monitored: Some(false),
      ..EditAuthorModal::default()
    };
    edit_author_modal
      .quality_profile_list
      .set_items(vec!["Lossless".to_owned(), "HD - 1080p".to_owned()]);
    edit_author_modal
      .metadata_profile_list
      .set_items(vec!["Standard".to_owned(), "Full".to_owned()]);
    edit_author_modal
      .monitor_list
      .set_items(Vec::from_iter(NewItemMonitorType::iter()));
    app.data.readarr_data.edit_author_modal = Some(edit_author_modal);
    app.data.readarr_data.authors.set_items(vec![Author {
      id: 42,
      monitored: false,
      ..Author::default()
    }]);
    app.data.readarr_data.quality_profile_map = BiMap::from_iter([
      (1111, "Lossless".to_owned()),
      (2222, "HD - 1080p".to_owned()),
    ]);
    app.data.readarr_data.metadata_profile_map =
      BiMap::from_iter([(3333, "Standard".to_owned()), (4444, "Full".to_owned())]);

    app
  }

  fn expected_edit_author_params() -> EditAuthorParams {
    EditAuthorParams {
      author_id: 42,
      monitored: Some(false),
      monitor_new_items: Some(NewItemMonitorType::All),
      quality_profile_id: Some(1111),
      metadata_profile_id: Some(3333),
      root_folder_path: Some("/nfs/Test Path".to_owned()),
      tag_input_string: Some("usenet, testing".to_owned()),
      ..EditAuthorParams::default()
    }
  }
}
