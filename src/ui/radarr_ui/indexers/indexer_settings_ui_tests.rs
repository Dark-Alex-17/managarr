#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::radarr_data::{ActiveRadarrBlock, INDEXER_SETTINGS_BLOCKS};
  use crate::ui::radarr_ui::indexers::indexer_settings_ui::IndexerSettingsUi;
  use crate::ui::DrawUi;

  #[test]
  fn test_indexer_settings_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if INDEXER_SETTINGS_BLOCKS.contains(&active_radarr_block) {
        assert!(IndexerSettingsUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!IndexerSettingsUi::accepts(active_radarr_block.into()));
      }
    });
  }
}
