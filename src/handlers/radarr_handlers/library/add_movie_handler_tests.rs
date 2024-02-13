#[cfg(test)]
mod tests {
  use pretty_assertions::assert_str_eq;
  use strum::IntoEnumIterator;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::radarr_handlers::library::add_movie_handler::AddMovieHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::radarr_models::{
    AddMovieSearchResult, MinimumAvailability, Monitor, RootFolder,
  };
  use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, ADD_MOVIE_BLOCKS};
  use crate::models::HorizontallyScrollableText;

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use strum::IntoEnumIterator;

    use crate::models::servarr_data::radarr::modals::AddMovieModal;
    use crate::models::servarr_data::radarr::radarr_data::ADD_MOVIE_SELECTION_BLOCKS;
    use crate::models::stateful_table::StatefulTable;
    use crate::models::BlockSelectionState;
    use crate::simple_stateful_iterable_vec;

    use super::*;

    #[rstest]
    fn test_add_movie_search_results_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::default();
      let mut add_searched_movies = StatefulTable::default();
      add_searched_movies.set_items(simple_stateful_iterable_vec!(
        AddMovieSearchResult,
        HorizontallyScrollableText
      ));
      app.data.radarr_data.add_searched_movies = Some(add_searched_movies);

      AddMovieHandler::with(
        &key,
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchResults,
        &None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .add_searched_movies
          .as_ref()
          .unwrap()
          .current_selection()
          .title
          .to_string(),
        "Test 2"
      );

      AddMovieHandler::with(
        &key,
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchResults,
        &None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .add_searched_movies
          .as_ref()
          .unwrap()
          .current_selection()
          .title
          .to_string(),
        "Test 1"
      );
    }

    #[rstest]
    fn test_add_movie_select_monitor_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let monitor_vec = Vec::from_iter(Monitor::iter());
      let mut app = App::default();
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
          AddMovieHandler::with(
            &key,
            &mut app,
            &ActiveRadarrBlock::AddMovieSelectMonitor,
            &None,
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
          AddMovieHandler::with(
            &key,
            &mut app,
            &ActiveRadarrBlock::AddMovieSelectMonitor,
            &None,
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
      let mut app = App::default();
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
          AddMovieHandler::with(
            &key,
            &mut app,
            &ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
            &None,
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
          AddMovieHandler::with(
            &key,
            &mut app,
            &ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
            &None,
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
      let mut app = App::default();
      app.data.radarr_data.add_movie_modal = Some(AddMovieModal::default());
      app
        .data
        .radarr_data
        .add_movie_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .set_items(vec!["Test 1".to_owned(), "Test 2".to_owned()]);

      AddMovieHandler::with(
        &key,
        &mut app,
        &ActiveRadarrBlock::AddMovieSelectQualityProfile,
        &None,
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

      AddMovieHandler::with(
        &key,
        &mut app,
        &ActiveRadarrBlock::AddMovieSelectQualityProfile,
        &None,
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
      let mut app = App::default();
      app.data.radarr_data.add_movie_modal = Some(AddMovieModal::default());
      app
        .data
        .radarr_data
        .add_movie_modal
        .as_mut()
        .unwrap()
        .root_folder_list
        .set_items(simple_stateful_iterable_vec!(RootFolder, String, path));

      AddMovieHandler::with(
        &key,
        &mut app,
        &ActiveRadarrBlock::AddMovieSelectRootFolder,
        &None,
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

      AddMovieHandler::with(
        &key,
        &mut app,
        &ActiveRadarrBlock::AddMovieSelectRootFolder,
        &None,
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
      let mut app = App::default();
      app.data.radarr_data.selected_block = BlockSelectionState::new(&ADD_MOVIE_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.next();

      AddMovieHandler::with(&key, &mut app, &ActiveRadarrBlock::AddMoviePrompt, &None).handle();

      if key == Key::Up {
        assert_eq!(
          app.data.radarr_data.selected_block.get_active_block(),
          &ActiveRadarrBlock::AddMovieSelectRootFolder
        );
      } else {
        assert_eq!(
          app.data.radarr_data.selected_block.get_active_block(),
          &ActiveRadarrBlock::AddMovieSelectMinimumAvailability
        );
      }
    }
  }

  mod test_handle_home_end {
    use strum::IntoEnumIterator;

    use crate::extended_stateful_iterable_vec;
    use crate::models::servarr_data::radarr::modals::AddMovieModal;
    use crate::models::stateful_table::StatefulTable;

    use super::*;

    #[test]
    fn test_add_movie_search_results_home_end() {
      let mut app = App::default();
      let mut add_searched_movies = StatefulTable::default();
      add_searched_movies.set_items(extended_stateful_iterable_vec!(
        AddMovieSearchResult,
        HorizontallyScrollableText
      ));
      app.data.radarr_data.add_searched_movies = Some(add_searched_movies);

      AddMovieHandler::with(
        &DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchResults,
        &None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .add_searched_movies
          .as_ref()
          .unwrap()
          .current_selection()
          .title
          .to_string(),
        "Test 3"
      );

      AddMovieHandler::with(
        &DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchResults,
        &None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .add_searched_movies
          .as_ref()
          .unwrap()
          .current_selection()
          .title
          .to_string(),
        "Test 1"
      );
    }

    #[test]
    fn test_add_movie_select_monitor_home_end() {
      let monitor_vec = Vec::from_iter(Monitor::iter());
      let mut app = App::default();
      app.data.radarr_data.add_movie_modal = Some(AddMovieModal::default());
      app
        .data
        .radarr_data
        .add_movie_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .set_items(monitor_vec.clone());

      AddMovieHandler::with(
        &DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        &ActiveRadarrBlock::AddMovieSelectMonitor,
        &None,
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

      AddMovieHandler::with(
        &DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        &ActiveRadarrBlock::AddMovieSelectMonitor,
        &None,
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
      let mut app = App::default();
      app.data.radarr_data.add_movie_modal = Some(AddMovieModal::default());
      app
        .data
        .radarr_data
        .add_movie_modal
        .as_mut()
        .unwrap()
        .minimum_availability_list
        .set_items(minimum_availability_vec.clone());

      AddMovieHandler::with(
        &DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        &ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
        &None,
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

      AddMovieHandler::with(
        &DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        &ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
        &None,
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
      let mut app = App::default();
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

      AddMovieHandler::with(
        &DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        &ActiveRadarrBlock::AddMovieSelectQualityProfile,
        &None,
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

      AddMovieHandler::with(
        &DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        &ActiveRadarrBlock::AddMovieSelectQualityProfile,
        &None,
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
      let mut app = App::default();
      app.data.radarr_data.add_movie_modal = Some(AddMovieModal::default());
      app
        .data
        .radarr_data
        .add_movie_modal
        .as_mut()
        .unwrap()
        .root_folder_list
        .set_items(extended_stateful_iterable_vec!(RootFolder, String, path));

      AddMovieHandler::with(
        &DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        &ActiveRadarrBlock::AddMovieSelectRootFolder,
        &None,
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

      AddMovieHandler::with(
        &DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        &ActiveRadarrBlock::AddMovieSelectRootFolder,
        &None,
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
      let mut app = App::default();
      app.data.radarr_data.add_movie_search = Some("Test".into());

      AddMovieHandler::with(
        &DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchInput,
        &None,
      )
      .handle();

      assert_eq!(
        *app
          .data
          .radarr_data
          .add_movie_search
          .as_ref()
          .unwrap()
          .offset
          .borrow(),
        4
      );

      AddMovieHandler::with(
        &DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchInput,
        &None,
      )
      .handle();

      assert_eq!(
        *app
          .data
          .radarr_data
          .add_movie_search
          .as_ref()
          .unwrap()
          .offset
          .borrow(),
        0
      );
    }

    #[test]
    fn test_add_movie_tags_input_home_end_keys() {
      let mut app = App::default();
      app.data.radarr_data.add_movie_modal = Some(AddMovieModal {
        tags: "Test".into(),
        ..AddMovieModal::default()
      });

      AddMovieHandler::with(
        &DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        &ActiveRadarrBlock::AddMovieTagsInput,
        &None,
      )
      .handle();

      assert_eq!(
        *app
          .data
          .radarr_data
          .add_movie_modal
          .as_ref()
          .unwrap()
          .tags
          .offset
          .borrow(),
        4
      );

      AddMovieHandler::with(
        &DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        &ActiveRadarrBlock::AddMovieTagsInput,
        &None,
      )
      .handle();

      assert_eq!(
        *app
          .data
          .radarr_data
          .add_movie_modal
          .as_ref()
          .unwrap()
          .tags
          .offset
          .borrow(),
        0
      );
    }
  }

  mod test_handle_left_right_action {
    use crate::models::servarr_data::radarr::modals::AddMovieModal;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_left_right_prompt_toggle(#[values(Key::Left, Key::Right)] key: Key) {
      let mut app = App::default();

      AddMovieHandler::with(&key, &mut app, &ActiveRadarrBlock::AddMoviePrompt, &None).handle();

      assert!(app.data.radarr_data.prompt_confirm);

      AddMovieHandler::with(&key, &mut app, &ActiveRadarrBlock::AddMoviePrompt, &None).handle();

      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[test]
    fn test_add_movie_search_input_left_right_keys() {
      let mut app = App::default();
      app.data.radarr_data.add_movie_search = Some("Test".into());

      AddMovieHandler::with(
        &DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchInput,
        &None,
      )
      .handle();

      assert_eq!(
        *app
          .data
          .radarr_data
          .add_movie_search
          .as_ref()
          .unwrap()
          .offset
          .borrow(),
        1
      );

      AddMovieHandler::with(
        &DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchInput,
        &None,
      )
      .handle();

      assert_eq!(
        *app
          .data
          .radarr_data
          .add_movie_search
          .as_ref()
          .unwrap()
          .offset
          .borrow(),
        0
      );
    }

    #[test]
    fn test_add_movie_tags_input_left_right_keys() {
      let mut app = App::default();
      app.data.radarr_data.add_movie_modal = Some(AddMovieModal {
        tags: "Test".into(),
        ..AddMovieModal::default()
      });

      AddMovieHandler::with(
        &DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        &ActiveRadarrBlock::AddMovieTagsInput,
        &None,
      )
      .handle();

      assert_eq!(
        *app
          .data
          .radarr_data
          .add_movie_modal
          .as_ref()
          .unwrap()
          .tags
          .offset
          .borrow(),
        1
      );

      AddMovieHandler::with(
        &DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        &ActiveRadarrBlock::AddMovieTagsInput,
        &None,
      )
      .handle();

      assert_eq!(
        *app
          .data
          .radarr_data
          .add_movie_modal
          .as_ref()
          .unwrap()
          .tags
          .offset
          .borrow(),
        0
      );
    }
  }

  mod test_handle_submit {
    use bimap::BiMap;
    use pretty_assertions::{assert_eq, assert_str_eq};
    use rstest::rstest;

    use crate::models::radarr_models::Movie;
    use crate::models::servarr_data::radarr::modals::AddMovieModal;
    use crate::models::servarr_data::radarr::radarr_data::ADD_MOVIE_SELECTION_BLOCKS;
    use crate::models::stateful_table::StatefulTable;
    use crate::models::BlockSelectionState;
    use crate::network::radarr_network::RadarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_add_movie_search_input_submit() {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.data.radarr_data.add_movie_search = Some("test".into());

      AddMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchInput,
        &None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMovieSearchResults.into()
      );
    }

    #[test]
    fn test_add_movie_search_input_submit_noop_on_empty_search() {
      let mut app = App::default();
      app.data.radarr_data.add_movie_search = Some(HorizontallyScrollableText::default());
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieSearchInput.into());
      app.should_ignore_quit_key = true;

      AddMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchInput,
        &None,
      )
      .handle();

      assert!(app.should_ignore_quit_key);
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMovieSearchInput.into()
      );
    }

    #[test]
    fn test_add_movie_search_results_submit() {
      let mut app = App::default();
      let mut add_searched_movies = StatefulTable::default();
      add_searched_movies.set_items(vec![AddMovieSearchResult::default()]);
      app.data.radarr_data.add_searched_movies = Some(add_searched_movies);
      app.data.radarr_data.quality_profile_map =
        BiMap::from_iter([(1, "B - Test 2".to_owned()), (0, "A - Test 1".to_owned())]);

      AddMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchResults,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMoviePrompt.into()
      );
      assert_eq!(
        app.data.radarr_data.selected_block.get_active_block(),
        &ActiveRadarrBlock::AddMovieSelectRootFolder
      );
      assert!(app.data.radarr_data.add_movie_modal.is_some());
      assert!(!app
        .data
        .radarr_data
        .add_movie_modal
        .as_ref()
        .unwrap()
        .monitor_list
        .items
        .is_empty());
      assert!(!app
        .data
        .radarr_data
        .add_movie_modal
        .as_ref()
        .unwrap()
        .minimum_availability_list
        .items
        .is_empty());
      assert!(!app
        .data
        .radarr_data
        .add_movie_modal
        .as_ref()
        .unwrap()
        .quality_profile_list
        .items
        .is_empty());
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
    fn test_add_movie_search_results_submit_does_nothing_on_empty_table() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieSearchResults.into());
      AddMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchResults,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMovieSearchResults.into()
      );
    }

    #[test]
    fn test_add_movie_search_results_submit_movie_already_in_library() {
      let mut app = App::default();
      let mut add_searched_movies = StatefulTable::default();
      add_searched_movies.set_items(vec![AddMovieSearchResult::default()]);
      app.data.radarr_data.add_searched_movies = Some(add_searched_movies);
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);

      AddMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchResults,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMovieAlreadyInLibrary.into()
      );
    }

    #[test]
    fn test_add_movie_prompt_prompt_decline_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());
      app.data.radarr_data.selected_block = BlockSelectionState::new(&ADD_MOVIE_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(ADD_MOVIE_SELECTION_BLOCKS.len() - 1);

      AddMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMoviePrompt,
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
    }

    #[test]
    fn test_add_movie_confirm_prompt_prompt_confirmation_submit() {
      let mut app = App::default();
      app.data.radarr_data.add_movie_modal = Some(AddMovieModal::default());
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());
      app.data.radarr_data.prompt_confirm = true;
      app.data.radarr_data.selected_block = BlockSelectionState::new(&ADD_MOVIE_SELECTION_BLOCKS);
      app
        .data
        .radarr_data
        .selected_block
        .set_index(ADD_MOVIE_SELECTION_BLOCKS.len() - 1);

      AddMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMoviePrompt,
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::AddMovie)
      );
      assert!(app.data.radarr_data.add_movie_modal.is_some());
    }

    #[rstest]
    #[case(ActiveRadarrBlock::AddMovieSelectRootFolder, 0)]
    #[case(ActiveRadarrBlock::AddMovieSelectMonitor, 1)]
    #[case(ActiveRadarrBlock::AddMovieSelectMinimumAvailability, 2)]
    #[case(ActiveRadarrBlock::AddMovieSelectQualityProfile, 3)]
    #[case(ActiveRadarrBlock::AddMovieTagsInput, 4)]
    fn test_add_movie_prompt_selected_block_submit(
      #[case] selected_block: ActiveRadarrBlock,
      #[case] index: usize,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(
        (
          ActiveRadarrBlock::AddMoviePrompt,
          Some(ActiveRadarrBlock::CollectionDetails),
        )
          .into(),
      );
      app.data.radarr_data.selected_block = BlockSelectionState::new(&ADD_MOVIE_SELECTION_BLOCKS);
      app.data.radarr_data.selected_block.set_index(index);

      AddMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMoviePrompt,
        &Some(ActiveRadarrBlock::CollectionDetails),
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &(selected_block, Some(ActiveRadarrBlock::CollectionDetails)).into()
      );
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);

      if selected_block == ActiveRadarrBlock::AddMovieTagsInput {
        assert!(app.should_ignore_quit_key);
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
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());
      app.push_navigation_stack(active_radarr_block.into());

      AddMovieHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &active_radarr_block,
        &Some(ActiveRadarrBlock::CollectionDetails),
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMoviePrompt.into()
      );

      if active_radarr_block == ActiveRadarrBlock::AddMovieTagsInput {
        assert!(!app.should_ignore_quit_key);
      }
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::models::servarr_data::radarr::modals::AddMovieModal;
    use crate::models::servarr_data::radarr::radarr_data::radarr_test_utils::utils::create_test_radarr_data;
    use crate::models::stateful_table::StatefulTable;
    use crate::simple_stateful_iterable_vec;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[test]
    fn test_add_movie_search_input_esc() {
      let mut app = App::default();
      app.data.radarr_data = create_test_radarr_data();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieSearchInput.into());

      AddMovieHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchInput,
        &None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
      assert_eq!(app.data.radarr_data.add_movie_search, None);
    }

    #[test]
    fn test_add_movie_input_esc() {
      let mut app = App::default();
      app.data.radarr_data = create_test_radarr_data();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieTagsInput.into());

      AddMovieHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMovieTagsInput,
        &None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMoviePrompt.into()
      );
    }

    #[rstest]
    fn test_add_movie_search_results_esc(
      #[values(
        ActiveRadarrBlock::AddMovieSearchResults,
        ActiveRadarrBlock::AddMovieEmptySearchResults
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieSearchInput.into());
      app.push_navigation_stack(active_radarr_block.into());
      let mut add_searched_movies = StatefulTable::default();
      add_searched_movies.set_items(simple_stateful_iterable_vec!(
        AddMovieSearchResult,
        HorizontallyScrollableText
      ));
      app.data.radarr_data.add_searched_movies = Some(add_searched_movies);

      AddMovieHandler::with(&ESC_KEY, &mut app, &active_radarr_block, &None).handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMovieSearchInput.into()
      );
      assert!(app.data.radarr_data.add_searched_movies.is_none());
      assert!(app.should_ignore_quit_key);
    }

    #[test]
    fn test_add_movie_already_in_library_esc() {
      let mut app = App::default();
      app.data.radarr_data = create_test_radarr_data();
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieSearchResults.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieAlreadyInLibrary.into());

      AddMovieHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMovieAlreadyInLibrary,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMovieSearchResults.into()
      );
    }

    #[test]
    fn test_add_movie_prompt_esc() {
      let mut app = App::default();
      app.data.radarr_data.add_movie_modal = Some(AddMovieModal::default());
      app.data.radarr_data = create_test_radarr_data();
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieSearchResults.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());

      AddMovieHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMoviePrompt,
        &None,
      )
      .handle();

      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMovieSearchResults.into()
      );
      assert!(app.data.radarr_data.add_movie_modal.is_none());
    }

    #[test]
    fn test_add_movie_tags_input_esc() {
      let mut app = App::default();
      app.data.radarr_data = create_test_radarr_data();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieTagsInput.into());

      AddMovieHandler::with(
        &ESC_KEY,
        &mut app,
        &ActiveRadarrBlock::AddMovieTagsInput,
        &None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AddMoviePrompt.into()
      );
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
      let mut app = App::default();
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

      AddMovieHandler::with(
        &ESC_KEY,
        &mut app,
        &active_radarr_block,
        &Some(ActiveRadarrBlock::CollectionDetails),
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &(
          ActiveRadarrBlock::AddMoviePrompt,
          Some(ActiveRadarrBlock::CollectionDetails),
        )
          .into()
      );
    }
  }

  mod test_handle_key_char {
    use super::*;
    use crate::models::servarr_data::radarr::modals::AddMovieModal;

    #[test]
    fn test_add_movie_search_input_backspace() {
      let mut app = App::default();
      app.data.radarr_data.add_movie_search = Some("Test".into());

      AddMovieHandler::with(
        &DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchInput,
        &None,
      )
      .handle();

      assert_str_eq!(
        app.data.radarr_data.add_movie_search.as_ref().unwrap().text,
        "Tes"
      );
    }

    #[test]
    fn test_add_movie_tags_input_backspace() {
      let mut app = App::default();
      app.data.radarr_data.add_movie_modal = Some(AddMovieModal {
        tags: "Test".into(),
        ..AddMovieModal::default()
      });

      AddMovieHandler::with(
        &DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        &ActiveRadarrBlock::AddMovieTagsInput,
        &None,
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
      let mut app = App::default();
      app.data.radarr_data.add_movie_search = Some(HorizontallyScrollableText::default());

      AddMovieHandler::with(
        &Key::Char('h'),
        &mut app,
        &ActiveRadarrBlock::AddMovieSearchInput,
        &None,
      )
      .handle();

      assert_str_eq!(
        app.data.radarr_data.add_movie_search.as_ref().unwrap().text,
        "h"
      );
    }

    #[test]
    fn test_add_movie_tags_input_char_key() {
      let mut app = App::default();
      app.data.radarr_data.add_movie_modal = Some(AddMovieModal::default());

      AddMovieHandler::with(
        &Key::Char('h'),
        &mut app,
        &ActiveRadarrBlock::AddMovieTagsInput,
        &None,
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
        "h"
      );
    }
  }

  #[test]
  fn test_add_movie_handler_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if ADD_MOVIE_BLOCKS.contains(&active_radarr_block) {
        assert!(AddMovieHandler::accepts(&active_radarr_block));
      } else {
        assert!(!AddMovieHandler::accepts(&active_radarr_block));
      }
    });
  }
}
