#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, ROOT_FOLDERS_BLOCKS};
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::root_folders::RootFoldersUi;

  #[test]
  fn test_root_folders_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if ROOT_FOLDERS_BLOCKS.contains(&active_radarr_block) {
        assert!(RootFoldersUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!RootFoldersUi::accepts(active_radarr_block.into()));
      }
    });
  }
}
