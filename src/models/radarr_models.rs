use std::fmt::{Display, Formatter};

use crate::models::servarr_models::IndexerTestResult;
use crate::{models::HorizontallyScrollableText, serde_enum_from};
use chrono::{DateTime, Utc};
use clap::ValueEnum;
use derivative::Derivative;
use enum_display_style_derive::EnumDisplayStyle;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use strum_macros::{Display, EnumIter};

use super::Serdeable;
use super::servarr_models::{
  DiskSpace, HostConfig, Indexer, Language, LogResponse, QualityProfile, QualityWrapper,
  QueueEvent, RootFolder, SecurityConfig, Tag, Update,
};

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
  pub minimum_availability: MinimumAvailability,
  pub monitored: bool,
  pub tags: Vec<i64>,
  #[serde(skip_serializing, skip_deserializing)]
  pub tag_input_string: Option<String>,
  pub add_options: AddMovieOptions,
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
pub struct AddMovieOptions {
  pub monitor: MovieMonitor,
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

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub struct DeleteMovieParams {
  pub id: i64,
  pub delete_movie_files: bool,
  pub add_list_exclusion: bool,
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
pub struct EditMovieParams {
  pub movie_id: i64,
  pub monitored: Option<bool>,
  pub minimum_availability: Option<MinimumAvailability>,
  pub quality_profile_id: Option<i64>,
  pub root_folder_path: Option<String>,
  pub tags: Option<Vec<i64>>,
  #[serde(skip_serializing, skip_deserializing)]
  pub tag_input_string: Option<String>,
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
  pub video_codec: Option<String>,
  #[derivative(Default(value = "Number::from(0)"))]
  pub video_fps: Number,
  pub resolution: String,
  pub run_time: String,
  pub scan_type: String,
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
  ValueEnum,
  Display,
  EnumDisplayStyle,
)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
pub enum MinimumAvailability {
  #[default]
  Announced,
  #[display_style(name = "In Cinemas")]
  InCinemas,
  Released,
  #[display_style(name = "TBA")]
  Tba,
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
  ValueEnum,
  Display,
  EnumDisplayStyle,
)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
pub enum MovieMonitor {
  #[default]
  #[display_style(name = "Movie only")]
  MovieOnly,
  #[display_style(name = "Movie and Collection")]
  MovieAndCollection,
  None,
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
  pub studio: Option<String>,
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

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct RadarrRelease {
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

#[derive(Default, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RadarrReleaseDownloadBody {
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

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RadarrHistoryWrapper {
  pub records: Vec<RadarrHistoryItem>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RadarrHistoryData {
  pub indexer: Option<String>,
  pub release_group: Option<String>,
  pub nzb_info_url: Option<String>,
  pub download_client: Option<String>,
  pub download_client_name: Option<String>,
  pub age: Option<String>,
  pub published_date: Option<DateTime<Utc>>,
  pub message: Option<String>,
  pub reason: Option<String>,
  pub dropped_path: Option<String>,
  pub imported_path: Option<String>,
  pub source_path: Option<String>,
  pub path: Option<String>,
}

#[derive(
  Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Display, EnumDisplayStyle,
)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
pub enum RadarrHistoryEventType {
  #[default]
  Unknown,
  Grabbed,
  #[display_style(name = "Download Folder Imported")]
  DownloadFolderImported,
  #[display_style(name = "Download Failed")]
  DownloadFailed,
  #[display_style(name = "Movie File Deleted")]
  MovieFileDeleted,
  #[display_style(name = "Movie Folder Imported")]
  MovieFolderImported,
  #[display_style(name = "Movie File Renamed")]
  MovieFileRenamed,
  #[display_style(name = "Download Ignored")]
  DownloadIgnored,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RadarrHistoryItem {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  pub source_title: HorizontallyScrollableText,
  #[serde(deserialize_with = "super::from_i64")]
  pub movie_id: i64,
  pub quality: QualityWrapper,
  pub languages: Vec<Language>,
  pub date: DateTime<Utc>,
  pub event_type: RadarrHistoryEventType,
  #[serde(default)]
  pub data: RadarrHistoryData,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RadarrTask {
  pub name: String,
  pub task_name: RadarrTaskName,
  #[serde(deserialize_with = "super::from_i64")]
  pub interval: i64,
  pub last_execution: DateTime<Utc>,
  pub last_duration: String,
  pub next_execution: DateTime<Utc>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Copy, ValueEnum)]
#[serde(rename_all = "PascalCase")]
pub enum RadarrTaskName {
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

impl Display for RadarrTaskName {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let task_name = serde_json::to_string(&self)
      .expect("Unable to serialize task name")
      .replace('"', "");
    write!(f, "{task_name}")
  }
}

impl From<RadarrSerdeable> for Serdeable {
  fn from(value: RadarrSerdeable) -> Serdeable {
    Serdeable::Radarr(value)
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
    HistoryWrapper(RadarrHistoryWrapper),
    HostConfig(HostConfig),
    Indexers(Vec<Indexer>),
    IndexerSettings(IndexerSettings),
    LogResponse(LogResponse),
    Movie(Movie),
    MovieHistoryItems(Vec<MovieHistoryItem>),
    Movies(Vec<Movie>),
    QualityProfiles(Vec<QualityProfile>),
    QueueEvents(Vec<QueueEvent>),
    Releases(Vec<RadarrRelease>),
    RootFolders(Vec<RootFolder>),
    SecurityConfig(SecurityConfig),
    SystemStatus(SystemStatus),
    Tags(Vec<Tag>),
    Tasks(Vec<RadarrTask>),
    Updates(Vec<Update>),
    AddMovieSearchResults(Vec<AddMovieSearchResult>),
    IndexerTestResults(Vec<IndexerTestResult>),
  }
);
