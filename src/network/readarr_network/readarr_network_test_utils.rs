#[cfg(test)]
#[allow(dead_code)]
pub mod test_utils {
  use crate::models::readarr_models::{
    AddAuthorBody, AddAuthorOptions, AddAuthorSearchResult, Author, AuthorStatus, BlocklistItem,
    Book, BookFile, DownloadRecord, DownloadStatus, DownloadsResponse, Edition, MonitorType,
    NewItemMonitorType, ReadarrHistoryData, ReadarrHistoryEventType, ReadarrHistoryItem,
    ReadarrRelease,
  };
  use crate::models::servarr_models::{Indexer, IndexerField, Quality, QualityWrapper, RootFolder};
  use chrono::DateTime;
  use serde_json::{Number, json};

  pub const AUTHOR_JSON: &str = r#"{
    "id": 1,
    "authorName": "Test Author",
    "foreignAuthorId": "test-foreign-id",
    "status": "continuing",
    "overview": "some interesting description of the author",
    "authorType": "Person",
    "disambiguation": "American novelist",
    "path": "/nfs/books/Test Author",
    "rootFolderPath": "/nfs/books/",
    "qualityProfileId": 1,
    "metadataProfileId": 1,
    "monitored": true,
    "monitorNewItems": "all",
    "genres": ["science fiction"],
    "tags": [1],
    "ratings": { "votes": 15, "value": 8.4, "popularity": 1.2 },
    "statistics": {
      "bookCount": 3,
      "bookFileCount": 2,
      "totalBookCount": 3,
      "availableBookCount": 2,
      "sizeOnDisk": 12345,
      "percentOfBooks": 66.6
    }
  }"#;

  pub const ADD_AUTHOR_SEARCH_RESULT_JSON: &str = r#"{
    "foreignAuthorId": "test-foreign-id",
    "authorName": "Test Author",
    "status": "continuing",
    "ended": false,
    "overview": "some interesting description of the author",
    "authorType": "Person",
    "disambiguation": "American novelist",
    "genres": ["science fiction"],
    "ratings": { "votes": 15, "value": 8.4, "popularity": 1.2 }
  }"#;

  pub const BOOK_JSON: &str = r#"{
    "id": 1,
    "title": "Test Book",
    "authorTitle": "Test Author Test Book",
    "seriesTitle": "Test Series",
    "authorId": 1,
    "foreignBookId": "test-foreign-book-id",
    "monitored": true,
    "anyEditionOk": true,
    "pageCount": 662,
    "ratings": { "votes": 15, "value": 8.4, "popularity": 1.2 },
    "releaseDate": "2023-01-01T00:00:00Z",
    "statistics": {
      "bookFileCount": 2,
      "totalBookCount": 3,
      "sizeOnDisk": 12345,
      "percentOfBooks": 66.6
    },
    "lastSearchTime": "2023-01-01T00:00:00Z",
    "grabbed": false
  }"#;

  pub const BOOK_FILE_JSON: &str = r#"{
    "id": 1,
    "authorId": 1,
    "bookId": 1,
    "path": "/nfs/books/Test Author/Test Book/Test Author - Test Book.azw3",
    "size": 1074732,
    "dateAdded": "2023-01-01T00:00:00Z",
    "quality": { "quality": { "name": "AZW3" } },
    "mediaInfo": {
      "audioBitRate": "128 kbps",
      "audioChannels": 2,
      "audioCodec": "MP3",
      "audioBits": "16",
      "audioSampleRate": "44.1 kHz"
    },
    "qualityCutoffNotMet": false
  }"#;

  pub const EDITION_JSON: &str = r#"{
    "id": 1,
    "bookId": 1,
    "foreignEditionId": "test-foreign-edition-id",
    "monitored": true,
    "isEbook": true,
    "title": "Test Edition",
    "language": "eng",
    "overview": "some interesting description of the edition",
    "format": "Paperback",
    "publisher": "Test Publisher",
    "pageCount": 662,
    "releaseDate": "2023-01-01T00:00:00Z",
    "isbn13": "9780000000001",
    "asin": "B000000001",
    "ratings": { "votes": 15, "value": 8.4, "popularity": 1.2 }
  }"#;

  pub const DOWNLOAD_RECORD_JSON: &str = r#"{
    "title": "Test Book Download",
    "status": "downloading",
    "id": 1,
    "bookId": 1,
    "authorId": 1,
    "size": 1000.0,
    "sizeleft": 250.0,
    "outputPath": "/nfs/nzbget/completed/books/Test Author - Test Book",
    "indexer": "test-indexer",
    "downloadClient": "NZBGet"
  }"#;

  pub const RELEASE_JSON: &str = r#"{
    "guid": "test-release-guid",
    "protocol": "torrent",
    "age": 4492,
    "title": "Test Author - Test Book [AZW3]",
    "authorName": "Test Author",
    "bookTitle": "Test Book",
    "indexer": "kickass torrents",
    "indexerId": 2,
    "size": 313924185,
    "rejected": true,
    "rejections": ["Unknown quality profile", "Release is already mapped"],
    "seeders": 2,
    "leechers": 1,
    "quality": {
      "quality": { "id": 12, "name": "AZW3" },
      "revision": { "version": 1, "real": 0, "isRepack": false }
    },
    "customFormatScore": 0,
    "downloadAllowed": true,
    "publishDate": "2014-05-11T14:38:20Z"
  }"#;

  pub const INDEXER_JSON: &str = r#"{
    "enableRss": true,
    "enableAutomaticSearch": true,
    "enableInteractiveSearch": true,
    "supportsRss": true,
    "supportsSearch": true,
    "protocol": "torrent",
    "priority": 25,
    "downloadClientId": 0,
    "name": "Test Indexer",
    "implementationName": "Torznab",
    "implementation": "Torznab",
    "configContract": "TorznabSettings",
    "infoLink": "https://wiki.servarr.com/readarr/supported#torznab",
    "fields": [
      {
        "order": 0,
        "name": "baseUrl",
        "label": "URL",
        "value": "https://test.com",
        "type": "textbox",
        "advanced": false,
        "isFloat": false
      },
      {
        "order": 2,
        "name": "apiKey",
        "label": "API Key",
        "value": "",
        "type": "textbox",
        "advanced": false,
        "isFloat": false
      },
      {
        "order": 7,
        "name": "seedCriteria.seedRatio",
        "label": "Seed Ratio",
        "value": "1.2",
        "type": "number",
        "advanced": false,
        "isFloat": true
      }
    ],
    "tags": [1],
    "id": 8
  }"#;

  pub fn stale_author() -> Author {
    Author {
      id: 99,
      author_name: "Stale Author".into(),
      foreign_author_id: "foreign-author-99".to_owned(),
      status: AuthorStatus::Continuing,
      path: "/nfs/books/Stale Author".to_owned(),
      root_folder_path: Some("/nfs/books/".to_owned()),
      quality_profile_id: 1,
      metadata_profile_id: 1,
      monitored: true,
      monitor_new_items: NewItemMonitorType::All,
      genres: vec![],
      tags: vec![],
      ..Author::default()
    }
  }

  pub fn stale_book() -> Book {
    Book {
      id: 99,
      title: "Stale Book".into(),
      author_id: 99,
      foreign_book_id: "foreign-book-99".to_owned(),
      monitored: true,
      ..Book::default()
    }
  }

  pub fn stale_book_file() -> BookFile {
    BookFile {
      id: 99,
      author_id: 99,
      book_id: 99,
      path: "/nfs/books/Stale Author/Stale Book.azw3".to_owned(),
      ..BookFile::default()
    }
  }

  pub fn stale_edition() -> Edition {
    Edition {
      id: 99,
      book_id: 99,
      foreign_edition_id: "foreign-edition-99".to_owned(),
      monitored: true,
      is_ebook: true,
      title: "Stale Edition".to_owned(),
      ..Edition::default()
    }
  }

  pub fn stale_add_author_search_result() -> AddAuthorSearchResult {
    AddAuthorSearchResult {
      foreign_author_id: "foreign-author-99".to_owned(),
      author_name: "Stale Author".into(),
      status: AuthorStatus::Continuing,
      ..AddAuthorSearchResult::default()
    }
  }

  pub fn stale_root_folder() -> RootFolder {
    RootFolder {
      id: 99,
      path: "/stale".to_owned(),
      accessible: false,
      free_space: 1,
      unmapped_folders: None,
    }
  }

  pub fn stale_readarr_history_item() -> ReadarrHistoryItem {
    ReadarrHistoryItem {
      id: 99,
      source_title: "Stale source title".into(),
      ..ReadarrHistoryItem::default()
    }
  }

  pub fn stale_download_record() -> DownloadRecord {
    DownloadRecord {
      id: 99,
      title: "Stale Download".to_owned(),
      status: DownloadStatus::Queued,
      ..DownloadRecord::default()
    }
  }

  pub fn quality_wrapper() -> QualityWrapper {
    QualityWrapper { quality: quality() }
  }

  pub fn stale_blocklist_item() -> BlocklistItem {
    BlocklistItem {
      id: 99,
      author_id: 99,
      source_title: "Stale source title".to_owned(),
      ..BlocklistItem::default()
    }
  }

  pub fn blocklist_item() -> BlocklistItem {
    BlocklistItem {
      id: 1,
      author_id: 1,
      book_ids: Some(vec![Number::from(1)]),
      source_title: "Test source title".to_owned(),
      quality: quality_wrapper(),
      date: DateTime::from(DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z").unwrap()),
      protocol: "usenet".to_owned(),
      indexer: "DrunkenSlug".to_owned(),
      message: "test message".to_owned(),
      author: serde_json::from_str(AUTHOR_JSON).unwrap(),
    }
  }

  pub fn stale_release() -> ReadarrRelease {
    ReadarrRelease {
      guid: "stale-release-guid".to_owned(),
      protocol: "usenet".to_owned(),
      title: "Stale Release".into(),
      indexer: "Stale Indexer".to_owned(),
      indexer_id: 99,
      ..ReadarrRelease::default()
    }
  }

  pub fn rejections() -> Vec<String> {
    vec![
      "Unknown quality profile".to_owned(),
      "Release is already mapped".to_owned(),
    ]
  }

  pub fn torrent_release() -> ReadarrRelease {
    ReadarrRelease {
      guid: "test-release-guid".to_owned(),
      protocol: "torrent".to_owned(),
      age: 4492,
      title: "Test Author - Test Book [AZW3]".into(),
      author_name: Some("Test Author".to_owned()),
      book_title: Some("Test Book".to_owned()),
      indexer: "kickass torrents".to_owned(),
      indexer_id: 2,
      size: 313924185,
      rejected: true,
      rejections: Some(rejections()),
      seeders: Some(Number::from(2)),
      leechers: Some(Number::from(1)),
      quality: quality_wrapper(),
    }
  }

  pub fn usenet_release() -> ReadarrRelease {
    ReadarrRelease {
      guid: "test-usenet-release-guid".to_owned(),
      protocol: "usenet".to_owned(),
      indexer: "DrunkenSlug".to_owned(),
      indexer_id: 4,
      seeders: None,
      leechers: None,
      ..torrent_release()
    }
  }

  pub fn quality() -> Quality {
    Quality {
      name: "AZW3".to_string(),
    }
  }

  pub fn readarr_history_item() -> ReadarrHistoryItem {
    ReadarrHistoryItem {
      id: 1,
      author_id: 1,
      book_id: 1,
      source_title: "Test source title".into(),
      quality: quality_wrapper(),
      date: DateTime::from(DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z").unwrap()),
      event_type: ReadarrHistoryEventType::Grabbed,
      data: readarr_history_data(),
    }
  }

  pub fn readarr_history_data() -> ReadarrHistoryData {
    ReadarrHistoryData {
      dropped_path: Some("/nfs/nzbget/completed/books/Something/cool.azw3".to_owned()),
      imported_path: Some("/nfs/books/Test Author/Book 1/Cool.azw3".to_owned()),
      ..ReadarrHistoryData::default()
    }
  }

  pub fn add_author_body() -> AddAuthorBody {
    AddAuthorBody {
      foreign_author_id: "test-foreign-id".to_owned(),
      author_name: "Test Author".to_owned(),
      monitored: true,
      root_folder_path: "/nfs/books".to_owned(),
      quality_profile_id: 1,
      metadata_profile_id: 1,
      tags: Vec::default(),
      tag_input_string: Some("usenet, testing".to_owned()),
      add_options: AddAuthorOptions {
        monitor: MonitorType::All,
        monitor_new_items: NewItemMonitorType::All,
        search_for_missing_books: true,
      },
    }
  }

  pub fn download_record() -> DownloadRecord {
    DownloadRecord {
      title: "Test Book Download".to_owned(),
      status: DownloadStatus::Downloading,
      id: 1,
      book_id: Some(Number::from(1)),
      author_id: Some(Number::from(1)),
      size: 1000.0,
      sizeleft: 250.0,
      output_path: Some("/nfs/nzbget/completed/books/Test Author - Test Book".into()),
      indexer: "test-indexer".to_owned(),
      download_client: Some("NZBGet".to_owned()),
    }
  }

  pub fn downloads_response() -> DownloadsResponse {
    DownloadsResponse {
      records: vec![download_record()],
    }
  }

  pub fn stale_indexer() -> Indexer {
    Indexer {
      id: 99,
      name: Some("Stale Indexer".to_owned()),
      protocol: "usenet".to_owned(),
      priority: 99,
      ..Indexer::default()
    }
  }

  pub fn indexer() -> Indexer {
    Indexer {
      id: 8,
      name: Some("Test Indexer".to_owned()),
      implementation: Some("Torznab".to_owned()),
      implementation_name: Some("Torznab".to_owned()),
      config_contract: Some("TorznabSettings".to_owned()),
      supports_rss: true,
      supports_search: true,
      fields: Some(vec![
        IndexerField {
          name: Some("baseUrl".to_owned()),
          value: Some(json!("https://test.com")),
        },
        IndexerField {
          name: Some("apiKey".to_owned()),
          value: Some(json!("")),
        },
        IndexerField {
          name: Some("seedCriteria.seedRatio".to_owned()),
          value: Some(json!("1.2")),
        },
      ]),
      enable_rss: true,
      enable_automatic_search: true,
      enable_interactive_search: true,
      protocol: "torrent".to_owned(),
      priority: 25,
      download_client_id: 0,
      tags: vec![Number::from(1)],
    }
  }
}
