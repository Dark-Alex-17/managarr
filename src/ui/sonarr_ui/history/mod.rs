use crate::app::App;
use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, HISTORY_BLOCKS};
use crate::models::sonarr_models::{SonarrHistoryData, SonarrHistoryEventType, SonarrHistoryItem};
use crate::models::Route;
use crate::ui::styles::ManagarrStyle;
use crate::ui::utils::{get_width_from_percentage, layout_block_top_border};
use crate::ui::widgets::managarr_table::ManagarrTable;
use crate::ui::widgets::message::Message;
use crate::ui::widgets::popup::{Popup, Size};
use crate::ui::{draw_input_box_popup, draw_popup_over, DrawUi};
use ratatui::layout::{Alignment, Constraint, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Cell, Row};
use ratatui::Frame;

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
      match active_sonarr_block {
        ActiveSonarrBlock::History | ActiveSonarrBlock::HistorySortPrompt => {
          draw_history_table(f, app, area)
        }
        ActiveSonarrBlock::SearchHistory => draw_popup_over(
          f,
          app,
          area,
          draw_history_table,
          draw_history_search_box,
          Size::InputBox,
        ),
        ActiveSonarrBlock::SearchHistoryError => {
          let popup = Popup::new(Message::new("History item not found!")).size(Size::Message);

          draw_history_table(f, app, area);
          f.render_widget(popup, f.area());
        }
        ActiveSonarrBlock::FilterHistory => draw_popup_over(
          f,
          app,
          area,
          draw_history_table,
          draw_filter_history_box,
          Size::InputBox,
        ),
        ActiveSonarrBlock::FilterHistoryError => {
          let popup = Popup::new(Message::new(
            "No history items found matching the given filter!",
          ))
          .size(Size::Message);

          draw_history_table(f, app, area);
          f.render_widget(popup, f.area());
        }
        ActiveSonarrBlock::HistoryItemDetails => {
          draw_history_table(f, app, area);
          draw_history_item_details_popup(f, app);
        }
        _ => (),
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
        language,
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
        Cell::from(language.name.to_owned()),
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
        .headers(["Source Title", "Event Type", "Language", "Quality", "Date"])
        .constraints([
          Constraint::Percentage(40),
          Constraint::Percentage(15),
          Constraint::Percentage(12),
          Constraint::Percentage(13),
          Constraint::Percentage(20),
        ]);

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
    SonarrHistoryEventType::Unknown => create_unknown_event_vec(current_selection),
    SonarrHistoryEventType::DownloadFolderImported => {
      create_download_folder_imported_event_vec(current_selection)
    }
    SonarrHistoryEventType::DownloadFailed => create_download_failed_event_vec(current_selection),
    SonarrHistoryEventType::EpisodeFileDeleted => {
      create_episode_file_deleted_event_vec(current_selection)
    }
    SonarrHistoryEventType::EpisodeFileRenamed => {
      create_episode_file_renamed_event_vec(current_selection)
    }
    _ => create_no_data_event_vec(current_selection),
  };
  let text = Text::from(line_vec);

  let message = Message::new(text)
    .title("Details")
    .style(Style::new().secondary())
    .alignment(Alignment::Left);

  f.render_widget(Popup::new(message).size(Size::NarrowMessage), f.area());
}

fn create_unknown_event_vec(history_item: SonarrHistoryItem) -> Vec<Line<'static>> {
  let SonarrHistoryItem {
    source_title, data, ..
  } = history_item;
  let SonarrHistoryData {
    indexer,
    release_group,
    series_match_type,
    nzb_info_url,
    download_client_name,
    age,
    published_date,
    ..
  } = data;

  vec![
    Line::from(vec![
      "Source Title: ".bold().secondary(),
      source_title.text.secondary(),
    ]),
    Line::from(vec![
      "Indexer: ".bold().secondary(),
      indexer.unwrap_or_default().secondary(),
    ]),
    Line::from(vec![
      "Release Group: ".bold().secondary(),
      release_group.unwrap_or_default().secondary(),
    ]),
    Line::from(vec![
      "Series Match Type: ".bold().secondary(),
      series_match_type.unwrap_or_default().secondary(),
    ]),
    Line::from(vec![
      "NZB Info URL: ".bold().secondary(),
      nzb_info_url.unwrap_or_default().secondary(),
    ]),
    Line::from(vec![
      "Download Client Name: ".bold().secondary(),
      download_client_name.unwrap_or_default().secondary(),
    ]),
    Line::from(vec![
      "Age: ".bold().secondary(),
      format!("{} days", age.unwrap_or("0".to_owned())).secondary(),
    ]),
    Line::from(vec![
      "Published Date: ".bold().secondary(),
      published_date.unwrap_or_default().to_string().secondary(),
    ]),
  ]
}

