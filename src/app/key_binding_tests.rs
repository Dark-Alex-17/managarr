#[cfg(test)]
mod test {
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;

  use crate::app::key_binding::{KeyBinding, DEFAULT_KEYBINDINGS};
  use crate::event::Key;
  use crate::matches_key;

  #[rstest]
  #[case(DEFAULT_KEYBINDINGS.add, Key::Char('a'), None, "add")]
  #[case(DEFAULT_KEYBINDINGS.up, Key::Up, Some(Key::Char('k')), "up")]
  #[case(DEFAULT_KEYBINDINGS.down, Key::Down, Some(Key::Char('j')), "down")]
  #[case(DEFAULT_KEYBINDINGS.left, Key::Left, Some(Key::Char('h')), "left")]
  #[case(DEFAULT_KEYBINDINGS.right, Key::Right, Some(Key::Char('l')), "right")]
  #[case(DEFAULT_KEYBINDINGS.pg_down, Key::PgDown, Some(Key::Ctrl('d')), "page down")]
  #[case(DEFAULT_KEYBINDINGS.pg_up, Key::PgUp, Some(Key::Ctrl('u')), "page up")]
  #[case(DEFAULT_KEYBINDINGS.backspace, Key::Backspace, Some(Key::Ctrl('h')), "backspace")]
  #[case(DEFAULT_KEYBINDINGS.next_servarr, Key::Tab, None, "next servarr")]
  #[case(DEFAULT_KEYBINDINGS.previous_servarr, Key::BackTab, None, "previous servarr")]
  #[case(DEFAULT_KEYBINDINGS.clear, Key::Char('c'), None, "clear")]
  #[case(DEFAULT_KEYBINDINGS.auto_search, Key::Char('S'), None, "auto search")]
  #[case(DEFAULT_KEYBINDINGS.search, Key::Char('s'), None, "search")]
  #[case(DEFAULT_KEYBINDINGS.settings, Key::Char('S'), None, "settings")]
  #[case(DEFAULT_KEYBINDINGS.filter, Key::Char('f'), None, "filter")]
  #[case(DEFAULT_KEYBINDINGS.sort, Key::Char('o'), None, "sort")]
  #[case(DEFAULT_KEYBINDINGS.edit, Key::Char('e'), None, "edit")]
  #[case(DEFAULT_KEYBINDINGS.events, Key::Char('e'), None, "events")]
  #[case(DEFAULT_KEYBINDINGS.logs, Key::Char('L'), None, "logs")]
  #[case(DEFAULT_KEYBINDINGS.tasks, Key::Char('t'), None, "tasks")]
  #[case(DEFAULT_KEYBINDINGS.test, Key::Char('t'), None, "test")]
  #[case(DEFAULT_KEYBINDINGS.test_all, Key::Char('T'), None, "test all")]
  #[case(DEFAULT_KEYBINDINGS.toggle_monitoring, Key::Char('m'), None, "toggle monitoring")]
  #[case(DEFAULT_KEYBINDINGS.refresh, Key::Ctrl('r'), None, "refresh")]
  #[case(DEFAULT_KEYBINDINGS.update, Key::Char('u'), None, "update")]
  #[case(DEFAULT_KEYBINDINGS.home, Key::Home, None, "home")]
  #[case(DEFAULT_KEYBINDINGS.end, Key::End, None, "end")]
  #[case(DEFAULT_KEYBINDINGS.delete, Key::Delete, None, "delete")]
  #[case(DEFAULT_KEYBINDINGS.submit, Key::Enter, None, "submit")]
  #[case(DEFAULT_KEYBINDINGS.confirm, Key::Ctrl('s'), None, "submit")]
  #[case(DEFAULT_KEYBINDINGS.quit, Key::Char('q'), None, "quit")]
  #[case(DEFAULT_KEYBINDINGS.esc, Key::Esc, None, "close")]
  fn test_default_key_bindings_and_descriptions(
    #[case] key_binding: KeyBinding,
    #[case] expected_key: Key,
    #[case] expected_alt_key: Option<Key>,
    #[case] expected_desc: &str,
  ) {
    assert_eq!(key_binding.key, expected_key);
    assert_eq!(key_binding.alt, expected_alt_key);
    assert_str_eq!(key_binding.desc, expected_desc);
  }

  #[test]
  fn test_matches_key_macro() {
    let key = Key::Char('t');

    assert!(matches_key!(test, key));
    assert!(!matches_key!(test, Key::Char('T')));
  }

  #[test]
  fn test_matches_key_macro_with_alt_keybinding() {
    let alt_key = Key::Char('k');
    let key = Key::Up;

    assert!(matches_key!(up, key));
    assert!(matches_key!(up, alt_key));
    assert!(!matches_key!(up, Key::Char('t')));
  }

  #[test]
  fn test_matches_key_macro_with_alt_keybinding_uses_alt_key_when_ignore_special_keys_is_false() {
    let alt_key = Key::Char('k');
    let key = Key::Up;

    assert!(matches_key!(up, key, false));
    assert!(matches_key!(up, alt_key, false));
    assert!(!matches_key!(up, Key::Char('t'), false));
  }

  #[test]
  fn test_matches_key_macro_with_alt_keybinding_ignores_alt_key_when_ignore_special_keys_is_true() {
    let alt_key = Key::Char('k');
    let key = Key::Up;

    assert!(matches_key!(up, key, true));
    assert!(!matches_key!(up, alt_key, true));
    assert!(!matches_key!(up, Key::Char('t'), true));
  }
}
