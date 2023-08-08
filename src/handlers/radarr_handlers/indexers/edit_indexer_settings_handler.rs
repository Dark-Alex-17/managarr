use crate::app::App;
use crate::event::Key;
use crate::handlers::KeyEventHandler;
use crate::models::servarr_data::radarr_data::{ActiveRadarrBlock, INDEXER_SETTINGS_BLOCKS};

#[cfg(test)]
#[path = "./edit_indexer_settings_handler_tests.rs"]
mod edit_indexer_settings_handler_tests;

pub(super) struct IndexerSettingsHandler<'a, 'b> {
  key: &'a Key,
  app: &'a mut App<'b>,
  active_radarr_block: &'a ActiveRadarrBlock,
  _context: &'a Option<ActiveRadarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for IndexerSettingsHandler<'a, 'b> {
  fn accepts(active_block: &'a ActiveRadarrBlock) -> bool {
    INDEXER_SETTINGS_BLOCKS.contains(active_block)
  }

  fn with(
    key: &'a Key,
    app: &'a mut App<'b>,
    active_block: &'a ActiveRadarrBlock,
    _context: &'a Option<ActiveRadarrBlock>,
  ) -> IndexerSettingsHandler<'a, 'b> {
    IndexerSettingsHandler {
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
    if self.active_radarr_block == &ActiveRadarrBlock::IndexerSettingsPrompt {
      self.app.data.radarr_data.selected_block.previous()
    }
  }

  fn handle_scroll_down(&mut self) {
    if self.active_radarr_block == &ActiveRadarrBlock::IndexerSettingsPrompt {
      self.app.data.radarr_data.selected_block.next()
    }
  }

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {}

  fn handle_submit(&mut self) {}

  fn handle_esc(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::IndexerSettingsPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.prompt_confirm = false;
        self.app.data.radarr_data.indexer_settings = None;
      }
      _ => self.app.pop_navigation_stack(), // Need to tweak this still and add unit tests
    }
  }

  fn handle_char_key_event(&mut self) {}
}
