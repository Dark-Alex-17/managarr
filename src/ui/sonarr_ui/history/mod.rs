use crate::app::App;
use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, HISTORY_BLOCKS};
use crate::models::sonarr_models::{SonarrHistoryEventType, SonarrHistoryItem};
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

use super::sonarr_ui_utils::{
  create_download_failed_history_event_details,
  create_download_folder_imported_history_event_details,
  create_episode_file_deleted_history_event_details,
  create_episode_file_renamed_history_event_details, create_grabbed_history_event_details,
  create_no_data_history_event_details,
};

#[cfg(test)]
#[path = "history_ui_tests.rs"]
mod history_ui_tests;

pub(super) struct HistoryUi;

impl DrawUi for HistoryUi {
  fn accepts(route: Route) -> bool {
    if let Route::Sonarr(active_sonarr_block, _) = route {
      return HISTORY_BLOCKS.contains(&active_sonarr_block);
    }

    false
  }

  fn draw(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
    if let Route::Sonarr(active_sonarr_block, _) = app.get_current_route() {
      draw_history_table(f, app, area);

      if active_sonarr_block == ActiveSonarrBlock::HistoryItemDetails {
        draw_history_item_details_popup(f, app);
      }
    }
  }
}

fn draw_history_table(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  let current_selection = if app.data.sonarr_data.history.items.is_empty() {
    SonarrHistoryItem::default()
  } else {
    app.data.sonarr_data.history.current_selection().clone()
  };
  if let Route::Sonarr(active_sonarr_block, _) = app.get_current_route() {
    let history_table_footer = app
      .data
      .sonarr_data
      .main_tabs
      .get_active_tab_contextual_help();

    let history_row_mapping = |history_item: &SonarrHistoryItem| {
      let SonarrHistoryItem {
        source_title,
        languages,
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
        Cell::from(
          languages
            .iter()
            .map(|language| language.name.to_owned())
            .collect::<Vec<String>>()
            .join(","),
        ),
        Cell::from(quality.quality.name.to_owned()),
        Cell::from(date.to_string()),
      ])
      .primary()
    };
    let history_table =
      ManagarrTable::new(Some(&mut app.data.sonarr_data.history), history_row_mapping)
        .block(layout_block_top_border())
        .loading(app.is_loading)
        .footer(history_table_footer)
        .sorting(active_sonarr_block == ActiveSonarrBlock::HistorySortPrompt)
        .searching(active_sonarr_block == ActiveSonarrBlock::SearchHistory)
        .search_produced_empty_results(active_sonarr_block == ActiveSonarrBlock::SearchHistoryError)
        .filtering(active_sonarr_block == ActiveSonarrBlock::FilterHistory)
        .filter_produced_empty_results(active_sonarr_block == ActiveSonarrBlock::FilterHistoryError)
        .headers(["Source Title", "Event Type", "Language", "Quality", "Date"])
        .constraints([
          Constraint::Percentage(40),
          Constraint::Percentage(15),
          Constraint::Percentage(12),
          Constraint::Percentage(13),
          Constraint::Percentage(20),
        ]);

    if [
      ActiveSonarrBlock::SearchHistory,
      ActiveSonarrBlock::FilterHistory,
    ]
    .contains(&active_sonarr_block)
    {
      history_table.show_cursor(f, area);
    }

    f.render_widget(history_table, area);
  }
}

fn draw_history_item_details_popup(f: &mut Frame<'_>, app: &mut App<'_>) {
  let current_selection = if app.data.sonarr_data.history.items.is_empty() {
    SonarrHistoryItem::default()
  } else {
    app.data.sonarr_data.history.current_selection().clone()
  };

  let line_vec = match current_selection.event_type {
    SonarrHistoryEventType::Grabbed => create_grabbed_history_event_details(current_selection),
    SonarrHistoryEventType::DownloadFolderImported => {
      create_download_folder_imported_history_event_details(current_selection)
    }
    SonarrHistoryEventType::DownloadFailed => {
      create_download_failed_history_event_details(current_selection)
    }
    SonarrHistoryEventType::EpisodeFileDeleted => {
      create_episode_file_deleted_history_event_details(current_selection)
    }
    SonarrHistoryEventType::EpisodeFileRenamed => {
      create_episode_file_renamed_history_event_details(current_selection)
    }
    _ => create_no_data_history_event_details(current_selection),
  };
  let text = Text::from(line_vec);

  let message = Message::new(text)
    .title("Details")
    .style(Style::new().secondary())
    .alignment(Alignment::Left);

  f.render_widget(Popup::new(message).size(Size::NarrowMessage), f.area());
}
