use std::fmt::{Display, Formatter};

use chrono::{DateTime, Utc};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_json::Number;
use strum_macros::{Display, EnumIter};

use crate::models::HorizontallyScrollableText;

#[cfg(test)]
#[path = "radarr_models_tests.rs"]
mod radarr_models_tests;

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DiskSpace {
  pub free_space: Number,
  pub total_space: Number,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SystemStatus {
  pub version: String,
  pub start_time: DateTime<Utc>,
}

#[derive(Derivative, Deserialize, Debug, Clone, Eq, PartialEq)]
#[derivative(Default)]
#[serde(rename_all = "camelCase")]
pub struct RootFolder {
  #[derivative(Default(value = "Number::from(0)"))]
  pub id: Number,
  pub path: String,
  pub accessible: bool,
  #[derivative(Default(value = "Number::from(0)"))]
  pub free_space: Number,
  pub unmapped_folders: Option<Vec<UnmappedFolder>>,
}

#[derive(Deserialize, Default, Debug, Clone, Eq, PartialEq)]
pub struct UnmappedFolder {
  pub name: String,
  pub path: String,
}

#[derive(Derivative, Deserialize, Debug, Clone, PartialEq, Eq)]
#[derivative(Default)]
#[serde(rename_all = "camelCase")]
pub struct Movie {
  #[derivative(Default(value = "Number::from(0)"))]
  pub id: Number,
  pub title: HorizontallyScrollableText,
  pub original_language: Language,
  #[derivative(Default(value = "Number::from(0)"))]
  pub size_on_disk: Number,
  pub status: String,
  pub overview: String,
  pub path: String,
  pub studio: String,
  pub genres: Vec<String>,
  #[derivative(Default(value = "Number::from(0)"))]
  pub year: Number,
  pub monitored: bool,
  pub has_file: bool,
  #[derivative(Default(value = "Number::from(0)"))]
  pub runtime: Number,
  #[derivative(Default(value = "Number::from(0)"))]
  pub tmdb_id: Number,
  #[derivative(Default(value = "Number::from(0)"))]
  pub quality_profile_id: Number,
  pub minimum_availability: MinimumAvailability,
  pub certification: Option<String>,
  pub tags: Vec<Number>,
  pub ratings: RatingsList,
  pub movie_file: Option<MovieFile>,
  pub collection: Option<Collection>,
}

#[derive(Derivative, Deserialize, Debug, Clone, PartialEq, Eq)]
#[derivative(Default)]
#[serde(rename_all = "camelCase")]
pub struct CollectionMovie {
  pub title: HorizontallyScrollableText,
  pub overview: String,
  #[derivative(Default(value = "Number::from(0)"))]
  pub year: Number,
  #[derivative(Default(value = "Number::from(0)"))]
  pub runtime: Number,
  #[derivative(Default(value = "Number::from(0)"))]
  pub tmdb_id: Number,
  pub genres: Vec<String>,
  pub ratings: RatingsList,
}

#[derive(Deserialize, Derivative, Clone, Debug, PartialEq, Eq)]
#[derivative(Default)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
  #[derivative(Default(value = "Number::from(0)"))]
  pub id: Number,
  #[serde(default)]
  pub title: HorizontallyScrollableText,
  pub root_folder_path: Option<String>,
  pub search_on_add: bool,
  pub monitored: bool,
  pub overview: Option<String>,
  pub minimum_availability: MinimumAvailability,
  #[derivative(Default(value = "Number::from(0)"))]
  pub quality_profile_id: Number,
  pub movies: Option<Vec<CollectionMovie>>,
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

#[derive(Deserialize, Derivative, Debug, Clone, PartialEq, Eq)]
#[derivative(Default)]
#[serde(rename_all = "camelCase")]
pub struct MediaInfo {
  #[derivative(Default(value = "Number::from(0)"))]
  pub audio_bitrate: Number,
  #[derivative(Default(value = "Number::from(0)"))]
  pub audio_channels: Number,
  pub audio_codec: Option<String>,
  pub audio_languages: Option<String>,
  #[derivative(Default(value = "Number::from(0)"))]
  pub audio_stream_count: Number,
  #[derivative(Default(value = "Number::from(0)"))]
  pub video_bit_depth: Number,
  #[derivative(Default(value = "Number::from(0)"))]
  pub video_bitrate: Number,
  pub video_codec: String,
  #[derivative(Default(value = "Number::from(0)"))]
  pub video_fps: Number,
  pub resolution: String,
  pub run_time: String,
  pub scan_type: String,
}

#[derive(Default, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RatingsList {
  pub imdb: Option<Rating>,
  pub tmdb: Option<Rating>,
  pub rotten_tomatoes: Option<Rating>,
}

