use crate::app::App;
use crate::event::Key;
use crate::handlers::{KeyEventHandler, handle_prompt_toggle};
use crate::matches_key;
use crate::models::readarr_models::ReadarrTaskName;
use crate::models::servarr_data::readarr::readarr_data::{
  ActiveReadarrBlock, SYSTEM_DETAILS_BLOCKS,
};
use crate::models::stateful_list::StatefulList;
use crate::models::{Route, Scrollable};
use crate::network::readarr_network::ReadarrEvent;

#[cfg(test)]
#[path = "system_details_handler_tests.rs"]
mod system_details_handler_tests;

pub(super) struct SystemDetailsHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_readarr_block: ActiveReadarrBlock,
  _context: Option<ActiveReadarrBlock>,
}

impl SystemDetailsHandler<'_, '_> {
  fn extract_task_name(&self) -> ReadarrTaskName {
    self
      .app
      .data
      .readarr_data
      .tasks
      .current_selection()
      .task_name
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveReadarrBlock> for SystemDetailsHandler<'a, 'b> {
  fn accepts(active_block: ActiveReadarrBlock) -> bool {
    SYSTEM_DETAILS_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveReadarrBlock,
    context: Option<ActiveReadarrBlock>,
  ) -> SystemDetailsHandler<'a, 'b> {
    SystemDetailsHandler {
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
    !self.app.is_loading
      && (!self.app.data.readarr_data.log_details.is_empty()
        || !self.app.data.readarr_data.tasks.is_empty()
        || !self.app.data.readarr_data.updates.is_empty())
  }

  fn handle_scroll_up(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::SystemLogs => self.app.data.readarr_data.log_details.scroll_up(),
      ActiveReadarrBlock::SystemTasks => self.app.data.readarr_data.tasks.scroll_up(),
      ActiveReadarrBlock::SystemUpdates => self.app.data.readarr_data.updates.scroll_up(),
      ActiveReadarrBlock::SystemQueuedEvents => {
        self.app.data.readarr_data.queued_events.scroll_up()
      }
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::SystemLogs => self.app.data.readarr_data.log_details.scroll_down(),
      ActiveReadarrBlock::SystemTasks => self.app.data.readarr_data.tasks.scroll_down(),
      ActiveReadarrBlock::SystemUpdates => self.app.data.readarr_data.updates.scroll_down(),
      ActiveReadarrBlock::SystemQueuedEvents => {
        self.app.data.readarr_data.queued_events.scroll_down()
      }
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::SystemLogs => self.app.data.readarr_data.log_details.scroll_to_top(),
      ActiveReadarrBlock::SystemTasks => self.app.data.readarr_data.tasks.scroll_to_top(),
      ActiveReadarrBlock::SystemUpdates => self.app.data.readarr_data.updates.scroll_to_top(),
      ActiveReadarrBlock::SystemQueuedEvents => {
        self.app.data.readarr_data.queued_events.scroll_to_top()
      }
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::SystemLogs => self.app.data.readarr_data.log_details.scroll_to_bottom(),
      ActiveReadarrBlock::SystemTasks => self.app.data.readarr_data.tasks.scroll_to_bottom(),
      ActiveReadarrBlock::SystemUpdates => self.app.data.readarr_data.updates.scroll_to_bottom(),
      ActiveReadarrBlock::SystemQueuedEvents => {
        self.app.data.readarr_data.queued_events.scroll_to_bottom()
      }
      _ => (),
    }
  }

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    let key = self.key;

    match self.active_readarr_block {
      ActiveReadarrBlock::SystemLogs => match self.key {
        _ if matches_key!(left, key) => {
          self
            .app
            .data
            .readarr_data
            .log_details
            .items
            .iter()
            .for_each(|log| log.scroll_right());
        }
        _ if matches_key!(right, key) => {
          self
            .app
            .data
            .readarr_data
            .log_details
            .items
            .iter()
            .for_each(|log| log.scroll_left());
        }
        _ => (),
      },
      ActiveReadarrBlock::SystemTaskStartConfirmPrompt => handle_prompt_toggle(self.app, self.key),
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::SystemTasks => {
        self
          .app
          .push_navigation_stack(ActiveReadarrBlock::SystemTaskStartConfirmPrompt.into());
      }
      ActiveReadarrBlock::SystemTaskStartConfirmPrompt => {
        if self.app.data.readarr_data.prompt_confirm {
          self.app.data.readarr_data.prompt_confirm_action =
            Some(ReadarrEvent::StartTask(self.extract_task_name()));
        }

        self.app.pop_navigation_stack();
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::SystemLogs => {
        self.app.data.readarr_data.log_details = StatefulList::default();
        self.app.pop_navigation_stack()
      }
      ActiveReadarrBlock::SystemQueuedEvents
      | ActiveReadarrBlock::SystemTasks
      | ActiveReadarrBlock::SystemUpdates => self.app.pop_navigation_stack(),
      ActiveReadarrBlock::SystemTaskStartConfirmPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.readarr_data.prompt_confirm = false;
      }
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    if SYSTEM_DETAILS_BLOCKS.contains(&self.active_readarr_block) && matches_key!(refresh, self.key)
    {
      self.app.should_refresh = true;
    }

    if self.active_readarr_block == ActiveReadarrBlock::SystemTaskStartConfirmPrompt
      && matches_key!(confirm, self.key)
    {
      self.app.data.readarr_data.prompt_confirm = true;
      self.app.data.readarr_data.prompt_confirm_action =
        Some(ReadarrEvent::StartTask(self.extract_task_name()));
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
