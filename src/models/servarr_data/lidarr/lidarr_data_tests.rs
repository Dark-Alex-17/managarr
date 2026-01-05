#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;

  use crate::models::{servarr_data::lidarr::lidarr_data::ActiveLidarrBlock, Route};

  #[test]
  fn test_from_active_lidarr_block_to_route() {
    assert_eq!(
      Route::from(ActiveLidarrBlock::Artists),
      Route::Lidarr(ActiveLidarrBlock::Artists, None)
    );
  }

  #[test]
  fn test_from_tuple_to_route_with_context() {
    assert_eq!(
      Route::from((ActiveLidarrBlock::Artists, Some(ActiveLidarrBlock::Artists))),
      Route::Lidarr(ActiveLidarrBlock::Artists, Some(ActiveLidarrBlock::Artists),)
    );
  }
}
