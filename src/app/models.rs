use tui::widgets::TableState;

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

impl<T> StatefulTable<T> {
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
