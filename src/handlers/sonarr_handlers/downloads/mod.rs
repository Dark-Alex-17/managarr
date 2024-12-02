use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::App;
use crate::event::Key;
use crate::handlers::sonarr_handlers::handle_change_tab_left_right_keys;
use crate::handlers::{handle_clear_errors, handle_prompt_toggle, KeyEventHandler};
use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, DOWNLOADS_BLOCKS};
use crate::models::Scrollable;
use crate::network::sonarr_network::SonarrEvent;

#[cfg(test)]
#[path = "downloads_handler_tests.rs"]
mod downloads_handler_tests;

pub(super) struct DownloadsHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_sonarr_block: ActiveSonarrBlock,
  _context: Option<ActiveSonarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveSonarrBlock> for DownloadsHandler<'a, 'b> {
  fn accepts(active_block: ActiveSonarrBlock) -> bool {
    DOWNLOADS_BLOCKS.contains(&active_block)
  }

  fn with(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveSonarrBlock,
    _context: Option<ActiveSonarrBlock>,
  ) -> DownloadsHandler<'a, 'b> {
    DownloadsHandler {
      key,
      app,
      active_sonarr_block: active_block,
      _context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    !self.app.is_loading && !self.app.data.sonarr_data.downloads.is_empty()
  }

  fn handle_scroll_up(&mut self) {
    if self.active_sonarr_block == ActiveSonarrBlock::Downloads {
      self.app.data.sonarr_data.downloads.scroll_up()
    }
  }

  fn handle_scroll_down(&mut self) {
    if self.active_sonarr_block == ActiveSonarrBlock::Downloads {
      self.app.data.sonarr_data.downloads.scroll_down()
    }
  }

  fn handle_home(&mut self) {
    if self.active_sonarr_block == ActiveSonarrBlock::Downloads {
      self.app.data.sonarr_data.downloads.scroll_to_top()
    }
  }

  fn handle_end(&mut self) {
    if self.active_sonarr_block == ActiveSonarrBlock::Downloads {
      self.app.data.sonarr_data.downloads.scroll_to_bottom()
    }
  }

  fn handle_delete(&mut self) {
    if self.active_sonarr_block == ActiveSonarrBlock::Downloads {
      self
        .app
        .push_navigation_stack(ActiveSonarrBlock::DeleteDownloadPrompt.into())
    }
  }

  fn handle_left_right_action(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::Downloads => handle_change_tab_left_right_keys(self.app, self.key),
      ActiveSonarrBlock::DeleteDownloadPrompt | ActiveSonarrBlock::UpdateDownloadsPrompt => {
        handle_prompt_toggle(self.app, self.key)
      }
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::DeleteDownloadPrompt => {
        if self.app.data.sonarr_data.prompt_confirm {
          self.app.data.sonarr_data.prompt_confirm_action = Some(SonarrEvent::DeleteDownload(None));
        }

        self.app.pop_navigation_stack();
      }
      ActiveSonarrBlock::UpdateDownloadsPrompt => {
        if self.app.data.sonarr_data.prompt_confirm {
          self.app.data.sonarr_data.prompt_confirm_action = Some(SonarrEvent::UpdateDownloads);
        }

        self.app.pop_navigation_stack();
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::DeleteDownloadPrompt | ActiveSonarrBlock::UpdateDownloadsPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.sonarr_data.prompt_confirm = false;
      }
      _ => handle_clear_errors(self.app),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_sonarr_block {
      ActiveSonarrBlock::Downloads => match self.key {
        _ if key == DEFAULT_KEYBINDINGS.update.key => {
          self
            .app
            .push_navigation_stack(ActiveSonarrBlock::UpdateDownloadsPrompt.into());
        }
        _ if key == DEFAULT_KEYBINDINGS.refresh.key => {
          self.app.should_refresh = true;
        }
        _ => (),
      },
      ActiveSonarrBlock::DeleteDownloadPrompt => {
        if key == DEFAULT_KEYBINDINGS.confirm.key {
          self.app.data.sonarr_data.prompt_confirm = true;
          self.app.data.sonarr_data.prompt_confirm_action = Some(SonarrEvent::DeleteDownload(None));

          self.app.pop_navigation_stack();
        }
      }
      ActiveSonarrBlock::UpdateDownloadsPrompt => {
        if key == DEFAULT_KEYBINDINGS.confirm.key {
          self.app.data.sonarr_data.prompt_confirm = true;
          self.app.data.sonarr_data.prompt_confirm_action = Some(SonarrEvent::UpdateDownloads);

          self.app.pop_navigation_stack();
        }
      }
      _ => (),
    }
  }
}
