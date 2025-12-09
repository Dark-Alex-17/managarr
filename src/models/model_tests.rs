#[cfg(test)]
mod tests {
  use std::sync::atomic::AtomicUsize;
  use std::sync::atomic::Ordering;

  use crate::app::ServarrConfig;
  use crate::app::context_clues::ContextClue;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::models::from_f64;
  use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
  use crate::models::{
    BlockSelectionState, HorizontallyScrollableText, Scrollable, ScrollableText, TabRoute, TabState,
  };
  use crate::models::{from_i64, strip_non_search_characters};
  use pretty_assertions::{assert_eq, assert_str_eq};
  use serde::de::IntoDeserializer;
  use serde::de::value::Error as ValueError;
  use serde::de::value::F64Deserializer;
  use serde::de::value::I64Deserializer;
  use serde_json::to_string;

  const BLOCKS: &[&[i32]] = &[&[11, 12], &[21, 22], &[31, 32]];
  static HELP_KEYBINDINGS: [ContextClue; 1] =
    [(DEFAULT_KEYBINDINGS.help, DEFAULT_KEYBINDINGS.help.desc)];
  static ESC_KEYBINDINGS: [ContextClue; 1] =
    [(DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc)];

  #[test]
  fn test_scrollable_text_with_string() {
    let scrollable_text = ScrollableText::with_string("Test \n String \n".to_owned());

    assert_eq!(scrollable_text.items.len(), 3);
    assert_eq!(scrollable_text.items, vec!["Test ", " String ", ""]);
    assert_eq!(scrollable_text.offset, 0);
  }

  #[test]
  fn test_scrollable_text_get_text() {
    let test_text = "Test \nString";
    let scrollable_text = ScrollableText::with_string(test_text.to_owned());

    assert_str_eq!(scrollable_text.get_text(), test_text);
  }

  #[test]
  fn test_scrollable_text_is_empty() {
    let scrollable_text = ScrollableText::default();

    assert_is_empty!(scrollable_text);

    let test_text = "Test \nString";
    let scrollable_text = ScrollableText::with_string(test_text.to_owned());

    assert!(!scrollable_text.is_empty());
  }

  #[test]
  fn test_scrollable_text_scroll() {
    let mut scrollable_text = ScrollableText::with_string("Test \nString".to_owned());

    scrollable_text.scroll_down();

    assert_eq!(scrollable_text.offset, 1);

    scrollable_text.scroll_down();

    assert_eq!(scrollable_text.offset, 1);

    scrollable_text.scroll_up();

    assert_eq!(scrollable_text.offset, 0);

    scrollable_text.scroll_up();

    assert_eq!(scrollable_text.offset, 0);

    scrollable_text.scroll_to_bottom();

    assert_eq!(scrollable_text.offset, 1);

    scrollable_text.scroll_to_top();

    assert_eq!(scrollable_text.offset, 0);
  }

  #[test]
  fn test_scrollable_text_scroll_up_or_down_performs_no_op_on_empty_text() {
    let mut scrollable_text = ScrollableText::default();

    scrollable_text.scroll_up();

    assert_eq!(scrollable_text.offset, 0);

    scrollable_text.scroll_down();

    assert_eq!(scrollable_text.offset, 0);
  }

  #[test]
  fn test_horizontally_scrollable_text_from_string() {
    let test_text = "Test string";
    let horizontally_scrollable_text = HorizontallyScrollableText::from(test_text.to_owned());

    assert_eq!(
      horizontally_scrollable_text.offset.load(Ordering::SeqCst),
      0
    );
    assert_str_eq!(horizontally_scrollable_text.text, test_text);
  }

  #[test]
  fn test_horizontally_scrollable_text_from_string_ref() {
    let test_text = "Test string".to_owned();
    let horizontally_scrollable_text = HorizontallyScrollableText::from(&test_text);

    assert_eq!(
      horizontally_scrollable_text.offset.load(Ordering::SeqCst),
      0
    );
    assert_str_eq!(horizontally_scrollable_text.text, test_text);
  }

  #[test]
  fn test_horizontally_scrollable_text_from_str() {
    let test_text = "Test string";
    let horizontally_scrollable_text = HorizontallyScrollableText::from(test_text);

    assert_eq!(
      horizontally_scrollable_text.offset.load(Ordering::SeqCst),
      0
    );
    assert_str_eq!(horizontally_scrollable_text.text, test_text);
  }

