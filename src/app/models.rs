use std::cell::RefCell;
use std::fmt::{Display, Formatter};

use serde::Deserialize;
use tui::widgets::TableState;

use crate::app::Route;

pub trait Scrollable {
  fn scroll_down(&mut self);
  fn scroll_up(&mut self);
}

pub struct StatefulTable<T> {
  pub state: TableState,
  pub items: Vec<T>,
}

impl<T> Default for StatefulTable<T> {
  fn default() -> StatefulTable<T> {
    StatefulTable {
      state: TableState::default(),
      items: Vec::new(),
    }
  }
}

impl<T: Clone> StatefulTable<T> {
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

  pub fn current_selection_clone(&self) -> T {
    self.items[self.state.selected().unwrap_or(0)].clone()
  }
}

impl<T> Scrollable for StatefulTable<T> {
  fn scroll_down(&mut self) {
    let selected_row = match self.state.selected() {
      Some(i) => {
        if i >= self.items.len() - 1 {
          0
        } else {
          i + 1
        }
      }
      None => 0,
    };

    self.state.select(Some(selected_row));
  }

  fn scroll_up(&mut self) {
    let selected_row = match self.state.selected() {
      Some(i) => {
        if i == 0 {
          self.items.len() - 1
        } else {
          i - 1
        }
      }
      None => 0,
    };

    self.state.select(Some(selected_row));
  }
}

#[derive(Default)]
pub struct ScrollableText {
  pub items: Vec<String>,
  pub offset: u16,
}

impl ScrollableText {
  pub fn with_string(item: String) -> ScrollableText {
    let items: Vec<&str> = item.split('\n').collect();
    let items: Vec<String> = items.iter().map(|it| it.to_string()).collect();
    ScrollableText { items, offset: 0 }
  }

  pub fn get_text(&self) -> String {
    self.items.join("\n")
  }
}

impl Scrollable for ScrollableText {
  fn scroll_down(&mut self) {
    if self.offset < self.items.len() as u16 {
      self.offset += 1;
    }
  }

  fn scroll_up(&mut self) {
    if self.offset > 0 {
      self.offset -= 1;
    }
  }
}

#[derive(Default, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(from = "String")]
pub struct HorizontallyScrollableText {
  pub text: String,
  pub offset: RefCell<usize>,
}

impl From<String> for HorizontallyScrollableText {
  fn from(input: String) -> HorizontallyScrollableText {
    HorizontallyScrollableText::new(input)
  }
}

impl Display for HorizontallyScrollableText {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if *self.offset.borrow() == 0 {
      write!(f, "{}", self.text)
    } else {
      write!(f, "{}", &self.text[*self.offset.borrow()..])
    }
  }
}

impl HorizontallyScrollableText {
  pub fn new(input: String) -> HorizontallyScrollableText {
    HorizontallyScrollableText {
      text: format!("{}        ", input),
      offset: RefCell::new(0),
    }
  }

  pub fn scroll_text(&self) {
    let new_offset = *self.offset.borrow() + 1;
    *self.offset.borrow_mut() = new_offset % self.text.len();
  }

  pub fn reset_offset(&self) {
    *self.offset.borrow_mut() = 0;
  }
}

#[derive(Clone)]
pub struct TabRoute {
  pub title: String,
  pub route: Route,
}

pub struct TabState {
  pub tabs: Vec<TabRoute>,
  pub index: usize,
}

impl TabState {
  pub fn new(tabs: Vec<TabRoute>) -> TabState {
    TabState { tabs, index: 0 }
  }

  pub fn set_index(&mut self, index: usize) -> &TabRoute {
    self.index = index;
    &self.tabs[self.index]
  }

  pub fn get_active_route(&self) -> &Route {
    &self.tabs[self.index].route
  }

  pub fn next(&mut self) {
    self.index = (self.index + 1) % self.tabs.len();
  }

  pub fn previous(&mut self) {
    if self.index > 0 {
      self.index -= 1;
    } else {
      self.index = self.tabs.len() - 1;
    }
  }
}
