use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Rect};
use tui::widgets::{Cell, Paragraph, Row};
use tui::Frame;

use crate::app::radarr::ActiveRadarrBlock;
use crate::app::App;
use crate::models::radarr_models::RootFolder;
use crate::models::Route;
use crate::ui::utils::{
  borderless_block, layout_block_top_border, show_cursor, style_default, style_help, style_primary,
  title_block_centered, vertical_chunks_with_margin,
};
use crate::ui::{
  draw_popup_over, draw_prompt_box, draw_prompt_popup_over, draw_table, DrawUi, TableProps,
};
use crate::utils::convert_to_gb;

pub(super) struct RootFoldersUi {}

impl DrawUi for RootFoldersUi {
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
          15,
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
      content: &mut app.data.radarr_data.root_folders,
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

      let space: f64 = convert_to_gb(free_space.as_u64().unwrap());

      Row::new(vec![
        Cell::from(path.to_owned()),
        Cell::from(format!("{:.2} GB", space)),
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
  );
}

fn draw_add_root_folder_prompt_box<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App<'_>,
  area: Rect,
) {
  let chunks = vertical_chunks_with_margin(
    vec![
      Constraint::Length(3),
      Constraint::Length(1),
      Constraint::Min(0),
    ],
    area,
    1,
  );
  let block_title = "Add Root Folder";
  let offset = *app.data.radarr_data.edit_path.offset.borrow();
  let block_content = &app.data.radarr_data.edit_path.text;

  let input = Paragraph::new(block_content.as_str())
    .style(style_default())
    .block(title_block_centered(block_title));
  let help = Paragraph::new("<esc> cancel")
    .style(style_help())
    .alignment(Alignment::Center)
    .block(borderless_block());
  show_cursor(f, chunks[0], offset, block_content);

  f.render_widget(input, chunks[0]);
  f.render_widget(help, chunks[1]);
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
      "Do you really want to delete this root folder: {}?",
      app.data.radarr_data.root_folders.current_selection().path
    )
    .as_str(),
    app.data.radarr_data.prompt_confirm,
  );
}
