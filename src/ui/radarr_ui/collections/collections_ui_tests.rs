#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::radarr::radarr_data::{
    ActiveRadarrBlock, COLLECTION_DETAILS_BLOCKS, COLLECTIONS_BLOCKS, EDIT_COLLECTION_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::collections::CollectionsUi;

  #[test]
  fn test_collections_ui_accepts() {
    let mut collections_ui_blocks = Vec::new();
    collections_ui_blocks.extend(COLLECTIONS_BLOCKS);
    collections_ui_blocks.extend(COLLECTION_DETAILS_BLOCKS);
    collections_ui_blocks.extend(EDIT_COLLECTION_BLOCKS);

    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if collections_ui_blocks.contains(&active_radarr_block) {
        assert!(CollectionsUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!CollectionsUi::accepts(active_radarr_block.into()));
      }
    });
  }
}
