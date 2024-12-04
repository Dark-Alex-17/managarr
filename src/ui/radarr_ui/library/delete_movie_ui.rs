use ratatui::layout::Rect;
use ratatui::Frame;

use crate::app::App;
use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, DELETE_MOVIE_BLOCKS};
use crate::models::Route;
use crate::ui::radarr_ui::library::draw_library;
use crate::ui::widgets::checkbox::Checkbox;
use crate::ui::widgets::confirmation_prompt::ConfirmationPrompt;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::DrawUi;

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

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
    if matches!(
      app.get_current_route(),
      Route::Radarr(ActiveRadarrBlock::DeleteMoviePrompt, _)
    ) {
      let selected_block = app.data.radarr_data.selected_block.get_active_block();
      let prompt = format!(
        "Do you really want to delete: \n{}?",
        app.data.radarr_data.movies.current_selection().title.text
      );
      let checkboxes = vec![
        Checkbox::new("Delete Movie File")
          .checked(app.data.radarr_data.delete_movie_files)
          .highlighted(selected_block == ActiveRadarrBlock::DeleteMovieToggleDeleteFile),
        Checkbox::new("Add List Exclusion")
          .checked(app.data.radarr_data.add_list_exclusion)
          .highlighted(selected_block == ActiveRadarrBlock::DeleteMovieToggleAddListExclusion),
      ];
      let confirmation_prompt = ConfirmationPrompt::new()
        .title("Delete Movie")
        .prompt(&prompt)
        .checkboxes(checkboxes)
        .yes_no_highlighted(selected_block == ActiveRadarrBlock::DeleteMovieConfirmPrompt)
        .yes_no_value(app.data.radarr_data.prompt_confirm);

      draw_library(f, app, area);
      f.render_widget(
        Popup::new(confirmation_prompt).size(Size::MediumPrompt),
        f.area(),
      );
    }
  }
}
