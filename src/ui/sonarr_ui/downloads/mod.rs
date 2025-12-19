use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::widgets::{Cell, Row};

use crate::app::App;
use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, DOWNLOADS_BLOCKS};
use crate::models::sonarr_models::DownloadRecord;
use crate::models::{HorizontallyScrollableText, Route};
use crate::ui::DrawUi;
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{get_width_from_percentage, layout_block_top_border};
use crate::ui::widgets::confirmation_prompt::ConfirmationPrompt;
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::popup::{Popup, Size};
use crate::utils::convert_f64_to_gb;

#[cfg(test)]
#[path = "downloads_ui_tests.rs"]
mod downloads_ui_tests;

pub(super) struct DownloadsUi;

impl DrawUi for DownloadsUi {
  fn accepts(route: Route) -> bool {
    if let Route::Sonarr(active_sonarr_block, _) = route {
      return DOWNLOADS_BLOCKS.contains(&active_sonarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
    if let Route::Sonarr(active_sonarr_block, _) = app.get_current_route() {
      draw_downloads(f, app, area);

      match active_sonarr_block {
        ActiveSonarrBlock::DeleteDownloadPrompt => {
          let prompt = format!(
            "Do you really want to delete this download: \n{}?",
            app.data.sonarr_data.downloads.current_selection().title
          );
          let confirmation_prompt = ConfirmationPrompt::new()
            .title("Cancel Download")
            .prompt(&prompt)
            .yes_no_value(app.data.sonarr_data.prompt_confirm);

          f.render_widget(
            Popup::new(confirmation_prompt).size(Size::MediumPrompt),
            f.area(),
          );
        }
        ActiveSonarrBlock::UpdateDownloadsPrompt => {
          let confirmation_prompt = ConfirmationPrompt::new()
            .title("Update Downloads")
            .prompt("Do you want to update your downloads?")
            .yes_no_value(app.data.sonarr_data.prompt_confirm);

          f.render_widget(
            Popup::new(confirmation_prompt).size(Size::MediumPrompt),
            f.area(),
          );
        }
        _ => (),
      }
    }
  }
}

fn draw_downloads(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let current_selection = if app.data.sonarr_data.downloads.items.is_empty() {
    DownloadRecord::default()
  } else {
    app.data.sonarr_data.downloads.current_selection().clone()
  };

  let downloads_row_mapping = |download_record: &DownloadRecord| {
    let DownloadRecord {
      title,
      size,
      sizeleft,
      download_client,
      indexer,
      output_path,
      ..
    } = download_record;

    if output_path.is_some() {
      output_path.as_ref().unwrap().scroll_left_or_reset(
        get_width_from_percentage(area, 18),
        current_selection == *download_record,
        app.ui_scroll_tick_count == 0,
      );
    }

    let percent = if *size == 0.0 {
      0.0
    } else {
      1f64 - (*sizeleft / *size)
    };
    let file_size: f64 = convert_f64_to_gb(*size);

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
      Cell::from(
        download_client
          .as_ref()
          .unwrap_or(&String::new())
          .to_owned(),
      ),
    ])
    .primary()
  };
  let downloads_table = ManagarrTable::new(
    Some(&mut app.data.sonarr_data.downloads),
    downloads_row_mapping,
  )
  .block(layout_block_top_border())
  .loading(app.is_loading)
  .headers([
    "Title",
    "Percent Complete",
    "Size",
    "Output Path",
    "Indexer",
    "Download Client",
  ])
  .constraints([
    Constraint::Percentage(30),
    Constraint::Percentage(11),
    Constraint::Percentage(11),
    Constraint::Percentage(18),
    Constraint::Percentage(17),
    Constraint::Percentage(13),
  ]);

  f.render_widget(downloads_table, area);
}
