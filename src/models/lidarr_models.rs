use std::fmt::{Display, Formatter};

use chrono::{DateTime, Utc};
use clap::ValueEnum;
use derivative::Derivative;
use enum_display_style_derive::EnumDisplayStyle;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use strum::EnumIter;
use strum_macros::Display;
use crate::models::radarr_models::IndexerTestResult;
use crate::models::servarr_models::{DiskSpace, HostConfig, Indexer, Language, LogResponse, QualityProfile, QualityWrapper, QueueEvent, RootFolder, SecurityConfig, Tag, Update};
use super::{HorizontallyScrollableText, Serdeable, from_f64, from_i64};
use crate::serde_enum_from;

// #[cfg(test)]
// #[path = "lidarr_models_tests.rs"]
// mod lidarr_models_tests;

#[derive(Default, Clone, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AddArtistBody {
    pub foreign_artist_id: String,
    pub quality_profile_id: i64,
    pub monitored: bool,
    pub root_folder_path: String,
    pub metadata_profile_id: i64,
    pub tags: Vec<i64>,
    #[serde(skip_serializing, skip_deserializing)]
    pub tag_input_string: Option<String>,
    pub add_options: AddArtistOptions,
}

#[derive(Default, Clone, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AddArtistOptions {
    pub monitor: String,
    pub search_for_missing_albums: bool,
}

#[derive(Derivative, Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AddArtistSearchResult {
    #[serde(deserialize_with = "from_i64")]
    pub id: i64,
    pub artist_name: HorizontallyScrollableText,
    pub foreign_artist_id: String,
    pub disambiguation: Option<String>,
    pub overview: Option<String>,
    pub status: Option<String>,
    pub genres: Vec<String>,
    #[serde(deserialize_with = "from_i64")]
    pub year: i64,
    pub ratings: Option<Rating>,
    pub statistics: Option<AddArtistSearchResultStatistics>,
}

#[derive(Derivative, Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AddArtistSearchResultStatistics {
    #[serde(deserialize_with = "from_i64")]
    pub album_count: i64,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    #[serde(deserialize_with = "from_i64")]
    pub id: i64,
    pub artist_name: HorizontallyScrollableText,
    pub foreign_artist_id: String,
    #[serde(deserialize_with = "from_i64")]
    pub quality_profile_id: i64,
    #[serde(deserialize_with = "from_i64")]
    pub metadata_profile_id: i64,
    pub monitored: bool,
    pub path: String,
    pub genres: Vec<String>,
    pub tags: Vec<Number>,
    pub ratings: Rating,
    pub status: ArtistStatus,
    pub overview: Option<String>,
    pub statistics: Option<ArtistStatistics>,
    pub albums: Option<Vec<Album>>,
}

#[derive(Derivative, Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ArtistStatistics {
    #[serde(deserialize_with = "from_i64")]
    pub album_count: i64,
    #[serde(deserialize_with = "from_i64")]
    pub track_file_count: i64,
    #[serde(deserialize_with = "from_i64")]
    pub track_count: i64,
    #[serde(deserialize_with = "from_i64")]
    pub total_track_count: i64,
    #[serde(deserialize_with = "from_i64")]
    pub size_on_disk: i64,
    #[serde(deserialize_with = "from_f64")]
    pub percent_of_tracks: f64,
}

impl Eq for ArtistStatistics {}

#[derive(
    Serialize,
    Deserialize,
    Default,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Debug,
    ValueEnum,
    EnumIter,
    Display,
    EnumDisplayStyle,
)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
pub enum ArtistStatus {
    #[default]
    Active,
    Ended,
    Upcoming,
    Deleted,
}

#[derive(Derivative, Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    #[serde(deserialize_with = "from_i64")]
    pub id: i64,
    #[serde(deserialize_with = "from_i64")]
    pub artist_id: i64,
    pub title: HorizontallyScrollableText,
    pub monitored: bool,
    pub statistics: Option<AlbumStatistics>,
}

impl Eq for Album {}

