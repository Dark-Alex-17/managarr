use crate::app::App;
use crate::app::context_clues::{BARE_POPUP_CONTEXT_CLUES, ContextClue, ContextClueProvider};
use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::models::Route;
use crate::models::servarr_data::lidarr::lidarr_data::{ADD_ARTIST_BLOCKS, ActiveLidarrBlock};

#[cfg(test)]
#[path = "lidarr_context_clues_tests.rs"]
mod lidarr_context_clues_tests;

pub static ARTISTS_CONTEXT_CLUES: [ContextClue; 10] = [
  (DEFAULT_KEYBINDINGS.add, DEFAULT_KEYBINDINGS.add.desc),
  (
    DEFAULT_KEYBINDINGS.toggle_monitoring,
    DEFAULT_KEYBINDINGS.toggle_monitoring.desc,
  ),
  (DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc),
  (DEFAULT_KEYBINDINGS.edit, DEFAULT_KEYBINDINGS.edit.desc),
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

pub static ADD_ARTIST_SEARCH_RESULTS_CONTEXT_CLUES: [ContextClue; 2] = [
  (DEFAULT_KEYBINDINGS.submit, "details"),
  (DEFAULT_KEYBINDINGS.esc, "edit search"),
];

pub(in crate::app) struct LidarrContextClueProvider;

impl ContextClueProvider for LidarrContextClueProvider {
  fn get_context_clues(app: &mut App<'_>) -> Option<&'static [ContextClue]> {
    let Route::Lidarr(active_lidarr_block, _context_option) = app.get_current_route() else {
      panic!("LidarrContextClueProvider::get_context_clues called with non-Lidarr route");
    };

    match active_lidarr_block {
      ActiveLidarrBlock::AddArtistSearchInput | ActiveLidarrBlock::AddArtistEmptySearchResults => {
        Some(&BARE_POPUP_CONTEXT_CLUES)
      }
      _ if EDIT_ARTIST_BLOCKS.contains(&active_lidarr_block) => {
        Some(&CONFIRMATION_PROMPT_CONTEXT_CLUES)
      }
      _ if ADD_ARTIST_BLOCKS.contains(&active_lidarr_block) => {
        Some(&ADD_ARTIST_SEARCH_RESULTS_CONTEXT_CLUES)
      }
      _ => app
        .data
        .lidarr_data
        .main_tabs
        .get_active_route_contextual_help(),
    }
  }
}
