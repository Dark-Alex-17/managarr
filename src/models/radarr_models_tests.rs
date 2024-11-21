#[cfg(test)]
mod tests {
  use pretty_assertions::{assert_eq, assert_str_eq};
  use serde_json::json;

  use crate::models::{
    radarr_models::{
      AddMovieSearchResult, BlocklistItem, BlocklistResponse, Collection, Credit, DiskSpace,
      DownloadRecord, DownloadsResponse, Indexer, IndexerSettings, IndexerTestResult,
      MinimumAvailability, Monitor, Movie, MovieHistoryItem, QualityProfile, RadarrSerdeable,
      Release, SystemStatus, Tag, Task, TaskName, Update,
    },
    servarr_models::{HostConfig, Log, LogResponse, QueueEvent, RootFolder, SecurityConfig},
    EnumDisplayStyle, Serdeable,
  };

  #[test]
  fn test_task_name_display() {
    assert_str_eq!(
      TaskName::ApplicationCheckUpdate.to_string(),
      "ApplicationCheckUpdate"
    );
  }

  #[test]
  fn test_minimum_availability_display() {
    assert_str_eq!(MinimumAvailability::Tba.to_string(), "tba");
    assert_str_eq!(MinimumAvailability::Announced.to_string(), "announced");
    assert_str_eq!(MinimumAvailability::InCinemas.to_string(), "inCinemas");
    assert_str_eq!(MinimumAvailability::Released.to_string(), "released");
  }

  #[test]
  fn test_minimum_availability_to_display_str() {
    assert_str_eq!(MinimumAvailability::Tba.to_display_str(), "TBA");
    assert_str_eq!(MinimumAvailability::Announced.to_display_str(), "Announced");
    assert_str_eq!(
      MinimumAvailability::InCinemas.to_display_str(),
      "In Cinemas"
    );
    assert_str_eq!(MinimumAvailability::Released.to_display_str(), "Released");
  }

  #[test]
  fn test_monitor_display() {
    assert_str_eq!(Monitor::MovieOnly.to_string(), "movieOnly");
    assert_str_eq!(
      Monitor::MovieAndCollection.to_string(),
      "movieAndCollection"
    );
    assert_str_eq!(Monitor::None.to_string(), "none");
  }

  #[test]
  fn test_monitor_to_display_str() {
    assert_str_eq!(Monitor::MovieOnly.to_display_str(), "Movie only");
    assert_str_eq!(
      Monitor::MovieAndCollection.to_display_str(),
      "Movie and Collection"
    );
    assert_str_eq!(Monitor::None.to_display_str(), "None");
  }

  #[test]
  fn test_download_record_default_indexer_value() {
    let json = r#"{ 
      "title": "test",
      "status": "test",
      "id": 0,
      "movieId": 0,
      "size": 0,
      "sizeleft": 0,
      "downloadClient": "test"
    }"#;
    let expected_record = DownloadRecord {
      title: "test".to_owned(),
      status: "test".to_owned(),
      id: 0,
      movie_id: 0,
      size: 0,
      sizeleft: 0,
      output_path: None,
      indexer: "".to_owned(),
      download_client: "test".to_owned(),
    };

    let result: DownloadRecord = serde_json::from_str(json).unwrap();

