use chrono::{DateTime, Utc};

use crate::app::StatefulTable;
use crate::network::radarr::Movie;

#[derive(Default)]
pub struct RadarrData {
  pub free_space: u64,
  pub total_space: u64,
  pub version: String,
  pub start_time: DateTime<Utc>,
  pub movies: StatefulTable<Movie>
}
