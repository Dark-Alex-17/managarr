#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::sonarr::sonarr_data::{ADD_SERIES_BLOCKS, ActiveSonarrBlock};
  use crate::ui::DrawUi;
  use crate::ui::sonarr_ui::library::add_series_ui::AddSeriesUi;

  #[test]
  fn test_add_series_ui_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if ADD_SERIES_BLOCKS.contains(&active_sonarr_block) {
        assert!(AddSeriesUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!AddSeriesUi::accepts(active_sonarr_block.into()));
      }
    });
  }
}
