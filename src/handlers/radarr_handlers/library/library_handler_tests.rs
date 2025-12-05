#[cfg(test)]
mod tests {
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use std::cmp::Ordering;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::assert_modal_absent;
  use crate::assert_modal_present;
  use crate::assert_navigation_pushed;
  use crate::event::Key;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::radarr_handlers::library::{LibraryHandler, movies_sorting_options};
  use crate::handlers::radarr_handlers::radarr_handler_test_utils::utils::movie;
  use crate::models::radarr_models::Movie;
  use crate::models::servarr_data::radarr::radarr_data::{
    ADD_MOVIE_BLOCKS, ActiveRadarrBlock, DELETE_MOVIE_BLOCKS, EDIT_MOVIE_BLOCKS, LIBRARY_BLOCKS,
    MOVIE_DETAILS_BLOCKS,
  };
  use crate::models::servarr_models::Language;
  use crate::test_handler_delegation;

  mod test_handle_delete {
    use pretty_assertions::assert_eq;

    use crate::assert_delete_prompt;
    use crate::models::servarr_data::radarr::radarr_data::DELETE_MOVIE_SELECTION_BLOCKS;

    use super::*;

    const DELETE_KEY: Key = DEFAULT_KEYBINDINGS.delete.key;

    #[test]
    fn test_movies_delete() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);

