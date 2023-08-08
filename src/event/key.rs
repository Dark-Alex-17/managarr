use std::fmt;
use std::fmt::{Display, Formatter};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[cfg(test)]
#[path = "key_tests.rs"]
mod key_tests;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
  Tab,
  Delete,
  Ctrl(char),
  Char(char),
  Unknown,
}

impl Display for Key {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match *self {
      Key::Char(c) => write!(f, "<{}>", c),
      Key::Ctrl(c) => write!(f, "<ctrl-{}>", c),
      Key::Up => write!(f, "<↑>"),
      Key::Down => write!(f, "<↓>"),
      Key::Left => write!(f, "<←>"),
      Key::Right => write!(f, "<→>"),
      Key::Enter => write!(f, "<enter>"),
      Key::Esc => write!(f, "<esc>"),
      Key::Backspace => write!(f, "<backspace>"),
      Key::Home => write!(f, "<home>"),
      Key::End => write!(f, "<end>"),
      Key::Tab => write!(f, "<tab>"),
      Key::Delete => write!(f, "<del>"),
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
        code: KeyCode::Tab, ..
      } => Key::Tab,
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
        modifiers: KeyModifiers::CONTROL,
        ..
      } => Key::Ctrl(c),
      KeyEvent {
        code: KeyCode::Char(c),
        ..
      } => Key::Char(c),
      _ => Key::Unknown,
    }
  }
}
