use crate::app::App;
use crate::event::Key;
use crate::handlers::KeyEventHandler;
use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
use crate::models::Scrollable;

#[cfg(test)]
#[path = "test_all_indexers_handler_tests.rs"]
mod test_all_indexers_handler_tests;

pub(super) struct TestAllIndexersHandler<'a, 'b> {
  key: &'a Key,
  app: &'a mut App<'b>,
  active_radarr_block: &'a ActiveRadarrBlock,
  _context: &'a Option<ActiveRadarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for TestAllIndexersHandler<'a, 'b> {
  fn accepts(active_block: &'a ActiveRadarrBlock) -> bool {
    active_block == &ActiveRadarrBlock::TestAllIndexers
  }

  fn with(
    key: &'a Key,
    app: &'a mut App<'b>,
    active_block: &'a ActiveRadarrBlock,
    _context: &'a Option<ActiveRadarrBlock>,
  ) -> TestAllIndexersHandler<'a, 'b> {
    TestAllIndexersHandler {
      key,
      app,
      active_radarr_block: active_block,
      _context,
    }
  }

  fn get_key(&self) -> &Key {
    self.key
  }

  fn handle_scroll_up(&mut self) {
    if self.active_radarr_block == &ActiveRadarrBlock::TestAllIndexers {
      self
        .app
        .data
        .radarr_data
        .indexer_test_all_results
        .as_mut()
        .unwrap()
        .scroll_up()
    }
  }

  fn handle_scroll_down(&mut self) {
    if self.active_radarr_block == &ActiveRadarrBlock::TestAllIndexers {
      self
        .app
        .data
        .radarr_data
        .indexer_test_all_results
        .as_mut()
        .unwrap()
        .scroll_down()
    }
  }

  fn handle_home(&mut self) {
    if self.active_radarr_block == &ActiveRadarrBlock::TestAllIndexers {
      self
        .app
        .data
        .radarr_data
        .indexer_test_all_results
        .as_mut()
        .unwrap()
        .scroll_to_top()
    }
  }

  fn handle_end(&mut self) {
    if self.active_radarr_block == &ActiveRadarrBlock::TestAllIndexers {
      self
        .app
        .data
        .radarr_data
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
    if self.active_radarr_block == &ActiveRadarrBlock::TestAllIndexers {
      self.app.pop_navigation_stack();
      self.app.data.radarr_data.indexer_test_all_results = None;
    }
  }

  fn handle_char_key_event(&mut self) {}
}
