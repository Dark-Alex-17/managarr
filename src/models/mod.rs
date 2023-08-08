use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter};

use serde::Deserialize;
use tui::widgets::{ListState, TableState};

use crate::app::radarr::ActiveRadarrBlock;

pub mod radarr_models;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Route {
  Radarr(ActiveRadarrBlock, Option<ActiveRadarrBlock>),
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
    #[derive(Default)]
    pub struct $name<T> {
      pub state: $state,
      pub items: Vec<T>,
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
    if self.offset < (self.items.len() - 1) as u16 {
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
    if is_current_selection && self.text.len().saturating_sub(4) > width {
      self.scroll_text();
    } else {
      self.reset_offset();
    }
  }

  pub fn stationary_style(&self) -> String {
    self.text.clone().trim().to_owned()
  }
}

#[derive(Clone, PartialEq, Eq, Debug)]
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

#[cfg(test)]
mod tests {
  use std::cell::RefCell;

  use pretty_assertions::{assert_eq, assert_str_eq};

  use crate::app::radarr::ActiveRadarrBlock;
  use crate::models::{
    HorizontallyScrollableText, Scrollable, ScrollableText, StatefulTable, TabRoute, TabState,
  };

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
  fn test_stateful_table_current_selection_clone() {
    let mut stateful_table = create_test_stateful_table();

    assert_str_eq!(
      stateful_table.current_selection_clone(),
      stateful_table.items[0]
    );

    stateful_table.state.select(Some(1));

    assert_str_eq!(
      stateful_table.current_selection_clone(),
      stateful_table.items[1]
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
  fn test_stateful_table_scroll_up() {
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
  fn test_horizontally_scrollable_text_from() {
    let test_text = "Test string";
    let horizontally_scrollable_text = HorizontallyScrollableText::from(test_text.to_owned());

    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 0);
    assert_str_eq!(
      horizontally_scrollable_text.text,
      format!("{}        ", test_text)
    );
  }

  #[test]
  fn test_horizontally_scrollable_text_to_string() {
    let test_text = "Test string";
    let horizontally_scrollable_text = HorizontallyScrollableText::from(test_text.to_owned());

    assert_str_eq!(
      horizontally_scrollable_text.to_string(),
      format!("{}        ", test_text)
    );

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
    assert_str_eq!(
      horizontally_scrollable_text.text,
      format!("{}        ", test_text)
    );
  }

  #[test]
  fn test_horizontally_scrollable_text_scroll_text() {
    let horizontally_scrollable_text = HorizontallyScrollableText::from("Test string".to_owned());

    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 0);

    for i in 1..horizontally_scrollable_text.text.len() {
      horizontally_scrollable_text.scroll_text();

      assert_eq!(*horizontally_scrollable_text.offset.borrow(), i);
    }

    horizontally_scrollable_text.scroll_text();

    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 0);
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
  fn test_horizontally_scrollable_text_scroll_or_reset() {
    let width = 3;
    let test_text = "Test string";
    let horizontally_scrollable_text = HorizontallyScrollableText::from(test_text.to_owned());

    horizontally_scrollable_text.scroll_or_reset(width, true);

    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 1);

    horizontally_scrollable_text.scroll_or_reset(width, false);

    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 0);

    horizontally_scrollable_text.scroll_or_reset(width, true);

    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 1);

    horizontally_scrollable_text.scroll_or_reset(test_text.len(), false);

    assert_eq!(*horizontally_scrollable_text.offset.borrow(), 0);
  }

  #[test]
  fn test_horizontally_scrollable_text_stationary_style() {
    let test_text = "Test string";
    let horizontally_scrollable_text = HorizontallyScrollableText::from(test_text.to_owned());

    assert_eq!(horizontally_scrollable_text.stationary_style(), test_text);
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
    let second_tab = create_test_tab_routes()[1].clone().route;
    let tab_state = TabState {
      tabs: create_test_tab_routes(),
      index: 1,
    };

    let active_route = tab_state.get_active_route();

    assert_eq!(active_route, &second_tab);
  }

  #[test]
  fn test_tab_state_get_active_tab_help() {
    let second_tab_help = create_test_tab_routes()[1].clone().help;
    let tab_state = TabState {
      tabs: create_test_tab_routes(),
      index: 1,
    };

    let tab_help = tab_state.get_active_tab_help();

    assert_str_eq!(tab_help, second_tab_help);
  }

  #[test]
  fn test_tab_state_get_active_tab_contextual_help() {
    let second_tab_contextual_help = create_test_tab_routes()[1].clone().contextual_help.unwrap();
    let tab_state = TabState {
      tabs: create_test_tab_routes(),
      index: 1,
    };

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

  fn create_test_tab_routes() -> Vec<TabRoute> {
    vec![
      TabRoute {
        title: "Test 1".to_owned(),
        route: ActiveRadarrBlock::Movies.into(),
        help: "Help for Test 1".to_owned(),
        contextual_help: Some("Contextual Help for Test 1".to_owned()),
      },
      TabRoute {
        title: "Test 2".to_owned(),
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
}
