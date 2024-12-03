#[cfg(test)]
mod tests {
  use core::sync::atomic::Ordering::SeqCst;
  use std::cmp::Ordering;

  use chrono::DateTime;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use strum::IntoEnumIterator;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::sonarr_handlers::history::{history_sorting_options, HistoryHandler};
  use crate::handlers::KeyEventHandler;
  use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, HISTORY_BLOCKS};
  use crate::models::servarr_models::{Language, Quality, QualityWrapper};
  use crate::models::sonarr_models::SonarrHistoryItem;
  use crate::models::stateful_table::SortOption;
  use crate::models::HorizontallyScrollableText;

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::{assert_eq, assert_str_eq};
    use rstest::rstest;

    use crate::models::sonarr_models::SonarrHistoryItem;
    use crate::{simple_stateful_iterable_vec, test_iterable_scroll};

    use super::*;

    test_iterable_scroll!(
      test_history_scroll,
      HistoryHandler,
      sonarr_data,
      history,
      simple_stateful_iterable_vec!(SonarrHistoryItem, HorizontallyScrollableText, source_title),
      ActiveSonarrBlock::History,
      None,
      source_title,
      to_string
    );

    #[rstest]
    fn test_history_scroll_no_op_when_not_ready(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::History.into());
      app.is_loading = true;
      app
        .data
        .sonarr_data
        .history
        .set_items(simple_stateful_iterable_vec!(
          SonarrHistoryItem,
          HorizontallyScrollableText,
          source_title
        ));

      HistoryHandler::with(key, &mut app, ActiveSonarrBlock::History, None).handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .history
          .current_selection()
          .source_title
          .to_string(),
        "Test 1"
      );

      HistoryHandler::with(key, &mut app, ActiveSonarrBlock::History, None).handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .history
          .current_selection()
          .source_title
          .to_string(),
        "Test 1"
      );
    }

    #[rstest]
    fn test_history_sort_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let history_field_vec = sort_options();
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::History.into());
      app.data.sonarr_data.history.sorting(sort_options());

      if key == Key::Up {
        for i in (0..history_field_vec.len()).rev() {
          HistoryHandler::with(key, &mut app, ActiveSonarrBlock::HistorySortPrompt, None).handle();

          assert_eq!(
            app
              .data
              .sonarr_data
              .history
              .sort
              .as_ref()
              .unwrap()
              .current_selection(),
            &history_field_vec[i]
          );
        }
      } else {
        for i in 0..history_field_vec.len() {
          HistoryHandler::with(key, &mut app, ActiveSonarrBlock::HistorySortPrompt, None).handle();

          assert_eq!(
            app
              .data
              .sonarr_data
              .history
              .sort
              .as_ref()
              .unwrap()
              .current_selection(),
            &history_field_vec[(i + 1) % history_field_vec.len()]
          );
        }
      }
    }
  }

  mod test_handle_home_end {
    use pretty_assertions::{assert_eq, assert_str_eq};

    use crate::models::sonarr_models::SonarrHistoryItem;
    use crate::{extended_stateful_iterable_vec, test_iterable_home_and_end};

    use super::*;

    test_iterable_home_and_end!(
      test_history_home_and_end,
      HistoryHandler,
      sonarr_data,
      history,
      extended_stateful_iterable_vec!(SonarrHistoryItem, HorizontallyScrollableText, source_title),
      ActiveSonarrBlock::History,
      None,
      source_title,
      to_string
    );

    #[test]
    fn test_history_home_and_end_no_op_when_not_ready() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::History.into());
      app.is_loading = true;
      app
        .data
        .sonarr_data
        .history
        .set_items(extended_stateful_iterable_vec!(
          SonarrHistoryItem,
          HorizontallyScrollableText,
          source_title
        ));

      HistoryHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::History,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .history
          .current_selection()
          .source_title
          .to_string(),
        "Test 1"
      );

      HistoryHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::History,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .history
          .current_selection()
          .source_title
          .to_string(),
        "Test 1"
      );
    }

    #[test]
    fn test_history_search_box_home_end_keys() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .history
        .set_items(vec![SonarrHistoryItem::default()]);
      app.data.sonarr_data.history.search = Some("Test".into());

      HistoryHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::SearchHistory,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .history
          .search
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
        4
      );

      HistoryHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::SearchHistory,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .history
          .search
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
        0
      );
    }

    #[test]
    fn test_history_filter_box_home_end_keys() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .history
        .set_items(vec![SonarrHistoryItem::default()]);
      app.data.sonarr_data.history.filter = Some("Test".into());

      HistoryHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::FilterHistory,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .history
          .filter
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
        4
      );

      HistoryHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::FilterHistory,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .history
          .filter
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
        0
      );
    }

    #[test]
    fn test_history_sort_home_end() {
      let history_field_vec = sort_options();
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::History.into());
      app.data.sonarr_data.history.sorting(sort_options());

      HistoryHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::HistorySortPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .history
          .sort
          .as_ref()
          .unwrap()
          .current_selection(),
        &history_field_vec[history_field_vec.len() - 1]
      );

      HistoryHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::HistorySortPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .history
          .sort
          .as_ref()
          .unwrap()
          .current_selection(),
        &history_field_vec[0]
      );
    }
  }

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_history_tab_left(#[values(true, false)] is_ready: bool) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::History.into());
      app.is_loading = is_ready;
      app.data.sonarr_data.main_tabs.set_index(3);

      HistoryHandler::with(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveSonarrBlock::History,
        None,
      )
      .handle();

      assert_eq!(
        app.data.sonarr_data.main_tabs.get_active_route(),
        ActiveSonarrBlock::Blocklist.into()
      );
      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Blocklist.into());
    }

    #[rstest]
    fn test_history_tab_right(#[values(true, false)] is_ready: bool) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::History.into());
      app.is_loading = is_ready;
      app.data.sonarr_data.main_tabs.set_index(3);

      HistoryHandler::with(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveSonarrBlock::History,
        None,
      )
      .handle();

      assert_eq!(
        app.data.sonarr_data.main_tabs.get_active_route(),
        ActiveSonarrBlock::RootFolders.into()
      );
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::RootFolders.into()
      );
    }

    #[test]
    fn test_history_search_box_left_right_keys() {
      let mut app = App::default();
      app.data.sonarr_data.history.search = Some("Test".into());

      HistoryHandler::with(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveSonarrBlock::SearchHistory,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .history
          .search
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
        1
      );

      HistoryHandler::with(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveSonarrBlock::SearchHistory,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .history
          .search
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
        0
      );
    }

    #[test]
    fn test_history_filter_box_left_right_keys() {
      let mut app = App::default();
      app.data.sonarr_data.history.filter = Some("Test".into());

      HistoryHandler::with(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveSonarrBlock::FilterHistory,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .history
          .filter
          .as_ref()
          .unwrap()
          .offset
          .load(SeqCst),
        1
      );

      HistoryHandler::with(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveSonarrBlock::FilterHistory,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .history
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

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_history_submit() {
      let mut app = App::default();
      app.data.sonarr_data.history.set_items(history_vec());
      app.push_navigation_stack(ActiveSonarrBlock::History.into());

      HistoryHandler::with(SUBMIT_KEY, &mut app, ActiveSonarrBlock::History, None).handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::HistoryItemDetails.into()
      );
    }

    #[test]
    fn test_history_submit_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.data.sonarr_data.history.set_items(history_vec());
      app.push_navigation_stack(ActiveSonarrBlock::History.into());

      HistoryHandler::with(SUBMIT_KEY, &mut app, ActiveSonarrBlock::History, None).handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::History.into());
    }

    #[test]
    fn test_search_history_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::History.into());
      app.push_navigation_stack(ActiveSonarrBlock::SearchHistory.into());
      app
        .data
        .sonarr_data
        .history
        .set_items(extended_stateful_iterable_vec!(
          SonarrHistoryItem,
          HorizontallyScrollableText,
          source_title
        ));
      app.data.sonarr_data.history.search = Some("Test 2".into());

      HistoryHandler::with(SUBMIT_KEY, &mut app, ActiveSonarrBlock::SearchHistory, None).handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .history
          .current_selection()
          .source_title
          .text,
        "Test 2"
      );
      assert_eq!(app.get_current_route(), ActiveSonarrBlock::History.into());
    }

    #[test]
    fn test_search_history_submit_error_on_no_search_hits() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::History.into());
      app.push_navigation_stack(ActiveSonarrBlock::SearchHistory.into());
      app
        .data
        .sonarr_data
        .history
        .set_items(extended_stateful_iterable_vec!(
          SonarrHistoryItem,
          HorizontallyScrollableText,
          source_title
        ));
      app.data.sonarr_data.history.search = Some("Test 5".into());

      HistoryHandler::with(SUBMIT_KEY, &mut app, ActiveSonarrBlock::SearchHistory, None).handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .history
          .current_selection()
          .source_title
          .text,
        "Test 1"
      );
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SearchHistoryError.into()
      );
    }

    #[test]
    fn test_search_filtered_history_submit() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .history
        .set_items(vec![SonarrHistoryItem::default()]);
      app.push_navigation_stack(ActiveSonarrBlock::History.into());
      app.push_navigation_stack(ActiveSonarrBlock::SearchHistory.into());
      app
        .data
        .sonarr_data
        .history
        .set_filtered_items(extended_stateful_iterable_vec!(
          SonarrHistoryItem,
          HorizontallyScrollableText,
          source_title
        ));
      app.data.sonarr_data.history.search = Some("Test 2".into());

      HistoryHandler::with(SUBMIT_KEY, &mut app, ActiveSonarrBlock::SearchHistory, None).handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .history
          .current_selection()
          .source_title
          .text,
        "Test 2"
      );
      assert_eq!(app.get_current_route(), ActiveSonarrBlock::History.into());
    }

    #[test]
    fn test_filter_history_submit() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::History.into());
      app.push_navigation_stack(ActiveSonarrBlock::FilterHistory.into());
      app
        .data
        .sonarr_data
        .history
        .set_items(extended_stateful_iterable_vec!(
          SonarrHistoryItem,
          HorizontallyScrollableText,
          source_title
        ));
      app.data.sonarr_data.history.filter = Some("Test".into());

      HistoryHandler::with(SUBMIT_KEY, &mut app, ActiveSonarrBlock::FilterHistory, None).handle();

      assert!(app.data.sonarr_data.history.filtered_items.is_some());
      assert!(!app.should_ignore_quit_key);
      assert_eq!(
        app
          .data
          .sonarr_data
          .history
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
          .history
          .current_selection()
          .source_title
          .text,
        "Test 1"
      );
      assert_eq!(app.get_current_route(), ActiveSonarrBlock::History.into());
    }

    #[test]
    fn test_filter_history_submit_error_on_no_filter_matches() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::History.into());
      app.push_navigation_stack(ActiveSonarrBlock::FilterHistory.into());
      app
        .data
        .sonarr_data
        .history
        .set_items(extended_stateful_iterable_vec!(
          SonarrHistoryItem,
          HorizontallyScrollableText,
          source_title
        ));
      app.data.sonarr_data.history.filter = Some("Test 5".into());

      HistoryHandler::with(SUBMIT_KEY, &mut app, ActiveSonarrBlock::FilterHistory, None).handle();

      assert!(!app.should_ignore_quit_key);
      assert!(app.data.sonarr_data.history.filtered_items.is_none());
      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::FilterHistoryError.into()
      );
    }

    #[test]
    fn test_history_sort_prompt_submit() {
      let mut app = App::default();
      app.data.sonarr_data.history.sort_asc = true;
      app.data.sonarr_data.history.sorting(sort_options());
      app.data.sonarr_data.history.set_items(history_vec());
      app.push_navigation_stack(ActiveSonarrBlock::History.into());
      app.push_navigation_stack(ActiveSonarrBlock::HistorySortPrompt.into());

      let mut expected_vec = history_vec();
      expected_vec.sort_by(|a, b| a.id.cmp(&b.id));
      expected_vec.reverse();

      HistoryHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::HistorySortPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::History.into());
      assert_eq!(app.data.sonarr_data.history.items, expected_vec);
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;
    use ratatui::widgets::TableState;
    use rstest::rstest;

    use crate::models::{
      servarr_data::sonarr::sonarr_data::sonarr_test_utils::utils::create_test_sonarr_data,
      stateful_table::StatefulTable,
    };

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_search_history_block_esc(
      #[values(
        ActiveSonarrBlock::SearchHistory,
        ActiveSonarrBlock::SearchHistoryError
      )]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveSonarrBlock::History.into());
      app.push_navigation_stack(active_sonarr_block.into());
      app.data.sonarr_data = create_test_sonarr_data();
      app.data.sonarr_data.history.search = Some("Test".into());

      HistoryHandler::with(ESC_KEY, &mut app, active_sonarr_block, None).handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::History.into());
      assert!(!app.should_ignore_quit_key);
      assert_eq!(app.data.sonarr_data.history.search, None);
    }

    #[rstest]
    fn test_filter_history_block_esc(
      #[values(
        ActiveSonarrBlock::FilterHistory,
        ActiveSonarrBlock::FilterHistoryError
      )]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveSonarrBlock::History.into());
      app.push_navigation_stack(active_sonarr_block.into());
      app.data.sonarr_data = create_test_sonarr_data();
      app.data.sonarr_data.history = StatefulTable {
        filter: Some("Test".into()),
        filtered_items: Some(Vec::new()),
        filtered_state: Some(TableState::default()),
        ..StatefulTable::default()
      };

      HistoryHandler::with(ESC_KEY, &mut app, active_sonarr_block, None).handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::History.into());
      assert!(!app.should_ignore_quit_key);
      assert_eq!(app.data.sonarr_data.history.filter, None);
      assert_eq!(app.data.sonarr_data.history.filtered_items, None);
      assert_eq!(app.data.sonarr_data.history.filtered_state, None);
    }

    #[test]
    fn test_esc_history_item_details() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::History.into());
      app.push_navigation_stack(ActiveSonarrBlock::HistoryItemDetails.into());

      HistoryHandler::with(
        ESC_KEY,
        &mut app,
        ActiveSonarrBlock::HistoryItemDetails,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::History.into());
    }

    #[test]
    fn test_history_sort_prompt_block_esc() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::History.into());
      app.push_navigation_stack(ActiveSonarrBlock::HistorySortPrompt.into());

      HistoryHandler::with(
        ESC_KEY,
        &mut app,
        ActiveSonarrBlock::HistorySortPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::History.into());
    }

    #[rstest]
    fn test_default_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::default();
      app.is_loading = is_ready;
      app.error = "test error".to_owned().into();
      app.push_navigation_stack(ActiveSonarrBlock::History.into());
      app.push_navigation_stack(ActiveSonarrBlock::History.into());

      HistoryHandler::with(ESC_KEY, &mut app, ActiveSonarrBlock::History, None).handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::History.into());
      assert!(app.error.text.is_empty());
    }
  }

  mod test_handle_key_char {
    use pretty_assertions::assert_eq;

    use crate::models::servarr_data::sonarr::sonarr_data::sonarr_test_utils::utils::create_test_sonarr_data;

    use super::*;

    #[test]
    fn test_search_history_key() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .history
        .set_items(vec![SonarrHistoryItem::default()]);

      HistoryHandler::with(
        DEFAULT_KEYBINDINGS.search.key,
        &mut app,
        ActiveSonarrBlock::History,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::SearchHistory.into()
      );
      assert!(app.should_ignore_quit_key);
      assert_eq!(
        app.data.sonarr_data.history.search,
        Some(HorizontallyScrollableText::default())
      );
    }

    #[test]
    fn test_search_history_key_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::History.into());
      app
        .data
        .sonarr_data
        .history
        .set_items(vec![SonarrHistoryItem::default()]);

      HistoryHandler::with(
        DEFAULT_KEYBINDINGS.search.key,
        &mut app,
        ActiveSonarrBlock::History,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::History.into());
      assert!(!app.should_ignore_quit_key);
      assert_eq!(app.data.sonarr_data.history.search, None);
    }

    #[test]
    fn test_filter_history_key() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .history
        .set_items(vec![SonarrHistoryItem::default()]);

      HistoryHandler::with(
        DEFAULT_KEYBINDINGS.filter.key,
        &mut app,
        ActiveSonarrBlock::History,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::FilterHistory.into()
      );
      assert!(app.should_ignore_quit_key);
      assert!(app.data.sonarr_data.history.filter.is_some());
    }

    #[test]
    fn test_filter_history_key_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::History.into());
      app
        .data
        .sonarr_data
        .history
        .set_items(vec![SonarrHistoryItem::default()]);

      HistoryHandler::with(
        DEFAULT_KEYBINDINGS.filter.key,
        &mut app,
        ActiveSonarrBlock::History,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::History.into());
      assert!(!app.should_ignore_quit_key);
      assert!(app.data.sonarr_data.history.filter.is_none());
    }

    #[test]
    fn test_filter_history_key_resets_previous_filter() {
      let mut app = App::default();
      app.should_ignore_quit_key = true;
      app.push_navigation_stack(ActiveSonarrBlock::History.into());
      app.data.sonarr_data = create_test_sonarr_data();
      app
        .data
        .sonarr_data
        .history
        .set_items(vec![SonarrHistoryItem::default()]);
      app.data.sonarr_data.history.filter = Some("Test".into());

      HistoryHandler::with(
        DEFAULT_KEYBINDINGS.filter.key,
        &mut app,
        ActiveSonarrBlock::History,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::FilterHistory.into()
      );
      assert!(app.should_ignore_quit_key);
      assert_eq!(
        app.data.sonarr_data.history.filter,
        Some(HorizontallyScrollableText::default())
      );
      assert!(app.data.sonarr_data.history.filtered_items.is_none());
      assert!(app.data.sonarr_data.history.filtered_state.is_none());
    }

    #[test]
    fn test_refresh_history_key() {
      let mut app = App::default();
      app.data.sonarr_data.history.set_items(history_vec());
      app.push_navigation_stack(ActiveSonarrBlock::History.into());

      HistoryHandler::with(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveSonarrBlock::History,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::History.into());
      assert!(app.should_refresh);
    }

    #[test]
    fn test_refresh_history_key_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.data.sonarr_data.history.set_items(history_vec());
      app.push_navigation_stack(ActiveSonarrBlock::History.into());

      HistoryHandler::with(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveSonarrBlock::History,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::History.into());
      assert!(!app.should_refresh);
    }

    #[test]
    fn test_search_history_box_backspace_key() {
      let mut app = App::default();
      app.data.sonarr_data.history.search = Some("Test".into());
      app
        .data
        .sonarr_data
        .history
        .set_items(vec![SonarrHistoryItem::default()]);

      HistoryHandler::with(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveSonarrBlock::SearchHistory,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.sonarr_data.history.search.as_ref().unwrap().text,
        "Tes"
      );
    }

    #[test]
    fn test_filter_history_box_backspace_key() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .history
        .set_items(vec![SonarrHistoryItem::default()]);
      app.data.sonarr_data.history.filter = Some("Test".into());

      HistoryHandler::with(
        DEFAULT_KEYBINDINGS.backspace.key,
        &mut app,
        ActiveSonarrBlock::FilterHistory,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.sonarr_data.history.filter.as_ref().unwrap().text,
        "Tes"
      );
    }

    #[test]
    fn test_search_history_box_char_key() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .history
        .set_items(vec![SonarrHistoryItem::default()]);
      app.data.sonarr_data.history.search = Some(HorizontallyScrollableText::default());

      HistoryHandler::with(
        Key::Char('h'),
        &mut app,
        ActiveSonarrBlock::SearchHistory,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.sonarr_data.history.search.as_ref().unwrap().text,
        "h"
      );
    }

    #[test]
    fn test_filter_history_box_char_key() {
      let mut app = App::default();
      app
        .data
        .sonarr_data
        .history
        .set_items(vec![SonarrHistoryItem::default()]);
      app.data.sonarr_data.history.filter = Some(HorizontallyScrollableText::default());

      HistoryHandler::with(
        Key::Char('h'),
        &mut app,
        ActiveSonarrBlock::FilterHistory,
        None,
      )
      .handle();

      assert_str_eq!(
        app.data.sonarr_data.history.filter.as_ref().unwrap().text,
        "h"
      );
    }

    #[test]
    fn test_sort_key() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::History.into());
      app.data.sonarr_data.history.set_items(history_vec());

      HistoryHandler::with(
        DEFAULT_KEYBINDINGS.sort.key,
        &mut app,
        ActiveSonarrBlock::History,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::HistorySortPrompt.into()
      );
      assert_eq!(
        app.data.sonarr_data.history.sort.as_ref().unwrap().items,
        history_sorting_options()
      );
      assert!(!app.data.sonarr_data.history.sort_asc);
    }

    #[test]
    fn test_sort_key_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::History.into());
      app.data.sonarr_data.history.set_items(history_vec());

      HistoryHandler::with(
        DEFAULT_KEYBINDINGS.sort.key,
        &mut app,
        ActiveSonarrBlock::History,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::History.into());
      assert!(app.data.sonarr_data.history.sort.is_none());
      assert!(!app.data.sonarr_data.history.sort_asc);
    }
  }

  #[test]
  fn test_history_sorting_options_source_title() {
    let expected_cmp_fn: fn(&SonarrHistoryItem, &SonarrHistoryItem) -> Ordering = |a, b| {
      a.source_title
        .text
        .to_lowercase()
        .cmp(&b.source_title.text.to_lowercase())
    };
    let mut expected_history_vec = history_vec();
    expected_history_vec.sort_by(expected_cmp_fn);

    let sort_option = history_sorting_options()[0].clone();
    let mut sorted_history_vec = history_vec();
    sorted_history_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_history_vec, expected_history_vec);
    assert_str_eq!(sort_option.name, "Source Title");
  }

  #[test]
  fn test_history_sorting_options_event_type() {
    let expected_cmp_fn: fn(&SonarrHistoryItem, &SonarrHistoryItem) -> Ordering = |a, b| {
      a.event_type
        .to_lowercase()
        .cmp(&b.event_type.to_lowercase())
    };
    let mut expected_history_vec = history_vec();
    expected_history_vec.sort_by(expected_cmp_fn);

    let sort_option = history_sorting_options()[1].clone();
    let mut sorted_history_vec = history_vec();
    sorted_history_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_history_vec, expected_history_vec);
    assert_str_eq!(sort_option.name, "Event Type");
  }

  #[test]
  fn test_history_sorting_options_language() {
    let expected_cmp_fn: fn(&SonarrHistoryItem, &SonarrHistoryItem) -> Ordering = |a, b| {
      a.language
        .name
        .to_lowercase()
        .cmp(&b.language.name.to_lowercase())
    };
    let mut expected_history_vec = history_vec();
    expected_history_vec.sort_by(expected_cmp_fn);

    let sort_option = history_sorting_options()[2].clone();
    let mut sorted_history_vec = history_vec();
    sorted_history_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_history_vec, expected_history_vec);
    assert_str_eq!(sort_option.name, "Language");
  }

  #[test]
  fn test_history_sorting_options_quality() {
    let expected_cmp_fn: fn(&SonarrHistoryItem, &SonarrHistoryItem) -> Ordering = |a, b| {
      a.quality
        .quality
        .name
        .to_lowercase()
        .cmp(&b.quality.quality.name.to_lowercase())
    };
    let mut expected_history_vec = history_vec();
    expected_history_vec.sort_by(expected_cmp_fn);

    let sort_option = history_sorting_options()[3].clone();
    let mut sorted_history_vec = history_vec();
    sorted_history_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_history_vec, expected_history_vec);
    assert_str_eq!(sort_option.name, "Quality");
  }

  #[test]
  fn test_history_sorting_options_date() {
    let expected_cmp_fn: fn(&SonarrHistoryItem, &SonarrHistoryItem) -> Ordering =
      |a, b| a.date.cmp(&b.date);
    let mut expected_history_vec = history_vec();
    expected_history_vec.sort_by(expected_cmp_fn);

    let sort_option = history_sorting_options()[4].clone();
    let mut sorted_history_vec = history_vec();
    sorted_history_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_history_vec, expected_history_vec);
    assert_str_eq!(sort_option.name, "Date");
  }

  #[test]
  fn test_history_handler_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if HISTORY_BLOCKS.contains(&active_sonarr_block) {
        assert!(HistoryHandler::accepts(active_sonarr_block));
      } else {
        assert!(!HistoryHandler::accepts(active_sonarr_block));
      }
    })
  }

  #[test]
  fn test_history_handler_not_ready_when_loading() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::History.into());
    app.is_loading = true;

    let handler = HistoryHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::History,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_history_handler_not_ready_when_history_is_empty() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::History.into());
    app.is_loading = false;

    let handler = HistoryHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::History,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_history_handler_ready_when_not_loading_and_history_is_not_empty() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::History.into());
    app.is_loading = false;
    app
      .data
      .sonarr_data
      .history
      .set_items(vec![SonarrHistoryItem::default()]);

    let handler = HistoryHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::History,
      None,
    );

    assert!(handler.is_ready());
  }

  fn history_vec() -> Vec<SonarrHistoryItem> {
    vec![
      SonarrHistoryItem {
        id: 3,
        source_title: "test 1".into(),
        event_type: "grabbed".to_owned(),
        language: Language {
          id: 1,
          name: "telgu".to_owned(),
        },
        quality: QualityWrapper {
          quality: Quality {
            name: "HD - 1080p".to_owned(),
          },
        },
        date: DateTime::from(DateTime::parse_from_rfc3339("2024-01-10T07:28:45Z").unwrap()),
        ..SonarrHistoryItem::default()
      },
      SonarrHistoryItem {
        id: 2,
        source_title: "test 2".into(),
        event_type: "downloadFolderImported".to_owned(),
        language: Language {
          id: 3,
          name: "chinese".to_owned(),
        },
        quality: QualityWrapper {
          quality: Quality {
            name: "SD - 720p".to_owned(),
          },
        },
        date: DateTime::from(DateTime::parse_from_rfc3339("2024-02-10T07:28:45Z").unwrap()),
        ..SonarrHistoryItem::default()
      },
      SonarrHistoryItem {
        id: 1,
        source_title: "test 3".into(),
        event_type: "episodeFileDeleted".to_owned(),
        language: Language {
          id: 1,
          name: "english".to_owned(),
        },
        quality: QualityWrapper {
          quality: Quality {
            name: "HD - 1080p".to_owned(),
          },
        },
        date: DateTime::from(DateTime::parse_from_rfc3339("2024-03-10T07:28:45Z").unwrap()),
        ..SonarrHistoryItem::default()
      },
    ]
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
