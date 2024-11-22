#[cfg(test)]
mod tests {
  use pretty_assertions::{assert_eq, assert_str_eq};
  use serde_json::json;

  use crate::models::{
    servarr_models::{
      DiskSpace, HostConfig, Indexer, Log, LogResponse, QualityProfile, QueueEvent, Release,
      RootFolder, SecurityConfig, Tag,
    },
    sonarr_models::{
      BlocklistItem, BlocklistResponse, DownloadRecord, DownloadsResponse, Episode,
      IndexerSettings, Series, SeriesStatus, SeriesType, SonarrHistoryEventType, SonarrHistoryItem,
      SonarrSerdeable, SonarrTask, SonarrTaskName, SystemStatus,
    },
    EnumDisplayStyle, Serdeable,
  };

  #[test]
  fn test_episode_display() {
    let episode = Episode {
      title: Some("Test Title".to_owned()),
      ..Episode::default()
    };

    assert_str_eq!(Episode::default().to_string(), "");
    assert_str_eq!(episode.to_string(), "Test Title");
  }

  #[test]
  fn test_series_status_display() {
    assert_str_eq!(SeriesStatus::Continuing.to_string(), "continuing");
    assert_str_eq!(SeriesStatus::Ended.to_string(), "ended");
    assert_str_eq!(SeriesStatus::Upcoming.to_string(), "upcoming");
    assert_str_eq!(SeriesStatus::Deleted.to_string(), "deleted");
  }

  #[test]
  fn test_series_status_to_display_str() {
    assert_str_eq!(SeriesStatus::Continuing.to_display_str(), "Continuing");
    assert_str_eq!(SeriesStatus::Ended.to_display_str(), "Ended");
    assert_str_eq!(SeriesStatus::Upcoming.to_display_str(), "Upcoming");
    assert_str_eq!(SeriesStatus::Deleted.to_display_str(), "Deleted");
  }

  #[test]
  fn test_series_type_display() {
    assert_str_eq!(SeriesType::Standard.to_string(), "standard");
    assert_str_eq!(SeriesType::Daily.to_string(), "daily");
    assert_str_eq!(SeriesType::Anime.to_string(), "anime");
  }

  #[test]
  fn test_series_type_to_display_str() {
    assert_str_eq!(SeriesType::Standard.to_display_str(), "Standard");
    assert_str_eq!(SeriesType::Daily.to_display_str(), "Daily");
    assert_str_eq!(SeriesType::Anime.to_display_str(), "Anime");
  }

  #[test]
  fn test_sonarr_history_event_type_display() {
    assert_str_eq!(SonarrHistoryEventType::Unknown.to_string(), "unknown",);
    assert_str_eq!(SonarrHistoryEventType::Grabbed.to_string(), "grabbed",);
    assert_str_eq!(
      SonarrHistoryEventType::SeriesFolderImported.to_string(),
      "seriesFolderImported",
    );
    assert_str_eq!(
      SonarrHistoryEventType::DownloadFolderImported.to_string(),
      "downloadFolderImported",
    );
    assert_str_eq!(
      SonarrHistoryEventType::DownloadFailed.to_string(),
      "downloadFailed",
    );
    assert_str_eq!(
      SonarrHistoryEventType::EpisodeFileDeleted.to_string(),
      "episodeFileDeleted",
    );
    assert_str_eq!(
      SonarrHistoryEventType::EpisodeFileRenamed.to_string(),
      "episodeFileRenamed",
    );
    assert_str_eq!(
      SonarrHistoryEventType::DownloadIgnored.to_string(),
      "downloadIgnored",
    );
  }

  #[test]
  fn test_sonarr_history_event_type_to_display_str() {
    assert_str_eq!(SonarrHistoryEventType::Unknown.to_display_str(), "Unknown",);
    assert_str_eq!(SonarrHistoryEventType::Grabbed.to_display_str(), "Grabbed",);
    assert_str_eq!(
      SonarrHistoryEventType::SeriesFolderImported.to_display_str(),
      "Series Folder Imported",
    );
    assert_str_eq!(
      SonarrHistoryEventType::DownloadFolderImported.to_display_str(),
      "Download Folder Imported",
    );
    assert_str_eq!(
      SonarrHistoryEventType::DownloadFailed.to_display_str(),
      "Download Failed",
    );
    assert_str_eq!(
      SonarrHistoryEventType::EpisodeFileDeleted.to_display_str(),
      "Episode File Deleted",
    );
    assert_str_eq!(
      SonarrHistoryEventType::EpisodeFileRenamed.to_display_str(),
      "Episode File Renamed",
    );
    assert_str_eq!(
      SonarrHistoryEventType::DownloadIgnored.to_display_str(),
      "Download Ignored",
    );
  }

