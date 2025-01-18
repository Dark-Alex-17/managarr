use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::App;
use crate::event::Key;
use crate::handlers::sonarr_handlers::handle_change_tab_left_right_keys;
use crate::handlers::sonarr_handlers::system::system_details_handler::SystemDetailsHandler;
use crate::handlers::{handle_clear_errors, KeyEventHandler};
use crate::models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock;
use crate::models::Scrollable;

mod system_details_handler;

#[cfg(test)]
#[path = "system_handler_tests.rs"]
mod system_handler_tests;

pub(super) struct SystemHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_sonarr_block: ActiveSonarrBlock,
  context: Option<ActiveSonarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveSonarrBlock> for SystemHandler<'a, 'b> {
  fn handle(&mut self) {
    match self.active_sonarr_block {
      _ if SystemDetailsHandler::accepts(self.active_sonarr_block) => {
        SystemDetailsHandler::new(self.key, self.app, self.active_sonarr_block, self.context)
          .handle()
      }
      _ => self.handle_key_event(),
    }
  }

  fn accepts(active_block: ActiveSonarrBlock) -> bool {
    SystemDetailsHandler::accepts(active_block) || active_block == ActiveSonarrBlock::System
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveSonarrBlock,
    context: Option<ActiveSonarrBlock>,
  ) -> SystemHandler<'a, 'b> {
    SystemHandler {
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
    !self.app.is_loading
      && !self.app.data.sonarr_data.logs.is_empty()
      && !self.app.data.sonarr_data.queued_events.is_empty()
      && !self.app.data.sonarr_data.tasks.is_empty()
  }

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    if self.active_sonarr_block == ActiveSonarrBlock::System {
      handle_change_tab_left_right_keys(self.app, self.key);
    }
  }

  fn handle_submit(&mut self) {}

  fn handle_esc(&mut self) {
    handle_clear_errors(self.app)
  }

  fn handle_char_key_event(&mut self) {
    if self.active_sonarr_block == ActiveSonarrBlock::System {
      let key = self.key;
      match self.key {
        _ if key == DEFAULT_KEYBINDINGS.refresh.key => {
          self.app.should_refresh = true;
        }
        _ if key == DEFAULT_KEYBINDINGS.events.key => {
          self
            .app
            .push_navigation_stack(ActiveSonarrBlock::SystemQueuedEvents.into());
        }
        _ if key == DEFAULT_KEYBINDINGS.logs.key => {
          self
            .app
            .push_navigation_stack(ActiveSonarrBlock::SystemLogs.into());
          self
            .app
            .data
            .sonarr_data
            .log_details
            .set_items(self.app.data.sonarr_data.logs.items.to_vec());
          self.app.data.sonarr_data.log_details.scroll_to_bottom();
        }
        _ if key == DEFAULT_KEYBINDINGS.tasks.key => {
          self
            .app
            .push_navigation_stack(ActiveSonarrBlock::SystemTasks.into());
        }
        _ if key == DEFAULT_KEYBINDINGS.update.key => {
          self
            .app
            .push_navigation_stack(ActiveSonarrBlock::SystemUpdates.into());
        }
        _ => (),
      }
    }
  }
}
