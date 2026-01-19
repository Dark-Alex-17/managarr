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
  use crate::handlers::lidarr_handlers::blocklist::{BlocklistHandler, blocklist_sorting_options};
  use crate::models::lidarr_models::{Artist, BlocklistItem};
  use crate::models::servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, BLOCKLIST_BLOCKS};
  use crate::models::servarr_models::{Quality, QualityWrapper};
  use crate::network::lidarr_network::lidarr_network_test_utils::test_utils::artist;

  mod test_handle_delete {
    use pretty_assertions::assert_eq;

    use super::*;

    const DELETE_KEY: Key = DEFAULT_KEYBINDINGS.delete.key;

    #[test]
    fn test_delete_blocklist_item_prompt() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Blocklist.into());
      app.data.lidarr_data.blocklist.set_items(blocklist_vec());

      BlocklistHandler::new(DELETE_KEY, &mut app, ActiveLidarrBlock::Blocklist, None).handle();

      assert_navigation_pushed!(app, ActiveLidarrBlock::DeleteBlocklistItemPrompt.into());
    }

    #[test]
    fn test_delete_blocklist_item_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveLidarrBlock::Blocklist.into());
      app.data.lidarr_data.blocklist.set_items(blocklist_vec());

      BlocklistHandler::new(DELETE_KEY, &mut app, ActiveLidarrBlock::Blocklist, None).handle();

      assert_eq!(app.get_current_route(), ActiveLidarrBlock::Blocklist.into());
    }
  }

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;
    use crate::assert_navigation_pushed;

    #[rstest]
    fn test_blocklist_tab_left(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Blocklist.into());
      app.is_loading = is_ready;
      app.data.lidarr_data.main_tabs.set_index(2);

      BlocklistHandler::new(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveLidarrBlock::Blocklist,
        None,
      )
      .handle();

      assert_eq!(
        app.data.lidarr_data.main_tabs.get_active_route(),
        ActiveLidarrBlock::Downloads.into()
      );
      assert_navigation_pushed!(app, ActiveLidarrBlock::Downloads.into());
    }

    #[rstest]
    fn test_blocklist_tab_right(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Blocklist.into());
      app.is_loading = is_ready;
      app.data.lidarr_data.main_tabs.set_index(2);

      BlocklistHandler::new(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveLidarrBlock::Blocklist,
        None,
      )
      .handle();

      assert_eq!(
        app.data.lidarr_data.main_tabs.get_active_route(),
        ActiveLidarrBlock::History.into()
      );
      assert_navigation_pushed!(app, ActiveLidarrBlock::History.into());
    }

    #[rstest]
    fn test_blocklist_left_right_prompt_toggle(
      #[values(
        ActiveLidarrBlock::DeleteBlocklistItemPrompt,
        ActiveLidarrBlock::BlocklistClearAllItemsPrompt
      )]
      active_lidarr_block: ActiveLidarrBlock,
      #[values(DEFAULT_KEYBINDINGS.left.key, DEFAULT_KEYBINDINGS.right.key)] key: Key,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Blocklist.into());

      BlocklistHandler::new(key, &mut app, active_lidarr_block, None).handle();

      assert!(app.data.lidarr_data.prompt_confirm);

      BlocklistHandler::new(key, &mut app, active_lidarr_block, None).handle();

      assert!(!app.data.lidarr_data.prompt_confirm);
    }
  }

  mod test_handle_submit {
    use crate::assert_navigation_popped;
    use crate::network::lidarr_network::LidarrEvent;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_blocklist_submit() {
      let mut app = App::test_default();
      app.data.lidarr_data.blocklist.set_items(blocklist_vec());
      app.push_navigation_stack(ActiveLidarrBlock::Blocklist.into());

      BlocklistHandler::new(SUBMIT_KEY, &mut app, ActiveLidarrBlock::Blocklist, None).handle();

      assert_navigation_pushed!(app, ActiveLidarrBlock::BlocklistItemDetails.into());
    }

    #[test]
    fn test_blocklist_submit_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.data.lidarr_data.blocklist.set_items(blocklist_vec());
      app.push_navigation_stack(ActiveLidarrBlock::Blocklist.into());

      BlocklistHandler::new(SUBMIT_KEY, &mut app, ActiveLidarrBlock::Blocklist, None).handle();

      assert_eq!(app.get_current_route(), ActiveLidarrBlock::Blocklist.into());
    }

    #[rstest]
    #[case(
      ActiveLidarrBlock::Blocklist,
      ActiveLidarrBlock::DeleteBlocklistItemPrompt,
      LidarrEvent::DeleteBlocklistItem(3)
    )]
    #[case(
      ActiveLidarrBlock::Blocklist,
      ActiveLidarrBlock::BlocklistClearAllItemsPrompt,
      LidarrEvent::ClearBlocklist
    )]
    fn test_blocklist_prompt_confirm_submit(
      #[case] base_route: ActiveLidarrBlock,
      #[case] prompt_block: ActiveLidarrBlock,
      #[case] expected_action: LidarrEvent,
    ) {
      let mut app = App::test_default();
      app.data.lidarr_data.blocklist.set_items(blocklist_vec());
      app.data.lidarr_data.prompt_confirm = true;
      app.push_navigation_stack(base_route.into());
      app.push_navigation_stack(prompt_block.into());

      BlocklistHandler::new(SUBMIT_KEY, &mut app, prompt_block, None).handle();

      assert!(app.data.lidarr_data.prompt_confirm);
      assert_some_eq_x!(
        &app.data.lidarr_data.prompt_confirm_action,
        &expected_action
      );
      assert_navigation_popped!(app, base_route.into());
    }

    #[rstest]
    fn test_blocklist_prompt_decline_submit(
      #[values(
        ActiveLidarrBlock::DeleteBlocklistItemPrompt,
        ActiveLidarrBlock::BlocklistClearAllItemsPrompt
      )]
      prompt_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default();
      app.data.lidarr_data.blocklist.set_items(blocklist_vec());
      app.push_navigation_stack(ActiveLidarrBlock::Blocklist.into());
      app.push_navigation_stack(prompt_block.into());

      BlocklistHandler::new(SUBMIT_KEY, &mut app, prompt_block, None).handle();

      assert!(!app.data.lidarr_data.prompt_confirm);
      assert_none!(app.data.lidarr_data.prompt_confirm_action);
      assert_navigation_popped!(app, ActiveLidarrBlock::Blocklist.into());
    }
  }

  mod test_handle_esc {
    use rstest::rstest;

    use super::*;
    use crate::assert_navigation_popped;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    #[case(
      ActiveLidarrBlock::Blocklist,
      ActiveLidarrBlock::DeleteBlocklistItemPrompt
    )]
    #[case(
      ActiveLidarrBlock::Blocklist,
      ActiveLidarrBlock::BlocklistClearAllItemsPrompt
    )]
    fn test_blocklist_prompt_blocks_esc(
      #[case] base_block: ActiveLidarrBlock,
      #[case] prompt_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default();
      app.push_navigation_stack(base_block.into());
      app.push_navigation_stack(prompt_block.into());
      app.data.lidarr_data.prompt_confirm = true;

      BlocklistHandler::new(ESC_KEY, &mut app, prompt_block, None).handle();

      assert_navigation_popped!(app, base_block.into());
      assert!(!app.data.lidarr_data.prompt_confirm);
    }

    #[test]
    fn test_esc_blocklist_item_details() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Blocklist.into());
      app.push_navigation_stack(ActiveLidarrBlock::BlocklistItemDetails.into());

      BlocklistHandler::new(
        ESC_KEY,
        &mut app,
        ActiveLidarrBlock::BlocklistItemDetails,
        None,
      )
      .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::Blocklist.into());
    }

    #[rstest]
    fn test_default_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.error = "test error".to_owned().into();
      app.push_navigation_stack(ActiveLidarrBlock::Blocklist.into());
      app.push_navigation_stack(ActiveLidarrBlock::Blocklist.into());

      BlocklistHandler::new(ESC_KEY, &mut app, ActiveLidarrBlock::Blocklist, None).handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::Blocklist.into());
      assert_is_empty!(app.error.text);
    }
  }

  mod test_handle_key_char {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::network::lidarr_network::LidarrEvent;

    use super::*;
    use crate::{assert_navigation_popped, assert_navigation_pushed};

    #[test]
    fn test_refresh_blocklist_key() {
      let mut app = App::test_default();
      app.data.lidarr_data.blocklist.set_items(blocklist_vec());
      app.push_navigation_stack(ActiveLidarrBlock::Blocklist.into());

      BlocklistHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveLidarrBlock::Blocklist,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveLidarrBlock::Blocklist.into());
      assert!(app.should_refresh);
    }

    #[test]
    fn test_refresh_blocklist_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.data.lidarr_data.blocklist.set_items(blocklist_vec());
      app.push_navigation_stack(ActiveLidarrBlock::Blocklist.into());

      BlocklistHandler::new(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveLidarrBlock::Blocklist,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveLidarrBlock::Blocklist.into());
      assert!(!app.should_refresh);
    }

    #[test]
    fn test_clear_blocklist_key() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Blocklist.into());
      app.data.lidarr_data.blocklist.set_items(blocklist_vec());

      BlocklistHandler::new(
        DEFAULT_KEYBINDINGS.clear.key,
        &mut app,
        ActiveLidarrBlock::Blocklist,
        None,
      )
      .handle();

      assert_navigation_pushed!(app, ActiveLidarrBlock::BlocklistClearAllItemsPrompt.into());
    }

    #[test]
    fn test_clear_blocklist_key_no_op_when_not_ready() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveLidarrBlock::Blocklist.into());
      app.data.lidarr_data.blocklist.set_items(blocklist_vec());

      BlocklistHandler::new(
        DEFAULT_KEYBINDINGS.clear.key,
        &mut app,
        ActiveLidarrBlock::Blocklist,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveLidarrBlock::Blocklist.into());
    }

    #[rstest]
    #[case(
      ActiveLidarrBlock::Blocklist,
      ActiveLidarrBlock::DeleteBlocklistItemPrompt,
      LidarrEvent::DeleteBlocklistItem(3)
    )]
    #[case(
      ActiveLidarrBlock::Blocklist,
      ActiveLidarrBlock::BlocklistClearAllItemsPrompt,
      LidarrEvent::ClearBlocklist
    )]
    fn test_blocklist_prompt_confirm(
      #[case] base_route: ActiveLidarrBlock,
      #[case] prompt_block: ActiveLidarrBlock,
      #[case] expected_action: LidarrEvent,
    ) {
      let mut app = App::test_default();
      app.data.lidarr_data.blocklist.set_items(blocklist_vec());
      app.push_navigation_stack(base_route.into());
      app.push_navigation_stack(prompt_block.into());

      BlocklistHandler::new(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        prompt_block,
        None,
      )
      .handle();

      assert!(app.data.lidarr_data.prompt_confirm);
      assert_some_eq_x!(
        &app.data.lidarr_data.prompt_confirm_action,
        &expected_action
      );
      assert_navigation_popped!(app, base_route.into());
    }
  }

  #[test]
  fn test_blocklist_sorting_options_artist_name() {
    let expected_cmp_fn: fn(&BlocklistItem, &BlocklistItem) -> Ordering = |a, b| {
      a.artist
        .artist_name
        .text
        .to_lowercase()
        .cmp(&b.artist.artist_name.text.to_lowercase())
    };
    let mut expected_blocklist_vec = blocklist_vec();
    expected_blocklist_vec.sort_by(expected_cmp_fn);

    let sort_option = blocklist_sorting_options()[0].clone();
    let mut sorted_blocklist_vec = blocklist_vec();
    sorted_blocklist_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_blocklist_vec, expected_blocklist_vec);
    assert_str_eq!(sort_option.name, "Artist Name");
  }

  #[test]
  fn test_blocklist_sorting_options_source_title() {
    let expected_cmp_fn: fn(&BlocklistItem, &BlocklistItem) -> Ordering = |a, b| {
      a.source_title
        .to_lowercase()
        .cmp(&b.source_title.to_lowercase())
    };
    let mut expected_blocklist_vec = blocklist_vec();
    expected_blocklist_vec.sort_by(expected_cmp_fn);

    let sort_option = blocklist_sorting_options()[1].clone();
    let mut sorted_blocklist_vec = blocklist_vec();
    sorted_blocklist_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_blocklist_vec, expected_blocklist_vec);
    assert_str_eq!(sort_option.name, "Source Title");
  }

  #[test]
  fn test_blocklist_sorting_options_quality() {
    let expected_cmp_fn: fn(&BlocklistItem, &BlocklistItem) -> Ordering = |a, b| {
      a.quality
        .quality
        .name
        .to_lowercase()
        .cmp(&b.quality.quality.name.to_lowercase())
    };
    let mut expected_blocklist_vec = blocklist_vec();
    expected_blocklist_vec.sort_by(expected_cmp_fn);

    let sort_option = blocklist_sorting_options()[2].clone();
    let mut sorted_blocklist_vec = blocklist_vec();
    sorted_blocklist_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_blocklist_vec, expected_blocklist_vec);
    assert_str_eq!(sort_option.name, "Quality");
  }

  #[test]
  fn test_blocklist_sorting_options_date() {
    let expected_cmp_fn: fn(&BlocklistItem, &BlocklistItem) -> Ordering =
      |a, b| a.date.cmp(&b.date);
    let mut expected_blocklist_vec = blocklist_vec();
    expected_blocklist_vec.sort_by(expected_cmp_fn);

    let sort_option = blocklist_sorting_options()[3].clone();
    let mut sorted_blocklist_vec = blocklist_vec();
    sorted_blocklist_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_blocklist_vec, expected_blocklist_vec);
    assert_str_eq!(sort_option.name, "Date");
  }

  #[test]
  fn test_blocklist_handler_accepts() {
    ActiveLidarrBlock::iter().for_each(|active_lidarr_block| {
      if BLOCKLIST_BLOCKS.contains(&active_lidarr_block) {
        assert!(BlocklistHandler::accepts(active_lidarr_block));
      } else {
        assert!(!BlocklistHandler::accepts(active_lidarr_block));
      }
    })
  }

  #[rstest]
  fn test_blocklist_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = BlocklistHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::default(),
      None,
    );

    assert_eq!(
      handler.ignore_special_keys(),
      ignore_special_keys_for_textbox_input
    );
  }

  #[test]
  fn test_extract_blocklist_item_id() {
    let mut app = App::test_default();
    app.data.lidarr_data.blocklist.set_items(blocklist_vec());

    let blocklist_item_id = BlocklistHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::Blocklist,
      None,
    )
    .extract_blocklist_item_id();

    assert_eq!(blocklist_item_id, 3);
  }

  #[test]
  fn test_blocklist_handler_not_ready_when_loading() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::Blocklist.into());
    app.is_loading = true;

    let handler = BlocklistHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::Blocklist,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_blocklist_handler_not_ready_when_blocklist_is_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::Blocklist.into());
    app.is_loading = false;

    let handler = BlocklistHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::Blocklist,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_blocklist_handler_ready_when_not_loading_and_blocklist_is_not_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::Blocklist.into());
    app.is_loading = false;
    app
      .data
      .lidarr_data
      .blocklist
      .set_items(vec![BlocklistItem::default()]);

    let handler = BlocklistHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::Blocklist,
      None,
    );

    assert!(handler.is_ready());
  }

  fn blocklist_vec() -> Vec<BlocklistItem> {
    vec![
      BlocklistItem {
        id: 3,
        source_title: "test 1".to_owned(),
        quality: QualityWrapper {
          quality: Quality {
            name: "Lossless".to_owned(),
          },
        },
        date: DateTime::from(DateTime::parse_from_rfc3339("2024-01-10T07:28:45Z").unwrap()),
        artist: Artist {
          artist_name: "test 3".into(),
          ..artist()
        },
        ..BlocklistItem::default()
      },
      BlocklistItem {
        id: 2,
        source_title: "test 2".to_owned(),
        quality: QualityWrapper {
          quality: Quality {
            name: "Lossy".to_owned(),
          },
        },
        date: DateTime::from(DateTime::parse_from_rfc3339("2024-02-10T07:28:45Z").unwrap()),
        artist: Artist {
          artist_name: "test 2".into(),
          ..artist()
        },
        ..BlocklistItem::default()
      },
      BlocklistItem {
        id: 1,
        source_title: "test 3".to_owned(),
        quality: QualityWrapper {
          quality: Quality {
            name: "Lossless".to_owned(),
          },
        },
        date: DateTime::from(DateTime::parse_from_rfc3339("2024-03-10T07:28:45Z").unwrap()),
        artist: Artist {
          artist_name: "".into(),
          ..artist()
        },
        ..BlocklistItem::default()
      },
    ]
  }
}
