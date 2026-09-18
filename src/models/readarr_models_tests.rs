#[cfg(test)]
mod tests {
  use chrono::Utc;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use serde::de::DeserializeOwned;
  use serde_json::{Value, json};

  use crate::models::readarr_models::{
    AddAuthorSearchResult, BlocklistItem, BlocklistResponse, Book, BookFile, DownloadRecord,
    DownloadsResponse, Edition, MediaInfo, MonitorType, NewItemMonitorType,
    ReadarrHistoryEventType, ReadarrHistoryItem, ReadarrHistoryWrapper, ReadarrRelease,
    ReadarrTask, ReadarrTaskName,
  };
  use crate::models::servarr_models::{
    DiskSpace, DownloadStatus, HostConfig, Indexer, IndexerSettings, IndexerTestResult, Log,
    LogResponse, MetadataProfile, QualityProfile, QueueEvent, RootFolder, SecurityConfig,
    SystemStatus, Tag, Update,
  };
  use crate::models::{
    Serdeable,
    readarr_models::{Author, AuthorStatistics, AuthorStatus, Ratings, ReadarrSerdeable},
  };
  use crate::network::readarr_network::readarr_network_test_utils::test_utils::{
    ADD_AUTHOR_SEARCH_RESULT_JSON, AUTHOR_JSON, download_record,
  };

  #[test]
  fn test_author_status_default() {
    assert_eq!(AuthorStatus::default(), AuthorStatus::Continuing);
  }

  #[test]
  fn test_new_item_monitor_type_display() {
    assert_str_eq!(NewItemMonitorType::All.to_string(), "all");
    assert_str_eq!(NewItemMonitorType::None.to_string(), "none");
    assert_str_eq!(NewItemMonitorType::New.to_string(), "new");
  }

  #[test]
  fn test_new_item_monitor_type_to_display_str() {
    assert_str_eq!(NewItemMonitorType::All.to_display_str(), "All Books");
    assert_str_eq!(NewItemMonitorType::None.to_display_str(), "No New Books");
    assert_str_eq!(NewItemMonitorType::New.to_display_str(), "New Books");
  }

  #[test]
  fn test_monitor_type_display() {
    assert_str_eq!(MonitorType::All.to_string(), "all");
    assert_str_eq!(MonitorType::Future.to_string(), "future");
    assert_str_eq!(MonitorType::Missing.to_string(), "missing");
    assert_str_eq!(MonitorType::Existing.to_string(), "existing");
    assert_str_eq!(MonitorType::First.to_string(), "first");
    assert_str_eq!(MonitorType::Latest.to_string(), "latest");
    assert_str_eq!(MonitorType::None.to_string(), "none");
    assert_str_eq!(MonitorType::Unknown.to_string(), "unknown");
  }

  #[test]
  fn test_monitor_type_to_display_str() {
    assert_str_eq!(MonitorType::All.to_display_str(), "All Books");
    assert_str_eq!(MonitorType::Future.to_display_str(), "Future Books");
    assert_str_eq!(MonitorType::Missing.to_display_str(), "Missing Books");
    assert_str_eq!(MonitorType::Existing.to_display_str(), "Existing Books");
    assert_str_eq!(MonitorType::First.to_display_str(), "First Book");
    assert_str_eq!(MonitorType::Latest.to_display_str(), "Latest Book");
    assert_str_eq!(MonitorType::None.to_display_str(), "None");
    assert_str_eq!(MonitorType::Unknown.to_display_str(), "Unknown");
  }

  #[test]
  fn test_readarr_serdeable_from() {
    let readarr_serdeable = ReadarrSerdeable::Value(json!({}));

    let serdeable: Serdeable = Serdeable::from(readarr_serdeable.clone());

    assert_eq!(serdeable, Serdeable::Readarr(readarr_serdeable));
  }

  #[test]
  fn test_readarr_serdeable_from_unit() {
    let readarr_serdeable = ReadarrSerdeable::from(());

    assert_eq!(readarr_serdeable, ReadarrSerdeable::Value(json!({})));
  }

  #[test]
  fn test_readarr_serdeable_from_value() {
    let value = json!({"test": "test"});

    let readarr_serdeable: ReadarrSerdeable = value.clone().into();

    assert_eq!(readarr_serdeable, ReadarrSerdeable::Value(value));
  }

  #[test]
  fn test_readarr_serdeable_from_authors() {
    let authors = vec![Author {
      id: 1,
      ..Author::default()
    }];

    let readarr_serdeable: ReadarrSerdeable = authors.clone().into();

    assert_eq!(readarr_serdeable, ReadarrSerdeable::Authors(authors));
  }

