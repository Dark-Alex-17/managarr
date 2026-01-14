use crate::app::App;
use crate::event::Key;
use crate::handlers::{KeyEventHandler, handle_prompt_toggle};
use crate::matches_key;
use crate::models::lidarr_models::LidarrTaskName;
use crate::models::servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, SYSTEM_DETAILS_BLOCKS};
use crate::models::stateful_list::StatefulList;
use crate::models::{Route, Scrollable};
use crate::network::lidarr_network::LidarrEvent;

#[cfg(test)]
#[path = "system_details_handler_tests.rs"]
mod system_details_handler_tests;

pub(super) struct SystemDetailsHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_lidarr_block: ActiveLidarrBlock,
  _context: Option<ActiveLidarrBlock>,
}

impl SystemDetailsHandler<'_, '_> {
  fn extract_task_name(&self) -> LidarrTaskName {
    self
      .app
      .data
      .lidarr_data
      .tasks
      .current_selection()
      .task_name
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveLidarrBlock> for SystemDetailsHandler<'a, 'b> {
  fn accepts(active_block: ActiveLidarrBlock) -> bool {
    SYSTEM_DETAILS_BLOCKS.contains(&active_block)
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveLidarrBlock,
    context: Option<ActiveLidarrBlock>,
  ) -> SystemDetailsHandler<'a, 'b> {
    SystemDetailsHandler {
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
    !self.app.is_loading
      && (!self.app.data.lidarr_data.log_details.is_empty()
        || !self.app.data.lidarr_data.tasks.is_empty()
        || !self.app.data.lidarr_data.updates.is_empty())
  }

  fn handle_scroll_up(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::SystemLogs => self.app.data.lidarr_data.log_details.scroll_up(),
      ActiveLidarrBlock::SystemTasks => self.app.data.lidarr_data.tasks.scroll_up(),
      ActiveLidarrBlock::SystemUpdates => self.app.data.lidarr_data.updates.scroll_up(),
      ActiveLidarrBlock::SystemQueuedEvents => self.app.data.lidarr_data.queued_events.scroll_up(),
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::SystemLogs => self.app.data.lidarr_data.log_details.scroll_down(),
      ActiveLidarrBlock::SystemTasks => self.app.data.lidarr_data.tasks.scroll_down(),
      ActiveLidarrBlock::SystemUpdates => self.app.data.lidarr_data.updates.scroll_down(),
      ActiveLidarrBlock::SystemQueuedEvents => {
        self.app.data.lidarr_data.queued_events.scroll_down()
      }
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::SystemLogs => self.app.data.lidarr_data.log_details.scroll_to_top(),
      ActiveLidarrBlock::SystemTasks => self.app.data.lidarr_data.tasks.scroll_to_top(),
      ActiveLidarrBlock::SystemUpdates => self.app.data.lidarr_data.updates.scroll_to_top(),
      ActiveLidarrBlock::SystemQueuedEvents => {
        self.app.data.lidarr_data.queued_events.scroll_to_top()
      }
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::SystemLogs => self.app.data.lidarr_data.log_details.scroll_to_bottom(),
      ActiveLidarrBlock::SystemTasks => self.app.data.lidarr_data.tasks.scroll_to_bottom(),
      ActiveLidarrBlock::SystemUpdates => self.app.data.lidarr_data.updates.scroll_to_bottom(),
      ActiveLidarrBlock::SystemQueuedEvents => {
        self.app.data.lidarr_data.queued_events.scroll_to_bottom()
      }
      _ => (),
    }
  }

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    let key = self.key;

    match self.active_lidarr_block {
      ActiveLidarrBlock::SystemLogs => match self.key {
        _ if matches_key!(left, key) => {
          self
            .app
            .data
            .lidarr_data
            .log_details
            .items
            .iter()
            .for_each(|log| log.scroll_right());
        }
        _ if matches_key!(right, key) => {
          self
            .app
            .data
            .lidarr_data
            .log_details
            .items
            .iter()
            .for_each(|log| log.scroll_left());
        }
        _ => (),
      },
      ActiveLidarrBlock::SystemTaskStartConfirmPrompt => handle_prompt_toggle(self.app, self.key),
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::SystemTasks => {
        self
          .app
          .push_navigation_stack(ActiveLidarrBlock::SystemTaskStartConfirmPrompt.into());
      }
      ActiveLidarrBlock::SystemTaskStartConfirmPrompt => {
        if self.app.data.lidarr_data.prompt_confirm {
          self.app.data.lidarr_data.prompt_confirm_action =
            Some(LidarrEvent::StartTask(self.extract_task_name()));
        }

        self.app.pop_navigation_stack();
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::SystemLogs => {
        self.app.data.lidarr_data.log_details = StatefulList::default();
        self.app.pop_navigation_stack()
      }
      ActiveLidarrBlock::SystemQueuedEvents
      | ActiveLidarrBlock::SystemTasks
      | ActiveLidarrBlock::SystemUpdates => self.app.pop_navigation_stack(),
      ActiveLidarrBlock::SystemTaskStartConfirmPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.lidarr_data.prompt_confirm = false;
      }
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    if SYSTEM_DETAILS_BLOCKS.contains(&self.active_lidarr_block) && matches_key!(refresh, self.key)
    {
      self.app.should_refresh = true;
    }

    if self.active_lidarr_block == ActiveLidarrBlock::SystemTaskStartConfirmPrompt
      && matches_key!(confirm, self.key)
    {
      self.app.data.lidarr_data.prompt_confirm = true;
      self.app.data.lidarr_data.prompt_confirm_action =
        Some(LidarrEvent::StartTask(self.extract_task_name()));
      self.app.pop_navigation_stack();
    }
  }

  fn app_mut(&mut self) -> &mut App<'b> {
    self.app
  }

  fn current_route(&self) -> Route {
    self.app.get_current_route()
  }
}
