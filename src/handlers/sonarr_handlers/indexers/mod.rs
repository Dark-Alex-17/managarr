use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::App;
use crate::event::Key;
use crate::handle_table_events;
use crate::handlers::sonarr_handlers::handle_change_tab_left_right_keys;
use crate::handlers::sonarr_handlers::indexers::edit_indexer_handler::EditIndexerHandler;
use crate::handlers::sonarr_handlers::indexers::edit_indexer_settings_handler::IndexerSettingsHandler;
use crate::handlers::sonarr_handlers::indexers::test_all_indexers_handler::TestAllIndexersHandler;
use crate::handlers::table_handler::TableHandlingConfig;
use crate::handlers::{handle_clear_errors, handle_prompt_toggle, KeyEventHandler};
use crate::models::servarr_data::sonarr::sonarr_data::{
  ActiveSonarrBlock, EDIT_INDEXER_NZB_SELECTION_BLOCKS, EDIT_INDEXER_TORRENT_SELECTION_BLOCKS,
  INDEXERS_BLOCKS, INDEXER_SETTINGS_SELECTION_BLOCKS,
};
use crate::models::servarr_models::Indexer;
use crate::models::BlockSelectionState;
use crate::network::sonarr_network::SonarrEvent;

mod edit_indexer_handler;
mod edit_indexer_settings_handler;
mod test_all_indexers_handler;

#[cfg(test)]
#[path = "indexers_handler_tests.rs"]
mod indexers_handler_tests;

pub(super) struct IndexersHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_sonarr_block: ActiveSonarrBlock,
  context: Option<ActiveSonarrBlock>,
}

impl<'a, 'b> IndexersHandler<'a, 'b> {
  handle_table_events!(self, indexers, self.app.data.sonarr_data.indexers, Indexer);

  fn extract_indexer_id(&self) -> i64 {
    self.app.data.sonarr_data.indexers.current_selection().id
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveSonarrBlock> for IndexersHandler<'a, 'b> {
  fn handle(&mut self) {
    let indexers_table_handling_config =
      TableHandlingConfig::new(ActiveSonarrBlock::Indexers.into());

    if !self.handle_indexers_table_events(indexers_table_handling_config) {
      match self.active_sonarr_block {
        _ if EditIndexerHandler::accepts(self.active_sonarr_block) => {
          EditIndexerHandler::with(self.key, self.app, self.active_sonarr_block, self.context)
            .handle()
        }
        _ if IndexerSettingsHandler::accepts(self.active_sonarr_block) => {
          IndexerSettingsHandler::with(self.key, self.app, self.active_sonarr_block, self.context)
            .handle()
        }
        _ if TestAllIndexersHandler::accepts(self.active_sonarr_block) => {
          TestAllIndexersHandler::with(self.key, self.app, self.active_sonarr_block, self.context)
            .handle()
        }
        _ => self.handle_key_event(),
      }
    }
  }

  fn accepts(active_block: ActiveSonarrBlock) -> bool {
    EditIndexerHandler::accepts(active_block)
      || IndexerSettingsHandler::accepts(active_block)
      || TestAllIndexersHandler::accepts(active_block)
      || INDEXERS_BLOCKS.contains(&active_block)
  }

  fn with(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveSonarrBlock,
    context: Option<ActiveSonarrBlock>,
  ) -> IndexersHandler<'a, 'b> {
    IndexersHandler {
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
    !self.app.is_loading && !self.app.data.sonarr_data.indexers.is_empty()
  }

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {
    if self.active_sonarr_block == ActiveSonarrBlock::Indexers {
      self
        .app
        .push_navigation_stack(ActiveSonarrBlock::DeleteIndexerPrompt.into());
    }
  }

  fn handle_left_right_action(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::Indexers => handle_change_tab_left_right_keys(self.app, self.key),
      ActiveSonarrBlock::DeleteIndexerPrompt => handle_prompt_toggle(self.app, self.key),
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::DeleteIndexerPrompt => {
        if self.app.data.sonarr_data.prompt_confirm {
          self.app.data.sonarr_data.prompt_confirm_action =
            Some(SonarrEvent::DeleteIndexer(self.extract_indexer_id()));
        }

        self.app.pop_navigation_stack();
      }
      ActiveSonarrBlock::Indexers => {
        self
          .app
          .push_navigation_stack(ActiveSonarrBlock::EditIndexerPrompt.into());
        self.app.data.sonarr_data.edit_indexer_modal = Some((&self.app.data.sonarr_data).into());
        let protocol = &self
          .app
          .data
          .sonarr_data
          .indexers
          .current_selection()
          .protocol;
        if protocol == "torrent" {
          self.app.data.sonarr_data.selected_block =
            BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);
        } else {
          self.app.data.sonarr_data.selected_block =
            BlockSelectionState::new(EDIT_INDEXER_NZB_SELECTION_BLOCKS);
        }
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::DeleteIndexerPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.sonarr_data.prompt_confirm = false;
      }
      ActiveSonarrBlock::TestIndexer => {
        self.app.pop_navigation_stack();
        self.app.data.sonarr_data.indexer_test_errors = None;
      }
      _ => handle_clear_errors(self.app),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_sonarr_block {
      ActiveSonarrBlock::Indexers => match self.key {
        _ if key == DEFAULT_KEYBINDINGS.refresh.key => {
          self.app.should_refresh = true;
        }
        _ if key == DEFAULT_KEYBINDINGS.test.key => {
          self
            .app
            .push_navigation_stack(ActiveSonarrBlock::TestIndexer.into());
        }
        _ if key == DEFAULT_KEYBINDINGS.test_all.key => {
          self
            .app
            .push_navigation_stack(ActiveSonarrBlock::TestAllIndexers.into());
        }
        _ if key == DEFAULT_KEYBINDINGS.settings.key => {
          self
            .app
            .push_navigation_stack(ActiveSonarrBlock::AllIndexerSettingsPrompt.into());
          self.app.data.sonarr_data.selected_block =
            BlockSelectionState::new(INDEXER_SETTINGS_SELECTION_BLOCKS);
        }
        _ => (),
      },
      ActiveSonarrBlock::DeleteIndexerPrompt => {
        if key == DEFAULT_KEYBINDINGS.confirm.key {
          self.app.data.sonarr_data.prompt_confirm = true;
          self.app.data.sonarr_data.prompt_confirm_action =
            Some(SonarrEvent::DeleteIndexer(self.extract_indexer_id()));

          self.app.pop_navigation_stack();
        }
      }
      _ => (),
    }
  }
}
