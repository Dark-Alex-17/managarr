#[cfg(test)]
mod tests {
  use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, EDIT_INDEXER_BLOCKS};
  use crate::ui::radarr_ui::indexers::edit_indexer_ui::EditIndexerUi;
  use crate::ui::DrawUi;
  use strum::IntoEnumIterator;

  #[test]
  fn test_edit_indexer_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if EDIT_INDEXER_BLOCKS.contains(&active_radarr_block) {
        assert!(EditIndexerUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!EditIndexerUi::accepts(active_radarr_block.into()));
      }
    });
  }
}
