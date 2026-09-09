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
  use crate::handlers::readarr_handlers::root_folders::add_root_folder_handler::AddRootFolderHandler;
  use crate::models::readarr_models::{AddReadarrRootFolderBody, MonitorType, NewItemMonitorType};
  use crate::models::servarr_data::readarr::modals::AddReadarrRootFolderModal;
  use crate::models::servarr_data::readarr::readarr_data::{
    ADD_ROOT_FOLDER_BLOCKS, ActiveReadarrBlock,
  };
  use crate::network::readarr_network::ReadarrEvent;

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use strum::IntoEnumIterator;

    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::readarr::readarr_data::ADD_ROOT_FOLDER_SELECTION_BLOCKS;

    use super::*;

    #[rstest]
    fn test_add_root_folder_select_monitor_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let monitor_type_vec = Vec::from_iter(MonitorType::iter());
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal::default());
      app
        .data
        .readarr_data
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
            ActiveReadarrBlock::AddRootFolderSelectMonitor,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .readarr_data
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
            ActiveReadarrBlock::AddRootFolderSelectMonitor,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .readarr_data
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
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal::default());
      app
        .data
        .readarr_data
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
            ActiveReadarrBlock::AddRootFolderSelectMonitorNewItems,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .readarr_data
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
            ActiveReadarrBlock::AddRootFolderSelectMonitorNewItems,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .readarr_data
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
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal::default());
      app
        .data
        .readarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .set_items(vec!["EPUB".to_owned(), "MOBI".to_owned()]);

      AddRootFolderHandler::new(
        key,
        &mut app,
        ActiveReadarrBlock::AddRootFolderSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "MOBI"
      );

      AddRootFolderHandler::new(
        key,
        &mut app,
        ActiveReadarrBlock::AddRootFolderSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "EPUB"
      );
    }

    #[rstest]
    fn test_add_root_folder_select_metadata_profile_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal::default());
      app
        .data
        .readarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .set_items(vec!["Standard".to_owned(), "Comprehensive".to_owned()]);

      AddRootFolderHandler::new(
        key,
        &mut app,
        ActiveReadarrBlock::AddRootFolderSelectMetadataProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .metadata_profile_list
          .current_selection(),
        "Comprehensive"
      );

      AddRootFolderHandler::new(
        key,
        &mut app,
        ActiveReadarrBlock::AddRootFolderSelectMetadataProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .metadata_profile_list
          .current_selection(),
        "Standard"
      );
    }

    #[rstest]
    fn test_add_root_folder_prompt_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal::default());
      app.data.readarr_data.selected_block =
        BlockSelectionState::new(ADD_ROOT_FOLDER_SELECTION_BLOCKS);
      app.data.readarr_data.selected_block.down();

      AddRootFolderHandler::new(key, &mut app, ActiveReadarrBlock::AddRootFolderPrompt, None)
        .handle();

      if key == Key::Up {
        assert_eq!(
          app.data.readarr_data.selected_block.get_active_block(),
          ActiveReadarrBlock::AddRootFolderNameInput
        );
      } else {
        assert_eq!(
          app.data.readarr_data.selected_block.get_active_block(),
          ActiveReadarrBlock::AddRootFolderSelectMonitor
        );
      }
    }

    #[rstest]
    fn test_add_root_folder_prompt_scroll_no_op_when_not_ready(
      #[values(Key::Up, Key::Down)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.is_loading = true;
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal::default());
      app.data.readarr_data.selected_block =
        BlockSelectionState::new(ADD_ROOT_FOLDER_SELECTION_BLOCKS);
      app.data.readarr_data.selected_block.down();

      AddRootFolderHandler::new(key, &mut app, ActiveReadarrBlock::AddRootFolderPrompt, None)
        .handle();

      assert_eq!(
        app.data.readarr_data.selected_block.get_active_block(),
        ActiveReadarrBlock::AddRootFolderPathInput
      );
    }
  }

  mod test_handle_home_end {
    use pretty_assertions::assert_eq;
    use std::sync::atomic::Ordering;
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn test_add_root_folder_select_monitor_home_end() {
      let monitor_type_vec = Vec::from_iter(MonitorType::iter());
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal::default());
      app
        .data
        .readarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .set_items(monitor_type_vec.clone());

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveReadarrBlock::AddRootFolderSelectMonitor,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
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
        ActiveReadarrBlock::AddRootFolderSelectMonitor,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
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
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal::default());
      app
        .data
        .readarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .monitor_new_items_list
        .set_items(monitor_type_vec.clone());

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveReadarrBlock::AddRootFolderSelectMonitorNewItems,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
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
        ActiveReadarrBlock::AddRootFolderSelectMonitorNewItems,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
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
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal::default());
      app
        .data
        .readarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .set_items(vec![
          "EPUB".to_owned(),
          "MOBI".to_owned(),
          "AZW3".to_owned(),
        ]);

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveReadarrBlock::AddRootFolderSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "AZW3"
      );

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveReadarrBlock::AddRootFolderSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "EPUB"
      );
    }

    #[test]
    fn test_add_root_folder_select_metadata_profile_home_end() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal::default());
      app
        .data
        .readarr_data
        .add_root_folder_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .set_items(vec![
          "Standard".to_owned(),
          "Comprehensive".to_owned(),
          "Minimal".to_owned(),
        ]);

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveReadarrBlock::AddRootFolderSelectMetadataProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .metadata_profile_list
          .current_selection(),
        "Minimal"
      );

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveReadarrBlock::AddRootFolderSelectMetadataProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .metadata_profile_list
          .current_selection(),
        "Standard"
      );
    }

    #[test]
    fn test_add_root_folder_name_input_home_end_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal {
        name: "Test".into(),
        ..AddReadarrRootFolderModal::default()
      });

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveReadarrBlock::AddRootFolderNameInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
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
        ActiveReadarrBlock::AddRootFolderNameInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
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
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal {
        path: "Testing".into(),
        ..AddReadarrRootFolderModal::default()
      });

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveReadarrBlock::AddRootFolderPathInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .path
          .offset
          .load(Ordering::SeqCst),
        7
      );

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveReadarrBlock::AddRootFolderPathInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
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
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal {
        tags: "Testing tags".into(),
        ..AddReadarrRootFolderModal::default()
      });

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveReadarrBlock::AddRootFolderTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .tags
          .offset
          .load(Ordering::SeqCst),
        12
      );

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveReadarrBlock::AddRootFolderTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
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
    use rstest::rstest;
    use std::sync::atomic::Ordering;

    use super::*;

    #[rstest]
    fn test_left_right_prompt_toggle(#[values(Key::Left, Key::Right)] key: Key) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal::default());

      AddRootFolderHandler::new(key, &mut app, ActiveReadarrBlock::AddRootFolderPrompt, None)
        .handle();

      assert!(app.data.readarr_data.prompt_confirm);

      AddRootFolderHandler::new(key, &mut app, ActiveReadarrBlock::AddRootFolderPrompt, None)
        .handle();

      assert!(!app.data.readarr_data.prompt_confirm);
    }

    #[test]
    fn test_add_root_folder_name_input_left_right_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal {
        name: "Test".into(),
        ..AddReadarrRootFolderModal::default()
      });

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveReadarrBlock::AddRootFolderNameInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
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
        ActiveReadarrBlock::AddRootFolderNameInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
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
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal {
        path: "Test".into(),
        ..AddReadarrRootFolderModal::default()
      });

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveReadarrBlock::AddRootFolderPathInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
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
        ActiveReadarrBlock::AddRootFolderPathInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
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
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal {
        tags: "Test".into(),
        ..AddReadarrRootFolderModal::default()
      });

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveReadarrBlock::AddRootFolderTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
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
        ActiveReadarrBlock::AddRootFolderTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
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
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::assert_navigation_popped;
    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::readarr::readarr_data::ADD_ROOT_FOLDER_SELECTION_BLOCKS;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_add_root_folder_name_input_submit() {
      let mut app = App::test_default();
      app.ignore_special_keys_for_textbox_input = true;
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal {
        name: "Test Name".into(),
        ..AddReadarrRootFolderModal::default()
      });
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveReadarrBlock::AddRootFolderPrompt.into());
      app.push_navigation_stack(ActiveReadarrBlock::AddRootFolderNameInput.into());

      AddRootFolderHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::AddRootFolderNameInput,
        None,
      )
      .handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert_str_eq!(
        app
          .data
          .readarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .name
          .text,
        "Test Name"
      );
      assert_navigation_popped!(app, ActiveReadarrBlock::AddRootFolderPrompt.into());
    }

    #[test]
    fn test_add_root_folder_path_input_submit() {
      let mut app = App::test_default();
      app.ignore_special_keys_for_textbox_input = true;
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal {
        path: "/nfs/Test Path".into(),
        ..AddReadarrRootFolderModal::default()
      });
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveReadarrBlock::AddRootFolderPrompt.into());
      app.push_navigation_stack(ActiveReadarrBlock::AddRootFolderPathInput.into());

      AddRootFolderHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::AddRootFolderPathInput,
        None,
      )
      .handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert_str_eq!(
        app
          .data
          .readarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .path
          .text,
        "/nfs/Test Path"
      );
      assert_navigation_popped!(app, ActiveReadarrBlock::AddRootFolderPrompt.into());
    }

    #[test]
    fn test_add_root_folder_tags_input_submit() {
      let mut app = App::test_default();
      app.ignore_special_keys_for_textbox_input = true;
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal {
        tags: "usenet, testing".into(),
        ..AddReadarrRootFolderModal::default()
      });
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveReadarrBlock::AddRootFolderPrompt.into());
      app.push_navigation_stack(ActiveReadarrBlock::AddRootFolderTagsInput.into());

      AddRootFolderHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::AddRootFolderTagsInput,
        None,
      )
      .handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert_str_eq!(
        app
          .data
          .readarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .tags
          .text,
        "usenet, testing"
      );
      assert_navigation_popped!(app, ActiveReadarrBlock::AddRootFolderPrompt.into());
    }

    #[test]
    fn test_add_root_folder_prompt_prompt_decline_submit() {
      let mut app = App::test_default();
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal::default());
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveReadarrBlock::AddRootFolderPrompt.into());
      app.data.readarr_data.selected_block =
        BlockSelectionState::new(ADD_ROOT_FOLDER_SELECTION_BLOCKS);
      app
        .data
        .readarr_data
        .selected_block
        .set_index(0, ADD_ROOT_FOLDER_SELECTION_BLOCKS.len() - 1);

      AddRootFolderHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::AddRootFolderPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::RootFolders.into());
      assert_none!(app.data.readarr_data.prompt_confirm_action);
    }

    #[test]
    fn test_add_root_folder_confirm_prompt_prompt_confirmation_submit() {
      let mut app = App::test_default();
      app.data.readarr_data.add_root_folder_modal = Some(populated_add_root_folder_modal());
      seed_distinct_profile_maps(&mut app);
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveReadarrBlock::AddRootFolderPrompt.into());
      app.data.readarr_data.prompt_confirm = true;
      app.data.readarr_data.selected_block =
        BlockSelectionState::new(ADD_ROOT_FOLDER_SELECTION_BLOCKS);
      app
        .data
        .readarr_data
        .selected_block
        .set_index(0, ADD_ROOT_FOLDER_SELECTION_BLOCKS.len() - 1);

      AddRootFolderHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::AddRootFolderPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::RootFolders.into());
      assert_eq!(
        app.data.readarr_data.prompt_confirm_action,
        Some(ReadarrEvent::AddRootFolder(expected_add_root_folder_body()))
      );
      assert_modal_absent!(app.data.readarr_data.add_root_folder_modal);
      assert!(app.should_refresh);
    }

    #[test]
    fn test_add_root_folder_confirm_prompt_prompt_confirmation_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal::default());
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveReadarrBlock::AddRootFolderPrompt.into());
      app.data.readarr_data.prompt_confirm = true;

      AddRootFolderHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::AddRootFolderPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveReadarrBlock::AddRootFolderPrompt.into()
      );
      assert_none!(app.data.readarr_data.prompt_confirm_action);
      assert!(!app.should_refresh);
    }

    #[rstest]
    #[case(ActiveReadarrBlock::AddRootFolderNameInput, 0)]
    #[case(ActiveReadarrBlock::AddRootFolderPathInput, 1)]
    #[case(ActiveReadarrBlock::AddRootFolderSelectMonitor, 2)]
    #[case(ActiveReadarrBlock::AddRootFolderSelectMonitorNewItems, 3)]
    #[case(ActiveReadarrBlock::AddRootFolderSelectQualityProfile, 4)]
    #[case(ActiveReadarrBlock::AddRootFolderSelectMetadataProfile, 5)]
    #[case(ActiveReadarrBlock::AddRootFolderTagsInput, 6)]
    fn test_add_root_folder_prompt_selected_block_submit(
      #[case] selected_block: ActiveReadarrBlock,
      #[case] y_index: usize,
    ) {
      let mut app = App::test_default();
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal::default());
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.push_navigation_stack(
        (
          ActiveReadarrBlock::AddRootFolderPrompt,
          Some(ActiveReadarrBlock::RootFolders),
        )
          .into(),
      );
      app.data.readarr_data.selected_block =
        BlockSelectionState::new(ADD_ROOT_FOLDER_SELECTION_BLOCKS);
      app.data.readarr_data.selected_block.set_index(0, y_index);

      AddRootFolderHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::AddRootFolderPrompt,
        Some(ActiveReadarrBlock::RootFolders),
      )
      .handle();

      assert_navigation_pushed!(
        app,
        (selected_block, Some(ActiveReadarrBlock::RootFolders)).into()
      );
      assert_none!(app.data.readarr_data.prompt_confirm_action);

      if selected_block == ActiveReadarrBlock::AddRootFolderNameInput
        || selected_block == ActiveReadarrBlock::AddRootFolderPathInput
        || selected_block == ActiveReadarrBlock::AddRootFolderTagsInput
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
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal::default());
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.push_navigation_stack(
        (
          ActiveReadarrBlock::AddRootFolderPrompt,
          Some(ActiveReadarrBlock::RootFolders),
        )
          .into(),
      );
      app.data.readarr_data.selected_block =
        BlockSelectionState::new(ADD_ROOT_FOLDER_SELECTION_BLOCKS);
      app.data.readarr_data.selected_block.set_index(0, y_index);

      AddRootFolderHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::AddRootFolderPrompt,
        Some(ActiveReadarrBlock::RootFolders),
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        (
          ActiveReadarrBlock::AddRootFolderPrompt,
          Some(ActiveReadarrBlock::RootFolders),
        )
          .into()
      );
      assert_none!(app.data.readarr_data.prompt_confirm_action);
      assert!(!app.ignore_special_keys_for_textbox_input);
    }

    #[rstest]
    fn test_add_root_folder_prompt_selecting_preferences_blocks_submit(
      #[values(
        ActiveReadarrBlock::AddRootFolderSelectMonitor,
        ActiveReadarrBlock::AddRootFolderSelectMonitorNewItems,
        ActiveReadarrBlock::AddRootFolderSelectQualityProfile,
        ActiveReadarrBlock::AddRootFolderSelectMetadataProfile,
        ActiveReadarrBlock::AddRootFolderNameInput,
        ActiveReadarrBlock::AddRootFolderPathInput,
        ActiveReadarrBlock::AddRootFolderTagsInput
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default();
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal::default());
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveReadarrBlock::AddRootFolderPrompt.into());
      app.push_navigation_stack(active_readarr_block.into());

      AddRootFolderHandler::new(
        SUBMIT_KEY,
        &mut app,
        active_readarr_block,
        Some(ActiveReadarrBlock::RootFolders),
      )
      .handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::AddRootFolderPrompt.into());

      if active_readarr_block == ActiveReadarrBlock::AddRootFolderNameInput
        || active_readarr_block == ActiveReadarrBlock::AddRootFolderPathInput
        || active_readarr_block == ActiveReadarrBlock::AddRootFolderTagsInput
      {
        assert!(!app.ignore_special_keys_for_textbox_input);
      }
    }
  }

  mod test_handle_esc {
    use rstest::rstest;

    use crate::assert_navigation_popped;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_add_root_folder_input_esc(
      #[values(
        ActiveReadarrBlock::AddRootFolderTagsInput,
        ActiveReadarrBlock::AddRootFolderPathInput,
        ActiveReadarrBlock::AddRootFolderNameInput
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default();
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal::default());
      app.ignore_special_keys_for_textbox_input = true;
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveReadarrBlock::AddRootFolderPrompt.into());
      app.push_navigation_stack(active_readarr_block.into());

      AddRootFolderHandler::new(ESC_KEY, &mut app, active_readarr_block, None).handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert_navigation_popped!(app, ActiveReadarrBlock::AddRootFolderPrompt.into());
    }

    #[test]
    fn test_add_root_folder_prompt_esc() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveReadarrBlock::AddRootFolderPrompt.into());
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal::default());
      app.data.readarr_data.prompt_confirm = true;

      AddRootFolderHandler::new(
        ESC_KEY,
        &mut app,
        ActiveReadarrBlock::AddRootFolderPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::RootFolders.into());
      assert_modal_absent!(app.data.readarr_data.add_root_folder_modal);
      assert!(!app.data.readarr_data.prompt_confirm);
    }

    #[rstest]
    fn test_add_root_folder_esc(
      #[values(
        ActiveReadarrBlock::AddRootFolderSelectMonitor,
        ActiveReadarrBlock::AddRootFolderSelectMonitorNewItems,
        ActiveReadarrBlock::AddRootFolderSelectQualityProfile,
        ActiveReadarrBlock::AddRootFolderSelectMetadataProfile
      )]
      active_readarr_block: ActiveReadarrBlock,
      #[values(true, false)] is_ready: bool,
    ) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal::default());
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.push_navigation_stack(active_readarr_block.into());

      AddRootFolderHandler::new(ESC_KEY, &mut app, active_readarr_block, None).handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::RootFolders.into());
    }
  }

  mod test_handle_key_char {
    use pretty_assertions::assert_eq;

    use crate::assert_navigation_popped;
    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::readarr::readarr_data::ADD_ROOT_FOLDER_SELECTION_BLOCKS;

    use super::*;

    #[test]
    fn test_add_root_folder_name_input_backspace() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal {
        name: "Test".into(),
        ..AddReadarrRootFolderModal::default()
      });

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveReadarrBlock::AddRootFolderNameInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
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
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal {
        path: "Path".into(),
        ..AddReadarrRootFolderModal::default()
      });

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveReadarrBlock::AddRootFolderPathInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .path
          .text,
        "Pat"
      );
    }

    #[test]
    fn test_add_root_folder_tags_input_backspace() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal {
        tags: "Tags".into(),
        ..AddReadarrRootFolderModal::default()
      });

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveReadarrBlock::AddRootFolderTagsInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .tags
          .text,
        "Tag"
      );
    }

    #[test]
    fn test_add_root_folder_name_input_char_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal::default());

      AddRootFolderHandler::new(
        Key::Char('n'),
        &mut app,
        ActiveReadarrBlock::AddRootFolderNameInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .name
          .text,
        "n"
      );
      assert_is_empty!(
        app
          .data
          .readarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .path
          .text
      );
      assert_is_empty!(
        app
          .data
          .readarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .tags
          .text
      );
    }

    #[test]
    fn test_add_root_folder_path_input_char_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal::default());

      AddRootFolderHandler::new(
        Key::Char('p'),
        &mut app,
        ActiveReadarrBlock::AddRootFolderPathInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .path
          .text,
        "p"
      );
      assert_is_empty!(
        app
          .data
          .readarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .name
          .text
      );
      assert_is_empty!(
        app
          .data
          .readarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .tags
          .text
      );
    }

    #[test]
    fn test_add_root_folder_tags_input_char_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal::default());

      AddRootFolderHandler::new(
        Key::Char('t'),
        &mut app,
        ActiveReadarrBlock::AddRootFolderTagsInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .tags
          .text,
        "t"
      );
      assert_is_empty!(
        app
          .data
          .readarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .name
          .text
      );
      assert_is_empty!(
        app
          .data
          .readarr_data
          .add_root_folder_modal
          .as_ref()
          .unwrap()
          .path
          .text
      );
    }

    #[test]
    fn test_add_root_folder_confirm_prompt_prompt_confirm() {
      let mut app = App::test_default();
      app.data.readarr_data.add_root_folder_modal = Some(populated_add_root_folder_modal());
      seed_distinct_profile_maps(&mut app);
      app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
      app.push_navigation_stack(ActiveReadarrBlock::AddRootFolderPrompt.into());
      app.data.readarr_data.selected_block =
        BlockSelectionState::new(ADD_ROOT_FOLDER_SELECTION_BLOCKS);
      app
        .data
        .readarr_data
        .selected_block
        .set_index(0, ADD_ROOT_FOLDER_SELECTION_BLOCKS.len() - 1);

      AddRootFolderHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveReadarrBlock::AddRootFolderPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::RootFolders.into());
      assert_eq!(
        app.data.readarr_data.prompt_confirm_action,
        Some(ReadarrEvent::AddRootFolder(expected_add_root_folder_body()))
      );
      assert_modal_absent!(app.data.readarr_data.add_root_folder_modal);
      assert!(app.should_refresh);
    }
  }

  #[test]
  fn test_add_root_folder_handler_accepts() {
    ActiveReadarrBlock::iter().for_each(|active_readarr_block| {
      if ADD_ROOT_FOLDER_BLOCKS.contains(&active_readarr_block) {
        assert!(
          AddRootFolderHandler::accepts(active_readarr_block),
          "{active_readarr_block} is not accepted by the AddRootFolderHandler"
        );
      } else {
        assert!(
          !AddRootFolderHandler::accepts(active_readarr_block),
          "{active_readarr_block} is accepted by the AddRootFolderHandler"
        );
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
      ActiveReadarrBlock::default(),
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
    app.data.readarr_data.add_root_folder_modal = Some(populated_add_root_folder_modal());
    seed_distinct_profile_maps(&mut app);

    let add_root_folder_body = AddRootFolderHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::AddRootFolderPrompt,
      None,
    )
    .build_add_root_folder_body();

    assert_eq!(add_root_folder_body, expected_add_root_folder_body());
    assert_modal_absent!(app.data.readarr_data.add_root_folder_modal);
  }

  #[test]
  fn test_build_add_root_folder_body_uses_the_selected_profiles() {
    let mut app = App::test_default();
    app.data.readarr_data.add_root_folder_modal = Some(populated_add_root_folder_modal());
    seed_distinct_profile_maps(&mut app);
    AddRootFolderHandler::new(
      DEFAULT_KEYBINDINGS.down.key,
      &mut app,
      ActiveReadarrBlock::AddRootFolderSelectQualityProfile,
      None,
    )
    .handle();
    AddRootFolderHandler::new(
      DEFAULT_KEYBINDINGS.down.key,
      &mut app,
      ActiveReadarrBlock::AddRootFolderSelectMetadataProfile,
      None,
    )
    .handle();

    let add_root_folder_body = AddRootFolderHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::AddRootFolderPrompt,
      None,
    )
    .build_add_root_folder_body();

    assert_eq!(add_root_folder_body.default_quality_profile_id, 1111);
    assert_eq!(add_root_folder_body.default_metadata_profile_id, 3333);
  }

  #[test]
  fn test_add_root_folder_handler_is_not_ready_when_loading() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
    app.is_loading = true;
    app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal::default());

    let handler = AddRootFolderHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::AddRootFolderPrompt,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_add_root_folder_handler_is_not_ready_when_add_root_folder_modal_is_none() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
    app.is_loading = false;

    let handler = AddRootFolderHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::AddRootFolderPrompt,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_add_root_folder_handler_is_ready_when_add_root_folder_modal_is_some() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::RootFolders.into());
    app.is_loading = false;
    app.data.readarr_data.add_root_folder_modal = Some(AddReadarrRootFolderModal::default());

    let handler = AddRootFolderHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::AddRootFolderPrompt,
      None,
    );

    assert!(handler.is_ready());
  }

  fn populated_add_root_folder_modal() -> AddReadarrRootFolderModal {
    let mut add_root_folder_modal = AddReadarrRootFolderModal {
      name: "Test Name".to_owned().into(),
      path: "/nfs/Test Path".to_owned().into(),
      tags: "usenet, testing".to_owned().into(),
      ..AddReadarrRootFolderModal::default()
    };
    add_root_folder_modal
      .quality_profile_list
      .set_items(vec!["EPUB".to_owned(), "MOBI".to_owned()]);
    add_root_folder_modal
      .quality_profile_list
      .state
      .select(Some(1));
    add_root_folder_modal
      .metadata_profile_list
      .set_items(vec!["Standard".to_owned(), "Comprehensive".to_owned()]);
    add_root_folder_modal
      .metadata_profile_list
      .state
      .select(Some(1));
    add_root_folder_modal
      .monitor_list
      .set_items(Vec::from_iter(MonitorType::iter()));
    add_root_folder_modal.monitor_list.state.select(Some(1));
    add_root_folder_modal
      .monitor_new_items_list
      .set_items(Vec::from_iter(NewItemMonitorType::iter()));
    add_root_folder_modal
      .monitor_new_items_list
      .state
      .select(Some(1));

    add_root_folder_modal
  }

  fn seed_distinct_profile_maps(app: &mut App<'_>) {
    app.data.readarr_data.quality_profile_map =
      BiMap::from_iter([(1111, "EPUB".to_owned()), (2222, "MOBI".to_owned())]);
    app.data.readarr_data.metadata_profile_map = BiMap::from_iter([
      (3333, "Standard".to_owned()),
      (4444, "Comprehensive".to_owned()),
    ]);
  }

  fn expected_add_root_folder_body() -> AddReadarrRootFolderBody {
    AddReadarrRootFolderBody {
      name: "Test Name".to_owned(),
      path: "/nfs/Test Path".to_owned(),
      default_quality_profile_id: 2222,
      default_metadata_profile_id: 4444,
      default_monitor_option: MonitorType::Future,
      default_new_item_monitor_option: NewItemMonitorType::None,
      default_tags: Vec::new(),
      tag_input_string: Some("usenet, testing".to_owned()),
    }
  }
}
