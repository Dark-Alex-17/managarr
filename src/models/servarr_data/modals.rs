use crate::models::HorizontallyScrollableText;

#[derive(Default, Clone, Eq, PartialEq, Debug)]
pub struct IndexerTestResultModalItem {
  pub name: String,
  pub is_valid: bool,
  pub validation_failures: HorizontallyScrollableText,
}
