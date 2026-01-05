#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::handlers::KeyEventHandler;
  use crate::handlers::lidarr_handlers::LidarrHandler;
  use crate::models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock;

  #[test]
  fn test_lidarr_handler_accepts() {
    for lidarr_block in ActiveLidarrBlock::iter() {
      assert!(LidarrHandler::accepts(lidarr_block));
    }
  }
}