  #[test]
  fn test_author_deserialization() {
    let author_json = json!({
      "id": 1,
      "authorName": "Test Author",
      "foreignAuthorId": "test-foreign-id",
      "status": "continuing",
      "overview": "Test overview",
      "authorType": "Person",
      "disambiguation": "American Novelist",
      "path": "/nfs/books/Test Author",
      "rootFolderPath": "/nfs/books/",
      "qualityProfileId": 1,
      "metadataProfileId": 1,
      "monitored": true,
      "monitorNewItems": "all",
      "genres": ["Fantasy", "Fiction"],
      "tags": [1, 2],
      "ratings": {
        "votes": 100,
        "value": 4.5,
        "popularity": 450.0
      },
      "statistics": {
        "bookCount": 14,
        "bookFileCount": 5,
        "totalBookCount": 14,
        "availableBookCount": 5,
        "sizeOnDisk": 51783488,
        "percentOfBooks": 35.71
      }
    });

    let author: Author = serde_json::from_value(author_json).unwrap();

    assert_eq!(author.id, 1);
    assert_str_eq!(author.author_name.text, "Test Author");
    assert_str_eq!(author.foreign_author_id, "test-foreign-id");
    assert_eq!(author.status, AuthorStatus::Continuing);
    assert_some_eq_x!(&author.overview, "Test overview");
    assert_some_eq_x!(&author.author_type, "Person");
    assert_some_eq_x!(&author.disambiguation, "American Novelist");
    assert_str_eq!(author.path, "/nfs/books/Test Author");
    assert_some_eq_x!(&author.root_folder_path, "/nfs/books/");
    assert_eq!(author.quality_profile_id, 1);
    assert_eq!(author.metadata_profile_id, 1);
    assert!(author.monitored);
    assert_eq!(author.monitor_new_items, NewItemMonitorType::All);
    assert_eq!(author.genres, vec!["Fantasy", "Fiction"]);
    assert_eq!(author.tags.len(), 2);
    assert_some!(&author.ratings);
    assert_some!(&author.statistics);

    let ratings = author.ratings.unwrap();
    assert_eq!(ratings.votes, 100);
    assert_eq!(ratings.value, 4.5);
    assert_some_eq_x!(ratings.popularity, 450.0);

    let stats = author.statistics.unwrap();
    assert_eq!(stats.book_count, 14);
    assert_eq!(stats.book_file_count, 5);
    assert_eq!(stats.total_book_count, 14);
    assert_eq!(stats.available_book_count, 5);
    assert_eq!(stats.size_on_disk, 51783488);
    assert_eq!(stats.percent_of_books, 35.71);
  }

  #[test]
  fn test_author_status_deserialization() {
    assert_eq!(
      serde_json::from_str::<AuthorStatus>("\"continuing\"").unwrap(),
      AuthorStatus::Continuing
    );
    assert_eq!(
      serde_json::from_str::<AuthorStatus>("\"ended\"").unwrap(),
      AuthorStatus::Ended
    );
    assert_eq!(
      serde_json::from_str::<AuthorStatus>("\"deleted\"").unwrap(),
      AuthorStatus::Deleted
    );
  }

  #[test]
  fn test_ratings_equality() {
    let ratings1 = Ratings {
      votes: 100,
      value: 4.5,
      popularity: Some(450.0),
    };
    let ratings2 = Ratings {
      votes: 100,
      value: 4.5,
      popularity: Some(450.0),
    };
    let ratings3 = Ratings {
      votes: 50,
      value: 3.0,
      popularity: None,
    };

    assert_eq!(ratings1, ratings2);
    assert_ne!(ratings1, ratings3);
  }

  #[test]
  fn test_author_statistics_equality() {
    let stats1 = AuthorStatistics {
      book_count: 14,
      book_file_count: 5,
      total_book_count: 14,
      available_book_count: 5,
      size_on_disk: 51783488,
      percent_of_books: 35.71,
    };
    let stats2 = AuthorStatistics {
      book_count: 14,
      book_file_count: 5,
      total_book_count: 14,
      available_book_count: 5,
      size_on_disk: 51783488,
      percent_of_books: 35.71,
    };
    let stats3 = AuthorStatistics::default();

    assert_eq!(stats1, stats2);
    assert_ne!(stats1, stats3);
  }

  #[test]
  fn test_author_with_optional_fields_none() {
    let author_json = json!({
      "id": 1,
      "authorName": "Test Author",
      "foreignAuthorId": "",
      "status": "continuing",
      "path": "",
      "qualityProfileId": 1,
      "metadataProfileId": 1,
      "monitored": false,
      "monitorNewItems": "all",
      "genres": [],
      "tags": []
    });

    let author: Author = serde_json::from_value(author_json).unwrap();

    assert_none!(&author.overview);
    assert_none!(&author.author_type);
    assert_none!(&author.disambiguation);
    assert_none!(&author.root_folder_path);
    assert_eq!(author.monitor_new_items, NewItemMonitorType::All);
    assert_none!(&author.ratings);
    assert_none!(&author.statistics);
  }

  #[test]
  fn test_readarr_serdeable_from_author() {
    let author = Author {
      id: 1,
      ..Author::default()
    };

    let readarr_serdeable: ReadarrSerdeable = author.clone().into();

    assert_eq!(readarr_serdeable, ReadarrSerdeable::Author(author));
  }

  #[test]
  fn test_readarr_serdeable_from_blocklist_response() {
    let blocklist_response = BlocklistResponse {
      records: vec![BlocklistItem {
        id: 1,
        ..BlocklistItem::default()
      }],
    };

    let readarr_serdeable: ReadarrSerdeable = blocklist_response.clone().into();

    assert_eq!(
      readarr_serdeable,
      ReadarrSerdeable::BlocklistResponse(blocklist_response)
    );
  }

