use crate::app::context_clues::ContextClue;
use crate::app::key_binding::DEFAULT_KEYBINDINGS;

#[cfg(test)]
#[path = "radarr_context_clues_tests.rs"]
mod radarr_context_clues_tests;

pub static LIBRARY_CONTEXT_CLUES: [ContextClue; 10] = [
  (DEFAULT_KEYBINDINGS.add, DEFAULT_KEYBINDINGS.add.desc),
  (DEFAULT_KEYBINDINGS.edit, DEFAULT_KEYBINDINGS.edit.desc),
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

pub static MANUAL_MOVIE_SEARCH_CONTEXT_CLUES: [ContextClue; 6] = [
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
  (DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc),
];

pub static MANUAL_MOVIE_SEARCH_CONTEXTUAL_CONTEXT_CLUES: [ContextClue; 1] =
  [(DEFAULT_KEYBINDINGS.submit, "details")];

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
