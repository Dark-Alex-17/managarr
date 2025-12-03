use crate::app::App;
use crate::app::context_clues::{
  BARE_POPUP_CONTEXT_CLUES, CONFIRMATION_PROMPT_CONTEXT_CLUES, ContextClue, ContextClueProvider,
};
use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::models::Route;
use crate::models::servarr_data::radarr::radarr_data::{
  ADD_MOVIE_BLOCKS, ActiveRadarrBlock, EDIT_COLLECTION_BLOCKS, EDIT_INDEXER_BLOCKS,
  EDIT_MOVIE_BLOCKS, INDEXER_SETTINGS_BLOCKS, MOVIE_DETAILS_BLOCKS,
};

#[cfg(test)]
#[path = "radarr_context_clues_tests.rs"]
mod radarr_context_clues_tests;

pub static LIBRARY_CONTEXT_CLUES: [ContextClue; 11] = [
  (DEFAULT_KEYBINDINGS.add, DEFAULT_KEYBINDINGS.add.desc),
  (DEFAULT_KEYBINDINGS.edit, DEFAULT_KEYBINDINGS.edit.desc),
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
  (DEFAULT_KEYBINDINGS.submit, "details"),
  (DEFAULT_KEYBINDINGS.esc, "cancel filter"),
];

pub static COLLECTIONS_CONTEXT_CLUES: [ContextClue; 8] = [
  (DEFAULT_KEYBINDINGS.search, DEFAULT_KEYBINDINGS.search.desc),
  (DEFAULT_KEYBINDINGS.edit, DEFAULT_KEYBINDINGS.edit.desc),
  (DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc),
  (DEFAULT_KEYBINDINGS.filter, DEFAULT_KEYBINDINGS.filter.desc),
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
  (DEFAULT_KEYBINDINGS.update, "update all"),
  (DEFAULT_KEYBINDINGS.submit, "details"),
  (DEFAULT_KEYBINDINGS.esc, "cancel filter"),
];

pub static MOVIE_DETAILS_CONTEXT_CLUES: [ContextClue; 5] = [
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
  (DEFAULT_KEYBINDINGS.update, DEFAULT_KEYBINDINGS.update.desc),
  (DEFAULT_KEYBINDINGS.edit, DEFAULT_KEYBINDINGS.edit.desc),
  (
    DEFAULT_KEYBINDINGS.auto_search,
    DEFAULT_KEYBINDINGS.auto_search.desc,
  ),
  (DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc),
];

pub static MANUAL_MOVIE_SEARCH_CONTEXT_CLUES: [ContextClue; 7] = [
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
  (DEFAULT_KEYBINDINGS.update, DEFAULT_KEYBINDINGS.update.desc),
  (DEFAULT_KEYBINDINGS.edit, DEFAULT_KEYBINDINGS.edit.desc),
  (DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc),
  (
    DEFAULT_KEYBINDINGS.auto_search,
    DEFAULT_KEYBINDINGS.auto_search.desc,
  ),
  (DEFAULT_KEYBINDINGS.submit, "details"),
  (DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc),
];

pub static ADD_MOVIE_SEARCH_RESULTS_CONTEXT_CLUES: [ContextClue; 2] = [
  (DEFAULT_KEYBINDINGS.submit, "details"),
  (DEFAULT_KEYBINDINGS.esc, "edit search"),
];

pub static SYSTEM_TASKS_CONTEXT_CLUES: [ContextClue; 2] = [
  (DEFAULT_KEYBINDINGS.submit, "start task"),
  (DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc),
];

pub static COLLECTION_DETAILS_CONTEXT_CLUES: [ContextClue; 3] = [
  (DEFAULT_KEYBINDINGS.submit, "show overview/add movie"),
  (DEFAULT_KEYBINDINGS.edit, "edit collection"),
  (DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc),
];

pub(in crate::app) struct RadarrContextClueProvider;

impl ContextClueProvider for RadarrContextClueProvider {
  fn get_context_clues(app: &mut App<'_>) -> Option<&'static [ContextClue]> {
    let Route::Radarr(active_radarr_block, context_option) = app.get_current_route() else {
      panic!("RadarrContextClueProvider::get_context_clues called with non-Radarr route");
    };
    match active_radarr_block {
      _ if MOVIE_DETAILS_BLOCKS.contains(&active_radarr_block) => app
        .data
        .radarr_data
        .movie_info_tabs
        .get_active_route_contextual_help(),
      ActiveRadarrBlock::TestAllIndexers
      | ActiveRadarrBlock::AddMovieSearchInput
      | ActiveRadarrBlock::AddMovieEmptySearchResults
      | ActiveRadarrBlock::SystemLogs
      | ActiveRadarrBlock::SystemUpdates => Some(&BARE_POPUP_CONTEXT_CLUES),
      _ if context_option.unwrap_or(active_radarr_block)
        == ActiveRadarrBlock::ViewMovieOverview =>
      {
        Some(&BARE_POPUP_CONTEXT_CLUES)
      }
      ActiveRadarrBlock::SystemTasks => Some(&SYSTEM_TASKS_CONTEXT_CLUES),
      _ if EDIT_COLLECTION_BLOCKS.contains(&active_radarr_block)
        || EDIT_INDEXER_BLOCKS.contains(&active_radarr_block)
        || INDEXER_SETTINGS_BLOCKS.contains(&active_radarr_block)
        || EDIT_MOVIE_BLOCKS.contains(&active_radarr_block) =>
      {
        Some(&CONFIRMATION_PROMPT_CONTEXT_CLUES)
      }
      ActiveRadarrBlock::AddMoviePrompt
      | ActiveRadarrBlock::AddMovieSelectMonitor
      | ActiveRadarrBlock::AddMovieSelectMinimumAvailability
      | ActiveRadarrBlock::AddMovieSelectQualityProfile
      | ActiveRadarrBlock::AddMovieSelectRootFolder
      | ActiveRadarrBlock::AddMovieTagsInput
      | ActiveRadarrBlock::SystemTaskStartConfirmPrompt => Some(&CONFIRMATION_PROMPT_CONTEXT_CLUES),
      _ if ADD_MOVIE_BLOCKS.contains(&active_radarr_block) => {
        Some(&ADD_MOVIE_SEARCH_RESULTS_CONTEXT_CLUES)
      }
      ActiveRadarrBlock::CollectionDetails => Some(&COLLECTION_DETAILS_CONTEXT_CLUES),
      _ => app
        .data
        .radarr_data
        .main_tabs
        .get_active_route_contextual_help(),
    }
  }
}
