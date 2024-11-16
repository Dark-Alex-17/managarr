#[cfg(test)]
mod tests {
  use pretty_assertions::{assert_eq, assert_str_eq};
  use serde_json::json;

  use crate::models::{
    sonarr_models::{
      BlocklistItem, BlocklistResponse, Episode, Log, LogResponse, QualityProfile, Series,
      SeriesStatus, SeriesType, SonarrSerdeable, SystemStatus,
    },
    Serdeable,
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
  fn test_sonarr_serdeable_from_series() {
    let series = vec![Series {
      id: 1,
      ..Series::default()
    }];

    let sonarr_serdeable: SonarrSerdeable = series.clone().into();

    assert_eq!(sonarr_serdeable, SonarrSerdeable::SeriesVec(series));
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
}
