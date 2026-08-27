#[cfg(test)]
#[allow(dead_code)]
pub mod test_utils {
  use crate::models::readarr_models::{
    AddAuthorBody, AddAuthorOptions, AddAuthorSearchResult, Author, AuthorStatus, Book, Edition,
    MonitorType, NewItemMonitorType, ReadarrHistoryData, ReadarrHistoryEventType,
    ReadarrHistoryItem,
  };
  use crate::models::servarr_models::{Quality, QualityWrapper, RootFolder};
  use chrono::DateTime;

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
    "grabbed": false
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

  pub fn quality_wrapper() -> QualityWrapper {
    QualityWrapper { quality: quality() }
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
}
