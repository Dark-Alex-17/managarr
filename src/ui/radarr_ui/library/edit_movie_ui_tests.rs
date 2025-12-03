#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, EDIT_MOVIE_BLOCKS};
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::library::edit_movie_ui::EditMovieUi;

  #[test]
  fn test_edit_movie_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if EDIT_MOVIE_BLOCKS.contains(&active_radarr_block) {
        assert!(EditMovieUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!EditMovieUi::accepts(active_radarr_block.into()));
      }
    });
  }
}
