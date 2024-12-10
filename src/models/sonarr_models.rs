use std::fmt::{Display, Formatter};

use chrono::{DateTime, Utc};
use clap::ValueEnum;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_json::{json, Number, Value};
use strum::EnumIter;

use crate::serde_enum_from;

use super::{
  radarr_models::IndexerTestResult,
  servarr_models::{
    DiskSpace, HostConfig, Indexer, Language, LogResponse, QualityProfile, QualityWrapper,
    QueueEvent, RootFolder, SecurityConfig, Tag, Update,
  },
  EnumDisplayStyle, HorizontallyScrollableText, Serdeable,
};

#[cfg(test)]
#[path = "sonarr_models_tests.rs"]
mod sonarr_models_tests;

#[derive(Default, Clone, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AddSeriesBody {
  pub tvdb_id: i64,
  pub title: String,
  pub monitored: bool,
  pub root_folder_path: String,
  pub quality_profile_id: i64,
  pub language_profile_id: i64,
  pub series_type: String,
  pub season_folder: bool,
  pub tags: Vec<i64>,
  pub add_options: AddSeriesOptions,
}

#[derive(Derivative, Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AddSeriesSearchResult {
  #[serde(deserialize_with = "super::from_i64")]
  pub tvdb_id: i64,
  pub title: HorizontallyScrollableText,
  pub status: Option<String>,
  pub ended: bool,
  pub overview: Option<String>,
  pub genres: Vec<String>,
  #[serde(deserialize_with = "super::from_i64")]
  pub year: i64,
  pub network: Option<String>,
  #[serde(deserialize_with = "super::from_i64")]
  pub runtime: i64,
  pub ratings: Option<Rating>,
  pub statistics: Option<AddSeriesSearchResultStatistics>,
}

#[derive(Derivative, Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AddSeriesSearchResultStatistics {
  #[serde(deserialize_with = "super::from_i64")]
  pub season_count: i64,
}

#[derive(Default, Clone, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AddSeriesOptions {
  pub monitor: String,
  pub search_for_cutoff_unmet_episodes: bool,
  pub search_for_missing_episodes: bool,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BlocklistItem {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub series_id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub series_title: Option<String>,
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

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct DeleteSeriesParams {
  pub id: i64,
  pub delete_series_files: bool,
  pub add_list_exclusion: bool,
}

#[derive(Derivative, Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DownloadRecord {
  pub title: String,
  pub status: String,
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub episode_id: i64,
  #[serde(deserialize_with = "super::from_f64")]
  pub size: f64,
  #[serde(deserialize_with = "super::from_f64")]
  pub sizeleft: f64,
  pub output_path: Option<HorizontallyScrollableText>,
  #[serde(default)]
  pub indexer: String,
  pub download_client: String,
}

impl Eq for DownloadRecord {}

#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DownloadsResponse {
  pub records: Vec<DownloadRecord>,
}

