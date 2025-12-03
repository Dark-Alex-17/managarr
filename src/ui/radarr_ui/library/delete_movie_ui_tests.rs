#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, DELETE_MOVIE_BLOCKS};
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::library::delete_movie_ui::DeleteMovieUi;

  #[test]
  fn test_delete_movie_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if DELETE_MOVIE_BLOCKS.contains(&active_radarr_block) {
        assert!(DeleteMovieUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!DeleteMovieUi::accepts(active_radarr_block.into()));
      }
    });
  }
}
