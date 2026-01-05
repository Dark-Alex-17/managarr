use crate::app::App;
use crate::app::key_binding::{DEFAULT_KEYBINDINGS, KeyBinding};
use crate::app::lidarr::lidarr_context_clues::LidarrContextClueProvider;
use crate::app::radarr::radarr_context_clues::RadarrContextClueProvider;
use crate::app::sonarr::sonarr_context_clues::SonarrContextClueProvider;
use crate::models::Route;

#[cfg(test)]
#[path = "context_clues_tests.rs"]
mod context_clues_tests;

pub type ContextClue = (KeyBinding, &'static str);

pub trait ContextClueProvider {
  fn get_context_clues(_app: &mut App<'_>) -> Option<&'static [ContextClue]>;
}

pub struct ServarrContextClueProvider;

impl ContextClueProvider for ServarrContextClueProvider {
  fn get_context_clues(app: &mut App<'_>) -> Option<&'static [ContextClue]> {
    match app.get_current_route() {
      Route::Radarr(_, _) => RadarrContextClueProvider::get_context_clues(app),
      Route::Sonarr(_, _) => SonarrContextClueProvider::get_context_clues(app),
      Route::Lidarr(_, _) => LidarrContextClueProvider::get_context_clues(app),
      _ => None,
    }
  }
}

pub static SERVARR_CONTEXT_CLUES: [ContextClue; 10] = [
  (DEFAULT_KEYBINDINGS.up, "scroll up"),
  (DEFAULT_KEYBINDINGS.down, "scroll down"),
  (DEFAULT_KEYBINDINGS.left, "previous tab"),
  (DEFAULT_KEYBINDINGS.right, "next tab"),
  (DEFAULT_KEYBINDINGS.pg_up, DEFAULT_KEYBINDINGS.pg_up.desc),
  (
    DEFAULT_KEYBINDINGS.pg_down,
    DEFAULT_KEYBINDINGS.pg_down.desc,
  ),
  (
    DEFAULT_KEYBINDINGS.next_servarr,
    DEFAULT_KEYBINDINGS.next_servarr.desc,
  ),
  (
    DEFAULT_KEYBINDINGS.previous_servarr,
    DEFAULT_KEYBINDINGS.previous_servarr.desc,
  ),
  (DEFAULT_KEYBINDINGS.quit, DEFAULT_KEYBINDINGS.quit.desc),
  (DEFAULT_KEYBINDINGS.help, DEFAULT_KEYBINDINGS.help.desc),
];

pub static BARE_POPUP_CONTEXT_CLUES: [ContextClue; 1] =
  [(DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc)];

pub static BLOCKLIST_CONTEXT_CLUES: [ContextClue; 5] = [
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
  (DEFAULT_KEYBINDINGS.sort, DEFAULT_KEYBINDINGS.sort.desc),
  (DEFAULT_KEYBINDINGS.submit, "details"),
  (DEFAULT_KEYBINDINGS.delete, DEFAULT_KEYBINDINGS.delete.desc),
  (DEFAULT_KEYBINDINGS.clear, "clear blocklist"),
];

pub static CONFIRMATION_PROMPT_CONTEXT_CLUES: [ContextClue; 2] = [
  (DEFAULT_KEYBINDINGS.confirm, "submit"),
  (DEFAULT_KEYBINDINGS.esc, "cancel"),
];

pub static DOWNLOADS_CONTEXT_CLUES: [ContextClue; 3] = [
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
  (DEFAULT_KEYBINDINGS.delete, DEFAULT_KEYBINDINGS.delete.desc),
  (DEFAULT_KEYBINDINGS.update, "update downloads"),
];

pub static ROOT_FOLDERS_CONTEXT_CLUES: [ContextClue; 3] = [
  (DEFAULT_KEYBINDINGS.add, DEFAULT_KEYBINDINGS.add.desc),
  (DEFAULT_KEYBINDINGS.delete, DEFAULT_KEYBINDINGS.delete.desc),
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
];

pub static INDEXERS_CONTEXT_CLUES: [ContextClue; 6] = [
  (DEFAULT_KEYBINDINGS.submit, "edit indexer"),
  (
    DEFAULT_KEYBINDINGS.settings,
    DEFAULT_KEYBINDINGS.settings.desc,
  ),
  (DEFAULT_KEYBINDINGS.delete, DEFAULT_KEYBINDINGS.delete.desc),
  (DEFAULT_KEYBINDINGS.test, "test indexer"),
  (DEFAULT_KEYBINDINGS.test_all, "test all indexers"),
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
];

pub static SYSTEM_CONTEXT_CLUES: [ContextClue; 5] = [
  (DEFAULT_KEYBINDINGS.tasks, "open tasks"),
  (DEFAULT_KEYBINDINGS.events, "open events"),
  (DEFAULT_KEYBINDINGS.logs, "open logs"),
  (DEFAULT_KEYBINDINGS.update, "open updates"),
  (
    DEFAULT_KEYBINDINGS.refresh,
    DEFAULT_KEYBINDINGS.refresh.desc,
  ),
];
