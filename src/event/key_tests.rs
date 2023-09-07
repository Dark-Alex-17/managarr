#[cfg(test)]
mod tests {
  use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;

  use crate::event::key::Key;

  #[rstest]
  #[case(Key::Up, "↑")]
  #[case(Key::Down, "↓")]
  #[case(Key::Left, "←")]
  #[case(Key::Right, "→")]
  #[case(Key::Enter, "enter")]
  #[case(Key::Esc, "esc")]
  #[case(Key::Backspace, "backspace")]
  #[case(Key::Home, "home")]
  #[case(Key::End, "end")]
  #[case(Key::Tab, "tab")]
  #[case(Key::Delete, "del")]
  #[case(Key::Char('q'), "q")]
  #[case(Key::Ctrl('q'), "ctrl-q")]
  fn test_key_formatter(#[case] key: Key, #[case] expected_str: &str) {
    assert_str_eq!(format!("{key}"), format!("<{expected_str}>"));
  }

  #[test]
  fn test_key_from_up() {
    assert_eq!(Key::from(KeyEvent::from(KeyCode::Up)), Key::Up);
  }

  #[test]
  fn test_key_from_down() {
    assert_eq!(Key::from(KeyEvent::from(KeyCode::Down)), Key::Down);
  }

  #[test]
  fn test_key_from_left() {
    assert_eq!(Key::from(KeyEvent::from(KeyCode::Left)), Key::Left);
  }

  #[test]
  fn test_key_from_right() {
    assert_eq!(Key::from(KeyEvent::from(KeyCode::Right)), Key::Right);
  }

  #[test]
  fn test_key_from_backspace() {
    assert_eq!(
      Key::from(KeyEvent::from(KeyCode::Backspace)),
      Key::Backspace
    );
  }

  #[test]
  fn test_key_from_home() {
    assert_eq!(Key::from(KeyEvent::from(KeyCode::Home)), Key::Home);
  }

  #[test]
  fn test_key_from_end() {
    assert_eq!(Key::from(KeyEvent::from(KeyCode::End)), Key::End);
  }

  #[test]
  fn test_key_from_tab() {
    assert_eq!(Key::from(KeyEvent::from(KeyCode::Tab)), Key::Tab);
  }

  #[test]
  fn test_key_from_delete() {
    assert_eq!(Key::from(KeyEvent::from(KeyCode::Delete)), Key::Delete);
  }

  #[test]
  fn test_key_from_enter() {
    assert_eq!(Key::from(KeyEvent::from(KeyCode::Enter)), Key::Enter);
  }

  #[test]
  fn test_key_from_esc() {
    assert_eq!(Key::from(KeyEvent::from(KeyCode::Esc)), Key::Esc);
  }

  #[test]
  fn test_key_from_char() {
    assert_eq!(
      Key::from(KeyEvent::from(KeyCode::Char('q'))),
      Key::Char('q')
    )
  }

  #[test]
  fn test_key_from_ctrl() {
    assert_eq!(
      Key::from(KeyEvent {
        code: KeyCode::Char('c'),
        modifiers: KeyModifiers::CONTROL,
        kind: KeyEventKind::Press,
        state: KeyEventState::NONE
      }),
      Key::Ctrl('c')
    );
  }

  #[test]
  fn test_key_from_unknown() {
    assert_eq!(Key::from(KeyEvent::from(KeyCode::Pause)), Key::Unknown);
  }
}
