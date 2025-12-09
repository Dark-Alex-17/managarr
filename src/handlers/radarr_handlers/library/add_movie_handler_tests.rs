#[cfg(test)]
mod tests {
  use crate::assert_modal_absent;
  use crate::assert_modal_present;
  use crate::assert_navigation_pushed;
  use crate::handlers::radarr_handlers::radarr_handler_test_utils::utils::add_movie_search_result;
  use crate::models::stateful_table::StatefulTable;
  use pretty_assertions::assert_str_eq;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::event::Key;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::radarr_handlers::library::add_movie_handler::AddMovieHandler;
  use crate::handlers::radarr_handlers::radarr_handler_test_utils::utils::add_movie_body;
  use crate::handlers::radarr_handlers::radarr_handler_test_utils::utils::collection_movie;
  use crate::models::HorizontallyScrollableText;
  use crate::models::radarr_models::{AddMovieSearchResult, MinimumAvailability, MovieMonitor};
  use crate::models::servarr_data::radarr::modals::AddMovieModal;
  use crate::models::servarr_data::radarr::radarr_data::{ADD_MOVIE_BLOCKS, ActiveRadarrBlock};
  use crate::models::servarr_models::RootFolder;
  use bimap::BiMap;

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use strum::IntoEnumIterator;

    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::radarr::modals::AddMovieModal;
    use crate::models::servarr_data::radarr::radarr_data::ADD_MOVIE_SELECTION_BLOCKS;
    use crate::simple_stateful_iterable_vec;

    use super::*;

    #[rstest]
    fn test_add_movie_select_monitor_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let monitor_vec = Vec::from_iter(MovieMonitor::iter());
      let mut app = App::test_default();
      app.data.radarr_data.add_movie_modal = Some(AddMovieModal::default());
      app
        .data
        .radarr_data
        .add_movie_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .set_items(monitor_vec.clone());

