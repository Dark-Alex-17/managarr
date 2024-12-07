#[cfg(test)]
mod tests {
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::sonarr_handlers::library::series_details_handler::SeriesDetailsHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, SERIES_DETAILS_BLOCKS,
  };
  use crate::models::sonarr_models::Season;
  use crate::models::sonarr_models::SonarrHistoryItem;
  use crate::models::stateful_table::{SortOption, StatefulTable};
  use core::sync::atomic::Ordering::SeqCst;
  use pretty_assertions::assert_str_eq;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  mod test_handle_scroll_up_and_down {
    use super::*;
    use pretty_assertions::assert_eq;

    #[rstest]
    fn test_seasons_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::default();
      app.data.sonarr_data.seasons.set_items(vec![
        Season {
          season_number: 1,
          ..Season::default()
        },
        Season {
          season_number: 2,
          ..Season::default()
        },
      ]);

      SeriesDetailsHandler::with(key, &mut app, ActiveSonarrBlock::SeriesDetails, None).handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .seasons
          .current_selection()
          .season_number,
        2
      );

      SeriesDetailsHandler::with(key, &mut app, ActiveSonarrBlock::SeriesDetails, None).handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .seasons
          .current_selection()
          .season_number,
        1
      );
    }

    #[rstest]
    fn test_series_history_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::default();
      let mut series_history = StatefulTable::default();
      series_history.set_items(vec![
        SonarrHistoryItem {
          source_title: "Test 1".into(),
          ..SonarrHistoryItem::default()
        },
        SonarrHistoryItem {
          source_title: "Test 2".into(),
          ..SonarrHistoryItem::default()
        },
      ]);
      app.data.sonarr_data.series_history = Some(series_history);

      SeriesDetailsHandler::with(key, &mut app, ActiveSonarrBlock::SeriesHistory, None).handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .series_history
          .as_ref()
          .unwrap()
          .current_selection()
          .source_title
          .text,
        "Test 2"
      );

      SeriesDetailsHandler::with(key, &mut app, ActiveSonarrBlock::SeriesHistory, None).handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .series_history
          .as_ref()
          .unwrap()
          .current_selection()
          .source_title
          .text,
        "Test 1"
      );
    }

    #[rstest]
    fn test_series_history_sort_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let series_history_item_field_vec = sort_options();
      let mut app = App::default();
      app.data.sonarr_data.series_history = Some(StatefulTable::default());
      app
        .data
        .sonarr_data
        .series_history
        .as_mut()
        .unwrap()
        .sorting(sort_options());

      if key == Key::Up {
        for i in (0..series_history_item_field_vec.len()).rev() {
          SeriesDetailsHandler::with(
            key,
            &mut app,
            ActiveSonarrBlock::SeriesHistorySortPrompt,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .sonarr_data
              .series_history
              .as_ref()
              .unwrap()
              .sort
              .as_ref()
              .unwrap()
              .current_selection(),
            &series_history_item_field_vec[i]
          );
        }
      } else {
        for i in 0..series_history_item_field_vec.len() {
          SeriesDetailsHandler::with(
            key,
            &mut app,
            ActiveSonarrBlock::SeriesHistorySortPrompt,
            None,
          )
          .handle();

          assert_eq!(
            app
              .data
              .sonarr_data
              .series_history
              .as_ref()
              .unwrap()
              .sort
              .as_ref()
              .unwrap()
              .current_selection(),
            &series_history_item_field_vec[(i + 1) % series_history_item_field_vec.len()]
          );
        }
      }
    }
  }

  mod test_handle_home_end {
    use super::*;
    use pretty_assertions::{assert_eq, assert_str_eq};

    #[test]
    fn test_seasons_home_and_end() {
      let mut app = App::default();
      app.data.sonarr_data.seasons.set_items(vec![
        Season {
          season_number: 1,
          ..Season::default()
        },
        Season {
          season_number: 2,
          ..Season::default()
        },
        Season {
          season_number: 3,
          ..Season::default()
        },
      ]);

      SeriesDetailsHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::SeriesDetails,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .seasons
          .current_selection()
          .season_number,
        3
      );

      SeriesDetailsHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::SeriesDetails,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .seasons
          .current_selection()
          .season_number,
        1
      );
    }

    #[test]
    fn test_series_history_home_and_end() {
      let mut app = App::default();
      let mut series_history = StatefulTable::default();
      series_history.set_items(vec![
        SonarrHistoryItem {
          source_title: "Test 1".into(),
          ..SonarrHistoryItem::default()
        },
        SonarrHistoryItem {
          source_title: "Test 2".into(),
          ..SonarrHistoryItem::default()
        },
        SonarrHistoryItem {
          source_title: "Test 3".into(),
          ..SonarrHistoryItem::default()
        },
      ]);
      app.data.sonarr_data.series_history = Some(series_history);

      SeriesDetailsHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::SeriesHistory,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .series_history
          .as_ref()
          .unwrap()
          .current_selection()
          .source_title
          .text,
        "Test 3"
      );

      SeriesDetailsHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::SeriesHistory,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .series_history
          .as_ref()
          .unwrap()
          .current_selection()
          .source_title
          .text,
        "Test 1"
      );
    }

    #[test]
    fn test_season_search_box_home_end_keys() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .seasons
        .set_items(vec![Season::default()]);
      app.data.sonarr_data.seasons.search = Some("Test".into());

      SeriesDetailsHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::SearchSeason,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .seasons
          .search
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
        4
      );

      SeriesDetailsHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::SearchSeason,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .seasons
          .search
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
        0
      );
    }

    #[test]
    fn test_series_history_search_box_home_end_keys() {
      let mut app = App::default();
      let mut series_history = StatefulTable::default();
      series_history.set_items(vec![SonarrHistoryItem::default()]);
      series_history.search = Some("Test".into());
      app.data.sonarr_data.series_history = Some(series_history);

      SeriesDetailsHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::SearchSeriesHistory,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .series_history
          .as_ref()
          .unwrap()
          .search
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
        4
      );

      SeriesDetailsHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::SearchSeriesHistory,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .series_history
          .as_ref()
          .unwrap()
          .search
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
        0
      );
    }

    #[test]
    fn test_series_history_filter_box_home_end_keys() {
      let mut app = App::default();
      let mut series_history = StatefulTable::default();
      series_history.set_items(vec![SonarrHistoryItem::default()]);
      series_history.filter = Some("Test".into());
      app.data.sonarr_data.series_history = Some(series_history);

      SeriesDetailsHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::FilterSeriesHistory,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .series_history
          .as_ref()
          .unwrap()
          .filter
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
        4
      );

      SeriesDetailsHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::FilterSeriesHistory,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .series_history
          .as_ref()
          .unwrap()
          .filter
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
        0
      );
    }

    #[test]
    fn test_series_history_sort_home_end() {
      let series_history_item_field_vec = sort_options();
      let mut app = App::default();
      let mut series_history = StatefulTable::default();
      series_history.sorting(sort_options());
      app.data.sonarr_data.series_history = Some(series_history);

      SeriesDetailsHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::SeriesHistorySortPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .series_history
          .as_ref()
          .unwrap()
          .sort
          .as_ref()
          .unwrap()
          .current_selection(),
        &series_history_item_field_vec[series_history_item_field_vec.len() - 1]
      );

      SeriesDetailsHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::SeriesHistorySortPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .series_history
          .as_ref()
          .unwrap()
          .sort
          .as_ref()
          .unwrap()
          .current_selection(),
        &series_history_item_field_vec[0]
      );
    }
  }

  mod test_handle_left_right_actions {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    fn test_left_right_prompt_toggle(
      #[values(
        ActiveSonarrBlock::AutomaticallySearchSeriesPrompt,
        ActiveSonarrBlock::UpdateAndScanSeriesPrompt
      )]
      active_sonarr_block: ActiveSonarrBlock,
      #[values(Key::Left, Key::Right)] key: Key,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());

      SeriesDetailsHandler::with(key, &mut app, active_sonarr_block, None).handle();

      assert!(app.data.sonarr_data.prompt_confirm);

      SeriesDetailsHandler::with(key, &mut app, active_sonarr_block, None).handle();

      assert!(!app.data.sonarr_data.prompt_confirm);
    }

    #[rstest]
    #[case(ActiveSonarrBlock::SeriesDetails, ActiveSonarrBlock::SeriesHistory)]
    #[case(ActiveSonarrBlock::SeriesHistory, ActiveSonarrBlock::SeriesDetails)]
    fn test_series_details_tabs_left_right_action(
      #[case] left_block: ActiveSonarrBlock,
      #[case] right_block: ActiveSonarrBlock,
      #[values(true, false)] is_ready: bool,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.is_loading = is_ready;
      app.push_navigation_stack(right_block.into());
      app.data.sonarr_data.series_info_tabs.index = app
        .data
        .sonarr_data
        .series_info_tabs
        .tabs
        .iter()
        .position(|tab_route| tab_route.route == right_block.into())
        .unwrap_or_default();

      SeriesDetailsHandler::with(DEFAULT_KEYBINDINGS.left.key, &mut app, right_block, None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        app.data.sonarr_data.series_info_tabs.get_active_route()
      );
      assert_eq!(app.get_current_route(), left_block.into());

      SeriesDetailsHandler::with(DEFAULT_KEYBINDINGS.right.key, &mut app, left_block, None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        app.data.sonarr_data.series_info_tabs.get_active_route()
      );
      assert_eq!(app.get_current_route(), right_block.into());
    }
  }

  mod test_handle_submit {
    use pretty_assertions::assert_eq;

    use crate::extended_stateful_iterable_vec;
    use crate::models::HorizontallyScrollableText;
    use crate::network::sonarr_network::SonarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_series_details_submit() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .seasons
        .set_items(extended_stateful_iterable_vec!(Season, Option));

      SeriesDetailsHandler::with(SUBMIT_KEY, &mut app, ActiveSonarrBlock::SeriesDetails, None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeasonDetails.into()
      );
    }

    #[test]
    fn test_series_details_submit_no_op_on_empty_seasons_table() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::SeriesDetails.into());

      SeriesDetailsHandler::with(SUBMIT_KEY, &mut app, ActiveSonarrBlock::SeriesDetails, None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeriesDetails.into()
      );
    }

    #[test]
    fn test_series_details_submit_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());

      SeriesDetailsHandler::with(SUBMIT_KEY, &mut app, ActiveSonarrBlock::SeriesDetails, None)
        .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
    }

    #[test]
    fn test_series_history_submit() {
      let mut app = App::default();
      let mut series_history = StatefulTable::default();
      series_history.set_items(vec![SonarrHistoryItem::default()]);
      app.data.sonarr_data.series_history = Some(series_history);

      SeriesDetailsHandler::with(SUBMIT_KEY, &mut app, ActiveSonarrBlock::SeriesHistory, None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeriesHistoryDetails.into()
      );
    }

    #[test]
    fn test_series_history_submit_no_op_when_series_history_is_empty() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::SeriesHistory.into());
      app.data.sonarr_data.series_history = Some(StatefulTable::default());

      SeriesDetailsHandler::with(SUBMIT_KEY, &mut app, ActiveSonarrBlock::SeriesHistory, None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeriesHistory.into()
      );
    }

    #[test]
    fn test_series_history_submit_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());

      SeriesDetailsHandler::with(SUBMIT_KEY, &mut app, ActiveSonarrBlock::SeriesHistory, None)
        .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
    }

    #[rstest]
    #[case(
      ActiveSonarrBlock::AutomaticallySearchSeriesPrompt,
      SonarrEvent::TriggerAutomaticSeriesSearch(None)
    )]
    #[case(
      ActiveSonarrBlock::UpdateAndScanSeriesPrompt,
      SonarrEvent::UpdateAndScanSeries(None)
    )]
    fn test_series_details_prompt_confirm_submit(
      #[case] prompt_block: ActiveSonarrBlock,
      #[case] expected_action: SonarrEvent,
    ) {
      let mut app = App::default();
      app.data.sonarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveSonarrBlock::SeriesDetails.into());
      app.push_navigation_stack(prompt_block.into());

      SeriesDetailsHandler::with(SUBMIT_KEY, &mut app, prompt_block, None).handle();

      assert!(app.data.sonarr_data.prompt_confirm);
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeriesDetails.into()
      );
      assert_eq!(
        app.data.sonarr_data.prompt_confirm_action,
        Some(expected_action)
      );
    }

    #[rstest]
    fn test_series_details_prompt_decline_submit(
      #[values(
        ActiveSonarrBlock::AutomaticallySearchSeriesPrompt,
        ActiveSonarrBlock::UpdateAndScanSeriesPrompt
      )]
      prompt_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::SeriesDetails.into());
      app.push_navigation_stack(prompt_block.into());

      SeriesDetailsHandler::with(SUBMIT_KEY, &mut app, prompt_block, None).handle();

      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeriesDetails.into()
      );
      assert_eq!(app.data.sonarr_data.prompt_confirm_action, None);
    }

    #[test]
    fn test_search_seasons_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::SeriesDetails.into());
      app.push_navigation_stack(ActiveSonarrBlock::SearchSeason.into());
      app
        .data
        .sonarr_data
        .seasons
        .set_items(extended_stateful_iterable_vec!(Season, Option));
      app.data.sonarr_data.seasons.search = Some("Test 2".into());

      SeriesDetailsHandler::with(SUBMIT_KEY, &mut app, ActiveSonarrBlock::SearchSeason, None)
        .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .seasons
          .current_selection()
          .title
          .as_ref()
          .unwrap(),
        "Test 2"
      );
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeriesDetails.into()
      );
    }

    #[test]
    fn test_search_seasons_submit_error_on_no_search_hits() {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveSonarrBlock::SeriesDetails.into());
      app.push_navigation_stack(ActiveSonarrBlock::SearchSeason.into());
      app
        .data
        .sonarr_data
        .seasons
        .set_items(extended_stateful_iterable_vec!(Season, Option));
      app.data.sonarr_data.seasons.search = Some("Test 5".into());

      SeriesDetailsHandler::with(SUBMIT_KEY, &mut app, ActiveSonarrBlock::SearchSeason, None)
        .handle();

      assert!(!app.should_ignore_quit_key);
      assert_str_eq!(
        app
          .data
          .sonarr_data
          .seasons
          .current_selection()
          .title
          .as_ref()
          .unwrap(),
        "Test 1"
      );
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SearchSeasonError.into()
      );
    }

    #[test]
    fn test_search_series_history_submit() {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveSonarrBlock::SeriesHistory.into());
      app.push_navigation_stack(ActiveSonarrBlock::SearchSeriesHistory.into());
      let mut series_history = StatefulTable::default();
      series_history.set_items(extended_stateful_iterable_vec!(
        SonarrHistoryItem,
        HorizontallyScrollableText,
        source_title
      ));
      series_history.search = Some("Test 2".into());
      app.data.sonarr_data.series_history = Some(series_history);

      SeriesDetailsHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::SearchSeriesHistory,
        None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert_str_eq!(
        app
          .data
          .sonarr_data
          .series_history
          .as_ref()
          .unwrap()
          .current_selection()
          .source_title
          .text,
        "Test 2"
      );
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeriesHistory.into()
      );
    }

    #[test]
    fn test_search_series_history_submit_error_on_no_search_hits() {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveSonarrBlock::SeriesHistory.into());
      app.push_navigation_stack(ActiveSonarrBlock::SearchSeriesHistory.into());
      let mut series_history = StatefulTable::default();
      series_history.set_items(extended_stateful_iterable_vec!(
        SonarrHistoryItem,
        HorizontallyScrollableText,
        source_title
      ));
      series_history.search = Some("Test 5".into());
      app.data.sonarr_data.series_history = Some(series_history);

      SeriesDetailsHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::SearchSeriesHistory,
        None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert_str_eq!(
        app
          .data
          .sonarr_data
          .series_history
          .as_ref()
          .unwrap()
          .current_selection()
          .source_title
          .text,
        "Test 1"
      );
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SearchSeriesHistoryError.into()
      );
    }

    #[test]
    fn test_filter_series_history_submit() {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveSonarrBlock::SeriesHistory.into());
      app.push_navigation_stack(ActiveSonarrBlock::FilterSeriesHistory.into());
      let mut series_history = StatefulTable::default();
      series_history.set_items(extended_stateful_iterable_vec!(
        SonarrHistoryItem,
        HorizontallyScrollableText,
        source_title
      ));
      series_history.filter = Some("Test".into());
      app.data.sonarr_data.series_history = Some(series_history);

      SeriesDetailsHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::FilterSeriesHistory,
        None,
      )
      .handle();

      assert!(app
        .data
        .sonarr_data
        .series_history
        .as_ref()
        .unwrap()
        .filtered_items
        .is_some());
      assert!(!app.should_ignore_quit_key);
      assert_eq!(
        app
          .data
          .sonarr_data
          .series_history
          .as_ref()
          .unwrap()
          .filtered_items
          .as_ref()
          .unwrap()
          .len(),
        3
      );
      assert_str_eq!(
        app
          .data
          .sonarr_data
          .series_history
          .as_ref()
          .unwrap()
          .current_selection()
          .source_title
          .text,
        "Test 1"
      );
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeriesHistory.into()
      );
    }

    #[test]
    fn test_filter_series_history_submit_error_on_no_filter_matches() {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveSonarrBlock::SeriesHistory.into());
      app.push_navigation_stack(ActiveSonarrBlock::SearchSeriesHistory.into());
      let mut series_history = StatefulTable::default();
      series_history.set_items(extended_stateful_iterable_vec!(
        SonarrHistoryItem,
        HorizontallyScrollableText,
        source_title
      ));
      series_history.filter = Some("Test 5".into());
      app.data.sonarr_data.series_history = Some(series_history);

      SeriesDetailsHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::FilterSeriesHistory,
        None,
      )
      .handle();

      assert!(!app.should_ignore_quit_key);
      assert!(app
        .data
        .sonarr_data
        .series_history
        .as_ref()
        .unwrap()
        .filtered_items
        .is_none());
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::FilterSeriesHistoryError.into()
      );
    }

    #[test]
    fn test_series_history_sort_prompt_submit() {
      let mut app = App::default();
      let mut series_history = StatefulTable::default();
      let series_history_vec = vec![
        SonarrHistoryItem {
          id: 3,
          source_title: "Test 1".into(),
          ..SonarrHistoryItem::default()
        },
        SonarrHistoryItem {
          id: 2,
          source_title: "Test 2".into(),
          ..SonarrHistoryItem::default()
        },
        SonarrHistoryItem {
          id: 1,
          source_title: "Test 3".into(),
          ..SonarrHistoryItem::default()
        },
      ];
      series_history.set_items(series_history_vec.clone());
      series_history.sorting(sort_options());
      series_history.sort_asc = true;
      app.data.sonarr_data.series_history = Some(series_history);
      app.push_navigation_stack(ActiveSonarrBlock::SeriesHistory.into());
      app.push_navigation_stack(ActiveSonarrBlock::SeriesHistorySortPrompt.into());

      let mut expected_vec = series_history_vec;
      expected_vec.sort_by(|a, b| a.id.cmp(&b.id));
      expected_vec.reverse();

      SeriesDetailsHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::SeriesHistorySortPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeriesHistory.into()
      );
      assert_eq!(
        app.data.sonarr_data.series_history.as_ref().unwrap().items,
        expected_vec
      );
    }
  }

  mod test_handle_esc {
    use super::*;
    use crate::models::servarr_data::sonarr::sonarr_data::sonarr_test_utils::utils::create_test_sonarr_data;
    use crate::models::stateful_table::StatefulTable;
    use pretty_assertions::assert_eq;
    use ratatui::widgets::TableState;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_search_season_block_esc(
      #[values(ActiveSonarrBlock::SearchSeason, ActiveSonarrBlock::SearchSeasonError)]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveSonarrBlock::SeriesDetails.into());
      app.push_navigation_stack(active_sonarr_block.into());
      app.data.sonarr_data = create_test_sonarr_data();
      app.data.sonarr_data.seasons.search = Some("Test".into());

      SeriesDetailsHandler::with(ESC_KEY, &mut app, active_sonarr_block, None).handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeriesDetails.into()
      );
      assert!(!app.should_ignore_quit_key);
      assert_eq!(app.data.sonarr_data.seasons.search, None);
    }

    #[rstest]
    fn test_search_series_history_block_esc(
      #[values(
        ActiveSonarrBlock::SearchSeriesHistory,
        ActiveSonarrBlock::SearchSeriesHistoryError
      )]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveSonarrBlock::SeriesHistory.into());
      app.push_navigation_stack(active_sonarr_block.into());
      app.data.sonarr_data = create_test_sonarr_data();
      app.data.sonarr_data.series_history.as_mut().unwrap().search = Some("Test".into());

      SeriesDetailsHandler::with(ESC_KEY, &mut app, active_sonarr_block, None).handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeriesHistory.into()
      );
      assert!(!app.should_ignore_quit_key);
      assert_eq!(
        app.data.sonarr_data.series_history.as_ref().unwrap().search,
        None
      );
    }

    #[rstest]
    fn test_filter_series_history_block_esc(
      #[values(
        ActiveSonarrBlock::FilterSeriesHistory,
        ActiveSonarrBlock::FilterSeriesHistoryError
      )]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveSonarrBlock::SeriesHistory.into());
      app.push_navigation_stack(active_sonarr_block.into());
      app.data.sonarr_data = create_test_sonarr_data();
      app.data.sonarr_data.series_history = Some(StatefulTable {
        filter: Some("Test".into()),
        filtered_items: Some(Vec::new()),
        filtered_state: Some(TableState::default()),
        ..StatefulTable::default()
      });

      SeriesDetailsHandler::with(ESC_KEY, &mut app, active_sonarr_block, None).handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeriesHistory.into()
      );
      assert!(!app.should_ignore_quit_key);
      assert_eq!(
        app.data.sonarr_data.series_history.as_ref().unwrap().filter,
        None
      );
      assert_eq!(
        app
          .data
          .sonarr_data
          .series_history
          .as_ref()
          .unwrap()
          .filtered_items,
        None
      );
      assert_eq!(
        app
          .data
          .sonarr_data
          .series_history
          .as_ref()
          .unwrap()
          .filtered_state,
        None
      );
    }

    #[test]
    fn test_series_history_sort_prompt_block_esc() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::SeriesHistory.into());
      app.push_navigation_stack(ActiveSonarrBlock::SeriesHistorySortPrompt.into());

      SeriesDetailsHandler::with(
        ESC_KEY,
        &mut app,
        ActiveSonarrBlock::SeriesHistorySortPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeriesHistory.into()
      );
    }

    #[test]
    fn test_series_history_details_block_esc() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::SeriesHistory.into());
      app.push_navigation_stack(ActiveSonarrBlock::SeriesHistoryDetails.into());

      SeriesDetailsHandler::with(
        ESC_KEY,
        &mut app,
        ActiveSonarrBlock::SeriesHistoryDetails,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeriesHistory.into()
      );
    }

    #[rstest]
    fn test_series_details_prompts_esc(
      #[values(
        ActiveSonarrBlock::AutomaticallySearchSeriesPrompt,
        ActiveSonarrBlock::UpdateAndScanSeriesPrompt
      )]
      prompt_block: ActiveSonarrBlock,
      #[values(true, false)] is_ready: bool,
    ) {
      let mut app = App::default();
      app.is_loading = is_ready;
      app.data.sonarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveSonarrBlock::SeriesDetails.into());
      app.push_navigation_stack(prompt_block.into());

      SeriesDetailsHandler::with(ESC_KEY, &mut app, prompt_block, None).handle();

      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeriesDetails.into()
      );
    }

    #[test]
    fn test_series_history_esc_resets_filter_if_one_is_set_instead_of_closing_the_window() {
      let mut app = App::default();
      let series_history = StatefulTable {
        filter: Some("Test".into()),
        filtered_items: Some(vec![SonarrHistoryItem::default()]),
        filtered_state: Some(TableState::default()),
        ..StatefulTable::default()
      };
      app.data.sonarr_data.series_history = Some(series_history);
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::SeriesHistory.into());

      SeriesDetailsHandler::with(ESC_KEY, &mut app, ActiveSonarrBlock::SeriesHistory, None)
        .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeriesHistory.into()
      );
      assert!(app
        .data
        .sonarr_data
        .series_history
        .as_ref()
        .unwrap()
        .filter
        .is_none());
      assert!(app
        .data
        .sonarr_data
        .series_history
        .as_ref()
        .unwrap()
        .filtered_items
        .is_none());
      assert!(app
        .data
        .sonarr_data
        .series_history
        .as_ref()
        .unwrap()
        .filtered_state
        .is_none());
    }
  }

  mod test_handle_key_char {
    use super::*;
    use crate::handlers::sonarr_handlers::history::history_sorting_options;
    use crate::models::servarr_data::sonarr::sonarr_data::sonarr_test_utils::utils::create_test_sonarr_data;
    use crate::models::servarr_data::sonarr::sonarr_data::SonarrData;
    use crate::models::sonarr_models::{Series, SeriesType};
    use crate::models::HorizontallyScrollableText;
    use crate::test_edit_series_key;
    use pretty_assertions::{assert_eq, assert_str_eq};
    use ratatui::widgets::TableState;
    use serde_json::Number;
    use strum::IntoEnumIterator;
    use crate::network::sonarr_network::SonarrEvent;

    #[test]
    fn test_search_season_key() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .seasons
        .set_items(vec![Season::default()]);

      SeriesDetailsHandler::with(
        DEFAULT_KEYBINDINGS.search.key,
        &mut app,
        ActiveSonarrBlock::SeriesDetails,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SearchSeason.into()
      );
      assert!(app.should_ignore_quit_key);
      assert_eq!(
        app.data.sonarr_data.seasons.search,
        Some(HorizontallyScrollableText::default())
      );
    }

    #[test]
    fn test_search_season_key_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::SeriesDetails.into());
      app
        .data
        .sonarr_data
        .seasons
        .set_items(vec![Season::default()]);

      SeriesDetailsHandler::with(
        DEFAULT_KEYBINDINGS.search.key,
        &mut app,
        ActiveSonarrBlock::SeriesDetails,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeriesDetails.into()
      );
      assert!(!app.should_ignore_quit_key);
      assert_eq!(app.data.sonarr_data.seasons.search, None);
    }

    #[test]
    fn test_search_series_history_key() {
      let mut app = App::default();
      let mut series_history = StatefulTable::default();
      series_history.set_items(vec![SonarrHistoryItem::default()]);
      app.data.sonarr_data.series_history = Some(series_history);

      SeriesDetailsHandler::with(
        DEFAULT_KEYBINDINGS.search.key,
        &mut app,
        ActiveSonarrBlock::SeriesHistory,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SearchSeriesHistory.into()
      );
      assert!(app.should_ignore_quit_key);
      assert_eq!(
        app.data.sonarr_data.series_history.as_ref().unwrap().search,
        Some(HorizontallyScrollableText::default())
      );
    }

    #[test]
    fn test_search_series_history_key_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::SeriesHistory.into());
      let mut series_history = StatefulTable::default();
      series_history.set_items(vec![SonarrHistoryItem::default()]);
      app.data.sonarr_data.series_history = Some(series_history);

      SeriesDetailsHandler::with(
        DEFAULT_KEYBINDINGS.search.key,
        &mut app,
        ActiveSonarrBlock::SeriesHistory,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeriesHistory.into()
      );
      assert!(!app.should_ignore_quit_key);
      assert_eq!(app.data.sonarr_data.seasons.search, None);
    }

    #[test]
    fn test_filter_series_history_key() {
      let mut app = App::default();
      let mut series_history = StatefulTable::default();
      series_history.set_items(vec![SonarrHistoryItem::default()]);
      app.data.sonarr_data.series_history = Some(series_history);

      SeriesDetailsHandler::with(
        DEFAULT_KEYBINDINGS.filter.key,
        &mut app,
        ActiveSonarrBlock::SeriesHistory,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::FilterSeriesHistory.into()
      );
      assert!(app.should_ignore_quit_key);
      assert_eq!(
        app.data.sonarr_data.series_history.as_ref().unwrap().filter,
        Some(HorizontallyScrollableText::default())
      );
    }

    #[test]
    fn test_filter_series_history_key_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::SeriesHistory.into());
      let mut series_history = StatefulTable::default();
      series_history.set_items(vec![SonarrHistoryItem::default()]);
      app.data.sonarr_data.series_history = Some(series_history);

      SeriesDetailsHandler::with(
        DEFAULT_KEYBINDINGS.filter.key,
        &mut app,
        ActiveSonarrBlock::SeriesHistory,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeriesHistory.into()
      );
      assert!(!app.should_ignore_quit_key);
      assert_eq!(app.data.sonarr_data.seasons.filter, None);
    }

    #[test]
    fn test_filter_series_history_key_resets_previous_filter() {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveSonarrBlock::SeriesHistory.into());
      let mut series_history = StatefulTable::default();
      series_history.set_items(vec![SonarrHistoryItem::default()]);
      series_history.filter = Some("Test".into());
      series_history.filtered_items = Some(vec![SonarrHistoryItem::default()]);
      series_history.filtered_state = Some(TableState::default());
      app.data.sonarr_data.series_history = Some(series_history);

      SeriesDetailsHandler::with(
        DEFAULT_KEYBINDINGS.filter.key,
        &mut app,
        ActiveSonarrBlock::SeriesHistory,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::FilterSeriesHistory.into()
      );
      assert!(app.should_ignore_quit_key);
      assert_eq!(
        app.data.sonarr_data.series_history.as_ref().unwrap().filter,
        Some(HorizontallyScrollableText::default())
      );
      assert!(app
        .data
        .sonarr_data
        .series_history
        .as_ref()
        .unwrap()
        .filtered_items
        .is_none());
      assert!(app
        .data
        .sonarr_data
        .series_history
        .as_ref()
        .unwrap()
        .filtered_state
        .is_none());
    }

    #[rstest]
    fn test_series_details_edit_key(
      #[values(ActiveSonarrBlock::SeriesDetails, ActiveSonarrBlock::SeriesHistory)]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      test_edit_series_key!(
        SeriesDetailsHandler,
        active_sonarr_block,
        active_sonarr_block
      );
    }

    #[rstest]
    fn test_series_edit_key_no_op_when_not_ready(
      #[values(ActiveSonarrBlock::SeriesDetails, ActiveSonarrBlock::SeriesHistory)]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(active_sonarr_block.into());

      SeriesDetailsHandler::with(
        DEFAULT_KEYBINDINGS.edit.key,
        &mut app,
        active_sonarr_block,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), active_sonarr_block.into());
      assert!(app.data.sonarr_data.edit_series_modal.is_none());
    }

    #[rstest]
    fn test_auto_search_key(
      #[values(ActiveSonarrBlock::SeriesDetails, ActiveSonarrBlock::SeriesHistory)]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      let mut series_history = StatefulTable::default();
      series_history.set_items(vec![SonarrHistoryItem::default()]);
      app.data.sonarr_data.series_history = Some(series_history);
      app.push_navigation_stack(active_sonarr_block.into());

      SeriesDetailsHandler::with(
        DEFAULT_KEYBINDINGS.auto_search.key,
        &mut app,
        active_sonarr_block,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::AutomaticallySearchSeriesPrompt.into()
      );
    }

    #[rstest]
    fn test_auto_search_key_no_op_when_not_ready(
      #[values(ActiveSonarrBlock::SeriesDetails, ActiveSonarrBlock::SeriesHistory)]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(active_sonarr_block.into());

      SeriesDetailsHandler::with(
        DEFAULT_KEYBINDINGS.auto_search.key,
        &mut app,
        active_sonarr_block,
        None,
      )
        .handle();

      assert_eq!(
        app.get_current_route(),
        active_sonarr_block.into()
      );
    }

    #[rstest]
    fn test_update_key(
      #[values(ActiveSonarrBlock::SeriesDetails, ActiveSonarrBlock::SeriesHistory)]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      let mut series_history = StatefulTable::default();
      series_history.set_items(vec![SonarrHistoryItem::default()]);
      app.data.sonarr_data.series_history = Some(series_history);
      app.push_navigation_stack(active_sonarr_block.into());

      SeriesDetailsHandler::with(
        DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        active_sonarr_block,
        None,
      )
        .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::UpdateAndScanSeriesPrompt.into()
      );
    }

    #[rstest]
    fn test_update_key_no_op_when_not_ready(
      #[values(ActiveSonarrBlock::SeriesDetails, ActiveSonarrBlock::SeriesHistory)]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(active_sonarr_block.into());

      SeriesDetailsHandler::with(
        DEFAULT_KEYBINDINGS.update.key,
        &mut app,
        active_sonarr_block,
        None,
      )
        .handle();

      assert_eq!(
        app.get_current_route(),
        active_sonarr_block.into()
      );
    }

    #[rstest]
    fn test_refresh_key(
      #[values(ActiveSonarrBlock::SeriesDetails, ActiveSonarrBlock::SeriesHistory)]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      let mut series_history = StatefulTable::default();
      series_history.set_items(vec![SonarrHistoryItem::default()]);
      app.data.sonarr_data.series_history = Some(series_history);
      app.push_navigation_stack(active_sonarr_block.into());
      app.is_routing = false;

      SeriesDetailsHandler::with(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        active_sonarr_block,
        None,
      )
        .handle();

      assert_eq!(
        app.get_current_route(),
        active_sonarr_block.into()
      );
      assert!(app.is_routing);
    }

    #[rstest]
    fn test_refresh_key_no_op_when_not_ready(
      #[values(ActiveSonarrBlock::SeriesDetails, ActiveSonarrBlock::SeriesHistory)]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(active_sonarr_block.into());
      app.is_routing = false;

      SeriesDetailsHandler::with(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        active_sonarr_block,
        None,
      )
        .handle();

      assert_eq!(
        app.get_current_route(),
        active_sonarr_block.into()
      );
      assert!(!app.is_routing);
    }

    #[test]
    fn test_sort_key() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::SeriesHistory.into());
      let mut series_history = StatefulTable::default();
      series_history.set_items(vec![SonarrHistoryItem::default()]);
      app.data.sonarr_data.series_history = Some(series_history);

      SeriesDetailsHandler::with(
        DEFAULT_KEYBINDINGS.sort.key,
        &mut app,
        ActiveSonarrBlock::SeriesHistory,
        None,
      )
        .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeriesHistorySortPrompt.into()
      );
      assert_eq!(
        app.data.sonarr_data.series_history.as_ref().unwrap().sort.as_ref().unwrap().items,
        history_sorting_options()
      );
      assert!(!app.data.sonarr_data.series_history.as_ref().unwrap().sort_asc);
    }

    #[test]
    fn test_sort_key_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::SeriesHistory.into());
      app.is_routing = false;
      let mut series_history = StatefulTable::default();
      series_history.set_items(vec![SonarrHistoryItem::default()]);
      app.data.sonarr_data.series_history = Some(series_history);

      SeriesDetailsHandler::with(
        DEFAULT_KEYBINDINGS.sort.key,
        &mut app,
        ActiveSonarrBlock::SeriesHistory,
        None,
      )
        .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::SeriesHistory.into());
      assert!(app.data.sonarr_data.series_history.as_ref().unwrap().sort.is_none());
      assert!(!app.data.sonarr_data.series_history.as_ref().unwrap().sort_asc);
      assert!(!app.is_routing);
    }

    #[test]
    fn test_search_season_box_backspace_key() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::SearchSeason.into());
      app.data.sonarr_data.seasons.search = Some("Test".into());
      app
        .data
        .sonarr_data
        .seasons
        .set_items(vec![Season::default()]);

      SeriesDetailsHandler::with(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveSonarrBlock::SearchSeason,
        None,
      )
        .handle();

      assert_str_eq!(
        app.data.sonarr_data.seasons.search.as_ref().unwrap().text,
        "Tes"
      );
    }

    #[test]
    fn test_search_series_history_box_backspace_key() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::SearchSeriesHistory.into());
      let mut series_history = StatefulTable::default();
      series_history.set_items(vec![SonarrHistoryItem::default()]);
      series_history.search = Some("Test".into());
      app.data.sonarr_data.series_history = Some(series_history);

      SeriesDetailsHandler::with(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveSonarrBlock::SearchSeriesHistory,
        None,
      )
        .handle();

      assert_str_eq!(
        app.data.sonarr_data.series_history.as_ref().unwrap().search.as_ref().unwrap().text,
        "Tes"
      );
    }

    #[test]
    fn test_filter_series_history_box_backspace_key() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::FilterSeriesHistory.into());
      let mut series_history = StatefulTable::default();
      series_history.set_items(vec![SonarrHistoryItem::default()]);
      series_history.filter = Some("Test".into());
      app.data.sonarr_data.series_history = Some(series_history);

      SeriesDetailsHandler::with(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveSonarrBlock::FilterSeriesHistory,
        None,
      )
        .handle();

      assert_str_eq!(
        app.data.sonarr_data.series_history.as_ref().unwrap().filter.as_ref().unwrap().text,
        "Tes"
      );
    }

    #[test]
    fn test_search_season_box_char_key() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::SearchSeason.into());
      app
        .data
        .sonarr_data
        .seasons
        .set_items(vec![Season::default()]);
      app.data.sonarr_data.seasons.search = Some(HorizontallyScrollableText::default());

      SeriesDetailsHandler::with(
        Key::Char('h'),
        &mut app,
        ActiveSonarrBlock::SearchSeason,
        None,
      )
        .handle();

      assert_str_eq!(
        app.data.sonarr_data.seasons.search.as_ref().unwrap().text,
        "h"
      );
    }

    #[test]
    fn test_search_series_history_box_char_key() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::SearchSeriesHistory.into());
      let mut series_history = StatefulTable::default();
      series_history.set_items(vec![SonarrHistoryItem::default()]);
      series_history.search = Some(HorizontallyScrollableText::default());
      app.data.sonarr_data.series_history = Some(series_history);

      SeriesDetailsHandler::with(
        Key::Char('h'),
        &mut app,
        ActiveSonarrBlock::SearchSeriesHistory,
        None,
      )
        .handle();

      assert_str_eq!(
        app.data.sonarr_data.series_history.as_ref().unwrap().search.as_ref().unwrap().text,
        "h"
      );
    }

    #[test]
    fn test_filter_series_history_box_char_key() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::FilterSeriesHistory.into());
      let mut series_history = StatefulTable::default();
      series_history.set_items(vec![SonarrHistoryItem::default()]);
      series_history.filter = Some(HorizontallyScrollableText::default());
      app.data.sonarr_data.series_history = Some(series_history);

      SeriesDetailsHandler::with(
        Key::Char('h'),
        &mut app,
        ActiveSonarrBlock::FilterSeriesHistory,
        None,
      )
        .handle();

      assert_str_eq!(
        app.data.sonarr_data.series_history.as_ref().unwrap().filter.as_ref().unwrap().text,
        "h"
      );
    }

    #[rstest]
    #[case(
      ActiveSonarrBlock::AutomaticallySearchSeriesPrompt,
      SonarrEvent::TriggerAutomaticSeriesSearch(None)
    )]
    #[case(
      ActiveSonarrBlock::UpdateAndScanSeriesPrompt,
      SonarrEvent::UpdateAndScanSeries(None)
    )]
    fn test_series_details_prompt_confirm_confirm_key(
      #[case] prompt_block: ActiveSonarrBlock,
      #[case] expected_action: SonarrEvent,
      #[values(ActiveSonarrBlock::SeriesDetails, ActiveSonarrBlock::SeriesHistory)] active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app.data.sonarr_data.prompt_confirm = true;
      app.push_navigation_stack(active_sonarr_block.into());
      app.push_navigation_stack(prompt_block.into());

      SeriesDetailsHandler::with(DEFAULT_KEYBINDINGS.confirm.key, &mut app, prompt_block, None).handle();

      assert!(app.data.sonarr_data.prompt_confirm);
      assert_eq!(
        app.get_current_route(),
        active_sonarr_block.into()
      );
      assert_eq!(
        app.data.sonarr_data.prompt_confirm_action,
        Some(expected_action)
      );
    }
  }

  #[test]
  fn test_series_details_handler_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if SERIES_DETAILS_BLOCKS.contains(&active_sonarr_block) {
        assert!(SeriesDetailsHandler::accepts(active_sonarr_block));
      } else {
        assert!(!SeriesDetailsHandler::accepts(active_sonarr_block));
      }
    });
  }

  #[test]
  fn test_series_details_handler_is_not_ready_when_loading() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::Series.into());
    app.is_loading = true;

    let handler = SeriesDetailsHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::SeriesDetails,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_series_details_handler_is_not_ready_when_not_loading_and_series_history_is_none() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::Series.into());

    let handler = SeriesDetailsHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::SeriesHistory,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_series_details_handler_ready_when_not_loading_and_series_history_is_some() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::Series.into());
    app.data.sonarr_data.series_history = Some(StatefulTable::default());

    let handler = SeriesDetailsHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::SeriesHistory,
      None,
    );

    assert!(handler.is_ready());
  }

  #[test]
  fn test_series_details_handler_ready_when_not_loading_for_series_details() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::Series.into());

    let handler = SeriesDetailsHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::SeriesDetails,
      None,
    );

    assert!(handler.is_ready());
  }

  fn sort_options() -> Vec<SortOption<SonarrHistoryItem>> {
    vec![SortOption {
      name: "Test 1",
      cmp_fn: Some(|a, b| {
        b.source_title
          .text
          .to_lowercase()
          .cmp(&a.source_title.text.to_lowercase())
      }),
    }]
  }
}
