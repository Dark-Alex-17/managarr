use super::{
  HorizontallyScrollableText, Serdeable,
  servarr_models::{
    DiskSpace, HostConfig, Indexer, IndexerTestResult, QualityProfile, QualityWrapper, RootFolder,
    SecurityConfig, Tag,
  },
};
use crate::models::servarr_models::{IndexerSettings, LogResponse, QueueEvent, Update};
use crate::serde_enum_from;
use chrono::{DateTime, Utc};
use clap::ValueEnum;
use derivative::Derivative;
use enum_display_style_derive::EnumDisplayStyle;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use std::fmt::{Display, Formatter};
use strum::{Display, EnumIter};

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
pub enum MonitorType {
  #[default]
  #[display_style(name = "All Albums")]
  All,
  #[display_style(name = "Future Albums")]
  Future,
  #[display_style(name = "Missing Albums")]
  Missing,
  #[display_style(name = "Existing Albums")]
  Existing,
  #[display_style(name = "First Album")]
  First,
  #[display_style(name = "Latest Album")]
  Latest,
  None,
  Unknown,
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

#[derive(Derivative, Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AddArtistSearchResult {
  pub foreign_artist_id: String,
  pub artist_name: HorizontallyScrollableText,
  pub status: ArtistStatus,
  pub overview: Option<String>,
  pub artist_type: Option<String>,
  pub disambiguation: Option<String>,
  pub genres: Vec<String>,
  pub ratings: Option<Ratings>,
}

#[derive(Default, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LidarrCommandBody {
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub artist_id: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub album_ids: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct DeleteParams {
  pub id: i64,
  pub delete_files: bool,
  pub add_import_list_exclusion: bool,
}

#[derive(Default, Clone, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AddArtistBody {
  pub foreign_artist_id: String,
  pub artist_name: String,
  pub monitored: bool,
  pub root_folder_path: String,
  pub quality_profile_id: i64,
  pub metadata_profile_id: i64,
  pub tags: Vec<i64>,
  #[serde(skip_serializing, skip_deserializing)]
  pub tag_input_string: Option<String>,
  pub add_options: AddArtistOptions,
}

#[derive(Default, Clone, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AddArtistOptions {
  pub monitor: MonitorType,
  pub monitor_new_items: NewItemMonitorType,
  pub search_for_missing_albums: bool,
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

#[derive(Default, Clone, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AddLidarrRootFolderBody {
  pub name: String,
  pub path: String,
  pub default_quality_profile_id: i64,
  pub default_metadata_profile_id: i64,
  pub default_monitor_option: MonitorType,
  pub default_new_item_monitor_option: NewItemMonitorType,
  pub default_tags: Vec<i64>,
  #[serde(skip_serializing, skip_deserializing)]
  pub tag_input_string: Option<String>,
}

#[derive(Derivative, Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Album {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  pub title: HorizontallyScrollableText,
  pub foreign_album_id: String,
  pub monitored: bool,
  #[serde(default)]
  pub any_release_ok: bool,
  #[serde(deserialize_with = "super::from_i64")]
  pub profile_id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub duration: i64,
  pub album_type: Option<String>,
  pub genres: Vec<String>,
  pub ratings: Option<Ratings>,
  pub release_date: Option<DateTime<Utc>>,
  pub statistics: Option<AlbumStatistics>,
}

#[derive(Derivative, Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AlbumStatistics {
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

