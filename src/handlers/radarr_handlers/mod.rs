use crate::handlers::radarr_handlers::blocklist::BlocklistHandler;
use crate::handlers::radarr_handlers::collections::CollectionsHandler;
use crate::handlers::radarr_handlers::downloads::DownloadsHandler;
use crate::handlers::radarr_handlers::indexers::IndexersHandler;
use crate::handlers::radarr_handlers::library::LibraryHandler;
use crate::handlers::radarr_handlers::root_folders::RootFoldersHandler;
use crate::handlers::radarr_handlers::system::SystemHandler;
use crate::handlers::KeyEventHandler;
use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
use crate::{matches_key, App, Key};

mod blocklist;
mod collections;
mod downloads;
mod indexers;
mod library;
mod root_folders;
mod system;

#[cfg(test)]
#[path = "radarr_handler_tests.rs"]
mod radarr_handler_tests;

#[cfg(test)]
#[path = "radarr_handler_test_utils.rs"]
mod radarr_handler_test_utils;

pub(super) struct RadarrHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_radarr_block: ActiveRadarrBlock,
  context: Option<ActiveRadarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for RadarrHandler<'a, 'b> {
  fn handle(&mut self) {
    match self.active_radarr_block {
      _ if LibraryHandler::accepts(self.active_radarr_block) => {
        LibraryHandler::new(self.key, self.app, self.active_radarr_block, self.context).handle();
      }
      _ if CollectionsHandler::accepts(self.active_radarr_block) => {
        CollectionsHandler::new(self.key, self.app, self.active_radarr_block, self.context).handle()
      }
      _ if IndexersHandler::accepts(self.active_radarr_block) => {
        IndexersHandler::new(self.key, self.app, self.active_radarr_block, self.context).handle()
      }
      _ if SystemHandler::accepts(self.active_radarr_block) => {
        SystemHandler::new(self.key, self.app, self.active_radarr_block, self.context).handle()
      }
      _ if DownloadsHandler::accepts(self.active_radarr_block) => {
        DownloadsHandler::new(self.key, self.app, self.active_radarr_block, self.context).handle()
      }
      _ if RootFoldersHandler::accepts(self.active_radarr_block) => {
        RootFoldersHandler::new(self.key, self.app, self.active_radarr_block, self.context).handle()
      }
      _ if BlocklistHandler::accepts(self.active_radarr_block) => {
        BlocklistHandler::new(self.key, self.app, self.active_radarr_block, self.context).handle()
      }
      _ => self.handle_key_event(),
    }
  }

  fn accepts(_active_block: ActiveRadarrBlock) -> bool {
    true
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveRadarrBlock,
    context: Option<ActiveRadarrBlock>,
  ) -> RadarrHandler<'a, 'b> {
    RadarrHandler {
      key,
      app,
      active_radarr_block: active_block,
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
    _ if matches_key!(left, key, app.ignore_special_keys_for_textbox_input) => {
      app.data.radarr_data.main_tabs.previous();
      app.pop_and_push_navigation_stack(app.data.radarr_data.main_tabs.get_active_route());
    }
    _ if matches_key!(right, key, app.ignore_special_keys_for_textbox_input) => {
      app.data.radarr_data.main_tabs.next();
      app.pop_and_push_navigation_stack(app.data.radarr_data.main_tabs.get_active_route());
    }
    _ => (),
  }
}

#[macro_export]
macro_rules! search_table {
  ($app:expr, $data_ref:ident, $error_block:expr) => {
    let search_index = if let Some(search_str) = $app.data.radarr_data.search.take() {
      let search_string = search_str.text.to_lowercase();

      $app
        .data
        .radarr_data
        .$data_ref
        .items
        .iter()
        .position(|item| strip_non_search_characters(&item.title.text).contains(&search_string))
    } else {
      None
    };

    $app.data.radarr_data.is_searching = false;
    $app.ignore_special_keys_for_textbox_input = false;

    if search_index.is_some() {
      $app.pop_navigation_stack();
      $app.data.radarr_data.$data_ref.select_index(search_index);
    } else {
      $app.pop_and_push_navigation_stack($error_block.into());
    }
  };
  ($app:expr, $data_ref:ident, $error_block:expr, $option:ident) => {
    let search_index = if let Some(search_str) = $app.data.radarr_data.search.take() {
      let search_string = search_str.text.to_lowercase();

      $app
        .data
        .radarr_data
        .$data_ref
        .as_ref()
        .unwrap()
        .items
        .iter()
        .position(|item| strip_non_search_characters(&item.title.text).contains(&search_string))
    } else {
      None
    };

    $app.data.radarr_data.is_searching = false;
    $app.ignore_special_keys_for_textbox_input = false;

    if search_index.is_some() {
      $app.pop_navigation_stack();
      $app
        .data
        .radarr_data
        .$data_ref
        .as_mut()
        .unwrap()
        .select_index(search_index);
    } else {
      $app.pop_and_push_navigation_stack($error_block.into());
    }
  };
}
