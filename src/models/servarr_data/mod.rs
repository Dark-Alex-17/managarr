use crate::models::Route;

pub mod modals;
pub mod radarr;
pub mod sonarr;
pub mod lidarr;

#[cfg(test)]
pub(in crate::models::servarr_data) mod data_test_utils;
#[cfg(test)]
mod servarr_data_tests;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ActiveKeybindingBlock {
  #[default]
  Help,
}

impl From<ActiveKeybindingBlock> for Route {
  fn from(_active_keybinding_block: ActiveKeybindingBlock) -> Route {
    Route::Keybindings
  }
}
