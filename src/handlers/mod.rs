use radarr_handlers::RadarrHandler;
use sonarr_handlers::SonarrHandler;

use crate::app::context_clues::{
  ContextClueProvider, ServarrContextClueProvider, SERVARR_CONTEXT_CLUES,
};
use crate::app::key_binding::KeyBinding;
use crate::app::App;
use crate::event::Key;
use crate::handlers::keybinding_handler::KeybindingHandler;
use crate::matches_key;
use crate::models::servarr_data::ActiveKeybindingBlock;
use crate::models::servarr_models::KeybindingItem;
use crate::models::stateful_table::StatefulTable;
use crate::models::{HorizontallyScrollableText, Route};

mod keybinding_handler;
mod radarr_handlers;
mod sonarr_handlers;

#[cfg(test)]
#[path = "handlers_tests.rs"]
mod handlers_tests;

#[cfg(test)]
#[path = "handler_test_utils.rs"]
pub mod handler_test_utils;
mod table_handler;

pub trait KeyEventHandler<'a, 'b, T: Into<Route> + Copy> {
  fn handle_key_event(&mut self) {
    let key = self.get_key();
    match key {
      _ if matches_key!(up, key, self.ignore_special_keys()) => {
        if self.is_ready() {
          self.handle_scroll_up();
        }
      }
      _ if matches_key!(down, key, self.ignore_special_keys()) => {
        if self.is_ready() {
          self.handle_scroll_down();
        }
      }
      _ if matches_key!(home, key) => {
        if self.is_ready() {
          self.handle_home();
        }
      }
      _ if matches_key!(end, key) => {
        if self.is_ready() {
          self.handle_end();
        }
      }
      _ if matches_key!(delete, key) => {
        if self.is_ready() {
          self.handle_delete();
        }
      }
      _ if matches_key!(left, key, self.ignore_special_keys())
        || matches_key!(right, key, self.ignore_special_keys()) =>
      {
        self.handle_left_right_action()
      }
      _ if matches_key!(submit, key) => {
        if self.is_ready() {
          self.handle_submit();
        }
      }
      _ if matches_key!(esc, key) => self.handle_esc(),
      _ => {
        if self.is_ready() {
          self.handle_char_key_event();
        }
      }
    }
  }

  fn handle(&mut self) {
    self.handle_key_event();
  }

  fn accepts(active_block: T) -> bool;
  fn new(key: Key, app: &'a mut App<'b>, active_block: T, context: Option<T>) -> Self;
  fn get_key(&self) -> Key;
  fn ignore_special_keys(&self) -> bool;
  fn is_ready(&self) -> bool;
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
  if matches_key!(next_servarr, key) {
    app.reset();
    app.server_tabs.next();
    app.pop_and_push_navigation_stack(app.server_tabs.get_active_route());
    app.cancellation_token.cancel();
  } else if matches_key!(previous_servarr, key) {
    app.reset();
    app.server_tabs.previous();
    app.pop_and_push_navigation_stack(app.server_tabs.get_active_route());
    app.cancellation_token.cancel();
  } else if matches_key!(help, key) && !app.ignore_special_keys_for_textbox_input {
    if app.keymapping_table.is_none() {
      populate_keymapping_table(app);
    } else {
      app.keymapping_table = None;
    }
  } else {
    match app.get_current_route() {
      _ if app.keymapping_table.is_some() => {
        KeybindingHandler::new(key, app, ActiveKeybindingBlock::Help, None).handle();
      }
      Route::Radarr(active_radarr_block, context) => {
        RadarrHandler::new(key, app, active_radarr_block, context).handle()
      }
      Route::Sonarr(active_sonarr_block, context) => {
        SonarrHandler::new(key, app, active_sonarr_block, context).handle()
      }
      _ => (),
    }
  }
}

fn populate_keymapping_table(app: &mut App<'_>) {
  let context_clue_to_keybinding_item = |key: &KeyBinding, desc: &&str| {
    let (key, alt_key) = if key.alt.is_some() {
      (key.key.to_string(), key.alt.as_ref().unwrap().to_string())
    } else {
      (key.key.to_string(), String::new())
    };
    KeybindingItem {
      key,
      alt_key,
      desc: desc.to_string(),
    }
  };
  let mut keybindings = Vec::new();
  let global_keybindings = Vec::from(SERVARR_CONTEXT_CLUES)
    .iter()
    .map(|(key, desc)| context_clue_to_keybinding_item(key, desc))
    .collect::<Vec<_>>();

  if let Some(contextual_help) = app.server_tabs.get_active_route_contextual_help() {
    keybindings.extend(
      contextual_help
        .iter()
        .map(|(key, desc)| context_clue_to_keybinding_item(key, desc)),
    );
  }

  if let Some(contextual_help) = ServarrContextClueProvider::get_context_clues(app) {
    keybindings.extend(
      contextual_help
        .iter()
        .map(|(key, desc)| context_clue_to_keybinding_item(key, desc)),
    );
  }

  keybindings.extend(global_keybindings);

  let mut table = StatefulTable::default();
  table.set_items(keybindings);
  app.keymapping_table = Some(table);
}

fn handle_clear_errors(app: &mut App<'_>) {
  if !app.error.text.is_empty() {
    app.error = HorizontallyScrollableText::default();
  }
}

fn handle_prompt_toggle(app: &mut App<'_>, key: Key) {
  match key {
    _ if matches_key!(left, key) || matches_key!(right, key) => match app.get_current_route() {
      Route::Radarr(_, _) => {
        app.data.radarr_data.prompt_confirm = !app.data.radarr_data.prompt_confirm
      }
      Route::Sonarr(_, _) => {
        app.data.sonarr_data.prompt_confirm = !app.data.sonarr_data.prompt_confirm
      }
      _ => (),
    },
    _ => (),
  }
}

#[macro_export]
macro_rules! handle_text_box_left_right_keys {
  ($self:expr, $key:expr, $input:expr) => {
    match $self.key {
      _ if $key == $crate::app::key_binding::DEFAULT_KEYBINDINGS.left.key => {
        $input.scroll_left();
      }
      _ if $key == $crate::app::key_binding::DEFAULT_KEYBINDINGS.right.key => {
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
      _ if $crate::matches_key!(backspace, $key) => {
        $input.pop();
      }
      Key::Char(character) => {
        $input.push(character);
      }
      _ => (),
    }
  };
}

#[macro_export]
macro_rules! handle_prompt_left_right_keys {
  ($self:expr, $confirm_prompt:expr, $data:ident) => {
    if $self.app.data.$data.selected_block.get_active_block() == $confirm_prompt {
      handle_prompt_toggle($self.app, $self.key);
    } else if $crate::matches_key!(left, $self.key) {
      $self.app.data.$data.selected_block.left();
    } else {
      $self.app.data.$data.selected_block.right();
    }
  };
}
