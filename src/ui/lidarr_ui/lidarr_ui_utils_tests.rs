#[cfg(test)]
mod tests {
  use chrono::Utc;
  use pretty_assertions::assert_eq;
  use ratatui::text::Line;

  use crate::models::lidarr_models::{
    LidarrHistoryData, LidarrHistoryEventType, LidarrHistoryItem,
  };
  use crate::models::servarr_models::{Quality, QualityWrapper};
  use crate::ui::lidarr_ui::lidarr_ui_utils::create_history_event_details;

  #[test]
  fn test_create_history_event_details_grabbed() {
    let history_item = lidarr_history_item(LidarrHistoryEventType::Grabbed);
    let LidarrHistoryItem {
      source_title,
      event_type,
      quality,
      date,
      data,
      ..
    } = history_item.clone();

    let result = create_history_event_details(history_item);

    assert_eq!(
      result[0],
      Line::from(format!("Source Title: {}", source_title.text))
    );
    assert_eq!(result[1], Line::from(format!("Event Type: {event_type}")));
    assert_eq!(
      result[2],
      Line::from(format!("Quality: {}", quality.quality.name))
    );
    assert_eq!(result[3], Line::from(format!("Date: {date}")));
    assert_eq!(
      result[4],
      Line::from(format!("Indexer: {}", data.indexer.unwrap()))
    );
    assert_eq!(
      result[5],
      Line::from(format!("NZB Info URL: {}", data.nzb_info_url.unwrap()))
    );
    assert_eq!(
      result[6],
      Line::from(format!("Release Group: {}", data.release_group.unwrap()))
    );
    assert_eq!(
      result[7],
      Line::from(format!("Age: {} days", data.age.unwrap()))
    );
    assert_eq!(
      result[8],
      Line::from(format!("Published Date: {}", data.published_date.unwrap()))
    );
    assert_eq!(
      result[9],
      Line::from(format!(
        "Download Client: {}",
        data.download_client_name.unwrap()
      ))
    );
    assert_eq!(result.len(), 10);
  }

  #[test]
  fn test_create_history_event_details_grabbed_uses_download_client_as_fallback() {
    let mut history_item = lidarr_history_item(LidarrHistoryEventType::Grabbed);
    history_item.data.download_client_name = None;
    history_item.data.download_client = Some("Fallback Client".to_owned());

    let result = create_history_event_details(history_item);

    assert_eq!(result[9], Line::from("Download Client: Fallback Client"));
  }

  #[test]
  fn test_create_history_event_details_download_imported() {
    let history_item = lidarr_history_item(LidarrHistoryEventType::DownloadImported);
    let LidarrHistoryItem {
      source_title,
      event_type,
      quality,
      date,
      data,
      ..
    } = history_item.clone();

    let result = create_history_event_details(history_item);

    assert_eq!(
      result[0],
      Line::from(format!("Source Title: {}", source_title.text))
    );
    assert_eq!(result[1], Line::from(format!("Event Type: {event_type}")));
    assert_eq!(
      result[2],
      Line::from(format!("Quality: {}", quality.quality.name))
    );
    assert_eq!(result[3], Line::from(format!("Date: {date}")));
    assert_eq!(
      result[4],
      Line::from(format!("Release Group: {}", data.release_group.unwrap()))
    );
    assert_eq!(result.len(), 5);
  }

  #[test]
  fn test_create_history_event_details_download_failed() {
    let history_item = lidarr_history_item(LidarrHistoryEventType::DownloadFailed);
    let LidarrHistoryItem {
      source_title,
      event_type,
      quality,
      date,
      data,
      ..
    } = history_item.clone();

    let result = create_history_event_details(history_item);

    assert_eq!(
      result[0],
      Line::from(format!("Source Title: {}", source_title.text))
    );
    assert_eq!(result[1], Line::from(format!("Event Type: {event_type}")));
    assert_eq!(
      result[2],
      Line::from(format!("Quality: {}", quality.quality.name))
    );
    assert_eq!(result[3], Line::from(format!("Date: {date}")));
    assert_eq!(
      result[4],
      Line::from(format!(
        "Download Client: {}",
        data.download_client_name.unwrap()
      ))
    );
    assert_eq!(
      result[5],
      Line::from(format!("Message: {}", data.message.unwrap()))
    );
    assert_eq!(
      result[6],
      Line::from(format!("Release Group: {}", data.release_group.unwrap()))
    );
    assert_eq!(
      result[7],
      Line::from(format!("Indexer: {}", data.indexer.unwrap()))
    );
    assert_eq!(result.len(), 8);
  }

