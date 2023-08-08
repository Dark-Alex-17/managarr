use std::fmt;
use std::fmt::{Display, Formatter};

use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Unknown
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Key::Char(c) => write!(f, "<{}>", c),
            _ => write!(f, "<{:?}>", self)
        }
    }
}

impl From<KeyEvent> for Key {
    fn from(key_event: KeyEvent) -> Self {
        match key_event {
            KeyEvent {
                code: KeyCode::Char(c),
                ..
            } => Key::Char(c),
            _ => Key::Unknown
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
        assert_eq!(Key::from(KeyEvent::from(KeyCode::Char('q'))), Key::Char('q'))
    }
}