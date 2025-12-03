#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::models::servarr_data::radarr::radarr_data::{
    ADD_MOVIE_BLOCKS, ActiveRadarrBlock, DELETE_MOVIE_BLOCKS, EDIT_MOVIE_BLOCKS, LIBRARY_BLOCKS,
    MOVIE_DETAILS_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::library::LibraryUi;

  #[test]
  fn test_library_ui_accepts() {
    let mut library_ui_blocks = Vec::new();
    library_ui_blocks.extend(LIBRARY_BLOCKS);
    library_ui_blocks.extend(MOVIE_DETAILS_BLOCKS);
    library_ui_blocks.extend(ADD_MOVIE_BLOCKS);
    library_ui_blocks.extend(EDIT_MOVIE_BLOCKS);
    library_ui_blocks.extend(DELETE_MOVIE_BLOCKS);

    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if library_ui_blocks.contains(&active_radarr_block) {
        assert!(LibraryUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!LibraryUi::accepts(active_radarr_block.into()));
      }
    });
  }
}
