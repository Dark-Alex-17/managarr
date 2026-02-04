use crate::models::Route;

pub mod lidarr;
pub mod modals;
pub mod radarr;
pub mod sonarr;

#[cfg(test)]
pub(in crate::models::servarr_data) mod data_test_utils;
#[cfg(test)]
mod servarr_data_tests;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Notification {
  pub title: String,
  pub message: String,
  pub success: bool,
}

impl Notification {
  pub fn new(title: String, message: String, success: bool) -> Self {
    Self {
      title,
      message,
      success,
    }
  }
}

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
