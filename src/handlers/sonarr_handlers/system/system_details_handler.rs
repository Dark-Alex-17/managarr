use crate::app::App;
use crate::event::Key;
use crate::handlers::{handle_prompt_toggle, KeyEventHandler};
use crate::matches_key;
use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, SYSTEM_DETAILS_BLOCKS};
use crate::models::sonarr_models::SonarrTaskName;
use crate::models::stateful_list::StatefulList;
use crate::models::Scrollable;
use crate::network::sonarr_network::SonarrEvent;

#[cfg(test)]
#[path = "system_details_handler_tests.rs"]
mod system_details_handler_tests;

pub(super) struct SystemDetailsHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_sonarr_block: ActiveSonarrBlock,
  _context: Option<ActiveSonarrBlock>,
}

impl SystemDetailsHandler<'_, '_> {
  fn extract_task_name(&self) -> SonarrTaskName {
    self
      .app
      .data
      .sonarr_data
      .tasks
      .current_selection()
      .task_name
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveSonarrBlock> for SystemDetailsHandler<'a, 'b> {
  fn accepts(active_block: ActiveSonarrBlock) -> bool {
    SYSTEM_DETAILS_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveSonarrBlock,
    context: Option<ActiveSonarrBlock>,
  ) -> SystemDetailsHandler<'a, 'b> {
    SystemDetailsHandler {
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
    !self.app.is_loading
      && (!self.app.data.sonarr_data.log_details.is_empty()
        || !self.app.data.sonarr_data.tasks.is_empty()
        || !self.app.data.sonarr_data.updates.is_empty())
  }

  fn handle_scroll_up(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::SystemLogs => self.app.data.sonarr_data.log_details.scroll_up(),
      ActiveSonarrBlock::SystemTasks => self.app.data.sonarr_data.tasks.scroll_up(),
      ActiveSonarrBlock::SystemUpdates => self.app.data.sonarr_data.updates.scroll_up(),
      ActiveSonarrBlock::SystemQueuedEvents => self.app.data.sonarr_data.queued_events.scroll_up(),
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::SystemLogs => self.app.data.sonarr_data.log_details.scroll_down(),
      ActiveSonarrBlock::SystemTasks => self.app.data.sonarr_data.tasks.scroll_down(),
      ActiveSonarrBlock::SystemUpdates => self.app.data.sonarr_data.updates.scroll_down(),
      ActiveSonarrBlock::SystemQueuedEvents => {
        self.app.data.sonarr_data.queued_events.scroll_down()
      }
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::SystemLogs => self.app.data.sonarr_data.log_details.scroll_to_top(),
      ActiveSonarrBlock::SystemTasks => self.app.data.sonarr_data.tasks.scroll_to_top(),
      ActiveSonarrBlock::SystemUpdates => self.app.data.sonarr_data.updates.scroll_to_top(),
      ActiveSonarrBlock::SystemQueuedEvents => {
        self.app.data.sonarr_data.queued_events.scroll_to_top()
      }
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::SystemLogs => self.app.data.sonarr_data.log_details.scroll_to_bottom(),
      ActiveSonarrBlock::SystemTasks => self.app.data.sonarr_data.tasks.scroll_to_bottom(),
      ActiveSonarrBlock::SystemUpdates => self.app.data.sonarr_data.updates.scroll_to_bottom(),
      ActiveSonarrBlock::SystemQueuedEvents => {
        self.app.data.sonarr_data.queued_events.scroll_to_bottom()
      }
      _ => (),
    }
  }

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    let key = self.key;

    match self.active_sonarr_block {
      ActiveSonarrBlock::SystemLogs => match self.key {
        _ if matches_key!(left, key) => {
          self
            .app
            .data
            .sonarr_data
            .log_details
            .items
            .iter()
            .for_each(|log| log.scroll_right());
        }
        _ if matches_key!(right, key) => {
          self
            .app
            .data
            .sonarr_data
            .log_details
            .items
            .iter()
            .for_each(|log| log.scroll_left());
        }
        _ => (),
      },
      ActiveSonarrBlock::SystemTaskStartConfirmPrompt => handle_prompt_toggle(self.app, self.key),
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::SystemTasks => {
        self
          .app
          .push_navigation_stack(ActiveSonarrBlock::SystemTaskStartConfirmPrompt.into());
      }
      ActiveSonarrBlock::SystemTaskStartConfirmPrompt => {
        if self.app.data.sonarr_data.prompt_confirm {
          self.app.data.sonarr_data.prompt_confirm_action =
            Some(SonarrEvent::StartTask(self.extract_task_name()));
        }

        self.app.pop_navigation_stack();
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::SystemLogs => {
        self.app.data.sonarr_data.log_details = StatefulList::default();
        self.app.pop_navigation_stack()
      }
      ActiveSonarrBlock::SystemQueuedEvents
      | ActiveSonarrBlock::SystemTasks
      | ActiveSonarrBlock::SystemUpdates => self.app.pop_navigation_stack(),
      ActiveSonarrBlock::SystemTaskStartConfirmPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.sonarr_data.prompt_confirm = false;
      }
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    if SYSTEM_DETAILS_BLOCKS.contains(&self.active_sonarr_block) && matches_key!(refresh, self.key)
    {
      self.app.should_refresh = true;
    }

    if self.active_sonarr_block == ActiveSonarrBlock::SystemTaskStartConfirmPrompt
      && matches_key!(confirm, self.key)
    {
      self.app.data.sonarr_data.prompt_confirm = true;
      self.app.data.sonarr_data.prompt_confirm_action =
        Some(SonarrEvent::StartTask(self.extract_task_name()));
      self.app.pop_navigation_stack();
    }
  }
}
