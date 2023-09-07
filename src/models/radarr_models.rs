use std::fmt::{Display, Formatter};

use chrono::{DateTime, Utc};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use strum_macros::{Display, EnumIter};

use crate::models::HorizontallyScrollableText;

#[cfg(test)]
#[path = "radarr_models_tests.rs"]
mod radarr_models_tests;

#[derive(Default, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddMovieBody {
  pub tmdb_id: i64,
  pub title: String,
  pub root_folder_path: String,
  pub quality_profile_id: i64,
  pub minimum_availability: String,
  pub monitored: bool,
  pub tags: Vec<i64>,
  pub add_options: AddOptions,
}

#[derive(Derivative, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AddMovieSearchResult {
  #[serde(deserialize_with = "super::from_i64")]
  pub tmdb_id: i64,
  pub title: HorizontallyScrollableText,
  pub original_language: Language,
  pub status: String,
  pub overview: String,
  pub genres: Vec<String>,
  #[serde(deserialize_with = "super::from_i64")]
  pub year: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub runtime: i64,
  pub ratings: RatingsList,
}

#[derive(Default, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AddOptions {
  pub monitor: String,
  pub search_for_movie: bool,
}

#[derive(Default, Serialize, Debug)]
pub struct AddRootFolderBody {
  pub path: String,
}

#[derive(Deserialize, Derivative, Default, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  #[serde(default)]
  pub title: HorizontallyScrollableText,
  pub root_folder_path: Option<String>,
  pub search_on_add: bool,
  pub monitored: bool,
  pub overview: Option<String>,
  pub minimum_availability: MinimumAvailability,
  #[serde(deserialize_with = "super::from_i64")]
  pub quality_profile_id: i64,
  pub movies: Option<Vec<CollectionMovie>>,
}

#[derive(Derivative, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CollectionMovie {
  pub title: HorizontallyScrollableText,
  pub overview: String,
  #[serde(deserialize_with = "super::from_i64")]
  pub year: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub runtime: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub tmdb_id: i64,
  pub genres: Vec<String>,
  pub ratings: RatingsList,
}

#[derive(Default, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommandBody {
  pub name: String,
}

#[derive(Deserialize, Default, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Credit {
  pub person_name: String,
  pub character: Option<String>,
  pub department: Option<String>,
  pub job: Option<String>,
  #[serde(rename(deserialize = "type"))]
  pub credit_type: CreditType,
}

#[derive(Deserialize, Default, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CreditType {
  #[default]
  Cast,
  Crew,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DiskSpace {
  #[serde(deserialize_with = "super::from_i64")]
  pub free_space: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub total_space: i64,
}

#[derive(Derivative, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DownloadRecord {
  pub title: String,
  pub status: String,
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub movie_id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub size: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub sizeleft: i64,
  pub output_path: Option<HorizontallyScrollableText>,
  pub indexer: String,
  pub download_client: String,
}

#[derive(Default, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DownloadsResponse {
  pub records: Vec<DownloadRecord>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Indexer {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  pub name: Option<String>,
  pub implementation: Option<String>,
  pub implementation_name: Option<String>,
  pub config_contract: Option<String>,
  pub supports_rss: bool,
  pub supports_search: bool,
  pub fields: Option<Vec<IndexerField>>,
  pub enable_rss: bool,
  pub enable_automatic_search: bool,
  pub enable_interactive_search: bool,
  pub protocol: String,
  #[serde(deserialize_with = "super::from_i64")]
  pub priority: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub download_client_id: i64,
  pub tags: Option<Vec<String>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IndexerField {
  #[serde(deserialize_with = "super::from_i64")]
  pub order: i64,
  pub name: Option<String>,
  pub label: Option<String>,
  pub value: Option<Value>,
  #[serde(rename(deserialize = "type"))]
  pub field_type: Option<String>,
  pub select_options: Option<Vec<IndexerSelectOption>>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IndexerSelectOption {
  #[serde(deserialize_with = "super::from_i64")]
  pub value: i64,
  pub name: Option<String>,
  #[serde(deserialize_with = "super::from_i64")]
  pub order: i64,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IndexerSettings {
  pub allow_hardcoded_subs: bool,
  #[serde(deserialize_with = "super::from_i64")]
  pub availability_delay: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub maximum_size: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub minimum_age: i64,
  pub prefer_indexer_flags: bool,
  #[serde(deserialize_with = "super::from_i64")]
  pub retention: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub rss_sync_interval: i64,
  pub whitelisted_hardcoded_subs: String,
}

#[derive(Deserialize, Default, Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Language {
  pub name: String,
}

#[derive(Default, Deserialize, Clone, Debug, Eq, PartialEq)]
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

#[derive(Default, Deserialize, Debug, Eq, PartialEq)]
pub struct LogResponse {
  pub records: Vec<Log>,
}

#[derive(Deserialize, Derivative, Debug, Clone, PartialEq, Eq)]
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
}

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Clone, Copy, Debug, EnumIter)]
#[serde(rename_all = "camelCase")]
pub enum MinimumAvailability {
  #[default]
  Announced,
  InCinemas,
  Released,
  Tba,
}

impl Display for MinimumAvailability {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let minimum_availability = match self {
      MinimumAvailability::Tba => "tba",
      MinimumAvailability::Announced => "announced",
      MinimumAvailability::InCinemas => "inCinemas",
      MinimumAvailability::Released => "released",
    };
    write!(f, "{minimum_availability}")
  }
}

impl MinimumAvailability {
  pub fn to_display_str<'a>(self) -> &'a str {
    match self {
      MinimumAvailability::Tba => "TBA",
      MinimumAvailability::Announced => "Announced",
      MinimumAvailability::InCinemas => "In Cinemas",
      MinimumAvailability::Released => "Released",
    }
  }
}

