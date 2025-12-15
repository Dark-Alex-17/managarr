use crate::app::App;
use crate::event::Key;
use crate::handlers::sonarr_handlers::handle_change_tab_left_right_keys;
use crate::handlers::table_handler::{TableHandlingConfig, handle_table};
use crate::handlers::{KeyEventHandler, handle_clear_errors, handle_prompt_toggle};
use crate::matches_key;
use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, BLOCKLIST_BLOCKS};
use crate::models::sonarr_models::BlocklistItem;
use crate::models::stateful_table::SortOption;
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

impl BlocklistHandler<'_, '_> {
  fn extract_blocklist_item_id(&self) -> i64 {
    self.app.data.sonarr_data.blocklist.current_selection().id
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveSonarrBlock> for BlocklistHandler<'a, 'b> {
  fn handle(&mut self) {
    let blocklist_table_handling_config =
      TableHandlingConfig::new(ActiveSonarrBlock::Blocklist.into())
        .sorting_block(ActiveSonarrBlock::BlocklistSortPrompt.into())
        .sort_options(blocklist_sorting_options());

    if !handle_table(
      self,
      |app| &mut app.data.sonarr_data.blocklist,
      blocklist_table_handling_config,
    ) {
      self.handle_key_event();
    }
  }

  fn accepts(active_block: ActiveSonarrBlock) -> bool {
    BLOCKLIST_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
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

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

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
          self.app.data.sonarr_data.prompt_confirm_action = Some(SonarrEvent::DeleteBlocklistItem(
            self.extract_blocklist_item_id(),
          ));
        }

        self.app.pop_navigation_stack();
      }
      ActiveSonarrBlock::BlocklistClearAllItemsPrompt => {
        if self.app.data.sonarr_data.prompt_confirm {
          self.app.data.sonarr_data.prompt_confirm_action = Some(SonarrEvent::ClearBlocklist);
        }

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
        _ if matches_key!(refresh, key) => {
          self.app.should_refresh = true;
        }
        _ if matches_key!(clear, key) => {
          self
            .app
            .push_navigation_stack(ActiveSonarrBlock::BlocklistClearAllItemsPrompt.into());
        }
        _ => (),
      },
      ActiveSonarrBlock::DeleteBlocklistItemPrompt => {
        if matches_key!(confirm, key) {
          self.app.data.sonarr_data.prompt_confirm = true;
          self.app.data.sonarr_data.prompt_confirm_action = Some(SonarrEvent::DeleteBlocklistItem(
            self.extract_blocklist_item_id(),
          ));

          self.app.pop_navigation_stack();
        }
      }
      ActiveSonarrBlock::BlocklistClearAllItemsPrompt => {
        if matches_key!(confirm, key) {
          self.app.data.sonarr_data.prompt_confirm = true;
          self.app.data.sonarr_data.prompt_confirm_action = Some(SonarrEvent::ClearBlocklist);

          self.app.pop_navigation_stack();
        }
      }
      _ => (),
    }
  }

  fn app_mut(&mut self) -> &mut App<'b> {
    self.app
  }

  fn current_route(&self) -> crate::models::Route {
    self.app.get_current_route()
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
        let a_languages = a
          .languages
          .iter()
          .map(|lang| {
            lang
              .as_ref()
              .unwrap_or(&Default::default())
              .name
              .to_lowercase()
          })
          .collect::<Vec<String>>()
          .join(", ");
        let b_languages = b
          .languages
          .iter()
          .map(|lang| {
            lang
              .as_ref()
              .unwrap_or(&Default::default())
              .name
              .to_lowercase()
          })
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
      name: "Date",
      cmp_fn: Some(|a, b| a.date.cmp(&b.date)),
    },
  ]
}
