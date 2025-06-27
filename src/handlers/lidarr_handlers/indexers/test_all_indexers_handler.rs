use crate::app::App;
use crate::event::Key;
use crate::handle_table_events;
use crate::handlers::table_handler::TableHandlingConfig;
use crate::handlers::KeyEventHandler;
use crate::models::servarr_data::modals::IndexerTestResultModalItem;
use crate::models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock;

#[cfg(test)]
#[path = "test_all_indexers_handler_tests.rs"]
mod test_all_indexers_handler_tests;

pub(super) struct TestAllIndexersHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_lidarr_block: ActiveLidarrBlock,
  _context: Option<ActiveLidarrBlock>,
}

impl TestAllIndexersHandler<'_, '_> {
  handle_table_events!(
    self,
    indexer_test_all_results,
    self
      .app
      .data
      .lidarr_data
      .indexer_test_all_results
      .as_mut()
      .unwrap(),
    IndexerTestResultModalItem
  );
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveLidarrBlock> for TestAllIndexersHandler<'a, 'b> {
  fn handle(&mut self) {
    let indexer_test_all_results_table_handling_config =
      TableHandlingConfig::new(ActiveLidarrBlock::TestAllIndexers.into());

    if !self
      .handle_indexer_test_all_results_table_events(indexer_test_all_results_table_handling_config)
    {
      self.handle_key_event();
    }
  }

  fn accepts(active_block: ActiveLidarrBlock) -> bool {
    active_block == ActiveLidarrBlock::TestAllIndexers
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveLidarrBlock,
    _context: Option<ActiveLidarrBlock>,
  ) -> TestAllIndexersHandler<'a, 'b> {
    TestAllIndexersHandler {
      key,
      app,
      active_lidarr_block: active_block,
      _context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    let table_is_ready = if let Some(table) = &self.app.data.lidarr_data.indexer_test_all_results {
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
    if self.active_lidarr_block == ActiveLidarrBlock::TestAllIndexers {
      self.app.pop_navigation_stack();
      self.app.data.lidarr_data.indexer_test_all_results = None;
    }
  }

  fn handle_char_key_event(&mut self) {}
}
