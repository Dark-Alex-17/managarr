use std::fmt::{Display, Formatter};

use chrono::{DateTime, Utc};
use clap::ValueEnum;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_json::{json, Number, Value};
use strum::EnumIter;

use crate::serde_enum_from;

use super::{HorizontallyScrollableText, Serdeable};

#[cfg(test)]
#[path = "sonarr_models_tests.rs"]
mod sonarr_models_tests;

#[derive(Derivative, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[derivative(Default)]
pub struct Rating {
  #[serde(deserialize_with = "super::from_i64")]
  pub votes: i64,
  #[serde(deserialize_with = "super::from_f64")]
  pub value: f64,
}

impl Eq for Rating {}

#[derive(Derivative, Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Season {
  #[serde(deserialize_with = "super::from_i64")]
  pub season_number: i64,
  pub monitored: bool,
  pub statistics: SeasonStatistics,
}

#[derive(Derivative, Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SeasonStatistics {
  pub next_airing: Option<DateTime<Utc>>,
  pub previous_airing: Option<DateTime<Utc>>,
  #[serde(deserialize_with = "super::from_i64")]
  pub episode_file_count: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub episode_count: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub total_episode_count: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub size_on_disk: i64,
  #[serde(deserialize_with = "super::from_f64")]
  pub percent_of_episodes: f64,
}

impl Eq for SeasonStatistics {}

#[derive(Derivative, Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Series {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub tvdb_id: i64,
  pub title: HorizontallyScrollableText,
  #[serde(deserialize_with = "super::from_i64")]
  pub quality_profile_id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub language_profile_id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub runtime: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub year: i64,
  pub monitored: bool,
  pub series_type: SeriesType,
  pub path: String,
  pub genres: Vec<String>,
  pub tags: Vec<Number>,
  pub ratings: Rating,
  pub ended: bool,
  pub status: SeriesStatus,
  pub overview: String,
  pub network: Option<String>,
  pub season_folder: bool,
  pub certification: Option<String>,
  pub statistics: Option<SeriesStatistics>,
  pub seasons: Option<Vec<Season>>,
}

#[derive(
  Serialize, Deserialize, Default, PartialEq, Eq, Clone, Copy, Debug, EnumIter, ValueEnum,
)]
#[serde(rename_all = "camelCase")]
pub enum SeriesType {
  #[default]
  Standard,
  Daily,
  Anime,
}

impl Display for SeriesType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let series_type = match self {
      SeriesType::Standard => "standard",
      SeriesType::Daily => "daily",
      SeriesType::Anime => "anime",
    };
    write!(f, "{series_type}")
  }
}

impl SeriesType {
  pub fn to_display_str<'a>(self) -> &'a str {
    match self {
      SeriesType::Standard => "Standard",
      SeriesType::Daily => "Daily",
      SeriesType::Anime => "Anime",
    }
  }
}

#[derive(Derivative, Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SeriesStatistics {
  #[serde(deserialize_with = "super::from_i64")]
  pub season_count: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub episode_file_count: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub episode_count: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub total_episode_count: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub size_on_disk: i64,
  #[serde(deserialize_with = "super::from_f64")]
  pub percent_of_episodes: f64,
}

impl Eq for SeriesStatistics {}

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Clone, Copy, Debug, EnumIter)]
#[serde(rename_all = "camelCase")]
pub enum SeriesStatus {
  #[default]
  Continuing,
  Ended,
  Upcoming,
  Deleted,
}

impl Display for SeriesStatus {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let series_status = match self {
      SeriesStatus::Continuing => "continuing",
      SeriesStatus::Ended => "ended",
      SeriesStatus::Upcoming => "upcoming",
      SeriesStatus::Deleted => "deleted",
    };
    write!(f, "{series_status}")
  }
}

impl SeriesStatus {
  pub fn to_display_str<'a>(self) -> &'a str {
    match self {
      SeriesStatus::Continuing => "Continuing",
      SeriesStatus::Ended => "Ended",
      SeriesStatus::Upcoming => "Upcoming",
      SeriesStatus::Deleted => "Deleted",
    }
  }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum SonarrSerdeable {
  Value(Value),
  SeriesVec(Vec<Series>),
  SystemStatus(SystemStatus),
}

impl From<SonarrSerdeable> for Serdeable {
  fn from(value: SonarrSerdeable) -> Serdeable {
    Serdeable::Sonarr(value)
  }
}

impl From<()> for SonarrSerdeable {
  fn from(_: ()) -> Self {
    SonarrSerdeable::Value(json!({}))
  }
}

serde_enum_from!(
  SonarrSerdeable {
    Value(Value),
    SeriesVec(Vec<Series>),
    SystemStatus(SystemStatus),
  }
);

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SystemStatus {
  pub version: String,
  pub start_time: DateTime<Utc>,
}
