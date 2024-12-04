use crate::app::App;
use crate::event::Key;
use crate::handlers::KeyEventHandler;
use crate::models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock;
use crate::models::Scrollable;

#[cfg(test)]
#[path = "test_all_indexers_handler_tests.rs"]
mod test_all_indexers_handler_tests;

pub(super) struct TestAllIndexersHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_sonarr_block: ActiveSonarrBlock,
  _context: Option<ActiveSonarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveSonarrBlock> for TestAllIndexersHandler<'a, 'b> {
  fn accepts(active_block: ActiveSonarrBlock) -> bool {
    active_block == ActiveSonarrBlock::TestAllIndexers
  }

  fn with(
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

  fn handle_scroll_up(&mut self) {
    if self.active_sonarr_block == ActiveSonarrBlock::TestAllIndexers {
      self
        .app
        .data
        .sonarr_data
        .indexer_test_all_results
        .as_mut()
        .unwrap()
        .scroll_up()
    }
  }

  fn handle_scroll_down(&mut self) {
    if self.active_sonarr_block == ActiveSonarrBlock::TestAllIndexers {
      self
        .app
        .data
        .sonarr_data
        .indexer_test_all_results
        .as_mut()
        .unwrap()
        .scroll_down()
    }
  }

  fn handle_home(&mut self) {
    if self.active_sonarr_block == ActiveSonarrBlock::TestAllIndexers {
      self
        .app
        .data
        .sonarr_data
        .indexer_test_all_results
        .as_mut()
        .unwrap()
        .scroll_to_top()
    }
  }

  fn handle_end(&mut self) {
    if self.active_sonarr_block == ActiveSonarrBlock::TestAllIndexers {
      self
        .app
        .data
        .sonarr_data
        .indexer_test_all_results
        .as_mut()
        .unwrap()
        .scroll_to_bottom()
    }
  }

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
}
