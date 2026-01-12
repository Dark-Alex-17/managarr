use crate::models::lidarr_models::{LidarrHistoryData, LidarrHistoryEventType, LidarrHistoryItem};
use ratatui::text::Line;

#[cfg(test)]
#[path = "lidarr_ui_utils_tests.rs"]
mod lidarr_ui_utils_tests;

pub(super) fn create_history_event_details(history_item: LidarrHistoryItem) -> Vec<Line<'static>> {
  let LidarrHistoryItem {
    source_title,
    event_type,
    quality,
    date,
    data,
    ..
  } = history_item;
  let LidarrHistoryData {
    indexer,
    nzb_info_url,
    release_group,
    age,
    published_date,
    download_client_name,
    download_client,
    message,
    reason,
    dropped_path,
    imported_path,
    source_path,
    path,
    status_messages,
  } = data;

  let mut lines = vec![
    Line::from(format!("Source Title: {}", source_title.text)),
    Line::from(format!("Event Type: {event_type}")),
    Line::from(format!("Quality: {}", quality.quality.name)),
    Line::from(format!("Date: {date}")),
  ];

  match event_type {
    LidarrHistoryEventType::Grabbed => {
      lines.push(Line::from(format!(
        "Indexer: {}",
        indexer.unwrap_or_default()
      )));
      lines.push(Line::from(format!(
        "NZB Info URL: {}",
        nzb_info_url.unwrap_or_default()
      )));
      lines.push(Line::from(format!(
        "Release Group: {}",
        release_group.unwrap_or_default()
      )));
      lines.push(Line::from(format!(
        "Age: {} days",
        age.unwrap_or("0".to_owned())
      )));
      lines.push(Line::from(format!(
        "Published Date: {}",
        published_date.unwrap_or_default()
      )));
      lines.push(Line::from(format!(
        "Download Client: {}",
        download_client_name.unwrap_or(download_client.unwrap_or_default())
      )));
    }
    LidarrHistoryEventType::DownloadImported => {
      lines.push(Line::from(format!(
        "Release Group: {}",
        release_group.unwrap_or_default()
      )));
    }
    LidarrHistoryEventType::DownloadFailed => {
      lines.push(Line::from(format!(
        "Download Client: {}",
        download_client_name.unwrap_or(download_client.unwrap_or_default())
      )));
      lines.push(Line::from(format!(
        "Message: {}",
        message.unwrap_or_default()
      )));
      lines.push(Line::from(format!(
        "Release Group: {}",
        release_group.unwrap_or_default()
      )));
      lines.push(Line::from(format!(
        "Indexer: {}",
        indexer.unwrap_or_default()
      )));
    }
    LidarrHistoryEventType::TrackFileDeleted => {
      lines.push(Line::from(format!(
        "Reason: {}",
        reason.unwrap_or_default()
      )));
      lines.push(Line::from(format!(
        "Release Group: {}",
        release_group.unwrap_or_default()
      )));
    }
    LidarrHistoryEventType::TrackFileImported => {
      lines.push(Line::from(format!(
        "Dropped Path: {}",
        dropped_path.unwrap_or_default()
      )));
      lines.push(Line::from(format!(
        "Imported Path: {}",
        imported_path.unwrap_or_default()
      )));
      lines.push(Line::from(format!(
        "Download Client: {}",
        download_client_name.unwrap_or(download_client.unwrap_or_default())
      )));
      lines.push(Line::from(format!(
        "Release Group: {}",
        release_group.unwrap_or_default()
      )));
    }
    LidarrHistoryEventType::TrackFileRenamed => {
      lines.push(Line::from(format!(
        "Source Path: {}",
        source_path.unwrap_or_default()
      )));
      lines.push(Line::from(format!("Path: {}", path.unwrap_or_default())));
      lines.push(Line::from(format!(
        "Release Group: {}",
        release_group.unwrap_or_default()
      )));
    }
    LidarrHistoryEventType::TrackFileRetagged => {
      lines.push(Line::from(format!(
        "Release Group: {}",
        release_group.unwrap_or_default()
      )));
    }
    LidarrHistoryEventType::AlbumImportIncomplete => {
      lines.push(Line::from(format!(
        "Status Messages: {}",
        status_messages.unwrap_or_default()
      )));
      lines.push(Line::from(format!(
        "Release Group: {}",
        release_group.unwrap_or_default()
      )));
    }
    _ => {
      lines.push(Line::from("No additional details available.".to_owned()));
    }
  }

  lines
}
