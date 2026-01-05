use crate::app::App;
use crate::app::context_clues::{ContextClue, ContextClueProvider};
use crate::models::Route;

#[cfg(test)]
#[path = "lidarr_context_clues_tests.rs"]
mod lidarr_context_clues_tests;

pub(in crate::app) struct LidarrContextClueProvider;

impl ContextClueProvider for LidarrContextClueProvider {
  fn get_context_clues(app: &mut App<'_>) -> Option<&'static [ContextClue]> {
    let Route::Lidarr(active_lidarr_block, _context_option) = app.get_current_route() else {
      panic!("LidarrContextClueProvider::get_context_clues called with non-Lidarr route");
    };

    match active_lidarr_block {
      _ => app
        .data
        .lidarr_data
        .main_tabs
        .get_active_route_contextual_help(),
    }
  }
}
