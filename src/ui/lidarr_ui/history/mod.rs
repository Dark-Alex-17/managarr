use crate::app::App;
use crate::models::servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, HISTORY_BLOCKS};
use crate::models::lidarr_models::{LidarrHistoryEventType, LidarrHistoryItem};
use crate::models::Route;
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{get_width_from_percentage, layout_block_top_border};
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::message::Message;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::DrawUi;
use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::style::Style;
use ratatui::text::Text;
use ratatui::widgets::{Cell, Row};
use ratatui::Frame;

// TODO
// use super::lidarr_ui_utils::{
  // create_download_failed_history_event_details,
  // create_download_folder_imported_history_event_details,
  // create_track_file_deleted_history_event_details,
  // create_track_file_renamed_history_event_details, create_grabbed_history_event_details,
  // create_no_data_history_event_details,
// };

#[cfg(test)]
#[path = "history_ui_tests.rs"]
mod history_ui_tests;

pub(super) struct HistoryUi;

impl DrawUi for HistoryUi {
  fn accepts(route: Route) -> bool {
    if let Route::Lidarr(active_lidarr_block, _) = route {
      return HISTORY_BLOCKS.contains(&active_lidarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
    if let Route::Lidarr(active_lidarr_block, _) = app.get_current_route() {
      draw_history_table(f, app, area);

      if active_lidarr_block == ActiveLidarrBlock::HistoryItemDetails {
        draw_history_item_details_popup(f, app);
      }
    }
  }
}

fn draw_history_table(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let current_selection = if app.data.lidarr_data.history.items.is_empty() {
    LidarrHistoryItem::default()
  } else {
    app.data.lidarr_data.history.current_selection().clone()
  };
  if let Route::Lidarr(active_lidarr_block, _) = app.get_current_route() {
    let history_table_footer = app
      .data
      .lidarr_data
      .main_tabs
      .get_active_tab_contextual_help();

    let history_row_mapping = |history_item: &LidarrHistoryItem| {
      let LidarrHistoryItem {
        source_title,
        quality,
        event_type,
        date,
        ..
      } = history_item;

      source_title.scroll_left_or_reset(
        get_width_from_percentage(area, 40),
        current_selection == *history_item,
        app.tick_count % app.ticks_until_scroll == 0,
      );

      Row::new(vec![
        Cell::from(source_title.to_string()),
        Cell::from(event_type.to_string()),
        Cell::from(quality.quality.name.to_owned()),
        Cell::from(date.to_string()),
      ])
      .primary()
    };
    let history_table =
      ManagarrTable::new(Some(&mut app.data.lidarr_data.history), history_row_mapping)
        .block(layout_block_top_border())
        .loading(app.is_loading)
        .footer(history_table_footer)
        .sorting(active_lidarr_block == ActiveLidarrBlock::HistorySortPrompt)
        .searching(active_lidarr_block == ActiveLidarrBlock::SearchHistory)
        .search_produced_empty_results(active_lidarr_block == ActiveLidarrBlock::SearchHistoryError)
        .filtering(active_lidarr_block == ActiveLidarrBlock::FilterHistory)
        .filter_produced_empty_results(active_lidarr_block == ActiveLidarrBlock::FilterHistoryError)
        .headers(["Source Title", "Event Type", "Quality", "Date"])
        .constraints([
          Constraint::Percentage(40),
          Constraint::Percentage(15),
          Constraint::Percentage(13),
          Constraint::Percentage(20),
        ]);

    if [
      ActiveLidarrBlock::SearchHistory,
      ActiveLidarrBlock::FilterHistory,
    ]
    .contains(&active_lidarr_block)
    {
      history_table.show_cursor(f, area);
    }

    f.render_widget(history_table, area);
  }
}

fn draw_history_item_details_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let current_selection = if app.data.lidarr_data.history.items.is_empty() {
    LidarrHistoryItem::default()
  } else {
    app.data.lidarr_data.history.current_selection().clone()
  };

  // TODO
  // let line_vec = match current_selection.event_type {
  //   LidarrHistoryEventType::Grabbed => create_grabbed_history_event_details(current_selection),
  //   LidarrHistoryEventType::DownloadFolderImported => {
  //     create_download_folder_imported_history_event_details(current_selection)
  //   }
  //   LidarrHistoryEventType::DownloadFailed => {
  //     create_download_failed_history_event_details(current_selection)
  //   }
  //   LidarrHistoryEventType::TrackFileDeleted => {
  //     create_track_file_deleted_history_event_details(current_selection)
  //   }
  //   LidarrHistoryEventType::TrackFileRenamed => {
  //     create_track_file_renamed_history_event_details(current_selection)
  //   }
  //   _ => create_no_data_history_event_details(current_selection),
  // };
  // let text = Text::from(line_vec);
  let text = Text::from(String::new());

  let message = Message::new(text)
    .title("Details")
    .style(Style::new().secondary())
    .alignment(Alignment::Left);

  f.render_widget(Popup::new(message).size(Size::NarrowMessage), f.area());
}
