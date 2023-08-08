use chrono::{DateTime, Utc};

#[derive(Default, Debug)]
pub struct RadarrData {
  pub free_space: u64,
  pub total_space: u64,
  pub version: String,
  pub start_time: DateTime<Utc>
}
