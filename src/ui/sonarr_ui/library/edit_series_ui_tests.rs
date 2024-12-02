#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, EDIT_SERIES_BLOCKS};
  use crate::ui::sonarr_ui::library::edit_series_ui::EditSeriesUi;
  use crate::ui::DrawUi;

  #[test]
  fn test_edit_movie_ui_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if EDIT_SERIES_BLOCKS.contains(&active_sonarr_block) {
        assert!(EditSeriesUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!EditSeriesUi::accepts(active_sonarr_block.into()));
      }
    });
  }
}
