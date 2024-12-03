#[cfg(test)]
mod tests {
  use std::cmp::Ordering;

  use chrono::DateTime;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use strum::IntoEnumIterator;

  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::sonarr_handlers::blocklist::{blocklist_sorting_options, BlocklistHandler};
  use crate::handlers::KeyEventHandler;
  use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, BLOCKLIST_BLOCKS};
  use crate::models::servarr_models::{Language, Quality, QualityWrapper};
  use crate::models::sonarr_models::BlocklistItem;
  use crate::models::stateful_table::SortOption;

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::{assert_eq, assert_str_eq};
    use rstest::rstest;

    use crate::models::sonarr_models::BlocklistItem;
    use crate::{simple_stateful_iterable_vec, test_iterable_scroll};

    use super::*;

    test_iterable_scroll!(
      test_blocklist_scroll,
      BlocklistHandler,
      sonarr_data,
      blocklist,
      simple_stateful_iterable_vec!(BlocklistItem, String, source_title),
      ActiveSonarrBlock::Blocklist,
      None,
      source_title,
      to_string
    );

    #[rstest]
    fn test_blocklist_scroll_no_op_when_not_ready(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Blocklist.into());
      app.is_loading = true;
      app
        .data
        .sonarr_data
        .blocklist
        .set_items(simple_stateful_iterable_vec!(
          BlocklistItem,
          String,
          source_title
        ));

      BlocklistHandler::with(key, &mut app, ActiveSonarrBlock::Blocklist, None).handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .blocklist
          .current_selection()
          .source_title
          .to_string(),
        "Test 1"
      );

      BlocklistHandler::with(key, &mut app, ActiveSonarrBlock::Blocklist, None).handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .blocklist
          .current_selection()
          .source_title
          .to_string(),
        "Test 1"
      );
    }

    #[rstest]
    fn test_blocklist_sort_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let blocklist_field_vec = sort_options();
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Blocklist.into());
      app.data.sonarr_data.blocklist.sorting(sort_options());

      if key == Key::Up {
        for i in (0..blocklist_field_vec.len()).rev() {
          BlocklistHandler::with(key, &mut app, ActiveSonarrBlock::BlocklistSortPrompt, None)
            .handle();

          assert_eq!(
            app
              .data
              .sonarr_data
              .blocklist
              .sort
              .as_ref()
              .unwrap()
              .current_selection(),
            &blocklist_field_vec[i]
          );
        }
      } else {
        for i in 0..blocklist_field_vec.len() {
          BlocklistHandler::with(key, &mut app, ActiveSonarrBlock::BlocklistSortPrompt, None)
            .handle();

          assert_eq!(
            app
              .data
              .sonarr_data
              .blocklist
              .sort
              .as_ref()
              .unwrap()
              .current_selection(),
            &blocklist_field_vec[(i + 1) % blocklist_field_vec.len()]
          );
        }
      }
    }
  }

  mod test_handle_home_end {
    use pretty_assertions::{assert_eq, assert_str_eq};

    use crate::models::sonarr_models::BlocklistItem;
    use crate::{extended_stateful_iterable_vec, test_iterable_home_and_end};

    use super::*;

    test_iterable_home_and_end!(
      test_blocklist_home_and_end,
      BlocklistHandler,
      sonarr_data,
      blocklist,
      extended_stateful_iterable_vec!(BlocklistItem, String, source_title),
      ActiveSonarrBlock::Blocklist,
      None,
      source_title,
      to_string
    );

    #[test]
    fn test_blocklist_home_and_end_no_op_when_not_ready() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Blocklist.into());
      app.is_loading = true;
      app
        .data
        .sonarr_data
        .blocklist
        .set_items(extended_stateful_iterable_vec!(
          BlocklistItem,
          String,
          source_title
        ));

      BlocklistHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::Blocklist,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .blocklist
          .current_selection()
          .source_title
          .to_string(),
        "Test 1"
      );

      BlocklistHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::Blocklist,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .blocklist
          .current_selection()
          .source_title
          .to_string(),
        "Test 1"
      );
    }

    #[test]
    fn test_blocklist_sort_home_end() {
      let blocklist_field_vec = sort_options();
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Blocklist.into());
      app.data.sonarr_data.blocklist.sorting(sort_options());

      BlocklistHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::BlocklistSortPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .blocklist
          .sort
          .as_ref()
          .unwrap()
          .current_selection(),
        &blocklist_field_vec[blocklist_field_vec.len() - 1]
      );

      BlocklistHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::BlocklistSortPrompt,
        None,
      )
      .handle();

      assert_eq!(
        app
          .data
          .sonarr_data
          .blocklist
          .sort
          .as_ref()
          .unwrap()
          .current_selection(),
        &blocklist_field_vec[0]
      );
    }
  }

  mod test_handle_delete {
    use pretty_assertions::assert_eq;

    use super::*;

    const DELETE_KEY: Key = DEFAULT_KEYBINDINGS.delete.key;

    #[test]
    fn test_delete_blocklist_item_prompt() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Blocklist.into());
      app.data.sonarr_data.blocklist.set_items(blocklist_vec());

      BlocklistHandler::with(DELETE_KEY, &mut app, ActiveSonarrBlock::Blocklist, None).handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::DeleteBlocklistItemPrompt.into()
      );
    }

    #[test]
    fn test_delete_blocklist_item_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Blocklist.into());
      app.data.sonarr_data.blocklist.set_items(blocklist_vec());

      BlocklistHandler::with(DELETE_KEY, &mut app, ActiveSonarrBlock::Blocklist, None).handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Blocklist.into());
    }
  }

  mod test_handle_left_right_action {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_blocklist_tab_left(#[values(true, false)] is_ready: bool) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Blocklist.into());
      app.is_loading = is_ready;
      app.data.sonarr_data.main_tabs.set_index(2);

      BlocklistHandler::with(
        DEFAULT_KEYBINDINGS.left.key,
        &mut app,
        ActiveSonarrBlock::Blocklist,
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
    fn test_blocklist_tab_right(#[values(true, false)] is_ready: bool) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Blocklist.into());
      app.is_loading = is_ready;
      app.data.sonarr_data.main_tabs.set_index(2);

      BlocklistHandler::with(
        DEFAULT_KEYBINDINGS.right.key,
        &mut app,
        ActiveSonarrBlock::Blocklist,
        None,
      )
      .handle();

      assert_eq!(
        app.data.sonarr_data.main_tabs.get_active_route(),
        ActiveSonarrBlock::History.into()
      );
      assert_eq!(app.get_current_route(), ActiveSonarrBlock::History.into());
    }

    #[rstest]
    fn test_blocklist_left_right_prompt_toggle(
      #[values(
        ActiveSonarrBlock::DeleteBlocklistItemPrompt,
        ActiveSonarrBlock::BlocklistClearAllItemsPrompt
      )]
      active_sonarr_block: ActiveSonarrBlock,
      #[values(DEFAULT_KEYBINDINGS.left.key, DEFAULT_KEYBINDINGS.right.key)] key: Key,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Blocklist.into());

      BlocklistHandler::with(key, &mut app, active_sonarr_block, None).handle();

      assert!(app.data.sonarr_data.prompt_confirm);

      BlocklistHandler::with(key, &mut app, active_sonarr_block, None).handle();

      assert!(!app.data.sonarr_data.prompt_confirm);
    }
  }

  mod test_handle_submit {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::network::sonarr_network::SonarrEvent;

    use super::*;

    const SUBMIT_KEY: Key = DEFAULT_KEYBINDINGS.submit.key;

    #[test]
    fn test_blocklist_submit() {
      let mut app = App::default();
      app.data.sonarr_data.blocklist.set_items(blocklist_vec());
      app.push_navigation_stack(ActiveSonarrBlock::Blocklist.into());

      BlocklistHandler::with(SUBMIT_KEY, &mut app, ActiveSonarrBlock::Blocklist, None).handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::BlocklistItemDetails.into()
      );
    }

    #[test]
    fn test_blocklist_submit_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.data.sonarr_data.blocklist.set_items(blocklist_vec());
      app.push_navigation_stack(ActiveSonarrBlock::Blocklist.into());

      BlocklistHandler::with(SUBMIT_KEY, &mut app, ActiveSonarrBlock::Blocklist, None).handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Blocklist.into());
    }

    #[rstest]
    #[case(
      ActiveSonarrBlock::Blocklist,
      ActiveSonarrBlock::DeleteBlocklistItemPrompt,
      SonarrEvent::DeleteBlocklistItem(None)
    )]
    #[case(
      ActiveSonarrBlock::Blocklist,
      ActiveSonarrBlock::BlocklistClearAllItemsPrompt,
      SonarrEvent::ClearBlocklist
    )]
    fn test_blocklist_prompt_confirm_submit(
      #[case] base_route: ActiveSonarrBlock,
      #[case] prompt_block: ActiveSonarrBlock,
      #[case] expected_action: SonarrEvent,
    ) {
      let mut app = App::default();
      app.data.sonarr_data.blocklist.set_items(blocklist_vec());
      app.data.sonarr_data.prompt_confirm = true;
      app.push_navigation_stack(base_route.into());
      app.push_navigation_stack(prompt_block.into());

      BlocklistHandler::with(SUBMIT_KEY, &mut app, prompt_block, None).handle();

      assert!(app.data.sonarr_data.prompt_confirm);
      assert_eq!(
        app.data.sonarr_data.prompt_confirm_action,
        Some(expected_action)
      );
      assert_eq!(app.get_current_route(), base_route.into());
    }

    #[rstest]
    fn test_blocklist_prompt_decline_submit(
      #[values(
        ActiveSonarrBlock::DeleteBlocklistItemPrompt,
        ActiveSonarrBlock::BlocklistClearAllItemsPrompt
      )]
      prompt_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app.data.sonarr_data.blocklist.set_items(blocklist_vec());
      app.push_navigation_stack(ActiveSonarrBlock::Blocklist.into());
      app.push_navigation_stack(prompt_block.into());

      BlocklistHandler::with(SUBMIT_KEY, &mut app, prompt_block, None).handle();

      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.data.sonarr_data.prompt_confirm_action, None);
      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Blocklist.into());
    }

    #[test]
    fn test_blocklist_sort_prompt_submit() {
      let mut app = App::default();
      app.data.sonarr_data.blocklist.sort_asc = true;
      app.data.sonarr_data.blocklist.sorting(sort_options());
      app.data.sonarr_data.blocklist.set_items(blocklist_vec());
      app.push_navigation_stack(ActiveSonarrBlock::Blocklist.into());
      app.push_navigation_stack(ActiveSonarrBlock::BlocklistSortPrompt.into());

      let mut expected_vec = blocklist_vec();
      expected_vec.sort_by(|a, b| a.id.cmp(&b.id));
      expected_vec.reverse();

      BlocklistHandler::with(
        SUBMIT_KEY,
        &mut app,
        ActiveSonarrBlock::BlocklistSortPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Blocklist.into());
      assert_eq!(app.data.sonarr_data.blocklist.items, expected_vec);
    }
  }

  mod test_handle_esc {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;

    const ESC_KEY: Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    #[case(
      ActiveSonarrBlock::Blocklist,
      ActiveSonarrBlock::DeleteBlocklistItemPrompt
    )]
    #[case(
      ActiveSonarrBlock::Blocklist,
      ActiveSonarrBlock::BlocklistClearAllItemsPrompt
    )]
    fn test_blocklist_prompt_blocks_esc(
      #[case] base_block: ActiveSonarrBlock,
      #[case] prompt_block: ActiveSonarrBlock,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(base_block.into());
      app.push_navigation_stack(prompt_block.into());
      app.data.sonarr_data.prompt_confirm = true;

      BlocklistHandler::with(ESC_KEY, &mut app, prompt_block, None).handle();

      assert_eq!(app.get_current_route(), base_block.into());
      assert!(!app.data.sonarr_data.prompt_confirm);
    }

    #[test]
    fn test_esc_blocklist_item_details() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Blocklist.into());
      app.push_navigation_stack(ActiveSonarrBlock::BlocklistItemDetails.into());

      BlocklistHandler::with(
        ESC_KEY,
        &mut app,
        ActiveSonarrBlock::BlocklistItemDetails,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Blocklist.into());
    }

    #[test]
    fn test_blocklist_sort_prompt_block_esc() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Blocklist.into());
      app.push_navigation_stack(ActiveSonarrBlock::BlocklistSortPrompt.into());

      BlocklistHandler::with(
        ESC_KEY,
        &mut app,
        ActiveSonarrBlock::BlocklistSortPrompt,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Blocklist.into());
    }

    #[rstest]
    fn test_default_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::default();
      app.is_loading = is_ready;
      app.error = "test error".to_owned().into();
      app.push_navigation_stack(ActiveSonarrBlock::Blocklist.into());
      app.push_navigation_stack(ActiveSonarrBlock::Blocklist.into());

      BlocklistHandler::with(ESC_KEY, &mut app, ActiveSonarrBlock::Blocklist, None).handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Blocklist.into());
      assert!(app.error.text.is_empty());
    }
  }

  mod test_handle_key_char {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::network::sonarr_network::SonarrEvent;

    use super::*;

    #[test]
    fn test_refresh_blocklist_key() {
      let mut app = App::default();
      app.data.sonarr_data.blocklist.set_items(blocklist_vec());
      app.push_navigation_stack(ActiveSonarrBlock::Blocklist.into());

      BlocklistHandler::with(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveSonarrBlock::Blocklist,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Blocklist.into());
      assert!(app.should_refresh);
    }

    #[test]
    fn test_refresh_blocklist_key_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.data.sonarr_data.blocklist.set_items(blocklist_vec());
      app.push_navigation_stack(ActiveSonarrBlock::Blocklist.into());

      BlocklistHandler::with(
        DEFAULT_KEYBINDINGS.refresh.key,
        &mut app,
        ActiveSonarrBlock::Blocklist,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Blocklist.into());
      assert!(!app.should_refresh);
    }

    #[test]
    fn test_clear_blocklist_key() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Blocklist.into());
      app.data.sonarr_data.blocklist.set_items(blocklist_vec());

      BlocklistHandler::with(
        DEFAULT_KEYBINDINGS.clear.key,
        &mut app,
        ActiveSonarrBlock::Blocklist,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::BlocklistClearAllItemsPrompt.into()
      );
    }

    #[test]
    fn test_clear_blocklist_key_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Blocklist.into());
      app.data.sonarr_data.blocklist.set_items(blocklist_vec());

      BlocklistHandler::with(
        DEFAULT_KEYBINDINGS.clear.key,
        &mut app,
        ActiveSonarrBlock::Blocklist,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Blocklist.into());
    }

    #[test]
    fn test_sort_key() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Blocklist.into());
      app.data.sonarr_data.blocklist.set_items(blocklist_vec());

      BlocklistHandler::with(
        DEFAULT_KEYBINDINGS.sort.key,
        &mut app,
        ActiveSonarrBlock::Blocklist,
        None,
      )
      .handle();

      assert_eq!(
        app.get_current_route(),
        ActiveSonarrBlock::BlocklistSortPrompt.into()
      );
      assert_eq!(
        app.data.sonarr_data.blocklist.sort.as_ref().unwrap().items,
        blocklist_sorting_options()
      );
      assert!(!app.data.sonarr_data.blocklist.sort_asc);
    }

    #[test]
    fn test_sort_key_no_op_when_not_ready() {
      let mut app = App::default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Blocklist.into());
      app.data.sonarr_data.blocklist.set_items(blocklist_vec());

      BlocklistHandler::with(
        DEFAULT_KEYBINDINGS.sort.key,
        &mut app,
        ActiveSonarrBlock::Blocklist,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Blocklist.into());
      assert!(app.data.sonarr_data.blocklist.sort.is_none());
      assert!(!app.data.sonarr_data.blocklist.sort_asc);
    }

    #[rstest]
    #[case(
      ActiveSonarrBlock::Blocklist,
      ActiveSonarrBlock::DeleteBlocklistItemPrompt,
      SonarrEvent::DeleteBlocklistItem(None)
    )]
    #[case(
      ActiveSonarrBlock::Blocklist,
      ActiveSonarrBlock::BlocklistClearAllItemsPrompt,
      SonarrEvent::ClearBlocklist
    )]
    fn test_blocklist_prompt_confirm(
      #[case] base_route: ActiveSonarrBlock,
      #[case] prompt_block: ActiveSonarrBlock,
      #[case] expected_action: SonarrEvent,
    ) {
      let mut app = App::default();
      app.data.sonarr_data.blocklist.set_items(blocklist_vec());
      app.push_navigation_stack(base_route.into());
      app.push_navigation_stack(prompt_block.into());

      BlocklistHandler::with(
        DEFAULT_KEYBINDINGS.confirm.key,
        &mut app,
        prompt_block,
        None,
      )
      .handle();

      assert!(app.data.sonarr_data.prompt_confirm);
      assert_eq!(
        app.data.sonarr_data.prompt_confirm_action,
        Some(expected_action)
      );
      assert_eq!(app.get_current_route(), base_route.into());
    }
  }

  #[test]
  fn test_blocklist_sorting_options_series_title() {
    let expected_cmp_fn: fn(&BlocklistItem, &BlocklistItem) -> Ordering = |a, b| {
      a.series_title
        .as_ref()
        .unwrap_or(&String::new())
        .to_lowercase()
        .cmp(
          &b.series_title
            .as_ref()
            .unwrap_or(&String::new())
            .to_lowercase(),
        )
    };
    let mut expected_blocklist_vec = blocklist_vec();
    expected_blocklist_vec.sort_by(expected_cmp_fn);

    let sort_option = blocklist_sorting_options()[0].clone();
    let mut sorted_blocklist_vec = blocklist_vec();
    sorted_blocklist_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_blocklist_vec, expected_blocklist_vec);
    assert_str_eq!(sort_option.name, "Series Title");
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
  fn test_blocklist_sorting_options_language() {
    let expected_cmp_fn: fn(&BlocklistItem, &BlocklistItem) -> Ordering = |a, b| {
      a.language
        .name
        .to_lowercase()
        .cmp(&b.language.name.to_lowercase())
    };
    let mut expected_blocklist_vec = blocklist_vec();
    expected_blocklist_vec.sort_by(expected_cmp_fn);

    let sort_option = blocklist_sorting_options()[2].clone();
    let mut sorted_blocklist_vec = blocklist_vec();
    sorted_blocklist_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_blocklist_vec, expected_blocklist_vec);
    assert_str_eq!(sort_option.name, "Language");
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

    let sort_option = blocklist_sorting_options()[3].clone();
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

    let sort_option = blocklist_sorting_options()[4].clone();
    let mut sorted_blocklist_vec = blocklist_vec();
    sorted_blocklist_vec.sort_by(sort_option.cmp_fn.unwrap());

    assert_eq!(sorted_blocklist_vec, expected_blocklist_vec);
    assert_str_eq!(sort_option.name, "Date");
  }

  #[test]
  fn test_blocklist_handler_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if BLOCKLIST_BLOCKS.contains(&active_sonarr_block) {
        assert!(BlocklistHandler::accepts(active_sonarr_block));
      } else {
        assert!(!BlocklistHandler::accepts(active_sonarr_block));
      }
    })
  }

  #[test]
  fn test_blocklist_handler_not_ready_when_loading() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::Blocklist.into());
    app.is_loading = true;

    let handler = BlocklistHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::Blocklist,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_blocklist_handler_not_ready_when_blocklist_is_empty() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::Blocklist.into());
    app.is_loading = false;

    let handler = BlocklistHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::Blocklist,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_blocklist_handler_ready_when_not_loading_and_blocklist_is_not_empty() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::Blocklist.into());
    app.is_loading = false;
    app
      .data
      .sonarr_data
      .blocklist
      .set_items(vec![BlocklistItem::default()]);

    let handler = BlocklistHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::Blocklist,
      None,
    );

    assert!(handler.is_ready());
  }

  fn blocklist_vec() -> Vec<BlocklistItem> {
    vec![
      BlocklistItem {
        id: 3,
        source_title: "test 1".to_owned(),
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
        series_title: Some("test 3".into()),
        ..BlocklistItem::default()
      },
      BlocklistItem {
        id: 2,
        source_title: "test 2".to_owned(),
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
        series_title: Some("test 2".into()),
        ..BlocklistItem::default()
      },
      BlocklistItem {
        id: 1,
        source_title: "test 3".to_owned(),
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
        series_title: None,
        ..BlocklistItem::default()
      },
    ]
  }

  fn sort_options() -> Vec<SortOption<BlocklistItem>> {
    vec![SortOption {
      name: "Test 1",
      cmp_fn: Some(|a, b| {
        b.source_title
          .to_lowercase()
          .cmp(&a.source_title.to_lowercase())
      }),
    }]
  }
}
