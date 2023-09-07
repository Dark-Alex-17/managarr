use tui::backend::Backend;
use tui::layout::{Constraint, Rect};
use tui::widgets::{Cell, Row};
use tui::Frame;

use crate::app::App;
use crate::models::radarr_models::RootFolder;
use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, ROOT_FOLDERS_BLOCKS};
use crate::models::Route;
use crate::ui::utils::{layout_block_top_border, style_primary};
use crate::ui::{
  draw_input_box_popup, draw_popup_over, draw_prompt_box, draw_prompt_popup_over, draw_table,
  DrawUi, TableProps,
};
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

  fn draw<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, content_rect: Rect) {
    if let Route::Radarr(active_radarr_block, _) = *app.get_current_route() {
      match active_radarr_block {
        ActiveRadarrBlock::RootFolders => draw_root_folders(f, app, content_rect),
        ActiveRadarrBlock::AddRootFolderPrompt => draw_popup_over(
          f,
          app,
          content_rect,
          draw_root_folders,
          draw_add_root_folder_prompt_box,
          30,
          13,
        ),
        ActiveRadarrBlock::DeleteRootFolderPrompt => draw_prompt_popup_over(
          f,
          app,
          content_rect,
          draw_root_folders,
          draw_delete_root_folder_prompt,
        ),
        _ => (),
      }
    }
  }
}

fn draw_root_folders<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect) {
  draw_table(
    f,
    area,
    layout_block_top_border(),
    TableProps {
      content: Some(&mut app.data.radarr_data.root_folders),
      wrapped_content: None,
      table_headers: vec!["Path", "Free Space", "Unmapped Folders"],
      constraints: vec![
        Constraint::Percentage(60),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
      ],
      help: app
        .data
        .radarr_data
        .main_tabs
        .get_active_tab_contextual_help(),
    },
    |root_folders| {
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
      .style(style_primary())
    },
    app.is_loading,
    true,
  );
}

fn draw_add_root_folder_prompt_box<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App<'_>,
  area: Rect,
) {
  draw_input_box_popup(
    f,
    area,
    "Add Root Folder",
    app.data.radarr_data.edit_root_folder.as_ref().unwrap(),
  );
}

fn draw_delete_root_folder_prompt<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App<'_>,
  prompt_area: Rect,
) {
  draw_prompt_box(
    f,
    prompt_area,
    "Delete Root Folder",
    format!(
      "Do you really want to delete this root folder: \n{}?",
      app.data.radarr_data.root_folders.current_selection().path
    )
    .as_str(),
    app.data.radarr_data.prompt_confirm,
  );
}
