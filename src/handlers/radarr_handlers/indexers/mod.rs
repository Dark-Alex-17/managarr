use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::App;
use crate::event::Key;
use crate::handle_table_events;
use crate::handlers::radarr_handlers::handle_change_tab_left_right_keys;
use crate::handlers::radarr_handlers::indexers::edit_indexer_handler::EditIndexerHandler;
use crate::handlers::radarr_handlers::indexers::edit_indexer_settings_handler::IndexerSettingsHandler;
use crate::handlers::radarr_handlers::indexers::test_all_indexers_handler::TestAllIndexersHandler;
use crate::handlers::table_handler::TableHandlingConfig;
use crate::handlers::{handle_clear_errors, handle_prompt_toggle, KeyEventHandler};
use crate::models::servarr_data::radarr::radarr_data::{
  ActiveRadarrBlock, EDIT_INDEXER_NZB_SELECTION_BLOCKS, EDIT_INDEXER_TORRENT_SELECTION_BLOCKS,
  INDEXERS_BLOCKS, INDEXER_SETTINGS_SELECTION_BLOCKS,
};
use crate::models::servarr_models::Indexer;
use crate::models::BlockSelectionState;
use crate::network::radarr_network::RadarrEvent;

mod edit_indexer_handler;
mod edit_indexer_settings_handler;
mod test_all_indexers_handler;

#[cfg(test)]
#[path = "indexers_handler_tests.rs"]
mod indexers_handler_tests;

pub(super) struct IndexersHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_radarr_block: ActiveRadarrBlock,
  context: Option<ActiveRadarrBlock>,
}

impl<'a, 'b> IndexersHandler<'a, 'b> {
  handle_table_events!(self, indexers, self.app.data.radarr_data.indexers, Indexer);
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for IndexersHandler<'a, 'b> {
  fn handle(&mut self) {
    let indexer_table_handling_config =
      TableHandlingConfig::new(ActiveRadarrBlock::Indexers.into());

    if !self.handle_indexers_table_events(indexer_table_handling_config) {
      match self.active_radarr_block {
        _ if EditIndexerHandler::accepts(self.active_radarr_block) => {
          EditIndexerHandler::with(self.key, self.app, self.active_radarr_block, self.context)
            .handle()
        }
        _ if IndexerSettingsHandler::accepts(self.active_radarr_block) => {
          IndexerSettingsHandler::with(self.key, self.app, self.active_radarr_block, self.context)
            .handle()
        }
        _ if TestAllIndexersHandler::accepts(self.active_radarr_block) => {
          TestAllIndexersHandler::with(self.key, self.app, self.active_radarr_block, self.context)
            .handle()
        }
        _ => self.handle_key_event(),
      }
    }
  }

  fn accepts(active_block: ActiveRadarrBlock) -> bool {
    EditIndexerHandler::accepts(active_block)
      || IndexerSettingsHandler::accepts(active_block)
      || TestAllIndexersHandler::accepts(active_block)
      || INDEXERS_BLOCKS.contains(&active_block)
  }

  fn with(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveRadarrBlock,
    context: Option<ActiveRadarrBlock>,
  ) -> IndexersHandler<'a, 'b> {
    IndexersHandler {
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
    !self.app.is_loading && !self.app.data.radarr_data.indexers.is_empty()
  }

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {
    if self.active_radarr_block == ActiveRadarrBlock::Indexers {
      self
        .app
        .push_navigation_stack(ActiveRadarrBlock::DeleteIndexerPrompt.into());
    }
  }

  fn handle_left_right_action(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Indexers => handle_change_tab_left_right_keys(self.app, self.key),
      ActiveRadarrBlock::DeleteIndexerPrompt => handle_prompt_toggle(self.app, self.key),
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::DeleteIndexerPrompt => {
        let radarr_data = &mut self.app.data.radarr_data;
        if radarr_data.prompt_confirm {
          radarr_data.prompt_confirm_action = Some(RadarrEvent::DeleteIndexer(None));
        }

        self.app.pop_navigation_stack();
      }
      ActiveRadarrBlock::Indexers => {
        self
          .app
          .push_navigation_stack(ActiveRadarrBlock::EditIndexerPrompt.into());
        self.app.data.radarr_data.edit_indexer_modal = Some((&self.app.data.radarr_data).into());
        let protocol = &self
          .app
          .data
          .radarr_data
          .indexers
          .current_selection()
          .protocol;
        if protocol == "torrent" {
          self.app.data.radarr_data.selected_block =
            BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);
        } else {
          self.app.data.radarr_data.selected_block =
            BlockSelectionState::new(EDIT_INDEXER_NZB_SELECTION_BLOCKS);
        }
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::DeleteIndexerPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.prompt_confirm = false;
      }
      ActiveRadarrBlock::TestIndexer => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.indexer_test_errors = None;
      }
      _ => handle_clear_errors(self.app),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_radarr_block {
      ActiveRadarrBlock::Indexers => match self.key {
        _ if key == DEFAULT_KEYBINDINGS.refresh.key => {
          self.app.should_refresh = true;
        }
        _ if key == DEFAULT_KEYBINDINGS.test.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::TestIndexer.into());
        }
        _ if key == DEFAULT_KEYBINDINGS.test_all.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::TestAllIndexers.into());
        }
        _ if key == DEFAULT_KEYBINDINGS.settings.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::AllIndexerSettingsPrompt.into());
          self.app.data.radarr_data.selected_block =
            BlockSelectionState::new(INDEXER_SETTINGS_SELECTION_BLOCKS);
        }
        _ => (),
      },
      ActiveRadarrBlock::DeleteIndexerPrompt => {
        if key == DEFAULT_KEYBINDINGS.confirm.key {
          self.app.data.radarr_data.prompt_confirm = true;
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::DeleteIndexer(None));

          self.app.pop_navigation_stack();
        }
      }
      _ => (),
    }
  }
}
