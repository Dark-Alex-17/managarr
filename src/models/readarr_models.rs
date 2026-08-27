use super::{
  HorizontallyScrollableText, Serdeable,
  servarr_models::{
    DiskSpace, HostConfig, Indexer, IndexerTestResult, MetadataProfile, QualityProfile,
    QualityWrapper, RootFolder, SecurityConfig, SystemStatus, Tag,
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
#[path = "readarr_models_tests.rs"]
mod readarr_models_tests;

#[derive(Derivative, Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Author {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  pub author_name: HorizontallyScrollableText,
  pub foreign_author_id: String,
  pub status: AuthorStatus,
  pub overview: Option<String>,
  pub author_type: Option<String>,
  pub disambiguation: Option<String>,
  pub path: String,
  pub root_folder_path: Option<String>,
  #[serde(deserialize_with = "super::from_i64")]
  pub quality_profile_id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub metadata_profile_id: i64,
  pub monitored: bool,
  pub monitor_new_items: NewItemMonitorType,
  pub genres: Vec<String>,
  pub tags: Vec<Number>,
  pub ratings: Option<Ratings>,
  pub statistics: Option<AuthorStatistics>,
}

#[derive(
  Serialize, Deserialize, Default, PartialEq, Eq, Clone, Copy, Debug, Display, EnumDisplayStyle,
)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
pub enum AuthorStatus {
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
  pub popularity: Option<f64>,
}

impl Eq for Ratings {}

#[derive(Derivative, Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AuthorStatistics {
  #[serde(deserialize_with = "super::from_i64")]
  pub book_count: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub book_file_count: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub total_book_count: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub available_book_count: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub size_on_disk: i64,
  #[serde(deserialize_with = "super::from_f64")]
  pub percent_of_books: f64,
}

impl Eq for AuthorStatistics {}

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
  #[display_style(name = "All Books")]
  All,
  #[display_style(name = "No New Books")]
  None,
  #[display_style(name = "New Books")]
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
  #[display_style(name = "All Books")]
  All,
  #[display_style(name = "Future Books")]
  Future,
  #[display_style(name = "Missing Books")]
  Missing,
  #[display_style(name = "Existing Books")]
  Existing,
  #[display_style(name = "First Book")]
  First,
  #[display_style(name = "Latest Book")]
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
  pub book_id: Option<Number>,
  pub author_id: Option<Number>,
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

#[derive(Derivative, Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AddAuthorSearchResult {
  pub foreign_author_id: String,
  pub author_name: HorizontallyScrollableText,
  pub status: AuthorStatus,
  #[serde(default)]
  pub ended: bool,
  pub overview: Option<String>,
  pub author_type: Option<String>,
  pub disambiguation: Option<String>,
  pub genres: Vec<String>,
  pub ratings: Option<Ratings>,
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
pub struct AddAuthorBody {
  pub foreign_author_id: String,
  pub author_name: String,
  pub monitored: bool,
  pub root_folder_path: String,
  pub quality_profile_id: i64,
  pub metadata_profile_id: i64,
  pub tags: Vec<i64>,
  #[serde(skip_serializing, skip_deserializing)]
  pub tag_input_string: Option<String>,
  pub add_options: AddAuthorOptions,
}

#[derive(Default, Clone, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AddAuthorOptions {
  pub monitor: MonitorType,
  pub monitor_new_items: NewItemMonitorType,
  pub search_for_missing_books: bool,
}

#[derive(Default, Clone, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EditAuthorParams {
  pub author_id: i64,
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
pub struct AddReadarrRootFolderBody {
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
pub struct Book {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  pub title: HorizontallyScrollableText,
  #[serde(deserialize_with = "super::from_i64")]
  pub author_id: i64,
  pub foreign_book_id: String,
  pub monitored: bool,
  #[serde(default)]
  pub any_edition_ok: bool,
  pub page_count: Option<i64>,
  pub ratings: Option<Ratings>,
  pub release_date: Option<DateTime<Utc>>,
  pub statistics: Option<BookStatistics>,
  pub grabbed: bool,
}

#[derive(Derivative, Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BookStatistics {
  #[serde(deserialize_with = "super::from_i64")]
  pub book_file_count: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub total_book_count: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub size_on_disk: i64,
  #[serde(deserialize_with = "super::from_f64")]
  pub percent_of_books: f64,
}

