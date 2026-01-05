use chrono::{DateTime, Utc};
use derivative::Derivative;
use enum_display_style_derive::EnumDisplayStyle;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use strum::EnumIter;

use super::{HorizontallyScrollableText, Serdeable};
use crate::serde_enum_from;

#[cfg(test)]
#[path = "lidarr_models_tests.rs"]
mod lidarr_models_tests;

#[derive(Derivative, Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  pub artist_name: HorizontallyScrollableText,
  pub foreign_artist_id: String,
  pub status: ArtistStatus,
  pub overview: Option<String>,
  pub artist_type: Option<String>,
  pub disambiguation: Option<String>,
  pub path: String,
  #[serde(deserialize_with = "super::from_i64")]
  pub quality_profile_id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub metadata_profile_id: i64,
  pub monitored: bool,
  pub genres: Vec<String>,
  pub tags: Vec<Number>,
  pub added: DateTime<Utc>,
  pub ratings: Option<Ratings>,
  pub statistics: Option<ArtistStatistics>,
}

#[derive(
  Serialize,
  Deserialize,
  Default,
  PartialEq,
  Eq,
  Clone,
  Copy,
  Debug,
  strum::Display,
  EnumDisplayStyle,
)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
pub enum ArtistStatus {
  #[default]
  Continuing,
  Ended,
  Deleted,
}

#[derive(Derivative, Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Ratings {
  #[serde(deserialize_with = "super::from_i64")]
  pub votes: i64,
  #[serde(deserialize_with = "super::from_f64")]
  pub value: f64,
}

impl Eq for Ratings {}

#[derive(Derivative, Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ArtistStatistics {
  #[serde(deserialize_with = "super::from_i64")]
  pub album_count: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub track_file_count: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub track_count: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub total_track_count: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub size_on_disk: i64,
  #[serde(deserialize_with = "super::from_f64")]
  pub percent_of_tracks: f64,
}

impl Eq for ArtistStatistics {}

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct MetadataProfile {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  pub name: String,
}

impl From<(&i64, &String)> for MetadataProfile {
  fn from(value: (&i64, &String)) -> Self {
    MetadataProfile {
      id: *value.0,
      name: value.1.clone(),
    }
  }
}

#[derive(Derivative, Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DownloadRecord {
  pub title: String,
  pub status: DownloadStatus,
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  pub album_id: Option<Number>,
  pub artist_id: Option<Number>,
  #[serde(deserialize_with = "super::from_f64")]
  pub size: f64,
  #[serde(deserialize_with = "super::from_f64")]
  pub sizeleft: f64,
  pub output_path: Option<HorizontallyScrollableText>,
  #[serde(default)]
  pub indexer: String,
  pub download_client: Option<String>,
}

impl Eq for DownloadRecord {}

#[derive(
  Serialize,
  Deserialize,
  Default,
  PartialEq,
  Eq,
  Clone,
  Copy,
  Debug,
  EnumIter,
  strum::Display,
  EnumDisplayStyle,
)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
pub enum DownloadStatus {
  #[default]
  Unknown,
  Queued,
  Paused,
  Downloading,
  Completed,
  Failed,
  Warning,
  Delay,
  #[display_style(name = "Download Client Unavailable")]
  DownloadClientUnavailable,
  Fallback,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DownloadsResponse {
  pub records: Vec<DownloadRecord>,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SystemStatus {
  pub version: String,
  pub start_time: DateTime<Utc>,
}

impl From<LidarrSerdeable> for Serdeable {
  fn from(value: LidarrSerdeable) -> Serdeable {
    Serdeable::Lidarr(value)
  }
}

serde_enum_from!(
  LidarrSerdeable {
    Artists(Vec<Artist>),
    DiskSpaces(Vec<super::servarr_models::DiskSpace>),
    DownloadsResponse(DownloadsResponse),
    MetadataProfiles(Vec<MetadataProfile>),
    QualityProfiles(Vec<super::servarr_models::QualityProfile>),
    RootFolders(Vec<super::servarr_models::RootFolder>),
    SystemStatus(SystemStatus),
    Tags(Vec<super::servarr_models::Tag>),
    Value(Value),
  }
);
