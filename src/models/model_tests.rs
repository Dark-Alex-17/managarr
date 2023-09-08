#[cfg(test)]
mod tests {
  use std::cell::RefCell;

  use pretty_assertions::{assert_eq, assert_str_eq};
  use serde::de::value::Error as ValueError;
  use serde::de::value::F64Deserializer;
  use serde::de::value::I64Deserializer;
  use serde::de::IntoDeserializer;
  use serde_json::to_string;

  use crate::models::from_i64;
  use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
  use crate::models::{
    BlockSelectionState, HorizontallyScrollableText, Scrollable, ScrollableText, StatefulList,
    StatefulTable, TabRoute, TabState,
  };

  const BLOCKS: [ActiveRadarrBlock; 6] = [
    ActiveRadarrBlock::AddMovieSelectRootFolder,
    ActiveRadarrBlock::AddMovieSelectMonitor,
    ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
    ActiveRadarrBlock::AddMovieSelectQualityProfile,
    ActiveRadarrBlock::AddMovieTagsInput,
    ActiveRadarrBlock::AddMovieConfirmPrompt,
  ];

  #[test]
  fn test_stateful_table_scrolling_on_empty_table_performs_no_op() {
    let mut stateful_table: StatefulTable<String> = StatefulTable::default();

    assert_eq!(stateful_table.state.selected(), None);

    stateful_table.scroll_up();

    assert_eq!(stateful_table.state.selected(), None);

    stateful_table.scroll_down();

    assert_eq!(stateful_table.state.selected(), None);

    stateful_table.scroll_to_top();

    assert_eq!(stateful_table.state.selected(), None);

    stateful_table.scroll_to_bottom();
  }

  #[test]
  fn test_stateful_table_scroll() {
    let mut stateful_table = create_test_stateful_table();

    assert_eq!(stateful_table.state.selected(), Some(0));

    stateful_table.scroll_down();

    assert_eq!(stateful_table.state.selected(), Some(1));

    stateful_table.scroll_down();

    assert_eq!(stateful_table.state.selected(), Some(0));

    stateful_table.scroll_up();

    assert_eq!(stateful_table.state.selected(), Some(1));

    stateful_table.scroll_up();

    assert_eq!(stateful_table.state.selected(), Some(0));

    stateful_table.scroll_to_bottom();

    assert_eq!(stateful_table.state.selected(), Some(1));

    stateful_table.scroll_to_top();

    assert_eq!(stateful_table.state.selected(), Some(0));
  }

  #[test]
  fn test_stateful_table_set_items() {
    let items_vec = vec!["Test 1", "Test 2", "Test 3"];
    let mut stateful_table: StatefulTable<&str> = StatefulTable::default();

    stateful_table.set_items(items_vec.clone());

    assert_eq!(stateful_table.state.selected(), Some(0));

    stateful_table.state.select(Some(1));
    stateful_table.set_items(items_vec.clone());

    assert_eq!(stateful_table.state.selected(), Some(1));

    stateful_table.state.select(Some(3));
    stateful_table.set_items(items_vec);

    assert_eq!(stateful_table.state.selected(), Some(2));
  }

  #[test]
  fn test_stateful_table_current_selection() {
    let mut stateful_table = create_test_stateful_table();

    assert_str_eq!(stateful_table.current_selection(), &stateful_table.items[0]);

    stateful_table.state.select(Some(1));

    assert_str_eq!(stateful_table.current_selection(), &stateful_table.items[1]);
  }

  #[test]
  fn test_stateful_table_select_index() {
    let mut stateful_table = create_test_stateful_table();

    assert_eq!(stateful_table.state.selected(), Some(0));

    stateful_table.select_index(Some(1));

    assert_eq!(stateful_table.state.selected(), Some(1));

    stateful_table.select_index(None);

    assert_eq!(stateful_table.state.selected(), None);
  }

