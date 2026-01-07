use crate::app::App;
use crate::event::Key;
use crate::handlers::KeyEventHandler;
use crate::handlers::table_handler::{TableHandlingConfig, handle_table};
use crate::models::Route;
use crate::models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock;

#[cfg(test)]
#[path = "test_all_indexers_handler_tests.rs"]
mod test_all_indexers_handler_tests;

pub(super) struct TestAllIndexersHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_sonarr_block: ActiveSonarrBlock,
  _context: Option<ActiveSonarrBlock>,
}

impl TestAllIndexersHandler<'_, '_> {}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveSonarrBlock> for TestAllIndexersHandler<'a, 'b> {
  fn handle(&mut self) {
    let indexer_test_all_results_table_handling_config =
      TableHandlingConfig::new(ActiveSonarrBlock::TestAllIndexers.into());

    if !handle_table(
      self,
      |app| {
        app
          .data
          .sonarr_data
          .indexer_test_all_results
          .as_mut()
          .unwrap()
      },
      indexer_test_all_results_table_handling_config,
    ) {
      self.handle_key_event();
    }
  }

  fn accepts(active_block: ActiveSonarrBlock) -> bool {
    active_block == ActiveSonarrBlock::TestAllIndexers
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveSonarrBlock,
    _context: Option<ActiveSonarrBlock>,
  ) -> TestAllIndexersHandler<'a, 'b> {
    TestAllIndexersHandler {
      key,
      app,
      active_sonarr_block: active_block,
      _context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    let table_is_ready = if let Some(table) = &self.app.data.sonarr_data.indexer_test_all_results {
      !table.is_empty()
    } else {
      false
    };

    !self.app.is_loading && table_is_ready
  }

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {}

  fn handle_submit(&mut self) {}

  fn handle_esc(&mut self) {
    if self.active_sonarr_block == ActiveSonarrBlock::TestAllIndexers {
      self.app.pop_navigation_stack();
      self.app.data.sonarr_data.indexer_test_all_results = None;
    }
  }

  fn handle_char_key_event(&mut self) {}

  fn app_mut(&mut self) -> &mut App<'b> {
    self.app
  }

  fn current_route(&self) -> Route {
    self.app.get_current_route()
  }
}
