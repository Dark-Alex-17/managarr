use crate::app::key_binding::{KeyMapping, DEFAULT_KEYBINDINGS};

pub static LIBRARY_KEY_MAPPINGS: [KeyMapping; 9] = [
  (DEFAULT_KEYBINDINGS.add, DEFAULT_KEYBINDINGS.add.desc),
  (DEFAULT_KEYBINDINGS.edit, DEFAULT_KEYBINDINGS.edit.desc),
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

pub static DOWNLOADS_KEY_MAPPINGS: [KeyMapping; 2] = [
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
  (DEFAULT_KEYBINDINGS.delete, DEFAULT_KEYBINDINGS.delete.desc),
];

pub static COLLECTIONS_KEY_MAPPINGS: [KeyMapping; 7] = [
  (DEFAULT_KEYBINDINGS.search, DEFAULT_KEYBINDINGS.search.desc),
  (DEFAULT_KEYBINDINGS.edit, DEFAULT_KEYBINDINGS.edit.desc),
  (DEFAULT_KEYBINDINGS.filter, DEFAULT_KEYBINDINGS.filter.desc),
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
  (DEFAULT_KEYBINDINGS.update, "update all"),
  (DEFAULT_KEYBINDINGS.submit, "details"),
  (DEFAULT_KEYBINDINGS.esc, "cancel filter"),
];

pub static ROOT_FOLDERS_KEY_MAPPINGS: [KeyMapping; 3] = [
  (DEFAULT_KEYBINDINGS.add, DEFAULT_KEYBINDINGS.add.desc),
  (DEFAULT_KEYBINDINGS.delete, DEFAULT_KEYBINDINGS.delete.desc),
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
];

pub static INDEXERS_KEY_MAPPINGS: [KeyMapping; 6] = [
  (DEFAULT_KEYBINDINGS.add, DEFAULT_KEYBINDINGS.add.desc),
  (DEFAULT_KEYBINDINGS.edit, DEFAULT_KEYBINDINGS.edit.desc),
  (
    DEFAULT_KEYBINDINGS.settings,
    DEFAULT_KEYBINDINGS.settings.desc,
  ),
  (
    DEFAULT_KEYBINDINGS.restrictions,
    DEFAULT_KEYBINDINGS.restrictions.desc,
  ),
  (DEFAULT_KEYBINDINGS.delete, DEFAULT_KEYBINDINGS.delete.desc),
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
];

pub static SYSTEM_KEY_MAPPINGS: [KeyMapping; 5] = [
  (DEFAULT_KEYBINDINGS.tasks, "open tasks"),
  (DEFAULT_KEYBINDINGS.events, "open events"),
  (DEFAULT_KEYBINDINGS.logs, "open logs"),
  (DEFAULT_KEYBINDINGS.update, "open updates"),
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
];

pub static MOVIE_DETAILS_KEY_MAPPINGS: [KeyMapping; 5] = [
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
  (DEFAULT_KEYBINDINGS.update, DEFAULT_KEYBINDINGS.update.desc),
  (DEFAULT_KEYBINDINGS.edit, DEFAULT_KEYBINDINGS.edit.desc),
  (DEFAULT_KEYBINDINGS.search, "auto search"),
  (DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc),
];

pub static MANUAL_MOVIE_SEARCH_KEY_MAPPINGS: [KeyMapping; 6] = [
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
  (DEFAULT_KEYBINDINGS.update, DEFAULT_KEYBINDINGS.update.desc),
  (DEFAULT_KEYBINDINGS.edit, DEFAULT_KEYBINDINGS.edit.desc),
  (DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc),
  (DEFAULT_KEYBINDINGS.search, "auto search"),
  (DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc),
];

pub static MANUAL_MOVIE_SEARCH_CONTEXTUAL_KEY_MAPPINGS: [KeyMapping; 1] =
  [(DEFAULT_KEYBINDINGS.submit, "details")];

pub static ADD_MOVIE_SEARCH_RESULTS_KEY_MAPPINGS: [KeyMapping; 2] = [
  (DEFAULT_KEYBINDINGS.submit, "details"),
  (DEFAULT_KEYBINDINGS.esc, "edit search"),
];

pub static SYSTEM_TASKS_KEY_MAPPINGS: [KeyMapping; 2] = [
  (DEFAULT_KEYBINDINGS.submit, "start task"),
  (DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc),
];

pub static COLLECTION_DETAILS_KEY_MAPPINGS: [KeyMapping; 2] = [
  (DEFAULT_KEYBINDINGS.submit, "show overview/add movie"),
  (DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc),
];