      if key == Key::Up {
        for i in (0..monitor_vec.len()).rev() {
          AddMovieHandler::new(
            key,
            &mut app,
            ActiveRadarrBlock::AddMovieSelectMonitor,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .radarr_data
              .add_movie_modal
              .as_ref()
              .unwrap()
              .monitor_list
              .current_selection(),
            &monitor_vec[i]
          );
        }
      } else {
        for i in 0..monitor_vec.len() {
          AddMovieHandler::new(
            key,
            &mut app,
            ActiveRadarrBlock::AddMovieSelectMonitor,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .radarr_data
              .add_movie_modal
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
    fn test_add_movie_select_minimum_availability_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let minimum_availability_vec = Vec::from_iter(MinimumAvailability::iter());
      let mut app = App::test_default();
      app.data.radarr_data.add_movie_modal = Some(AddMovieModal::default());
      app
        .data
        .radarr_data
        .add_movie_modal
        .as_mut()
        .unwrap()
        .minimum_availability_list
        .set_items(minimum_availability_vec.clone());

      if key == Key::Up {
        for i in (0..minimum_availability_vec.len()).rev() {
          AddMovieHandler::new(
            key,
            &mut app,
            ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .radarr_data
              .add_movie_modal
              .as_ref()
              .unwrap()
              .minimum_availability_list
              .current_selection(),
            &minimum_availability_vec[i]
          );
        }
      } else {
        for i in 0..minimum_availability_vec.len() {
          AddMovieHandler::new(
            key,
            &mut app,
            ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .radarr_data
              .add_movie_modal
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
    fn test_add_movie_select_quality_profile_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.data.radarr_data.add_movie_modal = Some(AddMovieModal::default());
      app
        .data
        .radarr_data
        .add_movie_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .set_items(vec!["Test 1".to_owned(), "Test 2".to_owned()]);

      AddMovieHandler::new(
        key,
        &mut app,
        ActiveRadarrBlock::AddMovieSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .add_movie_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 2"
      );

      AddMovieHandler::new(
        key,
        &mut app,
        ActiveRadarrBlock::AddMovieSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .add_movie_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[rstest]
    fn test_add_movie_select_root_folder_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.data.radarr_data.add_movie_modal = Some(AddMovieModal::default());
      app
        .data
        .radarr_data
        .add_movie_modal
        .as_mut()
        .unwrap()
        .root_folder_list
        .set_items(simple_stateful_iterable_vec!(RootFolder, String, path));

      AddMovieHandler::new(
        key,
        &mut app,
        ActiveRadarrBlock::AddMovieSelectRootFolder,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .add_movie_modal
          .as_ref()
          .unwrap()
          .root_folder_list
          .current_selection()
          .path,
        "Test 2"
      );

      AddMovieHandler::new(
        key,
        &mut app,
        ActiveRadarrBlock::AddMovieSelectRootFolder,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .add_movie_modal
          .as_ref()
          .unwrap()
          .root_folder_list
          .current_selection()
          .path,
        "Test 1"
      );
    }

    #[rstest]
    fn test_add_movie_prompt_scroll(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::test_default();
      app.data.radarr_data.selected_block = BlockSelectionState::new(ADD_MOVIE_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.down();

      AddMovieHandler::new(key, &mut app, ActiveRadarrBlock::AddMoviePrompt, None).handle();

      if key == Key::Up {
        assert_eq!(
          app.data.radarr_data.selected_block.get_active_block(),
          ActiveRadarrBlock::AddMovieSelectRootFolder
        );
      } else {
        assert_eq!(
          app.data.radarr_data.selected_block.get_active_block(),
          ActiveRadarrBlock::AddMovieSelectMinimumAvailability
        );
      }
    }

    #[rstest]
    fn test_add_movie_prompt_scroll_no_op_when_not_ready(#[values(Key::Up, Key::Down)] key: Key) {
      let mut app = App::test_default();
      app.is_loading = true;
      app.data.radarr_data.selected_block = BlockSelectionState::new(ADD_MOVIE_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.down();

      AddMovieHandler::new(key, &mut app, ActiveRadarrBlock::AddMoviePrompt, None).handle();

      assert_eq!(
        app.data.radarr_data.selected_block.get_active_block(),
        ActiveRadarrBlock::AddMovieSelectMonitor
      );
    }
  }

  mod test_handle_home_end {
    use std::sync::atomic::Ordering;

    use strum::IntoEnumIterator;

    use crate::extended_stateful_iterable_vec;
    use crate::models::servarr_data::radarr::modals::AddMovieModal;

    use super::*;

    #[test]
    fn test_add_movie_select_monitor_home_end() {
      let monitor_vec = Vec::from_iter(MovieMonitor::iter());
      let mut app = App::test_default();
      app.data.radarr_data.add_movie_modal = Some(AddMovieModal::default());
      app
        .data
        .radarr_data
        .add_movie_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .set_items(monitor_vec.clone());

      AddMovieHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveRadarrBlock::AddMovieSelectMonitor,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .add_movie_modal
          .as_ref()
          .unwrap()
          .monitor_list
          .current_selection(),
        &monitor_vec[monitor_vec.len() - 1]
      );

      AddMovieHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveRadarrBlock::AddMovieSelectMonitor,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .add_movie_modal
          .as_ref()
          .unwrap()
          .monitor_list
          .current_selection(),
        &monitor_vec[0]
      );
    }

    #[test]
    fn test_add_movie_select_minimum_availability_home_end() {
      let minimum_availability_vec = Vec::from_iter(MinimumAvailability::iter());
      let mut app = App::test_default();
      app.data.radarr_data.add_movie_modal = Some(AddMovieModal::default());
      app
        .data
        .radarr_data
        .add_movie_modal
        .as_mut()
        .unwrap()
        .minimum_availability_list
        .set_items(minimum_availability_vec.clone());

      AddMovieHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .add_movie_modal
          .as_ref()
          .unwrap()
          .minimum_availability_list
          .current_selection(),
        &minimum_availability_vec[minimum_availability_vec.len() - 1]
      );

      AddMovieHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .add_movie_modal
          .as_ref()
          .unwrap()
          .minimum_availability_list
          .current_selection(),
        &minimum_availability_vec[0]
      );
    }

    #[test]
    fn test_add_movie_select_quality_profile_home_end() {
      let mut app = App::test_default();
      app.data.radarr_data.add_movie_modal = Some(AddMovieModal::default());
      app
        .data
        .radarr_data
        .add_movie_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .set_items(vec![
          "Test 1".to_owned(),
          "Test 2".to_owned(),
          "Test 3".to_owned(),
        ]);

      AddMovieHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveRadarrBlock::AddMovieSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .add_movie_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 3"
      );

      AddMovieHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveRadarrBlock::AddMovieSelectQualityProfile,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .add_movie_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "Test 1"
      );
    }

    #[test]
    fn test_add_movie_select_root_folder_home_end() {
      let mut app = App::test_default();
      app.data.radarr_data.add_movie_modal = Some(AddMovieModal::default());
      app
        .data
        .radarr_data
        .add_movie_modal
        .as_mut()
        .unwrap()
        .root_folder_list
        .set_items(extended_stateful_iterable_vec!(RootFolder, String, path));

      AddMovieHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveRadarrBlock::AddMovieSelectRootFolder,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .add_movie_modal
          .as_ref()
          .unwrap()
          .root_folder_list
          .current_selection()
          .path,
        "Test 3"
      );

      AddMovieHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveRadarrBlock::AddMovieSelectRootFolder,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .add_movie_modal
          .as_ref()
          .unwrap()
          .root_folder_list
          .current_selection()
          .path,
        "Test 1"
      );
    }

