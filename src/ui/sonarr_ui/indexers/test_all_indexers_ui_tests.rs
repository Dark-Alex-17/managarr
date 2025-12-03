#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock;
  use crate::ui::DrawUi;
  use crate::ui::sonarr_ui::indexers::test_all_indexers_ui::TestAllIndexersUi;

  #[test]
  fn test_test_all_indexers_ui_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if active_sonarr_block == ActiveSonarrBlock::TestAllIndexers {
        assert!(TestAllIndexersUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!TestAllIndexersUi::accepts(active_sonarr_block.into()));
      }
    });
  }
}
