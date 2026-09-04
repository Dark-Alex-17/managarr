use crate::app::App;
use crate::event::Key;
use crate::handlers::readarr_handlers::handle_change_tab_left_right_keys;
use crate::handlers::table_handler::{TableHandlingConfig, handle_table};
use crate::handlers::{KeyEventHandler, handle_clear_errors, handle_prompt_toggle};
use crate::matches_key;
use crate::models::Route;
use crate::models::readarr_models::BlocklistItem;
use crate::models::servarr_data::readarr::readarr_data::{ActiveReadarrBlock, BLOCKLIST_BLOCKS};
use crate::models::stateful_table::SortOption;
use crate::network::readarr_network::ReadarrEvent;

#[cfg(test)]
#[path = "blocklist_handler_tests.rs"]
mod blocklist_handler_tests;

pub(super) struct BlocklistHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_readarr_block: ActiveReadarrBlock,
  _context: Option<ActiveReadarrBlock>,
}

impl BlocklistHandler<'_, '_> {
  fn extract_blocklist_item_id(&self) -> i64 {
    self.app.data.readarr_data.blocklist.current_selection().id
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveReadarrBlock> for BlocklistHandler<'a, 'b> {
  fn handle(&mut self) {
    let blocklist_table_handling_config =
      TableHandlingConfig::new(ActiveReadarrBlock::Blocklist.into())
        .sorting_block(ActiveReadarrBlock::BlocklistSortPrompt.into())
        .sort_options(blocklist_sorting_options());

    if !handle_table(
      self,
      |app| &mut app.data.readarr_data.blocklist,
      blocklist_table_handling_config,
    ) {
      self.handle_key_event();
    }
  }

  fn accepts(active_block: ActiveReadarrBlock) -> bool {
    BLOCKLIST_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveReadarrBlock,
    context: Option<ActiveReadarrBlock>,
  ) -> Self {
    BlocklistHandler {
      key,
      app,
      active_readarr_block: active_block,
      _context: context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    !self.app.is_loading && !self.app.data.readarr_data.blocklist.is_empty()
  }

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {
    if self.active_readarr_block == ActiveReadarrBlock::Blocklist {
      self
        .app
        .push_navigation_stack(ActiveReadarrBlock::DeleteBlocklistItemPrompt.into());
    }
  }

  fn handle_left_right_action(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::Blocklist => handle_change_tab_left_right_keys(self.app, self.key),
      ActiveReadarrBlock::DeleteBlocklistItemPrompt
      | ActiveReadarrBlock::BlocklistClearAllItemsPrompt => {
        handle_prompt_toggle(self.app, self.key)
      }
      _ => {}
    }
  }

  fn handle_submit(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::DeleteBlocklistItemPrompt => {
        if self.app.data.readarr_data.prompt_confirm {
          self.app.data.readarr_data.prompt_confirm_action = Some(
            ReadarrEvent::DeleteBlocklistItem(self.extract_blocklist_item_id()),
          );
        }

        self.app.pop_navigation_stack();
      }
      ActiveReadarrBlock::BlocklistClearAllItemsPrompt => {
        if self.app.data.readarr_data.prompt_confirm {
          self.app.data.readarr_data.prompt_confirm_action = Some(ReadarrEvent::ClearBlocklist);
        }

        self.app.pop_navigation_stack();
      }
      ActiveReadarrBlock::Blocklist => {
        self
          .app
          .push_navigation_stack(ActiveReadarrBlock::BlocklistItemDetails.into());
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::DeleteBlocklistItemPrompt
      | ActiveReadarrBlock::BlocklistClearAllItemsPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.readarr_data.prompt_confirm = false;
      }
      ActiveReadarrBlock::BlocklistItemDetails | ActiveReadarrBlock::BlocklistSortPrompt => {
        self.app.pop_navigation_stack();
      }
      _ => handle_clear_errors(self.app),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_readarr_block {
      ActiveReadarrBlock::Blocklist => match self.key {
        _ if matches_key!(refresh, key) => {
          self.app.should_refresh = true;
        }
        _ if matches_key!(clear, key) => {
          self
            .app
            .push_navigation_stack(ActiveReadarrBlock::BlocklistClearAllItemsPrompt.into());
        }
        _ => (),
      },
      ActiveReadarrBlock::DeleteBlocklistItemPrompt => {
        if matches_key!(confirm, key) {
          self.app.data.readarr_data.prompt_confirm = true;
          self.app.data.readarr_data.prompt_confirm_action = Some(
            ReadarrEvent::DeleteBlocklistItem(self.extract_blocklist_item_id()),
          );

          self.app.pop_navigation_stack();
        }
      }
      ActiveReadarrBlock::BlocklistClearAllItemsPrompt => {
        if matches_key!(confirm, key) {
          self.app.data.readarr_data.prompt_confirm = true;
          self.app.data.readarr_data.prompt_confirm_action = Some(ReadarrEvent::ClearBlocklist);

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
      name: "Author Name",
      cmp_fn: Some(|a, b| {
        a.author
          .author_name
          .text
          .to_lowercase()
          .cmp(&b.author.author_name.text.to_lowercase())
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
