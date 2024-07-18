#[cfg(test)]
mod tests {
  use pretty_assertions::assert_str_eq;
  use strum::IntoEnumIterator;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::radarr_handlers::collections::edit_collection_handler::EditCollectionHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::radarr_models::MinimumAvailability;
  use crate::models::servarr_data::radarr::modals::EditCollectionModal;
  use crate::models::servarr_data::radarr::radarr_data::{
    ActiveRadarrBlock, EDIT_COLLECTION_BLOCKS,
  };

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use strum::IntoEnumIterator;

    use crate::models::servarr_data::radarr::modals::EditCollectionModal;
    use crate::models::servarr_data::radarr::radarr_data::EDIT_COLLECTION_SELECTION_BLOCKS;
    use crate::models::BlockSelectionState;

    use super::*;

    #[rstest]
    fn test_edit_collection_select_minimum_availability_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let minimum_availability_vec = Vec::from_iter(MinimumAvailability::iter());
      let mut app = App::default();
      app.data.radarr_data.edit_collection_modal = Some(EditCollectionModal::default());
      app
        .data
        .radarr_data
        .edit_collection_modal
        .as_mut()
        .unwrap()
        .minimum_availability_list
        .set_items(minimum_availability_vec.clone());

      if key == Key::Up {
        for i in (0..minimum_availability_vec.len()).rev() {
          EditCollectionHandler::with(
            &key,
            &mut app,
            &ActiveRadarrBlock::EditCollectionSelectMinimumAvailability,
            &None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .radarr_data
              .edit_collection_modal
              .as_ref()
              .unwrap()
              .minimum_availability_list
              .current_selection(),
            &minimum_availability_vec[i]
          );
        }
      } else {
        for i in 0..minimum_availability_vec.len() {
          EditCollectionHandler::with(
            &key,
            &mut app,
            &ActiveRadarrBlock::EditCollectionSelectMinimumAvailability,
            &None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .radarr_data
              .edit_collection_modal
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
    fn test_edit_collection_select_quality_profile_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::default();
      app.data.radarr_data.edit_collection_modal = Some(EditCollectionModal::default());
      app
        .data
        .radarr_data
        .edit_collection_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .set_items(vec!["Test 1".to_owned(), "Test 2".to_owned()]);

      EditCollectionHandler::with(
        &key,
        &mut app,
        &ActiveRadarrBlock::EditCollectionSelectQualityProfile,
        &None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .edit_collection_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 2"
      );

      EditCollectionHandler::with(
        &key,
        &mut app,
        &ActiveRadarrBlock::EditCollectionSelectQualityProfile,
        &None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .edit_collection_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[rstest]
    fn test_edit_collection_prompt_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::default();
      app.data.radarr_data.edit_collection_modal = Some(EditCollectionModal::default());
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(&EDIT_COLLECTION_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.next();

      EditCollectionHandler::with(
        &key,
        &mut app,
        &ActiveRadarrBlock::EditCollectionPrompt,
        &None,
      )
      .handle();

      if key == Key::Up {
        assert_eq!(
          app.data.radarr_data.selected_block.get_active_block(),
          &ActiveRadarrBlock::EditCollectionToggleMonitored
        );
      } else {
        assert_eq!(
          app.data.radarr_data.selected_block.get_active_block(),
          &ActiveRadarrBlock::EditCollectionSelectQualityProfile
        );
      }
    }

    #[rstest]
    fn test_edit_collection_prompt_scroll_no_op_when_not_ready(
      #[values(Key::Up, Key::Down)] key: Key,
    ) {
      let mut app = App::default();
      app.is_loading = true;
      app.data.radarr_data.edit_collection_modal = Some(EditCollectionModal::default());
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(&EDIT_COLLECTION_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.next();

      EditCollectionHandler::with(
        &key,
        &mut app,
        &ActiveRadarrBlock::EditCollectionPrompt,
        &None,
      )
      .handle();

      assert_eq!(
        app.data.radarr_data.selected_block.get_active_block(),
        &ActiveRadarrBlock::EditCollectionSelectMinimumAvailability
      );
    }
  }

  mod test_handle_home_end {
    use pretty_assertions::assert_eq;
    use strum::IntoEnumIterator;

    use crate::models::servarr_data::radarr::modals::EditCollectionModal;

    use super::*;

    #[test]
    fn test_edit_collection_select_minimum_availability_home_end() {
      let minimum_availability_vec = Vec::from_iter(MinimumAvailability::iter());
      let mut app = App::default();
      app.data.radarr_data.edit_collection_modal = Some(EditCollectionModal::default());
      app
        .data
        .radarr_data
        .edit_collection_modal
        .as_mut()
        .unwrap()
        .minimum_availability_list
        .set_items(minimum_availability_vec.clone());

      EditCollectionHandler::with(
        &DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        &ActiveRadarrBlock::EditCollectionSelectMinimumAvailability,
        &None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .edit_collection_modal
          .as_ref()
          .unwrap()
          .minimum_availability_list
          .current_selection(),
        &minimum_availability_vec[minimum_availability_vec.len() - 1]
      );

      EditCollectionHandler::with(
        &DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        &ActiveRadarrBlock::EditCollectionSelectMinimumAvailability,
        &None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .edit_collection_modal
          .as_ref()
          .unwrap()
          .minimum_availability_list
          .current_selection(),
        &minimum_availability_vec[0]
      );
    }

    #[test]
    fn test_edit_collection_select_quality_profile_scroll() {
      let mut app = App::default();
      app.data.radarr_data.edit_collection_modal = Some(EditCollectionModal::default());
      app
        .data
        .radarr_data
        .edit_collection_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .set_items(vec![
          "Test 1".to_owned(),
          "Test 2".to_owned(),
          "Test 3".to_owned(),
        ]);

      EditCollectionHandler::with(
        &DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        &ActiveRadarrBlock::EditCollectionSelectQualityProfile,
        &None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .edit_collection_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 3"
      );

      EditCollectionHandler::with(
        &DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        &ActiveRadarrBlock::EditCollectionSelectQualityProfile,
        &None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .edit_collection_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[test]
    fn test_edit_collection_root_folder_path_input_home_end_keys() {
      let mut app = App::default();
      app.data.radarr_data.edit_collection_modal = Some(EditCollectionModal {
        path: "Test".into(),
        ..EditCollectionModal::default()
      });

      EditCollectionHandler::with(
        &DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        &ActiveRadarrBlock::EditCollectionRootFolderPathInput,
        &None,
      )
      .handle();

      assert_eq!(
        *app
          .data
          .radarr_data
          .edit_collection_modal
          .as_ref()
          .unwrap()
          .path
          .offset
          .borrow(),
        4
      );

      EditCollectionHandler::with(
        &DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        &ActiveRadarrBlock::EditCollectionRootFolderPathInput,
        &None,
      )
      .handle();

      assert_eq!(
        *app
          .data
          .radarr_data
          .edit_collection_modal
          .as_ref()
          .unwrap()
          .path
          .offset
          .borrow(),
        0
      );
    }
  }

  mod test_handle_left_right_action {
    use crate::models::servarr_data::radarr::modals::EditCollectionModal;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_left_right_prompt_toggle(#[values(Key::Left, Key::Right)] key: Key) {
      let mut app = App::default();

      EditCollectionHandler::with(
        &key,
        &mut app,
        &ActiveRadarrBlock::EditCollectionPrompt,
        &None,
      )
      .handle();

      assert!(app.data.radarr_data.prompt_confirm);

      EditCollectionHandler::with(
        &key,
        &mut app,
        &ActiveRadarrBlock::EditCollectionPrompt,
        &None,
      )
      .handle();

      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[test]
    fn test_edit_collection_root_folder_path_input_left_right_keys() {
      let mut app = App::default();
      app.data.radarr_data.edit_collection_modal = Some(EditCollectionModal {
        path: "Test".into(),
        ..EditCollectionModal::default()
      });

      EditCollectionHandler::with(
        &DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        &ActiveRadarrBlock::EditCollectionRootFolderPathInput,
        &None,
      )
      .handle();

      assert_eq!(
        *app
          .data
          .radarr_data
          .edit_collection_modal
          .as_ref()
          .unwrap()
          .path
          .offset
          .borrow(),
        1
      );

      EditCollectionHandler::with(
        &DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        &ActiveRadarrBlock::EditCollectionRootFolderPathInput,
        &None,
      )
      .handle();

      assert_eq!(
        *app
          .data
          .radarr_data
          .edit_collection_modal
          .as_ref()
          .unwrap()
          .path
          .offset
          .borrow(),
        0
      );
    }
  }

  mod test_handle_submit {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::models::servarr_data::radarr::modals::EditCollectionModal;
    use crate::models::servarr_data::radarr::radarr_data::EDIT_COLLECTION_SELECTION_BLOCKS;
    use crate::models::{BlockSelectionState, Route};
    use crate::network::radarr_network::RadarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_edit_collection_root_folder_path_input_submit() {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.data.radarr_data.edit_collection_modal = Some(EditCollectionModal {
        path: "Test Path".into(),
        ..EditCollectionModal::default()
      });
      app.push_navigation_stack(ActiveRadarrBlock::EditCollectionPrompt.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditCollectionRootFolderPathInput.into());

      EditCollectionHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::EditCollectionRootFolderPathInput,
        &None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert!(!app
        .data
        .radarr_data
        .edit_collection_modal
        .as_ref()
        .unwrap()
        .path
        .text
        .is_empty());
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::EditCollectionPrompt.into()
      );
    }

    #[test]
    fn test_edit_collection_prompt_prompt_decline_submit() {
      let mut app = App::default();
      app.data.radarr_data.edit_collection_modal = Some(EditCollectionModal::default());
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditCollectionPrompt.into());
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(&EDIT_COLLECTION_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(EDIT_COLLECTION_SELECTION_BLOCKS.len() - 1);

      EditCollectionHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::EditCollectionPrompt,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Collections.into()
      );
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
    }

    #[test]
    fn test_edit_collection_confirm_prompt_prompt_confirmation_submit() {
      let mut app = App::default();
      app.data.radarr_data.edit_collection_modal = Some(EditCollectionModal::default());
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditCollectionPrompt.into());
      app.data.radarr_data.prompt_confirm = true;
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(&EDIT_COLLECTION_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(EDIT_COLLECTION_SELECTION_BLOCKS.len() - 1);

      EditCollectionHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::EditCollectionPrompt,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Collections.into()
      );
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::EditCollection)
      );
      assert!(app.should_refresh);
    }

    #[test]
    fn test_edit_collection_confirm_prompt_prompt_confirmation_submit_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.data.radarr_data.edit_collection_modal = Some(EditCollectionModal::default());
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditCollectionPrompt.into());
      app.data.radarr_data.prompt_confirm = true;
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(&EDIT_COLLECTION_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(EDIT_COLLECTION_SELECTION_BLOCKS.len() - 1);

      EditCollectionHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::EditCollectionPrompt,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::EditCollectionPrompt.into()
      );
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
      assert!(!app.should_refresh);
    }

    #[test]
    fn test_edit_collection_toggle_monitored_submit() {
      let current_route = Route::from((
        ActiveRadarrBlock::EditCollectionPrompt,
        Some(ActiveRadarrBlock::Collections),
      ));
      let mut app = App::default();
      app.data.radarr_data.edit_collection_modal = Some(EditCollectionModal::default());
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(&EDIT_COLLECTION_SELECTION_BLOCKS);
      app.push_navigation_stack(current_route);

      EditCollectionHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::EditCollectionPrompt,
        &Some(ActiveRadarrBlock::Collections),
      )
      .handle();

      assert_eq!(app.get_current_route(), &current_route);
      assert_eq!(
        app
          .data
          .radarr_data
          .edit_collection_modal
          .as_ref()
          .unwrap()
          .monitored,
        Some(true)
      );

      EditCollectionHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::EditCollectionPrompt,
        &Some(ActiveRadarrBlock::Collections),
      )
      .handle();

