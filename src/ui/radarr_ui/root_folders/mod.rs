use ratatui::layout::{Constraint, Rect};
use ratatui::widgets::{Cell, Row};
use ratatui::Frame;

use crate::app::App;
use crate::models::radarr_models::RootFolder;
use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, ROOT_FOLDERS_BLOCKS};
use crate::models::Route;
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::layout_block_top_border;
use crate::ui::widgets::confirmation_prompt::ConfirmationPrompt;
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::{draw_input_box_popup, draw_popup_over, DrawUi};
use crate::utils::convert_to_gb;

#[cfg(test)]
#[path = "root_folders_ui_tests.rs"]
mod root_folders_ui_tests;

pub(super) struct RootFoldersUi;

impl DrawUi for RootFoldersUi {
  fn accepts(route: Route) -> bool {
    if let Route::Radarr(active_radarr_block, _) = route {
      return ROOT_FOLDERS_BLOCKS.contains(&active_radarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
    if let Route::Radarr(active_radarr_block, _) = *app.get_current_route() {
      match active_radarr_block {
        ActiveRadarrBlock::RootFolders => draw_root_folders(f, app, area),
        ActiveRadarrBlock::AddRootFolderPrompt => draw_popup_over(
          f,
          app,
          area,
          draw_root_folders,
          draw_add_root_folder_prompt_box,
          Size::InputBox,
        ),
        ActiveRadarrBlock::DeleteRootFolderPrompt => {
          let prompt = format!(
            "Do you really want to delete this root folder: \n{}?",
            app.data.radarr_data.root_folders.current_selection().path
          );
          let confirmation_prompt = ConfirmationPrompt::new()
            .title("Delete Root Folder")
            .prompt(&prompt)
            .yes_no_value(app.data.radarr_data.prompt_confirm);

          draw_root_folders(f, app, area);
          f.render_widget(Popup::new(confirmation_prompt).size(Size::Prompt), f.size());
        }
        _ => (),
      }
    }
  }
}

fn draw_root_folders(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let help_footer = app
    .data
    .radarr_data
    .main_tabs
    .get_active_tab_contextual_help();
  let root_folders_row_mapping = |root_folders: &RootFolder| {
    let RootFolder {
      path,
      free_space,
      unmapped_folders,
      ..
    } = root_folders;

    let space: f64 = convert_to_gb(*free_space);

    Row::new(vec![
      Cell::from(path.to_owned()),
      Cell::from(format!("{space:.2} GB")),
      Cell::from(
        unmapped_folders
          .as_ref()
          .unwrap_or(&Vec::new())
          .len()
          .to_string(),
      ),
    ])
    .primary()
  };

  let root_folders_table = ManagarrTable::new(
    Some(&mut app.data.radarr_data.root_folders),
    root_folders_row_mapping,
  )
  .block(layout_block_top_border())
  .loading(app.is_loading)
  .footer(help_footer)
  .headers(["Path", "Free Space", "Unmapped Folders"])
  .constraints([
    Constraint::Ratio(3, 5),
    Constraint::Ratio(1, 5),
    Constraint::Ratio(1, 5),
  ]);

  f.render_widget(root_folders_table, area);
}

fn draw_add_root_folder_prompt_box(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  draw_input_box_popup(
    f,
    area,
    "Add Root Folder",
    app.data.radarr_data.edit_root_folder.as_ref().unwrap(),
  );
}
