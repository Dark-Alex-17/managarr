use std::fmt::{Display, Formatter};

use chrono::{DateTime, Utc};
use clap::ValueEnum;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_json::{json, Number, Value};
use strum_macros::EnumIter;

use crate::{models::HorizontallyScrollableText, serde_enum_from};

use super::servarr_models::{
  HostConfig, Indexer, Language, LogResponse, QualityProfile, QualityWrapper, QueueEvent, Release,
  RootFolder, SecurityConfig,
};
use super::{EnumDisplayStyle, Serdeable};

#[cfg(test)]
#[path = "radarr_models_tests.rs"]
mod radarr_models_tests;

#[derive(Default, Clone, Serialize, Debug, PartialEq, Eq)]
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

#[derive(Derivative, Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
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

#[derive(Default, Clone, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AddOptions {
  pub monitor: String,
  pub search_for_movie: bool,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct BlocklistResponse {
  pub records: Vec<BlocklistItem>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BlocklistItem {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub movie_id: i64,
  pub source_title: String,
  pub languages: Vec<Language>,
  pub quality: QualityWrapper,
  pub custom_formats: Option<Vec<Language>>,
  pub date: DateTime<Utc>,
  pub protocol: String,
  pub indexer: String,
  pub message: String,
  pub movie: BlocklistItemMovie,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct BlocklistItemMovie {
  pub title: HorizontallyScrollableText,
}

#[derive(Serialize, Deserialize, Derivative, Default, Clone, Debug, PartialEq, Eq)]
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

#[derive(Derivative, Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Credit {
  pub person_name: String,
  pub character: Option<String>,
  pub department: Option<String>,
  pub job: Option<String>,
  #[serde(rename(deserialize = "type"))]
  pub credit_type: CreditType,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CreditType {
  #[default]
  Cast,
  Crew,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct DeleteMovieParams {
  pub id: i64,
  pub delete_movie_files: bool,
  pub add_list_exclusion: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DiskSpace {
  #[serde(deserialize_with = "super::from_i64")]
  pub free_space: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub total_space: i64,
}

#[derive(Derivative, Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
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
  #[serde(default)]
  pub indexer: String,
  pub download_client: String,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DownloadsResponse {
  pub records: Vec<DownloadRecord>,
}

#[derive(Default, Clone, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EditCollectionParams {
  pub collection_id: i64,
  pub monitored: Option<bool>,
  pub minimum_availability: Option<MinimumAvailability>,
  pub quality_profile_id: Option<i64>,
  pub root_folder_path: Option<String>,
  pub search_on_add: Option<bool>,
}

#[derive(Default, Clone, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EditIndexerParams {
  pub indexer_id: i64,
  pub name: Option<String>,
  pub enable_rss: Option<bool>,
  pub enable_automatic_search: Option<bool>,
  pub enable_interactive_search: Option<bool>,
  pub url: Option<String>,
  pub api_key: Option<String>,
  pub seed_ratio: Option<String>,
  pub tags: Option<Vec<i64>>,
  pub priority: Option<i64>,
  pub clear_tags: bool,
}

#[derive(Default, Clone, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EditMovieParams {
  pub movie_id: i64,
  pub monitored: Option<bool>,
  pub minimum_availability: Option<MinimumAvailability>,
  pub quality_profile_id: Option<i64>,
  pub root_folder_path: Option<String>,
  pub tags: Option<Vec<i64>>,
  pub clear_tags: bool,
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
  #[serde(default)]
  pub whitelisted_hardcoded_subs: HorizontallyScrollableText,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct IndexerTestResult {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  pub is_valid: bool,
  pub validation_failures: Vec<IndexerValidationFailure>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct IndexerValidationFailure {
  pub property_name: String,
  pub error_message: String,
  pub severity: String,
}

#[derive(Serialize, Deserialize, Derivative, Debug, Clone, PartialEq, Eq)]
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

#[derive(
  Serialize, Deserialize, Default, PartialEq, Eq, Clone, Copy, Debug, EnumIter, ValueEnum,
)]
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

impl<'a> EnumDisplayStyle<'a> for MinimumAvailability {
  fn to_display_str(self) -> &'a str {
    match self {
      MinimumAvailability::Tba => "TBA",
      MinimumAvailability::Announced => "Announced",
      MinimumAvailability::InCinemas => "In Cinemas",
      MinimumAvailability::Released => "Released",
    }
  }
}

#[derive(Default, PartialEq, Eq, Clone, Copy, Debug, EnumIter, ValueEnum)]
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

impl<'a> EnumDisplayStyle<'a> for Monitor {
  fn to_display_str(self) -> &'a str {
    match self {
      Monitor::MovieOnly => "Movie only",
      Monitor::MovieAndCollection => "Movie and Collection",
      Monitor::None => "None",
    }
  }
}

#[derive(Derivative, Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
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
  pub collection: Option<MovieCollection>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MovieCollection {
  pub title: Option<String>,
}

