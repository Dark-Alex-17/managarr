#[cfg(test)]
mod tests {
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use std::cmp::Ordering;
  use strum::IntoEnumIterator;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::sonarr_handlers::library::{series_sorting_options, LibraryHandler};
  use crate::handlers::KeyEventHandler;
  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, ADD_SERIES_BLOCKS, DELETE_SERIES_BLOCKS, EDIT_SERIES_BLOCKS,
    EPISODE_DETAILS_BLOCKS, LIBRARY_BLOCKS, SEASON_DETAILS_BLOCKS, SERIES_DETAILS_BLOCKS,
  };
  use crate::models::sonarr_models::{Series, SeriesStatus, SeriesType};
  use crate::test_handler_delegation;

  mod test_handle_delete {
    use pretty_assertions::assert_eq;

    use crate::assert_delete_prompt;
    use crate::models::servarr_data::sonarr::sonarr_data::DELETE_SERIES_SELECTION_BLOCKS;

    use super::*;

    const DELETE_KEY: Key = DEFAULT_KEYBINDINGS.delete.key;

    #[test]
    fn test_series_delete() {
      let mut app = App::test_default();
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);

      assert_delete_prompt!(
        LibraryHandler,
        app,
        ActiveSonarrBlock::Series,
        ActiveSonarrBlock::DeleteSeriesPrompt
      );
      assert_eq!(
        app.data.sonarr_data.selected_block.blocks,
        DELETE_SERIES_SELECTION_BLOCKS
      );
    }

    #[test]
    fn test_series_delete_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);

      LibraryHandler::new(DELETE_KEY, &mut app, ActiveSonarrBlock::Series, None).handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
    }
  }

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_series_tab_left(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.data.sonarr_data.main_tabs.set_index(0);

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveSonarrBlock::Series,
        None,
      )
      .handle();

      assert_eq!(
        app.data.sonarr_data.main_tabs.get_active_route(),
        ActiveSonarrBlock::System.into()
      );
      assert_eq!(app.get_current_route(), ActiveSonarrBlock::System.into());
    }

    #[rstest]
    fn test_series_tab_right(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.data.sonarr_data.main_tabs.set_index(0);

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveSonarrBlock::Series,
        None,
      )
      .handle();

      assert_eq!(
        app.data.sonarr_data.main_tabs.get_active_route(),
        ActiveSonarrBlock::Downloads.into()
      );
      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Downloads.into());
    }

    #[rstest]
    fn test_left_right_update_all_series_prompt_toggle(
      #[values(DEFAULT_KEYBINDINGS.left.key, DEFAULT_KEYBINDINGS.right.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());

      LibraryHandler::new(
        key,
        &mut app,
        ActiveSonarrBlock::UpdateAllSeriesPrompt,
        None,
      )
      .handle();

      assert!(app.data.sonarr_data.prompt_confirm);

      LibraryHandler::new(
        key,
        &mut app,
        ActiveSonarrBlock::UpdateAllSeriesPrompt,
        None,
      )
      .handle();

      assert!(!app.data.sonarr_data.prompt_confirm);
    }
  }

  mod test_handle_submit {
    use pretty_assertions::assert_eq;

    use crate::network::sonarr_network::SonarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_series_details_submit() {
      let mut app = App::test_default();
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);

      LibraryHandler::new(SUBMIT_KEY, &mut app, ActiveSonarrBlock::Series, None).handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeriesDetails.into()
      );
    }

    #[test]
    fn test_series_details_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);

      LibraryHandler::new(SUBMIT_KEY, &mut app, ActiveSonarrBlock::Series, None).handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
    }

    #[test]
    fn test_update_all_series_prompt_confirm_submit() {
      let mut app = App::test_default();
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);
      app.data.sonarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::UpdateAllSeriesPrompt.into());

      LibraryHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::UpdateAllSeriesPrompt,
        None,
      )
      .handle();

      assert!(app.data.sonarr_data.prompt_confirm);
      assert_eq!(
        app.data.sonarr_data.prompt_confirm_action,
        Some(SonarrEvent::UpdateAllSeries)
      );
      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
    }

    #[test]
    fn test_update_all_series_prompt_decline_submit() {
      let mut app = App::test_default();
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::UpdateAllSeriesPrompt.into());

      LibraryHandler::new(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::UpdateAllSeriesPrompt,
        None,
      )
      .handle();

      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.data.sonarr_data.prompt_confirm_action, None);
      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;

    use crate::models::servarr_data::sonarr::sonarr_data::sonarr_test_utils::utils::create_test_sonarr_data;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[test]
    fn test_update_all_series_prompt_blocks_esc() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::UpdateAllSeriesPrompt.into());
      app.data.sonarr_data.prompt_confirm = true;

      LibraryHandler::new(
        ESC_KEY,
        &mut app,
        ActiveSonarrBlock::UpdateAllSeriesPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
      assert!(!app.data.sonarr_data.prompt_confirm);
    }

    #[rstest]
    fn test_default_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.error = "test error".to_owned().into();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data = create_test_sonarr_data();

      LibraryHandler::new(ESC_KEY, &mut app, ActiveSonarrBlock::Series, None).handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
      assert!(app.error.text.is_empty());
    }
  }

  mod test_handle_key_char {
    use pretty_assertions::{assert_eq, assert_str_eq};
    use serde_json::Number;
    use strum::IntoEnumIterator;

    use crate::models::servarr_data::sonarr::sonarr_data::sonarr_test_utils::utils::create_test_sonarr_data;
    use crate::models::servarr_data::sonarr::sonarr_data::SonarrData;
    use crate::models::sonarr_models::SeriesType;

    use crate::network::sonarr_network::SonarrEvent;
    use crate::test_edit_series_key;

    use super::*;

    #[test]
    fn test_series_add_key() {
      let mut app = App::test_default();
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.add.key,
        &mut app,
        ActiveSonarrBlock::Series,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::AddSeriesSearchInput.into()
      );
      assert!(app.should_ignore_quit_key);
      assert!(app.data.sonarr_data.add_series_search.is_some());
    }

    #[test]
    fn test_series_add_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.add.key,
        &mut app,
        ActiveSonarrBlock::Series,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
      assert!(!app.should_ignore_quit_key);
      assert!(app.data.sonarr_data.add_series_search.is_none());
    }

    #[test]
    fn test_series_edit_key() {
      test_edit_series_key!(
        LibraryHandler,
        ActiveSonarrBlock::Series,
        ActiveSonarrBlock::Series
      );
    }

    #[test]
    fn test_series_edit_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.edit.key,
        &mut app,
        ActiveSonarrBlock::Series,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
      assert!(app.data.sonarr_data.edit_series_modal.is_none());
    }

    #[test]
    fn test_update_all_series_key() {
      let mut app = App::test_default();
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        ActiveSonarrBlock::Series,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::UpdateAllSeriesPrompt.into()
      );
    }

    #[test]
    fn test_update_all_series_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        ActiveSonarrBlock::Series,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
    }

    #[test]
    fn test_refresh_series_key() {
      let mut app = App::test_default();
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveSonarrBlock::Series,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
      assert!(app.should_refresh);
    }

    #[test]
    fn test_refresh_series_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveSonarrBlock::Series,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
      assert!(!app.should_refresh);
    }

    #[test]
    fn test_update_all_series_prompt_confirm() {
      let mut app = App::test_default();
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::UpdateAllSeriesPrompt.into());

      LibraryHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        ActiveSonarrBlock::UpdateAllSeriesPrompt,
        None,
      )
      .handle();

      assert!(app.data.sonarr_data.prompt_confirm);
      assert_eq!(
        app.data.sonarr_data.prompt_confirm_action,
        Some(SonarrEvent::UpdateAllSeries)
      );
      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
    }
  }

  #[rstest]
  fn test_delegates_add_series_blocks_to_add_series_handler(
    #[values(
      ActiveSonarrBlock::AddSeriesAlreadyInLibrary,
      ActiveSonarrBlock::AddSeriesEmptySearchResults,
      ActiveSonarrBlock::AddSeriesPrompt,
      ActiveSonarrBlock::AddSeriesSearchInput,
      ActiveSonarrBlock::AddSeriesSearchResults,
      ActiveSonarrBlock::AddSeriesSelectLanguageProfile,
      ActiveSonarrBlock::AddSeriesSelectMonitor,
      ActiveSonarrBlock::AddSeriesSelectQualityProfile,
      ActiveSonarrBlock::AddSeriesSelectRootFolder,
      ActiveSonarrBlock::AddSeriesSelectSeriesType,
      ActiveSonarrBlock::AddSeriesTagsInput
    )]
    active_sonarr_block: ActiveSonarrBlock,
  ) {
    test_handler_delegation!(
      LibraryHandler,
      ActiveSonarrBlock::Series,
      active_sonarr_block
    );
  }

  #[rstest]
  fn test_delegates_series_details_blocks_to_series_details_handler(
    #[values(
      ActiveSonarrBlock::SeriesDetails,
      ActiveSonarrBlock::SeriesHistory,
      ActiveSonarrBlock::SearchSeason,
      ActiveSonarrBlock::SearchSeasonError,
      ActiveSonarrBlock::UpdateAndScanSeriesPrompt,
      ActiveSonarrBlock::AutomaticallySearchSeriesPrompt,
      ActiveSonarrBlock::SearchSeriesHistory,
      ActiveSonarrBlock::SearchSeriesHistoryError,
      ActiveSonarrBlock::FilterSeriesHistory,
      ActiveSonarrBlock::FilterSeriesHistoryError,
      ActiveSonarrBlock::SeriesHistorySortPrompt,
      ActiveSonarrBlock::SeriesHistoryDetails
    )]
    active_sonarr_block: ActiveSonarrBlock,
  ) {
    test_handler_delegation!(
      LibraryHandler,
      ActiveSonarrBlock::Series,
      active_sonarr_block
    );
  }

  #[rstest]
  fn test_delegates_season_details_blocks_to_season_details_handler(
    #[values(
      ActiveSonarrBlock::SeasonDetails,
      ActiveSonarrBlock::SeasonHistory,
      ActiveSonarrBlock::SearchEpisodes,
      ActiveSonarrBlock::SearchEpisodesError,
      ActiveSonarrBlock::AutomaticallySearchSeasonPrompt,
      ActiveSonarrBlock::SearchSeasonHistory,
      ActiveSonarrBlock::SearchSeasonHistoryError,
      ActiveSonarrBlock::FilterSeasonHistory,
      ActiveSonarrBlock::FilterSeasonHistoryError,
      ActiveSonarrBlock::SeasonHistorySortPrompt,
      ActiveSonarrBlock::SeasonHistoryDetails,
      ActiveSonarrBlock::ManualSeasonSearch,
      ActiveSonarrBlock::ManualSeasonSearchSortPrompt,
      ActiveSonarrBlock::DeleteEpisodeFilePrompt
    )]
    active_sonarr_block: ActiveSonarrBlock,
  ) {
    test_handler_delegation!(
      LibraryHandler,
      ActiveSonarrBlock::Series,
      active_sonarr_block
    );
  }

  #[rstest]
  fn test_delegates_episode_details_blocks_to_season_details_handler(
    #[values(
      ActiveSonarrBlock::EpisodeDetails,
      ActiveSonarrBlock::EpisodeHistory,
      ActiveSonarrBlock::AutomaticallySearchEpisodePrompt,
      ActiveSonarrBlock::EpisodeHistoryDetails,
      ActiveSonarrBlock::ManualEpisodeSearch,
      ActiveSonarrBlock::ManualEpisodeSearchSortPrompt,
      ActiveSonarrBlock::DeleteEpisodeFilePrompt
    )]
    active_sonarr_block: ActiveSonarrBlock,
  ) {
    test_handler_delegation!(
      LibraryHandler,
      ActiveSonarrBlock::Series,
      active_sonarr_block
    );
  }

  #[rstest]
  fn test_delegates_edit_series_blocks_to_edit_series_handler(
    #[values(
      ActiveSonarrBlock::EditSeriesPrompt,
      ActiveSonarrBlock::EditSeriesPathInput,
      ActiveSonarrBlock::EditSeriesSelectSeriesType,
      ActiveSonarrBlock::EditSeriesSelectQualityProfile,
      ActiveSonarrBlock::EditSeriesSelectLanguageProfile,
      ActiveSonarrBlock::EditSeriesTagsInput
    )]
    active_sonarr_block: ActiveSonarrBlock,
  ) {
    test_handler_delegation!(
      LibraryHandler,
      ActiveSonarrBlock::Series,
      active_sonarr_block
    );
  }

  #[test]
  fn test_delegates_delete_series_blocks_to_delete_series_handler() {
    test_handler_delegation!(
      LibraryHandler,
      ActiveSonarrBlock::Series,
      ActiveSonarrBlock::DeleteSeriesPrompt
    );
  }

  #[test]
  fn test_series_sorting_options_title() {
    let expected_cmp_fn: fn(&Series, &Series) -> Ordering = |a, b| {
      a.title
        .text
        .to_lowercase()
        .cmp(&b.title.text.to_lowercase())
    };
    let mut expected_series_vec = series_vec();
    expected_series_vec.sort_by(expected_cmp_fn);

    let sort_option = series_sorting_options()[0].clone();
    let mut sorted_series_vec = series_vec();
    sorted_series_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_series_vec, expected_series_vec);
    assert_str_eq!(sort_option.name, "Title");
  }

  #[test]
  fn test_series_sorting_options_year() {
    let expected_cmp_fn: fn(&Series, &Series) -> Ordering = |a, b| a.year.cmp(&b.year);
    let mut expected_series_vec = series_vec();
    expected_series_vec.sort_by(expected_cmp_fn);

    let sort_option = series_sorting_options()[1].clone();
    let mut sorted_series_vec = series_vec();
    sorted_series_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_series_vec, expected_series_vec);
    assert_str_eq!(sort_option.name, "Year");
  }

  #[test]
  fn test_series_sorting_options_network() {
    let expected_cmp_fn: fn(&Series, &Series) -> Ordering = |a, b| {
      a.network
        .as_ref()
        .unwrap_or(&String::new())
        .to_lowercase()
        .cmp(&b.network.as_ref().unwrap_or(&String::new()).to_lowercase())
    };
    let mut expected_series_vec = series_vec();
    expected_series_vec.sort_by(expected_cmp_fn);

    let sort_option = series_sorting_options()[2].clone();
    let mut sorted_series_vec = series_vec();
    sorted_series_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_series_vec, expected_series_vec);
    assert_str_eq!(sort_option.name, "Network");
  }

  #[test]
  fn test_series_sorting_options_status() {
    let expected_cmp_fn: fn(&Series, &Series) -> Ordering = |a, b| {
      a.status
        .to_string()
        .to_lowercase()
        .cmp(&b.status.to_string().to_lowercase())
    };
    let mut expected_series_vec = series_vec();
    expected_series_vec.sort_by(expected_cmp_fn);

    let sort_option = series_sorting_options()[3].clone();
    let mut sorted_series_vec = series_vec();
    sorted_series_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_series_vec, expected_series_vec);
    assert_str_eq!(sort_option.name, "Status");
  }

  #[test]
  fn test_series_sorting_options_rating() {
    let expected_cmp_fn: fn(&Series, &Series) -> Ordering = |a, b| {
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
    let mut expected_series_vec = series_vec();
    expected_series_vec.sort_by(expected_cmp_fn);

    let sort_option = series_sorting_options()[4].clone();
    let mut sorted_series_vec = series_vec();
    sorted_series_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_series_vec, expected_series_vec);
    assert_str_eq!(sort_option.name, "Rating");
  }

  #[test]
  fn test_series_sorting_options_type() {
    let expected_cmp_fn: fn(&Series, &Series) -> Ordering = |a, b| {
      a.series_type
        .to_string()
        .to_lowercase()
        .cmp(&b.series_type.to_string().to_lowercase())
    };
    let mut expected_series_vec = series_vec();
    expected_series_vec.sort_by(expected_cmp_fn);

    let sort_option = series_sorting_options()[5].clone();
    let mut sorted_series_vec = series_vec();
    sorted_series_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_series_vec, expected_series_vec);
    assert_str_eq!(sort_option.name, "Type");
  }

  #[test]
  fn test_series_sorting_options_quality() {
    let expected_cmp_fn: fn(&Series, &Series) -> Ordering =
      |a, b| a.quality_profile_id.cmp(&b.quality_profile_id);
    let mut expected_series_vec = series_vec();
    expected_series_vec.sort_by(expected_cmp_fn);

    let sort_option = series_sorting_options()[6].clone();
    let mut sorted_series_vec = series_vec();
    sorted_series_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_series_vec, expected_series_vec);
    assert_str_eq!(sort_option.name, "Quality");
  }

  #[test]
  fn test_series_sorting_options_language() {
    let expected_cmp_fn: fn(&Series, &Series) -> Ordering =
      |a, b| a.language_profile_id.cmp(&b.language_profile_id);
    let mut expected_series_vec = series_vec();
    expected_series_vec.sort_by(expected_cmp_fn);

    let sort_option = series_sorting_options()[7].clone();
    let mut sorted_series_vec = series_vec();
    sorted_series_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_series_vec, expected_series_vec);
    assert_str_eq!(sort_option.name, "Language");
  }

  #[test]
  fn test_series_sorting_options_monitored() {
    let expected_cmp_fn: fn(&Series, &Series) -> Ordering = |a, b| a.monitored.cmp(&b.monitored);
    let mut expected_series_vec = series_vec();
    expected_series_vec.sort_by(expected_cmp_fn);

    let sort_option = series_sorting_options()[8].clone();
    let mut sorted_series_vec = series_vec();
    sorted_series_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_series_vec, expected_series_vec);
    assert_str_eq!(sort_option.name, "Monitored");
  }

  #[test]
  fn test_series_sorting_options_tags() {
    let expected_cmp_fn: fn(&Series, &Series) -> Ordering = |a, b| {
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
    let mut expected_series_vec = series_vec();
    expected_series_vec.sort_by(expected_cmp_fn);

    let sort_option = series_sorting_options()[9].clone();
    let mut sorted_series_vec = series_vec();
    sorted_series_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_series_vec, expected_series_vec);
    assert_str_eq!(sort_option.name, "Tags");
  }

  #[test]
  fn test_library_handler_accepts() {
    let mut library_handler_blocks = Vec::new();
    library_handler_blocks.extend(LIBRARY_BLOCKS);
    library_handler_blocks.extend(ADD_SERIES_BLOCKS);
    library_handler_blocks.extend(DELETE_SERIES_BLOCKS);
    library_handler_blocks.extend(EDIT_SERIES_BLOCKS);
    library_handler_blocks.extend(SERIES_DETAILS_BLOCKS);
    library_handler_blocks.extend(SEASON_DETAILS_BLOCKS);
    library_handler_blocks.extend(EPISODE_DETAILS_BLOCKS);

    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if library_handler_blocks.contains(&active_sonarr_block) {
        assert!(LibraryHandler::accepts(active_sonarr_block));
      } else {
        assert!(!LibraryHandler::accepts(active_sonarr_block));
      }
    });
  }

  #[test]
  fn test_library_handler_not_ready_when_loading() {
    let mut app = App::test_default();
    app.is_loading = true;

    let handler = LibraryHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::Series,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_library_handler_not_ready_when_series_is_empty() {
    let mut app = App::test_default();
    app.is_loading = false;

    let handler = LibraryHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::Series,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_library_handler_ready_when_not_loading_and_series_is_not_empty() {
    let mut app = App::test_default();
    app.is_loading = false;
    app
      .data
      .sonarr_data
      .series
      .set_items(vec![Series::default()]);

    let handler = LibraryHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::Series,
      None,
    );

    assert!(handler.is_ready());
  }

  fn series_vec() -> Vec<Series> {
    vec![
      Series {
        id: 3,
        title: "test 1".into(),
        network: Some("Network 1".to_owned()),
        year: 2024,
        monitored: false,
        season_folder: false,
        status: SeriesStatus::Ended,
        quality_profile_id: 1,
        language_profile_id: 1,
        certification: Some("TV-MA".to_owned()),
        series_type: SeriesType::Daily,
        tags: vec![1.into(), 2.into()],
        ..Series::default()
      },
      Series {
        id: 2,
        title: "test 2".into(),
        network: Some("Network 2".to_owned()),
        year: 1998,
        monitored: false,
        season_folder: false,
        status: SeriesStatus::Continuing,
        quality_profile_id: 2,
        language_profile_id: 2,
        certification: Some("TV-PG".to_owned()),
        series_type: SeriesType::Anime,
        tags: vec![1.into(), 3.into()],
        ..Series::default()
      },
      Series {
        id: 1,
        title: "test 3".into(),
        network: Some("network 3".to_owned()),
        year: 1954,
        monitored: true,
        season_folder: false,
        status: SeriesStatus::Upcoming,
        quality_profile_id: 3,
        language_profile_id: 3,
        certification: Some("TV-G".to_owned()),
        tags: vec![2.into(), 3.into()],
        series_type: SeriesType::Standard,
        ..Series::default()
      },
    ]
  }
}
