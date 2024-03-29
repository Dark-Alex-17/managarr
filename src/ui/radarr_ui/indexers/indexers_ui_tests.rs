#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::radarr::radarr_data::{
    ActiveRadarrBlock, EDIT_INDEXER_BLOCKS, INDEXERS_BLOCKS, INDEXER_SETTINGS_BLOCKS,
  };
  use crate::ui::radarr_ui::indexers::IndexersUi;
  use crate::ui::DrawUi;

  #[test]
  fn test_indexers_ui_accepts() {
    let mut indexers_blocks = Vec::new();
    indexers_blocks.extend(INDEXERS_BLOCKS);
    indexers_blocks.extend(INDEXER_SETTINGS_BLOCKS);
    indexers_blocks.extend(EDIT_INDEXER_BLOCKS);
    indexers_blocks.push(ActiveRadarrBlock::TestAllIndexers);

    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if indexers_blocks.contains(&active_radarr_block) {
        assert!(IndexersUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!IndexersUi::accepts(active_radarr_block.into()));
      }
    });
  }
}
