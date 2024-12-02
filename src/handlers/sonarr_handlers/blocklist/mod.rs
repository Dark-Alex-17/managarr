use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::App;
use crate::event::Key;
use crate::handlers::sonarr_handlers::handle_change_tab_left_right_keys;
use crate::handlers::{handle_clear_errors, handle_prompt_toggle, KeyEventHandler};
use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, BLOCKLIST_BLOCKS};
use crate::models::sonarr_models::BlocklistItem;
use crate::models::stateful_table::SortOption;
use crate::models::Scrollable;
use crate::network::sonarr_network::SonarrEvent;

#[cfg(test)]
#[path = "blocklist_handler_tests.rs"]
mod blocklist_handler_tests;

pub(super) struct BlocklistHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_sonarr_block: ActiveSonarrBlock,
  _context: Option<ActiveSonarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveSonarrBlock> for BlocklistHandler<'a, 'b> {
  fn accepts(active_block: ActiveSonarrBlock) -> bool {
    BLOCKLIST_BLOCKS.contains(&active_block)
  }

  fn with(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveSonarrBlock,
    context: Option<ActiveSonarrBlock>,
  ) -> Self {
    BlocklistHandler {
      key,
      app,
      active_sonarr_block: active_block,
      _context: context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    !self.app.is_loading && !self.app.data.sonarr_data.blocklist.is_empty()
  }

  fn handle_scroll_up(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::Blocklist => self.app.data.sonarr_data.blocklist.scroll_up(),
      ActiveSonarrBlock::BlocklistSortPrompt => self
        .app
        .data
        .sonarr_data
        .blocklist
        .sort
        .as_mut()
        .unwrap()
        .scroll_up(),
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::Blocklist => self.app.data.sonarr_data.blocklist.scroll_down(),
      ActiveSonarrBlock::BlocklistSortPrompt => self
        .app
        .data
        .sonarr_data
        .blocklist
        .sort
        .as_mut()
        .unwrap()
        .scroll_down(),
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::Blocklist => self.app.data.sonarr_data.blocklist.scroll_to_top(),
      ActiveSonarrBlock::BlocklistSortPrompt => self
        .app
        .data
        .sonarr_data
        .blocklist
        .sort
        .as_mut()
        .unwrap()
        .scroll_to_top(),
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::Blocklist => self.app.data.sonarr_data.blocklist.scroll_to_bottom(),
      ActiveSonarrBlock::BlocklistSortPrompt => self
        .app
        .data
        .sonarr_data
        .blocklist
        .sort
        .as_mut()
        .unwrap()
        .scroll_to_bottom(),
      _ => (),
    }
  }

  fn handle_delete(&mut self) {
    if self.active_sonarr_block == ActiveSonarrBlock::Blocklist {
      self
        .app
        .push_navigation_stack(ActiveSonarrBlock::DeleteBlocklistItemPrompt.into());
    }
  }

  fn handle_left_right_action(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::Blocklist => handle_change_tab_left_right_keys(self.app, self.key),
      ActiveSonarrBlock::DeleteBlocklistItemPrompt
      | ActiveSonarrBlock::BlocklistClearAllItemsPrompt => handle_prompt_toggle(self.app, self.key),
      _ => {}
    }
  }

  fn handle_submit(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::DeleteBlocklistItemPrompt => {
        if self.app.data.sonarr_data.prompt_confirm {
          self.app.data.sonarr_data.prompt_confirm_action =
            Some(SonarrEvent::DeleteBlocklistItem(None));
        }

        self.app.pop_navigation_stack();
      }
      ActiveSonarrBlock::BlocklistClearAllItemsPrompt => {
        if self.app.data.sonarr_data.prompt_confirm {
          self.app.data.sonarr_data.prompt_confirm_action = Some(SonarrEvent::ClearBlocklist);
        }

        self.app.pop_navigation_stack();
      }
      ActiveSonarrBlock::BlocklistSortPrompt => {
        self
          .app
          .data
          .sonarr_data
          .blocklist
          .items
          .sort_by(|a, b| a.id.cmp(&b.id));
        self.app.data.sonarr_data.blocklist.apply_sorting();

        self.app.pop_navigation_stack();
      }
      ActiveSonarrBlock::Blocklist => {
        self
          .app
          .push_navigation_stack(ActiveSonarrBlock::BlocklistItemDetails.into());
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::DeleteBlocklistItemPrompt
      | ActiveSonarrBlock::BlocklistClearAllItemsPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.sonarr_data.prompt_confirm = false;
      }
      ActiveSonarrBlock::BlocklistItemDetails | ActiveSonarrBlock::BlocklistSortPrompt => {
        self.app.pop_navigation_stack();
      }
      _ => handle_clear_errors(self.app),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_sonarr_block {
      ActiveSonarrBlock::Blocklist => match self.key {
        _ if key == DEFAULT_KEYBINDINGS.refresh.key => {
          self.app.should_refresh = true;
        }
        _ if key == DEFAULT_KEYBINDINGS.clear.key => {
          self
            .app
            .push_navigation_stack(ActiveSonarrBlock::BlocklistClearAllItemsPrompt.into());
        }
        _ if key == DEFAULT_KEYBINDINGS.sort.key => {
          self
            .app
            .data
            .sonarr_data
            .blocklist
            .sorting(blocklist_sorting_options());
          self
            .app
            .push_navigation_stack(ActiveSonarrBlock::BlocklistSortPrompt.into());
        }
        _ => (),
      },
      ActiveSonarrBlock::DeleteBlocklistItemPrompt => {
        if key == DEFAULT_KEYBINDINGS.confirm.key {
          self.app.data.sonarr_data.prompt_confirm = true;
          self.app.data.sonarr_data.prompt_confirm_action =
            Some(SonarrEvent::DeleteBlocklistItem(None));

          self.app.pop_navigation_stack();
        }
      }
      ActiveSonarrBlock::BlocklistClearAllItemsPrompt => {
        if key == DEFAULT_KEYBINDINGS.confirm.key {
          self.app.data.sonarr_data.prompt_confirm = true;
          self.app.data.sonarr_data.prompt_confirm_action = Some(SonarrEvent::ClearBlocklist);

          self.app.pop_navigation_stack();
        }
      }
      _ => (),
    }
  }
}

fn blocklist_sorting_options() -> Vec<SortOption<BlocklistItem>> {
  vec![
    SortOption {
      name: "Series Title",
      cmp_fn: Some(|a, b| {
        a.series_title
          .as_ref()
          .unwrap_or(&String::new())
          .to_lowercase()
          .cmp(
            &b.series_title
              .as_ref()
              .unwrap_or(&String::new())
              .to_lowercase(),
          )
      }),
    },
    SortOption {
      name: "Source Title",
      cmp_fn: Some(|a, b| {
        a.source_title
          .to_lowercase()
          .cmp(&b.source_title.to_lowercase())
      }),
    },
    SortOption {
      name: "Language",
      cmp_fn: Some(|a, b| {
        a.language
          .name
          .to_lowercase()
          .cmp(&b.language.name.to_lowercase())
      }),
    },
    SortOption {
      name: "Quality",
      cmp_fn: Some(|a, b| {
        a.quality
          .quality
          .name
          .to_lowercase()
          .cmp(&b.quality.quality.name.to_lowercase())
      }),
    },
    SortOption {
      name: "Date",
      cmp_fn: Some(|a, b| a.date.cmp(&b.date)),
    },
  ]
}