  #[test]
  fn test_horizontally_scrollable_text_to_string() {
    let test_text = "Test string";
    let horizontally_scrollable_text = HorizontallyScrollableText::from(test_text);

    assert_str_eq!(horizontally_scrollable_text.to_string(), test_text);

    let horizontally_scrollable_text = HorizontallyScrollableText {
      text: test_text.to_owned(),
      offset: AtomicUsize::new(test_text.len() - 1),
    };

    assert_str_eq!(horizontally_scrollable_text.to_string(), "g");

    let horizontally_scrollable_text = HorizontallyScrollableText {
      text: test_text.to_owned(),
      offset: AtomicUsize::new(test_text.len()),
    };

    assert_is_empty!(horizontally_scrollable_text.to_string());
  }

  #[test]
  fn test_horizontally_scrollable_text_new() {
    let test_text = "Test string";
    let horizontally_scrollable_text = HorizontallyScrollableText::new(test_text.to_owned());

    assert_eq!(
      horizontally_scrollable_text.offset.load(Ordering::SeqCst),
      0
    );
    assert_str_eq!(horizontally_scrollable_text.text, test_text);
  }

  #[test]
  fn test_horizontally_scrollable_text_len() {
    let test_text = "우리 생애 최고의 해Test.Text";
    let horizontally_scrollable_text = HorizontallyScrollableText::new(test_text.to_owned());

    assert_eq!(horizontally_scrollable_text.len(), 20);
    assert_eq!(horizontally_scrollable_text.text.len(), 36);
    assert_str_eq!(horizontally_scrollable_text.text, test_text);
  }

  #[test]
  fn test_horizontally_scrollable_text_scroll_text_left() {
    let horizontally_scrollable_text = HorizontallyScrollableText::from("Test string");

    assert_eq!(
      horizontally_scrollable_text.offset.load(Ordering::SeqCst),
      0
    );

    for i in 1..horizontally_scrollable_text.text.len() - 1 {
      horizontally_scrollable_text.scroll_left();

      assert_eq!(
        horizontally_scrollable_text.offset.load(Ordering::SeqCst),
        i
      );
    }

    horizontally_scrollable_text.scroll_left();

    assert_eq!(
      horizontally_scrollable_text.offset.load(Ordering::SeqCst),
      horizontally_scrollable_text.text.len() - 1
    );
  }

  #[test]
  fn test_horizontally_scrollable_text_scroll_text_left_uses_len_method() {
    let horizontally_scrollable_text = HorizontallyScrollableText::from("우리");

    horizontally_scrollable_text.scroll_left();

    assert_eq!(
      horizontally_scrollable_text.offset.load(Ordering::SeqCst),
      1
    );
    assert_str_eq!(horizontally_scrollable_text.to_string(), "리");

    horizontally_scrollable_text.scroll_left();

    assert_eq!(
      horizontally_scrollable_text.offset.load(Ordering::SeqCst),
      2
    );
    assert_str_eq!(horizontally_scrollable_text.to_string(), "");

    horizontally_scrollable_text.scroll_left();

    assert_eq!(
      horizontally_scrollable_text.offset.load(Ordering::SeqCst),
      2
    );
    assert_is_empty!(horizontally_scrollable_text.to_string());
  }

  #[test]
  fn test_horizontally_scrollable_text_scroll_text_right() {
    let horizontally_scrollable_text = HorizontallyScrollableText::from("Test string");
    horizontally_scrollable_text
      .offset
      .store(horizontally_scrollable_text.len(), Ordering::SeqCst);

    for i in 1..horizontally_scrollable_text.text.len() {
      horizontally_scrollable_text.scroll_right();

      assert_eq!(
        horizontally_scrollable_text.offset.load(Ordering::SeqCst),
        horizontally_scrollable_text.text.len() - i
      );
    }

    horizontally_scrollable_text.scroll_right();

    assert_eq!(
      horizontally_scrollable_text.offset.load(Ordering::SeqCst),
      0
    );
  }

