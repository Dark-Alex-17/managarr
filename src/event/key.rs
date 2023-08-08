use std::fmt;
use std::fmt::{Display, Formatter};

use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug, PartialEq, Eq)]
pub enum Key {
  Up,
  Down,
  Left,
  Right,
  Enter,
  Esc,
  Backspace,
  Home,
  End,
  Delete,
  Char(char),
  Unknown,
}

impl Display for Key {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match *self {
      Key::Char(c) => write!(f, "<{}>", c),
      _ => write!(f, "<{:?}>", self),
    }
  }
}

impl From<KeyEvent> for Key {
  fn from(key_event: KeyEvent) -> Self {
    match key_event {
      KeyEvent {
        code: KeyCode::Up, ..
      } => Key::Up,
      KeyEvent {
        code: KeyCode::Down,
        ..
      } => Key::Down,
      KeyEvent {
        code: KeyCode::Left,
        ..
      } => Key::Left,
      KeyEvent {
        code: KeyCode::Right,
        ..
      } => Key::Right,
      KeyEvent {
        code: KeyCode::Backspace,
        ..
      } => Key::Backspace,
      KeyEvent {
        code: KeyCode::Home,
        ..
      } => Key::Home,
      KeyEvent {
        code: KeyCode::End, ..
      } => Key::End,
      KeyEvent {
        code: KeyCode::Delete,
        ..
      } => Key::Delete,
      KeyEvent {
        code: KeyCode::Enter,
        ..
      } => Key::Enter,
      KeyEvent {
        code: KeyCode::Esc, ..
      } => Key::Esc,
      KeyEvent {
        code: KeyCode::Char(c),
        ..
      } => Key::Char(c),
      _ => Key::Unknown,
    }
  }
}

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
