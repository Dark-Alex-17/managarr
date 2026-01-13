use ratatui::text::Line;

use crate::models::radarr_models::{RadarrHistoryData, RadarrHistoryEventType, RadarrHistoryItem};

#[cfg(test)]
#[path = "radarr_ui_utils_tests.rs"]
mod radarr_ui_utils_tests;

pub(super) fn create_history_event_details(history_item: RadarrHistoryItem) -> Vec<Line<'static>> {
  let RadarrHistoryItem {
    source_title,
    event_type,
    data,
    ..
  } = history_item;
  let RadarrHistoryData {
    indexer,
    release_group,
    nzb_info_url,
    download_client,
    download_client_name,
    age,
    published_date,
    dropped_path,
    imported_path,
    message,
    reason,
    source_path,
    path,
    ..
  } = data;

  let mut lines = vec![
    Line::from(format!("Source Title: {}", source_title.text.trim_start())),
    Line::from(format!("Event Type: {}", event_type)),
  ];

  match event_type {
    RadarrHistoryEventType::Grabbed => {
      lines.push(Line::from(format!(
        "Indexer: {}",
        indexer.unwrap_or_default().trim_start(),
      )));
      lines.push(Line::from(format!(
        "Release Group: {}",
        release_group.unwrap_or_default().trim_start(),
      )));
      lines.push(Line::from(format!(
        "NZB Info URL: {}",
        nzb_info_url.unwrap_or_default().trim_start(),
      )));
      lines.push(Line::from(format!(
        "Download Client: {}",
        download_client
          .or(download_client_name)
          .unwrap_or_default()
          .trim_start()
      )));
      lines.push(Line::from(format!(
        "Age: {} days",
        age.unwrap_or("0".to_owned()).trim_start(),
      )));
      lines.push(Line::from(format!(
        "Published Date: {}",
        published_date.unwrap_or_default().to_string().trim_start(),
      )));
    }
    RadarrHistoryEventType::DownloadFolderImported => {
      lines.push(Line::from(format!(
        "Dropped Path: {}",
        dropped_path.unwrap_or_default().trim_start(),
      )));
      lines.push(Line::from(format!(
        "Imported Path: {}",
        imported_path.unwrap_or_default().trim_start(),
      )));
      lines.push(Line::from(format!(
        "Download Client: {}",
        download_client
          .or(download_client_name)
          .unwrap_or_default()
          .trim_start()
      )));
    }
    RadarrHistoryEventType::DownloadFailed => {
      lines.push(Line::from(format!(
        "Download Client: {}",
        download_client
          .or(download_client_name)
          .unwrap_or_default()
          .trim_start(),
      )));
      lines.push(Line::from(format!(
        "Message: {}",
        message.unwrap_or_default().trim_start(),
      )));
    }
    RadarrHistoryEventType::MovieFileDeleted => {
      lines.push(Line::from(format!(
        "Reason: {}",
        reason.unwrap_or_default().trim_start(),
      )));
    }
    RadarrHistoryEventType::MovieFileRenamed => {
      lines.push(Line::from(format!(
        "Source Path: {}",
        source_path.unwrap_or_default().trim_start(),
      )));
      lines.push(Line::from(format!(
        "Destination Path: {}",
        path.unwrap_or_default().trim_start(),
      )));
    }
    _ => {
      lines.push(Line::from(String::new()));
      lines.push(Line::from("No additional data available"));
    }
  }

  lines
}
