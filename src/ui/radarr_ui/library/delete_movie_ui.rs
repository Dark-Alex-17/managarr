use ratatui::layout::Rect;
use ratatui::Frame;

use crate::app::App;
use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, DELETE_MOVIE_BLOCKS};
use crate::models::Route;
use crate::ui::radarr_ui::library::draw_library;
use crate::ui::{draw_prompt_box_with_checkboxes, draw_prompt_popup_over, DrawUi};

#[cfg(test)]
#[path = "delete_movie_ui_tests.rs"]
mod delete_movie_ui_tests;

pub(super) struct DeleteMovieUi;

impl DrawUi for DeleteMovieUi {
  fn accepts(route: Route) -> bool {
    if let Route::Radarr(active_radarr_block, _) = route {
      return DELETE_MOVIE_BLOCKS.contains(&active_radarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, content_rect: Rect) {
    if matches!(
      *app.get_current_route(),
      Route::Radarr(ActiveRadarrBlock::DeleteMoviePrompt, _)
    ) {
      let draw_delete_movie_prompt = |f: &mut Frame<'_>, app: &mut App<'_>, prompt_area: Rect| {
        let selected_block = app.data.radarr_data.selected_block.get_active_block();
        draw_prompt_box_with_checkboxes(
          f,
          prompt_area,
          "Delete Movie",
          format!(
            "Do you really want to delete: \n{}?",
            app.data.radarr_data.movies.current_selection().title.text
          )
          .as_str(),
          vec![
            (
              "Delete Movie Files",
              app.data.radarr_data.delete_movie_files,
              selected_block == &ActiveRadarrBlock::DeleteMovieToggleDeleteFile,
            ),
            (
              "Add List Exclusion",
              app.data.radarr_data.add_list_exclusion,
              selected_block == &ActiveRadarrBlock::DeleteMovieToggleAddListExclusion,
            ),
          ],
          selected_block == &ActiveRadarrBlock::DeleteMovieConfirmPrompt,
          app.data.radarr_data.prompt_confirm,
        )
      };

      draw_prompt_popup_over(f, app, content_rect, draw_library, draw_delete_movie_prompt);
    }
  }
}
