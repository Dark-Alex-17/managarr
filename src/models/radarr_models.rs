use std::fmt::{Display, Formatter};

use chrono::{DateTime, Utc};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_json::Number;

use crate::models::HorizontallyScrollableText;

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RootFolder {
  pub path: String,
  pub accessible: bool,
  pub free_space: Number,
}

#[derive(Derivative, Deserialize, Debug, Clone, PartialEq, Eq)]
#[derivative(Default)]
#[serde(rename_all = "camelCase")]
pub struct Movie {
  #[derivative(Default(value = "Number::from(0)"))]
  pub id: Number,
  pub title: String,
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
  pub quality_profile_id: Number,
  pub certification: Option<String>,
  pub ratings: RatingsList,
  pub movie_file: Option<MovieFile>,
  pub collection: Option<Collection>,
}

#[derive(Derivative, Deserialize, Debug, Clone, PartialEq, Eq)]
#[derivative(Default)]
#[serde(rename_all = "camelCase")]
pub struct CollectionMovie {
  pub title: String,
  pub overview: String,
  #[derivative(Default(value = "Number::from(0)"))]
  pub year: Number,
  #[derivative(Default(value = "Number::from(0)"))]
  pub runtime: Number,
  pub genres: Vec<String>,
  pub ratings: RatingsList,
}

#[derive(Deserialize, Derivative, Clone, Debug, PartialEq, Eq)]
#[derivative(Default)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
  pub title: String,
  pub root_folder_path: Option<String>,
  pub search_on_add: bool,
  pub overview: Option<String>,
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
  pub media_info: MediaInfo,
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
  pub size: Number,
  #[derivative(Default(value = "Number::from(0)"))]
  pub sizeleft: Number,
  pub output_path: HorizontallyScrollableText,
  pub indexer: String,
  pub download_client: String,
}

#[derive(Derivative, Deserialize, Debug)]
#[derivative(Default)]
#[serde(rename_all = "camelCase")]
pub struct QualityProfile {
  #[derivative(Default(value = "Number::from(0)"))]
  pub id: Number,
  pub name: String,
}

#[derive(Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MovieHistoryItem {
  pub source_title: HorizontallyScrollableText,
  pub quality: QualityHistory,
  pub languages: Vec<Language>,
  pub date: DateTime<Utc>,
  pub event_type: String,
}

#[derive(Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct Language {
  pub name: String,
}

#[derive(Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct QualityHistory {
  pub quality: Quality,
}

#[derive(Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct Quality {
  pub name: String,
}

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CreditType {
  Cast,
  Crew,
}

#[derive(Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Credit {
  pub person_name: String,
  pub character: Option<String>,
  pub department: Option<String>,
  pub job: Option<String>,
  #[serde(rename(deserialize = "type"))]
  pub credit_type: CreditType,
}

#[derive(Default, Derivative, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddMovieBody {
  pub tmdb_id: u64,
  pub title: String,
  pub root_folder_path: String,
  pub quality_profile_id: u64,
  pub minimum_availability: String,
  pub monitored: bool,
  pub add_options: AddOptions,
}

#[derive(Default, Serialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AddOptions {
  pub monitor: String,
  pub search_for_movie: bool,
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

#[derive(Default, PartialEq, Eq, Clone, Debug)]
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
  pub fn vec() -> Vec<Self> {
    vec![
      MinimumAvailability::Announced,
      MinimumAvailability::InCinemas,
      MinimumAvailability::Released,
      MinimumAvailability::Tba,
    ]
  }

  pub fn to_display_str(&self) -> &str {
    match self {
      MinimumAvailability::Announced => "Announced",
      MinimumAvailability::InCinemas => "In Cinemas",
      MinimumAvailability::Released => "Released",
      MinimumAvailability::Tba => "TBA",
    }
  }
}

#[derive(Default, PartialEq, Eq, Clone, Debug)]
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
  pub fn vec() -> Vec<Self> {
    vec![
      Monitor::MovieOnly,
      Monitor::MovieAndCollection,
      Monitor::None,
    ]
  }

  pub fn to_display_str(&self) -> &str {
    match self {
      Monitor::MovieOnly => "Movie only",
      Monitor::MovieAndCollection => "Movie and Collection",
      Monitor::None => "None",
    }
  }
}