  #[test]
  fn test_task_name_display() {
    assert_str_eq!(
      SonarrTaskName::ApplicationUpdateCheck.to_string(),
      "ApplicationUpdateCheck"
    );
  }

  #[test]
  fn test_sonarr_serdeable_from() {
    let sonarr_serdeable = SonarrSerdeable::Value(json!({}));

    let serdeable: Serdeable = Serdeable::from(sonarr_serdeable.clone());

    assert_eq!(serdeable, Serdeable::Sonarr(sonarr_serdeable));
  }

  #[test]
  fn test_sonarr_serdeable_from_unit() {
    let sonarr_serdeable = SonarrSerdeable::from(());

    assert_eq!(sonarr_serdeable, SonarrSerdeable::Value(json!({})));
  }

  #[test]
  fn test_sonarr_serdeable_from_value() {
    let value = json!({"test": "test"});

    let sonarr_serdeable: SonarrSerdeable = value.clone().into();

    assert_eq!(sonarr_serdeable, SonarrSerdeable::Value(value));
  }

  #[test]
  fn test_sonarr_serdeable_from_episode() {
    let episode = Episode {
      id: 1,
      ..Episode::default()
    };

    let sonarr_serdeable: SonarrSerdeable = episode.clone().into();

    assert_eq!(sonarr_serdeable, SonarrSerdeable::Episode(episode));
  }

  #[test]
  fn test_sonarr_serdeable_from_episodes() {
    let episodes = vec![Episode {
      id: 1,
      ..Episode::default()
    }];

    let sonarr_serdeable: SonarrSerdeable = episodes.clone().into();

    assert_eq!(sonarr_serdeable, SonarrSerdeable::Episodes(episodes));
  }

  #[test]
  fn test_sonarr_serdeable_from_host_config() {
    let host_config = HostConfig {
      port: 1234,
      ..HostConfig::default()
    };

    let sonarr_serdeable: SonarrSerdeable = host_config.clone().into();

    assert_eq!(sonarr_serdeable, SonarrSerdeable::HostConfig(host_config));
  }

  #[test]
  fn test_sonarr_serdeable_from_indexers() {
    let indexers = vec![Indexer {
      id: 1,
      ..Indexer::default()
    }];

    let sonarr_serdeable: SonarrSerdeable = indexers.clone().into();

    assert_eq!(sonarr_serdeable, SonarrSerdeable::Indexers(indexers));
  }

  #[test]
  fn test_sonarr_serdeable_from_indexer_settings() {
    let indexer_settings = IndexerSettings {
      id: 1,
      ..IndexerSettings::default()
    };

    let sonarr_serdeable: SonarrSerdeable = indexer_settings.clone().into();

    assert_eq!(
      sonarr_serdeable,
      SonarrSerdeable::IndexerSettings(indexer_settings)
    );
  }

  #[test]
  fn test_sonarr_serdeable_from_series_vec() {
    let series_vec = vec![Series {
      id: 1,
      ..Series::default()
    }];

    let sonarr_serdeable: SonarrSerdeable = series_vec.clone().into();

    assert_eq!(sonarr_serdeable, SonarrSerdeable::SeriesVec(series_vec));
  }

  #[test]
  fn test_sonarr_serdeable_from_series() {
    let series = Series {
      id: 1,
      ..Series::default()
    };

    let sonarr_serdeable: SonarrSerdeable = series.clone().into();

    assert_eq!(sonarr_serdeable, SonarrSerdeable::Series(series));
  }

  #[test]
  fn test_sonarr_serdeable_from_sonarr_history_items() {
    let history_items = vec![SonarrHistoryItem {
      id: 1,
      ..SonarrHistoryItem::default()
    }];
    let sonarr_serdeable: SonarrSerdeable = history_items.clone().into();

    assert_eq!(
      sonarr_serdeable,
      SonarrSerdeable::SonarrHistoryItems(history_items)
    );
  }

  #[test]
  fn test_sonarr_serdeable_from_system_status() {
    let system_status = SystemStatus {
      version: "1".to_owned(),
      ..SystemStatus::default()
    };

    let sonarr_serdeable: SonarrSerdeable = system_status.clone().into();

    assert_eq!(
      sonarr_serdeable,
      SonarrSerdeable::SystemStatus(system_status)
    );
  }