  #[test]
  fn test_readarr_serdeable_from_disk_spaces() {
    let disk_spaces = vec![DiskSpace {
      path: Some("/path".to_owned()),
      free_space: 1,
      total_space: 1,
    }];

    let readarr_serdeable: ReadarrSerdeable = disk_spaces.clone().into();

    assert_eq!(readarr_serdeable, ReadarrSerdeable::DiskSpaces(disk_spaces));
  }

  #[test]
  fn test_readarr_serdeable_from_downloads_response() {
    let downloads_response = DownloadsResponse {
      records: vec![DownloadRecord {
        id: 1,
        ..DownloadRecord::default()
      }],
    };

    let readarr_serdeable: ReadarrSerdeable = downloads_response.clone().into();

    assert_eq!(
      readarr_serdeable,
      ReadarrSerdeable::DownloadsResponse(downloads_response)
    );
  }

  #[test]
  fn test_readarr_serdeable_from_readarr_history_wrapper() {
    let history_wrapper = ReadarrHistoryWrapper {
      records: vec![ReadarrHistoryItem {
        id: 1,
        ..ReadarrHistoryItem::default()
      }],
    };

    let readarr_serdeable: ReadarrSerdeable = history_wrapper.clone().into();

    assert_eq!(
      readarr_serdeable,
      ReadarrSerdeable::ReadarrHistoryWrapper(history_wrapper)
    );
  }

  #[test]
  fn test_readarr_serdeable_from_readarr_history_items() {
    let history_items = vec![ReadarrHistoryItem {
      id: 1,
      ..ReadarrHistoryItem::default()
    }];

    let readarr_serdeable: ReadarrSerdeable = history_items.clone().into();

    assert_eq!(
      readarr_serdeable,
      ReadarrSerdeable::ReadarrHistoryItems(history_items)
    );
  }

  #[test]
  fn test_readarr_serdeable_from_indexers() {
    let indexers = vec![Indexer {
      id: 1,
      ..Indexer::default()
    }];

    let readarr_serdeable: ReadarrSerdeable = indexers.clone().into();

    assert_eq!(readarr_serdeable, ReadarrSerdeable::Indexers(indexers));
  }

  #[test]
  fn test_readarr_serdeable_from_indexer_settings() {
    let indexer_settings = IndexerSettings {
      id: 1,
      ..IndexerSettings::default()
    };

    let readarr_serdeable: ReadarrSerdeable = indexer_settings.clone().into();

    assert_eq!(
      readarr_serdeable,
      ReadarrSerdeable::IndexerSettings(indexer_settings)
    );
  }

  #[test]
  fn test_readarr_serdeable_from_indexer_test_results() {
    let indexer_test_results = vec![IndexerTestResult {
      id: 1,
      ..IndexerTestResult::default()
    }];

    let readarr_serdeable: ReadarrSerdeable = indexer_test_results.clone().into();

    assert_eq!(
      readarr_serdeable,
      ReadarrSerdeable::IndexerTestResults(indexer_test_results)
    );
  }

  #[test]
  fn test_readarr_serdeable_from_log_response() {
    let log_response = LogResponse {
      records: vec![Log {
        level: "info".to_owned(),
        ..Log::default()
      }],
    };

    let readarr_serdeable: ReadarrSerdeable = log_response.clone().into();

    assert_eq!(
      readarr_serdeable,
      ReadarrSerdeable::LogResponse(log_response)
    );
  }

  #[test]
  fn test_readarr_serdeable_from_metadata_profiles() {
    let metadata_profiles = vec![MetadataProfile {
      id: 1,
      name: "Standard".to_owned(),
    }];

    let readarr_serdeable: ReadarrSerdeable = metadata_profiles.clone().into();

    assert_eq!(
      readarr_serdeable,
      ReadarrSerdeable::MetadataProfiles(metadata_profiles)
    );
  }

  #[test]
  fn test_readarr_serdeable_from_host_config() {
    let host_config = HostConfig {
      port: 8787,
      ..HostConfig::default()
    };

    let readarr_serdeable: ReadarrSerdeable = host_config.clone().into();

    assert_eq!(readarr_serdeable, ReadarrSerdeable::HostConfig(host_config));
  }

  #[test]
  fn test_readarr_serdeable_from_quality_profiles() {
    let quality_profiles = vec![QualityProfile {
      id: 1,
      name: "Any".to_owned(),
    }];

    let readarr_serdeable: ReadarrSerdeable = quality_profiles.clone().into();

    assert_eq!(
      readarr_serdeable,
      ReadarrSerdeable::QualityProfiles(quality_profiles)
    );
  }

  #[test]
  fn test_readarr_serdeable_from_queue_events() {
    let queue_events = vec![QueueEvent {
      trigger: "test".to_owned(),
      ..QueueEvent::default()
    }];

    let readarr_serdeable: ReadarrSerdeable = queue_events.clone().into();

    assert_eq!(
      readarr_serdeable,
      ReadarrSerdeable::QueueEvents(queue_events)
    );
  }

