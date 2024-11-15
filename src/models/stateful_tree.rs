use managarr_tree_widget::{TreeItem, TreeState};
use ratatui::text::ToText;

use super::Scrollable;
use core::hash::Hash;
use std::fmt::{Debug, Display};

#[cfg(test)]
#[path = "stateful_tree_tests.rs"]
mod stateful_tree_tests;

#[derive(Default)]
pub struct StatefulTree<T>
where
  T: ToText + Hash + Clone + PartialEq + Eq + Debug + Default + Display + PartialEq + Eq,
{
  pub state: TreeState,
  pub items: Vec<TreeItem<T>>,
}

impl<T> StatefulTree<T>
where
  T: ToText + Hash + Clone + PartialEq + Eq + Debug + Default + Display + PartialEq + Eq,
{
  pub fn set_items(&mut self, items: Vec<TreeItem<T>>) {
    self.items = items;
  }

  pub fn current_selection(&self) -> Option<&T> {
    self
      .state
      .flatten(&self.items)
      .into_iter()
      .find(|i| self.state.selected() == i.identifier)
      .map(|item| item.item.content())
  }

  pub fn is_empty(&self) -> bool {
    self.items.is_empty()
  }
}

impl<T> Scrollable for StatefulTree<T>
where
  T: ToText + Hash + Clone + PartialEq + Eq + Debug + Default + Display,
{
  fn scroll_down(&mut self) {
    self.state.key_down();
  }

  fn scroll_up(&mut self) {
    self.state.key_up();
  }

  fn scroll_to_top(&mut self) {
    self.state.select_first();
  }

  fn scroll_to_bottom(&mut self) {
    self.state.select_last();
  }
}