#[derive(Default, Clone, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EditSeriesParams {
  pub series_id: i64,
  pub monitored: Option<bool>,
  pub use_season_folders: Option<bool>,
  pub quality_profile_id: Option<i64>,
  pub language_profile_id: Option<i64>,
  pub series_type: Option<SeriesType>,
  pub root_folder_path: Option<String>,
  pub tags: Option<Vec<i64>>,
  pub clear_tags: bool,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeFile {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  pub relative_path: String,
  pub path: String,
  #[serde(deserialize_with = "super::from_i64")]
  pub size: i64,
  pub language: Language,
  pub quality: QualityWrapper,
  pub date_added: DateTime<Utc>,
  pub media_info: Option<MediaInfo>,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IndexerSettings {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub minimum_age: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub retention: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub maximum_size: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub rss_sync_interval: i64,
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
  pub subtitles: Option<String>,
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
  pub title: Option<String>,
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
  pub overview: Option<String>,
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
pub enum SeriesMonitor {
  #[default]
  All,
  Unknown,
  Future,
  Missing,
  Existing,
  FirstSeason,
  LastSeason,
  LatestSeason,
  Pilot,
  Recent,
  MonitorSpecials,
  UnmonitorSpecials,
  None,
  Skip,
}

impl Display for SeriesMonitor {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let series_monitor = match self {
      SeriesMonitor::Unknown => "unknown",
      SeriesMonitor::All => "all",
      SeriesMonitor::Future => "future",
      SeriesMonitor::Missing => "missing",
      SeriesMonitor::Existing => "existing",
      SeriesMonitor::FirstSeason => "firstSeason",
      SeriesMonitor::LastSeason => "lastSeason",
      SeriesMonitor::LatestSeason => "latestSeason",
      SeriesMonitor::Pilot => "pilot",
      SeriesMonitor::Recent => "recent",
      SeriesMonitor::MonitorSpecials => "monitorSpecials",
      SeriesMonitor::UnmonitorSpecials => "unmonitorSpecials",
      SeriesMonitor::None => "none",
      SeriesMonitor::Skip => "skip",
    };
    write!(f, "{series_monitor}")
  }
}

impl<'a> EnumDisplayStyle<'a> for SeriesMonitor {
  fn to_display_str(self) -> &'a str {
    match self {
      SeriesMonitor::Unknown => "Unknown",
      SeriesMonitor::All => "All Episodes",
      SeriesMonitor::Future => "Future Episodes",
      SeriesMonitor::Missing => "Missing Episodes",
      SeriesMonitor::Existing => "Existing Episodes",
      SeriesMonitor::FirstSeason => "Only First Season",
      SeriesMonitor::LastSeason => "Only Last Season",
      SeriesMonitor::LatestSeason => "Only Latest Season",
      SeriesMonitor::Pilot => "Pilot Episode",
      SeriesMonitor::Recent => "Recent Episodes",
      SeriesMonitor::MonitorSpecials => "Only Specials",
      SeriesMonitor::UnmonitorSpecials => "Not Specials",
      SeriesMonitor::None => "None",
      SeriesMonitor::Skip => "Skip",
    }
  }
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

impl<'a> EnumDisplayStyle<'a> for SeriesType {
  fn to_display_str(self) -> &'a str {
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

impl<'a> EnumDisplayStyle<'a> for SeriesStatus {
  fn to_display_str(self) -> &'a str {
    match self {
      SeriesStatus::Continuing => "Continuing",
      SeriesStatus::Ended => "Ended",
      SeriesStatus::Upcoming => "Upcoming",
      SeriesStatus::Deleted => "Deleted",
    }
  }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SonarrHistoryWrapper {
  pub records: Vec<SonarrHistoryItem>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SonarrHistoryData {
  pub dropped_path: Option<String>,
  pub imported_path: Option<String>,
  pub indexer: Option<String>,
  pub release_group: Option<String>,
  pub series_match_type: Option<String>,
  pub nzb_info_url: Option<String>,
  pub download_client_name: Option<String>,
  pub age: Option<String>,
  pub published_date: Option<DateTime<Utc>>,
  pub message: Option<String>,
  pub reason: Option<String>,
  pub source_path: Option<String>,
  pub source_relative_path: Option<String>,
  pub path: Option<String>,
  pub relative_path: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum SonarrHistoryEventType {
  #[default]
  Unknown,
  Grabbed,
  SeriesFolderImported,
  DownloadFolderImported,
  DownloadFailed,
  EpisodeFileDeleted,
  EpisodeFileRenamed,
  DownloadIgnored,
}

impl Display for SonarrHistoryEventType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let event_type = match self {
      SonarrHistoryEventType::Unknown => "unknown",
      SonarrHistoryEventType::Grabbed => "grabbed",
      SonarrHistoryEventType::SeriesFolderImported => "seriesFolderImported",
      SonarrHistoryEventType::DownloadFolderImported => "downloadFolderImported",
      SonarrHistoryEventType::DownloadFailed => "downloadFailed",
      SonarrHistoryEventType::EpisodeFileDeleted => "episodeFileDeleted",
      SonarrHistoryEventType::EpisodeFileRenamed => "episodeFileRenamed",
      SonarrHistoryEventType::DownloadIgnored => "downloadIgnored",
    };
    write!(f, "{event_type}")
  }
}

impl<'a> EnumDisplayStyle<'a> for SonarrHistoryEventType {
  fn to_display_str(self) -> &'a str {
    match self {
      SonarrHistoryEventType::Unknown => "Unknown",
      SonarrHistoryEventType::Grabbed => "Grabbed",
      SonarrHistoryEventType::SeriesFolderImported => "Series Folder Imported",
      SonarrHistoryEventType::DownloadFolderImported => "Download Folder Imported",
      SonarrHistoryEventType::DownloadFailed => "Download Failed",
      SonarrHistoryEventType::EpisodeFileDeleted => "Episode File Deleted",
      SonarrHistoryEventType::EpisodeFileRenamed => "Episode File Renamed",
      SonarrHistoryEventType::DownloadIgnored => "Download Ignored",
    }
  }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SonarrHistoryItem {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  pub source_title: HorizontallyScrollableText,
  #[serde(deserialize_with = "super::from_i64")]
  pub episode_id: i64,
  pub quality: QualityWrapper,
  pub language: Language,
  pub date: DateTime<Utc>,
  pub event_type: SonarrHistoryEventType,
  pub data: SonarrHistoryData,
}

#[derive(Default, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SonarrCommandBody {
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub series_id: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub season_number: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub episode_ids: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct SonarrRelease {
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
  pub full_season: bool,
}
#[derive(Default, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SonarrReleaseDownloadBody {
  pub guid: String,
  pub indexer_id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub series_id: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub episode_id: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub season_number: Option<i64>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SonarrTask {
  pub name: String,
  pub task_name: SonarrTaskName,
  #[serde(deserialize_with = "super::from_i64")]
  pub interval: i64,
  pub last_execution: DateTime<Utc>,
  pub next_execution: DateTime<Utc>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Copy, ValueEnum)]
#[serde(rename_all = "PascalCase")]
pub enum SonarrTaskName {
  #[default]
  ApplicationUpdateCheck,
  Backup,
  CheckHealth,
  CleanUpRecycleBin,
  Housekeeping,
  ImportListSync,
  MessagingCleanup,
  RefreshMonitoredDownloads,
  RefreshSeries,
  RssSync,
  UpdateSceneMapping,
}

impl Display for SonarrTaskName {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let task_name = serde_json::to_string(&self)
      .expect("Unable to serialize task name")
      .replace('"', "");
    write!(f, "{task_name}")
  }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum SonarrSerdeable {
  AddSeriesSearchResults(Vec<AddSeriesSearchResult>),
  BlocklistResponse(BlocklistResponse),
  DownloadsResponse(DownloadsResponse),
  DiskSpaces(Vec<DiskSpace>),
  Episode(Episode),
  Episodes(Vec<Episode>),
  EpisodeFiles(Vec<EpisodeFile>),
  HostConfig(HostConfig),
  IndexerSettings(IndexerSettings),
  Indexers(Vec<Indexer>),
  IndexerTestResults(Vec<IndexerTestResult>),
  LanguageProfiles(Vec<Language>),
  LogResponse(LogResponse),
  QualityProfiles(Vec<QualityProfile>),
  QueueEvents(Vec<QueueEvent>),
  Releases(Vec<SonarrRelease>),
  RootFolders(Vec<RootFolder>),
  SecurityConfig(SecurityConfig),
  SeriesVec(Vec<Series>),
  Series(Series),
  SonarrHistoryItems(Vec<SonarrHistoryItem>),
  SonarrHistoryWrapper(SonarrHistoryWrapper),
  SystemStatus(SystemStatus),
  Tag(Tag),
  Tags(Vec<Tag>),
  Tasks(Vec<SonarrTask>),
  Updates(Vec<Update>),
  Value(Value),
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
    AddSeriesSearchResults(Vec<AddSeriesSearchResult>),
    BlocklistResponse(BlocklistResponse),
    DownloadsResponse(DownloadsResponse),
    DiskSpaces(Vec<DiskSpace>),
    Episode(Episode),
    Episodes(Vec<Episode>),
    EpisodeFiles(Vec<EpisodeFile>),
    HostConfig(HostConfig),
    IndexerSettings(IndexerSettings),
    Indexers(Vec<Indexer>),
    IndexerTestResults(Vec<IndexerTestResult>),
    LanguageProfiles(Vec<Language>),
    LogResponse(LogResponse),
    QualityProfiles(Vec<QualityProfile>),
    QueueEvents(Vec<QueueEvent>),
    Releases(Vec<SonarrRelease>),
    RootFolders(Vec<RootFolder>),
    SecurityConfig(SecurityConfig),
    SeriesVec(Vec<Series>),
    Series(Series),
    SonarrHistoryItems(Vec<SonarrHistoryItem>),
    SonarrHistoryWrapper(SonarrHistoryWrapper),
    SystemStatus(SystemStatus),
    Tag(Tag),
    Tags(Vec<Tag>),
    Tasks(Vec<SonarrTask>),
    Updates(Vec<Update>),
    Value(Value),
  }
);

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SystemStatus {
  pub version: String,
  pub start_time: DateTime<Utc>,
}
