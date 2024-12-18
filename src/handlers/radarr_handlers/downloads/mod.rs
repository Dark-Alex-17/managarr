use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::App;
use crate::event::Key;
use crate::handle_table_events;
use crate::handlers::radarr_handlers::handle_change_tab_left_right_keys;
use crate::handlers::table_handler::TableHandlingConfig;
use crate::handlers::{handle_clear_errors, handle_prompt_toggle, KeyEventHandler};
use crate::models::radarr_models::DownloadRecord;
use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, DOWNLOADS_BLOCKS};
use crate::network::radarr_network::RadarrEvent;

#[cfg(test)]
#[path = "downloads_handler_tests.rs"]
mod downloads_handler_tests;

pub(super) struct DownloadsHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_radarr_block: ActiveRadarrBlock,
  _context: Option<ActiveRadarrBlock>,
}

impl<'a, 'b> DownloadsHandler<'a, 'b> {
  handle_table_events!(
    self,
    downloads,
    self.app.data.radarr_data.downloads,
    DownloadRecord
  );

  fn extract_download_id(&self) -> i64 {
    self.app.data.radarr_data.downloads.current_selection().id
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for DownloadsHandler<'a, 'b> {
  fn handle(&mut self) {
    let downloads_table_handling_config =
      TableHandlingConfig::new(ActiveRadarrBlock::Downloads.into());

    if !self.handle_downloads_table_events(downloads_table_handling_config) {
      self.handle_key_event();
    }
  }

  fn accepts(active_block: ActiveRadarrBlock) -> bool {
    DOWNLOADS_BLOCKS.contains(&active_block)
  }

  fn with(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveRadarrBlock,
    _context: Option<ActiveRadarrBlock>,
  ) -> DownloadsHandler<'a, 'b> {
    DownloadsHandler {
      key,
      app,
      active_radarr_block: active_block,
      _context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    !self.app.is_loading && !self.app.data.radarr_data.downloads.is_empty()
  }

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {
    if self.active_radarr_block == ActiveRadarrBlock::Downloads {
      self
        .app
        .push_navigation_stack(ActiveRadarrBlock::DeleteDownloadPrompt.into())
    }
  }

  fn handle_left_right_action(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::Downloads => handle_change_tab_left_right_keys(self.app, self.key),
      ActiveRadarrBlock::DeleteDownloadPrompt | ActiveRadarrBlock::UpdateDownloadsPrompt => {
        handle_prompt_toggle(self.app, self.key)
      }
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::DeleteDownloadPrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action =
            Some(RadarrEvent::DeleteDownload(self.extract_download_id()));
        }

        self.app.pop_navigation_stack();
      }
      ActiveRadarrBlock::UpdateDownloadsPrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::UpdateDownloads);
        }

        self.app.pop_navigation_stack();
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::DeleteDownloadPrompt | ActiveRadarrBlock::UpdateDownloadsPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.prompt_confirm = false;
      }
      _ => handle_clear_errors(self.app),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_radarr_block {
      ActiveRadarrBlock::Downloads => match self.key {
        _ if key == DEFAULT_KEYBINDINGS.update.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::UpdateDownloadsPrompt.into());
        }
        _ if key == DEFAULT_KEYBINDINGS.refresh.key => {
          self.app.should_refresh = true;
        }
        _ => (),
      },
      ActiveRadarrBlock::DeleteDownloadPrompt => {
        if key == DEFAULT_KEYBINDINGS.confirm.key {
          self.app.data.radarr_data.prompt_confirm = true;
          self.app.data.radarr_data.prompt_confirm_action =
            Some(RadarrEvent::DeleteDownload(self.extract_download_id()));

          self.app.pop_navigation_stack();
        }
      }
      ActiveRadarrBlock::UpdateDownloadsPrompt => {
        if key == DEFAULT_KEYBINDINGS.confirm.key {
          self.app.data.radarr_data.prompt_confirm = true;
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::UpdateDownloads);

          self.app.pop_navigation_stack();
        }
      }
      _ => (),
    }
  }
}
