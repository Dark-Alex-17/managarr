#[cfg(test)]
mod tests {
  use chrono::Utc;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use serde_json::json;

  use crate::models::lidarr_models::{
    AddArtistSearchResult, DownloadRecord, DownloadStatus, DownloadsResponse, Member,
    MetadataProfile, MonitorType, NewItemMonitorType, SystemStatus,
  };
  use crate::models::servarr_models::{
    DiskSpace, HostConfig, QualityProfile, RootFolder, SecurityConfig, Tag,
  };
  use crate::models::{
    Serdeable,
    lidarr_models::{Artist, ArtistStatistics, ArtistStatus, LidarrSerdeable, Ratings},
  };

  #[test]
  fn test_artist_status_default() {
    assert_eq!(ArtistStatus::default(), ArtistStatus::Continuing);
  }

  #[test]
  fn test_new_item_monitor_type_display() {
    assert_str_eq!(NewItemMonitorType::All.to_string(), "all");
    assert_str_eq!(NewItemMonitorType::None.to_string(), "none");
    assert_str_eq!(NewItemMonitorType::New.to_string(), "new");
  }

  #[test]
  fn test_new_item_monitor_type_to_display_str() {
    assert_str_eq!(NewItemMonitorType::All.to_display_str(), "All Albums");
    assert_str_eq!(NewItemMonitorType::None.to_display_str(), "No New Albums");
    assert_str_eq!(NewItemMonitorType::New.to_display_str(), "New Albums");
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
    assert_str_eq!(MonitorType::All.to_display_str(), "All Albums");
    assert_str_eq!(MonitorType::Future.to_display_str(), "Future Albums");
    assert_str_eq!(MonitorType::Missing.to_display_str(), "Missing Albums");
    assert_str_eq!(MonitorType::Existing.to_display_str(), "Existing Albums");
    assert_str_eq!(MonitorType::First.to_display_str(), "First Album");
    assert_str_eq!(MonitorType::Latest.to_display_str(), "Latest Album");
    assert_str_eq!(MonitorType::None.to_display_str(), "None");
    assert_str_eq!(MonitorType::Unknown.to_display_str(), "Unknown");
  }

  #[test]
  fn test_lidarr_serdeable_from() {
    let lidarr_serdeable = LidarrSerdeable::Value(json!({}));

    let serdeable: Serdeable = Serdeable::from(lidarr_serdeable.clone());

    assert_eq!(serdeable, Serdeable::Lidarr(lidarr_serdeable));
  }

  #[test]
  fn test_lidarr_serdeable_from_unit() {
    let lidarr_serdeable = LidarrSerdeable::from(());

    assert_eq!(lidarr_serdeable, LidarrSerdeable::Value(json!({})));
  }

  #[test]
  fn test_lidarr_serdeable_from_value() {
    let value = json!({"test": "test"});

    let lidarr_serdeable: LidarrSerdeable = value.clone().into();

    assert_eq!(lidarr_serdeable, LidarrSerdeable::Value(value));
  }

  #[test]
  fn test_lidarr_serdeable_from_artists() {
    let artists = vec![Artist {
      id: 1,
      ..Artist::default()
    }];

    let lidarr_serdeable: LidarrSerdeable = artists.clone().into();

    assert_eq!(lidarr_serdeable, LidarrSerdeable::Artists(artists));
  }