impl Eq for BookStatistics {}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Edition {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub book_id: i64,
  pub foreign_edition_id: String,
  pub monitored: bool,
  pub is_ebook: bool,
  pub title: String,
  pub language: Option<String>,
  pub overview: Option<String>,
  pub format: Option<String>,
  pub publisher: Option<String>,
  pub page_count: Option<i64>,
  pub release_date: Option<DateTime<Utc>>,
  pub isbn13: Option<String>,
  pub asin: Option<String>,
  pub ratings: Option<Ratings>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ReadarrHistoryWrapper {
  pub records: Vec<ReadarrHistoryItem>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ReadarrHistoryData {
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
pub enum ReadarrHistoryEventType {
  #[default]
  Unknown,
  Grabbed,
  #[display_style(name = "Author Folder Imported")]
  AuthorFolderImported,
  #[display_style(name = "Book Import Incomplete")]
  BookImportIncomplete,
  #[display_style(name = "Download Ignored")]
  DownloadIgnored,
  #[display_style(name = "Download Imported")]
  DownloadImported,
  #[display_style(name = "Download Failed")]
  DownloadFailed,
  #[display_style(name = "Book File Deleted")]
  BookFileDeleted,
  #[display_style(name = "Book File Imported")]
  BookFileImported,
  #[display_style(name = "Book File Renamed")]
  BookFileRenamed,
  #[display_style(name = "Book File Retagged")]
  BookFileRetagged,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ReadarrHistoryItem {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub author_id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub book_id: i64,
  pub source_title: HorizontallyScrollableText,
  #[serde(default)]
  pub quality: QualityWrapper,
  pub date: DateTime<Utc>,
  pub event_type: ReadarrHistoryEventType,
  #[serde(default)]
  pub data: ReadarrHistoryData,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ReadarrTask {
  pub name: String,
  pub task_name: ReadarrTaskName,
  #[serde(deserialize_with = "super::from_i64")]
  pub interval: i64,
  pub last_execution: DateTime<Utc>,
  pub last_duration: String,
  pub next_execution: DateTime<Utc>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Copy, ValueEnum)]
#[serde(rename_all = "PascalCase")]
pub enum ReadarrTaskName {
  #[default]
  ApplicationUpdateCheck,
  Backup,
  CheckHealth,
  Housekeeping,
  ImportListSync,
  MessagingCleanup,
  RefreshAuthor,
  RefreshMonitoredDownloads,
  RescanFolders,
  RssSync,
}

impl Display for ReadarrTaskName {
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
pub struct ReadarrRelease {
  pub guid: String,
  pub protocol: String,
  #[serde(deserialize_with = "super::from_i64")]
  pub age: i64,
  pub title: HorizontallyScrollableText,
  pub author_name: Option<String>,
  pub book_title: Option<String>,
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

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BlocklistItem {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub author_id: i64,
  pub book_ids: Option<Vec<Number>>,
  pub source_title: String,
  pub quality: QualityWrapper,
  pub date: DateTime<Utc>,
  pub protocol: String,
  pub indexer: String,
  pub message: String,
  pub author: Author,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct BlocklistResponse {
  pub records: Vec<BlocklistItem>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BookFile {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub author_id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub book_id: i64,
  pub path: String,
  #[serde(deserialize_with = "super::from_i64")]
  pub size: i64,
  pub date_added: DateTime<Utc>,
  pub quality: QualityWrapper,
  pub media_info: Option<MediaInfo>,
  pub quality_cutoff_not_met: bool,
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

impl From<ReadarrSerdeable> for Serdeable {
  fn from(value: ReadarrSerdeable) -> Serdeable {
    Serdeable::Readarr(value)
  }
}

serde_enum_from!(
  ReadarrSerdeable {
    AddAuthorSearchResults(Vec<AddAuthorSearchResult>),
    Author(Author),
    Authors(Vec<Author>),
    BlocklistResponse(BlocklistResponse),
    Book(Book),
    BookFiles(Vec<BookFile>),
    Books(Vec<Book>),
    DiskSpaces(Vec<DiskSpace>),
    DownloadsResponse(DownloadsResponse),
    Editions(Vec<Edition>),
    HostConfig(HostConfig),
    Indexers(Vec<Indexer>),
    IndexerSettings(IndexerSettings),
    IndexerTestResults(Vec<IndexerTestResult>),
    LogResponse(LogResponse),
    MetadataProfiles(Vec<MetadataProfile>),
    QualityProfiles(Vec<QualityProfile>),
    QueueEvents(Vec<QueueEvent>),
    ReadarrHistoryItems(Vec<ReadarrHistoryItem>),
    ReadarrHistoryWrapper(ReadarrHistoryWrapper),
    Releases(Vec<ReadarrRelease>),
    RootFolders(Vec<RootFolder>),
    SecurityConfig(SecurityConfig),
    SystemStatus(SystemStatus),
    Tag(Tag),
    Tags(Vec<Tag>),
    Tasks(Vec<ReadarrTask>),
    Updates(Vec<Update>),
    Value(Value),
  }
);
