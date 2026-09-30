#[cfg(test)]
mod tests {
  use std::cmp::Ordering;

  use chrono::DateTime;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::assert_navigation_pushed;
  use crate::event::Key;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::readarr_handlers::history::{HistoryHandler, history_sorting_options};
  use crate::models::readarr_models::{ReadarrHistoryEventType, ReadarrHistoryItem};
  use crate::models::servarr_data::readarr::readarr_data::{ActiveReadarrBlock, HISTORY_BLOCKS};
  use crate::models::servarr_models::{Quality, QualityWrapper};

  #[test]
  fn test_history_sorting_options_source_title() {
    let expected_cmp_fn: fn(&ReadarrHistoryItem, &ReadarrHistoryItem) -> Ordering = |a, b| {
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
    let expected_cmp_fn: fn(&ReadarrHistoryItem, &ReadarrHistoryItem) -> Ordering = |a, b| {
      a.event_type
        .to_string()
        .to_lowercase()
        .cmp(&b.event_type.to_string().to_lowercase())
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
  fn test_history_sorting_options_quality() {
    let expected_cmp_fn: fn(&ReadarrHistoryItem, &ReadarrHistoryItem) -> Ordering = |a, b| {
      a.quality
        .quality
        .name
        .to_lowercase()
        .cmp(&b.quality.quality.name.to_lowercase())
    };
    let mut expected_history_vec = history_vec();
    expected_history_vec.sort_by(expected_cmp_fn);

    let sort_option = history_sorting_options()[2].clone();
    let mut sorted_history_vec = history_vec();
    sorted_history_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_history_vec, expected_history_vec);
    assert_str_eq!(sort_option.name, "Quality");
  }

  #[test]
  fn test_history_sorting_options_date() {
    let expected_cmp_fn: fn(&ReadarrHistoryItem, &ReadarrHistoryItem) -> Ordering =
      |a, b| a.date.cmp(&b.date);
    let mut expected_history_vec = history_vec();
    expected_history_vec.sort_by(expected_cmp_fn);

    let sort_option = history_sorting_options()[3].clone();
    let mut sorted_history_vec = history_vec();
    sorted_history_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_history_vec, expected_history_vec);
    assert_str_eq!(sort_option.name, "Date");
  }

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;
    use crate::assert_navigation_pushed;

    #[rstest]
    fn test_history_tab_left(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::History.into());
      app.is_loading = is_ready;
      app.data.readarr_data.main_tabs.set_index(3);

      HistoryHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveReadarrBlock::History,
        None,
      )
      .handle();

