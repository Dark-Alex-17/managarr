#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;

  use crate::models::{
    servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, LidarrData},
    Route,
  };

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

  #[test]
  fn test_reset_delete_artist_preferences() {
    let mut lidarr_data = LidarrData{
      delete_artist_files: true,
      add_import_list_exclusion: true,
      ..LidarrData::default()
    };

    lidarr_data.reset_delete_artist_preferences();

    assert!(!lidarr_data.delete_artist_files);
    assert!(!lidarr_data.add_import_list_exclusion);
  }
}