  #[test]
  fn test_readarr_serdeable_from_root_folders() {
    let root_folders = vec![RootFolder {
      id: 1,
      path: "/nfs/books".to_owned(),
      accessible: true,
      free_space: 1000000,
      unmapped_folders: None,
    }];

    let readarr_serdeable: ReadarrSerdeable = root_folders.clone().into();

    assert_eq!(
      readarr_serdeable,
      ReadarrSerdeable::RootFolders(root_folders)
    );
  }

  #[test]
  fn test_readarr_serdeable_from_releases() {
    let releases = vec![ReadarrRelease {
      guid: "test".to_owned(),
      ..ReadarrRelease::default()
    }];

    let readarr_serdeable: ReadarrSerdeable = releases.clone().into();

    assert_eq!(readarr_serdeable, ReadarrSerdeable::Releases(releases));
  }

  #[test]
  fn test_readarr_serdeable_from_security_config() {
    let security_config = SecurityConfig {
      api_key: "test-key".to_owned(),
      ..SecurityConfig::default()
    };

    let readarr_serdeable: ReadarrSerdeable = security_config.clone().into();

    assert_eq!(
      readarr_serdeable,
      ReadarrSerdeable::SecurityConfig(security_config)
    );
  }

  #[test]
  fn test_readarr_serdeable_from_system_status() {
    let system_status = SystemStatus {
      version: "1.0.0".to_owned(),
      start_time: Utc::now(),
    };

    let readarr_serdeable: ReadarrSerdeable = system_status.clone().into();

    assert_eq!(
      readarr_serdeable,
      ReadarrSerdeable::SystemStatus(system_status)
    );
  }

  #[test]
  fn test_readarr_serdeable_from_tags() {
    let tags = vec![Tag {
      id: 1,
      label: "fantasy".to_owned(),
    }];

    let readarr_serdeable: ReadarrSerdeable = tags.clone().into();

    assert_eq!(readarr_serdeable, ReadarrSerdeable::Tags(tags));
  }

  #[test]
  fn test_readarr_serdeable_from_add_author_search_results() {
    let search_results = vec![AddAuthorSearchResult {
      foreign_author_id: "test-id".to_owned(),
      ..AddAuthorSearchResult::default()
    }];

    let readarr_serdeable: ReadarrSerdeable = search_results.clone().into();

    assert_eq!(
      readarr_serdeable,
      ReadarrSerdeable::AddAuthorSearchResults(search_results)
    );
  }

  #[test]
  fn test_readarr_serdeable_from_books() {
    let books = vec![Book {
      id: 1,
      ..Book::default()
    }];

    let readarr_serdeable: ReadarrSerdeable = books.clone().into();

    assert_eq!(readarr_serdeable, ReadarrSerdeable::Books(books));
  }

  #[test]
  fn test_readarr_serdeable_from_book() {
    let book = Book {
      id: 1,
      ..Book::default()
    };

    let readarr_serdeable: ReadarrSerdeable = book.clone().into();

    assert_eq!(readarr_serdeable, ReadarrSerdeable::Book(book));
  }

  #[test]
  fn test_readarr_serdeable_from_editions() {
    let editions = vec![Edition {
      id: 1,
      ..Edition::default()
    }];

    let readarr_serdeable: ReadarrSerdeable = editions.clone().into();

    assert_eq!(readarr_serdeable, ReadarrSerdeable::Editions(editions));
  }

  #[test]
  fn test_readarr_serdeable_from_book_files() {
    let book_files = vec![BookFile {
      id: 1,
      media_info: Some(MediaInfo {
        audio_channels: 2,
        ..MediaInfo::default()
      }),
      ..BookFile::default()
    }];

    let readarr_serdeable: ReadarrSerdeable = book_files.clone().into();

    assert_eq!(readarr_serdeable, ReadarrSerdeable::BookFiles(book_files));
  }

  #[test]
  fn test_readarr_serdeable_from_tasks() {
    let tasks = vec![ReadarrTask {
      name: "test".to_owned(),
      ..ReadarrTask::default()
    }];

    let readarr_serdeable: ReadarrSerdeable = tasks.clone().into();

    assert_eq!(readarr_serdeable, ReadarrSerdeable::Tasks(tasks));
  }

  #[test]
  fn test_readarr_serdeable_from_updates() {
    let updates = vec![Update {
      version: "test".to_owned(),
      ..Update::default()
    }];

    let readarr_serdeable: ReadarrSerdeable = updates.clone().into();

    assert_eq!(readarr_serdeable, ReadarrSerdeable::Updates(updates));
  }

  #[test]
  fn test_author_status_display() {
    assert_str_eq!(AuthorStatus::Continuing.to_string(), "continuing");
    assert_str_eq!(AuthorStatus::Ended.to_string(), "ended");
    assert_str_eq!(AuthorStatus::Deleted.to_string(), "deleted");
  }

  #[test]
  fn test_author_status_to_display_str() {
    assert_str_eq!(AuthorStatus::Continuing.to_display_str(), "Continuing");
    assert_str_eq!(AuthorStatus::Ended.to_display_str(), "Ended");
    assert_str_eq!(AuthorStatus::Deleted.to_display_str(), "Deleted");
  }

