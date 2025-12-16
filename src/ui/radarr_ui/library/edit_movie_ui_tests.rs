#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::BlockSelectionState;
  use crate::models::servarr_data::radarr::radarr_data::{
    ActiveRadarrBlock, EDIT_MOVIE_BLOCKS, EDIT_MOVIE_SELECTION_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::library::edit_movie_ui::EditMovieUi;
  use crate::ui::ui_test_utils::test_utils::{TerminalSize, render_to_string_with_app};

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

  mod snapshot_tests {
    use rstest::rstest;
    use super::*;

    #[rstest]
    #[case(ActiveRadarrBlock::EditMoviePrompt, None, 0)]
    #[case(ActiveRadarrBlock::EditMoviePrompt, Some(ActiveRadarrBlock::MovieDetails), 0)]
    #[case(ActiveRadarrBlock::EditMoviePrompt, Some(ActiveRadarrBlock::MovieHistory), 1)]
    #[case(ActiveRadarrBlock::EditMoviePrompt, Some(ActiveRadarrBlock::FileInfo), 2)]
    #[case(ActiveRadarrBlock::EditMoviePrompt, Some(ActiveRadarrBlock::Cast), 3)]
    #[case(ActiveRadarrBlock::EditMoviePrompt, Some(ActiveRadarrBlock::Crew), 4)]
    #[case(ActiveRadarrBlock::EditMoviePrompt, Some(ActiveRadarrBlock::ManualSearch), 5)]
    fn test_edit_movie_ui_renders_edit_movie_modal(
      #[case] active_radarr_block: ActiveRadarrBlock,
      #[case] context: Option<ActiveRadarrBlock>,
      #[case] index: usize,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack((active_radarr_block, context).into());
      if context.is_some() {
        app.data.radarr_data.movie_info_tabs.set_index(index);
      }
      app.data.radarr_data.selected_block = BlockSelectionState::new(EDIT_MOVIE_SELECTION_BLOCKS);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        EditMovieUi::draw(f, app, f.area());
      });

      if let Some(context) = context {
        insta::assert_snapshot!(format!("{}_{}", active_radarr_block.to_string(), context.to_string()), output);
      } else {
        insta::assert_snapshot!(active_radarr_block.to_string(), output);
      }
    }
  }
}