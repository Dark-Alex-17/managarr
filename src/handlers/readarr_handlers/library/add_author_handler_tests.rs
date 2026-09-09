#[cfg(test)]
mod tests {
  use bimap::BiMap;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use std::sync::atomic::Ordering;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::assert_modal_absent;
  use crate::assert_modal_present;
  use crate::assert_navigation_popped;
  use crate::assert_navigation_pushed;
  use crate::event::Key;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::readarr_handlers::library::add_author_handler::AddAuthorHandler;
  use crate::models::readarr_models::{
    AddAuthorBody, AddAuthorOptions, AddAuthorSearchResult, Author, AuthorStatus, MonitorType,
    NewItemMonitorType,
  };
  use crate::models::servarr_data::readarr::modals::AddAuthorModal;
  use crate::models::servarr_data::readarr::readarr_data::{
    ADD_AUTHOR_BLOCKS, ADD_AUTHOR_SELECTION_BLOCKS, ActiveReadarrBlock,
  };
  use crate::models::servarr_models::RootFolder;
  use crate::models::stateful_table::StatefulTable;
  use crate::models::{BlockSelectionState, HorizontallyScrollableText};
  use crate::simple_stateful_iterable_vec;

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use strum::IntoEnumIterator;

    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::readarr::readarr_data::ADD_AUTHOR_SELECTION_BLOCKS;

    use super::*;

    #[rstest]
    fn test_add_author_select_monitor_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let monitor_vec = Vec::from_iter(MonitorType::iter());
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.add_author_modal = Some(AddAuthorModal::default());
      app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .set_items(monitor_vec.clone());

