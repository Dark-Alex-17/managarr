#[cfg(test)]
mod tests {
  use chrono::Utc;
  use ratatui::text::Line;

  use crate::models::radarr_models::RadarrHistoryEventType;
  use crate::models::radarr_models::{RadarrHistoryData, RadarrHistoryItem};
  use crate::ui::radarr_ui::radarr_ui_utils::create_history_event_details;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_create_grabbed_history_event_details() {
    let history_item = radarr_history_item(RadarrHistoryEventType::Grabbed);
    let RadarrHistoryItem {
      source_title,
      data,
      event_type,
      ..
    } = history_item.clone();
    let RadarrHistoryData {
      indexer,
      release_group,
      nzb_info_url,
      download_client,
      download_client_name,
      age,
      published_date,
      ..
    } = data;
    let expected_vec = vec![
      Line::from(format!("Source Title: {}", source_title.text.trim_start(),)),
      Line::from(format!("Event Type: {event_type}")),
      Line::from(format!(
        "Indexer: {}",
        indexer.unwrap_or_default().trim_start(),
      )),
      Line::from(format!(
        "Release Group: {}",
        release_group.unwrap_or_default().trim_start()
      )),
      Line::from(format!(
        "NZB Info URL: {}",
        nzb_info_url.unwrap_or_default().trim_start()
      )),
      Line::from(format!(
        "Download Client: {}",
        download_client
          .or(download_client_name)
          .unwrap_or_default()
          .trim_start()
      )),
      Line::from(format!(
        "Age: {} days",
        age.unwrap_or("0".to_owned()).trim_start(),
      )),
      Line::from(format!(
        "Published Date: {}",
        published_date.unwrap_or_default().to_string().trim_start()
      )),
    ];

    let history_details_vec = create_history_event_details(history_item);

    assert_eq!(expected_vec, history_details_vec);
  }

  #[test]
  fn test_create_download_folder_imported_history_event_details() {
    let history_item = radarr_history_item(RadarrHistoryEventType::DownloadFolderImported);
    let RadarrHistoryItem {
      source_title,
      data,
      event_type,
      ..
    } = history_item.clone();
    let RadarrHistoryData {
      dropped_path,
      imported_path,
      download_client,
      ..
    } = data;
    let expected_vec = vec![
      Line::from(format!("Source Title: {}", source_title.text.trim_start(),)),
      Line::from(format!("Event Type: {event_type}")),
      Line::from(format!(
        "Dropped Path: {}",
        dropped_path.unwrap_or_default().trim_start()
      )),
      Line::from(format!(
        "Imported Path: {}",
        imported_path.unwrap_or_default().trim_start()
      )),
      Line::from(format!(
        "Download Client: {}",
        download_client.unwrap_or_default().trim_start(),
      )),
    ];

    let history_details_vec = create_history_event_details(history_item);

    assert_eq!(expected_vec, history_details_vec);
  }

  #[test]
  fn test_create_download_failed_history_event_details() {
    let history_item = radarr_history_item(RadarrHistoryEventType::DownloadFailed);
    let RadarrHistoryItem {
      source_title,
      data,
      event_type,
      ..
    } = history_item.clone();
    let RadarrHistoryData {
      message,
      download_client,
      ..
    } = data;
    let expected_vec = vec![
      Line::from(format!("Source Title: {}", source_title.text.trim_start())),
      Line::from(format!("Event Type: {event_type}")),
      Line::from(format!(
        "Download Client: {}",
        download_client.unwrap_or_default().trim_start()
      )),
      Line::from(format!(
        "Message: {}",
        message.unwrap_or_default().trim_start()
      )),
    ];

    let history_details_vec = create_history_event_details(history_item);

    assert_eq!(expected_vec, history_details_vec);
  }

  #[test]
  fn test_create_movie_file_deleted_history_event_details() {
    let history_item = radarr_history_item(RadarrHistoryEventType::MovieFileDeleted);
    let RadarrHistoryItem {
      source_title,
      data,
      event_type,
      ..
    } = history_item.clone();
    let RadarrHistoryData { reason, .. } = data;
    let expected_vec = vec![
      Line::from(format!("Source Title: {}", source_title.text.trim_start(),)),
      Line::from(format!("Event Type: {event_type}")),
      Line::from(format!(
        "Reason: {}",
        reason.unwrap_or_default().trim_start(),
      )),
    ];

    let history_details_vec = create_history_event_details(history_item);

    assert_eq!(expected_vec, history_details_vec);
  }

  #[test]
  fn test_create_movie_file_renamed_history_event_details() {
    let history_item = radarr_history_item(RadarrHistoryEventType::MovieFileRenamed);
    let RadarrHistoryItem {
      source_title,
      data,
      event_type,
      ..
    } = history_item.clone();
    let RadarrHistoryData {
      source_path, path, ..
    } = data;
    let expected_vec = vec![
      Line::from(format!("Source Title: {}", source_title.text.trim_start(),)),
      Line::from(format!("Event Type: {event_type}")),
      Line::from(format!(
        "Source Path: {}",
        source_path.unwrap_or_default().trim_start(),
      )),
      Line::from(format!(
        "Destination Path: {}",
        path.unwrap_or_default().trim_start(),
      )),
    ];

    let history_details_vec = create_history_event_details(history_item);

    assert_eq!(expected_vec, history_details_vec);
  }

  #[test]
  fn test_create_no_data_history_event_details() {
    let history_item = radarr_history_item(RadarrHistoryEventType::Unknown);
    let RadarrHistoryItem {
      source_title,
      event_type,
      ..
    } = history_item.clone();
    let expected_vec = vec![
      Line::from(format!("Source Title: {}", source_title.text.trim_start(),)),
      Line::from(format!("Event Type: {event_type}")),
      Line::from(String::new()),
      Line::from("No additional data available"),
    ];

    let history_details_vec = create_history_event_details(history_item);

    assert_eq!(expected_vec, history_details_vec);
  }

  fn radarr_history_item(event_type: RadarrHistoryEventType) -> RadarrHistoryItem {
    RadarrHistoryItem {
      source_title: "test.source.title".into(),
      event_type,
      data: radarr_history_data(),
      ..RadarrHistoryItem::default()
    }
  }

  fn radarr_history_data() -> RadarrHistoryData {
    RadarrHistoryData {
      dropped_path: Some("/dropped/test".into()),
      imported_path: Some("/imported/test".into()),
      indexer: Some("Test Indexer".into()),
      release_group: Some("test release group".into()),
      nzb_info_url: Some("test url".into()),
      download_client: Some("test download client".into()),
      download_client_name: None,
      age: Some("1".into()),
      published_date: Some(Utc::now()),
      message: Some("test message".into()),
      reason: Some("test reason".into()),
      source_path: Some("/source/path".into()),
      path: Some("/path".into()),
    }
  }
}
