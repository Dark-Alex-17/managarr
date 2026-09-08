use crate::app::App;
use crate::app::context_clues::{
  BARE_POPUP_CONTEXT_CLUES, CONFIRMATION_PROMPT_CONTEXT_CLUES, ContextClue, ContextClueProvider,
  SYSTEM_TASKS_CONTEXT_CLUES,
};
use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::models::Route;
use crate::models::servarr_data::readarr::readarr_data::{
  ADD_AUTHOR_BLOCKS, ADD_ROOT_FOLDER_BLOCKS, AUTHOR_DETAILS_BLOCKS, ActiveReadarrBlock,
  BOOK_DETAILS_BLOCKS, DELETE_AUTHOR_BLOCKS, DELETE_BOOK_BLOCKS, EDIT_AUTHOR_BLOCKS,
  EDIT_INDEXER_BLOCKS, EDITION_DETAILS_BLOCKS, INDEXER_SETTINGS_BLOCKS,
};

#[cfg(test)]
#[path = "readarr_context_clues_tests.rs"]
mod readarr_context_clues_tests;

pub static AUTHORS_CONTEXT_CLUES: [ContextClue; 10] = [
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

pub static ADD_AUTHOR_SEARCH_RESULTS_CONTEXT_CLUES: [ContextClue; 2] = [
  (DEFAULT_KEYBINDINGS.submit, "details"),
  (DEFAULT_KEYBINDINGS.esc, "edit search"),
];

pub static AUTHOR_DETAILS_CONTEXT_CLUES: [ContextClue; 8] = [
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
  (DEFAULT_KEYBINDINGS.edit, "edit author"),
  (DEFAULT_KEYBINDINGS.delete, "delete book"),
  (
    DEFAULT_KEYBINDINGS.toggle_monitoring,
    "toggle book monitoring",
  ),
  (DEFAULT_KEYBINDINGS.search, DEFAULT_KEYBINDINGS.search.desc),
  (DEFAULT_KEYBINDINGS.update, DEFAULT_KEYBINDINGS.update.desc),
  (
    DEFAULT_KEYBINDINGS.auto_search,
    DEFAULT_KEYBINDINGS.auto_search.desc,
  ),
  (DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc),
];

pub static AUTHOR_HISTORY_CONTEXT_CLUES: [ContextClue; 9] = [
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
  (DEFAULT_KEYBINDINGS.edit, "edit author"),
  (DEFAULT_KEYBINDINGS.submit, "details"),
  (DEFAULT_KEYBINDINGS.update, DEFAULT_KEYBINDINGS.update.desc),
  (DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc),
  (DEFAULT_KEYBINDINGS.search, DEFAULT_KEYBINDINGS.search.desc),
  (DEFAULT_KEYBINDINGS.filter, DEFAULT_KEYBINDINGS.filter.desc),
  (
    DEFAULT_KEYBINDINGS.auto_search,
    DEFAULT_KEYBINDINGS.auto_search.desc,
  ),
  (DEFAULT_KEYBINDINGS.esc, "cancel filter/close"),
];

pub static MANUAL_AUTHOR_SEARCH_CONTEXT_CLUES: [ContextClue; 7] = [
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
  (DEFAULT_KEYBINDINGS.edit, "edit author"),
  (DEFAULT_KEYBINDINGS.submit, "details"),
  (DEFAULT_KEYBINDINGS.update, DEFAULT_KEYBINDINGS.update.desc),
  (
    DEFAULT_KEYBINDINGS.auto_search,
    DEFAULT_KEYBINDINGS.auto_search.desc,
  ),
  (DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc),
  (DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc),
];

pub static BOOK_DETAILS_CONTEXT_CLUES: [ContextClue; 5] = [
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
  (DEFAULT_KEYBINDINGS.search, DEFAULT_KEYBINDINGS.search.desc),
  (
    DEFAULT_KEYBINDINGS.auto_search,
    DEFAULT_KEYBINDINGS.auto_search.desc,
  ),
  (DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc),
  (DEFAULT_KEYBINDINGS.submit, "edition details"),
];

pub static BOOK_HISTORY_CONTEXT_CLUES: [ContextClue; 7] = [
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
  (DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc),
  (DEFAULT_KEYBINDINGS.search, DEFAULT_KEYBINDINGS.search.desc),
  (DEFAULT_KEYBINDINGS.filter, DEFAULT_KEYBINDINGS.filter.desc),
  (
    DEFAULT_KEYBINDINGS.auto_search,
    DEFAULT_KEYBINDINGS.auto_search.desc,
  ),
  (DEFAULT_KEYBINDINGS.submit, "details"),
  (DEFAULT_KEYBINDINGS.esc, "cancel filter/close"),
];

pub static BOOK_FILE_CONTEXT_CLUES: [ContextClue; 3] = [
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
  (DEFAULT_KEYBINDINGS.delete, "delete book file"),
  (DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc),
];

pub static MANUAL_BOOK_SEARCH_CONTEXT_CLUES: [ContextClue; 5] = [
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
  (
    DEFAULT_KEYBINDINGS.auto_search,
    DEFAULT_KEYBINDINGS.auto_search.desc,
  ),
  (DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc),
  (DEFAULT_KEYBINDINGS.submit, "details"),
  (DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc),
];

pub static EDITION_DETAILS_CONTEXT_CLUES: [ContextClue; 2] = [
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
  (DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc),
];

pub(in crate::app) struct ReadarrContextClueProvider;

impl ContextClueProvider for ReadarrContextClueProvider {
  fn get_context_clues(app: &mut App<'_>) -> Option<&'static [ContextClue]> {
    let Route::Readarr(active_readarr_block, _context_option) = app.get_current_route() else {
      panic!("ReadarrContextClueProvider::get_context_clues called with non-Readarr route");
    };

    match active_readarr_block {
      _ if AUTHOR_DETAILS_BLOCKS.contains(&active_readarr_block) => app
        .data
        .readarr_data
        .author_info_tabs
        .get_active_route_contextual_help(),
      _ if BOOK_DETAILS_BLOCKS.contains(&active_readarr_block) => app
        .data
        .readarr_data
        .book_details_modal
        .as_ref()
        .expect("book_details_modal is empty")
        .book_details_tabs
        .get_active_route_contextual_help(),
      _ if EDITION_DETAILS_BLOCKS.contains(&active_readarr_block) => {
        Some(&EDITION_DETAILS_CONTEXT_CLUES)
      }
      ActiveReadarrBlock::AddAuthorSearchInput
      | ActiveReadarrBlock::AddAuthorEmptySearchResults
      | ActiveReadarrBlock::TestAllIndexers
      | ActiveReadarrBlock::SystemLogs
      | ActiveReadarrBlock::SystemUpdates => Some(&BARE_POPUP_CONTEXT_CLUES),
      _ if EDIT_AUTHOR_BLOCKS.contains(&active_readarr_block)
        || EDIT_INDEXER_BLOCKS.contains(&active_readarr_block)
        || INDEXER_SETTINGS_BLOCKS.contains(&active_readarr_block)
        || ADD_ROOT_FOLDER_BLOCKS.contains(&active_readarr_block) =>
      {
        Some(&CONFIRMATION_PROMPT_CONTEXT_CLUES)
      }
      _ if DELETE_AUTHOR_BLOCKS.contains(&active_readarr_block)
        || DELETE_BOOK_BLOCKS.contains(&active_readarr_block) =>
      {
        Some(&CONFIRMATION_PROMPT_CONTEXT_CLUES)
      }
      ActiveReadarrBlock::AddAuthorPrompt
      | ActiveReadarrBlock::AddAuthorSelectMonitor
      | ActiveReadarrBlock::AddAuthorSelectMonitorNewItems
      | ActiveReadarrBlock::AddAuthorSelectQualityProfile
      | ActiveReadarrBlock::AddAuthorSelectMetadataProfile
      | ActiveReadarrBlock::AddAuthorSelectRootFolder
      | ActiveReadarrBlock::AddAuthorTagsInput
      | ActiveReadarrBlock::AddAuthorAlreadyInLibrary => Some(&CONFIRMATION_PROMPT_CONTEXT_CLUES),
      _ if ADD_AUTHOR_BLOCKS.contains(&active_readarr_block) => {
        Some(&ADD_AUTHOR_SEARCH_RESULTS_CONTEXT_CLUES)
      }
      ActiveReadarrBlock::SystemTasks => Some(&SYSTEM_TASKS_CONTEXT_CLUES),
      _ => app
        .data
        .readarr_data
        .main_tabs
        .get_active_route_contextual_help(),
    }
  }
}
