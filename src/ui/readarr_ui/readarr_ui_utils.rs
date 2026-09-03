use crate::models::readarr_models::{
  ReadarrHistoryData, ReadarrHistoryEventType, ReadarrHistoryItem,
};
use ratatui::text::Line;

#[cfg(test)]
#[path = "readarr_ui_utils_tests.rs"]
mod readarr_ui_utils_tests;

pub(super) fn create_history_event_details(history_item: ReadarrHistoryItem) -> Vec<Line<'static>> {
  let ReadarrHistoryItem {
    source_title,
    event_type,
    quality,
    date,
    data,
    ..
  } = history_item;
  let ReadarrHistoryData {
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
    Line::from(format!("Source Title: {}", source_title.text.trim_start())),
    Line::from(format!("Event Type: {event_type}")),
    Line::from(format!("Quality: {}", quality.quality.name.trim_start())),
    Line::from(format!("Date: {date}")),
  ];

  match event_type {
    ReadarrHistoryEventType::Grabbed => {
      lines.push(Line::from(format!(
        "Indexer: {}",
        indexer.unwrap_or_default().trim_start()
      )));
      lines.push(Line::from(format!(
        "NZB Info URL: {}",
        nzb_info_url.unwrap_or_default().trim_start()
      )));
      lines.push(Line::from(format!(
        "Release Group: {}",
        release_group.unwrap_or_default().trim_start()
      )));
      lines.push(Line::from(format!(
        "Age: {} days",
        age.unwrap_or("0".to_owned()).trim_start()
      )));
      lines.push(Line::from(format!(
        "Published Date: {}",
        published_date.unwrap_or_default()
      )));
      lines.push(Line::from(format!(
        "Download Client: {}",
        download_client_name
          .unwrap_or(download_client.unwrap_or_default())
          .trim_start()
      )));
    }
    ReadarrHistoryEventType::DownloadImported => {
      lines.push(Line::from(format!(
        "Release Group: {}",
        release_group.unwrap_or_default().trim_start()
      )));
    }
    ReadarrHistoryEventType::DownloadFailed => {
      lines.push(Line::from(format!(
        "Download Client: {}",
        download_client_name
          .unwrap_or(download_client.unwrap_or_default())
          .trim_start()
      )));
      lines.push(Line::from(format!(
        "Message: {}",
        message.unwrap_or_default().trim_start()
      )));
      lines.push(Line::from(format!(
        "Release Group: {}",
        release_group.unwrap_or_default().trim_start()
      )));
      lines.push(Line::from(format!(
        "Indexer: {}",
        indexer.unwrap_or_default().trim_start()
      )));
    }
    ReadarrHistoryEventType::BookFileDeleted => {
      lines.push(Line::from(format!(
        "Reason: {}",
        reason.unwrap_or_default().trim_start()
      )));
      lines.push(Line::from(format!(
        "Release Group: {}",
        release_group.unwrap_or_default().trim_start()
      )));
    }
    ReadarrHistoryEventType::BookFileImported => {
      lines.push(Line::from(format!(
        "Dropped Path: {}",
        dropped_path.unwrap_or_default().trim_start()
      )));
      lines.push(Line::from(format!(
        "Imported Path: {}",
        imported_path.unwrap_or_default().trim_start()
      )));
      lines.push(Line::from(format!(
        "Download Client: {}",
        download_client_name
          .unwrap_or(download_client.unwrap_or_default())
          .trim_start()
      )));
      lines.push(Line::from(format!(
        "Release Group: {}",
        release_group.unwrap_or_default().trim_start()
      )));
    }
    ReadarrHistoryEventType::BookFileRenamed => {
      lines.push(Line::from(format!(
        "Source Path: {}",
        source_path.unwrap_or_default().trim_start()
      )));
      lines.push(Line::from(format!(
        "Path: {}",
        path.unwrap_or_default().trim_start()
      )));
      lines.push(Line::from(format!(
        "Release Group: {}",
        release_group.unwrap_or_default().trim_start()
      )));
    }
    ReadarrHistoryEventType::BookFileRetagged => {
      lines.push(Line::from(format!(
        "Release Group: {}",
        release_group.unwrap_or_default().trim_start()
      )));
    }
    ReadarrHistoryEventType::BookImportIncomplete => {
      lines.push(Line::from(format!(
        "Status Messages: {}",
        status_messages.unwrap_or_default().trim_start()
      )));
      lines.push(Line::from(format!(
        "Release Group: {}",
        release_group.unwrap_or_default().trim_start()
      )));
    }
    _ => {
      lines.push(Line::from("No additional details available.".to_owned()));
    }
  }

  lines
}
