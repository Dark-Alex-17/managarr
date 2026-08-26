#[cfg(test)]
#[allow(dead_code)]
pub mod test_utils {
  use crate::models::readarr_models::{Author, AuthorStatus, NewItemMonitorType};
  use crate::models::servarr_models::RootFolder;

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

  pub fn stale_root_folder() -> RootFolder {
    RootFolder {
      id: 99,
      path: "/stale".to_owned(),
      accessible: false,
      free_space: 1,
      unmapped_folders: None,
    }
  }
}
