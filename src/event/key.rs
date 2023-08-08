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
        code: KeyCode::End,
        ..
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

  use crate::event::key::Key;

  #[test]
  fn test_key_formatter() {
    assert_eq!(format!("{}", Key::Char('q')), "<q>");
  }

  #[test]
  fn test_key_from() {
    assert_eq!(
      Key::from(KeyEvent::from(KeyCode::Char('q'))),
      Key::Char('q')
    )
  }
}
