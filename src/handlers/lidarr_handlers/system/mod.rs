use crate::app::App;
use crate::event::Key;
use crate::handlers::lidarr_handlers::handle_change_tab_left_right_keys;
use crate::handlers::lidarr_handlers::system::system_details_handler::SystemDetailsHandler;
use crate::handlers::{KeyEventHandler, handle_clear_errors};
use crate::matches_key;
use crate::models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock;
use crate::models::{Route, Scrollable};

mod system_details_handler;

#[cfg(test)]
#[path = "system_handler_tests.rs"]
mod system_handler_tests;

pub(super) struct SystemHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_lidarr_block: ActiveLidarrBlock,
  context: Option<ActiveLidarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveLidarrBlock> for SystemHandler<'a, 'b> {
  fn handle(&mut self) {
    match self.active_lidarr_block {
      _ if SystemDetailsHandler::accepts(self.active_lidarr_block) => {
        SystemDetailsHandler::new(self.key, self.app, self.active_lidarr_block, self.context)
          .handle()
      }
      _ => self.handle_key_event(),
    }
  }

  fn accepts(active_block: ActiveLidarrBlock) -> bool {
    SystemDetailsHandler::accepts(active_block) || active_block == ActiveLidarrBlock::System
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveLidarrBlock,
    context: Option<ActiveLidarrBlock>,
  ) -> SystemHandler<'a, 'b> {
    SystemHandler {
      key,
      app,
      active_lidarr_block: active_block,
      context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn is_ready(&self) -> bool {
    !self.app.is_loading
      && !self.app.data.lidarr_data.logs.is_empty()
      && !self.app.data.lidarr_data.queued_events.is_empty()
      && !self.app.data.lidarr_data.tasks.is_empty()
  }

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    if self.active_lidarr_block == ActiveLidarrBlock::System {
      handle_change_tab_left_right_keys(self.app, self.key);
    }
  }

  fn handle_submit(&mut self) {}

  fn handle_esc(&mut self) {
    handle_clear_errors(self.app)
  }

  fn handle_char_key_event(&mut self) {
    if self.active_lidarr_block == ActiveLidarrBlock::System {
      let key = self.key;
      match self.key {
        _ if matches_key!(refresh, key) => {
          self.app.should_refresh = true;
        }
        _ if matches_key!(events, key) => {
          self
            .app
            .push_navigation_stack(ActiveLidarrBlock::SystemQueuedEvents.into());
        }
        _ if matches_key!(logs, key) => {
          self
            .app
            .push_navigation_stack(ActiveLidarrBlock::SystemLogs.into());
          self
            .app
            .data
            .lidarr_data
            .log_details
            .set_items(self.app.data.lidarr_data.logs.items.to_vec());
          self.app.data.lidarr_data.log_details.scroll_to_bottom();
        }
        _ if matches_key!(tasks, key) => {
          self
            .app
            .push_navigation_stack(ActiveLidarrBlock::SystemTasks.into());
        }
        _ if matches_key!(update, key) => {
          self
            .app
            .push_navigation_stack(ActiveLidarrBlock::SystemUpdates.into());
        }
        _ => (),
      }
    }
  }

  fn app_mut(&mut self) -> &mut App<'b> {
    self.app
  }

  fn current_route(&self) -> Route {
    self.app.get_current_route()
  }
}
