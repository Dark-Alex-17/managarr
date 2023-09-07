use tui::backend::Backend;
use tui::layout::{Constraint, Rect};
use tui::widgets::{Cell, Row};
use tui::Frame;

use crate::app::App;
use crate::models::radarr_models::DownloadRecord;
use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, DOWNLOADS_BLOCKS};
use crate::models::{HorizontallyScrollableText, Route};
use crate::ui::utils::{get_width_from_percentage, layout_block_top_border, style_primary};
use crate::ui::{draw_prompt_box, draw_prompt_popup_over, draw_table, DrawUi, TableProps};
use crate::utils::convert_to_gb;

#[cfg(test)]
#[path = "downloads_ui_tests.rs"]
mod downloads_ui_tests;

pub(super) struct DownloadsUi;

impl DrawUi for DownloadsUi {
  fn accepts(route: Route) -> bool {
    if let Route::Radarr(active_radarr_block, _) = route {
      return DOWNLOADS_BLOCKS.contains(&active_radarr_block);
    }

    false
  }

  fn draw<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, content_rect: Rect) {
    if let Route::Radarr(active_radarr_block, _) = *app.get_current_route() {
      match active_radarr_block {
        ActiveRadarrBlock::Downloads => draw_downloads(f, app, content_rect),
        ActiveRadarrBlock::DeleteDownloadPrompt => draw_prompt_popup_over(
          f,
          app,
          content_rect,
          draw_downloads,
          draw_delete_download_prompt,
        ),
        ActiveRadarrBlock::UpdateDownloadsPrompt => draw_prompt_popup_over(
          f,
          app,
          content_rect,
          draw_downloads,
          draw_update_downloads_prompt,
        ),
        _ => (),
      }
    }
  }
}

fn draw_downloads<B: Backend>(f: &mut Frame<'_, B>, app: &mut App<'_>, area: Rect) {
  let current_selection = if app.data.radarr_data.downloads.items.is_empty() {
    DownloadRecord::default()
  } else {
    app.data.radarr_data.downloads.current_selection().clone()
  };

  draw_table(
    f,
    area,
    layout_block_top_border(),
    TableProps {
      content: Some(&mut app.data.radarr_data.downloads),
      wrapped_content: None,
      table_headers: vec![
        "Title",
        "Percent Complete",
        "Size",
        "Output Path",
        "Indexer",
        "Download Client",
      ],
      constraints: vec![
        Constraint::Percentage(30),
        Constraint::Percentage(11),
        Constraint::Percentage(11),
        Constraint::Percentage(18),
        Constraint::Percentage(17),
        Constraint::Percentage(13),
      ],
      help: app
        .data
        .radarr_data
        .main_tabs
        .get_active_tab_contextual_help(),
    },
    |download_record| {
      let DownloadRecord {
        title,
        size,
        sizeleft,
        download_client,
        indexer,
        output_path,
        ..
      } = download_record;

      if matches!(output_path, Some(_)) {
        output_path.as_ref().unwrap().scroll_left_or_reset(
          get_width_from_percentage(area, 18),
          current_selection == *download_record,
          app.tick_count % app.ticks_until_scroll == 0,
        );
      }

      let percent = 1f64 - (*sizeleft as f64 / *size as f64);
      let file_size: f64 = convert_to_gb(*size);

      Row::new(vec![
        Cell::from(title.to_owned()),
        Cell::from(format!("{:.0}%", percent * 100.0)),
        Cell::from(format!("{file_size:.2} GB")),
        Cell::from(
          output_path
            .as_ref()
            .unwrap_or(&HorizontallyScrollableText::default())
            .to_string(),
        ),
        Cell::from(indexer.to_owned()),
        Cell::from(download_client.to_owned()),
      ])
      .style(style_primary())
    },
    app.is_loading,
    true,
  );
}

fn draw_delete_download_prompt<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App<'_>,
  prompt_area: Rect,
) {
  draw_prompt_box(
    f,
    prompt_area,
    "Cancel Download",
    format!(
      "Do you really want to delete this download: \n{}?",
      app.data.radarr_data.downloads.current_selection().title
    )
    .as_str(),
    app.data.radarr_data.prompt_confirm,
  );
}

fn draw_update_downloads_prompt<B: Backend>(
  f: &mut Frame<'_, B>,
  app: &mut App<'_>,
  prompt_area: Rect,
) {
  draw_prompt_box(
    f,
    prompt_area,
    "Update Downloads",
    "Do you want to update your downloads?",
    app.data.radarr_data.prompt_confirm,
  );
}
