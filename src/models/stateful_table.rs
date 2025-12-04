use crate::models::stateful_list::StatefulList;
use crate::models::{
  HorizontallyScrollableText, Paginated, Scrollable, strip_non_search_characters,
};
use ratatui::widgets::TableState;
use std::cmp::Ordering;
use std::fmt::Debug;

#[cfg(test)]
#[path = "stateful_table_tests.rs"]
mod stateful_table_tests;

#[derive(Clone, Debug, Default)]
pub struct SortOption<T>
where
  T: Clone + PartialEq + Eq + Debug,
{
  pub name: &'static str,
  pub cmp_fn: Option<fn(&T, &T) -> Ordering>,
}

impl<T> PartialEq for SortOption<T>
where
  T: Clone + PartialEq + Eq + Debug,
{
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name
  }
}

impl<T> Eq for SortOption<T> where T: Clone + PartialEq + Eq + Debug {}

#[derive(Default)]
pub struct StatefulTable<T>
where
  T: Clone + PartialEq + Eq + Debug,
{
  pub state: TableState,
  pub items: Vec<T>,
  pub filter: Option<HorizontallyScrollableText>,
  pub search: Option<HorizontallyScrollableText>,
  pub filtered_items: Option<Vec<T>>,
  pub filtered_state: Option<TableState>,
  pub sort_asc: bool,
  pub sort: Option<StatefulList<SortOption<T>>>,
}

impl<T> Scrollable for StatefulTable<T>
where
  T: Clone + PartialEq + Eq + Debug,
{
  fn scroll_down(&mut self) {
    if let Some(filtered_items) = self.filtered_items.as_ref() {
      if filtered_items.is_empty() {
        return;
      }

      match self
        .filtered_state
        .as_ref()
        .expect("filtered_state must exist when filtered_items exists")
        .selected()
      {
        Some(i) => {
          if i >= filtered_items.len() - 1 {
            self
              .filtered_state
              .as_mut()
              .expect("filtered_state must exist when filtered_items exists")
              .select_first();
          } else {
            self
              .filtered_state
              .as_mut()
              .expect("filtered_state must exist when filtered_items exists")
              .select_next();
          }
        }
        None => self
          .filtered_state
          .as_mut()
          .expect("filtered_state must exist when filtered_items exists")
          .select_first(),
      };

      return;
    }

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
    if let Some(filtered_items) = self.filtered_items.as_ref() {
      if filtered_items.is_empty() {
        return;
      }

      match self
        .filtered_state
        .as_ref()
        .expect("filtered_state must exist when filtered_items exists")
        .selected()
      {
        Some(i) => {
          if i == 0 {
            self
              .filtered_state
              .as_mut()
              .expect("filtered_state must exist when filtered_items exists")
              .select(Some(filtered_items.len() - 1));
          } else {
            self
              .filtered_state
              .as_mut()
              .expect("filtered_state must exist when filtered_items exists")
              .select_previous();
          }
        }
        None => self
          .filtered_state
          .as_mut()
          .expect("filtered_state must exist when filtered_items exists")
          .select_first(),
      };

      return;
    }

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
    if let Some(filtered_items) = self.filtered_items.as_ref() {
      if filtered_items.is_empty() {
        return;
      }

      self
        .filtered_state
        .as_mut()
        .expect("filtered_state must exist when filtered_items exists")
        .select_first();
      return;
    }

    if self.items.is_empty() {
      return;
    }

    self.state.select_first();
  }

  fn scroll_to_bottom(&mut self) {
    if let Some(filtered_items) = self.filtered_items.as_ref() {
      if filtered_items.is_empty() {
        return;
      }

      self
        .filtered_state
        .as_mut()
        .expect("filtered_state must exist when filtered_items exists")
        .select(Some(filtered_items.len() - 1));
      return;
    }

    if self.items.is_empty() {
      return;
    }

    self.state.select(Some(self.items.len() - 1));
  }
}

