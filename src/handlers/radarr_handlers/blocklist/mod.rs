use crate::app::App;
use crate::event::Key;
use crate::handlers::radarr_handlers::handle_change_tab_left_right_keys;
use crate::handlers::table_handler::{TableHandlingConfig, handle_table};
use crate::handlers::{KeyEventHandler, handle_clear_errors, handle_prompt_toggle};
use crate::matches_key;
use crate::models::radarr_models::BlocklistItem;
use crate::models::Route;
use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, BLOCKLIST_BLOCKS};
use crate::models::stateful_table::SortOption;
use crate::network::radarr_network::RadarrEvent;

#[cfg(test)]
#[path = "blocklist_handler_tests.rs"]
mod blocklist_handler_tests;

pub(super) struct BlocklistHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_radarr_block: ActiveRadarrBlock,
  _context: Option<ActiveRadarrBlock>,
}

impl BlocklistHandler<'_, '_> {
  fn extract_blocklist_item_id(&self) -> i64 {
    self.app.data.radarr_data.blocklist.current_selection().id
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for BlocklistHandler<'a, 'b> {
  fn handle(&mut self) {
    let blocklist_table_handling_config =
      TableHandlingConfig::new(ActiveRadarrBlock::Blocklist.into())
        .sorting_block(ActiveRadarrBlock::BlocklistSortPrompt.into())
        .sort_options(blocklist_sorting_options());

    if !handle_table(
      self,
      |app| &mut app.data.radarr_data.blocklist,
      blocklist_table_handling_config,
    ) {
      self.handle_key_event();
    }
  }

  fn accepts(active_block: ActiveRadarrBlock) -> bool {
    BLOCKLIST_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveRadarrBlock,
    context: Option<ActiveRadarrBlock>,
  ) -> Self {
    BlocklistHandler {
      key,
      app,
      active_radarr_block: active_block,
      _context: context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    !self.app.is_loading && !self.app.data.radarr_data.blocklist.is_empty()
  }

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {
    if self.active_radarr_block == ActiveRadarrBlock::Blocklist {
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
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::DeleteBlocklistItem(
            self.extract_blocklist_item_id(),
          ));
        }

        self.app.pop_navigation_stack();
      }
      ActiveRadarrBlock::BlocklistClearAllItemsPrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::ClearBlocklist);
        }

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
      ActiveRadarrBlock::BlocklistItemDetails => {
        self.app.pop_navigation_stack();
      }
      _ => handle_clear_errors(self.app),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_radarr_block {
      ActiveRadarrBlock::Blocklist => match self.key {
        _ if matches_key!(refresh, key) => {
          self.app.should_refresh = true;
        }
        _ if matches_key!(clear, key) => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::BlocklistClearAllItemsPrompt.into());
        }
        _ => (),
      },
      ActiveRadarrBlock::DeleteBlocklistItemPrompt => {
        if matches_key!(confirm, key) {
          self.app.data.radarr_data.prompt_confirm = true;
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::DeleteBlocklistItem(
            self.extract_blocklist_item_id(),
          ));

          self.app.pop_navigation_stack();
        }
      }
      ActiveRadarrBlock::BlocklistClearAllItemsPrompt => {
        if matches_key!(confirm, key) {
          self.app.data.radarr_data.prompt_confirm = true;
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::ClearBlocklist);

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
