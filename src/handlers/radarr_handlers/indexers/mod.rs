use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::App;
use crate::event::Key;
use crate::handlers::radarr_handlers::handle_change_tab_left_right_keys;
use crate::handlers::radarr_handlers::indexers::edit_indexer_settings_handler::IndexerSettingsHandler;
use crate::handlers::radarr_handlers::indexers::test_all_indexers_handler::TestAllIndexersHandler;
use crate::handlers::{handle_clear_errors, handle_prompt_toggle, KeyEventHandler};
use crate::models::servarr_data::radarr::radarr_data::{
  ActiveRadarrBlock, INDEXERS_BLOCKS, INDEXER_SETTINGS_SELECTION_BLOCKS,
};
use crate::models::{BlockSelectionState, Scrollable};
use crate::network::radarr_network::RadarrEvent;

mod edit_indexer_settings_handler;
mod test_all_indexers_handler;

#[cfg(test)]
#[path = "indexers_handler_tests.rs"]
mod indexers_handler_tests;

pub(super) struct IndexersHandler<'a, 'b> {
  key: &'a Key,
  app: &'a mut App<'b>,
  active_radarr_block: &'a ActiveRadarrBlock,
  context: &'a Option<ActiveRadarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for IndexersHandler<'a, 'b> {
  fn handle(&mut self) {
    match self.active_radarr_block {
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

  fn accepts(active_block: &'a ActiveRadarrBlock) -> bool {
    IndexerSettingsHandler::accepts(active_block) || INDEXERS_BLOCKS.contains(active_block)
  }

  fn with(
    key: &'a Key,
    app: &'a mut App<'b>,
    active_block: &'a ActiveRadarrBlock,
    context: &'a Option<ActiveRadarrBlock>,
  ) -> IndexersHandler<'a, 'b> {
    IndexersHandler {
      key,
      app,
      active_radarr_block: active_block,
      context,
    }
  }

  fn get_key(&self) -> &Key {
    self.key
  }

  fn handle_scroll_up(&mut self) {
    if self.active_radarr_block == &ActiveRadarrBlock::Indexers {
      self.app.data.radarr_data.indexers.scroll_up();
    }
  }

  fn handle_scroll_down(&mut self) {
    if self.active_radarr_block == &ActiveRadarrBlock::Indexers {
      self.app.data.radarr_data.indexers.scroll_down();
    }
  }

  fn handle_home(&mut self) {
    if self.active_radarr_block == &ActiveRadarrBlock::Indexers {
      self.app.data.radarr_data.indexers.scroll_to_top();
    }
  }

  fn handle_end(&mut self) {
    if self.active_radarr_block == &ActiveRadarrBlock::Indexers {
      self.app.data.radarr_data.indexers.scroll_to_bottom();
    }
  }

  fn handle_delete(&mut self) {
    if self.active_radarr_block == &ActiveRadarrBlock::Indexers {
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
          radarr_data.prompt_confirm_action = Some(RadarrEvent::DeleteIndexer);
        }

        self.app.pop_navigation_stack();
      }
      ActiveRadarrBlock::Indexers => {
        self
          .app
          .push_navigation_stack(ActiveRadarrBlock::EditIndexer.into());
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
      _ => handle_clear_errors(self.app),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    if self.active_radarr_block == &ActiveRadarrBlock::Indexers {
      match self.key {
        _ if *key == DEFAULT_KEYBINDINGS.add.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::AddIndexer.into());
        }
        _ if *key == DEFAULT_KEYBINDINGS.refresh.key => {
          self.app.should_refresh = true;
        }
        _ if *key == DEFAULT_KEYBINDINGS.test.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::TestAllIndexers.into());
        }
        _ if *key == DEFAULT_KEYBINDINGS.settings.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::IndexerSettingsPrompt.into());
          self.app.data.radarr_data.selected_block =
            BlockSelectionState::new(&INDEXER_SETTINGS_SELECTION_BLOCKS);
        }
        _ => (),
      }
    }
  }
}