      assert_eq!(app.get_current_route(), &current_route);
      assert_eq!(
        app
          .data
          .radarr_data
          .edit_collection_modal
          .as_ref()
          .unwrap()
          .monitored,
        Some(false)
      );
    }

    #[test]
    fn test_edit_collection_toggle_search_on_add_submit() {
      let current_route = Route::from((
        ActiveRadarrBlock::EditCollectionPrompt,
        Some(ActiveRadarrBlock::Collections),
      ));
      let mut app = App::default();
      app.data.radarr_data.edit_collection_modal = Some(EditCollectionModal::default());
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(&EDIT_COLLECTION_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(EDIT_COLLECTION_SELECTION_BLOCKS.len() - 2);
      app.push_navigation_stack(current_route);

      EditCollectionHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::EditCollectionPrompt,
        &Some(ActiveRadarrBlock::Collections),
      )
      .handle();

      assert_eq!(app.get_current_route(), &current_route);
      assert_eq!(
        app
          .data
          .radarr_data
          .edit_collection_modal
          .as_ref()
          .unwrap()
          .search_on_add,
        Some(true)
      );

      EditCollectionHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::EditCollectionPrompt,
        &Some(ActiveRadarrBlock::Collections),
      )
      .handle();

      assert_eq!(app.get_current_route(), &current_route);
      assert_eq!(
        app
          .data
          .radarr_data
          .edit_collection_modal
          .as_ref()
          .unwrap()
          .search_on_add,
        Some(false)
      );
    }

    #[rstest]
    #[case(ActiveRadarrBlock::EditCollectionSelectMinimumAvailability, 1)]
    #[case(ActiveRadarrBlock::EditCollectionSelectQualityProfile, 2)]
    #[case(ActiveRadarrBlock::EditCollectionRootFolderPathInput, 3)]
    fn test_edit_collection_prompt_selected_block_submit(
      #[case] selected_block: ActiveRadarrBlock,
      #[case] index: usize,
    ) {
      let mut app = App::default();
      app.data.radarr_data.edit_collection_modal = Some(EditCollectionModal::default());
      app.push_navigation_stack(
        (
          ActiveRadarrBlock::EditCollectionPrompt,
          Some(ActiveRadarrBlock::Collections),
        )
          .into(),
      );
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(&EDIT_COLLECTION_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.set_index(index);

      EditCollectionHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::EditCollectionPrompt,
        &Some(ActiveRadarrBlock::Collections),
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &(selected_block, Some(ActiveRadarrBlock::Collections)).into()
      );
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);

      if selected_block == ActiveRadarrBlock::EditCollectionRootFolderPathInput {
        assert!(app.should_ignore_quit_key);
      }
    }

    #[rstest]
    fn test_edit_collection_prompt_selecting_preferences_blocks_submit(
      #[values(
        ActiveRadarrBlock::EditCollectionSelectMinimumAvailability,
        ActiveRadarrBlock::EditCollectionSelectQualityProfile,
        ActiveRadarrBlock::EditCollectionRootFolderPathInput
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.data.radarr_data.edit_collection_modal = Some(EditCollectionModal::default());
      app.push_navigation_stack(ActiveRadarrBlock::EditCollectionPrompt.into());
      app.push_navigation_stack(active_radarr_block.into());

      EditCollectionHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &active_radarr_block,
        &Some(ActiveRadarrBlock::Collections),
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::EditCollectionPrompt.into()
      );

      if active_radarr_block == ActiveRadarrBlock::EditCollectionRootFolderPathInput {
        assert!(!app.should_ignore_quit_key);
      }
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::models::servarr_data::radarr::radarr_data::radarr_test_utils::utils::create_test_radarr_data;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[test]
    fn test_edit_collection_root_folder_path_input_esc() {
      let mut app = App::default();
      app.data.radarr_data = create_test_radarr_data();
      app.data.radarr_data.edit_collection_modal = Some(EditCollectionModal::default());
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveRadarrBlock::EditCollectionPrompt.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditCollectionRootFolderPathInput.into());

      EditCollectionHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::EditCollectionRootFolderPathInput,
        &None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::EditCollectionPrompt.into()
      );
    }

    #[test]
    fn test_edit_collection_prompt_esc() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(ActiveRadarrBlock::EditCollectionPrompt.into());
      app.data.radarr_data = create_test_radarr_data();

      EditCollectionHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::EditCollectionPrompt,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Collections.into()
      );
      let radarr_data = &app.data.radarr_data;

      assert!(radarr_data.edit_collection_modal.is_none());
      assert!(!radarr_data.prompt_confirm);
    }

    #[rstest]
    fn test_edit_collection_esc(
      #[values(
        ActiveRadarrBlock::EditCollectionSelectMinimumAvailability,
        ActiveRadarrBlock::EditCollectionSelectQualityProfile
      )]
      active_radarr_block: ActiveRadarrBlock,
      #[values(true, false)] is_ready: bool,
    ) {
      let mut app = App::default();
      app.is_loading = is_ready;
      app.data.radarr_data = create_test_radarr_data();
      app.push_navigation_stack(ActiveRadarrBlock::Collections.into());
      app.push_navigation_stack(active_radarr_block.into());

      EditCollectionHandler::with(&ESC_KEY, &mut app, &active_radarr_block, &None).handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::Collections.into()
      );
    }
  }

  mod test_handle_key_char {
    use super::*;
    use crate::models::servarr_data::radarr::modals::EditCollectionModal;

    #[test]
    fn test_edit_collection_root_folder_path_input_backspace() {
      let mut app = App::default();
      app.data.radarr_data.edit_collection_modal = Some(EditCollectionModal {
        path: "Test".into(),
        ..EditCollectionModal::default()
      });

      EditCollectionHandler::with(
        &DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        &ActiveRadarrBlock::EditCollectionRootFolderPathInput,
        &None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .edit_collection_modal
          .as_ref()
          .unwrap()
          .path
          .text,
        "Tes"
      );
    }

    #[test]
    fn test_edit_collection_root_folder_path_input_char_key() {
      let mut app = App::default();
      app.data.radarr_data.edit_collection_modal = Some(EditCollectionModal::default());

      EditCollectionHandler::with(
        &Key::Char('h'),
        &mut app,
        &ActiveRadarrBlock::EditCollectionRootFolderPathInput,
        &None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .edit_collection_modal
          .as_ref()
          .unwrap()
          .path
          .text,
        "h"
      );
    }
  }

  #[test]
  fn test_edit_collection_handler_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if EDIT_COLLECTION_BLOCKS.contains(&active_radarr_block) {
        assert!(EditCollectionHandler::accepts(&active_radarr_block));
      } else {
        assert!(!EditCollectionHandler::accepts(&active_radarr_block));
      }
    });
  }

  #[test]
  fn test_edit_collection_handler_is_not_ready_when_loading() {
    let mut app = App::default();
    app.is_loading = true;

    let handler = EditCollectionHandler::with(
      &DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      &ActiveRadarrBlock::EditCollectionPrompt,
      &None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_edit_collection_handler_is_not_ready_when_edit_collection_modal_is_none() {
    let mut app = App::default();
    app.is_loading = false;

    let handler = EditCollectionHandler::with(
      &DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      &ActiveRadarrBlock::EditCollectionPrompt,
      &None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_edit_collection_handler_is_ready_when_edit_collection_modal_is_some() {
    let mut app = App::default();
    app.is_loading = false;
    app.data.radarr_data.edit_collection_modal = Some(EditCollectionModal::default());

    let handler = EditCollectionHandler::with(
      &DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      &ActiveRadarrBlock::EditCollectionPrompt,
      &None,
    );

    assert!(handler.is_ready());
  }
}
