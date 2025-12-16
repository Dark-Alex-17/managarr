#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::radarr::radarr_data::{ADD_MOVIE_BLOCKS, ActiveRadarrBlock};
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::library::add_movie_ui::AddMovieUi;
  use crate::ui::ui_test_utils::test_utils::{TerminalSize, render_to_string_with_app};

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

  mod snapshot_tests {
    use super::*;
    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::radarr::radarr_data::ADD_MOVIE_SELECTION_BLOCKS;
    use rstest::rstest;

    #[test]
    fn test_add_movie_ui_renders_loading_for_search() {
      let mut app = App::test_default_fully_populated();
      app.data.radarr_data.add_searched_movies = None;
      app.push_navigation_stack(ActiveRadarrBlock::AddMovieSearchResults.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        AddMovieUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    #[case(ActiveRadarrBlock::AddMovieSearchInput, None)]
    #[case(ActiveRadarrBlock::AddMovieSearchResults, None)]
    #[case(ActiveRadarrBlock::AddMovieEmptySearchResults, None)]
    #[case(ActiveRadarrBlock::AddMoviePrompt, None)]
    #[case(ActiveRadarrBlock::AddMovieSelectMinimumAvailability, None)]
    #[case(ActiveRadarrBlock::AddMovieSelectMonitor, None)]
    #[case(ActiveRadarrBlock::AddMovieSelectQualityProfile, None)]
    #[case(ActiveRadarrBlock::AddMovieSelectRootFolder, None)]
    #[case(ActiveRadarrBlock::AddMovieAlreadyInLibrary, None)]
    #[case(ActiveRadarrBlock::AddMovieTagsInput, None)]
    #[case(
      ActiveRadarrBlock::AddMoviePrompt,
      Some(ActiveRadarrBlock::CollectionDetails)
    )]
    #[case(
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability,
      Some(ActiveRadarrBlock::CollectionDetails)
    )]
    #[case(
      ActiveRadarrBlock::AddMovieSelectMonitor,
      Some(ActiveRadarrBlock::CollectionDetails)
    )]
    #[case(
      ActiveRadarrBlock::AddMovieSelectQualityProfile,
      Some(ActiveRadarrBlock::CollectionDetails)
    )]
    #[case(
      ActiveRadarrBlock::AddMovieSelectRootFolder,
      Some(ActiveRadarrBlock::CollectionDetails)
    )]
    #[case(
      ActiveRadarrBlock::AddMovieTagsInput,
      Some(ActiveRadarrBlock::CollectionDetails)
    )]
    fn test_add_movie_ui_renders(
      #[case] active_radarr_block: ActiveRadarrBlock,
      #[case] context: Option<ActiveRadarrBlock>,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack((active_radarr_block, context).into());
      app.data.radarr_data.selected_block = BlockSelectionState::new(ADD_MOVIE_SELECTION_BLOCKS);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        AddMovieUi::draw(f, app, f.area());
      });

      if let Some(context) = context {
        insta::assert_snapshot!(
          format!(
            "{}_{}",
            active_radarr_block.to_string(),
            context.to_string()
          ),
          output
        );
      } else {
        insta::assert_snapshot!(active_radarr_block.to_string(), output);
      }
    }
  }
}
