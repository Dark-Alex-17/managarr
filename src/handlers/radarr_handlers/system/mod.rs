use crate::app::App;
use crate::event::Key;
use crate::handlers::radarr_handlers::handle_change_tab_left_right_keys;
use crate::handlers::radarr_handlers::system::system_details_handler::SystemDetailsHandler;
use crate::handlers::{KeyEventHandler, handle_clear_errors};
use crate::matches_key;
use crate::models::Scrollable;
use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;

mod system_details_handler;

#[cfg(test)]
#[path = "system_handler_tests.rs"]
mod system_handler_tests;

pub(super) struct SystemHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_radarr_block: ActiveRadarrBlock,
  context: Option<ActiveRadarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for SystemHandler<'a, 'b> {
  fn handle(&mut self) {
    match self.active_radarr_block {
      _ if SystemDetailsHandler::accepts(self.active_radarr_block) => {
        SystemDetailsHandler::new(self.key, self.app, self.active_radarr_block, self.context)
          .handle()
      }
      _ => self.handle_key_event(),
    }
  }

  fn accepts(active_block: ActiveRadarrBlock) -> bool {
    SystemDetailsHandler::accepts(active_block) || active_block == ActiveRadarrBlock::System
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveRadarrBlock,
    context: Option<ActiveRadarrBlock>,
  ) -> SystemHandler<'a, 'b> {
    SystemHandler {
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
    !self.app.is_loading
      && !self.app.data.radarr_data.logs.is_empty()
      && !self.app.data.radarr_data.queued_events.is_empty()
      && !self.app.data.radarr_data.tasks.is_empty()
  }

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    if self.active_radarr_block == ActiveRadarrBlock::System {
      handle_change_tab_left_right_keys(self.app, self.key);
    }
  }

  fn handle_submit(&mut self) {}

  fn handle_esc(&mut self) {
    handle_clear_errors(self.app)
  }

  fn handle_char_key_event(&mut self) {
    if self.active_radarr_block == ActiveRadarrBlock::System {
      let key = self.key;
      match self.key {
        _ if matches_key!(refresh, key) => {
          self.app.should_refresh = true;
        }
        _ if matches_key!(events, key) => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::SystemQueuedEvents.into());
        }
        _ if matches_key!(logs, key) => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::SystemLogs.into());
          self
            .app
            .data
            .radarr_data
            .log_details
            .set_items(self.app.data.radarr_data.logs.items.to_vec());
          self.app.data.radarr_data.log_details.scroll_to_bottom();
        }
        _ if matches_key!(tasks, key) => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::SystemTasks.into());
        }
        _ if matches_key!(update, key) => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::SystemUpdates.into());
        }
        _ => (),
      }
    }
  }

  fn app_mut(&mut self) -> &mut App<'b> {
    self.app
  }

  fn current_route(&self) -> crate::models::Route {
    self.app.get_current_route()
  }
}
