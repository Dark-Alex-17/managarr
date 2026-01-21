use crate::models::HorizontallyScrollableText;

#[cfg(test)]
#[path = "modals_tests.rs"]
mod modals_tests;

#[derive(Debug, PartialEq, Eq)]
pub struct EditIndexerModal {
  pub name: HorizontallyScrollableText,
  pub enable_rss: Option<bool>,
  pub enable_automatic_search: Option<bool>,
  pub enable_interactive_search: Option<bool>,
  pub url: HorizontallyScrollableText,
  pub api_key: HorizontallyScrollableText,
  pub seed_ratio: HorizontallyScrollableText,
  pub tags: HorizontallyScrollableText,
  pub priority: i64,
}

impl Default for EditIndexerModal {
  fn default() -> Self {
    Self {
      name: Default::default(),
      enable_rss: None,
      enable_automatic_search: None,
      enable_interactive_search: None,
      url: Default::default(),
      api_key: Default::default(),
      seed_ratio: Default::default(),
      tags: Default::default(),
      priority: 1,
    }
  }
}

#[derive(Default, Clone, Eq, PartialEq, Debug)]
pub struct IndexerTestResultModalItem {
  pub name: String,
  pub is_valid: bool,
  pub validation_failures: HorizontallyScrollableText,
}
