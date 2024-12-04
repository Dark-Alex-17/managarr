use ratatui::layout::Rect;
use ratatui::Frame;

use crate::app::App;
use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, DELETE_SERIES_BLOCKS};
use crate::models::Route;
use crate::ui::sonarr_ui::library::draw_library;
use crate::ui::widgets::checkbox::Checkbox;
use crate::ui::widgets::confirmation_prompt::ConfirmationPrompt;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::DrawUi;

#[cfg(test)]
#[path = "delete_series_ui_tests.rs"]
mod delete_series_ui_tests;

pub(super) struct DeleteSeriesUi;

impl DrawUi for DeleteSeriesUi {
  fn accepts(route: Route) -> bool {
    if let Route::Sonarr(active_sonarr_block, _) = route {
      return DELETE_SERIES_BLOCKS.contains(&active_sonarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
    if matches!(
      app.get_current_route(),
      Route::Sonarr(ActiveSonarrBlock::DeleteSeriesPrompt, _)
    ) {
      let selected_block = app.data.sonarr_data.selected_block.get_active_block();
      let prompt = format!(
        "Do you really want to delete: \n{}?",
        app.data.sonarr_data.series.current_selection().title.text
      );
      let checkboxes = vec![
        Checkbox::new("Delete Series File")
          .checked(app.data.sonarr_data.delete_series_files)
          .highlighted(selected_block == ActiveSonarrBlock::DeleteSeriesToggleDeleteFile),
        Checkbox::new("Add List Exclusion")
          .checked(app.data.sonarr_data.add_list_exclusion)
          .highlighted(selected_block == ActiveSonarrBlock::DeleteSeriesToggleAddListExclusion),
      ];
      let confirmation_prompt = ConfirmationPrompt::new()
        .title("Delete Series")
        .prompt(&prompt)
        .checkboxes(checkboxes)
        .yes_no_highlighted(selected_block == ActiveSonarrBlock::DeleteSeriesConfirmPrompt)
        .yes_no_value(app.data.sonarr_data.prompt_confirm);

      draw_library(f, app, area);
      f.render_widget(
        Popup::new(confirmation_prompt).size(Size::MediumPrompt),
        f.area(),
      );
    }
  }
}
