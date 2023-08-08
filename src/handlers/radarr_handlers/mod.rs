use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::handlers::radarr_handlers::collections::CollectionsHandler;
use crate::handlers::radarr_handlers::downloads::DownloadsHandler;
use crate::handlers::radarr_handlers::indexers::IndexersHandler;
use crate::handlers::radarr_handlers::library::LibraryHandler;
use crate::handlers::radarr_handlers::root_folders::RootFoldersHandler;
use crate::handlers::radarr_handlers::system::SystemHandler;
use crate::handlers::KeyEventHandler;
use crate::models::servarr_data::radarr_data::ActiveRadarrBlock;
use crate::utils::strip_non_search_characters;
use crate::{App, Key};

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
  key: &'a Key,
  app: &'a mut App<'b>,
  active_radarr_block: &'a ActiveRadarrBlock,
  context: &'a Option<ActiveRadarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for RadarrHandler<'a, 'b> {
  fn handle(&mut self) {
    match self.active_radarr_block {
      _ if LibraryHandler::accepts(self.active_radarr_block) => {
        LibraryHandler::with(self.key, self.app, self.active_radarr_block, self.context).handle();
      }
      _ if CollectionsHandler::accepts(self.active_radarr_block) => {
        CollectionsHandler::with(self.key, self.app, self.active_radarr_block, self.context)
          .handle()
      }
      _ if IndexersHandler::accepts(self.active_radarr_block) => {
        IndexersHandler::with(self.key, self.app, self.active_radarr_block, self.context).handle()
      }
      _ if SystemHandler::accepts(self.active_radarr_block) => {
        SystemHandler::with(self.key, self.app, self.active_radarr_block, self.context).handle()
      }
      _ if DownloadsHandler::accepts(self.active_radarr_block) => {
        DownloadsHandler::with(self.key, self.app, self.active_radarr_block, self.context).handle()
      }
      _ if RootFoldersHandler::accepts(self.active_radarr_block) => {
        RootFoldersHandler::with(self.key, self.app, self.active_radarr_block, self.context)
          .handle()
      }
      _ => self.handle_key_event(),
    }
  }

  fn accepts(_active_block: &'a ActiveRadarrBlock) -> bool {
    true
  }

  fn with(
    key: &'a Key,
    app: &'a mut App<'b>,
    active_block: &'a ActiveRadarrBlock,
    context: &'a Option<ActiveRadarrBlock>,
  ) -> RadarrHandler<'a, 'b> {
    RadarrHandler {
      key,
      app,
      active_radarr_block: active_block,
      context,
    }
  }

  fn get_key(&self) -> &Key {
    self.key
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

pub fn search_table<T, F>(app: &mut App<'_>, rows: &[T], field_selection_fn: F) -> Option<usize>
where
  F: Fn(&T) -> &str,
{
  let search_string = app.data.radarr_data.search.drain().to_lowercase();
  let search_index = rows.iter().position(|item| {
    strip_non_search_characters(field_selection_fn(item)).contains(&search_string)
  });

  app.data.radarr_data.is_searching = false;
  app.should_ignore_quit_key = false;

  if search_index.is_some() {
    app.pop_navigation_stack();
  }

  search_index
}

pub fn filter_table<T, F>(app: &mut App<'_>, rows: &[T], field_selection_fn: F) -> Vec<T>
where
  F: Fn(&T) -> &str,
  T: Clone,
{
  let filter = strip_non_search_characters(&app.data.radarr_data.filter.drain());
  let filter_matches: Vec<T> = rows
    .iter()
    .filter(|&item| strip_non_search_characters(field_selection_fn(item)).contains(&filter))
    .cloned()
    .collect();

  app.data.radarr_data.is_filtering = false;
  app.should_ignore_quit_key = false;

  if !filter_matches.is_empty() {
    app.pop_navigation_stack();
  }

  filter_matches
}

pub fn handle_change_tab_left_right_keys(app: &mut App<'_>, key: &Key) {
  let key_ref = key;
  match key_ref {
    _ if *key == DEFAULT_KEYBINDINGS.left.key => {
      app.data.radarr_data.main_tabs.previous();
      app.pop_and_push_navigation_stack(*app.data.radarr_data.main_tabs.get_active_route());
    }
    _ if *key == DEFAULT_KEYBINDINGS.right.key => {
      app.data.radarr_data.main_tabs.next();
      app.pop_and_push_navigation_stack(*app.data.radarr_data.main_tabs.get_active_route());
    }
    _ => (),
  }
}