      if key == Key::Up {
        for i in (0..monitor_vec.len()).rev() {
          AddAuthorHandler::new(
            key,
            &mut app,
            ActiveReadarrBlock::AddAuthorSelectMonitor,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .readarr_data
              .add_author_modal
              .as_ref()
              .unwrap()
              .monitor_list
              .current_selection(),
            &monitor_vec[i]
          );
        }
      } else {
        for i in 0..monitor_vec.len() {
          AddAuthorHandler::new(
            key,
            &mut app,
            ActiveReadarrBlock::AddAuthorSelectMonitor,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .readarr_data
              .add_author_modal
              .as_ref()
              .unwrap()
              .monitor_list
              .current_selection(),
            &monitor_vec[(i + 1) % monitor_vec.len()]
          );
        }
      }
    }

    #[rstest]
    fn test_add_author_select_monitor_new_items_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let monitor_new_items_vec = Vec::from_iter(NewItemMonitorType::iter());
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.add_author_modal = Some(AddAuthorModal::default());
      app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .monitor_new_items_list
        .set_items(monitor_new_items_vec.clone());

      if key == Key::Up {
        for i in (0..monitor_new_items_vec.len()).rev() {
          AddAuthorHandler::new(
            key,
            &mut app,
            ActiveReadarrBlock::AddAuthorSelectMonitorNewItems,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .readarr_data
              .add_author_modal
              .as_ref()
              .unwrap()
              .monitor_new_items_list
              .current_selection(),
            &monitor_new_items_vec[i]
          );
        }
      } else {
        for i in 0..monitor_new_items_vec.len() {
          AddAuthorHandler::new(
            key,
            &mut app,
            ActiveReadarrBlock::AddAuthorSelectMonitorNewItems,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .readarr_data
              .add_author_modal
              .as_ref()
              .unwrap()
              .monitor_new_items_list
              .current_selection(),
            &monitor_new_items_vec[(i + 1) % monitor_new_items_vec.len()]
          );
        }
      }
    }

    #[rstest]
    fn test_add_author_select_quality_profile_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.add_author_modal = Some(AddAuthorModal::default());
      app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .set_items(vec!["Test 1".to_owned(), "Test 2".to_owned()]);

      AddAuthorHandler::new(
        key,
        &mut app,
        ActiveReadarrBlock::AddAuthorSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .add_author_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 2"
      );

      AddAuthorHandler::new(
        key,
        &mut app,
        ActiveReadarrBlock::AddAuthorSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .add_author_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[rstest]
    fn test_add_author_select_metadata_profile_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.add_author_modal = Some(AddAuthorModal::default());
      app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .set_items(vec!["Test 1".to_owned(), "Test 2".to_owned()]);

      AddAuthorHandler::new(
        key,
        &mut app,
        ActiveReadarrBlock::AddAuthorSelectMetadataProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .add_author_modal
          .as_ref()
          .unwrap()
          .metadata_profile_list
          .current_selection(),
        "Test 2"
      );

      AddAuthorHandler::new(
        key,
        &mut app,
        ActiveReadarrBlock::AddAuthorSelectMetadataProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .add_author_modal
          .as_ref()
          .unwrap()
          .metadata_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[rstest]
    fn test_add_author_select_root_folder_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.add_author_modal = Some(AddAuthorModal::default());
      app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .root_folder_list
        .set_items(simple_stateful_iterable_vec!(RootFolder, String, path));

      AddAuthorHandler::new(
        key,
        &mut app,
        ActiveReadarrBlock::AddAuthorSelectRootFolder,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .add_author_modal
          .as_ref()
          .unwrap()
          .root_folder_list
          .current_selection()
          .path,
        "Test 2"
      );

      AddAuthorHandler::new(
        key,
        &mut app,
        ActiveReadarrBlock::AddAuthorSelectRootFolder,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .add_author_modal
          .as_ref()
          .unwrap()
          .root_folder_list
          .current_selection()
          .path,
        "Test 1"
      );
    }

    #[rstest]
    fn test_add_author_prompt_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.selected_block = BlockSelectionState::new(ADD_AUTHOR_SELECTION_BLOCKS);
      app.data.readarr_data.selected_block.down();

      AddAuthorHandler::new(key, &mut app, ActiveReadarrBlock::AddAuthorPrompt, None).handle();

      if key == Key::Up {
        assert_eq!(
          app.data.readarr_data.selected_block.get_active_block(),
          ActiveReadarrBlock::AddAuthorSelectRootFolder
        );
      } else {
        assert_eq!(
          app.data.readarr_data.selected_block.get_active_block(),
          ActiveReadarrBlock::AddAuthorSelectMonitorNewItems
        );
      }
    }

    #[rstest]
    fn test_add_author_prompt_scroll_no_op_when_not_ready(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.is_loading = true;
      app.data.readarr_data.selected_block = BlockSelectionState::new(ADD_AUTHOR_SELECTION_BLOCKS);
      app.data.readarr_data.selected_block.down();

      AddAuthorHandler::new(key, &mut app, ActiveReadarrBlock::AddAuthorPrompt, None).handle();

      assert_eq!(
        app.data.readarr_data.selected_block.get_active_block(),
        ActiveReadarrBlock::AddAuthorSelectMonitor
      );
    }
  }

  mod test_handle_home_end {
    use pretty_assertions::assert_eq;

    use crate::extended_stateful_iterable_vec;

    use super::*;

    #[test]
    fn test_add_author_select_monitor_home_end() {
      let monitor_vec = Vec::from_iter(MonitorType::iter());
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.add_author_modal = Some(AddAuthorModal::default());
      app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .set_items(monitor_vec.clone());

      AddAuthorHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveReadarrBlock::AddAuthorSelectMonitor,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
          .add_author_modal
          .as_ref()
          .unwrap()
          .monitor_list
          .current_selection(),
        &monitor_vec[monitor_vec.len() - 1]
      );

      AddAuthorHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveReadarrBlock::AddAuthorSelectMonitor,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
          .add_author_modal
          .as_ref()
          .unwrap()
          .monitor_list
          .current_selection(),
        &monitor_vec[0]
      );
    }

    #[test]
    fn test_add_author_select_monitor_new_items_home_end() {
      let monitor_new_items_vec = Vec::from_iter(NewItemMonitorType::iter());
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.add_author_modal = Some(AddAuthorModal::default());
      app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .monitor_new_items_list
        .set_items(monitor_new_items_vec.clone());

      AddAuthorHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveReadarrBlock::AddAuthorSelectMonitorNewItems,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
          .add_author_modal
          .as_ref()
          .unwrap()
          .monitor_new_items_list
          .current_selection(),
        &monitor_new_items_vec[monitor_new_items_vec.len() - 1]
      );

      AddAuthorHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveReadarrBlock::AddAuthorSelectMonitorNewItems,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
          .add_author_modal
          .as_ref()
          .unwrap()
          .monitor_new_items_list
          .current_selection(),
        &monitor_new_items_vec[0]
      );
    }

    #[test]
    fn test_add_author_select_quality_profile_home_end() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.add_author_modal = Some(AddAuthorModal::default());
      app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .set_items(vec![
          "Test 1".to_owned(),
          "Test 2".to_owned(),
          "Test 3".to_owned(),
        ]);

      AddAuthorHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveReadarrBlock::AddAuthorSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .add_author_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 3"
      );

      AddAuthorHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveReadarrBlock::AddAuthorSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .add_author_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[test]
    fn test_add_author_select_metadata_profile_home_end() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.add_author_modal = Some(AddAuthorModal::default());
      app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .set_items(vec![
          "Test 1".to_owned(),
          "Test 2".to_owned(),
          "Test 3".to_owned(),
        ]);

      AddAuthorHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveReadarrBlock::AddAuthorSelectMetadataProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .add_author_modal
          .as_ref()
          .unwrap()
          .metadata_profile_list
          .current_selection(),
        "Test 3"
      );

      AddAuthorHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveReadarrBlock::AddAuthorSelectMetadataProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .add_author_modal
          .as_ref()
          .unwrap()
          .metadata_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[test]
    fn test_add_author_select_root_folder_home_end() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.add_author_modal = Some(AddAuthorModal::default());
      app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .root_folder_list
        .set_items(extended_stateful_iterable_vec!(RootFolder, String, path));

      AddAuthorHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveReadarrBlock::AddAuthorSelectRootFolder,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .add_author_modal
          .as_ref()
          .unwrap()
          .root_folder_list
          .current_selection()
          .path,
        "Test 3"
      );

      AddAuthorHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveReadarrBlock::AddAuthorSelectRootFolder,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .add_author_modal
          .as_ref()
          .unwrap()
          .root_folder_list
          .current_selection()
          .path,
        "Test 1"
      );
    }

    #[test]
    fn test_add_author_search_input_home_end_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::AddAuthorSearchInput.into());
      app.data.readarr_data.add_author_search = Some("Test".into());

      AddAuthorHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveReadarrBlock::AddAuthorSearchInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
          .add_author_search
          .as_ref()
          .unwrap()
          .offset
          .load(Ordering::SeqCst),
        4
      );

      AddAuthorHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveReadarrBlock::AddAuthorSearchInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
          .add_author_search
          .as_ref()
          .unwrap()
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }

    #[test]
    fn test_add_author_tags_input_home_end_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.add_author_modal = Some(AddAuthorModal {
        tags: "Test".into(),
        ..AddAuthorModal::default()
      });

      AddAuthorHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveReadarrBlock::AddAuthorTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
          .add_author_modal
          .as_ref()
          .unwrap()
          .tags
          .offset
          .load(Ordering::SeqCst),
        4
      );

      AddAuthorHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveReadarrBlock::AddAuthorTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
          .add_author_modal
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

    use super::*;

    #[rstest]
    fn test_left_right_prompt_toggle(#[values(Key::Left, Key::Right)] key: Key) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());

      AddAuthorHandler::new(key, &mut app, ActiveReadarrBlock::AddAuthorPrompt, None).handle();

      assert!(app.data.readarr_data.prompt_confirm);

      AddAuthorHandler::new(key, &mut app, ActiveReadarrBlock::AddAuthorPrompt, None).handle();

      assert!(!app.data.readarr_data.prompt_confirm);
    }

    #[test]
    fn test_add_author_search_input_left_right_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::AddAuthorSearchInput.into());
      app.data.readarr_data.add_author_search = Some("Test".into());

      AddAuthorHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveReadarrBlock::AddAuthorSearchInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
          .add_author_search
          .as_ref()
          .unwrap()
          .offset
          .load(Ordering::SeqCst),
        1
      );

      AddAuthorHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveReadarrBlock::AddAuthorSearchInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
          .add_author_search
          .as_ref()
          .unwrap()
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }

    #[test]
    fn test_add_author_tags_input_left_right_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.add_author_modal = Some(AddAuthorModal {
        tags: "Test".into(),
        ..AddAuthorModal::default()
      });

      AddAuthorHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveReadarrBlock::AddAuthorTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
          .add_author_modal
          .as_ref()
          .unwrap()
          .tags
          .offset
          .load(Ordering::SeqCst),
        1
      );

      AddAuthorHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveReadarrBlock::AddAuthorTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .readarr_data
          .add_author_modal
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
    use super::*;
    use crate::network::readarr_network::ReadarrEvent;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_add_author_search_input_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::AddAuthorSearchInput.into());
      app.ignore_special_keys_for_textbox_input = true;
      app.data.readarr_data.add_author_search = Some("test".into());

      AddAuthorHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::AddAuthorSearchInput,
        None,
      )
      .handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert_navigation_pushed!(app, ActiveReadarrBlock::AddAuthorSearchResults.into());
    }

    #[test]
    fn test_add_author_search_input_submit_noop_on_empty_search() {
      let mut app = App::test_default();
      app.data.readarr_data.add_author_search = Some(HorizontallyScrollableText::default());
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(ActiveReadarrBlock::AddAuthorSearchInput.into());
      app.ignore_special_keys_for_textbox_input = true;

      AddAuthorHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::AddAuthorSearchInput,
        None,
      )
      .handle();

      assert!(app.ignore_special_keys_for_textbox_input);
      assert_eq!(
        app.get_current_route(),
        ActiveReadarrBlock::AddAuthorSearchInput.into()
      );
    }

    #[test]
    fn test_add_author_search_results_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::AddAuthorSearchResults.into());
      let mut add_searched_authors = StatefulTable::default();
      let mut decoy_search_result = add_author_search_result();
      decoy_search_result.foreign_author_id = "decoy-foreign-id".to_owned();
      decoy_search_result.author_name = "Decoy Author".into();
      add_searched_authors.set_items(vec![decoy_search_result, add_author_search_result()]);
      add_searched_authors.select_index(Some(1));
      app.data.readarr_data.add_searched_authors = Some(add_searched_authors);
      app.data.readarr_data.quality_profile_map =
        BiMap::from_iter([(1111, "Any".to_owned()), (2222, "eBook".to_owned())]);
      app.data.readarr_data.metadata_profile_map =
        BiMap::from_iter([(3333, "None".to_owned()), (4444, "Standard".to_owned())]);

      AddAuthorHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::AddAuthorSearchResults,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveReadarrBlock::AddAuthorPrompt.into()
      );
      assert_modal_present!(app.data.readarr_data.add_author_modal);
      assert_eq!(
        app.data.readarr_data.selected_block.get_active_block(),
        ActiveReadarrBlock::AddAuthorSelectRootFolder
      );
    }

    #[test]
    fn test_add_author_search_results_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(ActiveReadarrBlock::AddAuthorSearchResults.into());
      let mut add_searched_authors = StatefulTable::default();
      add_searched_authors.set_items(vec![add_author_search_result()]);
      app.data.readarr_data.add_searched_authors = Some(add_searched_authors);

      AddAuthorHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::AddAuthorSearchResults,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveReadarrBlock::AddAuthorSearchResults.into()
      );
      assert_modal_absent!(app.data.readarr_data.add_author_modal);
    }

    #[test]
    fn test_add_author_search_results_submit_does_nothing_on_empty_table() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(ActiveReadarrBlock::AddAuthorSearchResults.into());

      AddAuthorHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::AddAuthorSearchResults,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveReadarrBlock::AddAuthorSearchResults.into()
      );
    }

    #[test]
    fn test_add_author_search_results_submit_author_already_in_library() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::AddAuthorSearchResults.into());
      let mut add_searched_authors = StatefulTable::default();
      let mut decoy_search_result = add_author_search_result();
      decoy_search_result.foreign_author_id = "decoy-foreign-id".to_owned();
      decoy_search_result.author_name = "Decoy Author".into();
      add_searched_authors.set_items(vec![decoy_search_result, add_author_search_result()]);
      add_searched_authors.select_index(Some(1));
      app.data.readarr_data.add_searched_authors = Some(add_searched_authors);
      app
        .data
        .readarr_data
        .authors
        .set_items(vec![author(SEARCHED_FOREIGN_AUTHOR_ID)]);

      AddAuthorHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::AddAuthorSearchResults,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::AddAuthorAlreadyInLibrary.into());
      assert_modal_absent!(app.data.readarr_data.add_author_modal);
    }

    #[test]
    fn test_add_author_search_results_submit_author_not_in_library() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::AddAuthorSearchResults.into());
      let mut add_searched_authors = StatefulTable::default();
      let mut decoy_search_result = add_author_search_result();
      decoy_search_result.foreign_author_id = OTHER_FOREIGN_AUTHOR_ID.to_owned();
      decoy_search_result.author_name = "Decoy Author".into();
      add_searched_authors.set_items(vec![decoy_search_result, add_author_search_result()]);
      add_searched_authors.select_index(Some(1));
      app.data.readarr_data.add_searched_authors = Some(add_searched_authors);
      app
        .data
        .readarr_data
        .authors
        .set_items(vec![author(OTHER_FOREIGN_AUTHOR_ID)]);

      AddAuthorHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::AddAuthorSearchResults,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::AddAuthorPrompt.into());
      assert_modal_present!(app.data.readarr_data.add_author_modal);
      assert_eq!(
        app.data.readarr_data.selected_block.get_active_block(),
        ActiveReadarrBlock::AddAuthorSelectRootFolder
      );
    }

    #[test]
    fn test_add_author_prompt_prompt_decline_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(ActiveReadarrBlock::AddAuthorPrompt.into());
      app.data.readarr_data.selected_block = BlockSelectionState::new(ADD_AUTHOR_SELECTION_BLOCKS);
      app
        .data
        .readarr_data
        .selected_block
        .set_index(0, ADD_AUTHOR_SELECTION_BLOCKS.len() - 1);

      AddAuthorHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::AddAuthorPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::Authors.into());
      assert_none!(app.data.readarr_data.prompt_confirm_action);
    }

    #[test]
    fn test_add_author_confirm_prompt_prompt_confirm_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::AddAuthorSearchResults.into());
      app.push_navigation_stack(ActiveReadarrBlock::AddAuthorPrompt.into());
      app.data.readarr_data.selected_block = BlockSelectionState::new(ADD_AUTHOR_SELECTION_BLOCKS);
      app
        .data
        .readarr_data
        .selected_block
        .set_index(0, ADD_AUTHOR_SELECTION_BLOCKS.len() - 1);
      app.data.readarr_data.prompt_confirm = true;
      let mut add_searched_authors = StatefulTable::default();
      let mut decoy_search_result = add_author_search_result();
      decoy_search_result.foreign_author_id = "decoy-foreign-id".to_owned();
      decoy_search_result.author_name = "Decoy Author".into();
      add_searched_authors.set_items(vec![decoy_search_result, add_author_search_result()]);
      add_searched_authors.select_index(Some(1));
      app.data.readarr_data.add_searched_authors = Some(add_searched_authors);
      app.data.readarr_data.quality_profile_map =
        BiMap::from_iter([(1111, "Any".to_owned()), (2222, "eBook".to_owned())]);
      app.data.readarr_data.metadata_profile_map =
        BiMap::from_iter([(3333, "None".to_owned()), (4444, "Standard".to_owned())]);
      app.data.readarr_data.add_author_modal = Some(AddAuthorModal {
        tags: "usenet, testing".into(),
        ..AddAuthorModal::default()
      });
      app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .set_items(Vec::from_iter(MonitorType::iter()));
      app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .state
        .select(Some(1));
      app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .monitor_new_items_list
        .set_items(Vec::from_iter(NewItemMonitorType::iter()));
      app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .monitor_new_items_list
        .state
        .select(Some(1));
      app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .set_items(vec!["Any".to_owned(), "eBook".to_owned()]);
      app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .state
        .select(Some(1));
      app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .set_items(vec!["None".to_owned(), "Standard".to_owned()]);
      app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .state
        .select(Some(1));
      app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .root_folder_list
        .set_items(vec![
          RootFolder {
            path: "/decoy".to_owned(),
            ..RootFolder::default()
          },
          RootFolder {
            path: "/books".to_owned(),
            ..RootFolder::default()
          },
        ]);
      app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .root_folder_list
        .state
        .select(Some(1));
      let expected_add_author_body = AddAuthorBody {
        foreign_author_id: SEARCHED_FOREIGN_AUTHOR_ID.to_string(),
        author_name: "Test Author".to_string(),
        monitored: true,
        root_folder_path: "/books".to_string(),
        quality_profile_id: 2222,
        metadata_profile_id: 4444,
        tags: Vec::default(),
        tag_input_string: Some("usenet, testing".to_owned()),
        add_options: AddAuthorOptions {
          monitor: MonitorType::Future,
          monitor_new_items: NewItemMonitorType::None,
          search_for_missing_books: true,
        },
      };
      let expected_readarr_event = ReadarrEvent::AddAuthor(expected_add_author_body);

      AddAuthorHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::AddAuthorPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::AddAuthorSearchResults.into());
      assert_some_eq_x!(
        &app.data.readarr_data.prompt_confirm_action,
        &expected_readarr_event
      );
      assert_modal_absent!(app.data.readarr_data.add_author_modal);
    }

    #[rstest]
    #[case(ActiveReadarrBlock::AddAuthorSelectRootFolder, 0)]
    #[case(ActiveReadarrBlock::AddAuthorSelectMonitor, 1)]
    #[case(ActiveReadarrBlock::AddAuthorSelectMonitorNewItems, 2)]
    #[case(ActiveReadarrBlock::AddAuthorSelectQualityProfile, 3)]
    #[case(ActiveReadarrBlock::AddAuthorSelectMetadataProfile, 4)]
    #[case(ActiveReadarrBlock::AddAuthorTagsInput, 5)]
    fn test_add_author_prompt_selected_block_submit(
      #[case] block: ActiveReadarrBlock,
      #[case] y_index: usize,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::AddAuthorPrompt.into());
      app.data.readarr_data.selected_block = BlockSelectionState::new(ADD_AUTHOR_SELECTION_BLOCKS);
      app.data.readarr_data.selected_block.set_index(0, y_index);

      AddAuthorHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveReadarrBlock::AddAuthorPrompt,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, block.into());
    }

    #[rstest]
    fn test_add_author_prompt_selecting_preferences_blocks_submit(
      #[values(
        ActiveReadarrBlock::AddAuthorSelectRootFolder,
        ActiveReadarrBlock::AddAuthorSelectMonitor,
        ActiveReadarrBlock::AddAuthorSelectMonitorNewItems,
        ActiveReadarrBlock::AddAuthorSelectQualityProfile,
        ActiveReadarrBlock::AddAuthorSelectMetadataProfile,
        ActiveReadarrBlock::AddAuthorTagsInput
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(ActiveReadarrBlock::AddAuthorPrompt.into());
      app.push_navigation_stack(active_readarr_block.into());

      AddAuthorHandler::new(SUBMIT_KEY, &mut app, active_readarr_block, None).handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::AddAuthorPrompt.into());

      if active_readarr_block == ActiveReadarrBlock::AddAuthorTagsInput {
        assert!(!app.ignore_special_keys_for_textbox_input);
      }
    }
  }

  mod test_handle_esc {
    use rstest::rstest;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_add_author_search_input_esc(#[values(true, false)] is_loading: bool) {
      let mut app = App::test_default();
      app.is_loading = is_loading;
      app.data.readarr_data.add_author_search = Some("test".into());
      app.ignore_special_keys_for_textbox_input = true;
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(ActiveReadarrBlock::AddAuthorSearchInput.into());

      AddAuthorHandler::new(
        ESC_KEY,
        &mut app,
        ActiveReadarrBlock::AddAuthorSearchInput,
        None,
      )
      .handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert_navigation_popped!(app, ActiveReadarrBlock::Authors.into());
      assert_modal_absent!(app.data.readarr_data.add_author_search);
    }

    #[rstest]
    fn test_add_author_search_results_esc(
      #[values(
        ActiveReadarrBlock::AddAuthorSearchResults,
        ActiveReadarrBlock::AddAuthorEmptySearchResults
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::AddAuthorSearchInput.into());
      app.push_navigation_stack(active_readarr_block.into());
      let mut add_searched_authors = StatefulTable::default();
      add_searched_authors.set_items(vec![add_author_search_result()]);
      app.data.readarr_data.add_searched_authors = Some(add_searched_authors);

      AddAuthorHandler::new(ESC_KEY, &mut app, active_readarr_block, None).handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::AddAuthorSearchInput.into());
      assert_modal_absent!(app.data.readarr_data.add_searched_authors);
      assert!(app.ignore_special_keys_for_textbox_input);
    }

    #[test]
    fn test_add_author_already_in_library_esc() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(ActiveReadarrBlock::AddAuthorSearchResults.into());
      app.push_navigation_stack(ActiveReadarrBlock::AddAuthorAlreadyInLibrary.into());

      AddAuthorHandler::new(
        ESC_KEY,
        &mut app,
        ActiveReadarrBlock::AddAuthorAlreadyInLibrary,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::AddAuthorSearchResults.into());
    }

    #[test]
    fn test_add_author_prompt_esc() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(ActiveReadarrBlock::AddAuthorSearchResults.into());
      app.push_navigation_stack(ActiveReadarrBlock::AddAuthorPrompt.into());
      app.data.readarr_data.add_author_modal = Some(AddAuthorModal::default());
      app.data.readarr_data.prompt_confirm = true;

      AddAuthorHandler::new(ESC_KEY, &mut app, ActiveReadarrBlock::AddAuthorPrompt, None).handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::AddAuthorSearchResults.into());
      assert_modal_absent!(app.data.readarr_data.add_author_modal);
      assert!(!app.data.readarr_data.prompt_confirm);
    }

    #[test]
    fn test_add_author_tags_input_esc() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::AddAuthorPrompt.into());
      app.push_navigation_stack(ActiveReadarrBlock::AddAuthorTagsInput.into());
      app.ignore_special_keys_for_textbox_input = true;

      AddAuthorHandler::new(
        ESC_KEY,
        &mut app,
        ActiveReadarrBlock::AddAuthorTagsInput,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::AddAuthorPrompt.into());
      assert!(!app.ignore_special_keys_for_textbox_input);
    }

    #[rstest]
    fn test_add_author_selecting_preferences_blocks_esc(
      #[values(
        ActiveReadarrBlock::AddAuthorSelectMonitor,
        ActiveReadarrBlock::AddAuthorSelectMonitorNewItems,
        ActiveReadarrBlock::AddAuthorSelectQualityProfile,
        ActiveReadarrBlock::AddAuthorSelectMetadataProfile,
        ActiveReadarrBlock::AddAuthorSelectRootFolder
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.push_navigation_stack(ActiveReadarrBlock::AddAuthorPrompt.into());
      app.push_navigation_stack(active_readarr_block.into());
      app.data.readarr_data.add_author_modal = Some(AddAuthorModal::default());

      AddAuthorHandler::new(ESC_KEY, &mut app, active_readarr_block, None).handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::AddAuthorPrompt.into());
    }
  }

  mod test_handle_key_char {
    use super::*;
    use crate::network::readarr_network::ReadarrEvent;
    use pretty_assertions::assert_str_eq;

    #[test]
    fn test_add_author_search_input_backspace() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.add_author_search = Some("Test".into());

      AddAuthorHandler::new(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveReadarrBlock::AddAuthorSearchInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .add_author_search
          .as_ref()
          .unwrap()
          .text,
        "Tes"
      );
    }

    #[test]
    fn test_add_author_search_input_char_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.add_author_search = Some(HorizontallyScrollableText::default());

      AddAuthorHandler::new(
        Key::Char('a'),
        &mut app,
        ActiveReadarrBlock::AddAuthorSearchInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .add_author_search
          .as_ref()
          .unwrap()
          .text,
        "a"
      );
    }

    #[test]
    fn test_add_author_tags_input_backspace() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.add_author_modal = Some(AddAuthorModal {
        tags: "Test".into(),
        ..AddAuthorModal::default()
      });

      AddAuthorHandler::new(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveReadarrBlock::AddAuthorTagsInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .add_author_modal
          .as_ref()
          .unwrap()
          .tags
          .text,
        "Tes"
      );
    }

    #[test]
    fn test_add_author_tags_input_char_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Authors.into());
      app.data.readarr_data.add_author_modal = Some(AddAuthorModal::default());

      AddAuthorHandler::new(
        Key::Char('a'),
        &mut app,
        ActiveReadarrBlock::AddAuthorTagsInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .readarr_data
          .add_author_modal
          .as_ref()
          .unwrap()
          .tags
          .text,
        "a"
      );
    }

    #[test]
    fn test_add_author_confirm_prompt_confirm_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::AddAuthorSearchResults.into());
      app.push_navigation_stack(ActiveReadarrBlock::AddAuthorPrompt.into());
      app.data.readarr_data.selected_block = BlockSelectionState::new(ADD_AUTHOR_SELECTION_BLOCKS);
      app
        .data
        .readarr_data
        .selected_block
        .set_index(0, ADD_AUTHOR_SELECTION_BLOCKS.len() - 1);
      let mut add_searched_authors = StatefulTable::default();
      let mut decoy_search_result = add_author_search_result();
      decoy_search_result.foreign_author_id = "decoy-foreign-id".to_owned();
      decoy_search_result.author_name = "Decoy Author".into();
      add_searched_authors.set_items(vec![decoy_search_result, add_author_search_result()]);
      add_searched_authors.select_index(Some(1));
      app.data.readarr_data.add_searched_authors = Some(add_searched_authors);
      app.data.readarr_data.quality_profile_map =
        BiMap::from_iter([(1111, "Any".to_owned()), (2222, "eBook".to_owned())]);
      app.data.readarr_data.metadata_profile_map =
        BiMap::from_iter([(3333, "None".to_owned()), (4444, "Standard".to_owned())]);
      let mut add_author_modal = AddAuthorModal {
        tags: "usenet, testing".into(),
        ..AddAuthorModal::default()
      };
      add_author_modal
        .monitor_list
        .set_items(Vec::from_iter(MonitorType::iter()));
      add_author_modal.monitor_list.state.select(Some(1));
      add_author_modal
        .monitor_new_items_list
        .set_items(Vec::from_iter(NewItemMonitorType::iter()));
      add_author_modal
        .monitor_new_items_list
        .state
        .select(Some(1));
      add_author_modal
        .quality_profile_list
        .set_items(vec!["Any".to_owned(), "eBook".to_owned()]);
      add_author_modal.quality_profile_list.state.select(Some(1));
      add_author_modal
        .metadata_profile_list
        .set_items(vec!["None".to_owned(), "Standard".to_owned()]);
      add_author_modal.metadata_profile_list.state.select(Some(1));
      add_author_modal.root_folder_list.set_items(vec![
        RootFolder {
          id: 1,
          path: "/nfs".to_owned(),
          accessible: true,
          free_space: 219902325555200,
          unmapped_folders: None,
        },
        RootFolder {
          id: 2,
          path: "/nfs2".to_owned(),
          accessible: true,
          free_space: 21990232555520,
          unmapped_folders: None,
        },
      ]);
      add_author_modal.root_folder_list.state.select(Some(1));
      app.data.readarr_data.add_author_modal = Some(add_author_modal);
      let expected_add_author_body = AddAuthorBody {
        foreign_author_id: SEARCHED_FOREIGN_AUTHOR_ID.to_string(),
        author_name: "Test Author".to_string(),
        monitored: true,
        root_folder_path: "/nfs2".to_string(),
        quality_profile_id: 2222,
        metadata_profile_id: 4444,
        tags: Vec::default(),
        tag_input_string: Some("usenet, testing".to_owned()),
        add_options: AddAuthorOptions {
          monitor: MonitorType::Future,
          monitor_new_items: NewItemMonitorType::None,
          search_for_missing_books: true,
        },
      };
      let expected_readarr_event = ReadarrEvent::AddAuthor(expected_add_author_body);

      AddAuthorHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveReadarrBlock::AddAuthorPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::AddAuthorSearchResults.into());
      assert!(app.data.readarr_data.prompt_confirm);
      assert_some_eq_x!(
        &app.data.readarr_data.prompt_confirm_action,
        &expected_readarr_event
      );
      assert_modal_absent!(app.data.readarr_data.add_author_modal);
    }
  }

  #[test]
  fn test_add_author_handler_accepts() {
    ActiveReadarrBlock::iter().for_each(|readarr_block| {
      if ADD_AUTHOR_BLOCKS.contains(&readarr_block) {
        assert!(
          AddAuthorHandler::accepts(readarr_block),
          "{readarr_block} is not accepted by the AddAuthorHandler"
        );
      } else {
        assert!(!AddAuthorHandler::accepts(readarr_block));
      }
    });
  }

  #[rstest]
  fn test_add_author_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = AddAuthorHandler::new(
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
  fn test_add_author_search_no_panic_on_none_search_result() {
    let mut app = App::test_default();
    app.data.readarr_data.add_searched_authors = None;

    AddAuthorHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::AddAuthorSearchResults,
      None,
    )
    .handle();
  }

  #[test]
  fn test_add_author_handler_is_not_ready_when_loading() {
    let mut app = App::test_default();
    app.is_loading = true;

    let handler = AddAuthorHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::AddAuthorSearchInput,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_add_author_handler_is_ready_when_not_loading() {
    let mut app = App::test_default();
    app.is_loading = false;

    let handler = AddAuthorHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::AddAuthorSearchInput,
      None,
    );

    assert!(handler.is_ready());
  }

  #[test]
  fn test_build_add_author_body() {
    let mut app = App::test_default();
    let mut add_author_modal = AddAuthorModal {
      tags: "usenet, testing".into(),
      ..AddAuthorModal::default()
    };
    add_author_modal.root_folder_list.set_items(vec![
      RootFolder {
        id: 1,
        path: "/nfs".to_owned(),
        accessible: true,
        free_space: 219902325555200,
        unmapped_folders: None,
      },
      RootFolder {
        id: 2,
        path: "/nfs2".to_owned(),
        accessible: true,
        free_space: 21990232555520,
        unmapped_folders: None,
      },
    ]);
    add_author_modal.root_folder_list.state.select(Some(1));
    add_author_modal
      .quality_profile_list
      .set_items(vec!["Any".to_owned(), "eBook".to_owned()]);
    add_author_modal.quality_profile_list.state.select(Some(1));
    add_author_modal
      .metadata_profile_list
      .set_items(vec!["None".to_owned(), "Standard".to_owned()]);
    add_author_modal.metadata_profile_list.state.select(Some(1));
    add_author_modal
      .monitor_list
      .set_items(Vec::from_iter(MonitorType::iter()));
    add_author_modal.monitor_list.state.select(Some(1));
    add_author_modal
      .monitor_new_items_list
      .set_items(Vec::from_iter(NewItemMonitorType::iter()));
    add_author_modal
      .monitor_new_items_list
      .state
      .select(Some(1));
    app.data.readarr_data.add_author_modal = Some(add_author_modal);
    app.data.readarr_data.quality_profile_map =
      BiMap::from_iter([(1111, "Any".to_owned()), (2222, "eBook".to_owned())]);
    app.data.readarr_data.metadata_profile_map =
      BiMap::from_iter([(3333, "None".to_owned()), (4444, "Standard".to_owned())]);
    let mut add_searched_authors = StatefulTable::default();
    let mut decoy_search_result = add_author_search_result();
    decoy_search_result.foreign_author_id = "decoy-foreign-id".to_owned();
    decoy_search_result.author_name = "Decoy Author".into();
    add_searched_authors.set_items(vec![decoy_search_result, add_author_search_result()]);
    add_searched_authors.select_index(Some(1));
    app.data.readarr_data.add_searched_authors = Some(add_searched_authors);
    let expected_add_author_body = AddAuthorBody {
      foreign_author_id: SEARCHED_FOREIGN_AUTHOR_ID.to_string(),
      author_name: "Test Author".into(),
      monitored: true,
      root_folder_path: "/nfs2".to_string(),
      quality_profile_id: 2222,
      metadata_profile_id: 4444,
      tags: Vec::default(),
      tag_input_string: Some("usenet, testing".to_owned()),
      add_options: AddAuthorOptions {
        monitor: MonitorType::Future,
        monitor_new_items: NewItemMonitorType::None,
        search_for_missing_books: true,
      },
    };

    let add_author_body = AddAuthorHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::AddAuthorPrompt,
      None,
    )
    .build_add_author_body();

    assert_eq!(add_author_body, expected_add_author_body);
    assert_modal_absent!(app.data.readarr_data.add_author_modal);
  }

  const SEARCHED_FOREIGN_AUTHOR_ID: &str = "aaaa-1111";
  const OTHER_FOREIGN_AUTHOR_ID: &str = "bbbb-2222";

  fn add_author_search_result() -> AddAuthorSearchResult {
    AddAuthorSearchResult {
      foreign_author_id: SEARCHED_FOREIGN_AUTHOR_ID.to_owned(),
      author_name: "Test Author".into(),
      status: AuthorStatus::Continuing,
      ..AddAuthorSearchResult::default()
    }
  }

  fn author(foreign_author_id: &str) -> Author {
    Author {
      id: 1,
      author_name: "Test Author".into(),
      foreign_author_id: foreign_author_id.to_owned(),
      status: AuthorStatus::Continuing,
      ..Author::default()
    }
  }
}
