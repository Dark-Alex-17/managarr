#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::BlockSelectionState;
  use crate::models::servarr_data::radarr::radarr_data::{
    ActiveRadarrBlock, DELETE_MOVIE_BLOCKS, DELETE_MOVIE_SELECTION_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::library::delete_movie_ui::DeleteMovieUi;
  use crate::ui::ui_test_utils::test_utils::{TerminalSize, render_to_string_with_app};

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

  mod snapshot_tests {
    use super::*;

    #[test]
    fn test_delete_movie_ui_renders_delete_movie_prompt() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveRadarrBlock::DeleteMoviePrompt.into());
      app.data.radarr_data.selected_block = BlockSelectionState::new(DELETE_MOVIE_SELECTION_BLOCKS);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        DeleteMovieUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}