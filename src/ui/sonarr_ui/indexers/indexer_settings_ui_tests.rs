#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, INDEXER_SETTINGS_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::sonarr_ui::indexers::indexer_settings_ui::IndexerSettingsUi;

  #[test]
  fn test_indexer_settings_ui_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if INDEXER_SETTINGS_BLOCKS.contains(&active_sonarr_block) {
        assert!(IndexerSettingsUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!IndexerSettingsUi::accepts(active_sonarr_block.into()));
      }
    });
  }
}
