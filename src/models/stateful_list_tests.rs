#[cfg(test)]
mod tests {
  use crate::models::Scrollable;
  use crate::models::stateful_list::StatefulList;
  use pretty_assertions::{assert_eq, assert_str_eq};

  #[test]
  fn test_stateful_list_scrolling_on_empty_list_performs_no_op() {
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
    let mut stateful_list = create_test_stateful_list();

    assert_eq!(stateful_list.state.selected(), Some(0));

    stateful_list.scroll_up();

    assert_eq!(stateful_list.state.selected(), Some(1));

    stateful_list.scroll_up();

    assert_eq!(stateful_list.state.selected(), Some(0));
  }

  #[test]
  fn test_stateful_list_is_empty() {
    let mut stateful_list = create_test_stateful_list();

    assert!(!stateful_list.is_empty());

    stateful_list = StatefulList::default();

    assert!(stateful_list.is_empty());
  }

  fn create_test_stateful_list() -> StatefulList<&'static str> {
    let mut stateful_list = StatefulList::default();
    stateful_list.set_items(vec!["Test 1", "Test 2"]);

    stateful_list
  }
}