  #[test]
  fn test_create_history_event_details_track_file_deleted() {
    let history_item = lidarr_history_item(LidarrHistoryEventType::TrackFileDeleted);
    let LidarrHistoryItem {
      source_title,
      event_type,
      quality,
      date,
      data,
      ..
    } = history_item.clone();

    let result = create_history_event_details(history_item);

    assert_eq!(
      result[0],
      Line::from(format!("Source Title: {}", source_title.text))
    );
    assert_eq!(result[1], Line::from(format!("Event Type: {event_type}")));
    assert_eq!(
      result[2],
      Line::from(format!("Quality: {}", quality.quality.name))
    );
    assert_eq!(result[3], Line::from(format!("Date: {date}")));
    assert_eq!(
      result[4],
      Line::from(format!("Reason: {}", data.reason.unwrap()))
    );
    assert_eq!(
      result[5],
      Line::from(format!("Release Group: {}", data.release_group.unwrap()))
    );
    assert_eq!(result.len(), 6);
  }

  #[test]
  fn test_create_history_event_details_track_file_imported() {
    let history_item = lidarr_history_item(LidarrHistoryEventType::TrackFileImported);
    let LidarrHistoryItem {
      source_title,
      event_type,
      quality,
      date,
      data,
      ..
    } = history_item.clone();

    let result = create_history_event_details(history_item);

    assert_eq!(
      result[0],
      Line::from(format!("Source Title: {}", source_title.text))
    );
    assert_eq!(result[1], Line::from(format!("Event Type: {event_type}")));
    assert_eq!(
      result[2],
      Line::from(format!("Quality: {}", quality.quality.name))
    );
    assert_eq!(result[3], Line::from(format!("Date: {date}")));
    assert_eq!(
      result[4],
      Line::from(format!("Dropped Path: {}", data.dropped_path.unwrap()))
    );
    assert_eq!(
      result[5],
      Line::from(format!("Imported Path: {}", data.imported_path.unwrap()))
    );
    assert_eq!(
      result[6],
      Line::from(format!(
        "Download Client: {}",
        data.download_client_name.unwrap()
      ))
    );
    assert_eq!(
      result[7],
      Line::from(format!("Release Group: {}", data.release_group.unwrap()))
    );
    assert_eq!(result.len(), 8);
  }

  #[test]
  fn test_create_history_event_details_track_file_renamed() {
    let history_item = lidarr_history_item(LidarrHistoryEventType::TrackFileRenamed);
    let LidarrHistoryItem {
      source_title,
      event_type,
      quality,
      date,
      data,
      ..
    } = history_item.clone();

    let result = create_history_event_details(history_item);

    assert_eq!(
      result[0],
      Line::from(format!("Source Title: {}", source_title.text))
    );
    assert_eq!(result[1], Line::from(format!("Event Type: {event_type}")));
    assert_eq!(
      result[2],
      Line::from(format!("Quality: {}", quality.quality.name))
    );
    assert_eq!(result[3], Line::from(format!("Date: {date}")));
    assert_eq!(
      result[4],
      Line::from(format!("Source Path: {}", data.source_path.unwrap()))
    );
    assert_eq!(
      result[5],
      Line::from(format!("Path: {}", data.path.unwrap()))
    );
    assert_eq!(
      result[6],
      Line::from(format!("Release Group: {}", data.release_group.unwrap()))
    );
    assert_eq!(result.len(), 7);
  }

  #[test]
  fn test_create_history_event_details_track_file_retagged() {
    let history_item = lidarr_history_item(LidarrHistoryEventType::TrackFileRetagged);
    let LidarrHistoryItem {
      source_title,
      event_type,
      quality,
      date,
      data,
      ..
    } = history_item.clone();

    let result = create_history_event_details(history_item);

    assert_eq!(
      result[0],
      Line::from(format!("Source Title: {}", source_title.text))
    );
    assert_eq!(result[1], Line::from(format!("Event Type: {event_type}")));
    assert_eq!(
      result[2],
      Line::from(format!("Quality: {}", quality.quality.name))
    );
    assert_eq!(result[3], Line::from(format!("Date: {date}")));
    assert_eq!(
      result[4],
      Line::from(format!("Release Group: {}", data.release_group.unwrap()))
    );
    assert_eq!(result.len(), 5);
  }

