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
  use crate::handlers::lidarr_handlers::library::add_artist_handler::AddArtistHandler;
  use crate::models::lidarr_models::{
    AddArtistBody, AddArtistOptions, MonitorType, NewItemMonitorType,
  };
  use crate::models::servarr_data::lidarr::lidarr_data::{
    ADD_ARTIST_BLOCKS, ADD_ARTIST_SELECTION_BLOCKS, ActiveLidarrBlock,
  };
  use crate::models::servarr_data::lidarr::modals::AddArtistModal;
  use crate::models::servarr_models::RootFolder;
  use crate::models::stateful_table::StatefulTable;
  use crate::models::{BlockSelectionState, HorizontallyScrollableText};
  use crate::network::lidarr_network::lidarr_network_test_utils::test_utils::add_artist_search_result;
  use crate::simple_stateful_iterable_vec;

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use strum::IntoEnumIterator;

    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::lidarr::lidarr_data::ADD_ARTIST_SELECTION_BLOCKS;

    use super::*;

    #[rstest]
    fn test_add_artist_select_monitor_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let monitor_vec = Vec::from_iter(MonitorType::iter());
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.add_artist_modal = Some(AddArtistModal::default());
      app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .set_items(monitor_vec.clone());

      if key == Key::Up {
        for i in (0..monitor_vec.len()).rev() {
          AddArtistHandler::new(
            key,
            &mut app,
            ActiveLidarrBlock::AddArtistSelectMonitor,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .lidarr_data
              .add_artist_modal
              .as_ref()
              .unwrap()
              .monitor_list
              .current_selection(),
            &monitor_vec[i]
          );
        }
      } else {
        for i in 0..monitor_vec.len() {
          AddArtistHandler::new(
            key,
            &mut app,
            ActiveLidarrBlock::AddArtistSelectMonitor,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .lidarr_data
              .add_artist_modal
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
    fn test_add_artist_select_monitor_new_items_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let monitor_new_items_vec = Vec::from_iter(NewItemMonitorType::iter());
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.add_artist_modal = Some(AddArtistModal::default());
      app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .monitor_new_items_list
        .set_items(monitor_new_items_vec.clone());

      if key == Key::Up {
        for i in (0..monitor_new_items_vec.len()).rev() {
          AddArtistHandler::new(
            key,
            &mut app,
            ActiveLidarrBlock::AddArtistSelectMonitorNewItems,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .lidarr_data
              .add_artist_modal
              .as_ref()
              .unwrap()
              .monitor_new_items_list
              .current_selection(),
            &monitor_new_items_vec[i]
          );
        }
      } else {
        for i in 0..monitor_new_items_vec.len() {
          AddArtistHandler::new(
            key,
            &mut app,
            ActiveLidarrBlock::AddArtistSelectMonitorNewItems,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .lidarr_data
              .add_artist_modal
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
    fn test_add_artist_select_quality_profile_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.add_artist_modal = Some(AddArtistModal::default());
      app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .set_items(vec!["Test 1".to_owned(), "Test 2".to_owned()]);

      AddArtistHandler::new(
        key,
        &mut app,
        ActiveLidarrBlock::AddArtistSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .add_artist_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 2"
      );

      AddArtistHandler::new(
        key,
        &mut app,
        ActiveLidarrBlock::AddArtistSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .add_artist_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[rstest]
    fn test_add_artist_select_metadata_profile_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.add_artist_modal = Some(AddArtistModal::default());
      app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .set_items(vec!["Test 1".to_owned(), "Test 2".to_owned()]);

      AddArtistHandler::new(
        key,
        &mut app,
        ActiveLidarrBlock::AddArtistSelectMetadataProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .add_artist_modal
          .as_ref()
          .unwrap()
          .metadata_profile_list
          .current_selection(),
        "Test 2"
      );

      AddArtistHandler::new(
        key,
        &mut app,
        ActiveLidarrBlock::AddArtistSelectMetadataProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .add_artist_modal
          .as_ref()
          .unwrap()
          .metadata_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[rstest]
    fn test_add_artist_select_root_folder_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.add_artist_modal = Some(AddArtistModal::default());
      app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .root_folder_list
        .set_items(simple_stateful_iterable_vec!(RootFolder, String, path));

      AddArtistHandler::new(
        key,
        &mut app,
        ActiveLidarrBlock::AddArtistSelectRootFolder,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .add_artist_modal
          .as_ref()
          .unwrap()
          .root_folder_list
          .current_selection()
          .path,
        "Test 2"
      );

      AddArtistHandler::new(
        key,
        &mut app,
        ActiveLidarrBlock::AddArtistSelectRootFolder,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .add_artist_modal
          .as_ref()
          .unwrap()
          .root_folder_list
          .current_selection()
          .path,
        "Test 1"
      );
    }

    #[rstest]
    fn test_add_artist_prompt_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.selected_block = BlockSelectionState::new(ADD_ARTIST_SELECTION_BLOCKS);
      app.data.lidarr_data.selected_block.down();

      AddArtistHandler::new(key, &mut app, ActiveLidarrBlock::AddArtistPrompt, None).handle();

      if key == Key::Up {
        assert_eq!(
          app.data.lidarr_data.selected_block.get_active_block(),
          ActiveLidarrBlock::AddArtistSelectRootFolder
        );
      } else {
        assert_eq!(
          app.data.lidarr_data.selected_block.get_active_block(),
          ActiveLidarrBlock::AddArtistSelectMonitorNewItems
        );
      }
    }

    #[rstest]
    fn test_add_artist_prompt_scroll_no_op_when_not_ready(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.is_loading = true;
      app.data.lidarr_data.selected_block = BlockSelectionState::new(ADD_ARTIST_SELECTION_BLOCKS);
      app.data.lidarr_data.selected_block.down();

      AddArtistHandler::new(key, &mut app, ActiveLidarrBlock::AddArtistPrompt, None).handle();

      assert_eq!(
        app.data.lidarr_data.selected_block.get_active_block(),
        ActiveLidarrBlock::AddArtistSelectMonitor
      );
    }
  }

  mod test_handle_home_end {
    use pretty_assertions::assert_eq;

    use crate::extended_stateful_iterable_vec;

    use super::*;

    #[test]
    fn test_add_artist_select_monitor_home_end() {
      let monitor_vec = Vec::from_iter(MonitorType::iter());
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.add_artist_modal = Some(AddArtistModal::default());
      app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .set_items(monitor_vec.clone());

      AddArtistHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveLidarrBlock::AddArtistSelectMonitor,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .add_artist_modal
          .as_ref()
          .unwrap()
          .monitor_list
          .current_selection(),
        &monitor_vec[monitor_vec.len() - 1]
      );

      AddArtistHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveLidarrBlock::AddArtistSelectMonitor,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .add_artist_modal
          .as_ref()
          .unwrap()
          .monitor_list
          .current_selection(),
        &monitor_vec[0]
      );
    }

    #[test]
    fn test_add_artist_select_monitor_new_items_home_end() {
      let monitor_new_items_vec = Vec::from_iter(NewItemMonitorType::iter());
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.add_artist_modal = Some(AddArtistModal::default());
      app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .monitor_new_items_list
        .set_items(monitor_new_items_vec.clone());

      AddArtistHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveLidarrBlock::AddArtistSelectMonitorNewItems,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .add_artist_modal
          .as_ref()
          .unwrap()
          .monitor_new_items_list
          .current_selection(),
        &monitor_new_items_vec[monitor_new_items_vec.len() - 1]
      );

      AddArtistHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveLidarrBlock::AddArtistSelectMonitorNewItems,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .add_artist_modal
          .as_ref()
          .unwrap()
          .monitor_new_items_list
          .current_selection(),
        &monitor_new_items_vec[0]
      );
    }

    #[test]
    fn test_add_artist_select_quality_profile_home_end() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.add_artist_modal = Some(AddArtistModal::default());
      app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .set_items(vec![
          "Test 1".to_owned(),
          "Test 2".to_owned(),
          "Test 3".to_owned(),
        ]);

      AddArtistHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveLidarrBlock::AddArtistSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .add_artist_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 3"
      );

      AddArtistHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveLidarrBlock::AddArtistSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .add_artist_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[test]
    fn test_add_artist_select_metadata_profile_home_end() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.add_artist_modal = Some(AddArtistModal::default());
      app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .set_items(vec![
          "Test 1".to_owned(),
          "Test 2".to_owned(),
          "Test 3".to_owned(),
        ]);

      AddArtistHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveLidarrBlock::AddArtistSelectMetadataProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .add_artist_modal
          .as_ref()
          .unwrap()
          .metadata_profile_list
          .current_selection(),
        "Test 3"
      );

      AddArtistHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveLidarrBlock::AddArtistSelectMetadataProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .add_artist_modal
          .as_ref()
          .unwrap()
          .metadata_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[test]
    fn test_add_artist_select_root_folder_home_end() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.add_artist_modal = Some(AddArtistModal::default());
      app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .root_folder_list
        .set_items(extended_stateful_iterable_vec!(RootFolder, String, path));

      AddArtistHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveLidarrBlock::AddArtistSelectRootFolder,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .add_artist_modal
          .as_ref()
          .unwrap()
          .root_folder_list
          .current_selection()
          .path,
        "Test 3"
      );

      AddArtistHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveLidarrBlock::AddArtistSelectRootFolder,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .add_artist_modal
          .as_ref()
          .unwrap()
          .root_folder_list
          .current_selection()
          .path,
        "Test 1"
      );
    }

    #[test]
    fn test_add_artist_search_input_home_end_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistSearchInput.into());
      app.data.lidarr_data.add_artist_search = Some("Test".into());

      AddArtistHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveLidarrBlock::AddArtistSearchInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .add_artist_search
          .as_ref()
          .unwrap()
          .offset
          .load(Ordering::SeqCst),
        4
      );

      AddArtistHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveLidarrBlock::AddArtistSearchInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .add_artist_search
          .as_ref()
          .unwrap()
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }

    #[test]
    fn test_add_artist_tags_input_home_end_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.add_artist_modal = Some(AddArtistModal {
        tags: "Test".into(),
        ..AddArtistModal::default()
      });

      AddArtistHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveLidarrBlock::AddArtistTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .add_artist_modal
          .as_ref()
          .unwrap()
          .tags
          .offset
          .load(Ordering::SeqCst),
        4
      );

      AddArtistHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveLidarrBlock::AddArtistTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .add_artist_modal
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
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());

      AddArtistHandler::new(key, &mut app, ActiveLidarrBlock::AddArtistPrompt, None).handle();

      assert!(app.data.lidarr_data.prompt_confirm);

      AddArtistHandler::new(key, &mut app, ActiveLidarrBlock::AddArtistPrompt, None).handle();

      assert!(!app.data.lidarr_data.prompt_confirm);
    }

    #[test]
    fn test_add_artist_search_input_left_right_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistSearchInput.into());
      app.data.lidarr_data.add_artist_search = Some("Test".into());

      AddArtistHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveLidarrBlock::AddArtistSearchInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .add_artist_search
          .as_ref()
          .unwrap()
          .offset
          .load(Ordering::SeqCst),
        1
      );

      AddArtistHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveLidarrBlock::AddArtistSearchInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .add_artist_search
          .as_ref()
          .unwrap()
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }

    #[test]
    fn test_add_artist_tags_input_left_right_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.add_artist_modal = Some(AddArtistModal {
        tags: "Test".into(),
        ..AddArtistModal::default()
      });

      AddArtistHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveLidarrBlock::AddArtistTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .add_artist_modal
          .as_ref()
          .unwrap()
          .tags
          .offset
          .load(Ordering::SeqCst),
        1
      );

      AddArtistHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveLidarrBlock::AddArtistTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .lidarr_data
          .add_artist_modal
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
    use crate::models::lidarr_models::{AddArtistBody, AddArtistOptions};
    use crate::network::lidarr_network::LidarrEvent;
    use crate::network::lidarr_network::lidarr_network_test_utils::test_utils::artist;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_add_artist_search_input_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistSearchInput.into());
      app.ignore_special_keys_for_textbox_input = true;
      app.data.lidarr_data.add_artist_search = Some("test".into());

      AddArtistHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::AddArtistSearchInput,
        None,
      )
      .handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert_navigation_pushed!(app, ActiveLidarrBlock::AddArtistSearchResults.into());
    }

    #[test]
    fn test_add_artist_search_input_submit_noop_on_empty_search() {
      let mut app = App::test_default();
      app.data.lidarr_data.add_artist_search = Some(HorizontallyScrollableText::default());
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistSearchInput.into());
      app.ignore_special_keys_for_textbox_input = true;

      AddArtistHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::AddArtistSearchInput,
        None,
      )
      .handle();

      assert!(app.ignore_special_keys_for_textbox_input);
      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::AddArtistSearchInput.into()
      );
    }

    #[test]
    fn test_add_artist_search_results_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistSearchResults.into());
      let mut add_searched_artists = StatefulTable::default();
      add_searched_artists.set_items(vec![add_artist_search_result()]);
      app.data.lidarr_data.add_searched_artists = Some(add_searched_artists);
      app.data.lidarr_data.quality_profile_map = BiMap::from_iter([(1, "Test".to_owned())]);
      app.data.lidarr_data.metadata_profile_map = BiMap::from_iter([(1, "Test".to_owned())]);

      AddArtistHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::AddArtistSearchResults,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::AddArtistPrompt.into()
      );
      assert_modal_present!(app.data.lidarr_data.add_artist_modal);
      assert_eq!(
        app.data.lidarr_data.selected_block.get_active_block(),
        ActiveLidarrBlock::AddArtistSelectRootFolder
      );
    }

    #[test]
    fn test_add_artist_search_results_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistSearchResults.into());
      let mut add_searched_artists = StatefulTable::default();
      add_searched_artists.set_items(vec![add_artist_search_result()]);
      app.data.lidarr_data.add_searched_artists = Some(add_searched_artists);

      AddArtistHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::AddArtistSearchResults,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::AddArtistSearchResults.into()
      );
      assert_modal_absent!(app.data.lidarr_data.add_artist_modal);
    }

    #[test]
    fn test_add_artist_search_results_submit_does_nothing_on_empty_table() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistSearchResults.into());

      AddArtistHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::AddArtistSearchResults,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveLidarrBlock::AddArtistSearchResults.into()
      );
    }

    #[test]
    fn test_add_artist_search_results_submit_artist_already_in_library() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistSearchResults.into());
      let mut add_searched_artists = StatefulTable::default();
      let search_result = add_artist_search_result();
      add_searched_artists.set_items(vec![search_result.clone()]);
      app.data.lidarr_data.add_searched_artists = Some(add_searched_artists);
      app.data.lidarr_data.artists.set_items(vec![artist()]);

      AddArtistHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::AddArtistSearchResults,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveLidarrBlock::AddArtistAlreadyInLibrary.into());
    }

    #[test]
    fn test_add_artist_prompt_prompt_decline_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistPrompt.into());
      app.data.lidarr_data.selected_block = BlockSelectionState::new(ADD_ARTIST_SELECTION_BLOCKS);
      app
        .data
        .lidarr_data
        .selected_block
        .set_index(0, ADD_ARTIST_SELECTION_BLOCKS.len() - 1);

      AddArtistHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::AddArtistPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::Artists.into());
      assert_none!(app.data.lidarr_data.prompt_confirm_action);
    }

    #[test]
    fn test_add_artist_confirm_prompt_prompt_confirm_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistSearchResults.into());
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistPrompt.into());
      app.data.lidarr_data.selected_block = BlockSelectionState::new(ADD_ARTIST_SELECTION_BLOCKS);
      app
        .data
        .lidarr_data
        .selected_block
        .set_index(0, ADD_ARTIST_SELECTION_BLOCKS.len() - 1);
      app.data.lidarr_data.prompt_confirm = true;
      let mut add_searched_artists = StatefulTable::default();
      add_searched_artists.set_items(vec![add_artist_search_result()]);
      app.data.lidarr_data.add_searched_artists = Some(add_searched_artists);
      app.data.lidarr_data.quality_profile_map = BiMap::from_iter([(1, "Test".to_owned())]);
      app.data.lidarr_data.metadata_profile_map = BiMap::from_iter([(1, "Test".to_owned())]);
      app.data.lidarr_data.add_artist_modal = Some(AddArtistModal {
        tags: "usenet, testing".into(),
        ..AddArtistModal::default()
      });
      app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .set_items(Vec::from_iter(MonitorType::iter()));
      app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .monitor_new_items_list
        .set_items(Vec::from_iter(NewItemMonitorType::iter()));
      app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .set_items(vec!["Test".to_owned()]);
      app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .set_items(vec!["Test".to_owned()]);
      app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .root_folder_list
        .set_items(vec![RootFolder {
          path: "/music".to_owned(),
          ..RootFolder::default()
        }]);
      let expected_add_artist_body = AddArtistBody {
        foreign_artist_id: "test-foreign-id".to_string(),
        artist_name: "Test Artist".to_string(),
        monitored: true,
        root_folder_path: "/music".to_string(),
        quality_profile_id: 1,
        metadata_profile_id: 1,
        tags: Vec::default(),
        tag_input_string: Some("usenet, testing".to_owned()),
        add_options: AddArtistOptions {
          monitor: MonitorType::All,
          monitor_new_items: NewItemMonitorType::All,
          search_for_missing_albums: true,
        },
      };
      let expected_lidarr_event = LidarrEvent::AddArtist(expected_add_artist_body);

      AddArtistHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::AddArtistPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::AddArtistSearchResults.into());
      assert_some_eq_x!(
        &app.data.lidarr_data.prompt_confirm_action,
        &expected_lidarr_event
      );
      assert_modal_absent!(app.data.lidarr_data.add_artist_modal);
    }

    #[rstest]
    #[case(ActiveLidarrBlock::AddArtistSelectRootFolder, 0)]
    #[case(ActiveLidarrBlock::AddArtistSelectMonitor, 1)]
    #[case(ActiveLidarrBlock::AddArtistSelectMonitorNewItems, 2)]
    #[case(ActiveLidarrBlock::AddArtistSelectQualityProfile, 3)]
    #[case(ActiveLidarrBlock::AddArtistSelectMetadataProfile, 4)]
    #[case(ActiveLidarrBlock::AddArtistTagsInput, 5)]
    fn test_add_artist_prompt_selected_block_submit(
      #[case] block: ActiveLidarrBlock,
      #[case] y_index: usize,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistPrompt.into());
      app.data.lidarr_data.selected_block = BlockSelectionState::new(ADD_ARTIST_SELECTION_BLOCKS);
      app.data.lidarr_data.selected_block.set_index(0, y_index);

      AddArtistHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveLidarrBlock::AddArtistPrompt,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, block.into());
    }

    #[rstest]
    fn test_add_artist_prompt_selecting_preferences_blocks_submit(
      #[values(
        ActiveLidarrBlock::AddArtistSelectRootFolder,
        ActiveLidarrBlock::AddArtistSelectMonitor,
        ActiveLidarrBlock::AddArtistSelectMonitorNewItems,
        ActiveLidarrBlock::AddArtistSelectQualityProfile,
        ActiveLidarrBlock::AddArtistSelectMetadataProfile,
        ActiveLidarrBlock::AddArtistTagsInput
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistPrompt.into());
      app.push_navigation_stack(active_lidarr_block.into());

      AddArtistHandler::new(SUBMIT_KEY, &mut app, active_lidarr_block, None).handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::AddArtistPrompt.into());

      if active_lidarr_block == ActiveLidarrBlock::AddArtistTagsInput {
        assert!(!app.ignore_special_keys_for_textbox_input);
      }
    }
  }

  mod test_handle_esc {
    use rstest::rstest;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_add_artist_search_input_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.data.lidarr_data.add_artist_search = Some("test".into());
      app.ignore_special_keys_for_textbox_input = true;
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistSearchInput.into());

      AddArtistHandler::new(
        ESC_KEY,
        &mut app,
        ActiveLidarrBlock::AddArtistSearchInput,
        None,
      )
      .handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert_navigation_popped!(app, ActiveLidarrBlock::Artists.into());
      assert_modal_absent!(app.data.lidarr_data.add_artist_search);
    }

    #[rstest]
    fn test_add_artist_search_results_esc(
      #[values(
        ActiveLidarrBlock::AddArtistSearchResults,
        ActiveLidarrBlock::AddArtistEmptySearchResults
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistSearchInput.into());
      app.push_navigation_stack(active_lidarr_block.into());
      let mut add_searched_artists = StatefulTable::default();
      add_searched_artists.set_items(vec![add_artist_search_result()]);
      app.data.lidarr_data.add_searched_artists = Some(add_searched_artists);

      AddArtistHandler::new(ESC_KEY, &mut app, active_lidarr_block, None).handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::AddArtistSearchInput.into());
      assert_modal_absent!(app.data.lidarr_data.add_searched_artists);
      assert!(app.ignore_special_keys_for_textbox_input);
    }

    #[test]
    fn test_add_artist_already_in_library_esc() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistSearchResults.into());
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistAlreadyInLibrary.into());

      AddArtistHandler::new(
        ESC_KEY,
        &mut app,
        ActiveLidarrBlock::AddArtistAlreadyInLibrary,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::AddArtistSearchResults.into());
    }

    #[test]
    fn test_add_artist_prompt_esc() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistSearchResults.into());
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistPrompt.into());
      app.data.lidarr_data.add_artist_modal = Some(AddArtistModal::default());
      app.data.lidarr_data.prompt_confirm = true;

      AddArtistHandler::new(ESC_KEY, &mut app, ActiveLidarrBlock::AddArtistPrompt, None).handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::AddArtistSearchResults.into());
      assert_modal_absent!(app.data.lidarr_data.add_artist_modal);
      assert!(!app.data.lidarr_data.prompt_confirm);
    }

    #[test]
    fn test_add_artist_tags_input_esc() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistPrompt.into());
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistTagsInput.into());
      app.ignore_special_keys_for_textbox_input = true;

      AddArtistHandler::new(
        ESC_KEY,
        &mut app,
        ActiveLidarrBlock::AddArtistTagsInput,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::AddArtistPrompt.into());
      assert!(!app.ignore_special_keys_for_textbox_input);
    }

    #[rstest]
    fn test_add_artist_selecting_preferences_blocks_esc(
      #[values(
        ActiveLidarrBlock::AddArtistSelectMonitor,
        ActiveLidarrBlock::AddArtistSelectMonitorNewItems,
        ActiveLidarrBlock::AddArtistSelectQualityProfile,
        ActiveLidarrBlock::AddArtistSelectMetadataProfile,
        ActiveLidarrBlock::AddArtistSelectRootFolder
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistPrompt.into());
      app.push_navigation_stack(active_lidarr_block.into());
      app.data.lidarr_data.add_artist_modal = Some(AddArtistModal::default());

      AddArtistHandler::new(ESC_KEY, &mut app, active_lidarr_block, None).handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::AddArtistPrompt.into());
    }
  }

  mod test_handle_key_char {
    use super::*;
    use crate::models::lidarr_models::{AddArtistBody, AddArtistOptions};
    use crate::network::lidarr_network::LidarrEvent;
    use pretty_assertions::assert_str_eq;

    #[test]
    fn test_add_artist_search_input_backspace() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.add_artist_search = Some("Test".into());

      AddArtistHandler::new(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveLidarrBlock::AddArtistSearchInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .add_artist_search
          .as_ref()
          .unwrap()
          .text,
        "Tes"
      );
    }

    #[test]
    fn test_add_artist_search_input_char_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.add_artist_search = Some(HorizontallyScrollableText::default());

      AddArtistHandler::new(
        Key::Char('a'),
        &mut app,
        ActiveLidarrBlock::AddArtistSearchInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .add_artist_search
          .as_ref()
          .unwrap()
          .text,
        "a"
      );
    }

    #[test]
    fn test_add_artist_tags_input_backspace() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.add_artist_modal = Some(AddArtistModal {
        tags: "Test".into(),
        ..AddArtistModal::default()
      });

      AddArtistHandler::new(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveLidarrBlock::AddArtistTagsInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .add_artist_modal
          .as_ref()
          .unwrap()
          .tags
          .text,
        "Tes"
      );
    }

    #[test]
    fn test_add_artist_tags_input_char_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Artists.into());
      app.data.lidarr_data.add_artist_modal = Some(AddArtistModal::default());

      AddArtistHandler::new(
        Key::Char('a'),
        &mut app,
        ActiveLidarrBlock::AddArtistTagsInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .lidarr_data
          .add_artist_modal
          .as_ref()
          .unwrap()
          .tags
          .text,
        "a"
      );
    }

    #[test]
    fn test_add_artist_confirm_prompt_confirm_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistSearchResults.into());
      app.push_navigation_stack(ActiveLidarrBlock::AddArtistPrompt.into());
      app.data.lidarr_data.selected_block = BlockSelectionState::new(ADD_ARTIST_SELECTION_BLOCKS);
      app
        .data
        .lidarr_data
        .selected_block
        .set_index(0, ADD_ARTIST_SELECTION_BLOCKS.len() - 1);
      let mut add_searched_artists = StatefulTable::default();
      add_searched_artists.set_items(vec![add_artist_search_result()]);
      app.data.lidarr_data.add_searched_artists = Some(add_searched_artists);
      app.data.lidarr_data.quality_profile_map = BiMap::from_iter([(1, "Test".to_owned())]);
      app.data.lidarr_data.metadata_profile_map = BiMap::from_iter([(1, "Test".to_owned())]);
      let mut add_artist_modal = AddArtistModal {
        tags: "usenet, testing".into(),
        ..AddArtistModal::default()
      };
      add_artist_modal
        .monitor_list
        .set_items(Vec::from_iter(MonitorType::iter()));
      add_artist_modal
        .monitor_new_items_list
        .set_items(Vec::from_iter(NewItemMonitorType::iter()));
      add_artist_modal
        .quality_profile_list
        .set_items(vec!["Test".to_owned()]);
      add_artist_modal
        .metadata_profile_list
        .set_items(vec!["Test".to_owned()]);
      add_artist_modal.root_folder_list.set_items(vec![
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
      add_artist_modal.root_folder_list.state.select(Some(1));
      app.data.lidarr_data.add_artist_modal = Some(add_artist_modal);
      let expected_add_artist_body = AddArtistBody {
        foreign_artist_id: "test-foreign-id".to_string(),
        artist_name: "Test Artist".to_string(),
        monitored: true,
        root_folder_path: "/nfs2".to_string(),
        quality_profile_id: 1,
        metadata_profile_id: 1,
        tags: Vec::default(),
        tag_input_string: Some("usenet, testing".to_owned()),
        add_options: AddArtistOptions {
          monitor: Default::default(),
          monitor_new_items: Default::default(),
          search_for_missing_albums: true,
        },
      };
      let expected_lidarr_event = LidarrEvent::AddArtist(expected_add_artist_body);

      AddArtistHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveLidarrBlock::AddArtistPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::AddArtistSearchResults.into());
      assert!(app.data.lidarr_data.prompt_confirm);
      assert_some_eq_x!(
        &app.data.lidarr_data.prompt_confirm_action,
        &expected_lidarr_event
      );
      assert_modal_absent!(app.data.lidarr_data.add_artist_modal);
    }
  }

  #[test]
  fn test_add_artist_handler_accepts() {
    ActiveLidarrBlock::iter().for_each(|active_lidarr_block| {
      if ADD_ARTIST_BLOCKS.contains(&active_lidarr_block) {
        assert!(AddArtistHandler::accepts(active_lidarr_block));
      } else {
        assert!(!AddArtistHandler::accepts(active_lidarr_block));
      }
    });
  }

  #[rstest]
  fn test_add_artist_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = AddArtistHandler::new(
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
  fn test_add_artist_search_no_panic_on_none_search_result() {
    let mut app = App::test_default();
    app.data.lidarr_data.add_searched_artists = None;

    AddArtistHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::AddArtistSearchResults,
      None,
    )
    .handle();
  }

  #[test]
  fn test_add_artist_handler_is_not_ready_when_loading() {
    let mut app = App::test_default();
    app.is_loading = true;

    let handler = AddArtistHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::AddArtistSearchInput,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_add_artist_handler_is_ready_when_not_loading() {
    let mut app = App::test_default();
    app.is_loading = false;

    let handler = AddArtistHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::AddArtistSearchInput,
      None,
    );

    assert!(handler.is_ready());
  }

  #[test]
  fn test_build_add_artist_body() {
    let mut app = App::test_default();
    let mut add_artist_modal = AddArtistModal {
      tags: "usenet, testing".into(),
      ..AddArtistModal::default()
    };
    add_artist_modal.root_folder_list.set_items(vec![
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
    add_artist_modal.root_folder_list.state.select(Some(1));
    add_artist_modal
      .quality_profile_list
      .set_items(vec!["Lossless".to_owned()]);
    add_artist_modal
      .metadata_profile_list
      .set_items(vec!["Standard".to_owned()]);
    add_artist_modal
      .monitor_list
      .set_items(Vec::from_iter(MonitorType::iter()));
    add_artist_modal
      .monitor_new_items_list
      .set_items(Vec::from_iter(NewItemMonitorType::iter()));
    app.data.lidarr_data.add_artist_modal = Some(add_artist_modal);
    app.data.lidarr_data.quality_profile_map = BiMap::from_iter([(1, "Lossless".to_owned())]);
    app.data.lidarr_data.metadata_profile_map = BiMap::from_iter([(1, "Standard".to_owned())]);
    let mut add_searched_artists = StatefulTable::default();
    add_searched_artists.set_items(vec![add_artist_search_result()]);
    app.data.lidarr_data.add_searched_artists = Some(add_searched_artists);
    let expected_add_artist_body = AddArtistBody {
      foreign_artist_id: "test-foreign-id".to_string(),
      artist_name: "Test Artist".into(),
      monitored: true,
      root_folder_path: "/nfs2".to_string(),
      quality_profile_id: 1,
      metadata_profile_id: 1,
      tags: Vec::default(),
      tag_input_string: Some("usenet, testing".to_owned()),
      add_options: AddArtistOptions {
        monitor: Default::default(),
        monitor_new_items: Default::default(),
        search_for_missing_albums: true,
      },
    };

    let add_artist_body = AddArtistHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::AddArtistPrompt,
      None,
    )
    .build_add_artist_body();

    assert_eq!(add_artist_body, expected_add_artist_body);
    assert_modal_absent!(app.data.lidarr_data.add_artist_modal);
  }
}
