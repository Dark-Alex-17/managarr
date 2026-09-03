#[cfg(test)]
mod tests {
  use chrono::Utc;
  use pretty_assertions::assert_eq;
  use ratatui::text::Line;

  use crate::models::readarr_models::{
    ReadarrHistoryData, ReadarrHistoryEventType, ReadarrHistoryItem,
  };
  use crate::models::servarr_models::{Quality, QualityWrapper};
  use crate::ui::readarr_ui::readarr_ui_utils::create_history_event_details;

  #[test]
  fn test_create_history_event_details_grabbed() {
    let history_item = readarr_history_item(ReadarrHistoryEventType::Grabbed);
    let ReadarrHistoryItem {
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
      Line::from(format!("Source Title: {}", source_title.text.trim_start()))
    );
    assert_eq!(result[1], Line::from(format!("Event Type: {event_type}")));
    assert_eq!(
      result[2],
      Line::from(format!("Quality: {}", quality.quality.name.trim_start()))
    );
    assert_eq!(result[3], Line::from(format!("Date: {date}")));
    assert_eq!(
      result[4],
      Line::from(format!("Indexer: {}", data.indexer.unwrap().trim_start()))
    );
    assert_eq!(
      result[5],
      Line::from(format!(
        "NZB Info URL: {}",
        data.nzb_info_url.unwrap().trim_start()
      ))
    );
    assert_eq!(
      result[6],
      Line::from(format!(
        "Release Group: {}",
        data.release_group.unwrap().trim_start()
      ))
    );
    assert_eq!(
      result[7],
      Line::from(format!("Age: {} days", data.age.unwrap().trim_start()))
    );
    assert_eq!(
      result[8],
      Line::from(format!("Published Date: {}", data.published_date.unwrap()))
    );
    assert_eq!(
      result[9],
      Line::from(format!(
        "Download Client: {}",
        data.download_client_name.unwrap().trim_start()
      ))
    );
    assert_eq!(result.len(), 10);
  }

  #[test]
  fn test_create_history_event_details_grabbed_uses_download_client_as_fallback() {
    let mut history_item = readarr_history_item(ReadarrHistoryEventType::Grabbed);
    history_item.data.download_client_name = None;
    history_item.data.download_client = Some("\nFallback Client".to_owned());

    let result = create_history_event_details(history_item);

    assert_eq!(result[9], Line::from("Download Client: Fallback Client"));
  }

  #[test]
  fn test_create_history_event_details_download_imported() {
    let history_item = readarr_history_item(ReadarrHistoryEventType::DownloadImported);
    let ReadarrHistoryItem {
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
      Line::from(format!("Source Title: {}", source_title.text.trim_start()))
    );
    assert_eq!(result[1], Line::from(format!("Event Type: {event_type}")));
    assert_eq!(
      result[2],
      Line::from(format!("Quality: {}", quality.quality.name.trim_start()))
    );
    assert_eq!(result[3], Line::from(format!("Date: {date}")));
    assert_eq!(
      result[4],
      Line::from(format!(
        "Release Group: {}",
        data.release_group.unwrap().trim_start()
      ))
    );
    assert_eq!(result.len(), 5);
  }

  #[test]
  fn test_create_history_event_details_download_failed() {
    let history_item = readarr_history_item(ReadarrHistoryEventType::DownloadFailed);
    let ReadarrHistoryItem {
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
      Line::from(format!("Source Title: {}", source_title.text.trim_start()))
    );
    assert_eq!(result[1], Line::from(format!("Event Type: {event_type}")));
    assert_eq!(
      result[2],
      Line::from(format!("Quality: {}", quality.quality.name.trim_start()))
    );
    assert_eq!(result[3], Line::from(format!("Date: {date}")));
    assert_eq!(
      result[4],
      Line::from(format!(
        "Download Client: {}",
        data.download_client_name.unwrap().trim_start()
      ))
    );
    assert_eq!(
      result[5],
      Line::from(format!("Message: {}", data.message.unwrap().trim_start()))
    );
    assert_eq!(
      result[6],
      Line::from(format!(
        "Release Group: {}",
        data.release_group.unwrap().trim_start()
      ))
    );
    assert_eq!(
      result[7],
      Line::from(format!("Indexer: {}", data.indexer.unwrap().trim_start()))
    );
    assert_eq!(result.len(), 8);
  }

