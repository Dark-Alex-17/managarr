use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter};

use serde::Deserialize;
use tui::widgets::{ListState, TableState};

use crate::app::radarr::ActiveRadarrBlock;

pub mod radarr_models;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Route {
  Radarr(ActiveRadarrBlock),
  Sonarr,
  Readarr,
  Lidarr,
  Whisparr,
  Bazarr,
  Prowlarr,
  Tautulli,
}

pub trait Scrollable {
  fn scroll_down(&mut self);
  fn scroll_up(&mut self);
  fn scroll_to_top(&mut self);
  fn scroll_to_bottom(&mut self);
}

macro_rules! stateful_iterable {
  ($name:ident, $state:ty) => {
    pub struct $name<T> {
      pub state: $state,
      pub items: Vec<T>,
    }

    impl<T> Default for $name<T> {
      fn default() -> $name<T> {
        $name {
          state: <$state>::default(),
          items: Vec::new(),
        }
      }
    }

    impl<T> Scrollable for $name<T> {
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

      fn scroll_to_top(&mut self) {
        self.state.select(Some(0));
      }

      fn scroll_to_bottom(&mut self) {
        self.state.select(Some(self.items.len() - 1));
      }
    }

    impl<T> $name<T>
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

      pub fn current_selection_clone(&self) -> T {
        self.items[self.state.selected().unwrap_or(0)].clone()
      }
    }
  };
}

stateful_iterable!(StatefulList, ListState);
stateful_iterable!(StatefulTable, TableState);

impl<T> StatefulTable<T>
where
  T: Clone + PartialEq + Eq + Debug,
{
  pub fn select_index(&mut self, index: Option<usize>) {
    self.state.select(index);
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

  fn scroll_to_top(&mut self) {
    self.offset = 0;
  }

  fn scroll_to_bottom(&mut self) {
    self.offset = (self.items.len() - 1) as u16;
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
      let text_vec = self.text.chars().collect::<Vec<_>>();
      write!(
        f,
        "{}",
        text_vec[*self.offset.borrow()..]
          .iter()
          .cloned()
          .collect::<String>()
      )
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

  pub fn scroll_or_reset(&self, width: usize, is_current_selection: bool) {
    if is_current_selection && self.text.len() > width {
      self.scroll_text();
    } else {
      self.reset_offset();
    }
  }

  pub fn stationary_style(&self) -> String {
    self.text.clone().trim().to_owned()
  }
}

#[derive(Clone)]
pub struct TabRoute {
  pub title: String,
  pub route: Route,
  pub help: String,
  pub contextual_help: Option<String>,
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

  pub fn get_active_tab_help(&self) -> String {
    self.tabs[self.index].help.clone()
  }

  pub fn get_active_tab_contextual_help(&self) -> Option<String> {
    self.tabs[self.index].contextual_help.clone()
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
