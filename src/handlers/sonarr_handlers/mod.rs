use blocklist::BlocklistHandler;
use downloads::DownloadsHandler;
use library::LibraryHandler;

use crate::{
  app::{key_binding::DEFAULT_KEYBINDINGS, App},
  event::Key,
  models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock,
};

use super::KeyEventHandler;

mod blocklist;
mod downloads;
mod library;

#[cfg(test)]
#[path = "sonarr_handler_tests.rs"]
mod sonarr_handler_tests;

#[cfg(test)]
#[path = "sonarr_handler_test_utils.rs"]
mod sonarr_handler_test_utils;

pub(super) struct SonarrHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_sonarr_block: ActiveSonarrBlock,
  context: Option<ActiveSonarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveSonarrBlock> for SonarrHandler<'a, 'b> {
  fn handle(&mut self) {
    match self.active_sonarr_block {
      _ if LibraryHandler::accepts(self.active_sonarr_block) => {
        LibraryHandler::with(self.key, self.app, self.active_sonarr_block, self.context).handle();
      }
      _ if DownloadsHandler::accepts(self.active_sonarr_block) => {
        DownloadsHandler::with(self.key, self.app, self.active_sonarr_block, self.context).handle()
      }
      _ if BlocklistHandler::accepts(self.active_sonarr_block) => {
        BlocklistHandler::with(self.key, self.app, self.active_sonarr_block, self.context).handle()
      }
      _ => self.handle_key_event(),
    }
  }

  fn accepts(_active_block: ActiveSonarrBlock) -> bool {
    true
  }

  fn with(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveSonarrBlock,
    context: Option<ActiveSonarrBlock>,
  ) -> SonarrHandler<'a, 'b> {
    SonarrHandler {
      key,
      app,
      active_sonarr_block: active_block,
      context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
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
}

pub fn handle_change_tab_left_right_keys(app: &mut App<'_>, key: Key) {
  let key_ref = key;
  match key_ref {
    _ if key == DEFAULT_KEYBINDINGS.left.key => {
      app.data.sonarr_data.main_tabs.previous();
      app.pop_and_push_navigation_stack(app.data.sonarr_data.main_tabs.get_active_route());
    }
    _ if key == DEFAULT_KEYBINDINGS.right.key => {
      app.data.sonarr_data.main_tabs.next();
      app.pop_and_push_navigation_stack(app.data.sonarr_data.main_tabs.get_active_route());
    }
    _ => (),
  }
}
