use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

use crossterm::event;
use crossterm::event::Event as CrosstermEvent;

use crate::event::Key;

pub enum InputEvent<T> {
  KeyEvent(T),
  Tick,
}

pub struct Events {
  _tx: Sender<InputEvent<Key>>,
  rx: Receiver<InputEvent<Key>>,
}

impl Events {
  pub fn new() -> Self {
    let (tx, rx) = mpsc::channel();
    let tick_rate: Duration = Duration::from_millis(250);

    let event_tx = tx.clone();
    thread::spawn(move || {
      let mut last_tick = Instant::now();
      loop {
        let timeout = tick_rate
          .checked_sub(last_tick.elapsed())
          .unwrap_or_else(|| Duration::from_secs(0));
        if event::poll(timeout).unwrap() {
          if let CrosstermEvent::Key(key_event) = event::read().unwrap() {
            let key = Key::from(key_event);
            event_tx.send(InputEvent::KeyEvent(key)).unwrap();
          }
        }

        if last_tick.elapsed() >= tick_rate {
          event_tx.send(InputEvent::Tick).unwrap();
          last_tick = Instant::now();
        }
      }
    });

    Events { _tx: tx, rx }
  }

  pub fn next(&self) -> Result<InputEvent<Key>, mpsc::RecvError> {
    self.rx.recv()
  }
}