  #[test]
  fn test_stateful_table_scroll_up() {
    let mut stateful_table = create_test_stateful_table();

    assert_eq!(stateful_table.state.selected(), Some(0));

    stateful_table.scroll_up();

    assert_eq!(stateful_table.state.selected(), Some(1));

    stateful_table.scroll_up();

    assert_eq!(stateful_table.state.selected(), Some(0));
  }

  #[test]
  fn test_stateful_list_scrolling_on_empty_table_performs_no_op() {
    let mut stateful_list: StatefulList<String> = StatefulList::default();

    assert_eq!(stateful_list.state.selected(), None);

    stateful_list.scroll_up();

    assert_eq!(stateful_list.state.selected(), None);

    stateful_list.scroll_down();

    assert_eq!(stateful_list.state.selected(), None);

    stateful_list.scroll_to_top();

    assert_eq!(stateful_list.state.selected(), None);

    stateful_list.scroll_to_bottom();
  }

  #[test]
  fn test_stateful_list_scroll() {
    let mut stateful_list = create_test_stateful_list();

    assert_eq!(stateful_list.state.selected(), Some(0));

    stateful_list.scroll_down();

    assert_eq!(stateful_list.state.selected(), Some(1));

    stateful_list.scroll_down();

    assert_eq!(stateful_list.state.selected(), Some(0));

    stateful_list.scroll_up();

    assert_eq!(stateful_list.state.selected(), Some(1));

    stateful_list.scroll_up();

    assert_eq!(stateful_list.state.selected(), Some(0));

    stateful_list.scroll_to_bottom();

    assert_eq!(stateful_list.state.selected(), Some(1));

    stateful_list.scroll_to_top();

    assert_eq!(stateful_list.state.selected(), Some(0));
  }

  #[test]
  fn test_stateful_list_set_items() {
    let items_vec = vec!["Test 1", "Test 2", "Test 3"];
    let mut stateful_list: StatefulList<&str> = StatefulList::default();

    stateful_list.set_items(items_vec.clone());

    assert_eq!(stateful_list.state.selected(), Some(0));

    stateful_list.state.select(Some(1));
    stateful_list.set_items(items_vec.clone());

    assert_eq!(stateful_list.state.selected(), Some(1));

    stateful_list.state.select(Some(3));
    stateful_list.set_items(items_vec);

    assert_eq!(stateful_list.state.selected(), Some(2));
  }

  #[test]
  fn test_stateful_list_current_selection() {
    let mut stateful_list = create_test_stateful_list();

    assert_str_eq!(stateful_list.current_selection(), &stateful_list.items[0]);

    stateful_list.state.select(Some(1));

    assert_str_eq!(stateful_list.current_selection(), &stateful_list.items[1]);
  }

