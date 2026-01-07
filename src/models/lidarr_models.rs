use chrono::{DateTime, Utc};
use derivative::Derivative;
use enum_display_style_derive::EnumDisplayStyle;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use strum::{Display, EnumIter};

use super::{
  HorizontallyScrollableText, Serdeable,
  servarr_models::{DiskSpace, HostConfig, QualityProfile, RootFolder, SecurityConfig, Tag},
};
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
  pub members: Option<Vec<Member>>,
  pub path: String,
  #[serde(deserialize_with = "super::from_i64")]
  pub quality_profile_id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub metadata_profile_id: i64,
  pub monitored: bool,
  pub monitor_new_items: NewItemMonitorType,
  pub genres: Vec<String>,
  pub tags: Vec<Number>,
  pub added: DateTime<Utc>,
  pub ratings: Option<Ratings>,
  pub statistics: Option<ArtistStatistics>,
}

#[derive(
  Serialize, Deserialize, Default, PartialEq, Eq, Clone, Copy, Debug, Display, EnumDisplayStyle,
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
pub struct Member {
  pub name: Option<String>,
  pub instrument: Option<String>,
}

impl Eq for Member {}

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
  clap::ValueEnum,
  Display,
  EnumDisplayStyle,
)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
pub enum NewItemMonitorType {
  #[default]
  #[display_style(name = "All Albums")]
  All,
  #[display_style(name = "No New Albums")]
  None,
  #[display_style(name = "New Albums")]
  New,
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
  Display,
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

#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct DeleteArtistParams {
  pub id: i64,
  pub delete_files: bool,
  pub add_import_list_exclusion: bool,
}

#[derive(Default, Clone, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EditArtistParams {
  pub artist_id: i64,
  pub monitored: Option<bool>,
  pub monitor_new_items: Option<NewItemMonitorType>,
  pub quality_profile_id: Option<i64>,
  pub metadata_profile_id: Option<i64>,
  pub root_folder_path: Option<String>,
  pub tags: Option<Vec<i64>>,
  #[serde(skip_serializing, skip_deserializing)]
  pub tag_input_string: Option<String>,
  pub clear_tags: bool,
}

impl From<LidarrSerdeable> for Serdeable {
  fn from(value: LidarrSerdeable) -> Serdeable {
    Serdeable::Lidarr(value)
  }
}

serde_enum_from!(
  LidarrSerdeable {
    Artist(Artist),
    Artists(Vec<Artist>),
    DiskSpaces(Vec<DiskSpace>),
    DownloadsResponse(DownloadsResponse),
    HostConfig(HostConfig),
    MetadataProfiles(Vec<MetadataProfile>),
    QualityProfiles(Vec<QualityProfile>),
    RootFolders(Vec<RootFolder>),
    SecurityConfig(SecurityConfig),
    SystemStatus(SystemStatus),
    Tag(Tag),
    Tags(Vec<Tag>),
    Value(Value),
  }
);
