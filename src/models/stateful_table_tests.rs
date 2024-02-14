#[cfg(test)]
mod tests {
  use crate::models::stateful_table::{SortOption, StatefulTable};
  use crate::models::Scrollable;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use ratatui::widgets::TableState;

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
  fn test_stateful_table_filtered_scrolling_on_empty_table_performs_no_op() {
    let mut filtered_stateful_table: StatefulTable<String> = StatefulTable {
      filtered_items: Some(Vec::new()),
      filtered_state: Some(TableState::default()),
      ..StatefulTable::default()
    };

    assert_eq!(
      filtered_stateful_table
        .filtered_state
        .as_ref()
        .unwrap()
        .selected(),
      None
    );

    filtered_stateful_table.scroll_up();

    assert_eq!(
      filtered_stateful_table
        .filtered_state
        .as_ref()
        .unwrap()
        .selected(),
      None
    );

    filtered_stateful_table.scroll_down();

    assert_eq!(
      filtered_stateful_table
        .filtered_state
        .as_ref()
        .unwrap()
        .selected(),
      None
    );

    filtered_stateful_table.scroll_to_top();

    assert_eq!(
      filtered_stateful_table
        .filtered_state
        .as_ref()
        .unwrap()
        .selected(),
      None
    );

    filtered_stateful_table.scroll_to_bottom();
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
  fn test_stateful_table_filtered_items_scroll() {
    let mut filtered_stateful_table = create_test_filtered_stateful_table();

    assert_eq!(
      filtered_stateful_table
        .filtered_state
        .as_ref()
        .unwrap()
        .selected(),
      Some(0)
    );

    filtered_stateful_table.scroll_down();

    assert_eq!(
      filtered_stateful_table
        .filtered_state
        .as_ref()
        .unwrap()
        .selected(),
      Some(1)
    );

    filtered_stateful_table.scroll_down();

    assert_eq!(
      filtered_stateful_table
        .filtered_state
        .as_ref()
        .unwrap()
        .selected(),
      Some(0)
    );

    filtered_stateful_table.scroll_up();

    assert_eq!(
      filtered_stateful_table
        .filtered_state
        .as_ref()
        .unwrap()
        .selected(),
      Some(1)
    );

    filtered_stateful_table.scroll_up();

    assert_eq!(
      filtered_stateful_table
        .filtered_state
        .as_ref()
        .unwrap()
        .selected(),
      Some(0)
    );

    filtered_stateful_table.scroll_to_bottom();

    assert_eq!(
      filtered_stateful_table
        .filtered_state
        .as_ref()
        .unwrap()
        .selected(),
      Some(1)
    );

    filtered_stateful_table.scroll_to_top();

    assert_eq!(
      filtered_stateful_table
        .filtered_state
        .as_ref()
        .unwrap()
        .selected(),
      Some(0)
    );
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
  fn test_stateful_table_set_filtered_items() {
    let filtered_items_vec = vec!["Test 1", "Test 2", "Test 3"];
    let mut filtered_stateful_table: StatefulTable<&str> = StatefulTable::default();

    filtered_stateful_table.set_filtered_items(filtered_items_vec.clone());

    assert_eq!(
      filtered_stateful_table
        .filtered_state
        .as_ref()
        .unwrap()
        .selected(),
      Some(0)
    );
    assert_eq!(
      filtered_stateful_table.filtered_items,
      Some(filtered_items_vec.clone())
    );
  }

  #[test]
  fn test_stateful_table_current_selection() {
    let mut stateful_table = create_test_stateful_table();

    assert_str_eq!(stateful_table.current_selection(), &stateful_table.items[0]);

    stateful_table.state.select(Some(1));

    assert_str_eq!(stateful_table.current_selection(), &stateful_table.items[1]);
  }

  #[test]
  fn test_stateful_table_sorting() {
    let sort_options: Vec<SortOption<String>> = vec![
      SortOption {
        name: "Test 1",
        cmp_fn: None,
      },
      SortOption {
        name: "Test 2",
        cmp_fn: None,
      },
    ];
    let mut stateful_table: StatefulTable<String> = StatefulTable::default();
    stateful_table.sorting(sort_options.clone());

    assert_eq!(
      stateful_table.sort.as_ref().unwrap().items,
      sort_options.clone()
    );
    assert_eq!(
      stateful_table.sort.as_ref().unwrap().current_selection(),
      &sort_options[0]
    );
  }

  #[test]
  fn test_stateful_table_apply_sorting_toggle_no_op_no_sort_options() {
    let mut stateful_table = create_test_stateful_table();
    let expected_items = stateful_table.items.clone();

    stateful_table.apply_sorting_toggle(true);

    assert_eq!(stateful_table.items, expected_items);
    assert!(!stateful_table.sort_asc);
  }

  #[test]
  fn test_stateful_table_apply_sorting_toggle_no_op_no_cmp_fn() {
    let mut stateful_table = create_test_stateful_table();
    stateful_table.sorting(vec![SortOption {
      name: "Test 1",
      cmp_fn: None,
    }]);
    let expected_items = stateful_table.items.clone();

    stateful_table.apply_sorting_toggle(true);

    assert_eq!(stateful_table.items, expected_items);
    assert!(stateful_table.sort_asc);
  }

  #[test]
  fn test_filtered_stateful_table_apply_sorting_toggle_no_op_no_cmp_fn() {
    let mut filtered_stateful_table = create_test_filtered_stateful_table();
    filtered_stateful_table.sorting(vec![SortOption {
      name: "Test 1",
      cmp_fn: None,
    }]);
    let expected_items = filtered_stateful_table
      .filtered_items
      .as_ref()
      .unwrap()
      .clone();

    filtered_stateful_table.apply_sorting_toggle(true);

    assert_eq!(
      *filtered_stateful_table.filtered_items.as_ref().unwrap(),
      expected_items
    );
    assert!(filtered_stateful_table.sort_asc);
  }

  #[test]
  fn test_stateful_table_apply_sorting_toggles_direction() {
    let mut stateful_table = create_test_stateful_table();
    stateful_table.sorting(vec![SortOption {
      name: "Test 1",
      cmp_fn: Some(|a, b| a.cmp(b)),
    }]);
    let mut expected_items = stateful_table.items.clone();
    expected_items.sort();

    stateful_table.apply_sorting_toggle(true);

    assert_eq!(stateful_table.items, expected_items);
    assert!(stateful_table.sort_asc);

    stateful_table.apply_sorting_toggle(true);

    expected_items.reverse();
    assert_eq!(stateful_table.items, expected_items);
    assert!(!stateful_table.sort_asc);
  }

  #[test]
  fn test_stateful_table_apply_sorting_toggle() {
    let mut stateful_table = create_test_stateful_table();
    stateful_table.sorting(vec![SortOption {
      name: "Test 1",
      cmp_fn: Some(|a, b| a.cmp(b)),
    }]);
    let mut expected_items = stateful_table.items.clone();
    expected_items.sort();

    stateful_table.apply_sorting_toggle(true);

    assert_eq!(stateful_table.items, expected_items);
    assert!(stateful_table.sort_asc);

    stateful_table.apply_sorting_toggle(true);

    expected_items.reverse();
    assert_eq!(stateful_table.items, expected_items);
    assert!(!stateful_table.sort_asc);
  }

  #[test]
  fn test_stateful_table_apply_sorting_toggle_false_doesnt_toggle_direction() {
    let mut stateful_table = create_test_stateful_table();
    stateful_table.sorting(vec![SortOption {
      name: "Test 1",
      cmp_fn: Some(|a, b| a.cmp(b)),
    }]);
    let mut expected_items = stateful_table.items.clone();
    expected_items.sort();
    expected_items.reverse();

    stateful_table.apply_sorting_toggle(false);

    assert_eq!(stateful_table.items, expected_items);
    assert!(!stateful_table.sort_asc);
  }

  #[test]
  fn test_filtered_stateful_table_apply_sorting_toggle() {
    let mut filtered_stateful_table = create_test_filtered_stateful_table();
    filtered_stateful_table.sorting(vec![SortOption {
      name: "Test 1",
      cmp_fn: Some(|a, b| a.cmp(b)),
    }]);
    let mut expected_items = filtered_stateful_table
      .filtered_items
      .as_mut()
      .unwrap()
      .clone();
    expected_items.sort();

    filtered_stateful_table.apply_sorting_toggle(true);

    assert_eq!(
      *filtered_stateful_table.filtered_items.as_ref().unwrap(),
      expected_items
    );
    assert!(filtered_stateful_table.sort_asc);

    filtered_stateful_table.apply_sorting_toggle(true);

    expected_items.reverse();
    assert_eq!(
      *filtered_stateful_table.filtered_items.as_ref().unwrap(),
      expected_items
    );
    assert!(!filtered_stateful_table.sort_asc);
  }

  #[test]
  fn test_filtered_stateful_table_current_selection() {
    let mut filtered_stateful_table = create_test_filtered_stateful_table();

    assert_str_eq!(
      filtered_stateful_table.current_selection(),
      &filtered_stateful_table.filtered_items.as_ref().unwrap()[0]
    );

    filtered_stateful_table
      .filtered_state
      .as_mut()
      .unwrap()
      .select(Some(1));

    assert_str_eq!(
      filtered_stateful_table.current_selection(),
      &filtered_stateful_table.filtered_items.as_ref().unwrap()[1]
    );
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
  fn test_filtered_stateful_table_select_index() {
    let mut filtered_stateful_table = create_test_filtered_stateful_table();

    assert_eq!(
      filtered_stateful_table
        .filtered_state
        .as_ref()
        .unwrap()
        .selected(),
      Some(0)
    );

    filtered_stateful_table.select_index(Some(1));

    assert_eq!(
      filtered_stateful_table
        .filtered_state
        .as_ref()
        .unwrap()
        .selected(),
      Some(1)
    );

    filtered_stateful_table.select_index(None);

    assert_eq!(
      filtered_stateful_table
        .filtered_state
        .as_ref()
        .unwrap()
        .selected(),
      None
    );
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
  fn test_filtered_stateful_table_scroll_up() {
    let mut filtered_stateful_table = create_test_filtered_stateful_table();

    assert_eq!(
      filtered_stateful_table
        .filtered_state
        .as_ref()
        .unwrap()
        .selected(),
      Some(0)
    );

    filtered_stateful_table.scroll_up();

    assert_eq!(
      filtered_stateful_table
        .filtered_state
        .as_ref()
        .unwrap()
        .selected(),
      Some(1)
    );

    filtered_stateful_table.scroll_up();

    assert_eq!(
      filtered_stateful_table
        .filtered_state
        .as_ref()
        .unwrap()
        .selected(),
      Some(0)
    );
  }

  #[test]
  fn test_stateful_table_apply_filter() {
    let mut stateful_table: StatefulTable<&str> = StatefulTable::default();
    stateful_table.set_items(vec!["this", "is", "a", "test"]);
    stateful_table.filter = Some("i".into());
    let expected_items = vec!["this", "is"];
    let mut expected_state = TableState::default();
    expected_state.select(Some(0));

    let has_matches = stateful_table.apply_filter(|&item| item);

    assert_eq!(stateful_table.filter, None);
    assert_eq!(stateful_table.filtered_items, Some(expected_items));
    assert_eq!(stateful_table.filtered_state, Some(expected_state));
    assert!(has_matches);
  }

  #[test]
  fn test_stateful_table_apply_filter_no_matches() {
    let mut stateful_table: StatefulTable<&str> = StatefulTable::default();
    stateful_table.set_items(vec!["this", "is", "a", "test"]);
    stateful_table.filter = Some("z".into());

    let has_matches = stateful_table.apply_filter(|&item| item);

    assert_eq!(stateful_table.filter, None);
    assert_eq!(stateful_table.filtered_items, None);
    assert_eq!(stateful_table.filtered_state, None);
    assert!(!has_matches);
  }

  #[test]
  fn test_stateful_table_reset_filter() {
    let mut stateful_table = create_test_filtered_stateful_table();
    stateful_table.reset_filter();

    assert_eq!(stateful_table.filter, None);
    assert_eq!(stateful_table.filtered_items, None);
    assert_eq!(stateful_table.filtered_state, None);
  }

  #[test]
  fn test_stateful_table_apply_search() {
    let mut stateful_table: StatefulTable<&str> = StatefulTable::default();
    stateful_table.set_items(vec!["this", "is", "a", "test"]);
    stateful_table.search = Some("test".into());
    let mut expected_state = TableState::default();
    expected_state.select(Some(3));

    let has_match = stateful_table.apply_search(|&item| item);

    assert_eq!(stateful_table.search, None);
    assert_eq!(stateful_table.state, expected_state);
    assert!(has_match);
  }

  #[test]
  fn test_stateful_table_apply_search_no_match() {
    let mut stateful_table: StatefulTable<&str> = StatefulTable::default();
    stateful_table.set_items(vec!["this", "is", "a", "test"]);
    stateful_table.search = Some("shi-mon-a!".into());

    let has_match = stateful_table.apply_search(|&item| item);

    assert_eq!(stateful_table.search, None);
    assert!(!has_match);
  }

  #[test]
  fn test_filtered_stateful_table_apply_search() {
    let mut stateful_table: StatefulTable<&str> = StatefulTable::default();
    stateful_table.set_filtered_items(vec!["this", "is", "a", "test"]);
    stateful_table.search = Some("test".into());
    let mut expected_state = TableState::default();
    expected_state.select(Some(3));

    let has_match = stateful_table.apply_search(|&item| item);

    assert_eq!(stateful_table.search, None);
    assert_eq!(stateful_table.filtered_state, Some(expected_state));
    assert!(has_match);
  }

  #[test]
  fn test_filtered_stateful_table_apply_search_no_match() {
    let mut stateful_table: StatefulTable<&str> = StatefulTable::default();
    stateful_table.set_filtered_items(vec!["this", "is", "a", "test"]);
    stateful_table.search = Some("shi-mon-a!".into());
    let mut expected_state = TableState::default();
    expected_state.select(Some(0));

    let has_match = stateful_table.apply_search(|&item| item);

    assert_eq!(stateful_table.search, None);
    assert_eq!(stateful_table.filtered_state, Some(expected_state));
    assert!(!has_match);
  }

  #[test]
  fn test_stateful_table_reset_search() {
    let mut stateful_table = create_test_stateful_table();
    stateful_table.search = Some("test".into());
    stateful_table.reset_search();

    assert_eq!(stateful_table.search, None);
  }

  fn create_test_stateful_table() -> StatefulTable<&'static str> {
    let mut stateful_table = StatefulTable::default();
    stateful_table.set_items(vec!["Test 1", "Test 2"]);

    stateful_table
  }

  fn create_test_filtered_stateful_table() -> StatefulTable<&'static str> {
    let mut stateful_table = StatefulTable::default();
    stateful_table.set_filtered_items(vec!["Test 1", "Test 2"]);

    stateful_table
  }
}
