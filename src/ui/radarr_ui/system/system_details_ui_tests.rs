#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::radarr_data::{ActiveRadarrBlock, SYSTEM_DETAILS_BLOCKS};
  use crate::ui::radarr_ui::system::system_details_ui::SystemDetailsUi;
  use crate::ui::DrawUi;

  #[test]
  fn test_system_details_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if SYSTEM_DETAILS_BLOCKS.contains(&active_radarr_block) {
        assert!(SystemDetailsUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!SystemDetailsUi::accepts(active_radarr_block.into()));
      }
    });
  }
}
