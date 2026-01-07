#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use strum::IntoEnumIterator;

  use crate::models::Route;
  use crate::models::servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, EDIT_ARTIST_BLOCKS};
  use crate::ui::DrawUi;
  use crate::ui::lidarr_ui::library::edit_artist_ui::EditArtistUi;

  #[test]
  fn test_edit_artist_ui_accepts() {
    let mut edit_artist_ui_blocks = Vec::new();
    for block in ActiveLidarrBlock::iter() {
      if EditArtistUi::accepts(Route::Lidarr(block, None)) {
        edit_artist_ui_blocks.push(block);
      }
    }

    assert_eq!(edit_artist_ui_blocks, EDIT_ARTIST_BLOCKS.to_vec());
  }
}
