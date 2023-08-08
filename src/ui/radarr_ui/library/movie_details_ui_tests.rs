#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, MOVIE_DETAILS_BLOCKS};
  use crate::ui::radarr_ui::library::movie_details_ui::MovieDetailsUi;
  use crate::ui::DrawUi;

  #[test]
  fn test_movie_details_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if MOVIE_DETAILS_BLOCKS.contains(&active_radarr_block) {
        assert!(MovieDetailsUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!MovieDetailsUi::accepts(active_radarr_block.into()));
      }
    });
  }
}