  #[test]
  fn test_create_history_event_details_book_file_deleted() {
    let history_item = readarr_history_item(ReadarrHistoryEventType::BookFileDeleted);
    let ReadarrHistoryItem {
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
      Line::from(format!("Source Title: {}", source_title.text.trim_start()))
    );
    assert_eq!(result[1], Line::from(format!("Event Type: {event_type}")));
    assert_eq!(
      result[2],
      Line::from(format!("Quality: {}", quality.quality.name.trim_start()))
    );
    assert_eq!(result[3], Line::from(format!("Date: {date}")));
    assert_eq!(
      result[4],
      Line::from(format!("Reason: {}", data.reason.unwrap().trim_start()))
    );
    assert_eq!(
      result[5],
      Line::from(format!(
        "Release Group: {}",
        data.release_group.unwrap().trim_start()
      ))
    );
    assert_eq!(result.len(), 6);
  }

  #[test]
  fn test_create_history_event_details_book_file_imported() {
    let history_item = readarr_history_item(ReadarrHistoryEventType::BookFileImported);
    let ReadarrHistoryItem {
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
      Line::from(format!("Source Title: {}", source_title.text.trim_start()))
    );
    assert_eq!(result[1], Line::from(format!("Event Type: {event_type}")));
    assert_eq!(
      result[2],
      Line::from(format!("Quality: {}", quality.quality.name.trim_start()))
    );
    assert_eq!(result[3], Line::from(format!("Date: {date}")));
    assert_eq!(
      result[4],
      Line::from(format!(
        "Dropped Path: {}",
        data.dropped_path.unwrap().trim_start()
      ))
    );
    assert_eq!(
      result[5],
      Line::from(format!(
        "Imported Path: {}",
        data.imported_path.unwrap().trim_start()
      ))
    );
    assert_eq!(
      result[6],
      Line::from(format!(
        "Download Client: {}",
        data.download_client_name.unwrap().trim_start()
      ))
    );
    assert_eq!(
      result[7],
      Line::from(format!(
        "Release Group: {}",
        data.release_group.unwrap().trim_start()
      ))
    );
    assert_eq!(result.len(), 8);
  }

  #[test]
  fn test_create_history_event_details_book_file_renamed() {
    let history_item = readarr_history_item(ReadarrHistoryEventType::BookFileRenamed);
    let ReadarrHistoryItem {
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
      Line::from(format!("Source Title: {}", source_title.text.trim_start()))
    );
    assert_eq!(result[1], Line::from(format!("Event Type: {event_type}")));
    assert_eq!(
      result[2],
      Line::from(format!("Quality: {}", quality.quality.name.trim_start()))
    );
    assert_eq!(result[3], Line::from(format!("Date: {date}")));
    assert_eq!(
      result[4],
      Line::from(format!(
        "Source Path: {}",
        data.source_path.unwrap().trim_start()
      ))
    );
    assert_eq!(
      result[5],
      Line::from(format!("Path: {}", data.path.unwrap().trim_start()))
    );
    assert_eq!(
      result[6],
      Line::from(format!(
        "Release Group: {}",
        data.release_group.unwrap().trim_start()
      ))
    );
    assert_eq!(result.len(), 7);
  }

  #[test]
  fn test_create_history_event_details_book_file_retagged() {
    let history_item = readarr_history_item(ReadarrHistoryEventType::BookFileRetagged);
    let ReadarrHistoryItem {
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
      Line::from(format!("Source Title: {}", source_title.text.trim_start()))
    );
    assert_eq!(result[1], Line::from(format!("Event Type: {event_type}")));
    assert_eq!(
      result[2],
      Line::from(format!("Quality: {}", quality.quality.name.trim_start()))
    );
    assert_eq!(result[3], Line::from(format!("Date: {date}")));
    assert_eq!(
      result[4],
      Line::from(format!(
        "Release Group: {}",
        data.release_group.unwrap().trim_start()
      ))
    );
    assert_eq!(result.len(), 5);
  }

