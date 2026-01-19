use crate::app::App;
use crate::event::Key;
use crate::handlers::lidarr_handlers::handle_change_tab_left_right_keys;
use crate::handlers::table_handler::{TableHandlingConfig, handle_table};
use crate::handlers::{KeyEventHandler, handle_clear_errors, handle_prompt_toggle};
use crate::matches_key;
use crate::models::Route;
use crate::models::lidarr_models::BlocklistItem;
use crate::models::servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, BLOCKLIST_BLOCKS};
use crate::models::stateful_table::SortOption;
use crate::network::lidarr_network::LidarrEvent;

#[cfg(test)]
#[path = "blocklist_handler_tests.rs"]
mod blocklist_handler_tests;

pub(super) struct BlocklistHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_lidarr_block: ActiveLidarrBlock,
  _context: Option<ActiveLidarrBlock>,
}

impl BlocklistHandler<'_, '_> {
  fn extract_blocklist_item_id(&self) -> i64 {
    self.app.data.lidarr_data.blocklist.current_selection().id
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveLidarrBlock> for BlocklistHandler<'a, 'b> {
  fn handle(&mut self) {
    let blocklist_table_handling_config =
      TableHandlingConfig::new(ActiveLidarrBlock::Blocklist.into())
        .sorting_block(ActiveLidarrBlock::BlocklistSortPrompt.into())
        .sort_options(blocklist_sorting_options());

    if !handle_table(
      self,
      |app| &mut app.data.lidarr_data.blocklist,
      blocklist_table_handling_config,
    ) {
      self.handle_key_event();
    }
  }

  fn accepts(active_block: ActiveLidarrBlock) -> bool {
    BLOCKLIST_BLOCKS.contains(&active_block)
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveLidarrBlock,
    context: Option<ActiveLidarrBlock>,
  ) -> Self {
    BlocklistHandler {
      key,
      app,
      active_lidarr_block: active_block,
      _context: context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn is_ready(&self) -> bool {
    !self.app.is_loading && !self.app.data.lidarr_data.blocklist.is_empty()
  }

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {
    if self.active_lidarr_block == ActiveLidarrBlock::Blocklist {
      self
        .app
        .push_navigation_stack(ActiveLidarrBlock::DeleteBlocklistItemPrompt.into());
    }
  }

  fn handle_left_right_action(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::Blocklist => handle_change_tab_left_right_keys(self.app, self.key),
      ActiveLidarrBlock::DeleteBlocklistItemPrompt
      | ActiveLidarrBlock::BlocklistClearAllItemsPrompt => handle_prompt_toggle(self.app, self.key),
      _ => {}
    }
  }

  fn handle_submit(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::DeleteBlocklistItemPrompt => {
        if self.app.data.lidarr_data.prompt_confirm {
          self.app.data.lidarr_data.prompt_confirm_action = Some(LidarrEvent::DeleteBlocklistItem(
            self.extract_blocklist_item_id(),
          ));
        }

        self.app.pop_navigation_stack();
      }
      ActiveLidarrBlock::BlocklistClearAllItemsPrompt => {
        if self.app.data.lidarr_data.prompt_confirm {
          self.app.data.lidarr_data.prompt_confirm_action = Some(LidarrEvent::ClearBlocklist);
        }

        self.app.pop_navigation_stack();
      }
      ActiveLidarrBlock::Blocklist => {
        self
          .app
          .push_navigation_stack(ActiveLidarrBlock::BlocklistItemDetails.into());
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::DeleteBlocklistItemPrompt
      | ActiveLidarrBlock::BlocklistClearAllItemsPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.lidarr_data.prompt_confirm = false;
      }
      ActiveLidarrBlock::BlocklistItemDetails | ActiveLidarrBlock::BlocklistSortPrompt => {
        self.app.pop_navigation_stack();
      }
      _ => handle_clear_errors(self.app),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_lidarr_block {
      ActiveLidarrBlock::Blocklist => match self.key {
        _ if matches_key!(refresh, key) => {
          self.app.should_refresh = true;
        }
        _ if matches_key!(clear, key) => {
          self
            .app
            .push_navigation_stack(ActiveLidarrBlock::BlocklistClearAllItemsPrompt.into());
        }
        _ => (),
      },
      ActiveLidarrBlock::DeleteBlocklistItemPrompt => {
        if matches_key!(confirm, key) {
          self.app.data.lidarr_data.prompt_confirm = true;
          self.app.data.lidarr_data.prompt_confirm_action = Some(LidarrEvent::DeleteBlocklistItem(
            self.extract_blocklist_item_id(),
          ));

          self.app.pop_navigation_stack();
        }
      }
      ActiveLidarrBlock::BlocklistClearAllItemsPrompt => {
        if matches_key!(confirm, key) {
          self.app.data.lidarr_data.prompt_confirm = true;
          self.app.data.lidarr_data.prompt_confirm_action = Some(LidarrEvent::ClearBlocklist);

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
      name: "Artist Name",
      cmp_fn: Some(|a, b| {
        a.artist
          .artist_name
          .text
          .to_lowercase()
          .cmp(&b.artist.artist_name.text.to_lowercase())
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
