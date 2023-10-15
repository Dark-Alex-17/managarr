#[cfg(test)]
mod test {
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;

  use crate::app::key_binding::{KeyBinding, DEFAULT_KEYBINDINGS};
  use crate::event::Key;

  #[rstest]
  #[case(DEFAULT_KEYBINDINGS.add, Key::Char('a'), "add")]
  #[case(DEFAULT_KEYBINDINGS.up, Key::Up, "up")]
  #[case(DEFAULT_KEYBINDINGS.down, Key::Down, "down")]
  #[case(DEFAULT_KEYBINDINGS.left, Key::Left, "left")]
  #[case(DEFAULT_KEYBINDINGS.right, Key::Right, "right")]
  #[case(DEFAULT_KEYBINDINGS.backspace, Key::Backspace, "backspace")]
  #[case(DEFAULT_KEYBINDINGS.search, Key::Char('s'), "search")]
  #[case(DEFAULT_KEYBINDINGS.settings, Key::Char('s'), "settings")]
  #[case(DEFAULT_KEYBINDINGS.filter, Key::Char('f'), "filter")]
  #[case(DEFAULT_KEYBINDINGS.sort, Key::Char('o'), "sort")]
  #[case(DEFAULT_KEYBINDINGS.edit, Key::Char('e'), "edit")]
  #[case(DEFAULT_KEYBINDINGS.events, Key::Char('e'), "events")]
  #[case(DEFAULT_KEYBINDINGS.logs, Key::Char('l'), "logs")]
  #[case(DEFAULT_KEYBINDINGS.tasks, Key::Char('t'), "tasks")]
  #[case(DEFAULT_KEYBINDINGS.refresh, Key::Ctrl('r'), "refresh")]
  #[case(DEFAULT_KEYBINDINGS.update, Key::Char('u'), "update")]
  #[case(DEFAULT_KEYBINDINGS.home, Key::Home, "home")]
  #[case(DEFAULT_KEYBINDINGS.end, Key::End, "end")]
  #[case(DEFAULT_KEYBINDINGS.tab, Key::Tab, "tab")]
  #[case(DEFAULT_KEYBINDINGS.delete, Key::Delete, "delete")]
  #[case(DEFAULT_KEYBINDINGS.submit, Key::Enter, "submit")]
  #[case(DEFAULT_KEYBINDINGS.quit, Key::Char('q'), "quit")]
  #[case(DEFAULT_KEYBINDINGS.esc, Key::Esc, "close")]
  fn test_default_key_bindings_and_descriptions(
    #[case] key_binding: KeyBinding,
    #[case] expected_key: Key,
    #[case] expected_desc: &str,
  ) {
    assert_eq!(key_binding.key, expected_key);
    assert_str_eq!(key_binding.desc, expected_desc);
  }
}