    assert_eq!(result, expected_record);
  }

  #[test]
  fn test_radarr_serdeable_from() {
    let radarr_serdeable = RadarrSerdeable::Value(json!({}));

    let serdeable: Serdeable = Serdeable::from(radarr_serdeable.clone());

    assert_eq!(serdeable, Serdeable::Radarr(radarr_serdeable));
  }

  #[test]
  fn test_radarr_serdeable_from_unit() {
    let radarr_serdeable = RadarrSerdeable::from(());

    assert_eq!(radarr_serdeable, RadarrSerdeable::Value(json!({})));
  }

  #[test]
  fn test_radarr_serdeable_from_value() {
    let value = json!({"test": "test"});

    let radarr_serdeable: RadarrSerdeable = value.clone().into();

    assert_eq!(radarr_serdeable, RadarrSerdeable::Value(value));
  }

  #[test]
  fn test_radarr_serdeable_from_tag() {
    let tag = Tag {
      id: 1,
      ..Tag::default()
    };

    let radarr_serdeable: RadarrSerdeable = tag.clone().into();

    assert_eq!(radarr_serdeable, RadarrSerdeable::Tag(tag));
  }

  #[test]
  fn test_radarr_serdeable_from_blocklist_response() {
    let blocklist_response = BlocklistResponse {
      records: vec![BlocklistItem {
        id: 1,
        ..BlocklistItem::default()
      }],
    };

    let radarr_serdeable: RadarrSerdeable = blocklist_response.clone().into();

    assert_eq!(
      radarr_serdeable,
      RadarrSerdeable::BlocklistResponse(blocklist_response)
    );
  }

  #[test]
  fn test_radarr_serdeable_from_collections() {
    let collections = vec![Collection {
      id: 1,
      ..Collection::default()
    }];

    let radarr_serdeable: RadarrSerdeable = collections.clone().into();

    assert_eq!(radarr_serdeable, RadarrSerdeable::Collections(collections));
  }

  #[test]
  fn test_radarr_serdeable_from_credits() {
    let credits = vec![Credit {
      person_name: "me".to_owned(),
      ..Credit::default()
    }];

    let radarr_serdeable: RadarrSerdeable = credits.clone().into();

    assert_eq!(radarr_serdeable, RadarrSerdeable::Credits(credits));
  }

  #[test]
  fn test_radarr_serdeable_from_disk_spaces() {
    let disk_spaces = vec![DiskSpace {
      free_space: 1,
      total_space: 1,
    }];

    let radarr_serdeable: RadarrSerdeable = disk_spaces.clone().into();

    assert_eq!(radarr_serdeable, RadarrSerdeable::DiskSpaces(disk_spaces));
  }

  #[test]
  fn test_radarr_serdeable_from_host_config() {
    let host_config = HostConfig {
      port: 1234,
      ..HostConfig::default()
    };

    let radarr_serdeable: RadarrSerdeable = host_config.clone().into();

    assert_eq!(radarr_serdeable, RadarrSerdeable::HostConfig(host_config));
  }

  #[test]
  fn test_radarr_serdeable_from_downloads_response() {
    let downloads_response = DownloadsResponse {
      records: vec![DownloadRecord {
        id: 1,
        ..DownloadRecord::default()
      }],
    };

    let radarr_serdeable: RadarrSerdeable = downloads_response.clone().into();

    assert_eq!(
      radarr_serdeable,
      RadarrSerdeable::DownloadsResponse(downloads_response)
    );
  }

  #[test]
  fn test_radarr_serdeable_from_indexers() {
    let indexers = vec![Indexer {
      id: 1,
      ..Indexer::default()
    }];

    let radarr_serdeable: RadarrSerdeable = indexers.clone().into();

    assert_eq!(radarr_serdeable, RadarrSerdeable::Indexers(indexers));
  }

  #[test]
  fn test_radarr_serdeable_from_indexer_settings() {
    let indexer_settings = IndexerSettings {
      id: 1,
      ..IndexerSettings::default()
    };

    let radarr_serdeable: RadarrSerdeable = indexer_settings.clone().into();

    assert_eq!(
      radarr_serdeable,
      RadarrSerdeable::IndexerSettings(indexer_settings)
    );
  }

  #[test]
  fn test_radarr_serdeable_from_log_response() {
    let log_response = LogResponse {
      records: vec![Log {
        level: "info".to_owned(),
        ..Log::default()
      }],
    };

    let radarr_serdeable: RadarrSerdeable = log_response.clone().into();

    assert_eq!(radarr_serdeable, RadarrSerdeable::LogResponse(log_response));
  }

  #[test]
  fn test_radarr_serdeable_from_movie() {
    let movie = Movie {
      id: 1,
      ..Movie::default()
    };

    let radarr_serdeable: RadarrSerdeable = movie.clone().into();

    assert_eq!(radarr_serdeable, RadarrSerdeable::Movie(movie));
  }

  #[test]
  fn test_radarr_serdeable_from_movie_history_items() {
    let movie_history_items = vec![MovieHistoryItem {
      event_type: "test".to_owned(),
      ..MovieHistoryItem::default()
    }];

    let radarr_serdeable: RadarrSerdeable = movie_history_items.clone().into();

    assert_eq!(
      radarr_serdeable,
      RadarrSerdeable::MovieHistoryItems(movie_history_items)
    );
  }

  #[test]
  fn test_radarr_serdeable_from_movies() {
    let movies = vec![Movie {
      id: 1,
      ..Movie::default()
    }];

    let radarr_serdeable: RadarrSerdeable = movies.clone().into();

    assert_eq!(radarr_serdeable, RadarrSerdeable::Movies(movies));
  }

  #[test]
  fn test_radarr_serdeable_from_quality_profiles() {
    let quality_profiles = vec![QualityProfile {
      id: 1,
      ..QualityProfile::default()
    }];

    let radarr_serdeable: RadarrSerdeable = quality_profiles.clone().into();

    assert_eq!(
      radarr_serdeable,
      RadarrSerdeable::QualityProfiles(quality_profiles)
    );
  }

  #[test]
  fn test_radarr_serdeable_from_queue_events() {
    let queue_events = vec![QueueEvent {
      trigger: "test".to_owned(),
      ..QueueEvent::default()
    }];

    let radarr_serdeable: RadarrSerdeable = queue_events.clone().into();

    assert_eq!(radarr_serdeable, RadarrSerdeable::QueueEvents(queue_events));
  }

  #[test]
  fn test_radarr_serdeable_from_releases() {
    let releases = vec![Release {
      size: 1,
      ..Release::default()
    }];

    let radarr_serdeable: RadarrSerdeable = releases.clone().into();

    assert_eq!(radarr_serdeable, RadarrSerdeable::Releases(releases));
  }

  #[test]
  fn test_radarr_serdeable_from_root_folders() {
    let root_folders = vec![RootFolder {
      id: 1,
      ..RootFolder::default()
    }];

    let radarr_serdeable: RadarrSerdeable = root_folders.clone().into();

    assert_eq!(radarr_serdeable, RadarrSerdeable::RootFolders(root_folders));
  }

  #[test]
  fn test_radarr_serdeable_from_security_config() {
    let security_config = SecurityConfig {
      username: Some("Test".to_owned()),
      ..SecurityConfig::default()
    };

    let radarr_serdeable: RadarrSerdeable = security_config.clone().into();

    assert_eq!(
      radarr_serdeable,
      RadarrSerdeable::SecurityConfig(security_config)
    );
  }

  #[test]
  fn test_radarr_serdeable_from_system_status() {
    let system_status = SystemStatus {
      version: "1".to_owned(),
      ..SystemStatus::default()
    };

    let radarr_serdeable: RadarrSerdeable = system_status.clone().into();

    assert_eq!(
      radarr_serdeable,
      RadarrSerdeable::SystemStatus(system_status)
    );
  }

  #[test]
  fn test_radarr_serdeable_from_tags() {
    let tags = vec![Tag {
      id: 1,
      ..Tag::default()
    }];

    let radarr_serdeable: RadarrSerdeable = tags.clone().into();

    assert_eq!(radarr_serdeable, RadarrSerdeable::Tags(tags));
  }

  #[test]
  fn test_radarr_serdeable_from_tasks() {
    let tasks = vec![Task {
      name: "test".to_owned(),
      ..Task::default()
    }];

    let radarr_serdeable: RadarrSerdeable = tasks.clone().into();

    assert_eq!(radarr_serdeable, RadarrSerdeable::Tasks(tasks));
  }

  #[test]
  fn test_radarr_serdeable_from_updates() {
    let updates = vec![Update {
      version: "test".to_owned(),
      ..Update::default()
    }];

    let radarr_serdeable: RadarrSerdeable = updates.clone().into();

    assert_eq!(radarr_serdeable, RadarrSerdeable::Updates(updates));
  }

  #[test]
  fn test_radarr_serdeable_from_add_movie_search_results() {
    let add_movie_search_results = vec![AddMovieSearchResult {
      tmdb_id: 1,
      ..AddMovieSearchResult::default()
    }];

    let radarr_serdeable: RadarrSerdeable = add_movie_search_results.clone().into();

    assert_eq!(
      radarr_serdeable,
      RadarrSerdeable::AddMovieSearchResults(add_movie_search_results)
    );
  }

  #[test]
  fn test_radarr_serdeable_from_indexer_test_results() {
    let indexer_test_results = vec![IndexerTestResult {
      id: 1,
      ..IndexerTestResult::default()
    }];

    let radarr_serdeable: RadarrSerdeable = indexer_test_results.clone().into();

    assert_eq!(
      radarr_serdeable,
      RadarrSerdeable::IndexerTestResults(indexer_test_results)
    );
  }
}
