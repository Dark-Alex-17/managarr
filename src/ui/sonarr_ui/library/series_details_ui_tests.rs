#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, EPISODE_DETAILS_BLOCKS, SEASON_DETAILS_BLOCKS, SERIES_DETAILS_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::sonarr_ui::library::series_details_ui::SeriesDetailsUi;

  #[test]
  fn test_series_details_ui_accepts() {
    let mut blocks = SERIES_DETAILS_BLOCKS.clone().to_vec();
    blocks.extend(SEASON_DETAILS_BLOCKS);
    blocks.extend(EPISODE_DETAILS_BLOCKS);

    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if blocks.contains(&active_sonarr_block) {
        assert!(SeriesDetailsUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!SeriesDetailsUi::accepts(active_sonarr_block.into()));
      }
    });
  }
}
