#[cfg(test)]
#[allow(dead_code)]
pub mod test_utils {
  use crate::models::HorizontallyScrollableText;
  use crate::models::lidarr_models::{
    AddArtistSearchResult, Artist, ArtistStatistics, ArtistStatus, DownloadRecord, DownloadStatus,
    DownloadsResponse, EditArtistParams, Member, MetadataProfile, NewItemMonitorType, Ratings,
    SystemStatus,
  };
  use crate::models::servarr_models::{QualityProfile, RootFolder, Tag};
  use bimap::BiMap;
  use chrono::DateTime;
  use serde_json::Number;

  pub const ADD_ARTIST_SEARCH_RESULT_JSON: &str = r#"{
    "foreignArtistId": "test-foreign-id",
    "artistName": "Test Artist",
    "status": "continuing",
    "overview": "some interesting description of the artist",
    "artistType": "Person",
    "disambiguation": "American pianist",
    "genres": ["soundtrack"],
    "ratings": { "votes": 15, "value": 8.4 }
  }"#;

  pub const ARTIST_JSON: &str = r#"{
    "id": 1,
    "artistName": "Test Artist",
    "foreignArtistId": "test-foreign-id",
    "status": "continuing",
    "overview": "some interesting description of the artist",
    "artistType": "Person",
    "disambiguation": "American pianist",
    "path": "/music/test-artist",
    "members": [{"name": "alex", "instrument": "piano"}],
    "qualityProfileId": 1,
    "metadataProfileId": 1,
    "monitored": true,
    "monitorNewItems": "all",
    "genres": ["soundtrack"],
    "tags": [1],
    "added": "2023-01-01T00:00:00Z",
    "ratings": { "votes": 15, "value": 8.4 },
    "statistics": {
      "albumCount": 1,
      "trackFileCount": 15,
      "trackCount": 15,
      "totalTrackCount": 15,
      "sizeOnDisk": 12345,
      "percentOfTracks": 99.9
    }
  }"#;

  pub fn member() -> Member {
    Member {
      name: Some("alex".to_owned()),
      instrument: Some("piano".to_owned()),
    }
  }

  pub fn ratings() -> Ratings {
    Ratings {
      votes: 15,
      value: 8.4,
    }
  }

  pub fn artist_statistics() -> ArtistStatistics {
    ArtistStatistics {
      album_count: 1,
      track_file_count: 15,
      track_count: 15,
      total_track_count: 15,
      size_on_disk: 12345,
      percent_of_tracks: 99.9,
    }
  }

  pub fn artist() -> Artist {
    Artist {
      id: 1,
      artist_name: "Alex".into(),
      foreign_artist_id: "test-foreign-id".to_owned(),
      status: ArtistStatus::Continuing,
      overview: Some("some interesting description of the artist".to_owned()),
      artist_type: Some("Person".to_owned()),
      disambiguation: Some("American pianist".to_owned()),
      members: Some(vec![member()]),
      path: "/nfs/music/test-artist".to_owned(),
      quality_profile_id: quality_profile().id,
      metadata_profile_id: metadata_profile().id,
      monitored: true,
      monitor_new_items: NewItemMonitorType::All,
      genres: vec!["soundtrack".to_owned()],
      tags: vec![Number::from(tag().id)],
      added: DateTime::from(DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z").unwrap()),
      ratings: Some(ratings()),
      statistics: Some(artist_statistics()),
    }
  }

  pub fn quality_profile() -> QualityProfile {
    QualityProfile {
      id: 1,
      name: "Lossless".to_owned(),
    }
  }

  pub fn quality_profile_map() -> BiMap<i64, String> {
    let quality_profile = quality_profile();
    BiMap::from_iter(vec![(quality_profile.id, quality_profile.name)])
  }

  pub fn metadata_profile() -> MetadataProfile {
    MetadataProfile {
      id: 1,
      name: "Standard".to_owned(),
    }
  }

  pub fn metadata_profile_map() -> BiMap<i64, String> {
    let metadata_profile = metadata_profile();
    BiMap::from_iter(vec![(metadata_profile.id, metadata_profile.name)])
  }

  pub fn tag() -> Tag {
    Tag {
      id: 1,
      label: "alex".to_owned(),
    }
  }

  pub fn tags_map() -> BiMap<i64, String> {
    let tag = tag();
    BiMap::from_iter(vec![(tag.id, tag.label)])
  }

  pub fn download_record() -> DownloadRecord {
    DownloadRecord {
      title: "Test download title".to_owned(),
      status: DownloadStatus::Downloading,
      id: 1,
      album_id: Some(Number::from(1i64)),
      artist_id: Some(Number::from(1i64)),
      size: 3543348019f64,
      sizeleft: 1771674009f64,
      output_path: Some(HorizontallyScrollableText::from("/nfs/music/alex/album")),
      indexer: "kickass torrents".to_owned(),
      download_client: Some("transmission".to_owned()),
    }
  }

  pub fn downloads_response() -> DownloadsResponse {
    DownloadsResponse {
      records: vec![download_record()],
    }
  }

  pub fn system_status() -> SystemStatus {
    SystemStatus {
      version: "1.0".to_owned(),
      start_time: DateTime::from(DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z").unwrap()),
    }
  }

  pub fn root_folder() -> RootFolder {
    RootFolder {
      id: 1,
      path: "/nfs".to_owned(),
      accessible: true,
      free_space: 219902325555200,
      unmapped_folders: None,
    }
  }

  pub fn edit_artist_params() -> EditArtistParams {
    EditArtistParams {
      artist_id: artist().id,
      monitored: Some(true),
      monitor_new_items: Some(NewItemMonitorType::All),
      quality_profile_id: Some(quality_profile().id),
      metadata_profile_id: Some(metadata_profile().id),
      root_folder_path: Some("/nfs/music/test-artist".to_owned()),
      tags: Some(vec![tag().id]),
      tag_input_string: Some("alex".to_owned()),
      clear_tags: false,
    }
  }

  pub fn add_artist_search_result() -> AddArtistSearchResult {
    AddArtistSearchResult {
      foreign_artist_id: "test-foreign-id".to_owned(),
      artist_name: "Test Artist".into(),
      status: ArtistStatus::Continuing,
      overview: Some("some interesting description of the artist".to_owned()),
      artist_type: Some("Person".to_owned()),
      disambiguation: Some("American pianist".to_owned()),
      genres: vec!["soundtrack".to_owned()],
      ratings: Some(ratings()),
    }
  }
}
