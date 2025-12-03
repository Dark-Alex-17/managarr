#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, SYSTEM_DETAILS_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::sonarr_ui::system::system_details_ui::SystemDetailsUi;

  #[test]
  fn test_system_details_ui_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if SYSTEM_DETAILS_BLOCKS.contains(&active_sonarr_block) {
        assert!(SystemDetailsUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!SystemDetailsUi::accepts(active_sonarr_block.into()));
      }
    });
  }
}