#[derive(Derivative, Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AlbumStatistics {
    pub next_airing: Option<DateTime<Utc>>,
    pub previous_airing: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "from_i64")]
    pub track_file_count: i64,
    #[serde(deserialize_with = "from_i64")]
    pub track_count: i64,
    #[serde(deserialize_with = "from_i64")]
    pub total_track_count: i64,
    #[serde(deserialize_with = "from_i64")]
    pub size_on_disk: i64,
    #[serde(deserialize_with = "from_f64")]
    pub percent_of_tracks: f64,
}

impl Eq for AlbumStatistics {}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BlocklistItem {
    #[serde(deserialize_with = "super::from_i64")]
    pub id: i64,
    #[serde(deserialize_with = "super::from_i64")]
    pub artist_id: i64,
    pub album_ids: Vec<Number>,
    pub source_title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist_name: Option<String>,
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

#[derive(Derivative, Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DownloadRecord {
    pub title: String,
    pub status: DownloadStatus,
    #[serde(deserialize_with = "super::from_i64")]
    pub id: i64,
    pub album_id: Option<Number>,
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

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    #[serde(deserialize_with = "from_i64")]
    pub id: i64,
    #[serde(deserialize_with = "from_i64")]
    pub artist_id: i64,
    #[serde(deserialize_with = "from_i64")]
    pub album_id: i64,
    #[serde(deserialize_with = "from_i64")]
    pub track_file_id: i64,
    #[serde(deserialize_with = "from_i64")]
    pub track_number: i64,
    pub title: String,
    pub monitored: bool,
    pub has_file: bool,
    pub track_file: Option<TrackFile>,
}

impl Display for Track {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TrackFile {
    #[serde(deserialize_with = "from_i64")]
    pub id: i64,
    pub relative_path: String,
    pub path: String,
    #[serde(deserialize_with = "from_i64")]
    pub size: i64,
    pub quality: QualityWrapper,
    pub date_added: DateTime<Utc>,
    pub media_info: Option<MediaInfo>,
}

#[derive(Serialize, Deserialize, Derivative, Debug, Clone, PartialEq, Eq)]
#[derivative(Default)]
#[serde(rename_all = "camelCase")]
pub struct MediaInfo {
    #[serde(deserialize_with = "from_i64")]
    pub audio_bitrate: i64,
    #[derivative(Default(value = "Number::from(0)"))]
    pub audio_channels: Number,
    pub audio_codec: Option<String>,
    pub audio_languages: Option<String>,
    #[serde(deserialize_with = "from_i64")]
    pub audio_stream_count: i64,
    #[serde(deserialize_with = "from_i64")]
    pub video_bit_depth: i64,
    #[serde(deserialize_with = "from_i64")]
    pub video_bitrate: i64,
    pub video_codec: Option<String>,
    #[derivative(Default(value = "Number::from(0)"))]
    pub video_fps: Number,
    pub resolution: String,
    pub run_time: String,
    pub scan_type: String,
    pub subtitles: Option<String>,
}

#[derive(Default, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MonitorAlbumBody {
    pub album_ids: Vec<i64>,
    pub monitored: bool,
}

#[derive(Derivative, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[derivative(Default)]
pub struct Rating {
    #[serde(deserialize_with = "from_i64")]
    pub votes: i64,
    #[serde(deserialize_with = "from_f64")]
    pub value: f64,
}