  #[test]
  fn test_artist_deserialization() {
    let artist_json = json!({
      "id": 1,
      "artistName": "Test Artist",
      "foreignArtistId": "test-foreign-id",
      "status": "continuing",
      "overview": "Test overview",
      "artistType": "Group",
      "disambiguation": "UK Band",
      "path": "/music/test-artist",
      "members": [
        { "name": "alex", "instrument": "piano" },
        { "name": "madi", "instrument": "vocals" }
      ],
      "qualityProfileId": 1,
      "metadataProfileId": 1,
      "monitored": true,
      "monitorNewItems": "all",
      "genres": ["Rock", "Alternative"],
      "tags": [1, 2],
      "added": "2023-01-01T00:00:00Z",
      "ratings": {
        "votes": 100,
        "value": 4.5
      },
      "statistics": {
        "albumCount": 5,
        "trackFileCount": 50,
        "trackCount": 60,
        "totalTrackCount": 70,
        "sizeOnDisk": 1000000000,
        "percentOfTracks": 83.33
      }
    });
    let expected_members_vec = vec![
      Member {
        name: Some("alex".to_string()),
        instrument: Some("piano".to_string()),
      },
      Member {
        name: Some("madi".to_string()),
        instrument: Some("vocals".to_string()),
      },
    ];

    let artist: Artist = serde_json::from_value(artist_json).unwrap();

    assert_eq!(artist.id, 1);
    assert_str_eq!(artist.artist_name.text, "Test Artist");
    assert_str_eq!(artist.foreign_artist_id, "test-foreign-id");
    assert_eq!(artist.status, ArtistStatus::Continuing);
    assert_some_eq_x!(&artist.overview, "Test overview");
    assert_some_eq_x!(&artist.artist_type, "Group");
    assert_some_eq_x!(&artist.disambiguation, "UK Band");
    assert_str_eq!(artist.path, "/music/test-artist");
    assert_some_eq_x!(&artist.members, &expected_members_vec);
    assert_eq!(artist.quality_profile_id, 1);
    assert_eq!(artist.metadata_profile_id, 1);
    assert!(artist.monitored);
    assert_eq!(artist.monitor_new_items, NewItemMonitorType::All);
    assert_eq!(artist.genres, vec!["Rock", "Alternative"]);
    assert_eq!(artist.tags.len(), 2);
    assert_some!(&artist.ratings);
    assert_some!(&artist.statistics);

    let ratings = artist.ratings.unwrap();
    assert_eq!(ratings.votes, 100);
    assert_eq!(ratings.value, 4.5);

    let stats = artist.statistics.unwrap();
    assert_eq!(stats.album_count, 5);
    assert_eq!(stats.track_file_count, 50);
    assert_eq!(stats.track_count, 60);
    assert_eq!(stats.total_track_count, 70);
    assert_eq!(stats.size_on_disk, 1000000000);
    assert_eq!(stats.percent_of_tracks, 83.33);
  }

  #[test]
  fn test_artist_status_deserialization() {
    assert_eq!(
      serde_json::from_str::<ArtistStatus>("\"continuing\"").unwrap(),
      ArtistStatus::Continuing
    );
    assert_eq!(
      serde_json::from_str::<ArtistStatus>("\"ended\"").unwrap(),
      ArtistStatus::Ended
    );
    assert_eq!(
      serde_json::from_str::<ArtistStatus>("\"deleted\"").unwrap(),
      ArtistStatus::Deleted
    );
  }

  #[test]
  fn test_ratings_equality() {
    let ratings1 = Ratings {
      votes: 100,
      value: 4.5,
    };
    let ratings2 = Ratings {
      votes: 100,
      value: 4.5,
    };
    let ratings3 = Ratings {
      votes: 50,
      value: 3.0,
    };

    assert_eq!(ratings1, ratings2);
    assert_ne!(ratings1, ratings3);
  }

  #[test]
  fn test_artist_statistics_equality() {
    let stats1 = ArtistStatistics {
      album_count: 5,
      track_file_count: 50,
      track_count: 60,
      total_track_count: 70,
      size_on_disk: 1000000000,
      percent_of_tracks: 83.33,
    };
    let stats2 = ArtistStatistics {
      album_count: 5,
      track_file_count: 50,
      track_count: 60,
      total_track_count: 70,
      size_on_disk: 1000000000,
      percent_of_tracks: 83.33,
    };
    let stats3 = ArtistStatistics::default();

    assert_eq!(stats1, stats2);
    assert_ne!(stats1, stats3);
  }

  #[test]
  fn test_artist_with_optional_fields_none() {
    let artist_json = json!({
      "id": 1,
      "artistName": "Test Artist",
      "foreignArtistId": "",
      "status": "continuing",
      "path": "",
      "qualityProfileId": 1,
      "metadataProfileId": 1,
      "monitored": false,
      "monitorNewItems": "all",
      "genres": [],
      "tags": [],
      "added": "2023-01-01T00:00:00Z"
    });

    let artist: Artist = serde_json::from_value(artist_json).unwrap();

    assert_none!(&artist.overview);
    assert_none!(&artist.artist_type);
    assert_none!(&artist.disambiguation);
    assert_eq!(artist.monitor_new_items, NewItemMonitorType::All);
    assert_none!(&artist.ratings);
    assert_none!(&artist.statistics);
  }

  #[test]
  fn test_lidarr_serdeable_from_artist() {
    let artist = Artist {
      id: 1,
      ..Artist::default()
    };

    let lidarr_serdeable: LidarrSerdeable = artist.clone().into();

    assert_eq!(lidarr_serdeable, LidarrSerdeable::Artist(artist));
  }

