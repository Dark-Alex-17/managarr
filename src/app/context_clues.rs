use crate::app::key_binding::{KeyBinding, DEFAULT_KEYBINDINGS};

#[cfg(test)]
#[path = "context_clues_tests.rs"]
mod context_clues_tests;

pub(in crate::app) type ContextClue = (KeyBinding, &'static str);

pub fn build_context_clue_string(context_clues: &[(KeyBinding, &str)]) -> String {
  context_clues
    .iter()
    .map(|(key_binding, desc)| format!("{} {desc}", key_binding.key))
    .collect::<Vec<String>>()
    .join(" | ")
}

pub static SERVARR_CONTEXT_CLUES: [ContextClue; 2] = [
  (
    DEFAULT_KEYBINDINGS.next_servarr,
    DEFAULT_KEYBINDINGS.next_servarr.desc,
  ),
  (DEFAULT_KEYBINDINGS.quit, DEFAULT_KEYBINDINGS.quit.desc),
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
