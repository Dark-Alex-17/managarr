use crate::app::App;
use crate::event::Key;
use crate::handlers::KeyEventHandler;
use crate::handlers::table_handler::{TableHandlingConfig, handle_table};
use crate::models::Route;
use crate::models::servarr_data::readarr::readarr_data::ActiveReadarrBlock;

#[cfg(test)]
#[path = "test_all_indexers_handler_tests.rs"]
mod test_all_indexers_handler_tests;

pub(super) struct TestAllIndexersHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_readarr_block: ActiveReadarrBlock,
  _context: Option<ActiveReadarrBlock>,
}

impl TestAllIndexersHandler<'_, '_> {}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveReadarrBlock> for TestAllIndexersHandler<'a, 'b> {
  fn handle(&mut self) {
    let indexer_test_all_results_table_handling_config =
      TableHandlingConfig::new(ActiveReadarrBlock::TestAllIndexers.into());

    if !handle_table(
      self,
      |app| {
        app
          .data
          .readarr_data
          .indexer_test_all_results
          .as_mut()
          .unwrap()
      },
      indexer_test_all_results_table_handling_config,
    ) {
      self.handle_key_event();
    }
  }

  fn accepts(active_block: ActiveReadarrBlock) -> bool {
    active_block == ActiveReadarrBlock::TestAllIndexers
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveReadarrBlock,
    _context: Option<ActiveReadarrBlock>,
  ) -> TestAllIndexersHandler<'a, 'b> {
    TestAllIndexersHandler {
      key,
      app,
      active_readarr_block: active_block,
      _context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    let table_is_ready = if let Some(table) = &self.app.data.readarr_data.indexer_test_all_results {
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
    if self.active_readarr_block == ActiveReadarrBlock::TestAllIndexers {
      self.app.pop_navigation_stack();
      self.app.data.readarr_data.indexer_test_all_results = None;
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
