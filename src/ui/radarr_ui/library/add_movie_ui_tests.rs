#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::radarr_data::{ActiveRadarrBlock, ADD_MOVIE_BLOCKS};
  use crate::ui::radarr_ui::library::add_movie_ui::AddMovieUi;
  use crate::ui::DrawUi;

  #[test]
  fn test_add_movie_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if ADD_MOVIE_BLOCKS.contains(&active_radarr_block) {
        assert!(AddMovieUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!AddMovieUi::accepts(active_radarr_block.into()));
      }
    });
  }
}
