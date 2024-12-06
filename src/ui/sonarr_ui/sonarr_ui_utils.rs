use ratatui::style::Stylize;
use ratatui::text::Line;

use crate::models::sonarr_models::{SonarrHistoryData, SonarrHistoryItem};
use crate::ui::styles::ManagarrStyle;

#[cfg(test)]
#[path = "sonarr_ui_utils_tests.rs"]
mod sonarr_ui_utils_tests;

pub(super) fn create_grabbed_history_event_details(
  history_item: SonarrHistoryItem,
) -> Vec<Line<'static>> {
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

pub(super) fn create_download_folder_imported_history_event_details(
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

pub(super) fn create_download_failed_history_event_details(
  history_item: SonarrHistoryItem,
) -> Vec<Line<'static>> {
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

pub(super) fn create_episode_file_deleted_history_event_details(
  history_item: SonarrHistoryItem,
) -> Vec<Line<'static>> {
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

pub(super) fn create_episode_file_renamed_history_event_details(
  history_item: SonarrHistoryItem,
) -> Vec<Line<'static>> {
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

pub(super) fn create_no_data_history_event_details(
  history_item: SonarrHistoryItem,
) -> Vec<Line<'static>> {
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
