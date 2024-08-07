use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter};

use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
use regex::Regex;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Number;

pub mod radarr_models;
pub mod servarr_data;
pub mod stateful_list;
pub mod stateful_table;

#[cfg(test)]
#[path = "model_tests.rs"]
mod model_tests;

// Allowing dead code for now since we'll eventually be implementing additional Servarr support, and we'll need it then
#[allow(dead_code)]
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

  pub fn is_empty(&self) -> bool {
    self.items.is_empty()
  }
}

impl Scrollable for ScrollableText {
  fn scroll_down(&mut self) {
    if self.items.is_empty() {
      return;
    }

    if self.offset < (self.items.len() - 1) as u16 {
      self.offset += 1;
    }
  }

  fn scroll_up(&mut self) {
    if self.items.is_empty() {
      return;
    }

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
  fn from(text: String) -> HorizontallyScrollableText {
    HorizontallyScrollableText::new(text)
  }
}

impl From<&str> for HorizontallyScrollableText {
  fn from(text: &str) -> HorizontallyScrollableText {
    HorizontallyScrollableText::new(text.to_owned())
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

impl Serialize for HorizontallyScrollableText {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(&self.text)
  }
}

impl HorizontallyScrollableText {
  pub fn new(text: String) -> HorizontallyScrollableText {
    HorizontallyScrollableText {
      text,
      offset: RefCell::new(0),
    }
  }

  pub fn len(&self) -> usize {
    self.text.chars().count()
  }

  pub fn scroll_left(&self) {
    if *self.offset.borrow() < self.len() {
      let new_offset = *self.offset.borrow() + 1;
      *self.offset.borrow_mut() = new_offset;
    }
  }

  pub fn scroll_right(&self) {
    if *self.offset.borrow() > 0 {
      let new_offset = *self.offset.borrow() - 1;
      *self.offset.borrow_mut() = new_offset;
    }
  }

  pub fn scroll_home(&self) {
    *self.offset.borrow_mut() = self.len();
  }

  pub fn reset_offset(&self) {
    *self.offset.borrow_mut() = 0;
  }

  pub fn scroll_left_or_reset(&self, width: usize, is_current_selection: bool, can_scroll: bool) {
    if can_scroll && is_current_selection && self.len() >= width {
      if *self.offset.borrow() < self.len() {
        self.scroll_left();
      } else {
        self.reset_offset();
      }
    } else if *self.offset.borrow() != 0 && !is_current_selection {
      self.reset_offset();
    }
  }

  pub fn pop(&mut self) {
    if *self.offset.borrow() < self.len() {
      let (index, _) = self
        .text
        .chars()
        .enumerate()
        .nth(self.len() - *self.offset.borrow() - 1)
        .unwrap();
      self.text = self
        .text
        .chars()
        .enumerate()
        .filter(|(idx, _)| *idx != index)
        .map(|tuple| tuple.1)
        .collect();
    }
  }

  pub fn push(&mut self, character: char) {
    if self.text.is_empty() {
      self.text.push(character);
    } else {
      let index = self.len() - *self.offset.borrow();

      if index == self.len() {
        self.text.push(character);
      } else {
        let mut new_text = String::new();
        self
          .text
          .chars()
          .collect::<Vec<char>>()
          .iter()
          .enumerate()
          .for_each(|(idx, &c)| {
            if idx == index {
              new_text.push(character);
            }

            new_text.push(c);
          });

        self.text = new_text;
      }
    }
  }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TabRoute {
  pub title: &'static str,
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

  // Allowing this code for now since we'll eventually be implementing additional Servarr support and we'll need it then
  #[allow(dead_code)]
  pub fn set_index(&mut self, index: usize) -> &TabRoute {
    self.index = index;
    &self.tabs[self.index]
  }

  pub fn get_active_route(&self) -> &Route {
    &self.tabs[self.index].route
  }

  pub fn get_active_tab_help(&self) -> &str {
    &self.tabs[self.index].help
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

#[derive(Default, Eq, PartialEq, Debug)]
pub struct BlockSelectionState<'a, T>
where
  T: Sized + Clone + Copy + Default,
{
  pub blocks: &'a [T],
  pub index: usize,
}

impl<'a, T> BlockSelectionState<'a, T>
where
  T: Sized + Clone + Copy + Default,
{
  pub fn new(blocks: &'a [T]) -> BlockSelectionState<'a, T> {
    BlockSelectionState { blocks, index: 0 }
  }

  pub fn get_active_block(&self) -> &T {
    &self.blocks[self.index]
  }

  pub fn next(&mut self) {
    self.index = (self.index + 1) % self.blocks.len();
  }

  pub fn previous(&mut self) {
    if self.index > 0 {
      self.index -= 1;
    } else {
      self.index = self.blocks.len() - 1;
    }
  }
}

#[cfg(test)]
impl<'a, T> BlockSelectionState<'a, T>
where
  T: Sized + Clone + Copy + Default,
{
  pub fn set_index(&mut self, index: usize) {
    self.index = index;
  }
}

pub fn from_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
  D: Deserializer<'de>,
{
  let num: Number = Deserialize::deserialize(deserializer)?;
  num.as_i64().ok_or(de::Error::custom(format!(
    "Unable to convert Number to i64: {num:?}"
  )))
}

pub fn strip_non_search_characters(input: &str) -> String {
  Regex::new(r"[^a-zA-Z0-9.,/'\-:\s]")
    .unwrap()
    .replace_all(&input.to_lowercase(), "")
    .to_string()
}
