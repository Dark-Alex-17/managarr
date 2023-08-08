#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::radarr_data::{ActiveRadarrBlock, DOWNLOADS_BLOCKS};
  use crate::ui::radarr_ui::downloads::DownloadsUi;
  use crate::ui::DrawUi;

  #[test]
  fn test_downloads_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if DOWNLOADS_BLOCKS.contains(&active_radarr_block) {
        assert!(DownloadsUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!DownloadsUi::accepts(active_radarr_block.into()));
      }
    });
  }
}