impl Eq for AlbumStatistics {}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LidarrHistoryWrapper {
  pub records: Vec<LidarrHistoryItem>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LidarrHistoryData {
  pub indexer: Option<String>,
  pub release_group: Option<String>,
  pub nzb_info_url: Option<String>,
  pub download_client_name: Option<String>,
  pub download_client: Option<String>,
  pub age: Option<String>,
  pub published_date: Option<DateTime<Utc>>,
  pub message: Option<String>,
  pub reason: Option<String>,
  pub dropped_path: Option<String>,
  pub imported_path: Option<String>,
  pub source_path: Option<String>,
  pub path: Option<String>,
  pub status_messages: Option<String>,
}

#[derive(
  Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Display, EnumDisplayStyle,
)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
pub enum LidarrHistoryEventType {
  #[default]
  Unknown,
  Grabbed,
  #[display_style(name = "Artist Folder Imported")]
  ArtistFolderImported,
  #[display_style(name = "Album Import Incomplete")]
  AlbumImportIncomplete,
  #[display_style(name = "Download Ignored")]
  DownloadIgnored,
  #[display_style(name = "Download Imported")]
  DownloadImported,
  #[display_style(name = "Download Failed")]
  DownloadFailed,
  #[display_style(name = "Track File Deleted")]
  TrackFileDeleted,
  #[display_style(name = "Track File Imported")]
  TrackFileImported,
  #[display_style(name = "Track File Renamed")]
  TrackFileRenamed,
  #[display_style(name = "Track File Retagged")]
  TrackFileRetagged,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LidarrHistoryItem {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  pub source_title: HorizontallyScrollableText,
  #[serde(deserialize_with = "super::from_i64")]
  pub album_id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub artist_id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub track_id: i64,
  #[serde(default)]
  pub quality: QualityWrapper,
  pub date: DateTime<Utc>,
  pub event_type: LidarrHistoryEventType,
  #[serde(default)]
  pub data: LidarrHistoryData,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LidarrTask {
  pub name: String,
  pub task_name: LidarrTaskName,
  #[serde(deserialize_with = "super::from_i64")]
  pub interval: i64,
  pub last_execution: DateTime<Utc>,
  pub next_execution: DateTime<Utc>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Copy, ValueEnum)]
#[serde(rename_all = "PascalCase")]
pub enum LidarrTaskName {
  #[default]
  ApplicationUpdateCheck,
  Backup,
  CheckHealth,
  Housekeeping,
  ImportListSync,
  MessagingCleanup,
  RefreshArtist,
  RefreshMonitoredDownloads,
  RescanFolders,
  RssSync,
}

impl Display for LidarrTaskName {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let task_name = serde_json::to_string(&self)
      .expect("Unable to serialize task name")
      .replace('"', "");
    write!(f, "{task_name}")
  }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct LidarrRelease {
  pub guid: String,
  pub protocol: String,
  #[serde(deserialize_with = "super::from_i64")]
  pub age: i64,
  pub title: HorizontallyScrollableText,
  pub discography: bool,
  pub artist_name: Option<String>,
  pub album_title: Option<String>,
  pub indexer: String,
  #[serde(deserialize_with = "super::from_i64")]
  pub indexer_id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub size: i64,
  pub rejected: bool,
  pub rejections: Option<Vec<String>>,
  pub seeders: Option<Number>,
  pub leechers: Option<Number>,
  pub quality: QualityWrapper,
}

#[derive(Default, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LidarrReleaseDownloadBody {
  pub guid: String,
  pub indexer_id: i64,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BlocklistItem {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub artist_id: i64,
  pub album_ids: Option<Vec<Number>>,
  pub source_title: String,
  pub quality: QualityWrapper,
  pub date: DateTime<Utc>,
  pub protocol: String,
  pub indexer: String,
  pub message: String,
  pub artist: Artist,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct BlocklistResponse {
  pub records: Vec<BlocklistItem>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TrackFile {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  pub path: String,
  #[serde(deserialize_with = "super::from_i64")]
  pub size: i64,
  pub quality: QualityWrapper,
  pub date_added: DateTime<Utc>,
  pub media_info: Option<MediaInfo>,
  pub audio_tags: Option<AudioTags>,
}

#[derive(Serialize, Deserialize, Derivative, Debug, Clone, PartialEq, Eq)]
#[derivative(Default)]
#[serde(rename_all = "camelCase")]
pub struct MediaInfo {
  pub audio_bit_rate: Option<String>,
  #[serde(deserialize_with = "super::from_i64")]
  pub audio_channels: i64,
  pub audio_codec: Option<String>,
  pub audio_bits: Option<String>,
  pub audio_sample_rate: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AudioTags {
  pub title: String,
  pub artist_title: String,
  pub album_title: String,
  #[serde(deserialize_with = "super::from_i64")]
  pub disc_number: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub disc_count: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub year: i64,
  pub duration: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Track {
  pub id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub artist_id: i64,
  pub foreign_track_id: String,
  #[serde(deserialize_with = "super::from_i64")]
  pub track_file_id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub album_id: i64,
  pub explicit: bool,
  pub track_number: String,
  pub title: String,
  #[serde(deserialize_with = "super::from_i64")]
  pub duration: i64,
  pub has_file: bool,
  pub ratings: Ratings,
  pub track_file: Option<TrackFile>,
}

impl From<LidarrSerdeable> for Serdeable {
  fn from(value: LidarrSerdeable) -> Serdeable {
    Serdeable::Lidarr(value)
  }
}

serde_enum_from!(
  LidarrSerdeable {
    AddArtistSearchResults(Vec<AddArtistSearchResult>),
    Albums(Vec<Album>),
    Album(Album),
    Artist(Artist),
    Artists(Vec<Artist>),
    BlocklistResponse(BlocklistResponse),
    DiskSpaces(Vec<DiskSpace>),
    DownloadsResponse(DownloadsResponse),
    LidarrHistoryWrapper(LidarrHistoryWrapper),
    LidarrHistoryItems(Vec<LidarrHistoryItem>),
    HostConfig(HostConfig),
    IndexerSettings(IndexerSettings),
    Indexers(Vec<Indexer>),
    IndexerTestResults(Vec<IndexerTestResult>),
    LogResponse(LogResponse),
    MetadataProfiles(Vec<MetadataProfile>),
    QualityProfiles(Vec<QualityProfile>),
    QueueEvents(Vec<QueueEvent>),
    Releases(Vec<LidarrRelease>),
    RootFolders(Vec<RootFolder>),
    SecurityConfig(SecurityConfig),
    SystemStatus(SystemStatus),
    Tag(Tag),
    Tags(Vec<Tag>),
    Tasks(Vec<LidarrTask>),
    Track(Track),
    Tracks(Vec<Track>),
    TrackFiles(Vec<TrackFile>),
    Updates(Vec<Update>),
    Value(Value),
  }
);
