#[cfg(test)]
mod tests {
  use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, BLOCKLIST_BLOCKS};
  use crate::ui::radarr_ui::blocklist::BlocklistUi;
  use crate::ui::DrawUi;
  use strum::IntoEnumIterator;

  #[test]
  fn test_blocklist_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if BLOCKLIST_BLOCKS.contains(&active_radarr_block) {
        assert!(BlocklistUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!BlocklistUi::accepts(active_radarr_block.into()));
      }
    });
  }
}
