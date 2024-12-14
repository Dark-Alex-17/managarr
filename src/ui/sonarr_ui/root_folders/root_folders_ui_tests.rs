#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, ROOT_FOLDERS_BLOCKS};
  use crate::ui::sonarr_ui::root_folders::RootFoldersUi;
  use crate::ui::DrawUi;

  #[test]
  fn test_root_folders_ui_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if ROOT_FOLDERS_BLOCKS.contains(&active_sonarr_block) {
        assert!(RootFoldersUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!RootFoldersUi::accepts(active_sonarr_block.into()));
      }
    });
  }
}