impl<T> Paginated for StatefulTable<T>
where
  T: Clone + PartialEq + Eq + Debug,
{
  fn page_down(&mut self) {
    if let Some(filtered_items) = self.filtered_items.as_ref() {
      if filtered_items.is_empty() {
        return;
      }

      match self
        .filtered_state
        .as_ref()
        .expect("filtered_state must exist when filtered_items exists")
        .selected()
      {
        Some(i) => {
          self
            .filtered_state
            .as_mut()
            .expect("filtered_state must exist when filtered_items exists")
            .select(Some(i.saturating_add(20) % (filtered_items.len() - 1)));
        }
        None => self
          .filtered_state
          .as_mut()
          .expect("filtered_state must exist when filtered_items exists")
          .select_first(),
      };

      return;
    }

    if self.items.is_empty() {
      return;
    }

    match self.state.selected() {
      Some(i) => {
        self
          .state
          .select(Some(i.saturating_add(20) % (self.items.len() - 1)));
      }
      None => self.state.select_first(),
    };
  }

  fn page_up(&mut self) {
    if let Some(filtered_items) = self.filtered_items.as_ref() {
      if filtered_items.is_empty() {
        return;
      }

      match self
        .filtered_state
        .as_ref()
        .expect("filtered_state must exist when filtered_items exists")
        .selected()
      {
        Some(i) => {
          let len = filtered_items.len() - 1;
          self
            .filtered_state
            .as_mut()
            .expect("filtered_state must exist when filtered_items exists")
            .select(Some((i + len - (20 % len)) % len));
        }
        None => self
          .filtered_state
          .as_mut()
          .expect("filtered_state must exist when filtered_items exists")
          .select_last(),
      };

      return;
    }

    if self.items.is_empty() {
      return;
    }

    match self.state.selected() {
      Some(i) => {
        let len = self.items.len() - 1;
        self.state.select(Some((i + len - (20 % len)) % len));
      }
      None => self.state.select_last(),
    };
  }
}

impl<T> StatefulTable<T>
where
  T: Clone + PartialEq + Eq + Debug + Default,
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

  pub fn set_filtered_items(&mut self, filtered_items: Vec<T>) {
    self.filtered_items = Some(filtered_items);
    let mut filtered_state: TableState = Default::default();
    filtered_state.select(Some(0));
    self.filtered_state = Some(filtered_state);
  }

  pub fn select_index(&mut self, index: Option<usize>) {
    if let Some(filtered_state) = &mut self.filtered_state {
      filtered_state.select(index);
    } else {
      self.state.select(index);
    }
  }

  pub fn current_selection(&self) -> &T {
    if let Some(filtered_items) = &self.filtered_items {
      &filtered_items[self
        .filtered_state
        .as_ref()
        .expect("filtered_state must exist when filtered_items exists")
        .selected()
        .unwrap_or(0)]
    } else {
      &self.items[self.state.selected().unwrap_or(0)]
    }
  }

  pub fn sorting(&mut self, sort_options: Vec<SortOption<T>>) {
    let mut sort_options_list = StatefulList::default();
    sort_options_list.set_items(sort_options);

    self.sort = Some(sort_options_list);
  }

  pub fn apply_sorting(&mut self) {
    self.apply_sorting_toggle(true);
  }

  pub fn apply_sorting_toggle(&mut self, toggle_dir: bool) {
    if let Some(sort_options) = &mut self.sort {
      if toggle_dir {
        self.sort_asc = !self.sort_asc;
      }
      let selected_sort_option = sort_options.current_selection();
      let mut items = self.filtered_items.as_ref().unwrap_or(&self.items).clone();
      if let Some(cmp_fn) = selected_sort_option.cmp_fn {
        if !self.sort_asc {
          items.sort_by(|a, b| cmp_fn(a, b).reverse());
        } else {
          items.sort_by(cmp_fn);
        }

        if self.filtered_items.is_some() {
          self.set_filtered_items(items);
        } else {
          self.set_items(items);
        }
      }
    }
  }

  pub fn apply_filter(&mut self, filter_field: fn(&T) -> &str) -> bool {
    let filter_matches = match self.filter.take() {
      Some(filter) if !filter.text.is_empty() => {
        let scrubbed_filter = strip_non_search_characters(&filter.text);

        self
          .items
          .iter()
          .filter(|item| strip_non_search_characters(filter_field(item)).contains(&scrubbed_filter))
          .cloned()
          .collect()
      }
      _ => Vec::new(),
    };

    if filter_matches.is_empty() {
      return false;
    }

    self.set_filtered_items(filter_matches);
    true
  }

  pub fn reset_filter(&mut self) {
    self.filter = None;
    self.filtered_items = None;
    self.filtered_state = None;
  }

  pub fn apply_search(&mut self, search_field: fn(&T) -> &str) -> bool {
    let search_index = match self.search.take() {
      Some(search) => {
        let search_string = search.text.to_lowercase();

        self
          .filtered_items
          .as_ref()
          .unwrap_or(&self.items)
          .iter()
          .position(|item| strip_non_search_characters(search_field(item)).contains(&search_string))
      }
      _ => None,
    };

    if search_index.is_none() {
      return false;
    }

    self.select_index(search_index);
    true
  }

  pub fn reset_search(&mut self) {
    self.search = None;
  }

  pub fn is_empty(&self) -> bool {
    self.items.is_empty()
  }
}