  #[test]
  fn test_horizontally_scrollable_text_scroll_home() {
    let horizontally_scrollable_text = HorizontallyScrollableText::from("Test string");

    horizontally_scrollable_text.scroll_home();

    assert_eq!(
      horizontally_scrollable_text.offset.load(Ordering::SeqCst),
      horizontally_scrollable_text.text.len()
    );
  }

  #[test]
  fn test_horizontally_scrollable_text_scroll_home_uses_len_method() {
    let horizontally_scrollable_text = HorizontallyScrollableText::from("우리");

    horizontally_scrollable_text.scroll_home();

    assert_eq!(
      horizontally_scrollable_text.offset.load(Ordering::SeqCst),
      2
    );
  }

  #[test]
  fn test_horizontally_scrollable_text_reset_offset() {
    let horizontally_scrollable_text = HorizontallyScrollableText {
      text: "Test string".to_owned(),
      offset: AtomicUsize::new(1),
    };

    horizontally_scrollable_text.reset_offset();

    assert_eq!(
      horizontally_scrollable_text.offset.load(Ordering::SeqCst),
      0
    );
  }

  #[test]
  fn test_horizontally_scrollable_text_scroll_left_or_reset() {
    let width = 3;
    let test_text = "Test string";
    let horizontally_scrollable_text = HorizontallyScrollableText::from(test_text);

    horizontally_scrollable_text.scroll_left_or_reset(width, true, true);

    assert_eq!(
      horizontally_scrollable_text.offset.load(Ordering::SeqCst),
      1
    );

    horizontally_scrollable_text.scroll_left_or_reset(width, false, true);

    assert_eq!(
      horizontally_scrollable_text.offset.load(Ordering::SeqCst),
      0
    );

    horizontally_scrollable_text.scroll_left_or_reset(width, true, false);

    assert_eq!(
      horizontally_scrollable_text.offset.load(Ordering::SeqCst),
      0
    );

    horizontally_scrollable_text.scroll_left_or_reset(width, true, true);

    assert_eq!(
      horizontally_scrollable_text.offset.load(Ordering::SeqCst),
      1
    );

    horizontally_scrollable_text.scroll_left_or_reset(test_text.len(), false, true);

    assert_eq!(
      horizontally_scrollable_text.offset.load(Ordering::SeqCst),
      0
    );
  }

  #[test]
  fn test_horizontally_scrollable_text_scroll_left_or_reset_resets_when_text_unselected() {
    let horizontally_scrollable_test = HorizontallyScrollableText::from("Test string");
    horizontally_scrollable_test.scroll_left();

    assert_eq!(
      horizontally_scrollable_test.offset.load(Ordering::SeqCst),
      1
    );

    horizontally_scrollable_test.scroll_left_or_reset(3, false, false);

    assert_eq!(
      horizontally_scrollable_test.offset.load(Ordering::SeqCst),
      0
    );
  }

  #[test]
  fn test_horizontally_scrollable_text_scroll_left_or_reset_uses_len_method() {
    let horizontally_scrollable_text = HorizontallyScrollableText::from("우리");
    let width = 1;

    horizontally_scrollable_text.scroll_left_or_reset(width, true, true);

    assert_eq!(
      horizontally_scrollable_text.offset.load(Ordering::SeqCst),
      1
    );

    horizontally_scrollable_text.scroll_left_or_reset(width, true, true);

    assert_eq!(
      horizontally_scrollable_text.offset.load(Ordering::SeqCst),
      2
    );

    horizontally_scrollable_text.scroll_left_or_reset(width, true, true);

    assert_eq!(
      horizontally_scrollable_text.offset.load(Ordering::SeqCst),
      0
    );
  }