#[derive(Derivative, Deserialize, Debug, Clone, PartialEq, Eq)]
#[derivative(Default)]
pub struct Rating {
  #[derivative(Default(value = "Number::from(0)"))]
  pub value: Number,
}

#[derive(Derivative, Deserialize, Debug)]
#[derivative(Default)]
#[serde(rename_all = "camelCase")]
pub struct DownloadsResponse {
  pub records: Vec<DownloadRecord>,
}

#[derive(Derivative, Deserialize, Debug, Clone, PartialEq, Eq)]
#[derivative(Default)]
#[serde(rename_all = "camelCase")]
pub struct DownloadRecord {
  pub title: String,
  pub status: String,
  #[derivative(Default(value = "Number::from(0)"))]
  pub id: Number,
  #[derivative(Default(value = "Number::from(0)"))]
  pub movie_id: Number,
  #[derivative(Default(value = "Number::from(0)"))]
  pub size: Number,
  #[derivative(Default(value = "Number::from(0)"))]
  pub sizeleft: Number,
  pub output_path: Option<HorizontallyScrollableText>,
  pub indexer: String,
  pub download_client: String,
}

#[derive(Derivative, Deserialize, Debug)]
#[derivative(Default)]
pub struct QualityProfile {
  #[derivative(Default(value = "Number::from(0)"))]
  pub id: Number,
  pub name: String,
}

#[derive(Derivative, Deserialize, Debug)]
#[derivative(Default)]
pub struct Tag {
  #[derivative(Default(value = "Number::from(0)"))]
  pub id: Number,
  pub label: String,
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
pub struct Language {
  pub name: String,
}

#[derive(Deserialize, Default, Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Quality {
  pub name: String,
}

#[derive(Deserialize, Default, Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct QualityWrapper {
  pub quality: Quality,
}

#[derive(Deserialize, Default, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CreditType {
  #[default]
  Cast,
  Crew,
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

#[derive(Deserialize, Derivative, Clone, Debug, PartialEq, Eq)]
#[derivative(Default)]
#[serde(rename_all = "camelCase")]
pub struct Release {
  pub guid: String,
  pub protocol: String,
  #[derivative(Default(value = "Number::from(0)"))]
  pub age: Number,
  pub title: HorizontallyScrollableText,
  pub indexer: String,
  #[derivative(Default(value = "Number::from(0)"))]
  pub indexer_id: Number,
  #[derivative(Default(value = "Number::from(0)"))]
  pub size: Number,
  pub rejected: bool,
  pub rejections: Option<Vec<String>>,
  pub seeders: Option<Number>,
  pub leechers: Option<Number>,
  pub languages: Option<Vec<Language>>,
  pub quality: QualityWrapper,
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

#[derive(Default, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddMovieBody {
  pub tmdb_id: u64,
  pub title: String,
  pub root_folder_path: String,
  pub quality_profile_id: u64,
  pub minimum_availability: String,
  pub monitored: bool,
  pub tags: Vec<u64>,
  pub add_options: AddOptions,
}

#[derive(Default, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AddOptions {
  pub monitor: String,
  pub search_for_movie: bool,
}

#[derive(Default, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseDownloadBody {
  pub guid: String,
  pub indexer_id: u64,
}

#[derive(Derivative, Deserialize, Debug, Clone, PartialEq, Eq)]
#[derivative(Default)]
#[serde(rename_all = "camelCase")]
pub struct AddMovieSearchResult {
  #[derivative(Default(value = "Number::from(0)"))]
  pub tmdb_id: Number,
  pub title: HorizontallyScrollableText,
  pub original_language: Language,
  pub status: String,
  pub overview: String,
  pub genres: Vec<String>,
  #[derivative(Default(value = "Number::from(0)"))]
  pub year: Number,
  #[derivative(Default(value = "Number::from(0)"))]
  pub runtime: Number,
  pub ratings: RatingsList,
}

#[derive(Default, Serialize, Debug)]
pub struct AddRootFolderBody {
  pub path: String,
}

#[derive(Default, Derivative, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MovieCommandBody {
  pub name: String,
  pub movie_ids: Vec<u64>,
}

#[derive(Default, Derivative, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommandBody {
  pub name: String,
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
    write!(f, "{}", minimum_availability)
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
    write!(f, "{}", monitor)
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

#[derive(Derivative, Deserialize, Debug, Clone, PartialEq, Eq)]
#[derivative(Default)]
#[serde(rename_all = "camelCase")]
pub struct Task {
  pub name: String,
  pub task_name: String,
  #[derivative(Default(value = "Number::from(0)"))]
  pub interval: Number,
  pub last_execution: DateTime<Utc>,
  pub last_duration: String,
  pub next_execution: DateTime<Utc>,
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
