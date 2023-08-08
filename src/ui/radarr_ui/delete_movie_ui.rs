use tui::backend::Backend;
use tui::layout::Rect;
use tui::Frame;

use crate::app::radarr::ActiveRadarrBlock;
use crate::app::App;
use crate::models::Route;
use crate::ui::draw_prompt_box_with_checkboxes;

pub(super) fn draw_delete_movie_prompt<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App<'_>,
  prompt_area: Rect,
) {
  if matches!(
    *app.get_current_route(),
    Route::Radarr(ActiveRadarrBlock::DeleteMoviePrompt, _)
  ) {
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
  }
}