  #[test]
  fn test_readarr_history_event_type_display() {
    assert_str_eq!(ReadarrHistoryEventType::Unknown.to_string(), "unknown");
    assert_str_eq!(ReadarrHistoryEventType::Grabbed.to_string(), "grabbed");
    assert_str_eq!(
      ReadarrHistoryEventType::AuthorFolderImported.to_string(),
      "authorFolderImported"
    );
    assert_str_eq!(
      ReadarrHistoryEventType::BookImportIncomplete.to_string(),
      "bookImportIncomplete"
    );
    assert_str_eq!(
      ReadarrHistoryEventType::DownloadIgnored.to_string(),
      "downloadIgnored"
    );
    assert_str_eq!(
      ReadarrHistoryEventType::DownloadImported.to_string(),
      "downloadImported"
    );
    assert_str_eq!(
      ReadarrHistoryEventType::DownloadFailed.to_string(),
      "downloadFailed"
    );
    assert_str_eq!(
      ReadarrHistoryEventType::BookFileDeleted.to_string(),
      "bookFileDeleted"
    );
    assert_str_eq!(
      ReadarrHistoryEventType::BookFileImported.to_string(),
      "bookFileImported"
    );
    assert_str_eq!(
      ReadarrHistoryEventType::BookFileRenamed.to_string(),
      "bookFileRenamed"
    );
    assert_str_eq!(
      ReadarrHistoryEventType::BookFileRetagged.to_string(),
      "bookFileRetagged"
    );
  }

  #[test]
  fn test_readarr_history_event_type_to_display_str() {
    assert_str_eq!(ReadarrHistoryEventType::Unknown.to_display_str(), "Unknown");
    assert_str_eq!(ReadarrHistoryEventType::Grabbed.to_display_str(), "Grabbed");
    assert_str_eq!(
      ReadarrHistoryEventType::AuthorFolderImported.to_display_str(),
      "Author Folder Imported"
    );
    assert_str_eq!(
      ReadarrHistoryEventType::BookImportIncomplete.to_display_str(),
      "Book Import Incomplete"
    );
    assert_str_eq!(
      ReadarrHistoryEventType::DownloadIgnored.to_display_str(),
      "Download Ignored"
    );
    assert_str_eq!(
      ReadarrHistoryEventType::DownloadImported.to_display_str(),
      "Download Imported"
    );
    assert_str_eq!(
      ReadarrHistoryEventType::DownloadFailed.to_display_str(),
      "Download Failed"
    );
    assert_str_eq!(
      ReadarrHistoryEventType::BookFileDeleted.to_display_str(),
      "Book File Deleted"
    );
    assert_str_eq!(
      ReadarrHistoryEventType::BookFileImported.to_display_str(),
      "Book File Imported"
    );
    assert_str_eq!(
      ReadarrHistoryEventType::BookFileRenamed.to_display_str(),
      "Book File Renamed"
    );
    assert_str_eq!(
      ReadarrHistoryEventType::BookFileRetagged.to_display_str(),
      "Book File Retagged"
    );
  }

  #[test]
  fn test_readarr_history_event_type_deserialization() {
    assert_eq!(
      deserialize_history_event_type(json!("unknown")),
      ReadarrHistoryEventType::Unknown
    );
    assert_eq!(
      deserialize_history_event_type(json!("grabbed")),
      ReadarrHistoryEventType::Grabbed
    );
    assert_eq!(
      deserialize_history_event_type(json!("authorFolderImported")),
      ReadarrHistoryEventType::AuthorFolderImported
    );
    assert_eq!(
      deserialize_history_event_type(json!("bookImportIncomplete")),
      ReadarrHistoryEventType::BookImportIncomplete
    );
    assert_eq!(
      deserialize_history_event_type(json!("downloadIgnored")),
      ReadarrHistoryEventType::DownloadIgnored
    );
    assert_eq!(
      deserialize_history_event_type(json!("downloadImported")),
      ReadarrHistoryEventType::DownloadImported
    );
    assert_eq!(
      deserialize_history_event_type(json!("downloadFailed")),
      ReadarrHistoryEventType::DownloadFailed
    );
    assert_eq!(
      deserialize_history_event_type(json!("bookFileDeleted")),
      ReadarrHistoryEventType::BookFileDeleted
    );
    assert_eq!(
      deserialize_history_event_type(json!("bookFileImported")),
      ReadarrHistoryEventType::BookFileImported
    );
    assert_eq!(
      deserialize_history_event_type(json!("bookFileRenamed")),
      ReadarrHistoryEventType::BookFileRenamed
    );
    assert_eq!(
      deserialize_history_event_type(json!("bookFileRetagged")),
      ReadarrHistoryEventType::BookFileRetagged
    );
  }

  #[test]
  fn test_readarr_history_event_type_deserialization_falls_back_to_unknown() {
    assert_eq!(
      deserialize_history_event_type(json!("bookImportPartiallyComplete")),
      ReadarrHistoryEventType::Unknown
    );
    assert_eq!(
      deserialize_history_event_type(json!(2)),
      ReadarrHistoryEventType::Unknown
    );
    assert_eq!(
      deserialize_history_event_type(json!(11)),
      ReadarrHistoryEventType::Unknown
    );
    assert_eq!(
      deserialize_history_event_type(json!(null)),
      ReadarrHistoryEventType::Unknown
    );
    assert_eq!(
      deserialize_history_event_type(json!({ "id": 2 })),
      ReadarrHistoryEventType::Unknown
    );
  }

