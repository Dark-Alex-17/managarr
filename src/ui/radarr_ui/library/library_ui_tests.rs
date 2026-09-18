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

  mod test_movie_row_styling {
    use pretty_assertions::assert_eq;
    use ratatui::style::Style;

    use crate::models::radarr_models::Movie;
    use crate::network::radarr_network::radarr_network_test_utils::test_utils::movie;
    use crate::ui::styles::{downloading_style, missing_style};
    use crate::ui::ui_test_utils::test_utils::create_test_terminal;

    use super::*;

    #[test]
    fn test_library_ui_renders_downloading_movie_with_downloading_style() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      set_movies_with_unselected_probe(&mut app);

      let style = rendered_row_style(&mut app, "Unselected movie");

      assert_eq!(style.fg, downloading_style().fg);
    }

    #[test]
    fn test_library_ui_renders_missing_movie_with_missing_style_when_queue_is_empty() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveRadarrBlock::Movies.into());
      app.data.radarr_data.downloads.set_items(vec![]);
      set_movies_with_unselected_probe(&mut app);

      let style = rendered_row_style(&mut app, "Unselected movie");

      assert_eq!(style.fg, missing_style().fg);
    }

    fn set_movies_with_unselected_probe(app: &mut App<'_>) {
      app.data.radarr_data.movies.set_items(vec![
        movie(),
        Movie {
          has_file: false,
          monitored: true,
          title: "Unselected movie".into(),
          ..movie()
        },
      ]);
    }

    fn rendered_row_style(app: &mut App<'_>, needle: &str) -> Style {
      let (width, height) = TerminalSize::Large.to_cartesian();
      let mut terminal = create_test_terminal(width, height);

      terminal
        .draw(|f| {
          LibraryUi::draw(f, app, f.area());
        })
        .unwrap();

      let buffer = terminal.backend().buffer();

      for y in 0..height {
        let row = (0..width)
          .map(|x| buffer.cell((x, y)).expect("a rendered cell").symbol())
          .collect::<String>();

        if let Some(byte_index) = row.find(needle) {
          let column = row[..byte_index].chars().count() as u16;

          return buffer.cell((column, y)).expect("a rendered cell").style();
        }
      }

      panic!("no rendered row contained {needle}");
    }
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

      insta::assert_snapshot!(format!("library_tab_{active_radarr_block}"), output);
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

      insta::assert_snapshot!(format!("add_movie_ui_{active_radarr_block}"), output);
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