  #[test]
  fn test_create_history_event_details_album_import_incomplete() {
    let history_item = lidarr_history_item(LidarrHistoryEventType::AlbumImportIncomplete);
    let LidarrHistoryItem {
      source_title,
      event_type,
      quality,
      date,
      data,
      ..
    } = history_item.clone();

    let result = create_history_event_details(history_item);

    assert_eq!(
      result[0],
      Line::from(format!("Source Title: {}", source_title.text))
    );
    assert_eq!(result[1], Line::from(format!("Event Type: {event_type}")));
    assert_eq!(
      result[2],
      Line::from(format!("Quality: {}", quality.quality.name))
    );
    assert_eq!(result[3], Line::from(format!("Date: {date}")));
    assert_eq!(
      result[4],
      Line::from(format!(
        "Status Messages: {}",
        data.status_messages.unwrap()
      ))
    );
    assert_eq!(
      result[5],
      Line::from(format!("Release Group: {}", data.release_group.unwrap()))
    );
    assert_eq!(result.len(), 6);
  }

  #[test]
  fn test_create_history_event_details_unknown() {
    let history_item = lidarr_history_item(LidarrHistoryEventType::Unknown);
    let LidarrHistoryItem {
      source_title,
      event_type,
      quality,
      date,
      ..
    } = history_item.clone();

    let result = create_history_event_details(history_item);

    assert_eq!(
      result[0],
      Line::from(format!("Source Title: {}", source_title.text))
    );
    assert_eq!(result[1], Line::from(format!("Event Type: {event_type}")));
    assert_eq!(
      result[2],
      Line::from(format!("Quality: {}", quality.quality.name))
    );
    assert_eq!(result[3], Line::from(format!("Date: {date}")));
    assert_eq!(result[4], Line::from("No additional details available."));
    assert_eq!(result.len(), 5);
  }

  #[test]
  fn test_create_history_event_details_with_empty_optional_fields() {
    let mut history_item = lidarr_history_item(LidarrHistoryEventType::Grabbed);
    history_item.data = LidarrHistoryData::default();

    let result = create_history_event_details(history_item);

    assert_eq!(result[4], Line::from("Indexer: "));
    assert_eq!(result[5], Line::from("NZB Info URL: "));
    assert_eq!(result[6], Line::from("Release Group: "));
    assert_eq!(result[7], Line::from("Age: 0 days"));
    assert!(result[8].to_string().starts_with("Published Date:"));
    assert_eq!(result[9], Line::from("Download Client: "));
  }

  fn lidarr_history_item(event_type: LidarrHistoryEventType) -> LidarrHistoryItem {
    LidarrHistoryItem {
      id: 1,
      source_title: "Test Album - Artist Name".into(),
      album_id: 100,
      artist_id: 10,
      event_type,
      quality: QualityWrapper {
        quality: Quality {
          name: "FLAC".to_owned(),
        },
      },
      date: Utc::now(),
      data: lidarr_history_data(),
    }
  }

  fn lidarr_history_data() -> LidarrHistoryData {
    LidarrHistoryData {
      indexer: Some("Test Indexer".to_owned()),
      release_group: Some("Test Release Group".to_owned()),
      nzb_info_url: Some("https://test.url".to_owned()),
      download_client_name: Some("Test Download Client".to_owned()),
      download_client: Some("Fallback Download Client".to_owned()),
      age: Some("7".to_owned()),
      published_date: Some(Utc::now()),
      message: Some("Test failure message".to_owned()),
      reason: Some("Test deletion reason".to_owned()),
      dropped_path: Some("/downloads/completed/album".to_owned()),
      imported_path: Some("/music/artist/album".to_owned()),
      source_path: Some("/music/artist/old_album_name".to_owned()),
      path: Some("/music/artist/new_album_name".to_owned()),
      status_messages: Some("Missing tracks: 1, 2, 3".to_owned()),
    }
  }
}
