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
  use crate::handlers::lidarr_handlers::root_folders::add_root_folder_handler::AddRootFolderHandler;
  use crate::models::lidarr_models::{AddLidarrRootFolderBody, MonitorType, NewItemMonitorType};
  use crate::models::servarr_data::lidarr::lidarr_data::{
    ADD_ROOT_FOLDER_BLOCKS, ActiveLidarrBlock,
  };
  use crate::models::servarr_data::lidarr::modals::AddRootFolderModal;
  use crate::network::lidarr_network::LidarrEvent;

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use strum::IntoEnumIterator;

    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::lidarr::lidarr_data::ADD_ROOT_FOLDER_SELECTION_BLOCKS;

    use super::*;

    #[rstest]
    fn test_add_root_folder_select_monitor_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let monitor_type_vec = Vec::from_iter(MonitorType::iter());
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal::default());
      app
        .data
        .lidarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .set_items(monitor_type_vec.clone());

      if key == Key::Up {
        for i in (0..monitor_type_vec.len()).rev() {
          AddRootFolderHandler::new(
            key,
            &mut app,
            ActiveLidarrBlock::AddRootFolderSelectMonitor,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .lidarr_data
              .add_root_folder_modal
              .as_ref()
              .unwrap()
              .monitor_list
              .current_selection(),
            &monitor_type_vec[i]
          );
        }
      } else {
        for i in 0..monitor_type_vec.len() {
          AddRootFolderHandler::new(
            key,
            &mut app,
            ActiveLidarrBlock::AddRootFolderSelectMonitor,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .lidarr_data
              .add_root_folder_modal
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
    fn test_add_root_folder_select_monitor_new_items_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let monitor_type_vec = Vec::from_iter(NewItemMonitorType::iter());
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal::default());
      app
        .data
        .lidarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .monitor_new_items_list
        .set_items(monitor_type_vec.clone());

      if key == Key::Up {
        for i in (0..monitor_type_vec.len()).rev() {
          AddRootFolderHandler::new(
            key,
            &mut app,
            ActiveLidarrBlock::AddRootFolderSelectMonitorNewItems,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .lidarr_data
              .add_root_folder_modal
              .as_ref()
              .unwrap()
              .monitor_new_items_list
              .current_selection(),
            &monitor_type_vec[i]
          );
        }
      } else {
        for i in 0..monitor_type_vec.len() {
          AddRootFolderHandler::new(
            key,
            &mut app,
            ActiveLidarrBlock::AddRootFolderSelectMonitorNewItems,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .lidarr_data
              .add_root_folder_modal
              .as_ref()
              .unwrap()
              .monitor_new_items_list
              .current_selection(),
            &monitor_type_vec[(i + 1) % monitor_type_vec.len()]
          );
        }
      }
    }

    #[rstest]
    fn test_add_root_folder_select_quality_profile_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal::default());
      app
        .data
        .lidarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .set_items(vec!["Test 1".to_owned(), "Test 2".to_owned()]);

      AddRootFolderHandler::new(
        key,
        &mut app,
        ActiveLidarrBlock::AddRootFolderSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 2"
      );

      AddRootFolderHandler::new(
        key,
        &mut app,
        ActiveLidarrBlock::AddRootFolderSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[rstest]
    fn test_add_root_folder_select_metadata_profile_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal::default());
      app
        .data
        .lidarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .set_items(vec!["Test 1".to_owned(), "Test 2".to_owned()]);

      AddRootFolderHandler::new(
        key,
        &mut app,
        ActiveLidarrBlock::AddRootFolderSelectMetadataProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .metadata_profile_list
          .current_selection(),
        "Test 2"
      );

      AddRootFolderHandler::new(
        key,
        &mut app,
        ActiveLidarrBlock::AddRootFolderSelectMetadataProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .metadata_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[rstest]
    fn test_add_root_folder_prompt_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal::default());
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(ADD_ROOT_FOLDER_SELECTION_BLOCKS);
      app.data.lidarr_data.selected_block.down();

      AddRootFolderHandler::new(key, &mut app, ActiveLidarrBlock::AddRootFolderPrompt, None)
        .handle();

      if key == Key::Up {
        assert_eq!(
          app.data.lidarr_data.selected_block.get_active_block(),
          ActiveLidarrBlock::AddRootFolderNameInput
        );
      } else {
        assert_eq!(
          app.data.lidarr_data.selected_block.get_active_block(),
          ActiveLidarrBlock::AddRootFolderSelectMonitor
        );
      }
    }

    #[rstest]
    fn test_add_root_folder_prompt_scroll_no_op_when_not_ready(
      #[values(Key::Up, Key::Down)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.is_loading = true;
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal::default());
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(ADD_ROOT_FOLDER_SELECTION_BLOCKS);
      app.data.lidarr_data.selected_block.down();

      AddRootFolderHandler::new(key, &mut app, ActiveLidarrBlock::AddRootFolderPrompt, None)
        .handle();

      assert_eq!(
        app.data.lidarr_data.selected_block.get_active_block(),
        ActiveLidarrBlock::AddRootFolderPathInput
      );
    }
  }

  mod test_handle_home_end {
    use pretty_assertions::assert_eq;
    use std::sync::atomic::Ordering;

    use strum::IntoEnumIterator;

    use crate::models::servarr_data::lidarr::modals::AddRootFolderModal;

    use super::*;

    #[test]
    fn test_add_root_folder_select_monitor_home_end() {
      let monitor_type_vec = Vec::from_iter(MonitorType::iter());
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal::default());
      app
        .data
        .lidarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .set_items(monitor_type_vec.clone());

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveLidarrBlock::AddRootFolderSelectMonitor,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .monitor_list
          .current_selection(),
        &monitor_type_vec[monitor_type_vec.len() - 1]
      );

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveLidarrBlock::AddRootFolderSelectMonitor,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .monitor_list
          .current_selection(),
        &monitor_type_vec[0]
      );
    }

    #[test]
    fn test_add_root_folder_select_monitor_new_items_home_end() {
      let monitor_type_vec = Vec::from_iter(NewItemMonitorType::iter());
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal::default());
      app
        .data
        .lidarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .monitor_new_items_list
        .set_items(monitor_type_vec.clone());

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveLidarrBlock::AddRootFolderSelectMonitorNewItems,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .monitor_new_items_list
          .current_selection(),
        &monitor_type_vec[monitor_type_vec.len() - 1]
      );

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveLidarrBlock::AddRootFolderSelectMonitorNewItems,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .monitor_new_items_list
          .current_selection(),
        &monitor_type_vec[0]
      );
    }

    #[test]
    fn test_add_root_folder_select_quality_profile_home_end() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal::default());
      app
        .data
        .lidarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .set_items(vec![
          "Test 1".to_owned(),
          "Test 2".to_owned(),
          "Test 3".to_owned(),
        ]);

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveLidarrBlock::AddRootFolderSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 3"
      );

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveLidarrBlock::AddRootFolderSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[test]
    fn test_add_root_folder_select_metadata_profile_home_end() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal::default());
      app
        .data
        .lidarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .set_items(vec![
          "Test 1".to_owned(),
          "Test 2".to_owned(),
          "Test 3".to_owned(),
        ]);

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveLidarrBlock::AddRootFolderSelectMetadataProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .metadata_profile_list
          .current_selection(),
        "Test 3"
      );

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveLidarrBlock::AddRootFolderSelectMetadataProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .metadata_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[test]
    fn test_add_root_folder_name_input_home_end_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal {
        name: "Test".into(),
        ..AddRootFolderModal::default()
      });

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveLidarrBlock::AddRootFolderNameInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .name
          .offset
          .load(Ordering::SeqCst),
        4
      );

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveLidarrBlock::AddRootFolderNameInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .name
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }

    #[test]
    fn test_add_root_folder_path_input_home_end_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal {
        path: "Test".into(),
        ..AddRootFolderModal::default()
      });

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveLidarrBlock::AddRootFolderPathInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .path
          .offset
          .load(Ordering::SeqCst),
        4
      );

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveLidarrBlock::AddRootFolderPathInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .path
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }

    #[test]
    fn test_add_root_folder_tags_input_home_end_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal {
        tags: "Test".into(),
        ..AddRootFolderModal::default()
      });

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveLidarrBlock::AddRootFolderTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .tags
          .offset
          .load(Ordering::SeqCst),
        4
      );

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveLidarrBlock::AddRootFolderTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .add_root_folder_modal
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

    use crate::models::servarr_data::lidarr::modals::AddRootFolderModal;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_left_right_prompt_toggle(#[values(Key::Left, Key::Right)] key: Key) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal::default());

      AddRootFolderHandler::new(key, &mut app, ActiveLidarrBlock::AddRootFolderPrompt, None)
        .handle();

      assert!(app.data.lidarr_data.prompt_confirm);

      AddRootFolderHandler::new(key, &mut app, ActiveLidarrBlock::AddRootFolderPrompt, None)
        .handle();

      assert!(!app.data.lidarr_data.prompt_confirm);
    }

    #[test]
    fn test_add_root_folder_name_input_left_right_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal {
        name: "Test".into(),
        ..AddRootFolderModal::default()
      });

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveLidarrBlock::AddRootFolderNameInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .name
          .offset
          .load(Ordering::SeqCst),
        1
      );

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveLidarrBlock::AddRootFolderNameInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .name
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }

    #[test]
    fn test_add_root_folder_path_input_left_right_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal {
        path: "Test".into(),
        ..AddRootFolderModal::default()
      });

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveLidarrBlock::AddRootFolderPathInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .path
          .offset
          .load(Ordering::SeqCst),
        1
      );

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveLidarrBlock::AddRootFolderPathInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .path
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }

    #[test]
    fn test_add_root_folder_tags_input_left_right_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal {
        tags: "Test".into(),
        ..AddRootFolderModal::default()
      });

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveLidarrBlock::AddRootFolderTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .tags
          .offset
          .load(Ordering::SeqCst),
        1
      );

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveLidarrBlock::AddRootFolderTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .add_root_folder_modal
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
    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::lidarr::lidarr_data::ADD_ROOT_FOLDER_SELECTION_BLOCKS;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_add_root_folder_name_input_submit() {
      let mut app = App::test_default();
      app.ignore_special_keys_for_textbox_input = true;
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal {
        name: "Test Name".into(),
        ..AddRootFolderModal::default()
      });
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveLidarrBlock::AddRootFolderPrompt.into());
      app.push_navigation_stack(ActiveLidarrBlock::AddRootFolderNameInput.into());

      AddRootFolderHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::AddRootFolderNameInput,
        None,
      )
      .handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert!(
        !app
          .data
          .lidarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .name
          .text
          .is_empty()
      );
      assert_navigation_popped!(app, ActiveLidarrBlock::AddRootFolderPrompt.into());
    }

    #[test]
    fn test_add_root_folder_path_input_submit() {
      let mut app = App::test_default();
      app.ignore_special_keys_for_textbox_input = true;
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal {
        path: "Test Path".into(),
        ..AddRootFolderModal::default()
      });
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveLidarrBlock::AddRootFolderPrompt.into());
      app.push_navigation_stack(ActiveLidarrBlock::AddRootFolderPathInput.into());

      AddRootFolderHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::AddRootFolderPathInput,
        None,
      )
      .handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert!(
        !app
          .data
          .lidarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .path
          .text
          .is_empty()
      );
      assert_navigation_popped!(app, ActiveLidarrBlock::AddRootFolderPrompt.into());
    }

    #[test]
    fn test_add_root_folder_tags_input_submit() {
      let mut app = App::test_default();
      app.ignore_special_keys_for_textbox_input = true;
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal {
        tags: "Test Tags".into(),
        ..AddRootFolderModal::default()
      });
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveLidarrBlock::AddRootFolderPrompt.into());
      app.push_navigation_stack(ActiveLidarrBlock::AddRootFolderTagsInput.into());

      AddRootFolderHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::AddRootFolderTagsInput,
        None,
      )
      .handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert!(
        !app
          .data
          .lidarr_data
          .add_root_folder_modal
          .as_mut()
          .unwrap()
          .tags
          .text
          .is_empty()
      );
      assert_navigation_popped!(app, ActiveLidarrBlock::AddRootFolderPrompt.into());
    }

    #[test]
    fn test_add_root_folder_prompt_prompt_decline_submit() {
      let mut app = App::test_default();
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal::default());
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveLidarrBlock::AddRootFolderPrompt.into());
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(ADD_ROOT_FOLDER_SELECTION_BLOCKS);
      app
        .data
        .lidarr_data
        .selected_block
        .set_index(0, ADD_ROOT_FOLDER_SELECTION_BLOCKS.len() - 1);

      AddRootFolderHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::AddRootFolderPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::RootFolders.into());
      assert_none!(app.data.lidarr_data.prompt_confirm_action);
    }

    #[test]
    fn test_add_root_folder_confirm_prompt_prompt_confirmation_submit() {
      let mut app = App::test_default();
      let mut add_root_folder = AddRootFolderModal {
        name: "Test Name".to_owned().into(),
        path: "/nfs/Test Path".to_owned().into(),
        tags: "usenet, testing".to_owned().into(),
        ..AddRootFolderModal::default()
      };
      add_root_folder
        .quality_profile_list
        .set_items(vec!["Lossless".to_owned(), "FLAC".to_owned()]);
      add_root_folder
        .metadata_profile_list
        .set_items(vec!["Standard".to_owned(), "Full".to_owned()]);
      add_root_folder
        .monitor_list
        .set_items(Vec::from_iter(MonitorType::iter()));
      add_root_folder
        .monitor_new_items_list
        .set_items(Vec::from_iter(NewItemMonitorType::iter()));
      app.data.lidarr_data.add_root_folder_modal = Some(add_root_folder);
      app.data.lidarr_data.quality_profile_map =
        BiMap::from_iter([(1111, "Lossless".to_owned()), (2222, "FLAC".to_owned())]);
      app.data.lidarr_data.metadata_profile_map =
        BiMap::from_iter([(1111, "Standard".to_owned()), (2222, "Full".to_owned())]);
      let expected_add_root_folder_body = AddLidarrRootFolderBody {
        name: "Test Name".to_owned(),
        path: "/nfs/Test Path".to_owned(),
        default_quality_profile_id: 1111,
        default_metadata_profile_id: 1111,
        default_monitor_option: MonitorType::All,
        default_new_item_monitor_option: NewItemMonitorType::All,
        default_tags: Vec::new(),
        tag_input_string: Some("usenet, testing".to_owned()),
      };
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveLidarrBlock::AddRootFolderPrompt.into());
      app.data.lidarr_data.prompt_confirm = true;
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(ADD_ROOT_FOLDER_SELECTION_BLOCKS);
      app
        .data
        .lidarr_data
        .selected_block
        .set_index(0, ADD_ROOT_FOLDER_SELECTION_BLOCKS.len() - 1);

      AddRootFolderHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::AddRootFolderPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::RootFolders.into());
      assert_eq!(
        app.data.lidarr_data.prompt_confirm_action,
        Some(LidarrEvent::AddRootFolder(expected_add_root_folder_body))
      );
      assert_modal_absent!(app.data.lidarr_data.add_root_folder_modal);
      assert!(app.should_refresh);
    }

    #[test]
    fn test_add_root_folder_confirm_prompt_prompt_confirmation_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal::default());
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveLidarrBlock::AddRootFolderPrompt.into());
      app.data.lidarr_data.prompt_confirm = true;

      AddRootFolderHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::AddRootFolderPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::AddRootFolderPrompt.into()
      );
      assert_none!(app.data.lidarr_data.prompt_confirm_action);
      assert!(!app.should_refresh);
    }

    #[rstest]
    #[case(ActiveLidarrBlock::AddRootFolderNameInput, 0)]
    #[case(ActiveLidarrBlock::AddRootFolderPathInput, 1)]
    #[case(ActiveLidarrBlock::AddRootFolderSelectMonitor, 2)]
    #[case(ActiveLidarrBlock::AddRootFolderSelectMonitorNewItems, 3)]
    #[case(ActiveLidarrBlock::AddRootFolderSelectQualityProfile, 4)]
    #[case(ActiveLidarrBlock::AddRootFolderSelectMetadataProfile, 5)]
    #[case(ActiveLidarrBlock::AddRootFolderTagsInput, 6)]
    fn test_add_root_folder_prompt_selected_block_submit(
      #[case] selected_block: ActiveLidarrBlock,
      #[case] y_index: usize,
    ) {
      let mut app = App::test_default();
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal::default());
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.push_navigation_stack(
        (
          ActiveLidarrBlock::AddRootFolderPrompt,
          Some(ActiveLidarrBlock::RootFolders),
        )
          .into(),
      );
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(ADD_ROOT_FOLDER_SELECTION_BLOCKS);
      app.data.lidarr_data.selected_block.set_index(0, y_index);

      AddRootFolderHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::AddRootFolderPrompt,
        Some(ActiveLidarrBlock::RootFolders),
      )
      .handle();

      assert_navigation_pushed!(
        app,
        (selected_block, Some(ActiveLidarrBlock::RootFolders)).into()
      );
      assert_none!(app.data.lidarr_data.prompt_confirm_action);

      if selected_block == ActiveLidarrBlock::AddRootFolderNameInput
        || selected_block == ActiveLidarrBlock::AddRootFolderPathInput
        || selected_block == ActiveLidarrBlock::AddRootFolderTagsInput
      {
        assert!(app.ignore_special_keys_for_textbox_input);
      }
    }

    #[rstest]
    fn test_add_root_folder_prompt_selected_block_submit_no_op_when_not_ready(
      #[values(0, 1, 2, 3, 4, 5, 6)] y_index: usize,
    ) {
      let mut app = App::test_default();
      app.is_loading = true;
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal::default());
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.push_navigation_stack(
        (
          ActiveLidarrBlock::AddRootFolderPrompt,
          Some(ActiveLidarrBlock::RootFolders),
        )
          .into(),
      );
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(ADD_ROOT_FOLDER_SELECTION_BLOCKS);
      app.data.lidarr_data.selected_block.set_index(0, y_index);

      AddRootFolderHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::AddRootFolderPrompt,
        Some(ActiveLidarrBlock::RootFolders),
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        (
          ActiveLidarrBlock::AddRootFolderPrompt,
          Some(ActiveLidarrBlock::RootFolders),
        )
          .into()
      );
      assert_none!(app.data.lidarr_data.prompt_confirm_action);
      assert!(!app.ignore_special_keys_for_textbox_input);
    }

    #[rstest]
    fn test_add_root_folder_prompt_selecting_preferences_blocks_submit(
      #[values(
        ActiveLidarrBlock::AddRootFolderSelectMonitor,
        ActiveLidarrBlock::AddRootFolderSelectMonitorNewItems,
        ActiveLidarrBlock::AddRootFolderSelectQualityProfile,
        ActiveLidarrBlock::AddRootFolderSelectMetadataProfile,
        ActiveLidarrBlock::AddRootFolderNameInput,
        ActiveLidarrBlock::AddRootFolderPathInput,
        ActiveLidarrBlock::AddRootFolderTagsInput
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default();
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal::default());
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveLidarrBlock::AddRootFolderPrompt.into());
      app.push_navigation_stack(active_lidarr_block.into());

      AddRootFolderHandler::new(
        SUBMIT_KEY,
        &mut app,
        active_lidarr_block,
        Some(ActiveLidarrBlock::RootFolders),
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::AddRootFolderPrompt.into());

      if active_lidarr_block == ActiveLidarrBlock::AddRootFolderNameInput
        || active_lidarr_block == ActiveLidarrBlock::AddRootFolderPathInput
        || active_lidarr_block == ActiveLidarrBlock::AddRootFolderTagsInput
      {
        assert!(!app.ignore_special_keys_for_textbox_input);
      }
    }
  }

  mod test_handle_esc {
    use crate::assert_navigation_popped;
    use crate::models::servarr_data::lidarr::modals::AddRootFolderModal;
    use rstest::rstest;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_add_root_folder_input_esc(
      #[values(
        ActiveLidarrBlock::AddRootFolderTagsInput,
        ActiveLidarrBlock::AddRootFolderPathInput,
        ActiveLidarrBlock::AddRootFolderNameInput
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default();
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal::default());
      app.ignore_special_keys_for_textbox_input = true;
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveLidarrBlock::AddRootFolderPrompt.into());
      app.push_navigation_stack(active_lidarr_block.into());

      AddRootFolderHandler::new(ESC_KEY, &mut app, active_lidarr_block, None).handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert_navigation_popped!(app, ActiveLidarrBlock::AddRootFolderPrompt.into());
    }

    #[test]
    fn test_add_root_folder_prompt_esc() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveLidarrBlock::AddRootFolderPrompt.into());
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal::default());

      AddRootFolderHandler::new(
        ESC_KEY,
        &mut app,
        ActiveLidarrBlock::AddRootFolderPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::RootFolders.into());

      assert_modal_absent!(app.data.lidarr_data.add_root_folder_modal);
      assert!(!app.data.lidarr_data.prompt_confirm);
    }

    #[rstest]
    fn test_add_root_folder_esc(
      #[values(
        ActiveLidarrBlock::AddRootFolderSelectMonitor,
        ActiveLidarrBlock::AddRootFolderSelectMonitorNewItems,
        ActiveLidarrBlock::AddRootFolderSelectQualityProfile,
        ActiveLidarrBlock::AddRootFolderSelectMetadataProfile
      )]
      active_lidarr_block: ActiveLidarrBlock,
      #[values(true, false)] is_ready: bool,
    ) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal::default());
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.push_navigation_stack(active_lidarr_block.into());

      AddRootFolderHandler::new(ESC_KEY, &mut app, active_lidarr_block, None).handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::RootFolders.into());
    }
  }

  mod test_handle_key_char {
    use super::*;
    use crate::{
      assert_navigation_popped,
      models::{
        BlockSelectionState,
        servarr_data::lidarr::{
          lidarr_data::ADD_ROOT_FOLDER_SELECTION_BLOCKS, modals::AddRootFolderModal,
        },
      },
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn test_add_root_folder_name_input_backspace() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal {
        name: "Test".into(),
        ..AddRootFolderModal::default()
      });

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveLidarrBlock::AddRootFolderNameInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .name
          .text,
        "Tes"
      );
    }

    #[test]
    fn test_add_root_folder_path_input_backspace() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal {
        path: "Test".into(),
        ..AddRootFolderModal::default()
      });

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveLidarrBlock::AddRootFolderPathInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .path
          .text,
        "Tes"
      );
    }

    #[test]
    fn test_add_root_folder_tags_input_backspace() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal {
        tags: "Test".into(),
        ..AddRootFolderModal::default()
      });

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveLidarrBlock::AddRootFolderTagsInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .tags
          .text,
        "Tes"
      );
    }

    #[test]
    fn test_add_root_folder_name_input_char_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal::default());

      AddRootFolderHandler::new(
        Key::Char('a'),
        &mut app,
        ActiveLidarrBlock::AddRootFolderNameInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .name
          .text,
        "a"
      );
    }

    #[test]
    fn test_add_root_folder_path_input_char_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal::default());

      AddRootFolderHandler::new(
        Key::Char('a'),
        &mut app,
        ActiveLidarrBlock::AddRootFolderPathInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .path
          .text,
        "a"
      );
    }

    #[test]
    fn test_add_root_folder_tags_input_char_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal::default());

      AddRootFolderHandler::new(
        Key::Char('a'),
        &mut app,
        ActiveLidarrBlock::AddRootFolderTagsInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .tags
          .text,
        "a"
      );
    }

    #[test]
    fn test_add_root_folder_confirm_prompt_prompt_confirm() {
      let mut app = App::test_default();
      let mut add_root_folder = AddRootFolderModal {
        name: "Test Name".to_owned().into(),
        path: "/nfs/Test Path".to_owned().into(),
        tags: "usenet, testing".to_owned().into(),
        ..AddRootFolderModal::default()
      };
      add_root_folder
        .quality_profile_list
        .set_items(vec!["Lossless".to_owned(), "FLAC".to_owned()]);
      add_root_folder
        .metadata_profile_list
        .set_items(vec!["Standard".to_owned(), "Full".to_owned()]);
      add_root_folder
        .monitor_list
        .set_items(Vec::from_iter(MonitorType::iter()));
      add_root_folder
        .monitor_new_items_list
        .set_items(Vec::from_iter(NewItemMonitorType::iter()));
      app.data.lidarr_data.add_root_folder_modal = Some(add_root_folder);
      app.data.lidarr_data.quality_profile_map =
        BiMap::from_iter([(1111, "Lossless".to_owned()), (2222, "FLAC".to_owned())]);
      app.data.lidarr_data.metadata_profile_map =
        BiMap::from_iter([(1111, "Standard".to_owned()), (2222, "Full".to_owned())]);
      let expected_add_root_folder_body = AddLidarrRootFolderBody {
        name: "Test Name".to_owned(),
        path: "/nfs/Test Path".to_owned(),
        default_quality_profile_id: 1111,
        default_metadata_profile_id: 1111,
        default_monitor_option: MonitorType::All,
        default_new_item_monitor_option: NewItemMonitorType::All,
        default_tags: Vec::new(),
        tag_input_string: Some("usenet, testing".to_owned()),
      };
      app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveLidarrBlock::AddRootFolderPrompt.into());
      app.data.lidarr_data.selected_block =
        BlockSelectionState::new(ADD_ROOT_FOLDER_SELECTION_BLOCKS);
      app
        .data
        .lidarr_data
        .selected_block
        .set_index(0, ADD_ROOT_FOLDER_SELECTION_BLOCKS.len() - 1);

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveLidarrBlock::AddRootFolderPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::RootFolders.into());
      assert_eq!(
        app.data.lidarr_data.prompt_confirm_action,
        Some(LidarrEvent::AddRootFolder(expected_add_root_folder_body))
      );
      assert_modal_absent!(app.data.lidarr_data.add_root_folder_modal);
      assert!(app.should_refresh);
    }
  }

  #[test]
  fn test_add_root_folder_handler_accepts() {
    ActiveLidarrBlock::iter().for_each(|active_lidarr_block| {
      if ADD_ROOT_FOLDER_BLOCKS.contains(&active_lidarr_block) {
        assert!(AddRootFolderHandler::accepts(active_lidarr_block));
      } else {
        assert!(!AddRootFolderHandler::accepts(active_lidarr_block));
      }
    });
  }

  #[rstest]
  fn test_add_root_folder_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = AddRootFolderHandler::new(
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
  fn test_build_add_root_folder_body() {
    let mut app = App::test_default();
    let mut add_root_folder = AddRootFolderModal {
      name: "Test Name".to_owned().into(),
      path: "/nfs/Test Path".to_owned().into(),
      tags: "usenet, testing".to_owned().into(),
      ..AddRootFolderModal::default()
    };
    add_root_folder
      .quality_profile_list
      .set_items(vec!["Lossless".to_owned(), "FLAC".to_owned()]);
    add_root_folder
      .metadata_profile_list
      .set_items(vec!["Standard".to_owned(), "Full".to_owned()]);
    add_root_folder
      .monitor_list
      .set_items(Vec::from_iter(MonitorType::iter()));
    add_root_folder
      .monitor_new_items_list
      .set_items(Vec::from_iter(NewItemMonitorType::iter()));
    app.data.lidarr_data.add_root_folder_modal = Some(add_root_folder);
    app.data.lidarr_data.quality_profile_map =
      BiMap::from_iter([(1111, "Lossless".to_owned()), (2222, "FLAC".to_owned())]);
    app.data.lidarr_data.metadata_profile_map =
      BiMap::from_iter([(1111, "Standard".to_owned()), (2222, "Full".to_owned())]);
    let expected_add_root_folder_body = AddLidarrRootFolderBody {
      name: "Test Name".to_owned(),
      path: "/nfs/Test Path".to_owned(),
      default_quality_profile_id: 1111,
      default_metadata_profile_id: 1111,
      default_monitor_option: MonitorType::All,
      default_new_item_monitor_option: NewItemMonitorType::All,
      default_tags: Vec::new(),
      tag_input_string: Some("usenet, testing".to_owned()),
    };

    let add_root_folder_body = AddRootFolderHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::AddRootFolderPrompt,
      None,
    )
    .build_add_root_folder_body();

    assert_eq!(add_root_folder_body, expected_add_root_folder_body);
    assert_modal_absent!(app.data.lidarr_data.add_root_folder_modal);
  }

  #[test]
  fn test_add_root_folder_handler_is_not_ready_when_loading() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
    app.is_loading = true;

    let handler = AddRootFolderHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::AddRootFolderPrompt,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_add_root_folder_handler_is_not_ready_when_add_root_folder_modal_is_none() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
    app.is_loading = false;

    let handler = AddRootFolderHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::AddRootFolderPrompt,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_add_root_folder_handler_is_ready_when_add_root_folder_modal_is_some() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::RootFolders.into());
    app.is_loading = false;
    app.data.lidarr_data.add_root_folder_modal = Some(AddRootFolderModal::default());

    let handler = AddRootFolderHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::AddRootFolderPrompt,
      None,
    );

    assert!(handler.is_ready());
  }
}