#[derive(Default, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MovieCommandBody {
  pub name: String,
  pub movie_ids: Vec<i64>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MovieFile {
  pub relative_path: String,
  pub path: String,
  pub date_added: DateTime<Utc>,
  pub media_info: Option<MediaInfo>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MovieHistoryItem {
  pub source_title: HorizontallyScrollableText,
  pub quality: QualityWrapper,
  pub languages: Vec<Language>,
  pub date: DateTime<Utc>,
  pub event_type: String,
}

#[derive(Derivative, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[derivative(Default)]
pub struct Rating {
  #[derivative(Default(value = "Number::from(0)"))]
  pub value: Number,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RatingsList {
  pub imdb: Option<Rating>,
  pub tmdb: Option<Rating>,
  pub rotten_tomatoes: Option<Rating>,
}

#[derive(Default, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseDownloadBody {
  pub guid: String,
  pub indexer_id: i64,
  pub movie_id: i64,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SystemStatus {
  pub version: String,
  pub start_time: DateTime<Utc>,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Tag {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  pub label: String,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Task {
  pub name: String,
  pub task_name: TaskName,
  #[serde(deserialize_with = "super::from_i64")]
  pub interval: i64,
  pub last_execution: DateTime<Utc>,
  pub last_duration: String,
  pub next_execution: DateTime<Utc>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Copy, ValueEnum)]
#[serde(rename_all = "PascalCase")]
pub enum TaskName {
  #[default]
  ApplicationCheckUpdate,
  Backup,
  CheckHealth,
  CleanUpRecycleBin,
  Housekeeping,
  ImportListSync,
  MessagingCleanup,
  RefreshCollections,
  RefreshMonitoredDownloads,
  RefreshMovie,
  RssSync,
}

impl Display for TaskName {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let task_name = serde_json::to_string(&self)
      .expect("Unable to serialize task name")
      .replace('"', "");
    write!(f, "{task_name}")
  }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Update {
  pub version: String,
  pub release_date: DateTime<Utc>,
  pub installed: bool,
  pub latest: bool,
  pub installed_on: Option<DateTime<Utc>>,
  pub changes: UpdateChanges,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UpdateChanges {
  pub new: Option<Vec<String>>,
  pub fixed: Option<Vec<String>>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum RadarrSerdeable {
  Value(Value),
  Tag(Tag),
  BlocklistResponse(BlocklistResponse),
  Collections(Vec<Collection>),
  Credits(Vec<Credit>),
  DiskSpaces(Vec<DiskSpace>),
  DownloadsResponse(DownloadsResponse),
  HostConfig(HostConfig),
  Indexers(Vec<Indexer>),
  IndexerSettings(IndexerSettings),
  LogResponse(LogResponse),
  Movie(Movie),
  MovieHistoryItems(Vec<MovieHistoryItem>),
  Movies(Vec<Movie>),
  QualityProfiles(Vec<QualityProfile>),
  QueueEvents(Vec<QueueEvent>),
  Releases(Vec<Release>),
  RootFolders(Vec<RootFolder>),
  SecurityConfig(SecurityConfig),
  SystemStatus(SystemStatus),
  Tags(Vec<Tag>),
  Tasks(Vec<Task>),
  Updates(Vec<Update>),
  AddMovieSearchResults(Vec<AddMovieSearchResult>),
  IndexerTestResults(Vec<IndexerTestResult>),
}

impl From<RadarrSerdeable> for Serdeable {
  fn from(value: RadarrSerdeable) -> Serdeable {
    Serdeable::Radarr(value)
  }
}

impl From<()> for RadarrSerdeable {
  fn from(_: ()) -> Self {
    RadarrSerdeable::Value(json!({}))
  }
}

serde_enum_from!(
  RadarrSerdeable {
    Value(Value),
    Tag(Tag),
    BlocklistResponse(BlocklistResponse),
    Collections(Vec<Collection>),
    Credits(Vec<Credit>),
    DiskSpaces(Vec<DiskSpace>),
    DownloadsResponse(DownloadsResponse),
    HostConfig(HostConfig),
    Indexers(Vec<Indexer>),
    IndexerSettings(IndexerSettings),
    LogResponse(LogResponse),
    Movie(Movie),
    MovieHistoryItems(Vec<MovieHistoryItem>),
    Movies(Vec<Movie>),
    QualityProfiles(Vec<QualityProfile>),
    QueueEvents(Vec<QueueEvent>),
    Releases(Vec<Release>),
    RootFolders(Vec<RootFolder>),
    SecurityConfig(SecurityConfig),
    SystemStatus(SystemStatus),
    Tags(Vec<Tag>),
    Tasks(Vec<Task>),
    Updates(Vec<Update>),
    AddMovieSearchResults(Vec<AddMovieSearchResult>),
    IndexerTestResults(Vec<IndexerTestResult>),
  }
);
