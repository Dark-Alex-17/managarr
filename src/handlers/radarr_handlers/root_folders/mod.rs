use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::App;
use crate::event::Key;
use crate::handlers::radarr_handlers::handle_change_tab_left_right_keys;
use crate::handlers::table_handler::TableHandlingConfig;
use crate::handlers::{handle_clear_errors, handle_prompt_toggle, KeyEventHandler};
use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, ROOT_FOLDERS_BLOCKS};
use crate::models::servarr_models::{AddRootFolderBody, RootFolder};
use crate::models::HorizontallyScrollableText;
use crate::network::radarr_network::RadarrEvent;
use crate::{handle_table_events, handle_text_box_keys, handle_text_box_left_right_keys};

#[cfg(test)]
#[path = "root_folders_handler_tests.rs"]
mod root_folders_handler_tests;

pub(super) struct RootFoldersHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_radarr_block: ActiveRadarrBlock,
  _context: Option<ActiveRadarrBlock>,
}

impl<'a, 'b> RootFoldersHandler<'a, 'b> {
  handle_table_events!(
    self,
    root_folders,
    self.app.data.radarr_data.root_folders,
    RootFolder
  );
  
  fn build_add_root_folder_body(&mut self) -> AddRootFolderBody {
    let path = self.app
      .data
      .radarr_data
      .edit_root_folder
      .as_ref()
      .unwrap()
      .text
      .clone();

    self.app.data.radarr_data.edit_root_folder = None;

    AddRootFolderBody { path }
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for RootFoldersHandler<'a, 'b> {
  fn handle(&mut self) {
    let root_folder_table_handling_config =
      TableHandlingConfig::new(ActiveRadarrBlock::RootFolders.into());

    if !self.handle_root_folders_table_events(root_folder_table_handling_config) {
      self.handle_key_event();
    }
  }

  fn accepts(active_block: ActiveRadarrBlock) -> bool {
    ROOT_FOLDERS_BLOCKS.contains(&active_block)
  }

  fn with(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveRadarrBlock,
    _context: Option<ActiveRadarrBlock>,
  ) -> RootFoldersHandler<'a, 'b> {
    RootFoldersHandler {
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
    !self.app.is_loading && !self.app.data.radarr_data.root_folders.is_empty()
  }

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {
    if self.active_radarr_block == ActiveRadarrBlock::AddRootFolderPrompt {
      self
        .app
        .data
        .radarr_data
        .edit_root_folder
        .as_mut()
        .unwrap()
        .scroll_home()
    }
  }

  fn handle_end(&mut self) {
    if self.active_radarr_block == ActiveRadarrBlock::AddRootFolderPrompt {
      self
        .app
        .data
        .radarr_data
        .edit_root_folder
        .as_mut()
        .unwrap()
        .reset_offset()
    }
  }

  fn handle_delete(&mut self) {
    if self.active_radarr_block == ActiveRadarrBlock::RootFolders {
      self
        .app
        .push_navigation_stack(ActiveRadarrBlock::DeleteRootFolderPrompt.into())
    }
  }

  fn handle_left_right_action(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::RootFolders => handle_change_tab_left_right_keys(self.app, self.key),
      ActiveRadarrBlock::DeleteRootFolderPrompt => handle_prompt_toggle(self.app, self.key),
      ActiveRadarrBlock::AddRootFolderPrompt => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self.app.data.radarr_data.edit_root_folder.as_mut().unwrap()
        )
      }
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::DeleteRootFolderPrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action =
            Some(RadarrEvent::DeleteRootFolder(None));
        }

        self.app.pop_navigation_stack();
      }
      _ if self.active_radarr_block == ActiveRadarrBlock::AddRootFolderPrompt
        && !self
          .app
          .data
          .radarr_data
          .edit_root_folder
          .as_ref()
          .unwrap()
          .text
          .is_empty() =>
      {
        self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::AddRootFolder(self.build_add_root_folder_body()));
        self.app.data.radarr_data.prompt_confirm = true;
        self.app.should_ignore_quit_key = false;
        self.app.pop_navigation_stack();
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::AddRootFolderPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.edit_root_folder = None;
        self.app.data.radarr_data.prompt_confirm = false;
        self.app.should_ignore_quit_key = false;
      }
      ActiveRadarrBlock::DeleteRootFolderPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.prompt_confirm = false;
      }
      _ => handle_clear_errors(self.app),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_radarr_block {
      ActiveRadarrBlock::RootFolders => match self.key {
        _ if key == DEFAULT_KEYBINDINGS.refresh.key => {
          self.app.should_refresh = true;
        }
        _ if key == DEFAULT_KEYBINDINGS.add.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::AddRootFolderPrompt.into());
          self.app.data.radarr_data.edit_root_folder = Some(HorizontallyScrollableText::default());
          self.app.should_ignore_quit_key = true;
        }
        _ => (),
      },
      ActiveRadarrBlock::AddRootFolderPrompt => {
        handle_text_box_keys!(
          self,
          key,
          self.app.data.radarr_data.edit_root_folder.as_mut().unwrap()
        )
      }
      ActiveRadarrBlock::DeleteRootFolderPrompt => {
        if key == DEFAULT_KEYBINDINGS.confirm.key {
          self.app.data.radarr_data.prompt_confirm = true;
          self.app.data.radarr_data.prompt_confirm_action =
            Some(RadarrEvent::DeleteRootFolder(None));

          self.app.pop_navigation_stack();
        }
      }
      _ => (),
    }
  }
}
