#[cfg(test)]
mod tests {
  use core::sync::atomic::Ordering::SeqCst;
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
    ActiveSonarrBlock, DELETE_SERIES_BLOCKS, LIBRARY_BLOCKS,
  };
  use crate::models::sonarr_models::{Series, SeriesStatus, SeriesType};
  use crate::models::stateful_table::SortOption;
  use crate::models::HorizontallyScrollableText;
  use crate::test_handler_delegation;

  mod test_handle_scroll_up_and_down {
    use crate::{simple_stateful_iterable_vec, test_iterable_scroll};
    use pretty_assertions::assert_eq;

    use super::*;

    test_iterable_scroll!(
      test_series_scroll,
      LibraryHandler,
      sonarr_data,
      series,
      simple_stateful_iterable_vec!(Series, HorizontallyScrollableText),
      ActiveSonarrBlock::Series,
      None,
      title,
      to_string
    );

    #[rstest]
    fn test_series_scroll_no_op_when_not_ready(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::default();
      app.is_loading = true;
      app
        .data
        .sonarr_data
        .series
        .set_items(simple_stateful_iterable_vec!(
          Series,
          HorizontallyScrollableText
        ));

      LibraryHandler::with(key, &mut app, ActiveSonarrBlock::Series, None).handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .series
          .current_selection()
          .title
          .to_string(),
        "Test 1"
      );

      LibraryHandler::with(key, &mut app, ActiveSonarrBlock::Series, None).handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .series
          .current_selection()
          .title
          .to_string(),
        "Test 1"
      );
    }

    #[rstest]
    fn test_series_sort_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let series_field_vec = sort_options();
      let mut app = App::default();
      app.data.sonarr_data.series.sorting(sort_options());

      if key == Key::Up {
        for i in (0..series_field_vec.len()).rev() {
          LibraryHandler::with(key, &mut app, ActiveSonarrBlock::SeriesSortPrompt, None).handle();

          assert_eq!(
            app
              .data
              .sonarr_data
              .series
              .sort
              .as_ref()
              .unwrap()
              .current_selection(),
            &series_field_vec[i]
          );
        }
      } else {
        for i in 0..series_field_vec.len() {
          LibraryHandler::with(key, &mut app, ActiveSonarrBlock::SeriesSortPrompt, None).handle();

          assert_eq!(
            app
              .data
              .sonarr_data
              .series
              .sort
              .as_ref()
              .unwrap()
              .current_selection(),
            &series_field_vec[(i + 1) % series_field_vec.len()]
          );
        }
      }
    }
  }

  mod test_handle_home_end {
    use pretty_assertions::assert_eq;

    use crate::{extended_stateful_iterable_vec, test_iterable_home_and_end};

    use super::*;

    test_iterable_home_and_end!(
      test_series_home_end,
      LibraryHandler,
      sonarr_data,
      series,
      extended_stateful_iterable_vec!(Series, HorizontallyScrollableText),
      ActiveSonarrBlock::Series,
      None,
      title,
      to_string
    );

    #[test]
    fn test_series_home_end_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app
        .data
        .sonarr_data
        .series
        .set_items(extended_stateful_iterable_vec!(
          Series,
          HorizontallyScrollableText
        ));

      LibraryHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::Series,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .series
          .current_selection()
          .title
          .to_string(),
        "Test 1"
      );

      LibraryHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::Series,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .series
          .current_selection()
          .title
          .to_string(),
        "Test 1"
      );
    }

    #[test]
    fn test_series_search_box_home_end_keys() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);
      app.data.sonarr_data.series.search = Some("Test".into());

      LibraryHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::SearchSeries,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .series
          .search
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
        4
      );

      LibraryHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::SearchSeries,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .series
          .search
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
        0
      );
    }

    #[test]
    fn test_series_filter_box_home_end_keys() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);
      app.data.sonarr_data.series.filter = Some("Test".into());

      LibraryHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::FilterSeries,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .series
          .filter
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
        4
      );

      LibraryHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::FilterSeries,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .series
          .filter
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
        0
      );
    }

    #[test]
    fn test_series_sort_home_end() {
      let series_field_vec = sort_options();
      let mut app = App::default();
      app.data.sonarr_data.series.sorting(sort_options());

      LibraryHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::SeriesSortPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .series
          .sort
          .as_ref()
          .unwrap()
          .current_selection(),
        &series_field_vec[series_field_vec.len() - 1]
      );

      LibraryHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::SeriesSortPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .series
          .sort
          .as_ref()
          .unwrap()
          .current_selection(),
        &series_field_vec[0]
      );
    }
  }

  mod test_handle_delete {
    use pretty_assertions::assert_eq;

    use crate::assert_delete_prompt;
    use crate::models::servarr_data::sonarr::sonarr_data::DELETE_SERIES_SELECTION_BLOCKS;

    use super::*;

    const DELETE_KEY: Key = DEFAULT_KEYBINDINGS.delete.key;

    #[test]
    fn test_series_delete() {
      let mut app = App::default();
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
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);

      LibraryHandler::with(DELETE_KEY, &mut app, ActiveSonarrBlock::Series, None).handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
    }
  }

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_series_tab_left(#[values(true, false)] is_ready: bool) {
      let mut app = App::default();
      app.is_loading = is_ready;
      app.data.sonarr_data.main_tabs.set_index(0);

      LibraryHandler::with(
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
      let mut app = App::default();
      app.is_loading = is_ready;
      app.data.sonarr_data.main_tabs.set_index(0);

      LibraryHandler::with(
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
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());

      LibraryHandler::with(
        key,
        &mut app,
        ActiveSonarrBlock::UpdateAllSeriesPrompt,
        None,
      )
      .handle();

      assert!(app.data.sonarr_data.prompt_confirm);

      LibraryHandler::with(
        key,
        &mut app,
        ActiveSonarrBlock::UpdateAllSeriesPrompt,
        None,
      )
      .handle();

      assert!(!app.data.sonarr_data.prompt_confirm);
    }

    #[test]
    fn test_series_search_box_left_right_keys() {
      let mut app = App::default();
      app.data.sonarr_data.series.search = Some("Test".into());

      LibraryHandler::with(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveSonarrBlock::SearchSeries,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .series
          .search
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
        1
      );

      LibraryHandler::with(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveSonarrBlock::SearchSeries,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .series
          .search
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
        0
      );
    }

    #[test]
    fn test_series_filter_box_left_right_keys() {
      let mut app = App::default();
      app.data.sonarr_data.series.filter = Some("Test".into());

      LibraryHandler::with(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveSonarrBlock::FilterSeries,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .series
          .filter
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
        1
      );

      LibraryHandler::with(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveSonarrBlock::FilterSeries,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .series
          .filter
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
        0
      );
    }
  }

  mod test_handle_submit {
    use pretty_assertions::assert_eq;

    use crate::extended_stateful_iterable_vec;
    use crate::network::sonarr_network::SonarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_series_details_submit() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);

      LibraryHandler::with(SUBMIT_KEY, &mut app, ActiveSonarrBlock::Series, None).handle();

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
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);

      LibraryHandler::with(SUBMIT_KEY, &mut app, ActiveSonarrBlock::Series, None).handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
    }

    #[test]
    fn test_search_series_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::SearchSeries.into());
      app
        .data
        .sonarr_data
        .series
        .set_items(extended_stateful_iterable_vec!(
          Series,
          HorizontallyScrollableText
        ));
      app.data.sonarr_data.series.search = Some("Test 2".into());

      LibraryHandler::with(SUBMIT_KEY, &mut app, ActiveSonarrBlock::SearchSeries, None).handle();

      assert_str_eq!(
        app.data.sonarr_data.series.current_selection().title.text,
        "Test 2"
      );
      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
    }

    #[test]
    fn test_search_series_submit_error_on_no_search_hits() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::SearchSeries.into());
      app
        .data
        .sonarr_data
        .series
        .set_items(extended_stateful_iterable_vec!(
          Series,
          HorizontallyScrollableText
        ));
      app.data.sonarr_data.series.search = Some("Test 5".into());

      LibraryHandler::with(SUBMIT_KEY, &mut app, ActiveSonarrBlock::SearchSeries, None).handle();

      assert_str_eq!(
        app.data.sonarr_data.series.current_selection().title.text,
        "Test 1"
      );
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SearchSeriesError.into()
      );
    }

    #[test]
    fn test_search_filtered_series_submit() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::SearchSeries.into());
      app
        .data
        .sonarr_data
        .series
        .set_filtered_items(extended_stateful_iterable_vec!(
          Series,
          HorizontallyScrollableText
        ));
      app.data.sonarr_data.series.search = Some("Test 2".into());

      LibraryHandler::with(SUBMIT_KEY, &mut app, ActiveSonarrBlock::SearchSeries, None).handle();

      assert_str_eq!(
        app.data.sonarr_data.series.current_selection().title.text,
        "Test 2"
      );
      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
    }

    #[test]
    fn test_filter_series_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::FilterSeries.into());
      app
        .data
        .sonarr_data
        .series
        .set_items(extended_stateful_iterable_vec!(
          Series,
          HorizontallyScrollableText
        ));
      app.data.sonarr_data.series.filter = Some("Test".into());

      LibraryHandler::with(SUBMIT_KEY, &mut app, ActiveSonarrBlock::FilterSeries, None).handle();

      assert!(app.data.sonarr_data.series.filtered_items.is_some());
      assert!(!app.should_ignore_quit_key);
      assert_eq!(
        app
          .data
          .sonarr_data
          .series
          .filtered_items
          .as_ref()
          .unwrap()
          .len(),
        3
      );
      assert_str_eq!(
        app.data.sonarr_data.series.current_selection().title.text,
        "Test 1"
      );
      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
    }

    #[test]
    fn test_filter_series_submit_error_on_no_filter_matches() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::FilterSeries.into());
      app
        .data
        .sonarr_data
        .series
        .set_items(extended_stateful_iterable_vec!(
          Series,
          HorizontallyScrollableText
        ));
      app.data.sonarr_data.series.filter = Some("Test 5".into());

      LibraryHandler::with(SUBMIT_KEY, &mut app, ActiveSonarrBlock::FilterSeries, None).handle();

      assert!(!app.should_ignore_quit_key);
      assert!(app.data.sonarr_data.series.filtered_items.is_none());
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::FilterSeriesError.into()
      );
    }

    #[test]
    fn test_update_all_series_prompt_confirm_submit() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);
      app.data.sonarr_data.prompt_confirm = true;
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::UpdateAllSeriesPrompt.into());

      LibraryHandler::with(
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
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::UpdateAllSeriesPrompt.into());

      LibraryHandler::with(
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

    #[test]
    fn test_series_sort_prompt_submit() {
      let mut app = App::default();
      app.data.sonarr_data.series.sort_asc = true;
      app.data.sonarr_data.series.sorting(sort_options());
      app.data.sonarr_data.series.set_items(series_vec());
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::SeriesSortPrompt.into());

      let mut expected_vec = series_vec();
      expected_vec.sort_by(|a, b| a.id.cmp(&b.id));
      expected_vec.reverse();

      LibraryHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::SeriesSortPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
      assert_eq!(app.data.sonarr_data.series.items, expected_vec);
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;
    use ratatui::widgets::TableState;

    use crate::models::servarr_data::sonarr::sonarr_data::sonarr_test_utils::utils::create_test_sonarr_data;
    use crate::models::stateful_table::StatefulTable;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_search_series_block_esc(
      #[values(ActiveSonarrBlock::SearchSeries, ActiveSonarrBlock::SearchSeriesError)]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(active_sonarr_block.into());
      app.data.sonarr_data = create_test_sonarr_data();
      app.data.sonarr_data.series.search = Some("Test".into());

      LibraryHandler::with(ESC_KEY, &mut app, active_sonarr_block, None).handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
      assert!(!app.should_ignore_quit_key);
      assert_eq!(app.data.sonarr_data.series.search, None);
    }

    #[rstest]
    fn test_filter_series_block_esc(
      #[values(ActiveSonarrBlock::FilterSeries, ActiveSonarrBlock::FilterSeriesError)]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(active_sonarr_block.into());
      app.data.sonarr_data = create_test_sonarr_data();
      app.data.sonarr_data.series = StatefulTable {
        filter: Some("Test".into()),
        filtered_items: Some(Vec::new()),
        filtered_state: Some(TableState::default()),
        ..StatefulTable::default()
      };

      LibraryHandler::with(ESC_KEY, &mut app, active_sonarr_block, None).handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
      assert!(!app.should_ignore_quit_key);
      assert_eq!(app.data.sonarr_data.series.filter, None);
      assert_eq!(app.data.sonarr_data.series.filtered_items, None);
      assert_eq!(app.data.sonarr_data.series.filtered_state, None);
    }

    #[test]
    fn test_update_all_series_prompt_blocks_esc() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::UpdateAllSeriesPrompt.into());
      app.data.sonarr_data.prompt_confirm = true;

      LibraryHandler::with(
        ESC_KEY,
        &mut app,
        ActiveSonarrBlock::UpdateAllSeriesPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
      assert!(!app.data.sonarr_data.prompt_confirm);
    }

    #[test]
    fn test_series_sort_prompt_block_esc() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::SeriesSortPrompt.into());

      LibraryHandler::with(ESC_KEY, &mut app, ActiveSonarrBlock::SeriesSortPrompt, None).handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
    }

    #[rstest]
    fn test_default_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::default();
      app.is_loading = is_ready;
      app.error = "test error".to_owned().into();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data = create_test_sonarr_data();
      app.data.sonarr_data.series = StatefulTable {
        search: Some("Test".into()),
        filter: Some("Test".into()),
        filtered_items: Some(Vec::new()),
        filtered_state: Some(TableState::default()),
        ..StatefulTable::default()
      };

      LibraryHandler::with(ESC_KEY, &mut app, ActiveSonarrBlock::Series, None).handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
      assert!(app.error.text.is_empty());
      assert_eq!(app.data.sonarr_data.series.search, None);
      assert_eq!(app.data.sonarr_data.series.filter, None);
      assert_eq!(app.data.sonarr_data.series.filtered_items, None);
      assert_eq!(app.data.sonarr_data.series.filtered_state, None);
    }
  }

  mod test_handle_key_char {
    use bimap::BiMap;
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
    fn test_search_series_key() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);

      LibraryHandler::with(
        DEFAULT_KEYBINDINGS.search.key,
        &mut app,
        ActiveSonarrBlock::Series,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SearchSeries.into()
      );
      assert!(app.should_ignore_quit_key);
      assert_eq!(
        app.data.sonarr_data.series.search,
        Some(HorizontallyScrollableText::default())
      );
    }

    #[test]
    fn test_search_series_key_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);

      LibraryHandler::with(
        DEFAULT_KEYBINDINGS.search.key,
        &mut app,
        ActiveSonarrBlock::Series,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
      assert!(!app.should_ignore_quit_key);
      assert_eq!(app.data.sonarr_data.series.search, None);
    }

    #[test]
    fn test_filter_series_key() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);

      LibraryHandler::with(
        DEFAULT_KEYBINDINGS.filter.key,
        &mut app,
        ActiveSonarrBlock::Series,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::FilterSeries.into()
      );
      assert!(app.should_ignore_quit_key);
      assert!(app.data.sonarr_data.series.filter.is_some());
    }

    #[test]
    fn test_filter_series_key_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);

      LibraryHandler::with(
        DEFAULT_KEYBINDINGS.filter.key,
        &mut app,
        ActiveSonarrBlock::Series,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
      assert!(!app.should_ignore_quit_key);
      assert!(app.data.sonarr_data.series.filter.is_none());
    }

    #[test]
    fn test_filter_series_key_resets_previous_filter() {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data = create_test_sonarr_data();
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);
      app.data.sonarr_data.series.filter = Some("Test".into());

      LibraryHandler::with(
        DEFAULT_KEYBINDINGS.filter.key,
        &mut app,
        ActiveSonarrBlock::Series,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::FilterSeries.into()
      );
      assert!(app.should_ignore_quit_key);
      assert_eq!(
        app.data.sonarr_data.series.filter,
        Some(HorizontallyScrollableText::default())
      );
      assert!(app.data.sonarr_data.series.filtered_items.is_none());
      assert!(app.data.sonarr_data.series.filtered_state.is_none());
    }

    #[test]
    fn test_series_add_key() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);

      LibraryHandler::with(
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
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);

      LibraryHandler::with(
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
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);

      LibraryHandler::with(
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
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);

      LibraryHandler::with(
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
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);

      LibraryHandler::with(
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
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());

      LibraryHandler::with(
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
      let mut app = App::default();
      app.is_loading = true;
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());

      LibraryHandler::with(
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
    fn test_search_series_box_backspace_key() {
      let mut app = App::default();
      app.data.sonarr_data.series.search = Some("Test".into());
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);

      LibraryHandler::with(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveSonarrBlock::SearchSeries,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.sonarr_data.series.search.as_ref().unwrap().text,
        "Tes"
      );
    }

    #[test]
    fn test_filter_series_box_backspace_key() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);
      app.data.sonarr_data.series.filter = Some("Test".into());

      LibraryHandler::with(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveSonarrBlock::FilterSeries,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.sonarr_data.series.filter.as_ref().unwrap().text,
        "Tes"
      );
    }

    #[test]
    fn test_search_series_box_char_key() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);
      app.data.sonarr_data.series.search = Some(HorizontallyScrollableText::default());

      LibraryHandler::with(
        Key::Char('h'),
        &mut app,
        ActiveSonarrBlock::SearchSeries,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.sonarr_data.series.search.as_ref().unwrap().text,
        "h"
      );
    }

    #[test]
    fn test_filter_series_box_char_key() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);
      app.data.sonarr_data.series.filter = Some(HorizontallyScrollableText::default());

      LibraryHandler::with(
        Key::Char('h'),
        &mut app,
        ActiveSonarrBlock::FilterSeries,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.sonarr_data.series.filter.as_ref().unwrap().text,
        "h"
      );
    }

    #[test]
    fn test_sort_key() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);

      LibraryHandler::with(
        DEFAULT_KEYBINDINGS.sort.key,
        &mut app,
        ActiveSonarrBlock::Series,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SeriesSortPrompt.into()
      );
      assert_eq!(
        app.data.sonarr_data.series.sort.as_ref().unwrap().items,
        series_sorting_options()
      );
      assert!(!app.data.sonarr_data.series.sort_asc);
    }

    #[test]
    fn test_sort_key_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);

      LibraryHandler::with(
        DEFAULT_KEYBINDINGS.sort.key,
        &mut app,
        ActiveSonarrBlock::Series,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Series.into());
      assert!(app.data.sonarr_data.series.sort.is_none());
    }

    #[test]
    fn test_update_all_series_prompt_confirm() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .series
        .set_items(vec![Series::default()]);
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::UpdateAllSeriesPrompt.into());

      LibraryHandler::with(
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

  // #[rstest]
  // fn test_delegates_add_series_blocks_to_add_series_handler(
  //   #[values(
  //     ActiveSonarrBlock::AddSeriesSearchInput,
  //     ActiveSonarrBlock::AddSeriesSearchResults,
  //     ActiveSonarrBlock::AddSeriesPrompt,
  //     ActiveSonarrBlock::AddSeriesSelectMonitor,
  //     ActiveSonarrBlock::AddSeriesSelectSeriesType,
  //     ActiveSonarrBlock::AddSeriesSelectQualityProfile,
  //     ActiveSonarrBlock::AddSeriesSelectRootFolder,
  //     ActiveSonarrBlock::AddSeriesAlreadyInLibrary,
  //     ActiveSonarrBlock::AddSeriesTagsInput
  //   )]
  //   active_sonarr_block: ActiveSonarrBlock,
  // ) {
  //   test_handler_delegation!(
  //     LibraryHandler,
  //     ActiveSonarrBlock::Series,
  //     active_sonarr_block
  //   );
  // }

  // #[rstest]
  // fn test_delegates_series_details_blocks_to_series_details_handler(
  //   #[values(
  //     ActiveSonarrBlock::SeriesDetails,
  //     ActiveSonarrBlock::SeriesHistory,
  //     ActiveSonarrBlock::FileInfo,
  //     ActiveSonarrBlock::Cast,
  //     ActiveSonarrBlock::Crew,
  //     ActiveSonarrBlock::AutomaticallySearchSeriesPrompt,
  //     ActiveSonarrBlock::UpdateAndScanPrompt,
  //     ActiveSonarrBlock::ManualSearch,
  //     ActiveSonarrBlock::ManualSearchConfirmPrompt
  //   )]
  //   active_sonarr_block: ActiveSonarrBlock,
  // ) {
  //   test_handler_delegation!(
  //     LibraryHandler,
  //     ActiveSonarrBlock::Series,
  //     active_sonarr_block
  //   );
  // }

  // #[rstest]
  // fn test_delegates_edit_series_blocks_to_edit_series_handler(
  //   #[values(
  //     ActiveSonarrBlock::EditSeriesPrompt,
  //     ActiveSonarrBlock::EditSeriesPathInput,
  //     ActiveSonarrBlock::EditSeriesSelectMinimumAvailability,
  //     ActiveSonarrBlock::EditSeriesSelectQualityProfile,
  //     ActiveSonarrBlock::EditSeriesTagsInput
  //   )]
  //   active_sonarr_block: ActiveSonarrBlock,
  // ) {
  //   test_handler_delegation!(
  //     LibraryHandler,
  //     ActiveSonarrBlock::Series,
  //     active_sonarr_block
  //   );
  // }

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
    library_handler_blocks.extend(DELETE_SERIES_BLOCKS);

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
    let mut app = App::default();
    app.is_loading = true;

    let handler = LibraryHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::Series,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_library_handler_not_ready_when_series_is_empty() {
    let mut app = App::default();
    app.is_loading = false;

    let handler = LibraryHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::Series,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_library_handler_ready_when_not_loading_and_series_is_not_empty() {
    let mut app = App::default();
    app.is_loading = false;
    app
      .data
      .sonarr_data
      .series
      .set_items(vec![Series::default()]);

    let handler = LibraryHandler::with(
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

  fn sort_options() -> Vec<SortOption<Series>> {
    vec![SortOption {
      name: "Test 1",
      cmp_fn: Some(|a, b| {
        b.title
          .text
          .to_lowercase()
          .cmp(&a.title.text.to_lowercase())
      }),
    }]
  }
}