#[derive(Default, PartialEq, Eq, Clone, Copy, Debug, EnumIter)]
pub enum Monitor {
  #[default]
  MovieOnly,
  MovieAndCollection,
  None,
}

impl Display for Monitor {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let monitor = match self {
      Monitor::MovieOnly => "movieOnly",
      Monitor::MovieAndCollection => "movieAndCollection",
      Monitor::None => "none",
    };
    write!(f, "{monitor}")
  }
}

impl Monitor {
  pub fn to_display_str<'a>(self) -> &'a str {
    match self {
      Monitor::MovieOnly => "Movie only",
      Monitor::MovieAndCollection => "Movie and Collection",
      Monitor::None => "None",
    }
  }
}

#[derive(Derivative, Deserialize, Debug, Clone, PartialEq, Eq)]
#[derivative(Default)]
#[serde(rename_all = "camelCase")]
pub struct Movie {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  pub title: HorizontallyScrollableText,
  pub original_language: Language,
  #[serde(deserialize_with = "super::from_i64")]
  pub size_on_disk: i64,
  pub status: String,
  pub overview: String,
  pub path: String,
  pub studio: String,
  pub genres: Vec<String>,
  #[serde(deserialize_with = "super::from_i64")]
  pub year: i64,
  pub monitored: bool,
  pub has_file: bool,
  #[serde(deserialize_with = "super::from_i64")]
  pub runtime: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub tmdb_id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub quality_profile_id: i64,
  pub minimum_availability: MinimumAvailability,
  pub certification: Option<String>,
  pub tags: Vec<Number>,
  pub ratings: RatingsList,
  pub movie_file: Option<MovieFile>,
  pub collection: Option<Collection>,
}

#[derive(Default, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MovieCommandBody {
  pub name: String,
  pub movie_ids: Vec<i64>,
}

#[derive(Deserialize, Derivative, Debug, Clone, PartialEq, Eq)]
#[derivative(Default)]
#[serde(rename_all = "camelCase")]
pub struct MovieFile {
  pub relative_path: String,
  pub path: String,
  pub date_added: DateTime<Utc>,
  pub media_info: Option<MediaInfo>,
}

#[derive(Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MovieHistoryItem {
  pub source_title: HorizontallyScrollableText,
  pub quality: QualityWrapper,
  pub languages: Vec<Language>,
  pub date: DateTime<Utc>,
  pub event_type: String,
}

#[derive(Deserialize, Default, Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Quality {
  pub name: String,
}

#[derive(Default, Deserialize, Debug)]
pub struct QualityProfile {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  pub name: String,
}

#[derive(Deserialize, Default, Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct QualityWrapper {
  pub quality: Quality,
}

#[derive(Default, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct QueueEvent {
  pub trigger: String,
  pub name: String,
  pub command_name: String,
  pub status: String,
  pub queued: DateTime<Utc>,
  pub started: Option<DateTime<Utc>>,
  pub ended: Option<DateTime<Utc>>,
  pub duration: Option<String>,
}

#[derive(Derivative, Deserialize, Debug, Clone, PartialEq, Eq)]
#[derivative(Default)]
pub struct Rating {
  #[derivative(Default(value = "Number::from(0)"))]
  pub value: Number,
}

#[derive(Default, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RatingsList {
  pub imdb: Option<Rating>,
  pub tmdb: Option<Rating>,
  pub rotten_tomatoes: Option<Rating>,
}

#[derive(Deserialize, Default, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Release {
  pub guid: String,
  pub protocol: String,
  #[serde(deserialize_with = "super::from_i64")]
  pub age: i64,
  pub title: HorizontallyScrollableText,
  pub indexer: String,
  #[serde(deserialize_with = "super::from_i64")]
  pub indexer_id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub size: i64,
  pub rejected: bool,
  pub rejections: Option<Vec<String>>,
  pub seeders: Option<Number>,
  pub leechers: Option<Number>,
  pub languages: Option<Vec<Language>>,
  pub quality: QualityWrapper,
}

#[derive(Default, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseDownloadBody {
  pub guid: String,
  pub indexer_id: i64,
  pub movie_id: i64,
}

#[derive(Default, PartialEq, Eq, Clone, Copy, Debug, EnumIter, Display)]
pub enum ReleaseField {
  #[default]
  Source,
  Age,
  Rejected,
  Title,
  Indexer,
  Size,
  Peers,
  Language,
  Quality,
}

#[derive(Default, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RootFolder {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  pub path: String,
  pub accessible: bool,
  #[serde(deserialize_with = "super::from_i64")]
  pub free_space: i64,
  pub unmapped_folders: Option<Vec<UnmappedFolder>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SystemStatus {
  pub version: String,
  pub start_time: DateTime<Utc>,
}

#[derive(Default, Deserialize, Debug)]
pub struct Tag {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  pub label: String,
}

#[derive(Default, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Task {
  pub name: String,
  pub task_name: String,
  #[serde(deserialize_with = "super::from_i64")]
  pub interval: i64,
  pub last_execution: DateTime<Utc>,
  pub last_duration: String,
  pub next_execution: DateTime<Utc>,
}

#[derive(Deserialize, Default, Debug, Clone, Eq, PartialEq)]
pub struct UnmappedFolder {
  pub name: String,
  pub path: String,
}

#[derive(Default, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Update {
  pub version: String,
  pub release_date: DateTime<Utc>,
  pub installed: bool,
  pub latest: bool,
  pub installed_on: Option<DateTime<Utc>>,
  pub changes: UpdateChanges,
}

#[derive(Default, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateChanges {
  pub new: Option<Vec<String>>,
  pub fixed: Option<Vec<String>>,
}
