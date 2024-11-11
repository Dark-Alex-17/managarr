#[cfg(test)]
mod tests {
  use pretty_assertions::{assert_eq, assert_str_eq};
  use serde_json::json;

  use crate::models::{
    sonarr_models::{Series, SeriesStatus, SeriesType, SonarrSerdeable, SystemStatus},
    Serdeable,
  };

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
}
