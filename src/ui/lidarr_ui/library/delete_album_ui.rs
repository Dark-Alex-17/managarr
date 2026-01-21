use ratatui::Frame;
use ratatui::layout::Rect;

use crate::app::App;
use crate::models::Route;
use crate::models::servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, DELETE_ALBUM_BLOCKS};
use crate::ui::DrawUi;
use crate::ui::widgets::checkbox::Checkbox;
use crate::ui::widgets::confirmation_prompt::ConfirmationPrompt;
use crate::ui::widgets::popup::{Popup, Size};

#[cfg(test)]
#[path = "delete_album_ui_tests.rs"]
mod delete_album_ui_tests;

pub(in crate::ui::lidarr_ui) struct DeleteAlbumUi;

impl DrawUi for DeleteAlbumUi {
  fn accepts(route: Route) -> bool {
    let Route::Lidarr(active_lidarr_block, _) = route else {
      return false;
    };
    DELETE_ALBUM_BLOCKS.contains(&active_lidarr_block)
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, _area: Rect) {
    if matches!(
      app.get_current_route(),
      Route::Lidarr(ActiveLidarrBlock::DeleteAlbumPrompt, _)
    ) {
      let selected_block = app.data.lidarr_data.selected_block.get_active_block();
      let prompt = format!(
        "Do you really want to delete the album: \n{}?",
        app.data.lidarr_data.albums.current_selection().title.text
      );
      let checkboxes = vec![
        Checkbox::new("Delete Album Files")
          .checked(app.data.lidarr_data.delete_files)
          .highlighted(selected_block == ActiveLidarrBlock::DeleteAlbumToggleDeleteFile),
        Checkbox::new("Add List Exclusion")
          .checked(app.data.lidarr_data.add_import_list_exclusion)
          .highlighted(selected_block == ActiveLidarrBlock::DeleteAlbumToggleAddListExclusion),
      ];
      let confirmation_prompt = ConfirmationPrompt::new()
        .title("Delete Album")
        .prompt(&prompt)
        .checkboxes(checkboxes)
        .yes_no_highlighted(selected_block == ActiveLidarrBlock::DeleteAlbumConfirmPrompt)
        .yes_no_value(app.data.lidarr_data.prompt_confirm);

      f.render_widget(
        Popup::new(confirmation_prompt).size(Size::MediumPrompt),
        f.area(),
      );
    }
  }
}
