#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::Route;
  use crate::models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock;
  use crate::ui::DrawUi;
  use crate::ui::lidarr_ui::LidarrUi;

  #[test]
  fn test_lidarr_ui_accepts() {
    for lidarr_block in ActiveLidarrBlock::iter() {
      assert!(LidarrUi::accepts(Route::Lidarr(lidarr_block, None)));
    }
  }
}