  #[test]
  fn test_readarr_history_event_type_serialization() {
    assert_str_eq!(
      serialize_history_event_type(ReadarrHistoryEventType::Unknown),
      "unknown"
    );
    assert_str_eq!(
      serialize_history_event_type(ReadarrHistoryEventType::Grabbed),
      "grabbed"
    );
    assert_str_eq!(
      serialize_history_event_type(ReadarrHistoryEventType::AuthorFolderImported),
      "authorFolderImported"
    );
    assert_str_eq!(
      serialize_history_event_type(ReadarrHistoryEventType::BookImportIncomplete),
      "bookImportIncomplete"
    );
    assert_str_eq!(
      serialize_history_event_type(ReadarrHistoryEventType::DownloadIgnored),
      "downloadIgnored"
    );
    assert_str_eq!(
      serialize_history_event_type(ReadarrHistoryEventType::DownloadImported),
      "downloadImported"
    );
    assert_str_eq!(
      serialize_history_event_type(ReadarrHistoryEventType::DownloadFailed),
      "downloadFailed"
    );
    assert_str_eq!(
      serialize_history_event_type(ReadarrHistoryEventType::BookFileDeleted),
      "bookFileDeleted"
    );
    assert_str_eq!(
      serialize_history_event_type(ReadarrHistoryEventType::BookFileImported),
      "bookFileImported"
    );
    assert_str_eq!(
      serialize_history_event_type(ReadarrHistoryEventType::BookFileRenamed),
      "bookFileRenamed"
    );
    assert_str_eq!(
      serialize_history_event_type(ReadarrHistoryEventType::BookFileRetagged),
      "bookFileRetagged"
    );
  }

  #[test]
  fn test_readarr_task_name_display() {
    assert_str_eq!(
      ReadarrTaskName::ApplicationUpdateCheck.to_string(),
      "ApplicationUpdateCheck"
    );
    assert_str_eq!(ReadarrTaskName::Backup.to_string(), "Backup");
    assert_str_eq!(ReadarrTaskName::CheckHealth.to_string(), "CheckHealth");
    assert_str_eq!(ReadarrTaskName::Housekeeping.to_string(), "Housekeeping");
    assert_str_eq!(
      ReadarrTaskName::ImportListSync.to_string(),
      "ImportListSync"
    );
    assert_str_eq!(
      ReadarrTaskName::MessagingCleanup.to_string(),
      "MessagingCleanup"
    );
    assert_str_eq!(ReadarrTaskName::RefreshAuthor.to_string(), "RefreshAuthor");
    assert_str_eq!(
      ReadarrTaskName::RefreshMonitoredDownloads.to_string(),
      "RefreshMonitoredDownloads"
    );
    assert_str_eq!(ReadarrTaskName::RescanFolders.to_string(), "RescanFolders");
    assert_str_eq!(ReadarrTaskName::RssSync.to_string(), "RssSync");
  }

  #[test]
  fn test_book_deserialization() {
    let book_json = json!({
      "id": 1,
      "title": "The Name of the Wind",
      "authorId": 3,
      "foreignBookId": "test-foreign-book-id",
      "monitored": true,
      "anyEditionOk": true,
      "pageCount": 662,
      "releaseDate": "2007-03-27T00:00:00Z",
      "grabbed": false,
      "ratings": {
        "votes": 100,
        "value": 4.5,
        "popularity": 450.0
      },
      "statistics": {
        "bookFileCount": 1,
        "totalBookCount": 1,
        "sizeOnDisk": 51783488,
        "percentOfBooks": 100.0
      }
    });

    let book: Book = serde_json::from_value(book_json).unwrap();

    assert_eq!(book.id, 1);
    assert_str_eq!(book.title.text, "The Name of the Wind");
    assert_eq!(book.author_id, 3);
    assert_str_eq!(book.foreign_book_id, "test-foreign-book-id");
    assert!(book.monitored);
    assert!(book.any_edition_ok);
    assert_some_eq_x!(book.page_count, 662);
    assert_some!(&book.release_date);
    assert!(!book.grabbed);
    assert_some!(&book.ratings);

    let stats = book.statistics.unwrap();
    assert_eq!(stats.book_file_count, 1);
    assert_eq!(stats.total_book_count, 1);
    assert_eq!(stats.size_on_disk, 51783488);
    assert_eq!(stats.percent_of_books, 100.0);
  }

  #[test]
  fn test_book_with_optional_fields_none() {
    let book_json = json!({
      "id": 1,
      "title": "The Name of the Wind",
      "authorId": 3,
      "foreignBookId": "",
      "monitored": false,
      "grabbed": false
    });

    let book: Book = serde_json::from_value(book_json).unwrap();

    assert!(!book.any_edition_ok);
    assert_none!(&book.page_count);
    assert_none!(&book.release_date);
    assert_none!(&book.ratings);
    assert_none!(&book.statistics);
  }

