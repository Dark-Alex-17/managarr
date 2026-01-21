#[cfg(test)]
mod tests {
  use chrono::Utc;
  use ratatui::text::Line;

  use crate::models::sonarr_models::SonarrHistoryEventType;
  use crate::models::sonarr_models::{SonarrHistoryData, SonarrHistoryItem};
  use crate::ui::sonarr_ui::sonarr_ui_utils::create_history_event_details;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_create_grabbed_history_event_details() {
    let history_item = sonarr_history_item(SonarrHistoryEventType::Grabbed);
    let SonarrHistoryItem {
      source_title,
      data,
      event_type,
      ..
    } = history_item.clone();
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
    let expected_vec = vec![
      Line::from(format!("Source Title: {}", source_title.text.trim_start())),
      Line::from(format!("Event Type: {event_type}")),
      Line::from(format!(
        "Indexer: {}",
        indexer.unwrap_or_default().trim_start()
      )),
      Line::from(format!(
        "Release Group: {}",
        release_group.unwrap_or_default().trim_start()
      )),
      Line::from(format!(
        "Series Match Type: {}",
        series_match_type.unwrap_or_default().trim_start()
      )),
      Line::from(format!(
        "NZB Info URL: {}",
        nzb_info_url.unwrap_or_default().trim_start()
      )),
      Line::from(format!(
        "Download Client Name: {}",
        download_client_name.unwrap_or_default().trim_start()
      )),
      Line::from(format!(
        "Age: {}",
        format!("{} days", age.unwrap_or("0".to_owned())).trim_start()
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
    let history_item = sonarr_history_item(SonarrHistoryEventType::DownloadFolderImported);
    let SonarrHistoryItem {
      source_title,
      data,
      event_type,
      ..
    } = history_item.clone();
    let SonarrHistoryData {
      dropped_path,
      imported_path,
      ..
    } = data;
    let expected_vec = vec![
      Line::from(format!("Source Title: {}", source_title.text.trim_start())),
      Line::from(format!("Event Type: {event_type}")),
      Line::from(format!(
        "Dropped Path: {}",
        dropped_path.unwrap_or_default().trim_start()
      )),
      Line::from(format!(
        "Imported Path: {}",
        imported_path.unwrap_or_default().trim_start()
      )),
    ];

    let history_details_vec = create_history_event_details(history_item);

    assert_eq!(expected_vec, history_details_vec);
  }

  #[test]
  fn test_create_download_failed_history_event_details() {
    let history_item = sonarr_history_item(SonarrHistoryEventType::DownloadFailed);
    let SonarrHistoryItem {
      source_title,
      data,
      event_type,
      ..
    } = history_item.clone();
    let SonarrHistoryData { message, .. } = data;
    let expected_vec = vec![
      Line::from(format!("Source Title: {}", source_title.text.trim_start())),
      Line::from(format!("Event Type: {event_type}")),
      Line::from(format!(
        "Message: {}",
        message.unwrap_or_default().trim_start()
      )),
    ];

    let history_details_vec = create_history_event_details(history_item);

    assert_eq!(expected_vec, history_details_vec);
  }

  #[test]
  fn test_create_episode_file_deleted_history_event_details() {
    let history_item = sonarr_history_item(SonarrHistoryEventType::EpisodeFileDeleted);
    let SonarrHistoryItem {
      source_title,
      data,
      event_type,
      ..
    } = history_item.clone();
    let SonarrHistoryData { reason, .. } = data;
    let expected_vec = vec![
      Line::from(format!("Source Title: {}", source_title.text.trim_start())),
      Line::from(format!("Event Type: {event_type}")),
      Line::from(format!(
        "Reason: {}",
        reason.unwrap_or_default().trim_start()
      )),
    ];

    let history_details_vec = create_history_event_details(history_item);

    assert_eq!(expected_vec, history_details_vec);
  }

  #[test]
  fn test_create_episode_file_renamed_history_event_details() {
    let history_item = sonarr_history_item(SonarrHistoryEventType::EpisodeFileRenamed);
    let SonarrHistoryItem {
      source_title,
      data,
      event_type,
      ..
    } = history_item.clone();
    let SonarrHistoryData {
      source_path,
      source_relative_path,
      path,
      relative_path,
      ..
    } = data;
    let expected_vec = vec![
      Line::from(format!("Source Title: {}", source_title.text.trim_start())),
      Line::from(format!("Event Type: {event_type}")),
      Line::from(format!(
        "Source Path: {}",
        source_path.unwrap_or_default().trim_start()
      )),
      Line::from(format!(
        "Source Relative Path: {}",
        source_relative_path.unwrap_or_default().trim_start()
      )),
      Line::from(format!(
        "Destination Path: {}",
        path.unwrap_or_default().trim_start()
      )),
      Line::from(format!(
        "Destination Relative Path: {}",
        relative_path.unwrap_or_default().trim_start()
      )),
    ];

    let history_details_vec = create_history_event_details(history_item);

    assert_eq!(expected_vec, history_details_vec);
  }

  #[test]
  fn test_create_no_data_history_event_details() {
    let history_item = sonarr_history_item(SonarrHistoryEventType::Unknown);
    let SonarrHistoryItem {
      source_title,
      event_type,
      ..
    } = history_item.clone();
    let expected_vec = vec![
      Line::from(format!("Source Title: {}", source_title.text.trim_start())),
      Line::from(format!("Event Type: {event_type}")),
      Line::from(String::new()),
      Line::from("No additional data available"),
    ];

    let history_details_vec = create_history_event_details(history_item);

    assert_eq!(expected_vec, history_details_vec);
  }

  fn sonarr_history_item(event_type: SonarrHistoryEventType) -> SonarrHistoryItem {
    SonarrHistoryItem {
      source_title: "\ntest.source.title".into(),
      event_type,
      data: sonarr_history_data(),
      ..SonarrHistoryItem::default()
    }
  }

  fn sonarr_history_data() -> SonarrHistoryData {
    SonarrHistoryData {
      dropped_path: Some("\n/dropped/test".into()),
      imported_path: Some("\n/imported/test".into()),
      indexer: Some("\nTest Indexer".into()),
      release_group: Some("\ntest release group".into()),
      series_match_type: Some("\ntest match type".into()),
      nzb_info_url: Some("\ntest url".into()),
      download_client_name: Some("\ntest download client".into()),
      age: Some("\n1".into()),
      published_date: Some(Utc::now()),
      message: Some("\ntest message".into()),
      reason: Some("\ntest reason".into()),
      source_path: Some("\n/source/path".into()),
      source_relative_path: Some("\n/relative/source/path".into()),
      path: Some("\n/path".into()),
      relative_path: Some("\n/relative/path".into()),
    }
  }
}
