#[cfg(test)]
mod tests {
  use crate::models::servarr_data::modals::EditIndexerModal;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_edit_indexer_modal_default() {
    let edit_indexer_modal = EditIndexerModal::default();

    assert_is_empty!(edit_indexer_modal.name.text);
    assert_none!(&edit_indexer_modal.enable_rss);
    assert_none!(&edit_indexer_modal.enable_automatic_search);
    assert_none!(&edit_indexer_modal.enable_interactive_search);
    assert_is_empty!(edit_indexer_modal.url.text);
    assert_is_empty!(edit_indexer_modal.api_key.text);
    assert_is_empty!(edit_indexer_modal.seed_ratio.text);
    assert_is_empty!(edit_indexer_modal.tags.text);
    assert_eq!(edit_indexer_modal.priority, 1);
  }
}
