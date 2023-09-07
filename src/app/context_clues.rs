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
  (DEFAULT_KEYBINDINGS.tab, "change servarr"),
  (DEFAULT_KEYBINDINGS.quit, DEFAULT_KEYBINDINGS.quit.desc),
];

pub static BARE_POPUP_CONTEXT_CLUES: [ContextClue; 1] =
  [(DEFAULT_KEYBINDINGS.esc, DEFAULT_KEYBINDINGS.esc.desc)];