impl Eq for Rating {}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LidarrHistoryWrapper {
    pub records: Vec<LidarrHistoryItem>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LidarrHistoryData {
    pub dropped_path: Option<String>,
    pub imported_path: Option<String>,
    pub indexer: Option<String>,
    pub release_group: Option<String>,
    pub artist_match_type: Option<String>,
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
    #[display_style(name = "Download Folder Imported")]
    DownloadFolderImported,
    #[display_style(name = "Download Failed")]
    DownloadFailed,
    #[display_style(name = "Track File Deleted")]
    TrackFileDeleted,
    #[display_style(name = "Track File Renamed")]
    TrackFileRenamed,
    #[display_style(name = "Download Ignored")]
    DownloadIgnored,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LidarrHistoryItem {
    #[serde(deserialize_with = "from_i64")]
    pub id: i64,
    pub source_title: HorizontallyScrollableText,
    #[serde(deserialize_with = "from_i64")]
    pub album_id: i64,
    pub quality: QualityWrapper,
    pub date: DateTime<Utc>,
    pub event_type: LidarrHistoryEventType,
    pub data: LidarrHistoryData,
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LidarrRelease {
    pub guid: String,
    pub protocol: String,
    #[serde(deserialize_with = "from_i64")]
    pub age: i64,
    pub title: HorizontallyScrollableText,
    pub indexer: String,
    #[serde(deserialize_with = "from_i64")]
    pub indexer_id: i64,
    #[serde(deserialize_with = "from_i64")]
    pub size: i64,
    pub rejected: bool,
    pub rejections: Option<Vec<String>>,
    pub seeders: Option<Number>,
    pub leechers: Option<Number>,
    pub quality: QualityWrapper,
    pub full_album: bool,
}

#[derive(Default, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LidarrReleaseDownloadBody {
    pub guid: String,
    pub indexer_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_id: Option<i64>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LidarrTask {
    pub name: String,
    pub task_name: LidarrTaskName,
    #[serde(deserialize_with = "from_i64")]
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
    CleanUpRecycleBin,
    Housekeeping,
    ImportListSync,
    MessagingCleanup,
    RefreshMonitoredDownloads,
    RefreshArtists,
    RssSync,
    UpdateSceneMapping,
}

impl Display for LidarrTaskName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let task_name = serde_json::to_string(&self)
            .expect("Unable to serialize task name")
            .replace('"', "");
        write!(f, "{task_name}")
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SystemStatus {
    pub version: String,
    pub start_time: DateTime<Utc>,
}

#[derive(Default, Clone, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EditArtistParams {
    pub artist_id: i64,
    pub monitored: Option<bool>,
    pub quality_profile_id: Option<i64>,
    pub metadata_profile_id: Option<i64>,
    pub root_folder_path: Option<String>,
    pub tags: Option<Vec<i64>>,
    #[serde(skip_serializing, skip_deserializing)]
    pub tag_input_string: Option<String>,
    pub clear_tags: bool,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub struct DeleteArtistParams {
    pub id: i64,
    pub delete_artist_files: bool,
    pub add_list_exclusion: bool,
}

#[derive(Default, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LidarrCommandBody {
    pub name: String,
    pub artist_id: Option<i64>,
    pub album_ids: Option<Vec<i64>>,
}

impl From<LidarrSerdeable> for Serdeable {
    fn from(value: LidarrSerdeable) -> Serdeable {
        Serdeable::Lidarr(value)
    }
}

serde_enum_from!(
    LidarrSerdeable {
        AddArtistSearchResults(Vec<AddArtistSearchResult>),
        Artists(Vec<Artist>),
        Artist(Artist),
        Albums(Vec<Album>),
        Album(Album),
        BlocklistResponse(BlocklistResponse),
        DownloadsResponse(DownloadsResponse),
        DiskSpaces(Vec<DiskSpace>),
        HostConfig(HostConfig),
        Indexers(Vec<Indexer>),
        IndexerSettings(IndexerSettings),
        IndexerTestResults(Vec<IndexerTestResult>),
        LogResponse(LogResponse),
        MetadataProfiles(Vec<Language>),
        QualityProfiles(Vec<QualityProfile>),
        QueueEvents(Vec<QueueEvent>),
        RootFolders(Vec<RootFolder>),
        SecurityConfig(SecurityConfig),
        Tag(Tag),
        Tags(Vec<Tag>),
        Tracks(Vec<Track>),
        Track(Track),
        TrackFiles(Vec<TrackFile>),
        LidarrHistoryItems(Vec<LidarrHistoryItem>),
        LidarrHistoryWrapper(LidarrHistoryWrapper),
        LidarrReleases(Vec<LidarrRelease>),
        LidarrTasks(Vec<LidarrTask>),
        Updates(Vec<Update>),
        SystemStatus(SystemStatus),
        Value(Value),
    }
);