  #[test]
  fn test_edition_deserialization() {
    let edition_json = json!({
      "id": 106,
      "bookId": 1,
      "foreignEditionId": "test-foreign-edition-id",
      "monitored": true,
      "isEbook": false,
      "title": "The Name of the Wind",
      "language": "eng",
      "overview": "Test overview",
      "format": "Hardcover",
      "publisher": "DAW Books",
      "pageCount": 662,
      "releaseDate": "2007-03-27T00:00:00Z",
      "isbn13": "9780756404079",
      "asin": "0756404070",
      "ratings": {
        "votes": 100,
        "value": 4.5,
        "popularity": 450.0
      }
    });

    let edition: Edition = serde_json::from_value(edition_json).unwrap();

    assert_eq!(edition.id, 106);
    assert_eq!(edition.book_id, 1);
    assert_str_eq!(edition.foreign_edition_id, "test-foreign-edition-id");
    assert!(edition.monitored);
    assert!(!edition.is_ebook);
    assert_str_eq!(edition.title, "The Name of the Wind");
    assert_some_eq_x!(&edition.language, "eng");
    assert_some_eq_x!(&edition.overview, "Test overview");
    assert_some_eq_x!(&edition.format, "Hardcover");
    assert_some_eq_x!(&edition.publisher, "DAW Books");
    assert_some_eq_x!(edition.page_count, 662);
    assert_some!(&edition.release_date);
    assert_some_eq_x!(&edition.isbn13, "9780756404079");
    assert_some_eq_x!(&edition.asin, "0756404070");
    assert_some!(&edition.ratings);
  }

  #[test]
  fn test_edition_with_optional_fields_none() {
    let edition_json = json!({
      "id": 106,
      "bookId": 1,
      "foreignEditionId": "",
      "monitored": false,
      "isEbook": true,
      "title": ""
    });

    let edition: Edition = serde_json::from_value(edition_json).unwrap();

    assert_none!(&edition.language);
    assert_none!(&edition.overview);
    assert_none!(&edition.format);
    assert_none!(&edition.publisher);
    assert_none!(&edition.page_count);
    assert_none!(&edition.release_date);
    assert_none!(&edition.isbn13);
    assert_none!(&edition.asin);
    assert_none!(&edition.ratings);
  }

  #[test]
  fn test_media_info_deserialization() {
    let media_info_json = json!({
      "audioChannels": 2,
      "audioBitRate": "99 kbps",
      "audioCodec": "AAC",
      "audioBits": "",
      "audioSampleRate": "44.1kHz"
    });

    let media_info: MediaInfo = serde_json::from_value(media_info_json).unwrap();

    assert_eq!(media_info.audio_channels, 2);
    assert_some_eq_x!(&media_info.audio_bit_rate, "99 kbps");
    assert_some_eq_x!(&media_info.audio_codec, "AAC");
    assert_some_eq_x!(&media_info.audio_bits, "");
    assert_some_eq_x!(&media_info.audio_sample_rate, "44.1kHz");
  }

  #[test]
  fn test_download_record_deserialization() {
    let download_record_json = json!({
      "title": "Test Download",
      "status": "downloading",
      "id": 1,
      "bookId": 2,
      "authorId": 3,
      "size": 3543348019.0,
      "sizeleft": 1771674009.5,
      "outputPath": "/nfs/books/Test Author",
      "downloadClient": "transmission"
    });

    let download_record: DownloadRecord = serde_json::from_value(download_record_json).unwrap();

    assert_str_eq!(download_record.title, "Test Download");
    assert_eq!(download_record.status, DownloadStatus::Downloading);
    assert_eq!(download_record.id, 1);
    assert_some!(&download_record.book_id);
    assert_some!(&download_record.author_id);
    assert_eq!(download_record.size, 3543348019.0);
    assert_eq!(download_record.sizeleft, 1771674009.5);
    assert_some!(&download_record.output_path);
    assert!(download_record.indexer.is_empty());
    assert_some_eq_x!(&download_record.download_client, "transmission");
  }

  #[test]
  fn test_add_author_search_result_deserialization() {
    let search_result_json = json!({
      "foreignAuthorId": "test-foreign-id",
      "authorName": "Test Author",
      "status": "continuing",
      "ended": false,
      "overview": "Test overview",
      "authorType": "Person",
      "disambiguation": "American Novelist",
      "genres": ["Fantasy", "Fiction"],
      "ratings": {
        "votes": 100,
        "value": 4.5,
        "popularity": 450.0
      }
    });

    let search_result: AddAuthorSearchResult = serde_json::from_value(search_result_json).unwrap();

    assert_str_eq!(search_result.foreign_author_id, "test-foreign-id");
    assert_str_eq!(search_result.author_name.text, "Test Author");
    assert_eq!(search_result.status, AuthorStatus::Continuing);
    assert!(!search_result.ended);
    assert_some_eq_x!(&search_result.overview, "Test overview");
    assert_some_eq_x!(&search_result.author_type, "Person");
    assert_some_eq_x!(&search_result.disambiguation, "American Novelist");
    assert_eq!(search_result.genres, vec!["Fantasy", "Fiction"]);
    assert_some!(&search_result.ratings);

    let ratings = search_result.ratings.unwrap();
    assert_eq!(ratings.votes, 100);
    assert_eq!(ratings.value, 4.5);
  }

