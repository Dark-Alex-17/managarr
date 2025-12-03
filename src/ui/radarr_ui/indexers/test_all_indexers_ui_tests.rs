#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::indexers::test_all_indexers_ui::TestAllIndexersUi;

  #[test]
  fn test_test_all_indexers_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if active_radarr_block == ActiveRadarrBlock::TestAllIndexers {
        assert!(TestAllIndexersUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!TestAllIndexersUi::accepts(active_radarr_block.into()));
      }
    });
  }
}
