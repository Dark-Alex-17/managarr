#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::radarr::radarr_data::{
    ADD_MOVIE_BLOCKS, ActiveRadarrBlock, DELETE_MOVIE_BLOCKS, EDIT_MOVIE_BLOCKS, LIBRARY_BLOCKS,
    MOVIE_DETAILS_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::library::LibraryUi;
  use crate::ui::ui_test_utils::test_utils::{TerminalSize, render_to_string_with_app};

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

  mod snapshot_tests {
    use super::*;
    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::radarr::radarr_data::{
      ADD_MOVIE_SELECTION_BLOCKS, EDIT_MOVIE_SELECTION_BLOCKS,
    };
    use rstest::rstest;

    #[test]
    fn test_library_ui_renders_library_tab_loading() {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        LibraryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_library_ui_renders_library_tab_empty_movies() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        LibraryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    fn test_library_ui_renders_library_tab(
      #[values(
        ActiveRadarrBlock::Movies,
        ActiveRadarrBlock::MoviesSortPrompt,
        ActiveRadarrBlock::SearchMovie,
        ActiveRadarrBlock::SearchMovieError,
        ActiveRadarrBlock::FilterMovies,
        ActiveRadarrBlock::FilterMoviesError,
        ActiveRadarrBlock::UpdateAllMoviesPrompt
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_radarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        LibraryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(active_radarr_block.to_string(), output);
    }

    #[rstest]
    fn test_library_movie_ui_renders_add_movie_ui(
      #[values(
        ActiveRadarrBlock::AddMovieSearchInput,
        ActiveRadarrBlock::AddMovieSearchResults,
        ActiveRadarrBlock::AddMovieEmptySearchResults,
        ActiveRadarrBlock::AddMoviePrompt,
        ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
        ActiveRadarrBlock::AddMovieSelectMonitor,
        ActiveRadarrBlock::AddMovieSelectQualityProfile,
        ActiveRadarrBlock::AddMovieSelectRootFolder,
        ActiveRadarrBlock::AddMovieAlreadyInLibrary,
        ActiveRadarrBlock::AddMovieTagsInput
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_radarr_block.into());
      app.data.radarr_data.selected_block = BlockSelectionState::new(ADD_MOVIE_SELECTION_BLOCKS);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        LibraryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(active_radarr_block.to_string(), output);
    }

    #[test]
    fn test_edit_movie_ui_renders_edit_movie_modal() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveRadarrBlock::EditMoviePrompt.into());
      app.data.radarr_data.selected_block = BlockSelectionState::new(EDIT_MOVIE_SELECTION_BLOCKS);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        LibraryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
