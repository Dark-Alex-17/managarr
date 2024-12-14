#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, DELETE_SERIES_BLOCKS};
  use crate::ui::sonarr_ui::library::delete_series_ui::DeleteSeriesUi;
  use crate::ui::DrawUi;

  #[test]
  fn test_delete_series_ui_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if DELETE_SERIES_BLOCKS.contains(&active_sonarr_block) {
        assert!(DeleteSeriesUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!DeleteSeriesUi::accepts(active_sonarr_block.into()));
      }
    });
  }
}