  #[test]
  fn test_sonarr_serdeable_from_blocklist_response() {
    let blocklist_response = BlocklistResponse {
      records: vec![BlocklistItem {
        id: 1,
        ..BlocklistItem::default()
      }],
    };

    let sonarr_serdeable: SonarrSerdeable = blocklist_response.clone().into();

    assert_eq!(
      sonarr_serdeable,
      SonarrSerdeable::BlocklistResponse(blocklist_response)
    );
  }

  #[test]
  fn test_sonarr_serdeable_from_downloads_response() {
    let downloads_response = DownloadsResponse {
      records: vec![DownloadRecord {
        id: 1,
        ..DownloadRecord::default()
      }],
    };
    let sonarr_serdeable: SonarrSerdeable = downloads_response.clone().into();

    assert_eq!(
      sonarr_serdeable,
      SonarrSerdeable::DownloadsResponse(downloads_response)
    );
  }

  #[test]
  fn test_sonarr_serdeable_from_disk_spaces() {
    let disk_spaces = vec![DiskSpace {
      free_space: 1,
      total_space: 1,
    }];

    let sonarr_serdeable: SonarrSerdeable = disk_spaces.clone().into();

    assert_eq!(sonarr_serdeable, SonarrSerdeable::DiskSpaces(disk_spaces));
  }

  #[test]
  fn test_sonarr_serdeable_from_log_response() {
    let log_response = LogResponse {
      records: vec![Log {
        level: "info".to_owned(),
        ..Log::default()
      }],
    };

    let sonarr_serdeable: SonarrSerdeable = log_response.clone().into();

    assert_eq!(sonarr_serdeable, SonarrSerdeable::LogResponse(log_response));
  }

  #[test]
  fn test_sonarr_serdeable_from_quality_profiles() {
    let quality_profiles = vec![QualityProfile {
      name: "Test Profile".to_owned(),
      id: 1,
    }];

    let sonarr_serdeable: SonarrSerdeable = quality_profiles.clone().into();

    assert_eq!(
      sonarr_serdeable,
      SonarrSerdeable::QualityProfiles(quality_profiles)
    );
  }

  #[test]
  fn test_sonarr_serdeable_from_queue_events() {
    let queue_events = vec![QueueEvent {
      trigger: "test".to_owned(),
      ..QueueEvent::default()
    }];

    let sonarr_serdeable: SonarrSerdeable = queue_events.clone().into();

    assert_eq!(sonarr_serdeable, SonarrSerdeable::QueueEvents(queue_events));
  }

  #[test]
  fn test_sonarr_serdeable_from_releases() {
    let releases = vec![Release {
      size: 1,
      ..Release::default()
    }];

    let sonarr_serdeable: SonarrSerdeable = releases.clone().into();

    assert_eq!(sonarr_serdeable, SonarrSerdeable::Releases(releases));
  }

  #[test]
  fn test_sonarr_serdeable_from_root_folders() {
    let root_folders = vec![RootFolder {
      id: 1,
      ..RootFolder::default()
    }];

    let sonarr_serdeable: SonarrSerdeable = root_folders.clone().into();

    assert_eq!(sonarr_serdeable, SonarrSerdeable::RootFolders(root_folders));
  }

  #[test]
  fn test_sonarr_serdeable_from_security_config() {
    let security_config = SecurityConfig {
      username: Some("Test".to_owned()),
      ..SecurityConfig::default()
    };

    let sonarr_serdeable: SonarrSerdeable = security_config.clone().into();

    assert_eq!(
      sonarr_serdeable,
      SonarrSerdeable::SecurityConfig(security_config)
    );
  }

  #[test]
  fn test_sonarr_serdeable_from_tag() {
    let tag = Tag {
      id: 1,
      ..Tag::default()
    };

    let sonarr_serdeable: SonarrSerdeable = tag.clone().into();

    assert_eq!(sonarr_serdeable, SonarrSerdeable::Tag(tag));
  }

  #[test]
  fn test_sonarr_serdeable_from_tags() {
    let tags = vec![Tag {
      id: 1,
      ..Tag::default()
    }];

    let sonarr_serdeable: SonarrSerdeable = tags.clone().into();

    assert_eq!(sonarr_serdeable, SonarrSerdeable::Tags(tags));
  }

  #[test]
  fn test_sonarr_serdeable_from_tasks() {
    let tasks = vec![SonarrTask {
      name: "test".to_owned(),
      ..SonarrTask::default()
    }];

    let sonarr_serdeable: SonarrSerdeable = tasks.clone().into();

    assert_eq!(sonarr_serdeable, SonarrSerdeable::Tasks(tasks));
  }
}
