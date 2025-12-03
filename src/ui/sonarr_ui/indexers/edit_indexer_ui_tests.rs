#[cfg(test)]
mod tests {
  use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, EDIT_INDEXER_BLOCKS};
  use crate::ui::DrawUi;
  use crate::ui::sonarr_ui::indexers::edit_indexer_ui::EditIndexerUi;
  use strum::IntoEnumIterator;

  #[test]
  fn test_edit_indexer_ui_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if EDIT_INDEXER_BLOCKS.contains(&active_sonarr_block) {
        assert!(EditIndexerUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!EditIndexerUi::accepts(active_sonarr_block.into()));
      }
    });
  }
}