      assert_eq!(
        app.data.readarr_data.main_tabs.get_active_route(),
        ActiveReadarrBlock::Blocklist.into()
      );
      assert_navigation_pushed!(app, ActiveReadarrBlock::Blocklist.into());
    }

    #[rstest]
    fn test_history_tab_right(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::History.into());
      app.is_loading = is_ready;
      app.data.readarr_data.main_tabs.set_index(3);

      HistoryHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveReadarrBlock::History,
        None,
      )
      .handle();

      assert_eq!(
        app.data.readarr_data.main_tabs.get_active_route(),
        ActiveReadarrBlock::RootFolders.into()
      );
      assert_navigation_pushed!(app, ActiveReadarrBlock::RootFolders.into());
    }
  }

  mod test_handle_submit {
    use pretty_assertions::assert_eq;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_history_submit() {
      let mut app = App::test_default();
      app.data.readarr_data.history.set_items(history_vec());
      app.push_navigation_stack(ActiveReadarrBlock::History.into());

      HistoryHandler::new(SUBMIT_KEY, &mut app, ActiveReadarrBlock::History, None).handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::HistoryItemDetails.into());
    }

    #[test]
    fn test_history_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.data.readarr_data.history.set_items(history_vec());
      app.push_navigation_stack(ActiveReadarrBlock::History.into());

      HistoryHandler::new(SUBMIT_KEY, &mut app, ActiveReadarrBlock::History, None).handle();

      assert_eq!(app.get_current_route(), ActiveReadarrBlock::History.into());
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;
    use crate::assert_navigation_popped;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[test]
    fn test_esc_history_item_details() {
      let mut app = App::test_default();
      app
        .data
        .readarr_data
        .history
        .set_items(vec![ReadarrHistoryItem::default()]);
      app.push_navigation_stack(ActiveReadarrBlock::History.into());
      app.push_navigation_stack(ActiveReadarrBlock::HistoryItemDetails.into());

      HistoryHandler::new(
        ESC_KEY,
        &mut app,
        ActiveReadarrBlock::HistoryItemDetails,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::History.into());
    }

    #[rstest]
    fn test_default_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.error = "test error".to_owned().into();
      app.push_navigation_stack(ActiveReadarrBlock::History.into());
      app.push_navigation_stack(ActiveReadarrBlock::History.into());
      app
        .data
        .readarr_data
        .history
        .set_items(vec![ReadarrHistoryItem::default()]);

      HistoryHandler::new(ESC_KEY, &mut app, ActiveReadarrBlock::History, None).handle();

      assert_eq!(app.get_current_route(), ActiveReadarrBlock::History.into());
      assert_is_empty!(app.error.text);
    }
  }

  mod test_handle_key_char {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::assert_navigation_pushed;

    #[test]
    fn test_refresh_history_key() {
      let mut app = App::test_default();
      app.data.readarr_data.history.set_items(history_vec());
      app.push_navigation_stack(ActiveReadarrBlock::History.into());

      HistoryHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveReadarrBlock::History,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::History.into());
      assert!(app.should_refresh);
    }

    #[test]
    fn test_refresh_history_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.data.readarr_data.history.set_items(history_vec());
      app.push_navigation_stack(ActiveReadarrBlock::History.into());

      HistoryHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveReadarrBlock::History,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveReadarrBlock::History.into());
      assert!(!app.should_refresh);
    }

    #[test]
    fn test_search_history_key() {
      let mut app = App::test_default();
      app.data.readarr_data.history.set_items(history_vec());
      app.push_navigation_stack(ActiveReadarrBlock::History.into());

      HistoryHandler::new(
        DEFAULT_KEYBINDINGS.search.key,
        &mut app,
        ActiveReadarrBlock::History,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::SearchHistory.into());
    }

    #[test]
    fn test_filter_history_key() {
      let mut app = App::test_default();
      app.data.readarr_data.history.set_items(history_vec());
      app.push_navigation_stack(ActiveReadarrBlock::History.into());

      HistoryHandler::new(
        DEFAULT_KEYBINDINGS.filter.key,
        &mut app,
        ActiveReadarrBlock::History,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveReadarrBlock::FilterHistory.into());
    }
  }

  #[test]
  fn test_history_handler_accepts() {
    ActiveReadarrBlock::iter().for_each(|active_readarr_block| {
      if HISTORY_BLOCKS.contains(&active_readarr_block) {
        assert!(HistoryHandler::accepts(active_readarr_block));
      } else {
        assert!(!HistoryHandler::accepts(active_readarr_block));
      }
    })
  }

  #[rstest]
  fn test_history_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = HistoryHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::default(),
      None,
    );

    assert_eq!(
      handler.ignore_special_keys(),
      ignore_special_keys_for_textbox_input
    );
  }

  #[test]
  fn test_history_handler_not_ready_when_loading() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::History.into());
    app.is_loading = true;

    let handler = HistoryHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::History,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_history_handler_not_ready_when_history_is_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::History.into());
    app.is_loading = false;

    let handler = HistoryHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::History,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_history_handler_ready_when_not_loading_and_history_is_not_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::History.into());
    app.is_loading = false;
    app
      .data
      .readarr_data
      .history
      .set_items(vec![ReadarrHistoryItem::default()]);

    let handler = HistoryHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::History,
      None,
    );

    assert!(handler.is_ready());
  }

  fn history_vec() -> Vec<ReadarrHistoryItem> {
    vec![
      ReadarrHistoryItem {
        id: 3,
        source_title: "test 1".into(),
        event_type: ReadarrHistoryEventType::Grabbed,
        quality: QualityWrapper {
          quality: Quality {
            name: "EPUB".to_owned(),
          },
        },
        date: DateTime::from(DateTime::parse_from_rfc3339("2024-01-10T07:28:45Z").unwrap()),
        ..ReadarrHistoryItem::default()
      },
      ReadarrHistoryItem {
        id: 2,
        source_title: "test 2".into(),
        event_type: ReadarrHistoryEventType::DownloadImported,
        quality: QualityWrapper {
          quality: Quality {
            name: "AZW3".to_owned(),
          },
        },
        date: DateTime::from(DateTime::parse_from_rfc3339("2024-02-10T07:28:45Z").unwrap()),
        ..ReadarrHistoryItem::default()
      },
      ReadarrHistoryItem {
        id: 1,
        source_title: "test 3".into(),
        event_type: ReadarrHistoryEventType::BookFileDeleted,
        quality: QualityWrapper {
          quality: Quality {
            name: "MOBI".to_owned(),
          },
        },
        date: DateTime::from(DateTime::parse_from_rfc3339("2024-03-10T07:28:45Z").unwrap()),
        ..ReadarrHistoryItem::default()
      },
    ]
  }
}