  #[test]
  fn test_horizontally_scrollable_text_pop() {
    let test_text = "Test sTrin우gs";
    let mut horizontally_scrollable_text = HorizontallyScrollableText::from(test_text);
    horizontally_scrollable_text.pop();

    assert_str_eq!(horizontally_scrollable_text.text, "Test sTrin우g");
    assert_eq!(
      horizontally_scrollable_text.offset.load(Ordering::SeqCst),
      0
    );

    horizontally_scrollable_text.scroll_left();
    horizontally_scrollable_text.pop();

    assert_str_eq!(horizontally_scrollable_text.text, "Test sTring");
    assert_eq!(
      horizontally_scrollable_text.offset.load(Ordering::SeqCst),
      1
    );

    horizontally_scrollable_text.scroll_right();
    horizontally_scrollable_text.scroll_right();
    horizontally_scrollable_text.pop();

    assert_str_eq!(horizontally_scrollable_text.text, "Test sTrin");
    assert_eq!(
      horizontally_scrollable_text.offset.load(Ordering::SeqCst),
      0
    );

    horizontally_scrollable_text.scroll_home();
    horizontally_scrollable_text.pop();

    assert_str_eq!(horizontally_scrollable_text.text, "Test sTrin");
    assert_eq!(
      horizontally_scrollable_text.offset.load(Ordering::SeqCst),
      10
    );

    horizontally_scrollable_text.scroll_right();
    horizontally_scrollable_text.pop();

    assert_str_eq!(horizontally_scrollable_text.text, "est sTrin");
    assert_eq!(
      horizontally_scrollable_text.offset.load(Ordering::SeqCst),
      9
    );
  }

  #[test]
  fn test_horizontally_scrollable_text_pop_uses_len_method() {
    let mut horizontally_scrollable_text = HorizontallyScrollableText::from("우리");
    horizontally_scrollable_text.pop();

    assert_str_eq!(horizontally_scrollable_text.text, "우");
    assert_eq!(
      horizontally_scrollable_text.offset.load(Ordering::SeqCst),
      0
    );

    horizontally_scrollable_text.pop();

    assert_is_empty!(horizontally_scrollable_text.text);
    assert_eq!(
      horizontally_scrollable_text.offset.load(Ordering::SeqCst),
      0
    );

    horizontally_scrollable_text.pop();

    assert_is_empty!(horizontally_scrollable_text.text);
    assert_eq!(
      horizontally_scrollable_text.offset.load(Ordering::SeqCst),
      0
    );
  }

  #[test]
  fn test_horizontally_scrollable_text_push() {
    let test_text = "Test stri우ng";
    let mut horizontally_scrollable_text = HorizontallyScrollableText::from(test_text);
    horizontally_scrollable_text.push('h');

    assert_str_eq!(horizontally_scrollable_text.text, "Test stri우ngh");
    assert_eq!(
      horizontally_scrollable_text.offset.load(Ordering::SeqCst),
      0
    );

    horizontally_scrollable_text.scroll_left();
    horizontally_scrollable_text.push('l');

    assert_str_eq!(horizontally_scrollable_text.text, "Test stri우nglh");
    assert_eq!(
      horizontally_scrollable_text.offset.load(Ordering::SeqCst),
      1
    );

    horizontally_scrollable_text.scroll_right();
    horizontally_scrollable_text.scroll_right();
    horizontally_scrollable_text.push('리');

    assert_str_eq!(horizontally_scrollable_text.text, "Test stri우nglh리");
    assert_eq!(
      horizontally_scrollable_text.offset.load(Ordering::SeqCst),
      0
    );
  }

  #[test]
  fn test_tab_state_new() {
    let tab_state = TabState::new(create_test_tab_routes());

    assert_eq!(tab_state.index, 0);
  }

  #[test]
  fn test_tab_state_set_index() {
    let mut tab_state = TabState::new(create_test_tab_routes());

    let result = tab_state.set_index(1);

    assert_eq!(result, &create_test_tab_routes()[1]);
    assert_eq!(tab_state.index, 1);
  }

  #[test]
  fn test_tab_state_get_active_route() {
    let tabs = create_test_tab_routes();
    let second_tab = tabs[1].route;
    let tab_state = TabState { tabs, index: 1 };

    let active_route = tab_state.get_active_route();

    assert_eq!(active_route, second_tab);
  }

  #[test]
  fn test_tab_state_get_active_config() {
    let mut tabs = create_test_tab_routes();
    tabs[1].config = Some(ServarrConfig {
      name: Some("Test".to_owned()),
      ..ServarrConfig::default()
    });
    let tab_state = TabState { tabs, index: 1 };

    let active_config = tab_state.get_active_config();

    assert_some!(active_config);
    assert_str_eq!(active_config.clone().unwrap().name.unwrap(), "Test");
  }

  #[test]
  fn test_tab_state_get_active_config_defaults_to_none() {
    let tabs = create_test_tab_routes();
    let tab_state = TabState { tabs, index: 1 };

    let active_config = tab_state.get_active_config();

    assert_none!(active_config);
  }

