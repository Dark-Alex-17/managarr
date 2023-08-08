#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::radarr::radarr_data::{
    ActiveRadarrBlock, EDIT_COLLECTION_BLOCKS,
  };
  use crate::ui::radarr_ui::collections::edit_collection_ui::EditCollectionUi;
  use crate::ui::DrawUi;

  #[test]
  fn test_edit_collection_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if EDIT_COLLECTION_BLOCKS.contains(&active_radarr_block) {
        assert!(EditCollectionUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!EditCollectionUi::accepts(active_radarr_block.into()));
      }
    })
  }
}
