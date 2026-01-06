use crate::app::App;
use crate::app::context_clues::{ContextClue, ContextClueProvider};
use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::models::Route;

#[cfg(test)]
#[path = "lidarr_context_clues_tests.rs"]
mod lidarr_context_clues_tests;

pub static ARTISTS_CONTEXT_CLUES: [ContextClue; 8] = [
  (
    DEFAULT_KEYBINDINGS.toggle_monitoring,
    DEFAULT_KEYBINDINGS.toggle_monitoring.desc,
  ),
  (DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc),
  (DEFAULT_KEYBINDINGS.delete, DEFAULT_KEYBINDINGS.delete.desc),
  (DEFAULT_KEYBINDINGS.search, DEFAULT_KEYBINDINGS.search.desc),
  (DEFAULT_KEYBINDINGS.filter, DEFAULT_KEYBINDINGS.filter.desc),
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
  (DEFAULT_KEYBINDINGS.update, "update all"),
  (DEFAULT_KEYBINDINGS.esc, "cancel filter"),
];

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
