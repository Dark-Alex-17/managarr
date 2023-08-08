#[cfg(test)]
mod tests {
  use crossterm::event::{KeyCode, KeyEvent};
  use pretty_assertions::{assert_eq, assert_str_eq};

  use crate::event::key::Key;

  #[test]
  fn test_key_formatter() {
    assert_str_eq!(format!("{}", Key::Esc), "<Esc>");
  }

  #[test]
  fn test_key_formatter_char() {
    assert_str_eq!(format!("{}", Key::Char('q')), "<q>");
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
  fn test_key_from_unknown() {
    assert_eq!(Key::from(KeyEvent::from(KeyCode::Pause)), Key::Unknown);
  }
}