  #[test]
  fn test_select_tab_by_title() {
    let tabs = create_test_tab_routes();
    let mut tab_state = TabState { tabs, index: 0 };

    let result = tab_state.select_tab_by_title("Test 2");

    assert!(result);
    assert_eq!(tab_state.index, 1);

    let result = tab_state.select_tab_by_title("Not real");

    assert!(!result);
    assert_eq!(tab_state.index, 1);
  }

  #[test]
  fn test_select_tab_by_title_empty_tabs_returns_false() {
    let mut tab_state = TabState {
      tabs: vec![],
      index: 0,
    };

    let result = tab_state.select_tab_by_title("Test 2");

    assert!(!result);
    assert_eq!(tab_state.index, 0);
  }

  #[test]
  fn test_select_tab_by_config() {
    let mut tabs = create_test_tab_routes();
    tabs[0].config = Some(ServarrConfig {
      name: Some("Test 1".to_owned()),
      ..ServarrConfig::default()
    });
    tabs[1].config = Some(ServarrConfig {
      host: Some("http://localhost".to_owned()),
      port: Some(7878),
      ..ServarrConfig::default()
    });
    let mut tab_state = TabState { tabs, index: 0 };

    let result = tab_state.select_tab_by_config(&ServarrConfig {
      host: Some("http://localhost".to_owned()),
      port: Some(7878),
      ..ServarrConfig::default()
    });

    assert!(result);
    assert_eq!(tab_state.index, 1);

    let result = tab_state.select_tab_by_config(&ServarrConfig {
      name: Some("Not real".to_owned()),
      ..ServarrConfig::default()
    });

    assert!(!result);
    assert_eq!(tab_state.index, 1);
  }

  #[test]
  fn test_select_tab_by_config_empty_tabs_returns_false() {
    let mut tab_state = TabState {
      tabs: vec![],
      index: 0,
    };

    let result = tab_state.select_tab_by_config(&ServarrConfig {
      host: Some("http://localhost".to_owned()),
      port: Some(7878),
      ..ServarrConfig::default()
    });

    assert!(!result);
    assert_eq!(tab_state.index, 0);
  }

  #[test]
  fn test_tab_state_get_active_route_contextual_help() {
    let tabs = create_test_tab_routes();
    let second_tab_help = tabs[1].contextual_help;
    let tab_state = TabState { tabs, index: 1 };

    let tab_help = tab_state.get_active_route_contextual_help();

    assert_some_eq_x!(tab_help, second_tab_help.unwrap());
  }

  #[test]
  fn test_tab_state_next() {
    let tab_routes = create_test_tab_routes();
    let mut tab_state = TabState::new(create_test_tab_routes());

    assert_eq!(tab_state.get_active_route(), tab_routes[0].route);

    tab_state.next();

    assert_eq!(tab_state.get_active_route(), tab_routes[1].route);

    tab_state.next();

    assert_eq!(tab_state.get_active_route(), tab_routes[0].route);
  }

  #[test]
  fn test_tab_state_previous() {
    let tab_routes = create_test_tab_routes();
    let mut tab_state = TabState::new(create_test_tab_routes());

    assert_eq!(tab_state.get_active_route(), tab_routes[0].route);

    tab_state.previous();

    assert_eq!(tab_state.get_active_route(), tab_routes[1].route);

    tab_state.previous();

    assert_eq!(tab_state.get_active_route(), tab_routes[0].route);
  }

  #[test]
  fn test_block_selection_state_new() {
    let block_selection_state = BlockSelectionState::new(BLOCKS);

    assert_eq!(block_selection_state.x, 0);
    assert_eq!(block_selection_state.y, 0);
  }

  #[test]
  fn test_block_selection_state_get_active_block() {
    let second_block = BLOCKS[1][1];
    let block_selection_state = BlockSelectionState {
      blocks: BLOCKS,
      x: 1,
      y: 1,
    };

    let active_block = block_selection_state.get_active_block();

    assert_eq!(active_block, second_block);
  }

