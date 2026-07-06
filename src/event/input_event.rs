use anyhow::Result;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::{Duration, Instant};

use crossterm::event;
use crossterm::event::{Event as CrosstermEvent, KeyEventKind};

use crate::event::Key;

pub enum InputEvent<T> {
  KeyEvent(T),
  Tick,
}

pub struct Events {
  rx: Receiver<InputEvent<Key>>,
}

const DEFAULT_TICK_RATE_MS: u64 = 50;

impl Events {
  pub fn new() -> Self {
    let (tx, rx) = mpsc::channel();
    let tick_rate: Duration = Duration::from_millis(DEFAULT_TICK_RATE_MS);

    thread::spawn(move || {
      let mut last_tick = Instant::now();
      loop {
        let timeout = tick_rate
          .checked_sub(last_tick.elapsed())
          .unwrap_or_else(|| Duration::from_secs(0));
        if event::poll(timeout).unwrap()
          && let CrosstermEvent::Key(key_event) = event::read().unwrap()
        {
          // Only process the key event if it's a press event
          // Source: https://ratatui.rs/faq/ Why am I getting duplicate key events on Windows?
          if key_event.kind == KeyEventKind::Press {
            let key = Key::from(key_event);
            tx.send(InputEvent::KeyEvent(key)).unwrap();
          }
        }

        if last_tick.elapsed() >= tick_rate {
          tx.send(InputEvent::Tick).unwrap();
          last_tick = Instant::now();
        }
      }
    });

    Events { rx }
  }

  pub fn next(&self) -> Result<Option<InputEvent<Key>>> {
    match self.rx.recv() {
      Ok(event) => Ok(Some(event)),
      _ => Ok(None),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn defaults_to_original_tick_rate() {
    assert_eq!(DEFAULT_TICK_RATE_MS, 50);
  }
}
