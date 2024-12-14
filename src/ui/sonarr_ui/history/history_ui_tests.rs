#[cfg(test)]
mod tests {
  use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, HISTORY_BLOCKS};
  use crate::ui::sonarr_ui::history::HistoryUi;
  use crate::ui::DrawUi;
  use strum::IntoEnumIterator;

  #[test]
  fn test_history_ui_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if HISTORY_BLOCKS.contains(&active_sonarr_block) {
        assert!(HistoryUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!HistoryUi::accepts(active_sonarr_block.into()));
      }
    });
  }
}