  #[test]
  fn test_block_selection_state_down() {
    let mut block_selection_state = BlockSelectionState::new(BLOCKS);

    assert_eq!(block_selection_state.get_active_block(), BLOCKS[0][0]);

    block_selection_state.down();

    assert_eq!(block_selection_state.get_active_block(), BLOCKS[1][0]);

    block_selection_state.down();

    assert_eq!(block_selection_state.get_active_block(), BLOCKS[2][0]);

    block_selection_state.down();

    assert_eq!(block_selection_state.get_active_block(), BLOCKS[0][0]);
  }

  #[test]
  fn test_block_selection_state_up() {
    let mut block_selection_state = BlockSelectionState::new(BLOCKS);

    assert_eq!(block_selection_state.get_active_block(), BLOCKS[0][0]);

    block_selection_state.up();

    assert_eq!(block_selection_state.get_active_block(), BLOCKS[2][0]);

    block_selection_state.up();

    assert_eq!(block_selection_state.get_active_block(), BLOCKS[1][0]);

    block_selection_state.up();

    assert_eq!(block_selection_state.get_active_block(), BLOCKS[0][0]);
  }

  #[test]
  fn test_block_selection_state_left() {
    let mut block_selection_state = BlockSelectionState::new(BLOCKS);

    assert_eq!(block_selection_state.get_active_block(), BLOCKS[0][0]);

    block_selection_state.left();

    assert_eq!(block_selection_state.get_active_block(), BLOCKS[0][1]);

    block_selection_state.left();

    assert_eq!(block_selection_state.get_active_block(), BLOCKS[0][0]);

    block_selection_state.set_index(0, 1);

    assert_eq!(block_selection_state.get_active_block(), BLOCKS[1][0]);

    block_selection_state.left();

    assert_eq!(block_selection_state.get_active_block(), BLOCKS[1][1]);

    block_selection_state.left();

    assert_eq!(block_selection_state.get_active_block(), BLOCKS[1][0]);
  }

  #[test]
  fn test_block_selection_state_right() {
    let mut block_selection_state = BlockSelectionState::new(BLOCKS);

    assert_eq!(block_selection_state.get_active_block(), BLOCKS[0][0]);

    block_selection_state.right();

    assert_eq!(block_selection_state.get_active_block(), BLOCKS[0][1]);

    block_selection_state.right();

    assert_eq!(block_selection_state.get_active_block(), BLOCKS[0][0]);

    block_selection_state.set_index(0, 1);

    assert_eq!(block_selection_state.get_active_block(), BLOCKS[1][0]);

    block_selection_state.right();

    assert_eq!(block_selection_state.get_active_block(), BLOCKS[1][1]);

    block_selection_state.right();

    assert_eq!(block_selection_state.get_active_block(), BLOCKS[1][0]);
  }

  #[test]
  fn test_from_i64() {
    let deserializer: I64Deserializer<ValueError> = 1i64.into_deserializer();

    assert_ok_eq_x!(from_i64(deserializer), 1);
  }

  #[test]
  fn test_from_i64_error() {
    let deserializer: F64Deserializer<ValueError> = 1f64.into_deserializer();

    assert_str_eq!(
      from_i64(deserializer).unwrap_err().to_string(),
      "Unable to convert Number to i64: Number(1.0)"
    );
  }

  #[test]
  fn test_from_f64() {
    let deserializer: F64Deserializer<ValueError> = 1f64.into_deserializer();

    assert_ok_eq_x!(from_f64(deserializer), 1.0);
  }

  #[test]
  fn test_horizontally_scrollable_serialize() {
    let text = HorizontallyScrollableText::from("Test");

    let serialized = to_string(&text).expect("Serialization failed!");

    assert_str_eq!(serialized, r#""Test""#);
  }

  fn create_test_tab_routes() -> Vec<TabRoute> {
    vec![
      TabRoute {
        title: "Test 1".to_owned(),
        route: ActiveRadarrBlock::Movies.into(),
        contextual_help: Some(&HELP_KEYBINDINGS),
        config: None,
      },
      TabRoute {
        title: "Test 2".to_owned(),
        route: ActiveRadarrBlock::Collections.into(),
        contextual_help: Some(&ESC_KEYBINDINGS),
        config: None,
      },
    ]
  }

  #[test]
  fn test_strip_non_alphanumeric_characters() {
    assert_eq!(
      strip_non_search_characters("Te$t S7r!ng::'~-@_`,(.)/*}^&%#+="),
      "tet s7rng::'-,./".to_owned()
    )
  }
}
