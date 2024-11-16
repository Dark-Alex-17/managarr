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

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BlocklistItem {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub series_id: i64,
  pub episode_ids: Vec<Number>,
  pub source_title: String,
  pub language: Language,
  pub quality: QualityWrapper,
  pub date: DateTime<Utc>,
  pub protocol: String,
  pub indexer: String,
  pub message: String,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct BlocklistResponse {
  pub records: Vec<BlocklistItem>,
}

#[derive(Derivative, Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DownloadRecord {
  pub title: String,
  pub status: String,
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub episode_id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub size: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub sizeleft: i64,
  pub output_path: Option<HorizontallyScrollableText>,
  #[serde(default)]
  pub indexer: String,
  pub download_client: String,
}

#[derive(Default, Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Episode {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub series_id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub tvdb_id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub episode_file_id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub season_number: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub episode_number: i64,
  pub title: Option<String>,
  pub air_date_utc: Option<DateTime<Utc>>,
  pub overview: Option<String>,
  pub has_file: bool,
  pub monitored: bool,
  pub episode_file: Option<EpisodeFile>,
}

impl Display for Episode {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.title.as_ref().unwrap_or(&String::new()))
  }
}

#[derive(Default, Serialize, Deserialize, Hash, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeFile {
  pub relative_path: String,
  pub path: String,
  #[serde(deserialize_with = "super::from_i64")]
  pub size: i64,
  pub language: Language,
  pub date_added: DateTime<Utc>,
  pub media_info: Option<MediaInfo>,
}

#[derive(Serialize, Deserialize, Default, Debug, Hash, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Language {
  pub name: String,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Log {
  pub time: DateTime<Utc>,
  pub exception: Option<String>,
  pub exception_type: Option<String>,
  pub level: String,
  pub logger: Option<String>,
  pub message: Option<String>,
  pub method: Option<String>,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct LogResponse {
  pub records: Vec<Log>,
}

#[derive(Serialize, Deserialize, Derivative, Hash, Debug, Clone, PartialEq, Eq)]
#[derivative(Default)]
#[serde(rename_all = "camelCase")]
pub struct MediaInfo {
  #[serde(deserialize_with = "super::from_i64")]
  pub audio_bitrate: i64,
  #[derivative(Default(value = "Number::from(0)"))]
  pub audio_channels: Number,
  pub audio_codec: Option<String>,
  pub audio_languages: Option<String>,
  #[serde(deserialize_with = "super::from_i64")]
  pub audio_stream_count: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub video_bit_depth: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub video_bitrate: i64,
  pub video_codec: String,
  #[derivative(Default(value = "Number::from(0)"))]
  pub video_fps: Number,
  pub resolution: String,
  pub run_time: String,
  pub scan_type: String,
  pub subtitles: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Hash, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Quality {
  pub name: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Hash, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct QualityWrapper {
  pub quality: Quality,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct QualityProfile {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  pub name: String,
}

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
  Episode(Episode),
  Episodes(Vec<Episode>),
  QualityProfiles(Vec<QualityProfile>),
  SeriesVec(Vec<Series>),
  SystemStatus(SystemStatus),
  BlocklistResponse(BlocklistResponse),
  LogResponse(LogResponse),
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
    Episode(Episode),
    Episodes(Vec<Episode>),
    QualityProfiles(Vec<QualityProfile>),
    SeriesVec(Vec<Series>),
    SystemStatus(SystemStatus),
    BlocklistResponse(BlocklistResponse),
    LogResponse(LogResponse),
  }
);

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SystemStatus {
  pub version: String,
  pub start_time: DateTime<Utc>,
}
