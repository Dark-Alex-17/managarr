#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::radarr_data::{ActiveRadarrBlock, COLLECTION_DETAILS_BLOCKS};
  use crate::ui::radarr_ui::collections::collection_details_ui::CollectionDetailsUi;
  use crate::ui::DrawUi;

  #[test]
  fn test_collection_details_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if COLLECTION_DETAILS_BLOCKS.contains(&active_radarr_block) {
        assert!(CollectionDetailsUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!CollectionDetailsUi::accepts(active_radarr_block.into()));
      }
    });
  }
}