  #[test]
  fn test_lidarr_serdeable_from_disk_spaces() {
    let disk_spaces = vec![DiskSpace {
      free_space: 1,
      total_space: 1,
    }];

    let lidarr_serdeable: LidarrSerdeable = disk_spaces.clone().into();

    assert_eq!(lidarr_serdeable, LidarrSerdeable::DiskSpaces(disk_spaces));
  }

  #[test]
  fn test_lidarr_serdeable_from_downloads_response() {
    let downloads_response = DownloadsResponse {
      records: vec![DownloadRecord {
        id: 1,
        ..DownloadRecord::default()
      }],
    };

    let lidarr_serdeable: LidarrSerdeable = downloads_response.clone().into();

    assert_eq!(
      lidarr_serdeable,
      LidarrSerdeable::DownloadsResponse(downloads_response)
    );
  }

  #[test]
  fn test_lidarr_serdeable_from_metadata_profiles() {
    let metadata_profiles = vec![MetadataProfile {
      id: 1,
      name: "Standard".to_owned(),
    }];

    let lidarr_serdeable: LidarrSerdeable = metadata_profiles.clone().into();

    assert_eq!(
      lidarr_serdeable,
      LidarrSerdeable::MetadataProfiles(metadata_profiles)
    );
  }

  #[test]
  fn test_lidarr_serdeable_from_host_config() {
    let host_config = HostConfig {
      port: 8686,
      ..HostConfig::default()
    };

    let lidarr_serdeable: LidarrSerdeable = host_config.clone().into();

    assert_eq!(lidarr_serdeable, LidarrSerdeable::HostConfig(host_config));
  }

  #[test]
  fn test_lidarr_serdeable_from_quality_profiles() {
    let quality_profiles = vec![QualityProfile {
      id: 1,
      name: "Any".to_owned(),
    }];

    let lidarr_serdeable: LidarrSerdeable = quality_profiles.clone().into();

    assert_eq!(
      lidarr_serdeable,
      LidarrSerdeable::QualityProfiles(quality_profiles)
    );
  }

  #[test]
  fn test_lidarr_serdeable_from_root_folders() {
    let root_folders = vec![RootFolder {
      id: 1,
      path: "/music".to_owned(),
      accessible: true,
      free_space: 1000000,
      unmapped_folders: None,
    }];

    let lidarr_serdeable: LidarrSerdeable = root_folders.clone().into();

    assert_eq!(lidarr_serdeable, LidarrSerdeable::RootFolders(root_folders));
  }

  #[test]
  fn test_lidarr_serdeable_from_security_config() {
    let security_config = SecurityConfig {
      api_key: "test-key".to_owned(),
      ..SecurityConfig::default()
    };

    let lidarr_serdeable: LidarrSerdeable = security_config.clone().into();

    assert_eq!(
      lidarr_serdeable,
      LidarrSerdeable::SecurityConfig(security_config)
    );
  }

  #[test]
  fn test_lidarr_serdeable_from_system_status() {
    let system_status = SystemStatus {
      version: "1.0.0".to_owned(),
      start_time: Utc::now(),
    };

    let lidarr_serdeable: LidarrSerdeable = system_status.clone().into();

    assert_eq!(
      lidarr_serdeable,
      LidarrSerdeable::SystemStatus(system_status)
    );
  }

  #[test]
  fn test_lidarr_serdeable_from_tags() {
    let tags = vec![Tag {
      id: 1,
      label: "rock".to_owned(),
    }];

    let lidarr_serdeable: LidarrSerdeable = tags.clone().into();

    assert_eq!(lidarr_serdeable, LidarrSerdeable::Tags(tags));
  }

  #[test]
  fn test_artist_status_display() {
    assert_str_eq!(ArtistStatus::Continuing.to_string(), "continuing");
    assert_str_eq!(ArtistStatus::Ended.to_string(), "ended");
    assert_str_eq!(ArtistStatus::Deleted.to_string(), "deleted");
  }

  #[test]
  fn test_artist_status_to_display_str() {
    assert_str_eq!(ArtistStatus::Continuing.to_display_str(), "Continuing");
    assert_str_eq!(ArtistStatus::Ended.to_display_str(), "Ended");
    assert_str_eq!(ArtistStatus::Deleted.to_display_str(), "Deleted");
  }

