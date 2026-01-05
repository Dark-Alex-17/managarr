#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, LIBRARY_BLOCKS};
  use crate::models::Route;
  use crate::ui::DrawUi;
  use crate::ui::lidarr_ui::library::LibraryUi;

  #[test]
  fn test_library_ui_accepts() {
    for lidarr_block in ActiveLidarrBlock::iter() {
      if LIBRARY_BLOCKS.contains(&lidarr_block) {
        assert!(LibraryUi::accepts(Route::Lidarr(lidarr_block, None)));
      } else {
        assert!(!LibraryUi::accepts(Route::Lidarr(lidarr_block, None)));
      }
    }
  }
}
