#[cfg(test)]
mod tests {
  use std::hash::{DefaultHasher, Hash, Hasher};

  use crate::models::stateful_tree::StatefulTree;
  use managarr_tree_widget::{Tree, TreeItem, TreeState};
  use pretty_assertions::{assert_eq, assert_str_eq};
  use ratatui::buffer::Buffer;
  use ratatui::layout::Rect;
  use ratatui::widgets::StatefulWidget;
  use crate::models::Scrollable;

  #[test]
  fn test_stateful_tree_scrolling_on_empty_tree_performs_no_op() {
    let mut stateful_tree: StatefulTree<&str> = StatefulTree::default();
    render(&mut stateful_tree.state, &stateful_tree.items);
    stateful_tree.state.key_down();
    render(&mut stateful_tree.state, &stateful_tree.items);

    assert_eq!(stateful_tree.state.selected(), Vec::<u64>::new());

    stateful_tree.scroll_up();
    render(&mut stateful_tree.state, &stateful_tree.items);

    assert_eq!(stateful_tree.state.selected(), Vec::<u64>::new());

    stateful_tree.scroll_down();
    render(&mut stateful_tree.state, &stateful_tree.items);

    assert_eq!(stateful_tree.state.selected(), Vec::<u64>::new());

    stateful_tree.scroll_to_top();
    render(&mut stateful_tree.state, &stateful_tree.items);

    assert_eq!(stateful_tree.state.selected(), Vec::<u64>::new());

    stateful_tree.scroll_to_bottom();
    render(&mut stateful_tree.state, &stateful_tree.items);
  }

  #[test]
  fn test_stateful_tree_scroll() {
    let mut stateful_tree = create_test_stateful_tree();
    let hash = |s: &str| {
      let mut hasher = DefaultHasher::new();
      s.hash(&mut hasher);
      hasher.finish()
    };
    render(&mut stateful_tree.state, &stateful_tree.items);
    stateful_tree.scroll_down();
    render(&mut stateful_tree.state, &stateful_tree.items);

    assert_eq!(stateful_tree.state.selected(), &[hash("Test 1")]);

    stateful_tree.scroll_down();
    render(&mut stateful_tree.state, &stateful_tree.items);

    assert_eq!(stateful_tree.state.selected(), &[hash("Test 2")]);

    stateful_tree.scroll_down();
    render(&mut stateful_tree.state, &stateful_tree.items);

    assert_eq!(stateful_tree.state.selected(), &[hash("Test 3")]);

    stateful_tree.scroll_down();
    render(&mut stateful_tree.state, &stateful_tree.items);

    assert_eq!(stateful_tree.state.selected(), &[hash("Test 3")]);

    stateful_tree.scroll_up();
    render(&mut stateful_tree.state, &stateful_tree.items);

    assert_eq!(stateful_tree.state.selected(), &[hash("Test 2")]);

    stateful_tree.scroll_up();
    render(&mut stateful_tree.state, &stateful_tree.items);

    assert_eq!(stateful_tree.state.selected(), &[hash("Test 1")]);

    stateful_tree.scroll_to_bottom();
    render(&mut stateful_tree.state, &stateful_tree.items);

    assert_eq!(stateful_tree.state.selected(), &[hash("Test 3")]);

    stateful_tree.scroll_to_top();
    render(&mut stateful_tree.state, &stateful_tree.items);

    assert_eq!(stateful_tree.state.selected(), &[hash("Test 1")]);
  }

  #[test]
  fn test_stateful_tree_set_items() {
    let items_vec = vec![
      TreeItem::new_leaf("Test 1"),
      TreeItem::new_leaf("Test 2"),
      TreeItem::new_leaf("Test 3"),
    ];
    let hash = |s: &str| {
      let mut hasher = DefaultHasher::new();
      s.hash(&mut hasher);
      hasher.finish()
    };
    let mut stateful_tree: StatefulTree<&str> = StatefulTree::default();

    stateful_tree.set_items(items_vec.clone());
    render(&mut stateful_tree.state, &stateful_tree.items);
    stateful_tree.state.key_down();
    render(&mut stateful_tree.state, &stateful_tree.items);
  
    assert_eq!(stateful_tree.state.selected(), &[hash("Test 1")]);

    stateful_tree.state.key_down();
    render(&mut stateful_tree.state, &stateful_tree.items);
    stateful_tree.set_items(items_vec.clone());
    render(&mut stateful_tree.state, &stateful_tree.items);
  
    assert_eq!(stateful_tree.state.selected(), &[hash("Test 2")]);

    stateful_tree.state.key_down();
    render(&mut stateful_tree.state, &stateful_tree.items);
    stateful_tree.set_items(items_vec);
    render(&mut stateful_tree.state, &stateful_tree.items);
  
    assert_eq!(stateful_tree.state.selected(), &[hash("Test 3")]);
  }

  #[test]
  fn test_stateful_tree_current_selection() {
    let mut stateful_tree = create_test_stateful_tree();
    render(&mut stateful_tree.state, &stateful_tree.items);
    stateful_tree.state.key_down();
    render(&mut stateful_tree.state, &stateful_tree.items);
    
    let current_selection = stateful_tree.current_selection();

    assert!(current_selection.is_some());
    assert_str_eq!(current_selection.unwrap(), stateful_tree.items[0].content());

    stateful_tree.state.key_down();
    render(&mut stateful_tree.state, &stateful_tree.items);
    let current_selection = stateful_tree.current_selection();

    assert!(current_selection.is_some());
    assert_str_eq!(current_selection.unwrap(), stateful_tree.items[1].content());
  }

  #[test]
  fn test_stateful_tree_is_empty() {
    let mut stateful_tree = create_test_stateful_tree();
    render(&mut stateful_tree.state, &stateful_tree.items);

    assert!(!stateful_tree.is_empty());

    stateful_tree = StatefulTree::default();
    render(&mut stateful_tree.state, &stateful_tree.items);

    assert!(stateful_tree.is_empty());
  }

  fn render(state: &mut TreeState, items: &[TreeItem<&str>]) {
    let tree = Tree::new(items).unwrap();
    let area = Rect::new(0, 0, 10, 4);
    let mut buffer = Buffer::empty(area);
    StatefulWidget::render(tree, area, &mut buffer, state);
  }

  fn create_test_stateful_tree() -> StatefulTree<&'static str> {
    let mut stateful_tree = StatefulTree::default();
    stateful_tree.set_items(vec![
      TreeItem::new_leaf("Test 1"),
      TreeItem::new_leaf("Test 2"),
      TreeItem::new_leaf("Test 3"),
    ]);

    stateful_tree
  }
}