  #[test]
  fn test_create_history_event_details_book_import_incomplete() {
    let history_item = readarr_history_item(ReadarrHistoryEventType::BookImportIncomplete);
    let ReadarrHistoryItem {
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
      Line::from(format!("Source Title: {}", source_title.text.trim_start()))
    );
    assert_eq!(result[1], Line::from(format!("Event Type: {event_type}")));
    assert_eq!(
      result[2],
      Line::from(format!("Quality: {}", quality.quality.name.trim_start()))
    );
    assert_eq!(result[3], Line::from(format!("Date: {date}")));
    assert_eq!(
      result[4],
      Line::from(format!(
        "Status Messages: {}",
        data.status_messages.unwrap().trim_start()
      ))
    );
    assert_eq!(
      result[5],
      Line::from(format!(
        "Release Group: {}",
        data.release_group.unwrap().trim_start()
      ))
    );
    assert_eq!(result.len(), 6);
  }

  #[test]
  fn test_create_history_event_details_unknown() {
    let history_item = readarr_history_item(ReadarrHistoryEventType::Unknown);
    let ReadarrHistoryItem {
      source_title,
      event_type,
      quality,
      date,
      ..
    } = history_item.clone();

    let result = create_history_event_details(history_item);

    assert_eq!(
      result[0],
      Line::from(format!("Source Title: {}", source_title.text.trim_start()))
    );
    assert_eq!(result[1], Line::from(format!("Event Type: {event_type}")));
    assert_eq!(
      result[2],
      Line::from(format!("Quality: {}", quality.quality.name.trim_start()))
    );
    assert_eq!(result[3], Line::from(format!("Date: {date}")));
    assert_eq!(result[4], Line::from("No additional details available."));
    assert_eq!(result.len(), 5);
  }

  #[test]
  fn test_create_history_event_details_author_folder_imported_has_no_additional_details() {
    let history_item = readarr_history_item(ReadarrHistoryEventType::AuthorFolderImported);

    let result = create_history_event_details(history_item);

    assert_eq!(result[4], Line::from("No additional details available."));
    assert_eq!(result.len(), 5);
  }

  #[test]
  fn test_create_history_event_details_download_ignored_has_no_additional_details() {
    let history_item = readarr_history_item(ReadarrHistoryEventType::DownloadIgnored);

    let result = create_history_event_details(history_item);

    assert_eq!(result[4], Line::from("No additional details available."));
    assert_eq!(result.len(), 5);
  }

  #[test]
  fn test_create_history_event_details_with_empty_optional_fields() {
    let mut history_item = readarr_history_item(ReadarrHistoryEventType::Grabbed);
    history_item.data = ReadarrHistoryData::default();

    let result = create_history_event_details(history_item);

    assert_eq!(result[4], Line::from("Indexer: "));
    assert_eq!(result[5], Line::from("NZB Info URL: "));
    assert_eq!(result[6], Line::from("Release Group: "));
    assert_eq!(result[7], Line::from("Age: 0 days"));
    assert!(result[8].to_string().starts_with("Published Date:"));
    assert_eq!(result[9], Line::from("Download Client: "));
  }

  fn readarr_history_item(event_type: ReadarrHistoryEventType) -> ReadarrHistoryItem {
    ReadarrHistoryItem {
      id: 1,
      source_title: "\nTest Book - Author Name".into(),
      author_id: 10,
      book_id: 100,
      event_type,
      quality: QualityWrapper {
        quality: Quality {
          name: "\nEPUB".to_owned(),
        },
      },
      date: Utc::now(),
      data: readarr_history_data(),
    }
  }

  fn readarr_history_data() -> ReadarrHistoryData {
    ReadarrHistoryData {
      indexer: Some("\nTest Indexer".to_owned()),
      release_group: Some("\nTest Release Group".to_owned()),
      nzb_info_url: Some("\nhttps://test.url".to_owned()),
      download_client_name: Some("\nTest Download Client".to_owned()),
      download_client: Some("\nFallback Download Client".to_owned()),
      age: Some("\n7".to_owned()),
      published_date: Some(Utc::now()),
      message: Some("\nTest failure message".to_owned()),
      reason: Some("\nTest deletion reason".to_owned()),
      dropped_path: Some("\n/downloads/completed/book".to_owned()),
      imported_path: Some("\n/books/author/book".to_owned()),
      source_path: Some("\n/books/author/old_book_name".to_owned()),
      path: Some("\n/books/author/new_book_name".to_owned()),
      status_messages: Some("\nMissing chapters: 1, 2, 3".to_owned()),
    }
  }
}
