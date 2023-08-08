use tui::backend::Backend;
use tui::layout::Rect;
use tui::Frame;

use crate::app::radarr::ActiveRadarrBlock;
use crate::app::App;
use crate::models::Route;
use crate::ui::radarr_ui::library_ui::draw_library;
use crate::ui::{draw_prompt_box_with_checkboxes, draw_prompt_popup_over, DrawUi};

pub(super) struct DeleteMovieUi {}

impl DrawUi for DeleteMovieUi {
  fn draw<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, content_rect: Rect) {
    if matches!(
      *app.get_current_route(),
      Route::Radarr(ActiveRadarrBlock::DeleteMoviePrompt, _)
    ) {
      let draw_delete_movie_prompt =
        |f: &mut Frame<'_, B>, app: &mut App<'_>, prompt_area: Rect| {
          let selected_block = app.data.radarr_data.selected_block.get_active_block();
          draw_prompt_box_with_checkboxes(
            f,
            prompt_area,
            "Delete Movie",
            format!(
              "Do you really want to delete: {}?",
              app.data.radarr_data.movies.current_selection().title
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
