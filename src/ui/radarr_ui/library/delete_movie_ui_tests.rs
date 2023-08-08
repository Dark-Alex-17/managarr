#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::radarr::{ActiveRadarrBlock, DELETE_MOVIE_BLOCKS};
  use crate::ui::radarr_ui::library::delete_movie_ui::DeleteMovieUi;
  use crate::ui::DrawUi;

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