    #[test]
    fn test_add_movie_search_input_home_end_keys() {
      let mut app = App::test_default();
      app.data.radarr_data.add_movie_search = Some("Test".into());

      AddMovieHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveRadarrBlock::AddMovieSearchInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .add_movie_search
          .as_ref()
          .unwrap()
          .offset
          .load(Ordering::SeqCst),
        4
      );

      AddMovieHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveRadarrBlock::AddMovieSearchInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .add_movie_search
          .as_ref()
          .unwrap()
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }

    #[test]
    fn test_add_movie_tags_input_home_end_keys() {
      let mut app = App::test_default();
      app.data.radarr_data.add_movie_modal = Some(AddMovieModal {
        tags: "Test".into(),
        ..AddMovieModal::default()
      });

      AddMovieHandler::new(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveRadarrBlock::AddMovieTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .add_movie_modal
          .as_ref()
          .unwrap()
          .tags
          .offset
          .load(Ordering::SeqCst),
        4
      );

      AddMovieHandler::new(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveRadarrBlock::AddMovieTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .add_movie_modal
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

    use crate::models::servarr_data::radarr::modals::AddMovieModal;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_left_right_prompt_toggle(#[values(Key::Left, Key::Right)] key: Key) {
      let mut app = App::test_default();

      AddMovieHandler::new(key, &mut app, ActiveRadarrBlock::AddMoviePrompt, None).handle();

      assert!(app.data.radarr_data.prompt_confirm);

      AddMovieHandler::new(key, &mut app, ActiveRadarrBlock::AddMoviePrompt, None).handle();

      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[test]
    fn test_add_movie_search_input_left_right_keys() {
      let mut app = App::test_default();
      app.data.radarr_data.add_movie_search = Some("Test".into());

      AddMovieHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveRadarrBlock::AddMovieSearchInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .add_movie_search
          .as_ref()
          .unwrap()
          .offset
          .load(Ordering::SeqCst),
        1
      );

      AddMovieHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveRadarrBlock::AddMovieSearchInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .add_movie_search
          .as_ref()
          .unwrap()
          .offset
          .load(Ordering::SeqCst),
        0
      );
    }

    #[test]
    fn test_add_movie_tags_input_left_right_keys() {
      let mut app = App::test_default();
      app.data.radarr_data.add_movie_modal = Some(AddMovieModal {
        tags: "Test".into(),
        ..AddMovieModal::default()
      });

      AddMovieHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveRadarrBlock::AddMovieTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .add_movie_modal
          .as_ref()
          .unwrap()
          .tags
          .offset
          .load(Ordering::SeqCst),
        1
      );

      AddMovieHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveRadarrBlock::AddMovieTagsInput,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .add_movie_modal
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
    use crate::handlers::radarr_handlers::radarr_handler_test_utils::utils::{
      add_movie_body, add_movie_search_result, collection_movie,
    };
    use crate::models::BlockSelectionState;
    use crate::models::radarr_models::Movie;
    use crate::models::servarr_data::radarr::modals::AddMovieModal;
    use crate::models::servarr_data::radarr::radarr_data::ADD_MOVIE_SELECTION_BLOCKS;
    use crate::models::stateful_table::StatefulTable;
    use crate::network::radarr_network::RadarrEvent;
    use bimap::BiMap;
    use pretty_assertions::{assert_eq, assert_str_eq};
    use rstest::rstest;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_add_movie_search_input_submit() {
      let mut app = App::test_default();
      app.ignore_special_keys_for_textbox_input = true;
      app.data.radarr_data.add_movie_search = Some("test".into());

      AddMovieHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::AddMovieSearchInput,
        None,
      )
      .handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::AddMovieSearchResults.into()
      );
    }

    #[test]
    fn test_add_movie_search_input_submit_noop_on_empty_search() {
      let mut app = App::test_default();
      app.data.radarr_data.add_movie_search = Some(HorizontallyScrollableText::default());
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieSearchInput.into());
      app.ignore_special_keys_for_textbox_input = true;

      AddMovieHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::AddMovieSearchInput,
        None,
      )
      .handle();

      assert!(app.ignore_special_keys_for_textbox_input);
      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::AddMovieSearchInput.into()
      );
    }

    #[test]
    fn test_add_movie_search_results_submit() {
      let mut app = App::test_default();
      let mut add_searched_movies = StatefulTable::default();
      add_searched_movies.set_items(vec![AddMovieSearchResult::default()]);
      app.data.radarr_data.add_searched_movies = Some(add_searched_movies);
      app.data.radarr_data.quality_profile_map =
        BiMap::from_iter([(1, "B - Test 2".to_owned()), (0, "A - Test 1".to_owned())]);

      AddMovieHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::AddMovieSearchResults,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::AddMoviePrompt.into()
      );
      assert_eq!(
        app.data.radarr_data.selected_block.get_active_block(),
        ActiveRadarrBlock::AddMovieSelectRootFolder
      );
      assert_modal_present!(app.data.radarr_data.add_movie_modal);
      assert!(
        !app
          .data
          .radarr_data
          .add_movie_modal
          .as_ref()
          .unwrap()
          .monitor_list
          .items
          .is_empty()
      );
      assert!(
        !app
          .data
          .radarr_data
          .add_movie_modal
          .as_ref()
          .unwrap()
          .minimum_availability_list
          .items
          .is_empty()
      );
      assert!(
        !app
          .data
          .radarr_data
          .add_movie_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .items
          .is_empty()
      );
      assert_str_eq!(
        app
          .data
          .radarr_data
          .add_movie_modal
          .as_ref()
          .unwrap()
          .quality_profile_list
          .current_selection(),
        "A - Test 1"
      );
    }

    #[test]
    fn test_add_movie_search_results_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieSearchResults.into());
      let mut add_searched_movies = StatefulTable::default();
      add_searched_movies.set_items(vec![AddMovieSearchResult::default()]);

      AddMovieHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::AddMovieSearchResults,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::AddMovieSearchResults.into()
      );
      assert_modal_absent!(app.data.radarr_data.add_movie_modal);
    }

    #[test]
    fn test_add_movie_search_results_submit_does_nothing_on_empty_table() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieSearchResults.into());
      AddMovieHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::AddMovieSearchResults,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::AddMovieSearchResults.into()
      );
    }

    #[test]
    fn test_add_movie_search_results_submit_movie_already_in_library() {
      let mut app = App::test_default();
      let mut add_searched_movies = StatefulTable::default();
      add_searched_movies.set_items(vec![AddMovieSearchResult::default()]);
      app.data.radarr_data.add_searched_movies = Some(add_searched_movies);
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);

      AddMovieHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::AddMovieSearchResults,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveRadarrBlock::AddMovieAlreadyInLibrary.into()
      );
    }

    #[test]
    fn test_add_movie_prompt_prompt_decline_submit() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());
      app.data.radarr_data.selected_block = BlockSelectionState::new(ADD_MOVIE_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(0, ADD_MOVIE_SELECTION_BLOCKS.len() - 1);

      AddMovieHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::AddMoviePrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveRadarrBlock::Movies.into());
      assert_none!(app.data.radarr_data.prompt_confirm_action);
    }

    #[rstest]
    fn test_add_movie_confirm_prompt_prompt_confirmation_submit(
      #[values(true, false)] movie_details_context: bool,
    ) {
      let mut app = App::test_default();
      app.data.radarr_data.add_movie_modal = Some(AddMovieModal::default());
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());
      app.data.radarr_data.prompt_confirm = true;
      app.data.radarr_data.selected_block = BlockSelectionState::new(ADD_MOVIE_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(0, ADD_MOVIE_SELECTION_BLOCKS.len() - 1);
      let mut add_movie_modal = AddMovieModal {
        tags: "usenet, testing".into(),
        ..AddMovieModal::default()
      };
      add_movie_modal.root_folder_list.set_items(vec![
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
      add_movie_modal.root_folder_list.state.select(Some(1));
      add_movie_modal
        .quality_profile_list
        .set_items(vec!["HD - 1080p".to_owned()]);
      add_movie_modal
        .monitor_list
        .set_items(Vec::from_iter(MovieMonitor::iter()));
      add_movie_modal
        .minimum_availability_list
        .set_items(Vec::from_iter(MinimumAvailability::iter()));
      app.data.radarr_data.add_movie_modal = Some(add_movie_modal);
      app.data.radarr_data.quality_profile_map =
        BiMap::from_iter([(2222, "HD - 1080p".to_owned())]);
      let context = if movie_details_context {
        app
          .data
          .radarr_data
          .collection_movies
          .set_items(vec![collection_movie()]);
        Some(ActiveRadarrBlock::CollectionDetails)
      } else {
        let mut add_searched_movies = StatefulTable::default();
        add_searched_movies.set_items(vec![add_movie_search_result()]);
        app.data.radarr_data.add_searched_movies = Some(add_searched_movies);
        None
      };

      AddMovieHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::AddMoviePrompt,
        context,
      )
      .handle();

      assert_navigation_popped!(app, ActiveRadarrBlock::Movies.into());
      assert_some_eq_x!(
        &app.data.radarr_data.prompt_confirm_action,
        &RadarrEvent::AddMovie(add_movie_body())
      );
      assert_modal_absent!(app.data.radarr_data.add_movie_modal);
    }

    #[rstest]
    #[case(ActiveRadarrBlock::AddMovieSelectRootFolder, 0)]
    #[case(ActiveRadarrBlock::AddMovieSelectMonitor, 1)]
    #[case(ActiveRadarrBlock::AddMovieSelectMinimumAvailability, 2)]
    #[case(ActiveRadarrBlock::AddMovieSelectQualityProfile, 3)]
    #[case(ActiveRadarrBlock::AddMovieTagsInput, 4)]
    fn test_add_movie_prompt_selected_block_submit(
      #[case] selected_block: ActiveRadarrBlock,
      #[case] y_index: usize,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(
        (
          ActiveRadarrBlock::AddMoviePrompt,
          Some(ActiveRadarrBlock::CollectionDetails),
        )
          .into(),
      );
      app.data.radarr_data.selected_block = BlockSelectionState::new(ADD_MOVIE_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.set_index(0, y_index);

      AddMovieHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::AddMoviePrompt,
        Some(ActiveRadarrBlock::CollectionDetails),
      )
      .handle();

      assert_navigation_pushed!(
        app,
        (selected_block, Some(ActiveRadarrBlock::CollectionDetails)).into()
      );
      assert_none!(app.data.radarr_data.prompt_confirm_action);

      if selected_block == ActiveRadarrBlock::AddMovieTagsInput {
        assert!(app.ignore_special_keys_for_textbox_input);
      }
    }

    #[rstest]
    fn test_add_movie_prompt_selecting_preferences_blocks_submit(
      #[values(
        ActiveRadarrBlock::AddMovieSelectMonitor,
        ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
        ActiveRadarrBlock::AddMovieSelectQualityProfile,
        ActiveRadarrBlock::AddMovieSelectRootFolder,
        ActiveRadarrBlock::AddMovieTagsInput
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());
      app.push_navigation_stack(active_radarr_block.into());

      AddMovieHandler::new(
        SUBMIT_KEY,
        &mut app,
        active_radarr_block,
        Some(ActiveRadarrBlock::CollectionDetails),
      )
      .handle();

      assert_navigation_popped!(app, ActiveRadarrBlock::AddMoviePrompt.into());

      if active_radarr_block == ActiveRadarrBlock::AddMovieTagsInput {
        assert!(!app.ignore_special_keys_for_textbox_input);
      }
    }
  }

  mod test_handle_esc {
    use rstest::rstest;

    use crate::models::servarr_data::radarr::modals::AddMovieModal;
    use crate::models::servarr_data::radarr::radarr_data::radarr_test_utils::utils::create_test_radarr_data;
    use crate::models::stateful_table::StatefulTable;
    use crate::{assert_navigation_popped, simple_stateful_iterable_vec};

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_add_movie_search_input_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.data.radarr_data = create_test_radarr_data();
      app.ignore_special_keys_for_textbox_input = true;
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieSearchInput.into());

      AddMovieHandler::new(
        ESC_KEY,
        &mut app,
        ActiveRadarrBlock::AddMovieSearchInput,
        None,
      )
      .handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert_navigation_popped!(app, ActiveRadarrBlock::Movies.into());
      assert_none!(app.data.radarr_data.add_movie_search);
    }

    #[test]
    fn test_add_movie_input_esc() {
      let mut app = App::test_default();
      app.data.radarr_data = create_test_radarr_data();
      app.ignore_special_keys_for_textbox_input = true;
      app.push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieTagsInput.into());

      AddMovieHandler::new(
        ESC_KEY,
        &mut app,
        ActiveRadarrBlock::AddMovieTagsInput,
        None,
      )
      .handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert_navigation_popped!(app, ActiveRadarrBlock::AddMoviePrompt.into());
    }

    #[rstest]
    fn test_add_movie_search_results_esc(
      #[values(
        ActiveRadarrBlock::AddMovieSearchResults,
        ActiveRadarrBlock::AddMovieEmptySearchResults
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieSearchInput.into());
      app.push_navigation_stack(active_radarr_block.into());
      let mut add_searched_movies = StatefulTable::default();
      add_searched_movies.set_items(simple_stateful_iterable_vec!(
        AddMovieSearchResult,
        HorizontallyScrollableText
      ));
      app.data.radarr_data.add_searched_movies = Some(add_searched_movies);

      AddMovieHandler::new(ESC_KEY, &mut app, active_radarr_block, None).handle();

      assert_navigation_popped!(app, ActiveRadarrBlock::AddMovieSearchInput.into());
      assert_modal_absent!(app.data.radarr_data.add_searched_movies);
      assert!(app.ignore_special_keys_for_textbox_input);
    }

    #[test]
    fn test_add_movie_already_in_library_esc() {
      let mut app = App::test_default();
      app.data.radarr_data = create_test_radarr_data();
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieSearchResults.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieAlreadyInLibrary.into());

      AddMovieHandler::new(
        ESC_KEY,
        &mut app,
        ActiveRadarrBlock::AddMovieAlreadyInLibrary,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveRadarrBlock::AddMovieSearchResults.into());
    }

    #[test]
    fn test_add_movie_prompt_esc() {
      let mut app = App::test_default();
      app.data.radarr_data.add_movie_modal = Some(AddMovieModal::default());
      app.data.radarr_data = create_test_radarr_data();
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieSearchResults.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());

      AddMovieHandler::new(ESC_KEY, &mut app, ActiveRadarrBlock::AddMoviePrompt, None).handle();

      assert!(!app.data.radarr_data.prompt_confirm);
      assert_navigation_popped!(app, ActiveRadarrBlock::AddMovieSearchResults.into());
      assert_modal_absent!(app.data.radarr_data.add_movie_modal);
    }

    #[test]
    fn test_add_movie_tags_input_esc() {
      let mut app = App::test_default();
      app.data.radarr_data = create_test_radarr_data();
      app.ignore_special_keys_for_textbox_input = true;
      app.push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieTagsInput.into());

      AddMovieHandler::new(
        ESC_KEY,
        &mut app,
        ActiveRadarrBlock::AddMovieTagsInput,
        None,
      )
      .handle();

      assert!(!app.ignore_special_keys_for_textbox_input);
      assert_navigation_popped!(app, ActiveRadarrBlock::AddMoviePrompt.into());
    }

    #[rstest]
    fn test_selecting_preferences_blocks_esc(
      #[values(
        ActiveRadarrBlock::AddMovieSelectMonitor,
        ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
        ActiveRadarrBlock::AddMovieSelectQualityProfile,
        ActiveRadarrBlock::AddMovieSelectRootFolder
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(
        (
          ActiveRadarrBlock::AddMoviePrompt,
          Some(ActiveRadarrBlock::CollectionDetails),
        )
          .into(),
      );
      app.push_navigation_stack(
        (
          active_radarr_block,
          Some(ActiveRadarrBlock::CollectionDetails),
        )
          .into(),
      );

      AddMovieHandler::new(
        ESC_KEY,
        &mut app,
        active_radarr_block,
        Some(ActiveRadarrBlock::CollectionDetails),
      )
      .handle();

      assert_navigation_popped!(
        app,
        (
          ActiveRadarrBlock::AddMoviePrompt,
          Some(ActiveRadarrBlock::CollectionDetails),
        )
          .into()
      );
    }
  }

  mod test_handle_key_char {
    use bimap::BiMap;
    use rstest::rstest;

    use super::*;
    use crate::{
      assert_navigation_popped,
      handlers::radarr_handlers::radarr_handler_test_utils::utils::{
        add_movie_body, add_movie_search_result, collection_movie,
      },
      models::{
        BlockSelectionState,
        servarr_data::radarr::{modals::AddMovieModal, radarr_data::ADD_MOVIE_SELECTION_BLOCKS},
        stateful_table::StatefulTable,
      },
      network::radarr_network::RadarrEvent,
    };

    #[test]
    fn test_add_movie_search_input_backspace() {
      let mut app = App::test_default();
      app.data.radarr_data.add_movie_search = Some("Test".into());

      AddMovieHandler::new(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveRadarrBlock::AddMovieSearchInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.radarr_data.add_movie_search.as_ref().unwrap().text,
        "Tes"
      );
    }

    #[test]
    fn test_add_movie_tags_input_backspace() {
      let mut app = App::test_default();
      app.data.radarr_data.add_movie_modal = Some(AddMovieModal {
        tags: "Test".into(),
        ..AddMovieModal::default()
      });

      AddMovieHandler::new(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveRadarrBlock::AddMovieTagsInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .add_movie_modal
          .as_ref()
          .unwrap()
          .tags
          .text,
        "Tes"
      );
    }

    #[test]
    fn test_add_movie_search_input_char_key() {
      let mut app = App::test_default();
      app.data.radarr_data.add_movie_search = Some(HorizontallyScrollableText::default());

      AddMovieHandler::new(
        Key::Char('a'),
        &mut app,
        ActiveRadarrBlock::AddMovieSearchInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.radarr_data.add_movie_search.as_ref().unwrap().text,
        "a"
      );
    }

    #[test]
    fn test_add_movie_tags_input_char_key() {
      let mut app = App::test_default();
      app.data.radarr_data.add_movie_modal = Some(AddMovieModal::default());

      AddMovieHandler::new(
        Key::Char('a'),
        &mut app,
        ActiveRadarrBlock::AddMovieTagsInput,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .add_movie_modal
          .as_ref()
          .unwrap()
          .tags
          .text,
        "a"
      );
    }

    #[rstest]
    fn test_add_movie_confirm_prompt_prompt_confirmation_confirm(
      #[values(true, false)] movie_details_context: bool,
    ) {
      let mut app = App::test_default();
      app.data.radarr_data.add_movie_modal = Some(AddMovieModal::default());
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());
      app.data.radarr_data.selected_block = BlockSelectionState::new(ADD_MOVIE_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(0, ADD_MOVIE_SELECTION_BLOCKS.len() - 1);
      let mut add_movie_modal = AddMovieModal {
        tags: "usenet, testing".into(),
        ..AddMovieModal::default()
      };
      add_movie_modal.root_folder_list.set_items(vec![
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
      add_movie_modal.root_folder_list.state.select(Some(1));
      add_movie_modal
        .quality_profile_list
        .set_items(vec!["HD - 1080p".to_owned()]);
      add_movie_modal
        .monitor_list
        .set_items(Vec::from_iter(MovieMonitor::iter()));
      add_movie_modal
        .minimum_availability_list
        .set_items(Vec::from_iter(MinimumAvailability::iter()));
      app.data.radarr_data.add_movie_modal = Some(add_movie_modal);
      app.data.radarr_data.quality_profile_map =
        BiMap::from_iter([(2222, "HD - 1080p".to_owned())]);
      let context = if movie_details_context {
        app
          .data
          .radarr_data
          .collection_movies
          .set_items(vec![collection_movie()]);
        Some(ActiveRadarrBlock::CollectionDetails)
      } else {
        let mut add_searched_movies = StatefulTable::default();
        add_searched_movies.set_items(vec![add_movie_search_result()]);
        app.data.radarr_data.add_searched_movies = Some(add_searched_movies);
        None
      };

      AddMovieHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveRadarrBlock::AddMoviePrompt,
        context,
      )
      .handle();

      assert_navigation_popped!(app, ActiveRadarrBlock::Movies.into());
      assert_some_eq_x!(
        &app.data.radarr_data.prompt_confirm_action,
        &RadarrEvent::AddMovie(add_movie_body())
      );
      assert_modal_absent!(app.data.radarr_data.add_movie_modal);
    }
  }

  #[test]
  fn test_add_movie_handler_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if ADD_MOVIE_BLOCKS.contains(&active_radarr_block) {
        assert!(AddMovieHandler::accepts(active_radarr_block));
      } else {
        assert!(!AddMovieHandler::accepts(active_radarr_block));
      }
    });
  }

  #[rstest]
  fn test_add_movie_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = AddMovieHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::default(),
      None,
    );

    assert_eq!(
      handler.ignore_special_keys(),
      ignore_special_keys_for_textbox_input
    );
  }

  #[test]
  fn test_add_movie_search_no_panic_on_none_search_result() {
    let mut app = App::test_default();
    app.data.radarr_data.add_searched_movies = None;

    AddMovieHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::AddMovieSearchResults,
      None,
    )
    .handle();
  }

  #[rstest]
  fn test_build_add_movie_body(#[values(true, false)] movie_details_context: bool) {
    let mut app = App::test_default();
    let mut add_movie_modal = AddMovieModal {
      tags: "usenet, testing".into(),
      ..AddMovieModal::default()
    };
    add_movie_modal.root_folder_list.set_items(vec![
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
    add_movie_modal.root_folder_list.state.select(Some(1));
    add_movie_modal
      .quality_profile_list
      .set_items(vec!["HD - 1080p".to_owned()]);
    add_movie_modal
      .monitor_list
      .set_items(Vec::from_iter(MovieMonitor::iter()));
    add_movie_modal
      .minimum_availability_list
      .set_items(Vec::from_iter(MinimumAvailability::iter()));
    app.data.radarr_data.add_movie_modal = Some(add_movie_modal);
    app.data.radarr_data.quality_profile_map = BiMap::from_iter([(2222, "HD - 1080p".to_owned())]);
    let context = if movie_details_context {
      app
        .data
        .radarr_data
        .collection_movies
        .set_items(vec![collection_movie()]);
      Some(ActiveRadarrBlock::CollectionDetails)
    } else {
      let mut add_searched_movies = StatefulTable::default();
      add_searched_movies.set_items(vec![add_movie_search_result()]);
      app.data.radarr_data.add_searched_movies = Some(add_searched_movies);
      None
    };

    let actual_add_movie_body = AddMovieHandler::new(
      DEFAULT_KEYBINDINGS.confirm.key,
      &mut app,
      ActiveRadarrBlock::AddMoviePrompt,
      context,
    )
    .build_add_movie_body();

    assert_eq!(actual_add_movie_body, add_movie_body());
  }

  #[test]
  fn test_add_movie_handler_is_not_ready_when_loading() {
    let mut app = App::test_default();
    app.is_loading = true;

    let handler = AddMovieHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::AddMoviePrompt,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_add_movie_handler_is_ready_when_not_loading() {
    let mut app = App::test_default();
    app.is_loading = false;

    let handler = AddMovieHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::AddMoviePrompt,
      None,
    );

    assert!(handler.is_ready());
  }
}