  #[test]
  fn test_add_author_search_result_with_optional_fields_none() {
    let search_result_json = json!({
      "foreignAuthorId": "test-foreign-id",
      "authorName": "Test Author",
      "status": "ended",
      "genres": []
    });

    let search_result: AddAuthorSearchResult = serde_json::from_value(search_result_json).unwrap();

    assert_str_eq!(search_result.foreign_author_id, "test-foreign-id");
    assert_str_eq!(search_result.author_name.text, "Test Author");
    assert_eq!(search_result.status, AuthorStatus::Ended);
    assert!(!search_result.ended);
    assert_none!(&search_result.overview);
    assert_none!(&search_result.author_type);
    assert_none!(&search_result.disambiguation);
    assert!(search_result.genres.is_empty());
    assert_none!(&search_result.ratings);
  }

  #[test]
  fn test_author_status_deserialization_falls_back_to_default() {
    let expected = Author {
      status: AuthorStatus::default(),
      ..serde_json::from_str(AUTHOR_JSON).unwrap()
    };

    assert_eq!(
      deserialize_with_field::<Author>(AUTHOR_JSON, "status", json!(2)),
      expected
    );
    assert_eq!(
      deserialize_with_field::<Author>(AUTHOR_JSON, "status", json!("notAThing")),
      expected
    );
  }

  #[test]
  fn test_author_monitor_new_items_deserialization_falls_back_to_default() {
    let expected = Author {
      monitor_new_items: NewItemMonitorType::default(),
      ..serde_json::from_str(AUTHOR_JSON).unwrap()
    };

    assert_eq!(
      deserialize_with_field::<Author>(AUTHOR_JSON, "monitorNewItems", json!(2)),
      expected
    );
    assert_eq!(
      deserialize_with_field::<Author>(AUTHOR_JSON, "monitorNewItems", json!("notAThing")),
      expected
    );
  }

  #[test]
  fn test_add_author_search_result_status_deserialization_falls_back_to_default() {
    let expected = AddAuthorSearchResult {
      status: AuthorStatus::default(),
      ..serde_json::from_str(ADD_AUTHOR_SEARCH_RESULT_JSON).unwrap()
    };

    assert_eq!(
      deserialize_with_field::<AddAuthorSearchResult>(
        ADD_AUTHOR_SEARCH_RESULT_JSON,
        "status",
        json!(2)
      ),
      expected
    );
    assert_eq!(
      deserialize_with_field::<AddAuthorSearchResult>(
        ADD_AUTHOR_SEARCH_RESULT_JSON,
        "status",
        json!("notAThing")
      ),
      expected
    );
  }

  #[test]
  fn test_download_record_status_deserialization_falls_back_to_default() {
    let download_record_json = serde_json::to_string(&download_record()).unwrap();
    let expected = DownloadRecord {
      status: DownloadStatus::default(),
      ..download_record()
    };

    assert_eq!(
      deserialize_with_field::<DownloadRecord>(&download_record_json, "status", json!(2)),
      expected
    );
    assert_eq!(
      deserialize_with_field::<DownloadRecord>(&download_record_json, "status", json!("notAThing")),
      expected
    );
  }

  #[test]
  fn test_readarr_task_task_name_deserialization_falls_back_to_default() {
    let task = ReadarrTask {
      name: "Backup".to_owned(),
      task_name: ReadarrTaskName::Backup,
      ..ReadarrTask::default()
    };
    let task_json = serde_json::to_string(&task).unwrap();
    let expected = ReadarrTask {
      task_name: ReadarrTaskName::default(),
      ..task
    };

    assert_eq!(
      deserialize_with_field::<ReadarrTask>(&task_json, "taskName", json!(2)),
      expected
    );
    assert_eq!(
      deserialize_with_field::<ReadarrTask>(&task_json, "taskName", json!("notAThing")),
      expected
    );
  }

  fn deserialize_with_field<T: DeserializeOwned>(json: &str, field: &str, value: Value) -> T {
    let mut fixture_json: Value = serde_json::from_str(json).unwrap();
    fixture_json[field] = value;

    serde_json::from_value(fixture_json).unwrap()
  }

  fn deserialize_history_event_type(event_type: Value) -> ReadarrHistoryEventType {
    let history_item_json = json!({
      "id": 1,
      "authorId": 1,
      "bookId": 1,
      "sourceTitle": "Test Source Title",
      "date": "2024-01-01T00:00:00Z",
      "eventType": event_type
    });

    serde_json::from_value::<ReadarrHistoryItem>(history_item_json)
      .unwrap()
      .event_type
  }

  fn serialize_history_event_type(event_type: ReadarrHistoryEventType) -> String {
    let history_item = ReadarrHistoryItem {
      event_type,
      ..ReadarrHistoryItem::default()
    };

    serde_json::to_value(history_item).unwrap()["eventType"]
      .as_str()
      .unwrap()
      .to_owned()
  }
}
