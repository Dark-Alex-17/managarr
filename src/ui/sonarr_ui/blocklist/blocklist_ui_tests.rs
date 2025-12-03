#[cfg(test)]
mod tests {
  use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, BLOCKLIST_BLOCKS};
  use crate::ui::DrawUi;
  use crate::ui::sonarr_ui::blocklist::BlocklistUi;
  use strum::IntoEnumIterator;

  #[test]
  fn test_blocklist_ui_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if BLOCKLIST_BLOCKS.contains(&active_sonarr_block) {
        assert!(BlocklistUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!BlocklistUi::accepts(active_sonarr_block.into()));
      }
    });
  }
}
