use crate::app::App;
use crate::event::Key;
use crate::handlers::{KeyEventHandler, handle_prompt_toggle};
use crate::matches_key;
use crate::models::Route;
use crate::models::readarr_models::DeleteParams;
use crate::models::servarr_data::readarr::readarr_data::{
  ActiveReadarrBlock, DELETE_AUTHOR_BLOCKS,
};
use crate::network::readarr_network::ReadarrEvent;

#[cfg(test)]
#[path = "delete_author_handler_tests.rs"]
mod delete_author_handler_tests;

pub(in crate::handlers::readarr_handlers) struct DeleteAuthorHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_readarr_block: ActiveReadarrBlock,
  _context: Option<ActiveReadarrBlock>,
}

impl DeleteAuthorHandler<'_, '_> {
  fn build_delete_author_params(&mut self) -> DeleteParams {
    let id = self.app.data.readarr_data.authors.current_selection().id;
    let delete_files = self.app.data.readarr_data.delete_files;
    let add_import_list_exclusion = self.app.data.readarr_data.add_import_list_exclusion;
    self.app.data.readarr_data.reset_delete_preferences();

    DeleteParams {
      id,
      delete_files,
      add_import_list_exclusion,
    }
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveReadarrBlock> for DeleteAuthorHandler<'a, 'b> {
  fn accepts(active_block: ActiveReadarrBlock) -> bool {
    DELETE_AUTHOR_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveReadarrBlock,
    _context: Option<ActiveReadarrBlock>,
  ) -> DeleteAuthorHandler<'a, 'b> {
    DeleteAuthorHandler {
      key,
      app,
      active_readarr_block: active_block,
      _context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    !self.app.is_loading
  }

  fn handle_scroll_up(&mut self) {
    if self.active_readarr_block == ActiveReadarrBlock::DeleteAuthorPrompt {
      self.app.data.readarr_data.selected_block.up();
    }
  }

  fn handle_scroll_down(&mut self) {
    if self.active_readarr_block == ActiveReadarrBlock::DeleteAuthorPrompt {
      self.app.data.readarr_data.selected_block.down();
    }
  }

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    if self.active_readarr_block == ActiveReadarrBlock::DeleteAuthorPrompt {
      handle_prompt_toggle(self.app, self.key);
    }
  }

  fn handle_submit(&mut self) {
    if self.active_readarr_block == ActiveReadarrBlock::DeleteAuthorPrompt {
      match self.app.data.readarr_data.selected_block.get_active_block() {
        ActiveReadarrBlock::DeleteAuthorConfirmPrompt => {
          if self.app.data.readarr_data.prompt_confirm {
            self.app.data.readarr_data.prompt_confirm_action = Some(ReadarrEvent::DeleteAuthor(
              self.build_delete_author_params(),
            ));
            self.app.should_refresh = true;
          } else {
            self.app.data.readarr_data.reset_delete_preferences();
          }

          self.app.pop_navigation_stack();
        }
        ActiveReadarrBlock::DeleteAuthorToggleDeleteFile => {
          self.app.data.readarr_data.delete_files = !self.app.data.readarr_data.delete_files;
        }
        ActiveReadarrBlock::DeleteAuthorToggleAddListExclusion => {
          self.app.data.readarr_data.add_import_list_exclusion =
            !self.app.data.readarr_data.add_import_list_exclusion;
        }
        _ => (),
      }
    }
  }

  fn handle_esc(&mut self) {
    if self.active_readarr_block == ActiveReadarrBlock::DeleteAuthorPrompt {
      self.app.pop_navigation_stack();
      self.app.data.readarr_data.reset_delete_preferences();
      self.app.data.readarr_data.prompt_confirm = false;
    }
  }

  fn handle_char_key_event(&mut self) {
    if self.active_readarr_block == ActiveReadarrBlock::DeleteAuthorPrompt
      && self.app.data.readarr_data.selected_block.get_active_block()
        == ActiveReadarrBlock::DeleteAuthorConfirmPrompt
      && matches_key!(confirm, self.key)
    {
      self.app.data.readarr_data.prompt_confirm = true;
      self.app.data.readarr_data.prompt_confirm_action = Some(ReadarrEvent::DeleteAuthor(
        self.build_delete_author_params(),
      ));
      self.app.should_refresh = true;

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