fn create_download_folder_imported_event_vec(
  history_item: SonarrHistoryItem,
) -> Vec<Line<'static>> {
  let SonarrHistoryItem {
    source_title, data, ..
  } = history_item;
  let SonarrHistoryData {
    dropped_path,
    imported_path,
    ..
  } = data;

  vec![
    Line::from(vec![
      "Source Title: ".bold().secondary(),
      source_title.text.secondary(),
    ]),
    Line::from(vec![
      "Dropped Path: ".bold().secondary(),
      dropped_path.unwrap_or_default().secondary(),
    ]),
    Line::from(vec![
      "Imported Path: ".bold().secondary(),
      imported_path.unwrap_or_default().secondary(),
    ]),
  ]
}

fn create_download_failed_event_vec(history_item: SonarrHistoryItem) -> Vec<Line<'static>> {
  let SonarrHistoryItem {
    source_title, data, ..
  } = history_item;
  let SonarrHistoryData { message, .. } = data;

  vec![
    Line::from(vec![
      "Source Title: ".bold().secondary(),
      source_title.text.secondary(),
    ]),
    Line::from(vec![
      "Message: ".bold().secondary(),
      message.unwrap_or_default().secondary(),
    ]),
  ]
}

fn create_episode_file_deleted_event_vec(history_item: SonarrHistoryItem) -> Vec<Line<'static>> {
  let SonarrHistoryItem {
    source_title, data, ..
  } = history_item;
  let SonarrHistoryData { reason, .. } = data;

  vec![
    Line::from(vec![
      "Source Title: ".bold().secondary(),
      source_title.text.secondary(),
    ]),
    Line::from(vec![
      "Reason: ".bold().secondary(),
      reason.unwrap_or_default().secondary(),
    ]),
  ]
}

fn create_episode_file_renamed_event_vec(history_item: SonarrHistoryItem) -> Vec<Line<'static>> {
  let SonarrHistoryItem {
    source_title, data, ..
  } = history_item;
  let SonarrHistoryData {
    source_path,
    source_relative_path,
    path,
    relative_path,
    ..
  } = data;

  vec![
    Line::from(vec![
      "Source Title: ".bold().secondary(),
      source_title.text.secondary(),
    ]),
    Line::from(vec![
      "Source Path: ".bold().secondary(),
      source_path.unwrap_or_default().secondary(),
    ]),
    Line::from(vec![
      "Source Relative Path: ".bold().secondary(),
      source_relative_path.unwrap_or_default().secondary(),
    ]),
    Line::from(vec![
      "Destination Path: ".bold().secondary(),
      path.unwrap_or_default().secondary(),
    ]),
    Line::from(vec![
      "Destination Relative Path: ".bold().secondary(),
      relative_path.unwrap_or_default().secondary(),
    ]),
  ]
}

fn create_no_data_event_vec(history_item: SonarrHistoryItem) -> Vec<Line<'static>> {
  let SonarrHistoryItem { source_title, .. } = history_item;

  vec![
    Line::from(vec![
      "Source Title: ".bold().secondary(),
      source_title.text.secondary(),
    ]),
    Line::from(vec![String::new().secondary()]),
    Line::from(vec!["No additional data available".bold().secondary()]),
  ]
}

fn draw_history_search_box(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  draw_input_box_popup(
    f,
    area,
    "Search",
    app.data.sonarr_data.history.search.as_ref().unwrap(),
  );
}

fn draw_filter_history_box(f: &mut Frame<'_>, app: &mut App<'_>, area: Rect) {
  draw_input_box_popup(
    f,
    area,
    "Filter",
    app.data.sonarr_data.history.filter.as_ref().unwrap(),
  )
}
