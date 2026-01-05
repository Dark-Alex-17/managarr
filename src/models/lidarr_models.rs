use chrono::{DateTime, Utc};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

use super::{HorizontallyScrollableText, Serdeable};
use crate::serde_enum_from;

#[derive(Derivative, Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
  #[serde(deserialize_with = "super::from_i64")]
  pub id: i64,
  pub mb_id: String,
  pub artist_name: HorizontallyScrollableText,
  pub foreign_artist_id: String,
  pub status: ArtistStatus,
  pub overview: Option<String>,
  pub artist_type: Option<String>,
  pub disambiguation: Option<String>,
  pub path: String,
  #[serde(deserialize_with = "super::from_i64")]
  pub quality_profile_id: i64,
  #[serde(deserialize_with = "super::from_i64")]
  pub metadata_profile_id: i64,
  pub monitored: bool,
  pub genres: Vec<String>,
  pub tags: Vec<Number>,
  pub added: DateTime<Utc>,
  pub ratings: Option<Ratings>,
  pub statistics: Option<ArtistStatistics>,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Clone, Copy, Debug)]
#[serde(rename_all = "camelCase")]
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

impl From<LidarrSerdeable> for Serdeable {
  fn from(value: LidarrSerdeable) -> Serdeable {
    Serdeable::Lidarr(value)
  }
}

serde_enum_from!(
  LidarrSerdeable {
    Artists(Vec<Artist>),
    Value(Value),
  }
);
