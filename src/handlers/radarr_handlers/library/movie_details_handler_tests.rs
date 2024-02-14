#[cfg(test)]
mod tests {
  use std::cmp::Ordering;

  use pretty_assertions::assert_str_eq;
  use serde_json::Number;
  use strum::IntoEnumIterator;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::radarr_handlers::library::movie_details_handler::{
    releases_sorting_options, MovieDetailsHandler,
  };
  use crate::handlers::KeyEventHandler;
  use crate::models::radarr_models::{
    Credit, Language, MovieHistoryItem, Quality, QualityWrapper, Release,
  };
  use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, MOVIE_DETAILS_BLOCKS};
  use crate::models::stateful_table::SortOption;
  use crate::models::{HorizontallyScrollableText, ScrollableText};

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::{assert_eq, assert_str_eq};
    use rstest::rstest;

    use crate::models::servarr_data::radarr::modals::MovieDetailsModal;
    use crate::simple_stateful_iterable_vec;

    use super::*;

    #[test]
    fn test_movie_details_scroll() {
      let mut app = App::default();
      app.data.radarr_data.movie_details_modal = Some(MovieDetailsModal {
        movie_details: ScrollableText::with_string("Test 1\nTest 2".to_owned()),
        ..MovieDetailsModal::default()
      });

      MovieDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.up.key,
        &mut app,
        &ActiveRadarrBlock::MovieDetails,
        &None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .movie_details_modal
          .as_ref()
          .unwrap()
          .movie_details
          .offset,
        0
      );

      MovieDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.down.key,
        &mut app,
        &ActiveRadarrBlock::MovieDetails,
        &None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .movie_details_modal
          .as_ref()
          .unwrap()
          .movie_details
          .offset,
        1
      );
    }

    #[rstest]
    fn test_movie_history_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::default();
      let mut movie_details_modal = MovieDetailsModal::default();
      movie_details_modal
        .movie_history
        .set_items(simple_stateful_iterable_vec!(
          MovieHistoryItem,
          HorizontallyScrollableText,
          source_title
        ));
      app.data.radarr_data.movie_details_modal = Some(movie_details_modal);

      MovieDetailsHandler::with(&key, &mut app, &ActiveRadarrBlock::MovieHistory, &None).handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .movie_details_modal
          .as_ref()
          .unwrap()
          .movie_history
          .current_selection()
          .source_title
          .to_string(),
        "Test 2"
      );

      MovieDetailsHandler::with(&key, &mut app, &ActiveRadarrBlock::MovieHistory, &None).handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .movie_details_modal
          .as_ref()
          .unwrap()
          .movie_history
          .current_selection()
          .source_title
          .to_string(),
        "Test 1"
      );
    }

    #[rstest]
    fn test_cast_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::default();
      let mut movie_details_modal = MovieDetailsModal::default();
      movie_details_modal
        .movie_cast
        .set_items(simple_stateful_iterable_vec!(Credit, String, person_name));
      app.data.radarr_data.movie_details_modal = Some(movie_details_modal);

      MovieDetailsHandler::with(&key, &mut app, &ActiveRadarrBlock::Cast, &None).handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .movie_details_modal
          .as_ref()
          .unwrap()
          .movie_cast
          .current_selection()
          .person_name,
        "Test 2"
      );

      MovieDetailsHandler::with(&key, &mut app, &ActiveRadarrBlock::Cast, &None).handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .movie_details_modal
          .as_ref()
          .unwrap()
          .movie_cast
          .current_selection()
          .person_name,
        "Test 1"
      );
    }

    #[rstest]
    fn test_crew_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::default();
      let mut movie_details_modal = MovieDetailsModal::default();
      movie_details_modal
        .movie_crew
        .set_items(simple_stateful_iterable_vec!(Credit, String, person_name));
      app.data.radarr_data.movie_details_modal = Some(movie_details_modal);

      MovieDetailsHandler::with(&key, &mut app, &ActiveRadarrBlock::Crew, &None).handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .movie_details_modal
          .as_ref()
          .unwrap()
          .movie_crew
          .current_selection()
          .person_name,
        "Test 2"
      );

      MovieDetailsHandler::with(&key, &mut app, &ActiveRadarrBlock::Crew, &None).handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .movie_details_modal
          .as_ref()
          .unwrap()
          .movie_crew
          .current_selection()
          .person_name,
        "Test 1"
      );
    }

    #[rstest]
    fn test_manual_search_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::default();
      let mut movie_details_modal = MovieDetailsModal::default();
      movie_details_modal
        .movie_releases
        .set_items(simple_stateful_iterable_vec!(
          Release,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.movie_details_modal = Some(movie_details_modal);

      MovieDetailsHandler::with(&key, &mut app, &ActiveRadarrBlock::ManualSearch, &None).handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .movie_details_modal
          .as_ref()
          .unwrap()
          .movie_releases
          .current_selection()
          .title
          .to_string(),
        "Test 2"
      );

      MovieDetailsHandler::with(&key, &mut app, &ActiveRadarrBlock::ManualSearch, &None).handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .movie_details_modal
          .as_ref()
          .unwrap()
          .movie_releases
          .current_selection()
          .title
          .to_string(),
        "Test 1"
      );
    }

    #[rstest]
    fn test_manual_search_sort_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let release_field_vec = sort_options();
      let mut app = App::default();
      let mut movie_details_modal = MovieDetailsModal::default();
      movie_details_modal.movie_releases.sorting(sort_options());
      app.data.radarr_data.movie_details_modal = Some(movie_details_modal);

      if key == Key::Up {
        for i in (0..release_field_vec.len()).rev() {
          MovieDetailsHandler::with(
            &key,
            &mut app,
            &ActiveRadarrBlock::ManualSearchSortPrompt,
            &None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .radarr_data
              .movie_details_modal
              .as_ref()
              .unwrap()
              .movie_releases
              .sort
              .as_ref()
              .unwrap()
              .current_selection(),
            &release_field_vec[i]
          );
        }
      } else {
        for i in 0..release_field_vec.len() {
          MovieDetailsHandler::with(
            &key,
            &mut app,
            &ActiveRadarrBlock::ManualSearchSortPrompt,
            &None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .radarr_data
              .movie_details_modal
              .as_ref()
              .unwrap()
              .movie_releases
              .sort
              .as_ref()
              .unwrap()
              .current_selection(),
            &release_field_vec[(i + 1) % release_field_vec.len()]
          );
        }
      }
    }
  }

  mod test_handle_home_end {
    use crate::extended_stateful_iterable_vec;
    use crate::models::servarr_data::radarr::modals::MovieDetailsModal;

    use super::*;

    #[test]
    fn test_movie_details_home_end() {
      let mut app = App::default();
      let movie_details_modal = MovieDetailsModal {
        movie_details: ScrollableText::with_string("Test 1\nTest 2".to_owned()),
        ..MovieDetailsModal::default()
      };
      app.data.radarr_data.movie_details_modal = Some(movie_details_modal);

      MovieDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        &ActiveRadarrBlock::MovieDetails,
        &None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .movie_details_modal
          .as_ref()
          .unwrap()
          .movie_details
          .offset,
        1
      );

      MovieDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        &ActiveRadarrBlock::MovieDetails,
        &None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .movie_details_modal
          .as_ref()
          .unwrap()
          .movie_details
          .offset,
        0
      );
    }

    #[test]
    fn test_movie_history_home_end() {
      let mut app = App::default();
      let mut movie_details_modal = MovieDetailsModal::default();
      movie_details_modal
        .movie_history
        .set_items(extended_stateful_iterable_vec!(
          MovieHistoryItem,
          HorizontallyScrollableText,
          source_title
        ));
      app.data.radarr_data.movie_details_modal = Some(movie_details_modal);

      MovieDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        &ActiveRadarrBlock::MovieHistory,
        &None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .movie_details_modal
          .as_ref()
          .unwrap()
          .movie_history
          .current_selection()
          .source_title
          .to_string(),
        "Test 3"
      );

      MovieDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        &ActiveRadarrBlock::MovieHistory,
        &None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .movie_details_modal
          .as_ref()
          .unwrap()
          .movie_history
          .current_selection()
          .source_title
          .to_string(),
        "Test 1"
      );
    }

    #[test]
    fn test_cast_home_end() {
      let mut app = App::default();
      let mut movie_details_modal = MovieDetailsModal::default();
      movie_details_modal
        .movie_cast
        .set_items(extended_stateful_iterable_vec!(Credit, String, person_name));
      app.data.radarr_data.movie_details_modal = Some(movie_details_modal);

      MovieDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        &ActiveRadarrBlock::Cast,
        &None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .movie_details_modal
          .as_ref()
          .unwrap()
          .movie_cast
          .current_selection()
          .person_name,
        "Test 3"
      );

      MovieDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        &ActiveRadarrBlock::Cast,
        &None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .movie_details_modal
          .as_ref()
          .unwrap()
          .movie_cast
          .current_selection()
          .person_name,
        "Test 1"
      );
    }

    #[test]
    fn test_crew_home_end() {
      let mut app = App::default();
      let mut movie_details_modal = MovieDetailsModal::default();
      movie_details_modal
        .movie_crew
        .set_items(extended_stateful_iterable_vec!(Credit, String, person_name));
      app.data.radarr_data.movie_details_modal = Some(movie_details_modal);

      MovieDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        &ActiveRadarrBlock::Crew,
        &None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .movie_details_modal
          .as_ref()
          .unwrap()
          .movie_crew
          .current_selection()
          .person_name,
        "Test 3"
      );

      MovieDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        &ActiveRadarrBlock::Crew,
        &None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .movie_details_modal
          .as_ref()
          .unwrap()
          .movie_crew
          .current_selection()
          .person_name,
        "Test 1"
      );
    }

    #[test]
    fn test_manual_search_home_end() {
      let mut app = App::default();
      let mut movie_details_modal = MovieDetailsModal::default();
      movie_details_modal
        .movie_releases
        .set_items(extended_stateful_iterable_vec!(
          Release,
          HorizontallyScrollableText
        ));
      app.data.radarr_data.movie_details_modal = Some(movie_details_modal);

      MovieDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        &ActiveRadarrBlock::ManualSearch,
        &None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .movie_details_modal
          .as_ref()
          .unwrap()
          .movie_releases
          .current_selection()
          .title
          .to_string(),
        "Test 3"
      );

      MovieDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        &ActiveRadarrBlock::ManualSearch,
        &None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .radarr_data
          .movie_details_modal
          .as_ref()
          .unwrap()
          .movie_releases
          .current_selection()
          .title
          .to_string(),
        "Test 1"
      );
    }

    #[test]
    fn test_manual_search_sort_home_end() {
      let release_field_vec = sort_options();
      let mut app = App::default();
      let mut movie_details_modal = MovieDetailsModal::default();
      movie_details_modal.movie_releases.sorting(sort_options());
      app.data.radarr_data.movie_details_modal = Some(movie_details_modal);

      MovieDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        &ActiveRadarrBlock::ManualSearchSortPrompt,
        &None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .movie_details_modal
          .as_ref()
          .unwrap()
          .movie_releases
          .sort
          .as_ref()
          .unwrap()
          .current_selection(),
        &release_field_vec[release_field_vec.len() - 1]
      );

      MovieDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        &ActiveRadarrBlock::ManualSearchSortPrompt,
        &None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .radarr_data
          .movie_details_modal
          .as_ref()
          .unwrap()
          .movie_releases
          .sort
          .as_ref()
          .unwrap()
          .current_selection(),
        &release_field_vec[0]
      );
    }
  }

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_left_right_prompt_toggle(
      #[values(
        ActiveRadarrBlock::AutomaticallySearchMoviePrompt,
        ActiveRadarrBlock::UpdateAndScanPrompt,
        ActiveRadarrBlock::ManualSearchConfirmPrompt
      )]
      active_radarr_block: ActiveRadarrBlock,
      #[values(Key::Left, Key::Right)] key: Key,
    ) {
      let mut app = App::default();

      MovieDetailsHandler::with(&key, &mut app, &active_radarr_block, &None).handle();

      assert!(app.data.radarr_data.prompt_confirm);

      MovieDetailsHandler::with(&key, &mut app, &active_radarr_block, &None).handle();

      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[rstest]
    #[case(ActiveRadarrBlock::MovieDetails, ActiveRadarrBlock::MovieHistory)]
    #[case(ActiveRadarrBlock::MovieHistory, ActiveRadarrBlock::FileInfo)]
    #[case(ActiveRadarrBlock::FileInfo, ActiveRadarrBlock::Cast)]
    #[case(ActiveRadarrBlock::Cast, ActiveRadarrBlock::Crew)]
    #[case(ActiveRadarrBlock::Crew, ActiveRadarrBlock::ManualSearch)]
    #[case(ActiveRadarrBlock::ManualSearch, ActiveRadarrBlock::MovieDetails)]
    fn test_movie_info_tabs_left_right_action(
      #[case] left_block: ActiveRadarrBlock,
      #[case] right_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(right_block.into());
      app.data.radarr_data.movie_info_tabs.index = app
        .data
        .radarr_data
        .movie_info_tabs
        .tabs
        .iter()
        .position(|tab_route| tab_route.route == right_block.into())
        .unwrap_or_default();

      MovieDetailsHandler::with(&DEFAULT_KEYBINDINGS.left.key, &mut app, &right_block, &None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        app.data.radarr_data.movie_info_tabs.get_active_route()
      );
      assert_eq!(app.get_current_route(), &left_block.into());

      MovieDetailsHandler::with(&DEFAULT_KEYBINDINGS.right.key, &mut app, &left_block, &None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        app.data.radarr_data.movie_info_tabs.get_active_route()
      );
      assert_eq!(app.get_current_route(), &right_block.into());
    }
  }

  mod test_handle_submit {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::models::servarr_data::radarr::modals::MovieDetailsModal;
    use crate::network::radarr_network::RadarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_manual_search_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::ManualSearch.into());

      MovieDetailsHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::ManualSearch,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::ManualSearchConfirmPrompt.into()
      );
    }

    #[rstest]
    #[case(
      ActiveRadarrBlock::AutomaticallySearchMoviePrompt,
      RadarrEvent::TriggerAutomaticSearch
    )]
    #[case(ActiveRadarrBlock::UpdateAndScanPrompt, RadarrEvent::UpdateAndScan)]
    #[case(
      ActiveRadarrBlock::ManualSearchConfirmPrompt,
      RadarrEvent::DownloadRelease
    )]
    fn test_movie_info_prompt_confirm_submit(
      #[case] prompt_block: ActiveRadarrBlock,
      #[case] expected_action: RadarrEvent,
    ) {
      let mut app = App::default();
      app.data.radarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveRadarrBlock::MovieDetails.into());
      app.push_navigation_stack(prompt_block.into());

      MovieDetailsHandler::with(&SUBMIT_KEY, &mut app, &prompt_block, &None).handle();

      assert!(app.data.radarr_data.prompt_confirm);
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::MovieDetails.into()
      );
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(expected_action)
      );
    }

    #[rstest]
    fn test_movie_info_prompt_decline_submit(
      #[values(
        ActiveRadarrBlock::AutomaticallySearchMoviePrompt,
        ActiveRadarrBlock::UpdateAndScanPrompt,
        ActiveRadarrBlock::ManualSearchConfirmPrompt
      )]
      prompt_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveRadarrBlock::MovieDetails.into());
      app.push_navigation_stack(prompt_block.into());

      MovieDetailsHandler::with(&SUBMIT_KEY, &mut app, &prompt_block, &None).handle();

      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::MovieDetails.into()
      );
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
    }

    #[test]
    fn test_manual_search_sort_prompt_submit() {
      let mut app = App::default();
      let mut movie_details_modal = MovieDetailsModal::default();
      movie_details_modal.movie_releases.sort_asc = true;
      movie_details_modal.movie_releases.sorting(sort_options());
      movie_details_modal.movie_releases.set_items(release_vec());
      app.data.radarr_data.movie_details_modal = Some(movie_details_modal);
      app.push_navigation_stack(ActiveRadarrBlock::ManualSearch.into());
      app.push_navigation_stack(ActiveRadarrBlock::ManualSearchSortPrompt.into());

      let mut expected_vec = release_vec();
      expected_vec.reverse();

      MovieDetailsHandler::with(
        &SUBMIT_KEY,
        &mut app,
        &ActiveRadarrBlock::ManualSearchSortPrompt,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::ManualSearch.into()
      );
      assert_eq!(
        app
          .data
          .radarr_data
          .movie_details_modal
          .as_ref()
          .unwrap()
          .movie_releases
          .items,
        expected_vec
      );
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::assert_movie_info_tabs_reset;
    use crate::models::servarr_data::radarr::radarr_data::radarr_test_utils::utils::create_test_radarr_data;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_movie_info_tabs_esc(
      #[values(
        ActiveRadarrBlock::MovieDetails,
        ActiveRadarrBlock::MovieHistory,
        ActiveRadarrBlock::FileInfo,
        ActiveRadarrBlock::Cast,
        ActiveRadarrBlock::Crew,
        ActiveRadarrBlock::ManualSearch
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.data.radarr_data = create_test_radarr_data();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(active_radarr_block.into());

      MovieDetailsHandler::with(&ESC_KEY, &mut app, &active_radarr_block, &None).handle();

      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
      assert_movie_info_tabs_reset!(app.data.radarr_data);
    }

    #[rstest]
    fn test_movie_info_prompts_esc(
      #[values(
        ActiveRadarrBlock::AutomaticallySearchMoviePrompt,
        ActiveRadarrBlock::UpdateAndScanPrompt,
        ActiveRadarrBlock::ManualSearchConfirmPrompt,
        ActiveRadarrBlock::ManualSearchSortPrompt
      )]
      prompt_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();
      app.data.radarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(prompt_block.into());

      MovieDetailsHandler::with(&ESC_KEY, &mut app, &prompt_block, &None).handle();

      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.get_current_route(), &ActiveRadarrBlock::Movies.into());
    }
  }

  mod test_handle_key_char {
    use bimap::BiMap;
    use pretty_assertions::{assert_eq, assert_str_eq};
    use rstest::rstest;
    use strum::IntoEnumIterator;

    use crate::handlers::radarr_handlers::library::movie_details_handler::releases_sorting_options;
    use crate::models::radarr_models::{MinimumAvailability, Movie};
    use crate::models::servarr_data::radarr::modals::MovieDetailsModal;
    use crate::models::servarr_data::radarr::radarr_data::radarr_test_utils::utils::create_test_radarr_data;
    use crate::models::servarr_data::radarr::radarr_data::{
      RadarrData, EDIT_MOVIE_SELECTION_BLOCKS,
    };
    use crate::test_edit_movie_key;

    use super::*;

    #[rstest]
    fn test_search_key(
      #[values(
        ActiveRadarrBlock::MovieDetails,
        ActiveRadarrBlock::MovieHistory,
        ActiveRadarrBlock::FileInfo,
        ActiveRadarrBlock::Cast,
        ActiveRadarrBlock::Crew,
        ActiveRadarrBlock::ManualSearch
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();

      MovieDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.search.key,
        &mut app,
        &active_radarr_block,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::AutomaticallySearchMoviePrompt.into()
      );
    }

    #[test]
    fn test_sort_key() {
      let mut app = App::default();
      app.data.radarr_data.movie_details_modal = Some(MovieDetailsModal::default());

      MovieDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.sort.key,
        &mut app,
        &ActiveRadarrBlock::ManualSearch,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::ManualSearchSortPrompt.into()
      );
      assert_eq!(
        app
          .data
          .radarr_data
          .movie_details_modal
          .as_ref()
          .unwrap()
          .movie_releases
          .sort
          .as_ref()
          .unwrap()
          .items,
        releases_sorting_options()
      );
      assert!(
        !app
          .data
          .radarr_data
          .movie_details_modal
          .as_ref()
          .unwrap()
          .movie_releases
          .sort_asc
      );
    }

    #[rstest]
    fn test_edit_key(
      #[values(
        ActiveRadarrBlock::MovieDetails,
        ActiveRadarrBlock::MovieHistory,
        ActiveRadarrBlock::FileInfo,
        ActiveRadarrBlock::Cast,
        ActiveRadarrBlock::Crew,
        ActiveRadarrBlock::ManualSearch
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      test_edit_movie_key!(
        MovieDetailsHandler,
        active_radarr_block,
        active_radarr_block
      );
    }

    #[rstest]
    fn test_update_key(
      #[values(
        ActiveRadarrBlock::MovieDetails,
        ActiveRadarrBlock::MovieHistory,
        ActiveRadarrBlock::FileInfo,
        ActiveRadarrBlock::Cast,
        ActiveRadarrBlock::Crew,
        ActiveRadarrBlock::ManualSearch
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();

      MovieDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        &active_radarr_block,
        &None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        &ActiveRadarrBlock::UpdateAndScanPrompt.into()
      );
    }

    #[rstest]
    fn test_refresh_key(
      #[values(
        ActiveRadarrBlock::MovieDetails,
        ActiveRadarrBlock::MovieHistory,
        ActiveRadarrBlock::FileInfo,
        ActiveRadarrBlock::Cast,
        ActiveRadarrBlock::Crew,
        ActiveRadarrBlock::ManualSearch
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::default();

      MovieDetailsHandler::with(
        &DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        &active_radarr_block,
        &None,
      )
      .handle();

      assert_eq!(app.get_current_route(), &active_radarr_block.into());
      assert!(app.is_routing);
    }
  }

  #[test]
  fn test_releases_sorting_options_source() {
    let expected_cmp_fn: fn(&Release, &Release) -> Ordering = |a, b| a.protocol.cmp(&b.protocol);
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[0].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Source");
  }

  #[test]
  fn test_releases_sorting_options_age() {
    let expected_cmp_fn: fn(&Release, &Release) -> Ordering = |a, b| a.age.cmp(&b.age);
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[1].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Age");
  }

  #[test]
  fn test_releases_sorting_options_rejected() {
    let expected_cmp_fn: fn(&Release, &Release) -> Ordering = |a, b| a.rejected.cmp(&b.rejected);
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[2].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Rejected");
  }

  #[test]
  fn test_releases_sorting_options_title() {
    let expected_cmp_fn: fn(&Release, &Release) -> Ordering = |a, b| {
      a.title
        .text
        .to_lowercase()
        .cmp(&b.title.text.to_lowercase())
    };
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[3].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Title");
  }

  #[test]
  fn test_releases_sorting_options_indexer() {
    let expected_cmp_fn: fn(&Release, &Release) -> Ordering =
      |a, b| a.indexer.to_lowercase().cmp(&b.indexer.to_lowercase());
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[4].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Indexer");
  }

  #[test]
  fn test_releases_sorting_options_size() {
    let expected_cmp_fn: fn(&Release, &Release) -> Ordering = |a, b| a.size.cmp(&b.size);
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[5].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Size");
  }

  #[test]
  fn test_releases_sorting_options_peers() {
    let expected_cmp_fn: fn(&Release, &Release) -> Ordering = |a, b| {
      let default_number = Number::from(i64::MAX);
      let seeder_a = a
        .seeders
        .as_ref()
        .unwrap_or(&default_number)
        .as_u64()
        .unwrap();
      let seeder_b = b
        .seeders
        .as_ref()
        .unwrap_or(&default_number)
        .as_u64()
        .unwrap();

      seeder_a.cmp(&seeder_b)
    };
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[6].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Peers");
  }

  #[test]
  fn test_releases_sorting_options_language() {
    let expected_cmp_fn: fn(&Release, &Release) -> Ordering = |a, b| {
      let default_language_vec = vec![Language {
        name: "_".to_owned(),
      }];
      let language_a = &a.languages.as_ref().unwrap_or(&default_language_vec)[0];
      let language_b = &b.languages.as_ref().unwrap_or(&default_language_vec)[0];

      language_a.cmp(language_b)
    };
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[7].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Language");
  }

  #[test]
  fn test_releases_sorting_options_quality() {
    let expected_cmp_fn: fn(&Release, &Release) -> Ordering = |a, b| a.quality.cmp(&b.quality);
    let mut expected_releases_vec = release_vec();
    expected_releases_vec.sort_by(expected_cmp_fn);

    let sort_option = releases_sorting_options()[8].clone();
    let mut sorted_releases_vec = release_vec();
    sorted_releases_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_releases_vec, expected_releases_vec);
    assert_str_eq!(sort_option.name, "Quality");
  }

  fn release_vec() -> Vec<Release> {
    let release_a = Release {
      protocol: "Protocol A".to_owned(),
      age: 1,
      title: HorizontallyScrollableText::from("Title A"),
      indexer: "Indexer A".to_owned(),
      size: 1,
      rejected: true,
      seeders: Some(Number::from(1)),
      languages: Some(vec![Language {
        name: "Language A".to_owned(),
      }]),
      quality: QualityWrapper {
        quality: Quality {
          name: "Quality A".to_owned(),
        },
      },
      ..Release::default()
    };
    let release_b = Release {
      protocol: "Protocol B".to_owned(),
      age: 2,
      title: HorizontallyScrollableText::from("title B"),
      indexer: "indexer B".to_owned(),
      size: 2,
      rejected: false,
      seeders: Some(Number::from(2)),
      languages: Some(vec![Language {
        name: "Language B".to_owned(),
      }]),
      quality: QualityWrapper {
        quality: Quality {
          name: "Quality B".to_owned(),
        },
      },
      ..Release::default()
    };
    let release_c = Release {
      protocol: "Protocol C".to_owned(),
      age: 3,
      title: HorizontallyScrollableText::from("Title C"),
      indexer: "Indexer C".to_owned(),
      size: 3,
      rejected: false,
      seeders: None,
      languages: None,
      quality: QualityWrapper {
        quality: Quality {
          name: "Quality C".to_owned(),
        },
      },
      ..Release::default()
    };

    vec![release_a, release_b, release_c]
  }

  fn sort_options() -> Vec<SortOption<Release>> {
    vec![SortOption {
      name: "Test 1",
      cmp_fn: Some(|a, b| a.age.cmp(&b.age)),
    }]
  }

  #[test]
  fn test_movie_details_handler_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if MOVIE_DETAILS_BLOCKS.contains(&active_radarr_block) {
        assert!(MovieDetailsHandler::accepts(&active_radarr_block));
      } else {
        assert!(!MovieDetailsHandler::accepts(&active_radarr_block));
      }
    });
  }
}
