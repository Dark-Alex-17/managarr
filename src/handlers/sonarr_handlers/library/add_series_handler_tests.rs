#[cfg(test)]
mod tests {
  use bimap::BiMap;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::sonarr_handlers::library::add_series_handler::AddSeriesHandler;
  use crate::handlers::sonarr_handlers::sonarr_handler_test_utils::utils::add_series_search_result;
  use crate::handlers::KeyEventHandler;
  use crate::models::servarr_data::sonarr::modals::AddSeriesModal;
  use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, ADD_SERIES_BLOCKS};
  use crate::models::servarr_models::RootFolder;
  use crate::models::sonarr_models::{
    AddSeriesBody, AddSeriesOptions, AddSeriesSearchResult, SeriesMonitor, SeriesType,
  };
  use crate::models::stateful_table::StatefulTable;
  use crate::models::HorizontallyScrollableText;

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use strum::IntoEnumIterator;

    use crate::models::servarr_data::sonarr::modals::AddSeriesModal;
    use crate::models::servarr_data::sonarr::sonarr_data::ADD_SERIES_SELECTION_BLOCKS;
    use crate::models::BlockSelectionState;
    use crate::simple_stateful_iterable_vec;

    use super::*;

    #[rstest]
    fn test_add_series_select_monitor_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let monitor_vec = Vec::from_iter(SeriesMonitor::iter());
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.add_series_modal = Some(AddSeriesModal::default());
      app
        .data
        .sonarr_data
        .add_series_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .set_items(monitor_vec.clone());

      if key == Key::Up {
        for i in (0..monitor_vec.len()).rev() {
          AddSeriesHandler::new(
            key,
            &mut app,
            ActiveSonarrBlock::AddSeriesSelectMonitor,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .sonarr_data
              .add_series_modal
              .as_ref()
              .unwrap()
              .monitor_list
              .current_selection(),
            &monitor_vec[i]
          );
        }
      } else {
        for i in 0..monitor_vec.len() {
          AddSeriesHandler::new(
            key,
            &mut app,
            ActiveSonarrBlock::AddSeriesSelectMonitor,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .sonarr_data
              .add_series_modal
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
    fn test_add_series_select_series_type_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let series_type_vec = Vec::from_iter(SeriesType::iter());
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.add_series_modal = Some(AddSeriesModal::default());
      app
        .data
        .sonarr_data
        .add_series_modal
        .as_mut()
        .unwrap()
        .series_type_list
        .set_items(series_type_vec.clone());

      if key == Key::Up {
        for i in (0..series_type_vec.len()).rev() {
          AddSeriesHandler::new(
            key,
            &mut app,
            ActiveSonarrBlock::AddSeriesSelectSeriesType,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .sonarr_data
              .add_series_modal
              .as_ref()
              .unwrap()
              .series_type_list
              .current_selection(),
            &series_type_vec[i]
          );
        }
      } else {
        for i in 0..series_type_vec.len() {
          AddSeriesHandler::new(
            key,
            &mut app,
            ActiveSonarrBlock::AddSeriesSelectSeriesType,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .sonarr_data
              .add_series_modal
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
    fn test_add_series_select_quality_profile_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.add_series_modal = Some(AddSeriesModal::default());
      app
        .data
        .sonarr_data
        .add_series_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .set_items(vec!["Test 1".to_owned(), "Test 2".to_owned()]);

      AddSeriesHandler::new(
        key,
        &mut app,
        ActiveSonarrBlock::AddSeriesSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .add_series_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 2"
      );

      AddSeriesHandler::new(
        key,
        &mut app,
        ActiveSonarrBlock::AddSeriesSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .add_series_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[rstest]
    fn test_add_series_select_language_profile_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.add_series_modal = Some(AddSeriesModal::default());
      app
        .data
        .sonarr_data
        .add_series_modal
        .as_mut()
        .unwrap()
        .language_profile_list
        .set_items(vec!["Test 1".to_owned(), "Test 2".to_owned()]);

      AddSeriesHandler::new(
        key,
        &mut app,
        ActiveSonarrBlock::AddSeriesSelectLanguageProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .add_series_modal
          .as_ref()
          .unwrap()
          .language_profile_list
          .current_selection(),
        "Test 2"
      );

      AddSeriesHandler::new(
        key,
        &mut app,
        ActiveSonarrBlock::AddSeriesSelectLanguageProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .add_series_modal
          .as_ref()
          .unwrap()
          .language_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[rstest]
    fn test_add_series_select_root_folder_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.add_series_modal = Some(AddSeriesModal::default());
      app
        .data
        .sonarr_data
        .add_series_modal
        .as_mut()
        .unwrap()
        .root_folder_list
        .set_items(simple_stateful_iterable_vec!(RootFolder, String, path));

      AddSeriesHandler::new(
        key,
        &mut app,
        ActiveSonarrBlock::AddSeriesSelectRootFolder,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .add_series_modal
          .as_ref()
          .unwrap()
          .root_folder_list
          .current_selection()
          .path,
        "Test 2"
      );

      AddSeriesHandler::new(
        key,
        &mut app,
        ActiveSonarrBlock::AddSeriesSelectRootFolder,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .add_series_modal
          .as_ref()
          .unwrap()
          .root_folder_list
          .current_selection()
          .path,
        "Test 1"
      );
    }

    #[rstest]
    fn test_add_series_prompt_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.selected_block = BlockSelectionState::new(ADD_SERIES_SELECTION_BLOCKS);
      app.data.sonarr_data.selected_block.down();

      AddSeriesHandler::new(key, &mut app, ActiveSonarrBlock::AddSeriesPrompt, None).handle();

      if key == Key::Up {
        assert_eq!(
          app.data.sonarr_data.selected_block.get_active_block(),
          ActiveSonarrBlock::AddSeriesSelectRootFolder
        );
      } else {
        assert_eq!(
          app.data.sonarr_data.selected_block.get_active_block(),
          ActiveSonarrBlock::AddSeriesSelectQualityProfile
        );
      }
    }

    #[rstest]
    fn test_add_series_prompt_scroll_no_op_when_not_ready(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.is_loading = true;
      app.data.sonarr_data.selected_block = BlockSelectionState::new(ADD_SERIES_SELECTION_BLOCKS);
      app.data.sonarr_data.selected_block.down();

      AddSeriesHandler::new(key, &mut app, ActiveSonarrBlock::AddSeriesPrompt, None).handle();

      assert_eq!(
        app.data.sonarr_data.selected_block.get_active_block(),
        ActiveSonarrBlock::AddSeriesSelectMonitor
      );
    }
  }

  mod test_handle_home_end {
    use pretty_assertions::assert_eq;
    use std::sync::atomic::Ordering;

    use strum::IntoEnumIterator;

    use crate::extended_stateful_iterable_vec;
    use crate::models::servarr_data::sonarr::modals::AddSeriesModal;

    use super::*;

    #[test]
    fn test_add_series_select_monitor_home_end() {
      let monitor_vec = Vec::from_iter(SeriesMonitor::iter());
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.add_series_modal = Some(AddSeriesModal::default());
      app
        .data
        .sonarr_data
        .add_series_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .set_items(monitor_vec.clone());

      AddSeriesHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::AddSeriesSelectMonitor,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .add_series_modal
          .as_ref()
          .unwrap()
          .monitor_list
          .current_selection(),
        &monitor_vec[monitor_vec.len() - 1]
      );

      AddSeriesHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::AddSeriesSelectMonitor,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .add_series_modal
          .as_ref()
          .unwrap()
          .monitor_list
          .current_selection(),
        &monitor_vec[0]
      );
    }

    #[test]
    fn test_add_series_select_series_type_home_end() {
      let series_type_vec = Vec::from_iter(SeriesType::iter());
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.add_series_modal = Some(AddSeriesModal::default());
      app
        .data
        .sonarr_data
        .add_series_modal
        .as_mut()
        .unwrap()
        .series_type_list
        .set_items(series_type_vec.clone());

      AddSeriesHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::AddSeriesSelectSeriesType,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .add_series_modal
          .as_ref()
          .unwrap()
          .series_type_list
          .current_selection(),
        &series_type_vec[series_type_vec.len() - 1]
      );

      AddSeriesHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::AddSeriesSelectSeriesType,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .add_series_modal
          .as_ref()
          .unwrap()
          .series_type_list
          .current_selection(),
        &series_type_vec[0]
      );
    }

    #[test]
    fn test_add_series_select_quality_profile_home_end() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.add_series_modal = Some(AddSeriesModal::default());
      app
        .data
        .sonarr_data
        .add_series_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .set_items(vec![
          "Test 1".to_owned(),
          "Test 2".to_owned(),
          "Test 3".to_owned(),
        ]);

      AddSeriesHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::AddSeriesSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .add_series_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 3"
      );

      AddSeriesHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::AddSeriesSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .add_series_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[test]
    fn test_add_series_select_language_profile_home_end() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.add_series_modal = Some(AddSeriesModal::default());
      app
        .data
        .sonarr_data
        .add_series_modal
        .as_mut()
        .unwrap()
        .language_profile_list
        .set_items(vec![
          "Test 1".to_owned(),
          "Test 2".to_owned(),
          "Test 3".to_owned(),
        ]);

      AddSeriesHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::AddSeriesSelectLanguageProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .add_series_modal
          .as_ref()
          .unwrap()
          .language_profile_list
          .current_selection(),
        "Test 3"
      );

      AddSeriesHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::AddSeriesSelectLanguageProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .add_series_modal
          .as_ref()
          .unwrap()
          .language_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[test]
    fn test_add_series_select_root_folder_home_end() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.add_series_modal = Some(AddSeriesModal::default());
      app
        .data
        .sonarr_data
        .add_series_modal
        .as_mut()
        .unwrap()
        .root_folder_list
        .set_items(extended_stateful_iterable_vec!(RootFolder, String, path));

      AddSeriesHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::AddSeriesSelectRootFolder,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .add_series_modal
          .as_ref()
          .unwrap()
          .root_folder_list
          .current_selection()
          .path,
        "Test 3"
      );

      AddSeriesHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::AddSeriesSelectRootFolder,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .add_series_modal
          .as_ref()
          .unwrap()
          .root_folder_list
          .current_selection()
          .path,
        "Test 1"
      );
    }

    #[test]
    fn test_add_series_search_input_home_end_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.add_series_search = Some("Test".into());

      AddSeriesHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::AddSeriesSearchInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .add_series_search
          .as_ref()
          .unwrap()
          .offset
          .load(Ordering::SeqCst),
        4
      );

      AddSeriesHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::AddSeriesSearchInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .add_series_search
          .as_ref()
          .unwrap()
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }

    #[test]
    fn test_add_series_tags_input_home_end_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.add_series_modal = Some(AddSeriesModal {
        tags: "Test".into(),
        ..AddSeriesModal::default()
      });

      AddSeriesHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::AddSeriesTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .add_series_modal
          .as_ref()
          .unwrap()
          .tags
          .offset
          .load(Ordering::SeqCst),
        4
      );

      AddSeriesHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::AddSeriesTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .add_series_modal
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

    use crate::models::servarr_data::sonarr::modals::AddSeriesModal;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_left_right_prompt_toggle(#[values(Key::Left, Key::Right)] key: Key) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());

      AddSeriesHandler::new(key, &mut app, ActiveSonarrBlock::AddSeriesPrompt, None).handle();

      assert!(app.data.sonarr_data.prompt_confirm);

      AddSeriesHandler::new(key, &mut app, ActiveSonarrBlock::AddSeriesPrompt, None).handle();

      assert!(!app.data.sonarr_data.prompt_confirm);
    }

    #[test]
    fn test_add_series_search_input_left_right_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.add_series_search = Some("Test".into());

      AddSeriesHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveSonarrBlock::AddSeriesSearchInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .add_series_search
          .as_ref()
          .unwrap()
          .offset
          .load(Ordering::SeqCst),
        1
      );

      AddSeriesHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveSonarrBlock::AddSeriesSearchInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .add_series_search
          .as_ref()
          .unwrap()
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }

    #[test]
    fn test_add_series_tags_input_left_right_keys() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.add_series_modal = Some(AddSeriesModal {
        tags: "Test".into(),
        ..AddSeriesModal::default()
      });

      AddSeriesHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveSonarrBlock::AddSeriesTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .add_series_modal
          .as_ref()
          .unwrap()
          .tags
          .offset
          .load(Ordering::SeqCst),
        1
      );

      AddSeriesHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveSonarrBlock::AddSeriesTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .add_series_modal
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
    use bimap::BiMap;
    use pretty_assertions::{assert_eq, assert_str_eq};
    use rstest::rstest;

    use crate::models::servarr_data::sonarr::modals::AddSeriesModal;
    use crate::models::servarr_data::sonarr::sonarr_data::ADD_SERIES_SELECTION_BLOCKS;
    use crate::models::sonarr_models::Series;
    use crate::models::stateful_table::StatefulTable;
    use crate::models::BlockSelectionState;
    use crate::network::sonarr_network::SonarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_add_series_search_input_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.should_ignore_quit_key = true;
      app.data.sonarr_data.add_series_search = Some("test".into());

      AddSeriesHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::AddSeriesSearchInput,
        None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::AddSeriesSearchResults.into()
      );
    }

    #[test]
    fn test_add_series_search_input_submit_noop_on_empty_search() {
      let mut app = App::test_default();
      app.data.sonarr_data.add_series_search = Some(HorizontallyScrollableText::default());
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::AddSeriesSearchInput.into());
      app.should_ignore_quit_key = true;

      AddSeriesHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::AddSeriesSearchInput,
        None,
      )
      .handle();

      assert!(app.should_ignore_quit_key);
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::AddSeriesSearchInput.into()
      );
    }

    #[test]
    fn test_add_series_search_results_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      let mut add_searched_series = StatefulTable::default();
      add_searched_series.set_items(vec![AddSeriesSearchResult::default()]);
      app.data.sonarr_data.add_searched_series = Some(add_searched_series);
      app.data.sonarr_data.quality_profile_map =
        BiMap::from_iter([(1, "B - Test 2".to_owned()), (0, "A - Test 1".to_owned())]);

      AddSeriesHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::AddSeriesSearchResults,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::AddSeriesPrompt.into()
      );
      assert_eq!(
        app.data.sonarr_data.selected_block.get_active_block(),
        ActiveSonarrBlock::AddSeriesSelectRootFolder
      );
      assert!(app.data.sonarr_data.add_series_modal.is_some());
      assert!(!app
        .data
        .sonarr_data
        .add_series_modal
        .as_ref()
        .unwrap()
        .monitor_list
        .items
        .is_empty());
      assert!(!app
        .data
        .sonarr_data
        .add_series_modal
        .as_ref()
        .unwrap()
        .series_type_list
        .items
        .is_empty());
      assert!(!app
        .data
        .sonarr_data
        .add_series_modal
        .as_ref()
        .unwrap()
        .quality_profile_list
        .items
        .is_empty());
      assert_str_eq!(
        app
          .data
          .sonarr_data
          .add_series_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "A - Test 1"
      );
    }

    #[test]
    fn test_add_series_search_results_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::AddSeriesSearchResults.into());
      let mut add_searched_series = StatefulTable::default();
      add_searched_series.set_items(vec![AddSeriesSearchResult::default()]);

      AddSeriesHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::AddSeriesSearchResults,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::AddSeriesSearchResults.into()
      );
      assert!(app.data.sonarr_data.add_series_modal.is_none());
    }

    #[test]
    fn test_add_series_search_results_submit_does_nothing_on_empty_table() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::AddSeriesSearchResults.into());
      AddSeriesHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::AddSeriesSearchResults,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::AddSeriesSearchResults.into()
      );
    }

    #[test]
    fn test_add_series_search_results_submit_series_already_in_library() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      let mut add_searched_series = StatefulTable::default();
      add_searched_series.set_items(vec![AddSeriesSearchResult::default()]);
      app.data.sonarr_data.add_searched_series = Some(add_searched_series);
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);

      AddSeriesHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::AddSeriesSearchResults,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::AddSeriesAlreadyInLibrary.into()
      );
    }

    #[test]
    fn test_add_series_prompt_prompt_decline_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::AddSeriesPrompt.into());
      app.data.sonarr_data.selected_block = BlockSelectionState::new(ADD_SERIES_SELECTION_BLOCKS);
      app
        .data
        .sonarr_data
        .selected_block
        .set_index(0, ADD_SERIES_SELECTION_BLOCKS.len() - 1);

      AddSeriesHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::AddSeriesPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
      assert_eq!(app.data.sonarr_data.prompt_confirm_action, None);
    }

    #[test]
    fn test_add_series_confirm_prompt_prompt_confirmation_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::AddSeriesPrompt.into());
      app.data.sonarr_data.prompt_confirm = true;
      let mut add_series_modal = AddSeriesModal {
        use_season_folder: true,
        tags: "usenet, testing".into(),
        ..AddSeriesModal::default()
      };
      add_series_modal.root_folder_list.set_items(vec![
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
      add_series_modal.root_folder_list.state.select(Some(1));
      add_series_modal
        .quality_profile_list
        .set_items(vec!["HD - 1080p".to_owned()]);
      add_series_modal
        .language_profile_list
        .set_items(vec!["English".to_owned()]);
      add_series_modal
        .monitor_list
        .set_items(Vec::from_iter(SeriesMonitor::iter()));
      add_series_modal
        .series_type_list
        .set_items(Vec::from_iter(SeriesType::iter()));
      app.data.sonarr_data.add_series_modal = Some(add_series_modal);
      app.data.sonarr_data.quality_profile_map =
        BiMap::from_iter([(2222, "HD - 1080p".to_owned())]);
      app.data.sonarr_data.language_profiles_map = BiMap::from_iter([(2222, "English".to_owned())]);
      let mut add_searched_series = StatefulTable::default();
      add_searched_series.set_items(vec![add_series_search_result()]);
      app.data.sonarr_data.add_searched_series = Some(add_searched_series);
      let expected_add_series_body = AddSeriesBody {
        tvdb_id: 1234,
        title: "Test".to_owned(),
        monitored: true,
        root_folder_path: "/nfs2".to_owned(),
        quality_profile_id: 2222,
        language_profile_id: 2222,
        series_type: "standard".to_owned(),
        season_folder: true,
        tags: Vec::default(),
        tag_input_string: Some("usenet, testing".to_owned()),
        add_options: AddSeriesOptions {
          monitor: "all".to_owned(),
          search_for_cutoff_unmet_episodes: true,
          search_for_missing_episodes: true,
        },
      };
      app.data.sonarr_data.selected_block = BlockSelectionState::new(ADD_SERIES_SELECTION_BLOCKS);
      app
        .data
        .sonarr_data
        .selected_block
        .set_index(0, ADD_SERIES_SELECTION_BLOCKS.len() - 1);

      AddSeriesHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::AddSeriesPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
      assert_eq!(
        app.data.sonarr_data.prompt_confirm_action,
        Some(SonarrEvent::AddSeries(expected_add_series_body))
      );
      assert!(app.data.sonarr_data.add_series_modal.is_none());
    }

    #[rstest]
    #[case(ActiveSonarrBlock::AddSeriesSelectRootFolder, 0)]
    #[case(ActiveSonarrBlock::AddSeriesSelectMonitor, 1)]
    #[case(ActiveSonarrBlock::AddSeriesSelectQualityProfile, 2)]
    #[case(ActiveSonarrBlock::AddSeriesSelectLanguageProfile, 3)]
    #[case(ActiveSonarrBlock::AddSeriesSelectSeriesType, 4)]
    #[case(ActiveSonarrBlock::AddSeriesTagsInput, 6)]
    fn test_add_series_prompt_selected_block_submit(
      #[case] selected_block: ActiveSonarrBlock,
      #[case] y_index: usize,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::AddSeriesPrompt.into());
      app.data.sonarr_data.selected_block = BlockSelectionState::new(ADD_SERIES_SELECTION_BLOCKS);
      app.data.sonarr_data.selected_block.set_index(0, y_index);

      AddSeriesHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::AddSeriesPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), selected_block.into());
      assert_eq!(app.data.sonarr_data.prompt_confirm_action, None);

      if selected_block == ActiveSonarrBlock::AddSeriesTagsInput {
        assert!(app.should_ignore_quit_key);
      }
    }

    #[rstest]
    fn test_add_series_prompt_selecting_preferences_blocks_submit(
      #[values(
        ActiveSonarrBlock::AddSeriesSelectMonitor,
        ActiveSonarrBlock::AddSeriesSelectSeriesType,
        ActiveSonarrBlock::AddSeriesSelectQualityProfile,
        ActiveSonarrBlock::AddSeriesSelectLanguageProfile,
        ActiveSonarrBlock::AddSeriesSelectRootFolder,
        ActiveSonarrBlock::AddSeriesTagsInput
      )]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::AddSeriesPrompt.into());
      app.push_navigation_stack(active_sonarr_block.into());

      AddSeriesHandler::new(SUBMIT_KEY, &mut app, active_sonarr_block, None).handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::AddSeriesPrompt.into()
      );

      if active_sonarr_block == ActiveSonarrBlock::AddSeriesTagsInput {
        assert!(!app.should_ignore_quit_key);
      }
    }

    #[test]
    fn test_add_series_toggle_use_season_folder_submit() {
      let mut app = App::test_default();
      app.data.sonarr_data.add_series_modal = Some(AddSeriesModal::default());
      app.data.sonarr_data.selected_block = BlockSelectionState::new(ADD_SERIES_SELECTION_BLOCKS);
      app.data.sonarr_data.selected_block.set_index(0, 5);
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::AddSeriesPrompt.into());

      AddSeriesHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::AddSeriesPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::AddSeriesPrompt.into()
      );
      assert!(
        app
          .data
          .sonarr_data
          .add_series_modal
          .as_ref()
          .unwrap()
          .use_season_folder
      );

      AddSeriesHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::AddSeriesPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::AddSeriesPrompt.into()
      );
      assert!(
        !app
          .data
          .sonarr_data
          .add_series_modal
          .as_ref()
          .unwrap()
          .use_season_folder
      );
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::models::servarr_data::sonarr::modals::AddSeriesModal;
    use crate::models::servarr_data::sonarr::sonarr_data::sonarr_test_utils::utils::create_test_sonarr_data;
    use crate::models::stateful_table::StatefulTable;
    use crate::simple_stateful_iterable_vec;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_add_series_search_input_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.data.sonarr_data = create_test_sonarr_data();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::AddSeriesSearchInput.into());

      AddSeriesHandler::new(
        ESC_KEY,
        &mut app,
        ActiveSonarrBlock::AddSeriesSearchInput,
        None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
      assert_eq!(app.data.sonarr_data.add_series_search, None);
    }

    #[test]
    fn test_add_series_input_esc() {
      let mut app = App::test_default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::AddSeriesPrompt.into());
      app.push_navigation_stack(ActiveSonarrBlock::AddSeriesTagsInput.into());

      AddSeriesHandler::new(
        ESC_KEY,
        &mut app,
        ActiveSonarrBlock::AddSeriesTagsInput,
        None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::AddSeriesPrompt.into()
      );
    }

    #[rstest]
    fn test_add_series_search_results_esc(
      #[values(
        ActiveSonarrBlock::AddSeriesSearchResults,
        ActiveSonarrBlock::AddSeriesEmptySearchResults
      )]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::AddSeriesSearchInput.into());
      app.push_navigation_stack(active_sonarr_block.into());
      let mut add_searched_series = StatefulTable::default();
      add_searched_series.set_items(simple_stateful_iterable_vec!(
        AddSeriesSearchResult,
        HorizontallyScrollableText
      ));
      app.data.sonarr_data.add_searched_series = Some(add_searched_series);

      AddSeriesHandler::new(ESC_KEY, &mut app, active_sonarr_block, None).handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::AddSeriesSearchInput.into()
      );
      assert!(app.data.sonarr_data.add_searched_series.is_none());
      assert!(app.should_ignore_quit_key);
    }

    #[test]
    fn test_add_series_already_in_library_esc() {
      let mut app = App::test_default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::AddSeriesSearchResults.into());
      app.push_navigation_stack(ActiveSonarrBlock::AddSeriesAlreadyInLibrary.into());

      AddSeriesHandler::new(
        ESC_KEY,
        &mut app,
        ActiveSonarrBlock::AddSeriesAlreadyInLibrary,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::AddSeriesSearchResults.into()
      );
    }

    #[test]
    fn test_add_series_prompt_esc() {
      let mut app = App::test_default();
      app.data.sonarr_data.add_series_modal = Some(AddSeriesModal::default());
      app.data.sonarr_data = create_test_sonarr_data();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::AddSeriesSearchResults.into());
      app.push_navigation_stack(ActiveSonarrBlock::AddSeriesPrompt.into());

      AddSeriesHandler::new(ESC_KEY, &mut app, ActiveSonarrBlock::AddSeriesPrompt, None).handle();

      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::AddSeriesSearchResults.into()
      );
      assert!(app.data.sonarr_data.add_series_modal.is_none());
    }

    #[test]
    fn test_add_series_tags_input_esc() {
      let mut app = App::test_default();
      app.data.sonarr_data = create_test_sonarr_data();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::AddSeriesPrompt.into());
      app.push_navigation_stack(ActiveSonarrBlock::AddSeriesTagsInput.into());

      AddSeriesHandler::new(
        ESC_KEY,
        &mut app,
        ActiveSonarrBlock::AddSeriesTagsInput,
        None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::AddSeriesPrompt.into()
      );
    }

    #[rstest]
    fn test_selecting_preferences_blocks_esc(
      #[values(
        ActiveSonarrBlock::AddSeriesSelectMonitor,
        ActiveSonarrBlock::AddSeriesSelectSeriesType,
        ActiveSonarrBlock::AddSeriesSelectQualityProfile,
        ActiveSonarrBlock::AddSeriesSelectLanguageProfile,
        ActiveSonarrBlock::AddSeriesSelectRootFolder
      )]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::AddSeriesPrompt.into());
      app.push_navigation_stack(active_sonarr_block.into());

      AddSeriesHandler::new(ESC_KEY, &mut app, active_sonarr_block, None).handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::AddSeriesPrompt.into()
      );
    }
  }

  mod test_handle_key_char {
    use super::*;
    use crate::{
      models::{
        servarr_data::sonarr::{modals::AddSeriesModal, sonarr_data::ADD_SERIES_SELECTION_BLOCKS},
        BlockSelectionState,
      },
      network::sonarr_network::SonarrEvent,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn test_add_series_search_input_backspace() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.add_series_search = Some("Test".into());

      AddSeriesHandler::new(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveSonarrBlock::AddSeriesSearchInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .add_series_search
          .as_ref()
          .unwrap()
          .text,
        "Tes"
      );
    }

    #[test]
    fn test_add_series_tags_input_backspace() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.add_series_modal = Some(AddSeriesModal {
        tags: "Test".into(),
        ..AddSeriesModal::default()
      });

      AddSeriesHandler::new(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveSonarrBlock::AddSeriesTagsInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .add_series_modal
          .as_ref()
          .unwrap()
          .tags
          .text,
        "Tes"
      );
    }

    #[test]
    fn test_add_series_search_input_char_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.add_series_search = Some(HorizontallyScrollableText::default());

      AddSeriesHandler::new(
        Key::Char('a'),
        &mut app,
        ActiveSonarrBlock::AddSeriesSearchInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .add_series_search
          .as_ref()
          .unwrap()
          .text,
        "a"
      );
    }

    #[test]
    fn test_add_series_tags_input_char_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.add_series_modal = Some(AddSeriesModal::default());

      AddSeriesHandler::new(
        Key::Char('a'),
        &mut app,
        ActiveSonarrBlock::AddSeriesTagsInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .add_series_modal
          .as_ref()
          .unwrap()
          .tags
          .text,
        "a"
      );
    }

    #[test]
    fn test_add_series_confirm_prompt_prompt_confirmation_confirm() {
      let mut app = App::test_default();
      let mut add_series_modal = AddSeriesModal {
        use_season_folder: true,
        tags: "usenet, testing".into(),
        ..AddSeriesModal::default()
      };
      add_series_modal.root_folder_list.set_items(vec![
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
      add_series_modal.root_folder_list.state.select(Some(1));
      add_series_modal
        .quality_profile_list
        .set_items(vec!["HD - 1080p".to_owned()]);
      add_series_modal
        .language_profile_list
        .set_items(vec!["English".to_owned()]);
      add_series_modal
        .monitor_list
        .set_items(Vec::from_iter(SeriesMonitor::iter()));
      add_series_modal
        .series_type_list
        .set_items(Vec::from_iter(SeriesType::iter()));
      app.data.sonarr_data.add_series_modal = Some(add_series_modal);
      app.data.sonarr_data.quality_profile_map =
        BiMap::from_iter([(2222, "HD - 1080p".to_owned())]);
      app.data.sonarr_data.language_profiles_map = BiMap::from_iter([(2222, "English".to_owned())]);
      let mut add_searched_series = StatefulTable::default();
      add_searched_series.set_items(vec![add_series_search_result()]);
      app.data.sonarr_data.add_searched_series = Some(add_searched_series);
      let expected_add_series_body = AddSeriesBody {
        tvdb_id: 1234,
        title: "Test".to_owned(),
        monitored: true,
        root_folder_path: "/nfs2".to_owned(),
        quality_profile_id: 2222,
        language_profile_id: 2222,
        series_type: "standard".to_owned(),
        season_folder: true,
        tags: Vec::default(),
        tag_input_string: Some("usenet, testing".to_owned()),
        add_options: AddSeriesOptions {
          monitor: "all".to_owned(),
          search_for_cutoff_unmet_episodes: true,
          search_for_missing_episodes: true,
        },
      };
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::AddSeriesPrompt.into());
      app.data.sonarr_data.selected_block = BlockSelectionState::new(ADD_SERIES_SELECTION_BLOCKS);
      app
        .data
        .sonarr_data
        .selected_block
        .set_index(0, ADD_SERIES_SELECTION_BLOCKS.len() - 1);

      AddSeriesHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveSonarrBlock::AddSeriesPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
      assert_eq!(
        app.data.sonarr_data.prompt_confirm_action,
        Some(SonarrEvent::AddSeries(expected_add_series_body))
      );
      assert!(app.data.sonarr_data.add_series_modal.is_none());
    }
  }

  #[test]
  fn test_add_series_handler_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if ADD_SERIES_BLOCKS.contains(&active_sonarr_block) {
        assert!(AddSeriesHandler::accepts(active_sonarr_block));
      } else {
        assert!(!AddSeriesHandler::accepts(active_sonarr_block));
      }
    });
  }

  #[rstest]
  fn test_add_series_handler_ignore_alt_navigation(
    #[values(true, false)] should_ignore_quit_key: bool,
  ) {
    let mut app = App::test_default();
    app.should_ignore_quit_key = should_ignore_quit_key;
    let handler = AddSeriesHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::default(),
      None,
    );

    assert_eq!(handler.ignore_alt_navigation(), should_ignore_quit_key);
  }

  #[test]
  fn test_add_series_search_no_panic_on_none_search_result() {
    let mut app = App::test_default();
    app.data.sonarr_data.add_series_search = None;

    AddSeriesHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::AddSeriesSearchResults,
      None,
    )
    .handle();
  }

  #[test]
  fn test_build_add_series_body() {
    let mut app = App::test_default();
    let mut add_series_modal = AddSeriesModal {
      use_season_folder: true,
      tags: "usenet, testing".into(),
      ..AddSeriesModal::default()
    };
    add_series_modal.root_folder_list.set_items(vec![
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
    add_series_modal.root_folder_list.state.select(Some(1));
    add_series_modal
      .quality_profile_list
      .set_items(vec!["HD - 1080p".to_owned()]);
    add_series_modal
      .language_profile_list
      .set_items(vec!["English".to_owned()]);
    add_series_modal
      .monitor_list
      .set_items(Vec::from_iter(SeriesMonitor::iter()));
    add_series_modal
      .series_type_list
      .set_items(Vec::from_iter(SeriesType::iter()));
    app.data.sonarr_data.add_series_modal = Some(add_series_modal);
    app.data.sonarr_data.quality_profile_map = BiMap::from_iter([(2222, "HD - 1080p".to_owned())]);
    app.data.sonarr_data.language_profiles_map = BiMap::from_iter([(2222, "English".to_owned())]);
    let mut add_searched_series = StatefulTable::default();
    add_searched_series.set_items(vec![add_series_search_result()]);
    app.data.sonarr_data.add_searched_series = Some(add_searched_series);
    let expected_add_series_body = AddSeriesBody {
      tvdb_id: 1234,
      title: "Test".to_owned(),
      monitored: true,
      root_folder_path: "/nfs2".to_owned(),
      quality_profile_id: 2222,
      language_profile_id: 2222,
      series_type: "standard".to_owned(),
      season_folder: true,
      tags: Vec::default(),
      tag_input_string: Some("usenet, testing".to_owned()),
      add_options: AddSeriesOptions {
        monitor: "all".to_owned(),
        search_for_cutoff_unmet_episodes: true,
        search_for_missing_episodes: true,
      },
    };

    let add_series_body = AddSeriesHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::AddSeriesPrompt,
      None,
    )
    .build_add_series_body();

    assert_eq!(add_series_body, expected_add_series_body);
    assert!(app.data.sonarr_data.add_series_modal.is_none());
  }

  #[test]
  fn test_add_series_handler_is_not_ready_when_loading() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveSonarrBlock::Series.into());
    app.is_loading = true;

    let handler = AddSeriesHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::AddSeriesPrompt,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_add_series_handler_is_ready_when_not_loading() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveSonarrBlock::Series.into());
    app.is_loading = false;

    let handler = AddSeriesHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::AddSeriesPrompt,
      None,
    );

    assert!(handler.is_ready());
  }
}
