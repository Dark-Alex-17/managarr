use chrono::Duration;

use crate::network::radarr::DiskSpace;

#[derive(Default, Debug)]
pub struct RadarrData {
  pub free_space: u64,
  pub total_space: u64,
  pub version: String,
}
