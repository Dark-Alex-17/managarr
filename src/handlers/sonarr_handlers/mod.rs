use blocklist::BlocklistHandler;
use downloads::DownloadsHandler;
use history::HistoryHandler;
use indexers::IndexersHandler;
use library::LibraryHandler;
use root_folders::RootFoldersHandler;
use system::SystemHandler;

use crate::{
  app::App, event::Key, matches_key, models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock,
};
use crate::models::Route;
use super::KeyEventHandler;

mod blocklist;
mod downloads;
mod history;
mod indexers;
mod library;
mod root_folders;
mod system;

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
        LibraryHandler::new(self.key, self.app, self.active_sonarr_block, self.context).handle();
      }
      _ if DownloadsHandler::accepts(self.active_sonarr_block) => {
        DownloadsHandler::new(self.key, self.app, self.active_sonarr_block, self.context).handle()
      }
      _ if BlocklistHandler::accepts(self.active_sonarr_block) => {
        BlocklistHandler::new(self.key, self.app, self.active_sonarr_block, self.context).handle()
      }
      _ if HistoryHandler::accepts(self.active_sonarr_block) => {
        HistoryHandler::new(self.key, self.app, self.active_sonarr_block, self.context).handle()
      }
      _ if RootFoldersHandler::accepts(self.active_sonarr_block) => {
        RootFoldersHandler::new(self.key, self.app, self.active_sonarr_block, self.context).handle()
      }
      _ if IndexersHandler::accepts(self.active_sonarr_block) => {
        IndexersHandler::new(self.key, self.app, self.active_sonarr_block, self.context).handle()
      }
      _ if SystemHandler::accepts(self.active_sonarr_block) => {
        SystemHandler::new(self.key, self.app, self.active_sonarr_block, self.context).handle()
      }
      _ => self.handle_key_event(),
    }
  }

  fn accepts(_active_block: ActiveSonarrBlock) -> bool {
    true
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
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

  fn app_mut(&mut self) -> &mut App<'b> {
    self.app
  }

  fn current_route(&self) -> Route {
    self.app.get_current_route()
  }
}

pub fn handle_change_tab_left_right_keys(app: &mut App<'_>, key: Key) {
  let key_ref = key;
  match key_ref {
    _ if matches_key!(left, key, app.ignore_special_keys_for_textbox_input) => {
      app.data.sonarr_data.main_tabs.previous();
      app.pop_and_push_navigation_stack(app.data.sonarr_data.main_tabs.get_active_route());
    }
    _ if matches_key!(right, key, app.ignore_special_keys_for_textbox_input) => {
      app.data.sonarr_data.main_tabs.next();
      app.pop_and_push_navigation_stack(app.data.sonarr_data.main_tabs.get_active_route());
    }
    _ => (),
  }
}
