use strum::EnumIter;
#[cfg(test)]
use strum::{Display, EnumString};

use crate::models::Route;

#[cfg(test)]
#[path = "lidarr_data_tests.rs"]
mod lidarr_data_tests;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, EnumIter)]
#[cfg_attr(test, derive(Display, EnumString))]
pub enum ActiveLidarrBlock {
  #[default]
  Artists,
}

impl From<ActiveLidarrBlock> for Route {
  fn from(active_lidarr_block: ActiveLidarrBlock) -> Route {
    Route::Lidarr(active_lidarr_block, None)
  }
}

impl From<(ActiveLidarrBlock, Option<ActiveLidarrBlock>)> for Route {
  fn from(value: (ActiveLidarrBlock, Option<ActiveLidarrBlock>)) -> Route {
    Route::Lidarr(value.0, value.1)
  }
}
