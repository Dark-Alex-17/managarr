use crate::models::Scrollable;
use ratatui::widgets::ListState;
use std::fmt::Debug;

#[cfg(test)]
#[path = "stateful_list_tests.rs"]
mod stateful_list_tests;

#[derive(Default)]
#[cfg_attr(test, derive(Debug))]
pub struct StatefulList<T> {
  pub state: ListState,
  pub items: Vec<T>,
}

impl<T> Scrollable for StatefulList<T> {
  fn scroll_down(&mut self) {
    if self.items.is_empty() {
      return;
    }

    match self.state.selected() {
      Some(i) => {
        if i >= self.items.len() - 1 {
          self.state.select_first();
        } else {
          self.state.select_next();
        }
      }
      None => self.state.select_first(),
    };
  }

  fn scroll_up(&mut self) {
    if self.items.is_empty() {
      return;
    }

    match self.state.selected() {
      Some(i) => {
        if i == 0 {
          self.state.select(Some(self.items.len() - 1));
        } else {
          self.state.select_previous();
        }
      }
      None => self.state.select_first(),
    };
  }

  fn scroll_to_top(&mut self) {
    if self.items.is_empty() {
      return;
    }

    self.state.select_first();
  }

  fn scroll_to_bottom(&mut self) {
    if self.items.is_empty() {
      return;
    }

    self.state.select(Some(self.items.len() - 1));
  }
}

impl<T> StatefulList<T>
where
  T: Clone + PartialEq + Eq + Debug,
{
  pub fn set_items(&mut self, items: Vec<T>) {
    let items_len = items.len();
    self.items = items;
    if !self.items.is_empty() {
      let selected_row = self.state.selected().map_or(0, |i| {
        if i > 0 && i < items_len {
          i
        } else if i >= items_len {
          items_len - 1
        } else {
          0
        }
      });
      self.state.select(Some(selected_row));
    }
  }

  pub fn current_selection(&self) -> &T {
    &self.items[self.state.selected().unwrap_or(0)]
  }

  pub fn is_empty(&self) -> bool {
    self.items.is_empty()
  }
}
