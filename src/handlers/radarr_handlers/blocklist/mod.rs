use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::App;
use crate::event::Key;
use crate::handlers::radarr_handlers::handle_change_tab_left_right_keys;
use crate::handlers::{handle_clear_errors, handle_prompt_toggle, KeyEventHandler};
use crate::models::radarr_models::BlocklistItem;
use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, BLOCKLIST_BLOCKS};
use crate::models::stateful_table::SortOption;
use crate::models::Scrollable;
use crate::network::radarr_network::RadarrEvent;

#[cfg(test)]
#[path = "blocklist_handler_tests.rs"]
mod blocklist_handler_tests;

pub(super) struct BlocklistHandler<'a, 'b> {
  key: &'a Key,
  app: &'a mut App<'b>,
  active_radarr_block: &'a ActiveRadarrBlock,
  _context: &'a Option<ActiveRadarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for BlocklistHandler<'a, 'b> {
  fn accepts(active_block: &'a ActiveRadarrBlock) -> bool {
    BLOCKLIST_BLOCKS.contains(active_block)
  }

  fn with(
    key: &'a Key,
    app: &'a mut App<'b>,
    active_block: &'a ActiveRadarrBlock,
    context: &'a Option<ActiveRadarrBlock>,
  ) -> Self {
    BlocklistHandler {
      key,
      app,
      active_radarr_block: active_block,
      _context: context,
    }
  }

  fn get_key(&self) -> &Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    !self.app.is_loading && !self.app.data.radarr_data.blocklist.is_empty()
  }

  fn handle_scroll_up(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Blocklist => self.app.data.radarr_data.blocklist.scroll_up(),
      ActiveRadarrBlock::BlocklistSortPrompt => self
        .app
        .data
        .radarr_data
        .blocklist
        .sort
        .as_mut()
        .unwrap()
        .scroll_up(),
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Blocklist => self.app.data.radarr_data.blocklist.scroll_down(),
      ActiveRadarrBlock::BlocklistSortPrompt => self
        .app
        .data
        .radarr_data
        .blocklist
        .sort
        .as_mut()
        .unwrap()
        .scroll_down(),
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Blocklist => self.app.data.radarr_data.blocklist.scroll_to_top(),
      ActiveRadarrBlock::BlocklistSortPrompt => self
        .app
        .data
        .radarr_data
        .blocklist
        .sort
        .as_mut()
        .unwrap()
        .scroll_to_top(),
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Blocklist => self.app.data.radarr_data.blocklist.scroll_to_bottom(),
      ActiveRadarrBlock::BlocklistSortPrompt => self
        .app
        .data
        .radarr_data
        .blocklist
        .sort
        .as_mut()
        .unwrap()
        .scroll_to_bottom(),
      _ => (),
    }
  }

  fn handle_delete(&mut self) {
    if self.active_radarr_block == &ActiveRadarrBlock::Blocklist {
      self
        .app
        .push_navigation_stack(ActiveRadarrBlock::DeleteBlocklistItemPrompt.into());
    }
  }

  fn handle_left_right_action(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Blocklist => handle_change_tab_left_right_keys(self.app, self.key),
      ActiveRadarrBlock::DeleteBlocklistItemPrompt
      | ActiveRadarrBlock::BlocklistClearAllItemsPrompt => handle_prompt_toggle(self.app, self.key),
      _ => {}
    }
  }

  fn handle_submit(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::DeleteBlocklistItemPrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::DeleteBlocklistItem);
        }

        self.app.pop_navigation_stack();
      }
      ActiveRadarrBlock::BlocklistClearAllItemsPrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::ClearBlocklist);
        }

        self.app.pop_navigation_stack();
      }
      ActiveRadarrBlock::BlocklistSortPrompt => {
        self
          .app
          .data
          .radarr_data
          .blocklist
          .items
          .sort_by(|a, b| a.id.cmp(&b.id));
        self.app.data.radarr_data.blocklist.apply_sorting();

        self.app.pop_navigation_stack();
      }
      ActiveRadarrBlock::Blocklist => {
        self
          .app
          .push_navigation_stack(ActiveRadarrBlock::BlocklistItemDetails.into());
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::DeleteBlocklistItemPrompt
      | ActiveRadarrBlock::BlocklistClearAllItemsPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.prompt_confirm = false;
      }
      ActiveRadarrBlock::BlocklistItemDetails | ActiveRadarrBlock::BlocklistSortPrompt => {
        self.app.pop_navigation_stack();
      }
      _ => handle_clear_errors(self.app),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    if self.active_radarr_block == &ActiveRadarrBlock::Blocklist {
      match self.key {
        _ if *key == DEFAULT_KEYBINDINGS.refresh.key => {
          self.app.should_refresh = true;
        }
        _ if *key == DEFAULT_KEYBINDINGS.clear.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::BlocklistClearAllItemsPrompt.into());
        }
        _ if *key == DEFAULT_KEYBINDINGS.sort.key => {
          self
            .app
            .data
            .radarr_data
            .blocklist
            .sorting(blocklist_sorting_options());
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::BlocklistSortPrompt.into());
        }
        _ => (),
      }
    }
  }
}

fn blocklist_sorting_options() -> Vec<SortOption<BlocklistItem>> {
  vec![
    SortOption {
      name: "Movie Title",
      cmp_fn: Some(|a, b| {
        a.movie
          .title
          .text
          .to_lowercase()
          .cmp(&b.movie.title.text.to_lowercase())
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
      name: "Languages",
      cmp_fn: Some(|a, b| {
        let a_languages = a
          .languages
          .iter()
          .map(|lang| lang.name.to_lowercase())
          .collect::<Vec<String>>()
          .join(", ");
        let b_languages = b
          .languages
          .iter()
          .map(|lang| lang.name.to_lowercase())
          .collect::<Vec<String>>()
          .join(", ");

        a_languages.cmp(&b_languages)
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
      name: "Formats",
      cmp_fn: Some(|a, b| {
        let a_custom_formats = a
          .custom_formats
          .as_ref()
          .unwrap_or(&Vec::new())
          .iter()
          .map(|lang| lang.name.to_lowercase())
          .collect::<Vec<String>>()
          .join(", ");
        let b_custom_formats = b
          .custom_formats
          .as_ref()
          .unwrap_or(&Vec::new())
          .iter()
          .map(|lang| lang.name.to_lowercase())
          .collect::<Vec<String>>()
          .join(", ");

        a_custom_formats.cmp(&b_custom_formats)
      }),
    },
    SortOption {
      name: "Date",
      cmp_fn: Some(|a, b| a.date.cmp(&b.date)),
    },
  ]
}
