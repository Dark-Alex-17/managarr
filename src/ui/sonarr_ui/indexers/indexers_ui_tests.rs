#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, EDIT_INDEXER_BLOCKS, INDEXER_SETTINGS_BLOCKS, INDEXERS_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::sonarr_ui::indexers::IndexersUi;

  #[test]
  fn test_indexers_ui_accepts() {
    let mut indexers_blocks = Vec::new();
    indexers_blocks.extend(INDEXERS_BLOCKS);
    indexers_blocks.extend(INDEXER_SETTINGS_BLOCKS);
    indexers_blocks.extend(EDIT_INDEXER_BLOCKS);
    indexers_blocks.push(ActiveSonarrBlock::TestAllIndexers);

    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if indexers_blocks.contains(&active_sonarr_block) {
        assert!(IndexersUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!IndexersUi::accepts(active_sonarr_block.into()));
      }
    });
  }
}