  #[test]
  fn test_stateful_list_scroll_up() {
    let mut stateful_table = create_test_stateful_table();

    assert_eq!(stateful_table.state.selected(), Some(0));

    stateful_table.scroll_up();

    assert_eq!(stateful_table.state.selected(), Some(1));

    stateful_table.scroll_up();

    assert_eq!(stateful_table.state.selected(), Some(0));
  }

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

    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 0);
    assert_str_eq!(horizontally_scrollable_text.text, test_text);
  }

  #[test]
  fn test_horizontally_scrollable_text_from_str() {
    let test_text = "Test string";
    let horizontally_scrollable_text = HorizontallyScrollableText::from(test_text);

    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 0);
    assert_str_eq!(horizontally_scrollable_text.text, test_text);
  }

  #[test]
  fn test_horizontally_scrollable_text_to_string() {
    let test_text = "Test string";
    let horizontally_scrollable_text = HorizontallyScrollableText::from(test_text);

    assert_str_eq!(horizontally_scrollable_text.to_string(), test_text);

    let horizontally_scrollable_text = HorizontallyScrollableText {
      text: test_text.to_owned(),
      offset: RefCell::new(test_text.len() - 1),
    };

    assert_str_eq!(horizontally_scrollable_text.to_string(), "g");

    let horizontally_scrollable_text = HorizontallyScrollableText {
      text: test_text.to_owned(),
      offset: RefCell::new(test_text.len()),
    };

    assert!(horizontally_scrollable_text.to_string().is_empty());
  }

  #[test]
  fn test_horizontally_scrollable_text_new() {
    let test_text = "Test string";
    let horizontally_scrollable_text = HorizontallyScrollableText::new(test_text.to_owned());

    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 0);
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

    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 0);

    for i in 1..horizontally_scrollable_text.text.len() - 1 {
      horizontally_scrollable_text.scroll_left();

      assert_eq!(*horizontally_scrollable_text.offset.borrow(), i);
    }

    horizontally_scrollable_text.scroll_left();

    assert_eq!(
      *horizontally_scrollable_text.offset.borrow(),
      horizontally_scrollable_text.text.len() - 1
    );
  }

  #[test]
  fn test_horizontally_scrollable_text_scroll_text_left_uses_len_method() {
    let horizontally_scrollable_text = HorizontallyScrollableText::from("우리");

    horizontally_scrollable_text.scroll_left();

    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 1);
    assert_str_eq!(horizontally_scrollable_text.to_string(), "리");

    horizontally_scrollable_text.scroll_left();

    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 2);
    assert_str_eq!(horizontally_scrollable_text.to_string(), "");

    horizontally_scrollable_text.scroll_left();

    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 2);
    assert!(horizontally_scrollable_text.to_string().is_empty());
  }

  #[test]
  fn test_horizontally_scrollable_text_scroll_text_right() {
    let horizontally_scrollable_text = HorizontallyScrollableText::from("Test string");
    *horizontally_scrollable_text.offset.borrow_mut() = horizontally_scrollable_text.text.len();

    for i in 1..horizontally_scrollable_text.text.len() {
      horizontally_scrollable_text.scroll_right();

      assert_eq!(
        *horizontally_scrollable_text.offset.borrow(),
        horizontally_scrollable_text.text.len() - i
      );
    }

    horizontally_scrollable_text.scroll_right();

    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 0);
  }

  #[test]
  fn test_horizontally_scrollable_text_scroll_home() {
    let horizontally_scrollable_text = HorizontallyScrollableText::from("Test string");

    horizontally_scrollable_text.scroll_home();

    assert_eq!(
      *horizontally_scrollable_text.offset.borrow(),
      horizontally_scrollable_text.text.len()
    );
  }

  #[test]
  fn test_horizontally_scrollable_text_scroll_home_uses_len_method() {
    let horizontally_scrollable_text = HorizontallyScrollableText::from("우리");

    horizontally_scrollable_text.scroll_home();

    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 2);
  }

  #[test]
  fn test_horizontally_scrollable_text_reset_offset() {
    let horizontally_scrollable_text = HorizontallyScrollableText {
      text: "Test string".to_owned(),
      offset: RefCell::new(1),
    };

    horizontally_scrollable_text.reset_offset();

    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 0);
  }

  #[test]
  fn test_horizontally_scrollable_text_scroll_left_or_reset() {
    let width = 3;
    let test_text = "Test string";
    let horizontally_scrollable_text = HorizontallyScrollableText::from(test_text);

    horizontally_scrollable_text.scroll_left_or_reset(width, true, true);

    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 1);

    horizontally_scrollable_text.scroll_left_or_reset(width, false, true);

    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 0);

    horizontally_scrollable_text.scroll_left_or_reset(width, true, false);

    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 0);

    horizontally_scrollable_text.scroll_left_or_reset(width, true, true);

    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 1);

    horizontally_scrollable_text.scroll_left_or_reset(test_text.len(), false, true);

    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 0);
  }

  #[test]
  fn test_horizontally_scrollable_text_scroll_left_or_reset_resets_when_text_unselected() {
    let horizontally_scrollable_test = HorizontallyScrollableText::from("Test string");
    horizontally_scrollable_test.scroll_left();

    assert_eq!(*horizontally_scrollable_test.offset.borrow(), 1);

    horizontally_scrollable_test.scroll_left_or_reset(3, false, false);

    assert_eq!(*horizontally_scrollable_test.offset.borrow(), 0);
  }

  #[test]
  fn test_horizontally_scrollable_text_scroll_left_or_reset_uses_len_method() {
    let horizontally_scrollable_text = HorizontallyScrollableText::from("우리");
    let width = 1;

    horizontally_scrollable_text.scroll_left_or_reset(width, true, true);

    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 1);

    horizontally_scrollable_text.scroll_left_or_reset(width, true, true);

    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 2);

    horizontally_scrollable_text.scroll_left_or_reset(width, true, true);

    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 0);
  }

  #[test]
  fn test_horizontally_scrollable_text_pop() {
    let test_text = "Test sTrin우gs";
    let mut horizontally_scrollable_text = HorizontallyScrollableText::from(test_text);
    horizontally_scrollable_text.pop();

    assert_str_eq!(horizontally_scrollable_text.text, "Test sTrin우g");
    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 0);

    horizontally_scrollable_text.scroll_left();
    horizontally_scrollable_text.pop();

    assert_str_eq!(horizontally_scrollable_text.text, "Test sTring");
    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 1);

    horizontally_scrollable_text.scroll_right();
    horizontally_scrollable_text.scroll_right();
    horizontally_scrollable_text.pop();

    assert_str_eq!(horizontally_scrollable_text.text, "Test sTrin");
    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 0);

    horizontally_scrollable_text.scroll_home();
    horizontally_scrollable_text.pop();

    assert_str_eq!(horizontally_scrollable_text.text, "Test sTrin");
    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 10);

    horizontally_scrollable_text.scroll_right();
    horizontally_scrollable_text.pop();

    assert_str_eq!(horizontally_scrollable_text.text, "est sTrin");
    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 9);
  }

  #[test]
  fn test_horizontally_scrollable_text_pop_uses_len_method() {
    let mut horizontally_scrollable_text = HorizontallyScrollableText::from("우리");
    horizontally_scrollable_text.pop();

    assert_str_eq!(horizontally_scrollable_text.text, "우");
    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 0);

    horizontally_scrollable_text.pop();

    assert!(horizontally_scrollable_text.text.is_empty());
    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 0);

    horizontally_scrollable_text.pop();

    assert!(horizontally_scrollable_text.text.is_empty());
    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 0);
  }

  #[test]
  fn test_horizontally_scrollable_text_push() {
    let test_text = "Test stri우ng";
    let mut horizontally_scrollable_text = HorizontallyScrollableText::from(test_text);
    horizontally_scrollable_text.push('h');

    assert_str_eq!(horizontally_scrollable_text.text, "Test stri우ngh");
    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 0);

    horizontally_scrollable_text.scroll_left();
    horizontally_scrollable_text.push('l');

    assert_str_eq!(horizontally_scrollable_text.text, "Test stri우nglh");
    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 1);

    horizontally_scrollable_text.scroll_right();
    horizontally_scrollable_text.scroll_right();
    horizontally_scrollable_text.push('리');

    assert_str_eq!(horizontally_scrollable_text.text, "Test stri우nglh리");
    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 0);
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

    assert_eq!(active_route, &second_tab);
  }

  #[test]
  fn test_tab_state_get_active_tab_help() {
    let tabs = create_test_tab_routes();
    let second_tab_help = tabs[1].help.clone();
    let tab_state = TabState { tabs, index: 1 };

    let tab_help = tab_state.get_active_tab_help();

    assert_str_eq!(tab_help, second_tab_help);
  }

  #[test]
  fn test_tab_state_get_active_tab_contextual_help() {
    let tabs = create_test_tab_routes();
    let second_tab_contextual_help = tabs[1].contextual_help.clone().unwrap();
    let tab_state = TabState { tabs, index: 1 };

    let tab_contextual_help = tab_state.get_active_tab_contextual_help();

    assert!(tab_contextual_help.is_some());
    assert_str_eq!(tab_contextual_help.unwrap(), second_tab_contextual_help);
  }

  #[test]
  fn test_tab_state_next() {
    let tab_routes = create_test_tab_routes();
    let mut tab_state = TabState::new(create_test_tab_routes());

    assert_eq!(tab_state.get_active_route(), &tab_routes[0].route);

    tab_state.next();

    assert_eq!(tab_state.get_active_route(), &tab_routes[1].route);

    tab_state.next();

    assert_eq!(tab_state.get_active_route(), &tab_routes[0].route);
  }

  #[test]
  fn test_tab_state_previous() {
    let tab_routes = create_test_tab_routes();
    let mut tab_state = TabState::new(create_test_tab_routes());

    assert_eq!(tab_state.get_active_route(), &tab_routes[0].route);

    tab_state.previous();

    assert_eq!(tab_state.get_active_route(), &tab_routes[1].route);

    tab_state.previous();

    assert_eq!(tab_state.get_active_route(), &tab_routes[0].route);
  }

  #[test]
  fn test_block_selection_state_new() {
    let block_selection_state = BlockSelectionState::new(&BLOCKS);

    assert_eq!(block_selection_state.index, 0);
  }

  #[test]
  fn test_block_selection_state_get_active_block() {
    let second_block = BLOCKS[1];
    let block_selection_state = BlockSelectionState {
      blocks: &BLOCKS,
      index: 1,
    };

    let active_block = block_selection_state.get_active_block();

    assert_eq!(active_block, &second_block);
  }

  #[test]
  fn test_block_selection_state_next() {
    let blocks = [
      ActiveRadarrBlock::AddMovieSelectRootFolder,
      ActiveRadarrBlock::AddMovieSelectMonitor,
    ];
    let mut block_selection_state = BlockSelectionState::new(&blocks);

    assert_eq!(block_selection_state.get_active_block(), &blocks[0]);

    block_selection_state.next();

    assert_eq!(block_selection_state.get_active_block(), &blocks[1]);

    block_selection_state.next();

    assert_eq!(block_selection_state.get_active_block(), &blocks[0]);
  }

  #[test]
  fn test_block_selection_state_previous() {
    let blocks = [
      ActiveRadarrBlock::AddMovieSelectRootFolder,
      ActiveRadarrBlock::AddMovieSelectMonitor,
    ];
    let mut block_selection_state = BlockSelectionState::new(&blocks);

    assert_eq!(block_selection_state.get_active_block(), &blocks[0]);

    block_selection_state.previous();

    assert_eq!(block_selection_state.get_active_block(), &blocks[1]);

    block_selection_state.previous();

    assert_eq!(block_selection_state.get_active_block(), &blocks[0]);
  }

  #[test]
  fn test_from_i64() {
    let deserializer: I64Deserializer<ValueError> = 1i64.into_deserializer();

    assert_eq!(from_i64(deserializer), Ok(1));
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
  fn test_horizontally_scrollable_serialize() {
    let text = HorizontallyScrollableText::from("Test");

    let serialized = to_string(&text).expect("Serialization failed!");

    assert_str_eq!(serialized, r#""Test""#);
  }

  fn create_test_tab_routes() -> Vec<TabRoute> {
    vec![
      TabRoute {
        title: "Test 1",
        route: ActiveRadarrBlock::Movies.into(),
        help: "Help for Test 1".to_owned(),
        contextual_help: Some("Contextual Help for Test 1".to_owned()),
      },
      TabRoute {
        title: "Test 2",
        route: ActiveRadarrBlock::Collections.into(),
        help: "Help for Test 2".to_owned(),
        contextual_help: Some("Contextual Help for Test 2".to_owned()),
      },
    ]
  }

  fn create_test_stateful_table() -> StatefulTable<&'static str> {
    let mut stateful_table = StatefulTable::default();
    stateful_table.set_items(vec!["Test 1", "Test 2"]);

    stateful_table
  }

  fn create_test_stateful_list() -> StatefulList<&'static str> {
    let mut stateful_list = StatefulList::default();
    stateful_list.set_items(vec!["Test 1", "Test 2"]);

    stateful_list
  }
}
