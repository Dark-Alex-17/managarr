use ratatui::text::Line;

use crate::models::sonarr_models::{SonarrHistoryData, SonarrHistoryEventType, SonarrHistoryItem};

#[cfg(test)]
#[path = "sonarr_ui_utils_tests.rs"]
mod sonarr_ui_utils_tests;

pub(super) fn create_history_event_details(history_item: SonarrHistoryItem) -> Vec<Line<'static>> {
  let SonarrHistoryItem {
    source_title,
    data,
    event_type,
    ..
  } = history_item;
  let SonarrHistoryData {
    indexer,
    release_group,
    series_match_type,
    nzb_info_url,
    download_client_name,
    age,
    published_date,
    dropped_path,
    imported_path,
    message,
    reason,
    source_path,
    source_relative_path,
    path,
    relative_path,
    ..
  } = data;

  let mut lines = vec![
    Line::from(format!("Source Title: {}", source_title.text.trim_start())),
    Line::from(format!("Event Type: {}", event_type)),
  ];

  match event_type {
    SonarrHistoryEventType::Grabbed => {
      lines.push(Line::from(format!(
        "Indexer: {}",
        indexer.unwrap_or_default().trim_start(),
      )));
      lines.push(Line::from(format!(
        "Release Group: {}",
        release_group.unwrap_or_default().trim_start(),
      )));
      lines.push(Line::from(format!(
        "Series Match Type: {}",
        series_match_type.unwrap_or_default().trim_start(),
      )));
      lines.push(Line::from(format!(
        "NZB Info URL: {}",
        nzb_info_url.unwrap_or_default().trim_start(),
      )));
      lines.push(Line::from(format!(
        "Download Client Name: {}",
        download_client_name.unwrap_or_default().trim_start(),
      )));
      lines.push(Line::from(format!(
        "Age: {}",
        format!("{} days", age.unwrap_or("0".to_owned())).trim_start(),
      )));
      lines.push(Line::from(format!(
        "Published Date: {}",
        published_date.unwrap_or_default().to_string().trim_start(),
      )));
    }
    SonarrHistoryEventType::DownloadFolderImported => {
      lines.push(Line::from(format!(
        "Dropped Path: {}",
        dropped_path.unwrap_or_default().trim_start(),
      )));
      lines.push(Line::from(format!(
        "Imported Path: {}",
        imported_path.unwrap_or_default().trim_start(),
      )));
    }
    SonarrHistoryEventType::DownloadFailed => {
      lines.push(Line::from(format!(
        "Message: {}",
        message.unwrap_or_default().trim_start(),
      )));
    }
    SonarrHistoryEventType::EpisodeFileDeleted => {
      lines.push(Line::from(format!(
        "Reason: {}",
        reason.unwrap_or_default().trim_start(),
      )));
    }
    SonarrHistoryEventType::EpisodeFileRenamed => {
      lines.push(Line::from(format!(
        "Source Path: {}",
        source_path.unwrap_or_default().trim_start(),
      )));
      lines.push(Line::from(format!(
        "Source Relative Path: {}",
        source_relative_path.unwrap_or_default().trim_start(),
      )));
      lines.push(Line::from(format!(
        "Destination Path: {}",
        path.unwrap_or_default().trim_start(),
      )));
      lines.push(Line::from(format!(
        "Destination Relative Path: {}",
        relative_path.unwrap_or_default().trim_start(),
      )));
    }
    _ => {
      lines.push(Line::from(String::new()));
      lines.push(Line::from("No additional data available"));
    }
  }

  lines
}
