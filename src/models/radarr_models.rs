use std::fmt::{Display, Formatter};

use chrono::{DateTime, Utc};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_json::Number;
use strum_macros::EnumIter;

use crate::models::HorizontallyScrollableText;

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

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
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
  #[serde(default)]
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
  pub size: Number,
  #[derivative(Default(value = "Number::from(0)"))]
  pub sizeleft: Number,
  pub output_path: Option<HorizontallyScrollableText>,
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
  pub quality: QualityWrapper,
  pub languages: Vec<Language>,
  pub date: DateTime<Utc>,
  pub event_type: String,
}

#[derive(Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct Language {
  pub name: String,
}

#[derive(Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct Quality {
  pub name: String,
}

#[derive(Deserialize, Default, Debug, Clone, PartialEq, Eq)]
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

#[derive(Default, Serialize, Debug)]
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

#[derive(Default, PartialEq, Eq, Clone, Debug, EnumIter)]
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
  pub fn to_display_str(&self) -> &str {
    match self {
      MinimumAvailability::Tba => "TBA",
      MinimumAvailability::Announced => "Announced",
      MinimumAvailability::InCinemas => "In Cinemas",
      MinimumAvailability::Released => "Released",
    }
  }
}

#[derive(Default, PartialEq, Eq, Clone, Debug, EnumIter)]
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
  pub fn to_display_str(&self) -> &str {
    match self {
      Monitor::MovieOnly => "Movie only",
      Monitor::MovieAndCollection => "Movie and Collection",
      Monitor::None => "None",
    }
  }
}

#[cfg(test)]
mod tests {
  use pretty_assertions::assert_str_eq;

  use crate::models::radarr_models::{MinimumAvailability, Monitor};

  #[test]
  fn test_minimum_availability_display() {
    assert_str_eq!(MinimumAvailability::Tba.to_string(), "tba");
    assert_str_eq!(MinimumAvailability::Announced.to_string(), "announced");
    assert_str_eq!(MinimumAvailability::InCinemas.to_string(), "inCinemas");
    assert_str_eq!(MinimumAvailability::Released.to_string(), "released");
  }

  #[test]
  fn test_minimum_availability_to_display_str() {
    assert_str_eq!(MinimumAvailability::Tba.to_display_str(), "TBA");
    assert_str_eq!(MinimumAvailability::Announced.to_display_str(), "Announced");
    assert_str_eq!(
      MinimumAvailability::InCinemas.to_display_str(),
      "In Cinemas"
    );
    assert_str_eq!(MinimumAvailability::Released.to_display_str(), "Released");
  }

  #[test]
  fn test_monitor_display() {
    assert_str_eq!(Monitor::MovieOnly.to_string(), "movieOnly");
    assert_str_eq!(
      Monitor::MovieAndCollection.to_string(),
      "movieAndCollection"
    );
    assert_str_eq!(Monitor::None.to_string(), "none");
  }

  #[test]
  fn test_monitor_to_display_str() {
    assert_str_eq!(Monitor::MovieOnly.to_display_str(), "Movie only");
    assert_str_eq!(
      Monitor::MovieAndCollection.to_display_str(),
      "Movie and Collection"
    );
    assert_str_eq!(Monitor::None.to_display_str(), "None");
  }
}
