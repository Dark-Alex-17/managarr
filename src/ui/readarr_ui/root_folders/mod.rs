use add_root_folder_ui::AddRootFolderUi;
use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::widgets::{Cell, Row};

use crate::app::App;
use crate::models::Route;
use crate::models::servarr_data::readarr::readarr_data::{
  ADD_ROOT_FOLDER_BLOCKS, ActiveReadarrBlock, ROOT_FOLDERS_BLOCKS,
};
use crate::models::servarr_models::RootFolder;
use crate::ui::DrawUi;
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::layout_block_top_border;
use crate::ui::widgets::confirmation_prompt::ConfirmationPrompt;
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::popup::{Popup, Size};
use crate::utils::format_size;

mod add_root_folder_ui;

#[cfg(test)]
#[path = "root_folders_ui_tests.rs"]
mod root_folders_ui_tests;

pub(super) struct RootFoldersUi;

impl DrawUi for RootFoldersUi {
  fn accepts(route: Route) -> bool {
    if let Route::Readarr(active_readarr_block, _) = route {
      return ROOT_FOLDERS_BLOCKS.contains(&active_readarr_block)
        || ADD_ROOT_FOLDER_BLOCKS.contains(&active_readarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
    if let Route::Readarr(active_readarr_block, _) = app.get_current_route() {
      draw_root_folders(f, app, area);

      if ADD_ROOT_FOLDER_BLOCKS.contains(&active_readarr_block) {
        AddRootFolderUi::draw(f, app, area);
      } else if active_readarr_block == ActiveReadarrBlock::DeleteRootFolderPrompt {
        let prompt = format!(
          "Do you really want to delete this root folder: \n{}?",
          app.data.readarr_data.root_folders.current_selection().path
        );
        let confirmation_prompt = ConfirmationPrompt::new()
          .title("Delete Root Folder")
          .prompt(&prompt)
          .yes_no_value(app.data.readarr_data.prompt_confirm);

        f.render_widget(
          Popup::new(confirmation_prompt).size(Size::MediumPrompt),
          f.area(),
        );
      }
    }
  }
}

fn draw_root_folders(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let root_folders_row_mapping = |root_folders: &RootFolder| {
    let RootFolder {
      path,
      free_space,
      unmapped_folders,
      ..
    } = root_folders;

    let space = format_size(*free_space, 2);

    Row::new(vec![
      Cell::from(path.to_owned()),
      Cell::from(space),
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
    Some(&mut app.data.readarr_data.root_folders),
    root_folders_row_mapping,
  )
  .block(layout_block_top_border())
  .loading(app.is_loading)
  .headers(["Path", "Free Space", "Unmapped Folders"])
  .constraints([
    Constraint::Ratio(3, 5),
    Constraint::Ratio(1, 5),
    Constraint::Ratio(1, 5),
  ]);

  f.render_widget(root_folders_table, area);
}
