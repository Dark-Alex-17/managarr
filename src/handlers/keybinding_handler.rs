use crate::app::App;
use crate::event::Key;
use crate::handle_table_events;
use crate::handlers::table_handler::TableHandlingConfig;
use crate::handlers::KeyEventHandler;
use crate::models::servarr_data::ActiveKeybindingBlock;
use crate::models::servarr_models::KeybindingItem;

#[cfg(test)]
#[path = "keybinding_handler_tests.rs"]
mod keybinding_handler_tests;

pub(super) struct KeybindingHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
}

impl KeybindingHandler<'_, '_> {
  handle_table_events!(
    self,
    keybindings,
    self.app.keymapping_table.as_mut().unwrap(),
    KeybindingItem
  );
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveKeybindingBlock> for KeybindingHandler<'a, 'b> {
  fn handle(&mut self) {
    let keybinding_table_handling_config = TableHandlingConfig::new(self.app.get_current_route());

    if !self.handle_keybindings_table_events(keybinding_table_handling_config) {
      self.handle_key_event();
    }
  }

  fn accepts(_active_block: ActiveKeybindingBlock) -> bool {
    true
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    _active_block: ActiveKeybindingBlock,
    _context: Option<ActiveKeybindingBlock>,
  ) -> KeybindingHandler<'a, 'b> {
    KeybindingHandler { key, app }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn is_ready(&self) -> bool {
    self.app.keymapping_table.is_some()
  }

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {}

  fn handle_submit(&mut self) {}

  fn handle_esc(&mut self) {
    self.app.keymapping_table = None;
  }

  fn handle_char_key_event(&mut self) {}
}