      assert_delete_prompt!(
        LibraryHandler,
        app,
        ActiveRadarrBlock::Movies,
        ActiveRadarrBlock::DeleteMoviePrompt
      );
      assert_eq!(
        app.data.radarr_data.selected_block.blocks,
        DELETE_MOVIE_SELECTION_BLOCKS
      );
    }

    #[test]
    fn test_movies_delete_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);

      LibraryHandler::new(DELETE_KEY, &mut app, ActiveRadarrBlock::Movies, None).handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Movies.into());
    }
  }

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;
    use crate::assert_navigation_pushed;

    #[rstest]
    fn test_movie_tab_left(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.data.radarr_data.main_tabs.set_index(0);

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveRadarrBlock::Movies,
        None,
      )
      .handle();

      assert_eq!(
        app.data.radarr_data.main_tabs.get_active_route(),
        ActiveRadarrBlock::System.into()
      );
      assert_navigation_pushed!(app, ActiveRadarrBlock::System.into());
    }

    #[rstest]
    fn test_movie_tab_right(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.data.radarr_data.main_tabs.set_index(0);

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveRadarrBlock::Movies,
        None,
      )
      .handle();

      assert_eq!(
        app.data.radarr_data.main_tabs.get_active_route(),
        ActiveRadarrBlock::Collections.into()
      );
      assert_navigation_pushed!(app, ActiveRadarrBlock::Collections.into());
    }

    #[rstest]
    fn test_left_right_update_all_movies_prompt_toggle(
      #[values(DEFAULT_KEYBINDINGS.left.key, DEFAULT_KEYBINDINGS.right.key)] key: Key,
    ) {
      let mut app = App::test_default();

      LibraryHandler::new(
        key,
        &mut app,
        ActiveRadarrBlock::UpdateAllMoviesPrompt,
        None,
      )
      .handle();

      assert!(app.data.radarr_data.prompt_confirm);

      LibraryHandler::new(
        key,
        &mut app,
        ActiveRadarrBlock::UpdateAllMoviesPrompt,
        None,
      )
      .handle();

      assert!(!app.data.radarr_data.prompt_confirm);
    }
  }

  mod test_handle_submit {
    use crate::assert_navigation_popped;
    use crate::network::radarr_network::RadarrEvent;
    use pretty_assertions::assert_eq;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_movie_details_submit() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);

      LibraryHandler::new(SUBMIT_KEY, &mut app, ActiveRadarrBlock::Movies, None).handle();

      assert_navigation_pushed!(app, ActiveRadarrBlock::MovieDetails.into());
    }

    #[test]
    fn test_movie_details_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);

      LibraryHandler::new(SUBMIT_KEY, &mut app, ActiveRadarrBlock::Movies, None).handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Movies.into());
    }

    #[test]
    fn test_update_all_movies_prompt_confirm_submit() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);
      app.data.radarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::UpdateAllMoviesPrompt.into());

      LibraryHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::UpdateAllMoviesPrompt,
        None,
      )
      .handle();

      assert!(app.data.radarr_data.prompt_confirm);
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::UpdateAllMovies)
      );
      assert_navigation_popped!(app, ActiveRadarrBlock::Movies.into());
    }

    #[test]
    fn test_update_all_movies_prompt_decline_submit() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::UpdateAllMoviesPrompt.into());

      LibraryHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveRadarrBlock::UpdateAllMoviesPrompt,
        None,
      )
      .handle();

      assert!(!app.data.radarr_data.prompt_confirm);
      assert_eq!(app.data.radarr_data.prompt_confirm_action, None);
      assert_navigation_popped!(app, ActiveRadarrBlock::Movies.into());
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;
    use ratatui::widgets::TableState;

    use crate::models::servarr_data::radarr::radarr_data::radarr_test_utils::utils::create_test_radarr_data;
    use crate::models::stateful_table::StatefulTable;

    use super::*;
    use crate::{assert_navigation_popped, assert_navigation_pushed};

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[test]
    fn test_update_all_movies_prompt_blocks_esc() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::UpdateAllMoviesPrompt.into());
      app.data.radarr_data.prompt_confirm = true;

      LibraryHandler::new(
        ESC_KEY,
        &mut app,
        ActiveRadarrBlock::UpdateAllMoviesPrompt,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveRadarrBlock::Movies.into());
      assert!(!app.data.radarr_data.prompt_confirm);
    }

    #[rstest]
    fn test_default_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.error = "test error".to_owned().into();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.data.radarr_data = create_test_radarr_data();
      app.data.radarr_data.movies = StatefulTable {
        search: Some("Test".into()),
        filter: Some("Test".into()),
        filtered_items: Some(Vec::new()),
        filtered_state: Some(TableState::default()),
        ..StatefulTable::default()
      };

      LibraryHandler::new(ESC_KEY, &mut app, ActiveRadarrBlock::Movies, None).handle();

      assert_navigation_popped!(app, ActiveRadarrBlock::Movies.into());
      assert!(app.error.text.is_empty());
    }
  }

  mod test_handle_key_char {
    use bimap::BiMap;
    use pretty_assertions::{assert_eq, assert_str_eq};
    use serde_json::Number;
    use strum::IntoEnumIterator;

    use crate::models::radarr_models::MinimumAvailability;
    use crate::models::servarr_data::radarr::radarr_data::radarr_test_utils::utils::create_test_radarr_data;
    use crate::models::servarr_data::radarr::radarr_data::{
      EDIT_MOVIE_SELECTION_BLOCKS, RadarrData,
    };

    use crate::network::radarr_network::RadarrEvent;
    use crate::{assert_navigation_popped, test_edit_movie_key};

    use super::*;

    #[test]
    fn test_movie_add_key() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.add.key,
        &mut app,
        ActiveRadarrBlock::Movies,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveRadarrBlock::AddMovieSearchInput.into());
      assert!(app.ignore_special_keys_for_textbox_input);
      assert_modal_present!(app.data.radarr_data.add_movie_search);
    }

    #[test]
    fn test_movie_add_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.add.key,
        &mut app,
        ActiveRadarrBlock::Movies,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Movies.into());
      assert!(!app.ignore_special_keys_for_textbox_input);
      assert_modal_absent!(app.data.radarr_data.add_movie_search);
    }

    #[test]
    fn test_movie_edit_key() {
      test_edit_movie_key!(
        LibraryHandler,
        ActiveRadarrBlock::Movies,
        ActiveRadarrBlock::Movies
      );
    }

    #[test]
    fn test_movie_edit_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.edit.key,
        &mut app,
        ActiveRadarrBlock::Movies,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Movies.into());
      assert_modal_absent!(app.data.radarr_data.edit_movie_modal);
    }

    #[test]
    fn test_toggle_monitoring_key() {
      let mut app = App::test_default();
      app.data.radarr_data = create_test_radarr_data();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.is_routing = false;

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.toggle_monitoring.key,
        &mut app,
        ActiveRadarrBlock::Movies,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Movies.into());
      assert!(app.data.radarr_data.prompt_confirm);
      assert!(app.is_routing);
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::ToggleMovieMonitoring(0))
      );
    }

    #[test]
    fn test_toggle_monitoring_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.is_routing = false;

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.toggle_monitoring.key,
        &mut app,
        ActiveRadarrBlock::Movies,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Movies.into());
      assert!(!app.data.radarr_data.prompt_confirm);
      assert_modal_absent!(app.data.radarr_data.prompt_confirm_action);
      assert!(!app.is_routing);
    }

    #[test]
    fn test_update_all_movies_key() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        ActiveRadarrBlock::Movies,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveRadarrBlock::UpdateAllMoviesPrompt.into());
    }

    #[test]
    fn test_update_all_movies_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        ActiveRadarrBlock::Movies,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Movies.into());
    }

    #[test]
    fn test_refresh_movies_key() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveRadarrBlock::Movies,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveRadarrBlock::Movies.into());
      assert!(app.should_refresh);
    }

    #[test]
    fn test_refresh_movies_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveRadarrBlock::Movies,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveRadarrBlock::Movies.into());
      assert!(!app.should_refresh);
    }

    #[test]
    fn test_update_all_movies_prompt_confirm() {
      let mut app = App::test_default();
      app
        .data
        .radarr_data
        .movies
        .set_items(vec![Movie::default()]);
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.push_navigation_stack(ActiveRadarrBlock::UpdateAllMoviesPrompt.into());

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveRadarrBlock::UpdateAllMoviesPrompt,
        None,
      )
      .handle();

      assert!(app.data.radarr_data.prompt_confirm);
      assert_eq!(
        app.data.radarr_data.prompt_confirm_action,
        Some(RadarrEvent::UpdateAllMovies)
      );
      assert_navigation_popped!(app, ActiveRadarrBlock::Movies.into());
    }
  }

  #[rstest]
  fn test_delegates_add_movie_blocks_to_add_movie_handler(
    #[values(
      ActiveRadarrBlock::AddMovieSearchInput,
      ActiveRadarrBlock::AddMovieSearchResults,
      ActiveRadarrBlock::AddMoviePrompt,
      ActiveRadarrBlock::AddMovieSelectMonitor,
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
      ActiveRadarrBlock::AddMovieSelectQualityProfile,
      ActiveRadarrBlock::AddMovieSelectRootFolder,
      ActiveRadarrBlock::AddMovieAlreadyInLibrary,
      ActiveRadarrBlock::AddMovieTagsInput
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(
      LibraryHandler,
      ActiveRadarrBlock::Movies,
      active_radarr_block
    );
  }

  #[rstest]
  fn test_delegates_movie_details_blocks_to_movie_details_handler(
    #[values(
      ActiveRadarrBlock::MovieDetails,
      ActiveRadarrBlock::MovieHistory,
      ActiveRadarrBlock::FileInfo,
      ActiveRadarrBlock::Cast,
      ActiveRadarrBlock::Crew,
      ActiveRadarrBlock::AutomaticallySearchMoviePrompt,
      ActiveRadarrBlock::UpdateAndScanPrompt,
      ActiveRadarrBlock::ManualSearch,
      ActiveRadarrBlock::ManualSearchConfirmPrompt
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(
      LibraryHandler,
      ActiveRadarrBlock::Movies,
      active_radarr_block
    );
  }

  #[rstest]
  fn test_delegates_edit_movie_blocks_to_edit_movie_handler(
    #[values(
      ActiveRadarrBlock::EditMoviePrompt,
      ActiveRadarrBlock::EditMoviePathInput,
      ActiveRadarrBlock::EditMovieSelectMinimumAvailability,
      ActiveRadarrBlock::EditMovieSelectQualityProfile,
      ActiveRadarrBlock::EditMovieTagsInput
    )]
    active_radarr_block: ActiveRadarrBlock,
  ) {
    test_handler_delegation!(
      LibraryHandler,
      ActiveRadarrBlock::Movies,
      active_radarr_block
    );
  }

  #[test]
  fn test_delegates_delete_movie_blocks_to_delete_movie_handler() {
    test_handler_delegation!(
      LibraryHandler,
      ActiveRadarrBlock::Movies,
      ActiveRadarrBlock::DeleteMoviePrompt
    );
  }

  #[test]
  fn test_extract_movie_id() {
    let mut app = App::test_default();
    app.data.radarr_data.movies.set_items(vec![movie()]);

    let movie_id = LibraryHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::Movies,
      None,
    )
    .extract_movie_id();

    assert_eq!(movie_id, 1);
  }

  #[test]
  fn test_movies_sorting_options_title() {
    let expected_cmp_fn: fn(&Movie, &Movie) -> Ordering = |a, b| {
      a.title
        .text
        .to_lowercase()
        .cmp(&b.title.text.to_lowercase())
    };
    let mut expected_movies_vec = movies_vec();
    expected_movies_vec.sort_by(expected_cmp_fn);

    let sort_option = movies_sorting_options()[0].clone();
    let mut sorted_movies_vec = movies_vec();
    sorted_movies_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_movies_vec, expected_movies_vec);
    assert_str_eq!(sort_option.name, "Title");
  }

  #[test]
  fn test_movies_sorting_options_year() {
    let expected_cmp_fn: fn(&Movie, &Movie) -> Ordering = |a, b| a.year.cmp(&b.year);
    let mut expected_movies_vec = movies_vec();
    expected_movies_vec.sort_by(expected_cmp_fn);

    let sort_option = movies_sorting_options()[1].clone();
    let mut sorted_movies_vec = movies_vec();
    sorted_movies_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_movies_vec, expected_movies_vec);
    assert_str_eq!(sort_option.name, "Year");
  }

  #[test]
  fn test_movies_sorting_options_studio() {
    let expected_cmp_fn: fn(&Movie, &Movie) -> Ordering = |a, b| {
      a.studio
        .as_ref()
        .unwrap_or(&String::new())
        .to_lowercase()
        .cmp(&b.studio.as_ref().unwrap_or(&String::new()).to_lowercase())
    };
    let mut expected_movies_vec = movies_vec();
    expected_movies_vec.sort_by(expected_cmp_fn);

    let sort_option = movies_sorting_options()[2].clone();
    let mut sorted_movies_vec = movies_vec();
    sorted_movies_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_movies_vec, expected_movies_vec);
    assert_str_eq!(sort_option.name, "Studio");
  }

  #[test]
  fn test_movies_sorting_options_runtime() {
    let expected_cmp_fn: fn(&Movie, &Movie) -> Ordering = |a, b| a.runtime.cmp(&b.runtime);
    let mut expected_movies_vec = movies_vec();
    expected_movies_vec.sort_by(expected_cmp_fn);

    let sort_option = movies_sorting_options()[3].clone();
    let mut sorted_movies_vec = movies_vec();
    sorted_movies_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_movies_vec, expected_movies_vec);
    assert_str_eq!(sort_option.name, "Runtime");
  }

  #[test]
  fn test_movies_sorting_options_rating() {
    let expected_cmp_fn: fn(&Movie, &Movie) -> Ordering = |a, b| {
      a.certification
        .as_ref()
        .unwrap_or(&String::new())
        .to_lowercase()
        .cmp(
          &b.certification
            .as_ref()
            .unwrap_or(&String::new())
            .to_lowercase(),
        )
    };
    let mut expected_movies_vec = movies_vec();
    expected_movies_vec.sort_by(expected_cmp_fn);

    let sort_option = movies_sorting_options()[4].clone();
    let mut sorted_movies_vec = movies_vec();
    sorted_movies_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_movies_vec, expected_movies_vec);
    assert_str_eq!(sort_option.name, "Rating");
  }

  #[test]
  fn test_movies_sorting_options_language() {
    let expected_cmp_fn: fn(&Movie, &Movie) -> Ordering = |a, b| {
      a.original_language
        .name
        .to_lowercase()
        .cmp(&b.original_language.name.to_lowercase())
    };
    let mut expected_movies_vec = movies_vec();
    expected_movies_vec.sort_by(expected_cmp_fn);

    let sort_option = movies_sorting_options()[5].clone();
    let mut sorted_movies_vec = movies_vec();
    sorted_movies_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_movies_vec, expected_movies_vec);
    assert_str_eq!(sort_option.name, "Language");
  }

  #[test]
  fn test_movies_sorting_options_size() {
    let expected_cmp_fn: fn(&Movie, &Movie) -> Ordering =
      |a, b| a.size_on_disk.cmp(&b.size_on_disk);
    let mut expected_movies_vec = movies_vec();
    expected_movies_vec.sort_by(expected_cmp_fn);

    let sort_option = movies_sorting_options()[6].clone();
    let mut sorted_movies_vec = movies_vec();
    sorted_movies_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_movies_vec, expected_movies_vec);
    assert_str_eq!(sort_option.name, "Size");
  }

  #[test]
  fn test_movies_sorting_options_quality() {
    let expected_cmp_fn: fn(&Movie, &Movie) -> Ordering =
      |a, b| a.quality_profile_id.cmp(&b.quality_profile_id);
    let mut expected_movies_vec = movies_vec();
    expected_movies_vec.sort_by(expected_cmp_fn);

    let sort_option = movies_sorting_options()[7].clone();
    let mut sorted_movies_vec = movies_vec();
    sorted_movies_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_movies_vec, expected_movies_vec);
    assert_str_eq!(sort_option.name, "Quality");
  }

  #[test]
  fn test_movies_sorting_options_monitored() {
    let expected_cmp_fn: fn(&Movie, &Movie) -> Ordering = |a, b| a.monitored.cmp(&b.monitored);
    let mut expected_movies_vec = movies_vec();
    expected_movies_vec.sort_by(expected_cmp_fn);

    let sort_option = movies_sorting_options()[8].clone();
    let mut sorted_movies_vec = movies_vec();
    sorted_movies_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_movies_vec, expected_movies_vec);
    assert_str_eq!(sort_option.name, "Monitored");
  }

  #[test]
  fn test_movies_sorting_options_tags() {
    let expected_cmp_fn: fn(&Movie, &Movie) -> Ordering = |a, b| {
      let a_str = a
        .tags
        .iter()
        .map(|tag| tag.as_i64().unwrap().to_string())
        .collect::<Vec<String>>()
        .join(",");
      let b_str = b
        .tags
        .iter()
        .map(|tag| tag.as_i64().unwrap().to_string())
        .collect::<Vec<String>>()
        .join(",");

      a_str.cmp(&b_str)
    };
    let mut expected_movies_vec = movies_vec();
    expected_movies_vec.sort_by(expected_cmp_fn);

    let sort_option = movies_sorting_options()[9].clone();
    let mut sorted_movies_vec = movies_vec();
    sorted_movies_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_movies_vec, expected_movies_vec);
    assert_str_eq!(sort_option.name, "Tags");
  }

  #[test]
  fn test_library_handler_accepts() {
    let mut library_handler_blocks = Vec::new();
    library_handler_blocks.extend(LIBRARY_BLOCKS);
    library_handler_blocks.extend(ADD_MOVIE_BLOCKS);
    library_handler_blocks.extend(DELETE_MOVIE_BLOCKS);
    library_handler_blocks.extend(EDIT_MOVIE_BLOCKS);
    library_handler_blocks.extend(MOVIE_DETAILS_BLOCKS);

    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if library_handler_blocks.contains(&active_radarr_block) {
        assert!(LibraryHandler::accepts(active_radarr_block));
      } else {
        assert!(!LibraryHandler::accepts(active_radarr_block));
      }
    });
  }

  #[rstest]
  fn test_library_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = LibraryHandler::new(
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
  fn test_library_handler_not_ready_when_loading() {
    let mut app = App::test_default();
    app.is_loading = true;

    let handler = LibraryHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::Movies,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_library_handler_not_ready_when_movies_is_empty() {
    let mut app = App::test_default();
    app.is_loading = false;

    let handler = LibraryHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::Movies,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_library_handler_ready_when_not_loading_and_movies_is_not_empty() {
    let mut app = App::test_default();
    app.is_loading = false;
    app
      .data
      .radarr_data
      .movies
      .set_items(vec![Movie::default()]);

    let handler = LibraryHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveRadarrBlock::Movies,
      None,
    );

    assert!(handler.is_ready());
  }

  fn movies_vec() -> Vec<Movie> {
    vec![
      Movie {
        id: 3,
        title: "test 1".into(),
        original_language: Language {
          id: 1,
          name: "English".to_owned(),
        },
        size_on_disk: 1024,
        studio: Some("Studio 1".to_owned()),
        year: 2024,
        monitored: false,
        runtime: 12.into(),
        quality_profile_id: 1,
        certification: Some("PG-13".to_owned()),
        tags: vec![1.into(), 2.into()],
        ..Movie::default()
      },
      Movie {
        id: 2,
        title: "test 2".into(),
        original_language: Language {
          id: 2,
          name: "Chinese".to_owned(),
        },
        size_on_disk: 2048,
        studio: Some("Studio 2".to_owned()),
        year: 1998,
        monitored: false,
        runtime: 60.into(),
        quality_profile_id: 2,
        certification: Some("R".to_owned()),
        tags: vec![1.into(), 3.into()],
        ..Movie::default()
      },
      Movie {
        id: 1,
        title: "test 3".into(),
        original_language: Language {
          id: 3,
          name: "Japanese".to_owned(),
        },
        size_on_disk: 512,
        studio: Some("studio 3".to_owned()),
        year: 1954,
        monitored: true,
        runtime: 120.into(),
        quality_profile_id: 3,
        certification: Some("G".to_owned()),
        tags: vec![2.into(), 3.into()],
        ..Movie::default()
      },
    ]
  }
}