  #[test]
  fn test_download_status_display() {
    assert_str_eq!(DownloadStatus::Unknown.to_string(), "unknown");
    assert_str_eq!(DownloadStatus::Queued.to_string(), "queued");
    assert_str_eq!(DownloadStatus::Paused.to_string(), "paused");
    assert_str_eq!(DownloadStatus::Downloading.to_string(), "downloading");
    assert_str_eq!(DownloadStatus::Completed.to_string(), "completed");
    assert_str_eq!(DownloadStatus::Failed.to_string(), "failed");
    assert_str_eq!(DownloadStatus::Warning.to_string(), "warning");
    assert_str_eq!(DownloadStatus::Delay.to_string(), "delay");
    assert_str_eq!(
      DownloadStatus::DownloadClientUnavailable.to_string(),
      "downloadClientUnavailable"
    );
    assert_str_eq!(DownloadStatus::Fallback.to_string(), "fallback");
  }

  #[test]
  fn test_download_status_to_display_str() {
    assert_str_eq!(DownloadStatus::Unknown.to_display_str(), "Unknown");
    assert_str_eq!(DownloadStatus::Queued.to_display_str(), "Queued");
    assert_str_eq!(DownloadStatus::Paused.to_display_str(), "Paused");
    assert_str_eq!(DownloadStatus::Downloading.to_display_str(), "Downloading");
    assert_str_eq!(DownloadStatus::Completed.to_display_str(), "Completed");
    assert_str_eq!(DownloadStatus::Failed.to_display_str(), "Failed");
    assert_str_eq!(DownloadStatus::Warning.to_display_str(), "Warning");
    assert_str_eq!(DownloadStatus::Delay.to_display_str(), "Delay");
    assert_str_eq!(
      DownloadStatus::DownloadClientUnavailable.to_display_str(),
      "Download Client Unavailable"
    );
    assert_str_eq!(DownloadStatus::Fallback.to_display_str(), "Fallback");
  }

  #[test]
  fn test_add_artist_search_result_deserialization() {
    let search_result_json = json!({
      "foreignArtistId": "test-foreign-id",
      "artistName": "Test Artist",
      "status": "continuing",
      "overview": "Test overview",
      "artistType": "Group",
      "disambiguation": "UK Band",
      "genres": ["Rock", "Alternative"],
      "ratings": {
        "votes": 100,
        "value": 4.5
      }
    });

    let search_result: AddArtistSearchResult = serde_json::from_value(search_result_json).unwrap();

    assert_str_eq!(search_result.foreign_artist_id, "test-foreign-id");
    assert_str_eq!(search_result.artist_name.text, "Test Artist");
    assert_eq!(search_result.status, ArtistStatus::Continuing);
    assert_some_eq_x!(&search_result.overview, "Test overview");
    assert_some_eq_x!(&search_result.artist_type, "Group");
    assert_some_eq_x!(&search_result.disambiguation, "UK Band");
    assert_eq!(search_result.genres, vec!["Rock", "Alternative"]);
    assert_some!(&search_result.ratings);

    let ratings = search_result.ratings.unwrap();
    assert_eq!(ratings.votes, 100);
    assert_eq!(ratings.value, 4.5);
  }

  #[test]
  fn test_add_artist_search_result_with_optional_fields_none() {
    let search_result_json = json!({
      "foreignArtistId": "test-foreign-id",
      "artistName": "Test Artist",
      "status": "ended",
      "genres": []
    });

    let search_result: AddArtistSearchResult = serde_json::from_value(search_result_json).unwrap();

    assert_str_eq!(search_result.foreign_artist_id, "test-foreign-id");
    assert_str_eq!(search_result.artist_name.text, "Test Artist");
    assert_eq!(search_result.status, ArtistStatus::Ended);
    assert_none!(&search_result.overview);
    assert_none!(&search_result.artist_type);
    assert_none!(&search_result.disambiguation);
    assert!(search_result.genres.is_empty());
    assert_none!(&search_result.ratings);
  }

  #[test]
  fn test_lidarr_serdeable_from_add_artist_search_results() {
    let search_results = vec![AddArtistSearchResult {
      foreign_artist_id: "test-id".to_owned(),
      ..AddArtistSearchResult::default()
    }];

    let lidarr_serdeable: LidarrSerdeable = search_results.clone().into();

    assert_eq!(
      lidarr_serdeable,
      LidarrSerdeable::AddArtistSearchResults(search_results)
    );
  }
}
