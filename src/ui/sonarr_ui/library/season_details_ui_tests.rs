#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, EPISODE_DETAILS_BLOCKS, SEASON_DETAILS_BLOCKS,
  };
  use crate::ui::sonarr_ui::library::season_details_ui::SeasonDetailsUi;
  use crate::ui::DrawUi;

  #[test]
  fn test_season_details_ui_accepts() {
    let mut blocks = SEASON_DETAILS_BLOCKS.clone().to_vec();
    blocks.extend(EPISODE_DETAILS_BLOCKS);

    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if blocks.contains(&active_sonarr_block) {
        assert!(SeasonDetailsUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!SeasonDetailsUi::accepts(active_sonarr_block.into()));
      }
    });
  }
}
