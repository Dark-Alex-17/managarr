use radarr_handlers::RadarrHandler;

use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::App;
use crate::event::Key;
use crate::models::{HorizontallyScrollableText, Route};

mod radarr_handlers;

#[cfg(test)]
#[path = "handlers_tests.rs"]
mod handlers_tests;

#[cfg(test)]
#[path = "handler_test_utils.rs"]
pub mod handler_test_utils;

pub trait KeyEventHandler<'a, 'b, T: Into<Route>> {
  fn handle_key_event(&mut self) {
    let key = self.get_key();
    match key {
      _ if *key == DEFAULT_KEYBINDINGS.up.key => self.handle_scroll_up(),
      _ if *key == DEFAULT_KEYBINDINGS.down.key => self.handle_scroll_down(),
      _ if *key == DEFAULT_KEYBINDINGS.home.key => self.handle_home(),
      _ if *key == DEFAULT_KEYBINDINGS.end.key => self.handle_end(),
      _ if *key == DEFAULT_KEYBINDINGS.delete.key => self.handle_delete(),
      _ if *key == DEFAULT_KEYBINDINGS.left.key || *key == DEFAULT_KEYBINDINGS.right.key => {
        self.handle_left_right_action()
      }
      _ if *key == DEFAULT_KEYBINDINGS.submit.key => self.handle_submit(),
      _ if *key == DEFAULT_KEYBINDINGS.esc.key => self.handle_esc(),
      _ => self.handle_char_key_event(),
    }
  }

  fn handle(&mut self) {
    self.handle_key_event();
  }

  fn accepts(active_block: &'a T) -> bool;
  fn with(key: &'a Key, app: &'a mut App<'b>, active_block: &'a T, context: &'a Option<T>) -> Self;
  fn get_key(&self) -> &Key;
  fn handle_scroll_up(&mut self);
  fn handle_scroll_down(&mut self);
  fn handle_home(&mut self);
  fn handle_end(&mut self);
  fn handle_delete(&mut self);
  fn handle_left_right_action(&mut self);
  fn handle_submit(&mut self);
  fn handle_esc(&mut self);
  fn handle_char_key_event(&mut self);
}

pub fn handle_events(key: Key, app: &mut App<'_>) {
  if let Route::Radarr(active_radarr_block, context) = *app.get_current_route() {
    RadarrHandler::with(&key, app, &active_radarr_block, &context).handle()
  }
}

fn handle_clear_errors(app: &mut App<'_>) {
  if !app.error.text.is_empty() {
    app.error = HorizontallyScrollableText::default();
  }
}

fn handle_prompt_toggle(app: &mut App<'_>, key: &Key) {
  match key {
    _ if *key == DEFAULT_KEYBINDINGS.left.key || *key == DEFAULT_KEYBINDINGS.right.key => {
      if let Route::Radarr(_, _) = *app.get_current_route() {
        app.data.radarr_data.prompt_confirm = !app.data.radarr_data.prompt_confirm;
      }
    }
    _ => (),
  }
}

#[macro_export]
macro_rules! handle_text_box_left_right_keys {
  ($self:expr, $key:expr, $input:expr) => {
    match $self.key {
      _ if *$key == DEFAULT_KEYBINDINGS.left.key => {
        $input.scroll_left();
      }
      _ if *$key == DEFAULT_KEYBINDINGS.right.key => {
        $input.scroll_right();
      }
      _ => (),
    }
  };
}

#[macro_export]
macro_rules! handle_text_box_keys {
  ($self:expr, $key:expr, $input:expr) => {
    match $self.key {
      _ if *$key == DEFAULT_KEYBINDINGS.backspace.key => {
        $input.pop();
      }
      Key::Char(character) => {
        $input.push(*character);
      }
      _ => (),
    }
  };
}
