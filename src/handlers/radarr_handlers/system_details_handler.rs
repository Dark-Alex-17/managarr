use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::radarr::{ActiveRadarrBlock, SYSTEM_DETAILS_BLOCKS};
use crate::app::App;
use crate::event::Key;
use crate::handlers::{handle_prompt_toggle, KeyEventHandler};
use crate::models::Scrollable;
use crate::network::radarr_network::RadarrEvent;

#[cfg(test)]
#[path = "system_details_handler_tests.rs"]
mod system_details_handler_tests;

pub(super) struct SystemDetailsHandler<'a, 'b> {
  key: &'a Key,
  app: &'a mut App<'b>,
  active_radarr_block: &'a ActiveRadarrBlock,
  _context: &'a Option<ActiveRadarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for SystemDetailsHandler<'a, 'b> {
  fn with(
    key: &'a Key,
    app: &'a mut App<'b>,
    active_block: &'a ActiveRadarrBlock,
    context: &'a Option<ActiveRadarrBlock>,
  ) -> SystemDetailsHandler<'a, 'b> {
    SystemDetailsHandler {
      key,
      app,
      active_radarr_block: active_block,
      _context: context,
    }
  }

  fn get_key(&self) -> &Key {
    self.key
  }

  fn handle_scroll_up(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::SystemLogs => self.app.data.radarr_data.log_details.scroll_up(),
      ActiveRadarrBlock::SystemTasks => self.app.data.radarr_data.tasks.scroll_up(),
      ActiveRadarrBlock::SystemUpdates => self.app.data.radarr_data.updates.scroll_up(),
      ActiveRadarrBlock::SystemQueue => self.app.data.radarr_data.queued_events.scroll_up(),
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::SystemLogs => self.app.data.radarr_data.log_details.scroll_down(),
      ActiveRadarrBlock::SystemTasks => self.app.data.radarr_data.tasks.scroll_down(),
      ActiveRadarrBlock::SystemUpdates => self.app.data.radarr_data.updates.scroll_down(),
      ActiveRadarrBlock::SystemQueue => self.app.data.radarr_data.queued_events.scroll_down(),
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::SystemLogs => self.app.data.radarr_data.log_details.scroll_to_top(),
      ActiveRadarrBlock::SystemTasks => self.app.data.radarr_data.tasks.scroll_to_top(),
      ActiveRadarrBlock::SystemUpdates => self.app.data.radarr_data.updates.scroll_to_top(),
      ActiveRadarrBlock::SystemQueue => self.app.data.radarr_data.queued_events.scroll_to_top(),
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::SystemLogs => self.app.data.radarr_data.log_details.scroll_to_bottom(),
      ActiveRadarrBlock::SystemTasks => self.app.data.radarr_data.tasks.scroll_to_bottom(),
      ActiveRadarrBlock::SystemUpdates => self.app.data.radarr_data.updates.scroll_to_bottom(),
      ActiveRadarrBlock::SystemQueue => self.app.data.radarr_data.queued_events.scroll_to_bottom(),
      _ => (),
    }
  }

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    let key = self.key;

    match self.active_radarr_block {
      ActiveRadarrBlock::SystemLogs => match self.key {
        _ if *key == DEFAULT_KEYBINDINGS.left.key => {
          self
            .app
            .data
            .radarr_data
            .log_details
            .items
            .iter()
            .for_each(|log| log.scroll_right());
        }
        _ if *key == DEFAULT_KEYBINDINGS.right.key => {
          self
            .app
            .data
            .radarr_data
            .log_details
            .items
            .iter()
            .for_each(|log| log.scroll_left());
        }
        _ => (),
      },
      ActiveRadarrBlock::SystemTaskStartConfirmPrompt => handle_prompt_toggle(self.app, self.key),
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::SystemTasks => {
        self
          .app
          .push_navigation_stack(ActiveRadarrBlock::SystemTaskStartConfirmPrompt.into());
      }
      ActiveRadarrBlock::SystemTaskStartConfirmPrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::StartTask);
        }

        self.app.pop_navigation_stack();
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::SystemLogs => {
        self.app.data.radarr_data.reset_log_details_list();
        self.app.pop_navigation_stack()
      }
      ActiveRadarrBlock::SystemQueue
      | ActiveRadarrBlock::SystemTasks
      | ActiveRadarrBlock::SystemUpdates => self.app.pop_navigation_stack(),
      ActiveRadarrBlock::SystemTaskStartConfirmPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.prompt_confirm = false;
      }
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    if SYSTEM_DETAILS_BLOCKS.contains(self.active_radarr_block)
      && self.key == &DEFAULT_KEYBINDINGS.refresh.key
    {
      self.app.should_refresh = true;
    }
  }
}
