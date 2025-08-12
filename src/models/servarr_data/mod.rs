use crate::models::Route;

pub mod modals;
pub mod radarr;
pub mod sonarr;

#[cfg(test)]
#[path = "servarr_data_tests.rs"]
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
