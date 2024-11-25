use crate::models::HorizontallyScrollableText;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct EditIndexerModal {
  pub name: HorizontallyScrollableText,
  pub enable_rss: Option<bool>,
  pub enable_automatic_search: Option<bool>,
  pub enable_interactive_search: Option<bool>,
  pub url: HorizontallyScrollableText,
  pub api_key: HorizontallyScrollableText,
  pub seed_ratio: HorizontallyScrollableText,
  pub tags: HorizontallyScrollableText,
}

#[derive(Default, Clone, Eq, PartialEq, Debug)]
pub struct IndexerTestResultModalItem {
  pub name: String,
  pub is_valid: bool,
  pub validation_failures: HorizontallyScrollableText,
}
