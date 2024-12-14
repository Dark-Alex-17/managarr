#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, DOWNLOADS_BLOCKS};
  use crate::ui::sonarr_ui::downloads::DownloadsUi;
  use crate::ui::DrawUi;

  #[test]
  fn test_downloads_ui_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if DOWNLOADS_BLOCKS.contains(&active_sonarr_block) {
        assert!(DownloadsUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!DownloadsUi::accepts(active_sonarr_block.into()));
      }
    });
  }
}
