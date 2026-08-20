use super::KeyEventHandler;
use crate::models::Route;
use crate::{
  app::App, event::Key, models::servarr_data::readarr::readarr_data::ActiveReadarrBlock,
};

#[cfg(test)]
#[path = "readarr_handler_tests.rs"]
mod readarr_handler_tests;

pub(super) struct ReadarrHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_readarr_block: ActiveReadarrBlock,
  context: Option<ActiveReadarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveReadarrBlock> for ReadarrHandler<'a, 'b> {
  fn accepts(_active_block: ActiveReadarrBlock) -> bool {
    true
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveReadarrBlock,
    context: Option<ActiveReadarrBlock>,
  ) -> ReadarrHandler<'a, 'b> {
    ReadarrHandler {
      key,
      app,
      active_readarr_block: active_block,
      context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn is_ready(&self) -> bool {
    true
  }

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {}

  fn handle_submit(&mut self) {}

  fn handle_esc(&mut self) {}

  fn handle_char_key_event(&mut self) {}

  fn app_mut(&mut self) -> &mut App<'b> {
    self.app
  }

  fn current_route(&self) -> Route {
    self.app.get_current_route()
  }
}
