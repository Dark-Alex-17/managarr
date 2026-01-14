use crate::app::App;
use crate::event::Key;
use crate::handlers::lidarr_handlers::handle_change_tab_left_right_keys;
use crate::handlers::lidarr_handlers::indexers::edit_indexer_handler::EditIndexerHandler;
use crate::handlers::lidarr_handlers::indexers::edit_indexer_settings_handler::IndexerSettingsHandler;
use crate::handlers::lidarr_handlers::indexers::test_all_indexers_handler::TestAllIndexersHandler;
use crate::handlers::table_handler::{TableHandlingConfig, handle_table};
use crate::handlers::{KeyEventHandler, handle_clear_errors, handle_prompt_toggle};
use crate::matches_key;
use crate::models::servarr_data::lidarr::lidarr_data::{
  ActiveLidarrBlock, EDIT_INDEXER_NZB_SELECTION_BLOCKS, EDIT_INDEXER_TORRENT_SELECTION_BLOCKS,
  INDEXER_SETTINGS_SELECTION_BLOCKS, INDEXERS_BLOCKS,
};
use crate::models::{BlockSelectionState, Route};
use crate::network::lidarr_network::LidarrEvent;

mod edit_indexer_handler;
mod edit_indexer_settings_handler;
mod test_all_indexers_handler;

#[cfg(test)]
#[path = "indexers_handler_tests.rs"]
mod indexers_handler_tests;

pub(super) struct IndexersHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_lidarr_block: ActiveLidarrBlock,
  context: Option<ActiveLidarrBlock>,
}

impl IndexersHandler<'_, '_> {
  fn extract_indexer_id(&self) -> i64 {
    self.app.data.lidarr_data.indexers.current_selection().id
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveLidarrBlock> for IndexersHandler<'a, 'b> {
  fn handle(&mut self) {
    let indexers_table_handling_config =
      TableHandlingConfig::new(ActiveLidarrBlock::Indexers.into());

    if !handle_table(
      self,
      |app| &mut app.data.lidarr_data.indexers,
      indexers_table_handling_config,
    ) {
      match self.active_lidarr_block {
        _ if EditIndexerHandler::accepts(self.active_lidarr_block) => {
          EditIndexerHandler::new(self.key, self.app, self.active_lidarr_block, self.context)
            .handle()
        }
        _ if IndexerSettingsHandler::accepts(self.active_lidarr_block) => {
          IndexerSettingsHandler::new(self.key, self.app, self.active_lidarr_block, self.context)
            .handle()
        }
        _ if TestAllIndexersHandler::accepts(self.active_lidarr_block) => {
          TestAllIndexersHandler::new(self.key, self.app, self.active_lidarr_block, self.context)
            .handle()
        }
        _ => self.handle_key_event(),
      }
    }
  }

  fn accepts(active_block: ActiveLidarrBlock) -> bool {
    EditIndexerHandler::accepts(active_block)
      || IndexerSettingsHandler::accepts(active_block)
      || TestAllIndexersHandler::accepts(active_block)
      || INDEXERS_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveLidarrBlock,
    context: Option<ActiveLidarrBlock>,
  ) -> IndexersHandler<'a, 'b> {
    IndexersHandler {
      key,
      app,
      active_lidarr_block: active_block,
      context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    !self.app.is_loading && !self.app.data.lidarr_data.indexers.is_empty()
  }

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {
    if self.active_lidarr_block == ActiveLidarrBlock::Indexers {
      self
        .app
        .push_navigation_stack(ActiveLidarrBlock::DeleteIndexerPrompt.into());
    }
  }

  fn handle_left_right_action(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::Indexers => handle_change_tab_left_right_keys(self.app, self.key),
      ActiveLidarrBlock::DeleteIndexerPrompt => handle_prompt_toggle(self.app, self.key),
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::DeleteIndexerPrompt => {
        if self.app.data.lidarr_data.prompt_confirm {
          self.app.data.lidarr_data.prompt_confirm_action =
            Some(LidarrEvent::DeleteIndexer(self.extract_indexer_id()));
        }

        self.app.pop_navigation_stack();
      }
      ActiveLidarrBlock::Indexers => {
        self
          .app
          .push_navigation_stack(ActiveLidarrBlock::EditIndexerPrompt.into());
        self.app.data.lidarr_data.edit_indexer_modal = Some((&self.app.data.lidarr_data).into());
        let protocol = &self
          .app
          .data
          .lidarr_data
          .indexers
          .current_selection()
          .protocol;
        if protocol == "torrent" {
          self.app.data.lidarr_data.selected_block =
            BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);
        } else {
          self.app.data.lidarr_data.selected_block =
            BlockSelectionState::new(EDIT_INDEXER_NZB_SELECTION_BLOCKS);
        }
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::DeleteIndexerPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.lidarr_data.prompt_confirm = false;
      }
      ActiveLidarrBlock::TestIndexer => {
        self.app.pop_navigation_stack();
        self.app.data.lidarr_data.indexer_test_errors = None;
      }
      _ => handle_clear_errors(self.app),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_lidarr_block {
      ActiveLidarrBlock::Indexers => match self.key {
        _ if matches_key!(refresh, key) => {
          self.app.should_refresh = true;
        }
        _ if matches_key!(test, key) => {
          self
            .app
            .push_navigation_stack(ActiveLidarrBlock::TestIndexer.into());
        }
        _ if matches_key!(test_all, key) => {
          self
            .app
            .push_navigation_stack(ActiveLidarrBlock::TestAllIndexers.into());
        }
        _ if matches_key!(settings, key) => {
          self
            .app
            .push_navigation_stack(ActiveLidarrBlock::AllIndexerSettingsPrompt.into());
          self.app.data.lidarr_data.selected_block =
            BlockSelectionState::new(INDEXER_SETTINGS_SELECTION_BLOCKS);
        }
        _ => (),
      },
      ActiveLidarrBlock::DeleteIndexerPrompt => {
        if matches_key!(confirm, key) {
          self.app.data.lidarr_data.prompt_confirm = true;
          self.app.data.lidarr_data.prompt_confirm_action =
            Some(LidarrEvent::DeleteIndexer(self.extract_indexer_id()));

          self.app.pop_navigation_stack();
        }
      }
      _ => (),
    }
  }

  fn app_mut(&mut self) -> &mut App<'b> {
    self.app
  }

  fn current_route(&self) -> Route {
    self.app.get_current_route()
  }
}
