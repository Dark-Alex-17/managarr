use crate::app::App;
use crate::event::Key;
use crate::handlers::{KeyEventHandler, handle_prompt_toggle};
use crate::models::readarr_models::EditAuthorParams;
use crate::models::servarr_data::readarr::modals::EditAuthorModal;
use crate::models::servarr_data::readarr::readarr_data::{ActiveReadarrBlock, EDIT_AUTHOR_BLOCKS};
use crate::models::{Route, Scrollable};
use crate::network::readarr_network::ReadarrEvent;
use crate::{handle_text_box_keys, handle_text_box_left_right_keys, matches_key};

#[cfg(test)]
#[path = "edit_author_handler_tests.rs"]
mod edit_author_handler_tests;

pub(in crate::handlers::readarr_handlers) struct EditAuthorHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_readarr_block: ActiveReadarrBlock,
  context: Option<ActiveReadarrBlock>,
}

impl EditAuthorHandler<'_, '_> {
  fn build_edit_author_params(&mut self) -> EditAuthorParams {
    let edit_author_modal = self
      .app
      .data
      .readarr_data
      .edit_author_modal
      .take()
      .expect("EditAuthorModal is None");
    let author_id = self.app.data.readarr_data.authors.current_selection().id;
    let tags = edit_author_modal.tags.text;

    let EditAuthorModal {
      monitored,
      path,
      monitor_list,
      quality_profile_list,
      metadata_profile_list,
      ..
    } = edit_author_modal;
    let quality_profile = quality_profile_list.current_selection();
    let quality_profile_id = *self
      .app
      .data
      .readarr_data
      .quality_profile_map
      .iter()
      .filter(|(_, value)| *value == quality_profile)
      .map(|(key, _)| key)
      .next()
      .unwrap();
    let metadata_profile = metadata_profile_list.current_selection();
    let metadata_profile_id = *self
      .app
      .data
      .readarr_data
      .metadata_profile_map
      .iter()
      .filter(|(_, value)| *value == metadata_profile)
      .map(|(key, _)| key)
      .next()
      .unwrap();

    EditAuthorParams {
      author_id,
      monitored,
      monitor_new_items: Some(*monitor_list.current_selection()),
      quality_profile_id: Some(quality_profile_id),
      metadata_profile_id: Some(metadata_profile_id),
      root_folder_path: Some(path.text),
      tag_input_string: Some(tags),
      ..EditAuthorParams::default()
    }
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveReadarrBlock> for EditAuthorHandler<'a, 'b> {
  fn accepts(active_block: ActiveReadarrBlock) -> bool {
    EDIT_AUTHOR_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveReadarrBlock,
    context: Option<ActiveReadarrBlock>,
  ) -> EditAuthorHandler<'a, 'b> {
    EditAuthorHandler {
      key,
      app,
      active_readarr_block: active_block,
      context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    !self.app.is_loading && self.app.data.readarr_data.edit_author_modal.is_some()
  }

  fn handle_scroll_up(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::EditAuthorSelectMonitorNewItems => self
        .app
        .data
        .readarr_data
        .edit_author_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .scroll_up(),
      ActiveReadarrBlock::EditAuthorSelectQualityProfile => self
        .app
        .data
        .readarr_data
        .edit_author_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_up(),
      ActiveReadarrBlock::EditAuthorSelectMetadataProfile => self
        .app
        .data
        .readarr_data
        .edit_author_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .scroll_up(),
      ActiveReadarrBlock::EditAuthorPrompt => self.app.data.readarr_data.selected_block.up(),
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::EditAuthorSelectMonitorNewItems => self
        .app
        .data
        .readarr_data
        .edit_author_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .scroll_down(),
      ActiveReadarrBlock::EditAuthorSelectQualityProfile => self
        .app
        .data
        .readarr_data
        .edit_author_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_down(),
      ActiveReadarrBlock::EditAuthorSelectMetadataProfile => self
        .app
        .data
        .readarr_data
        .edit_author_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .scroll_down(),
      ActiveReadarrBlock::EditAuthorPrompt => self.app.data.readarr_data.selected_block.down(),
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::EditAuthorSelectMonitorNewItems => self
        .app
        .data
        .readarr_data
        .edit_author_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .scroll_to_top(),
      ActiveReadarrBlock::EditAuthorSelectQualityProfile => self
        .app
        .data
        .readarr_data
        .edit_author_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_to_top(),
      ActiveReadarrBlock::EditAuthorSelectMetadataProfile => self
        .app
        .data
        .readarr_data
        .edit_author_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .scroll_to_top(),
      ActiveReadarrBlock::EditAuthorPathInput => self
        .app
        .data
        .readarr_data
        .edit_author_modal
        .as_mut()
        .unwrap()
        .path
        .scroll_home(),
      ActiveReadarrBlock::EditAuthorTagsInput => self
        .app
        .data
        .readarr_data
        .edit_author_modal
        .as_mut()
        .unwrap()
        .tags
        .scroll_home(),
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::EditAuthorSelectMonitorNewItems => self
        .app
        .data
        .readarr_data
        .edit_author_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .scroll_to_bottom(),
      ActiveReadarrBlock::EditAuthorSelectQualityProfile => self
        .app
        .data
        .readarr_data
        .edit_author_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_to_bottom(),
      ActiveReadarrBlock::EditAuthorSelectMetadataProfile => self
        .app
        .data
        .readarr_data
        .edit_author_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .scroll_to_bottom(),
      ActiveReadarrBlock::EditAuthorPathInput => self
        .app
        .data
        .readarr_data
        .edit_author_modal
        .as_mut()
        .unwrap()
        .path
        .reset_offset(),
      ActiveReadarrBlock::EditAuthorTagsInput => self
        .app
        .data
        .readarr_data
        .edit_author_modal
        .as_mut()
        .unwrap()
        .tags
        .reset_offset(),
      _ => (),
    }
  }

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::EditAuthorPrompt => handle_prompt_toggle(self.app, self.key),
      ActiveReadarrBlock::EditAuthorPathInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .readarr_data
            .edit_author_modal
            .as_mut()
            .unwrap()
            .path
        )
      }
      ActiveReadarrBlock::EditAuthorTagsInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .readarr_data
            .edit_author_modal
            .as_mut()
            .unwrap()
            .tags
        )
      }
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::EditAuthorPrompt => {
        match self.app.data.readarr_data.selected_block.get_active_block() {
          ActiveReadarrBlock::EditAuthorConfirmPrompt => {
            if self.app.data.readarr_data.prompt_confirm {
              self.app.data.readarr_data.prompt_confirm_action =
                Some(ReadarrEvent::EditAuthor(self.build_edit_author_params()));
              self.app.should_refresh = true;
            }

            self.app.pop_navigation_stack();
          }
          ActiveReadarrBlock::EditAuthorSelectMonitorNewItems
          | ActiveReadarrBlock::EditAuthorSelectQualityProfile
          | ActiveReadarrBlock::EditAuthorSelectMetadataProfile => self.app.push_navigation_stack(
            (
              self.app.data.readarr_data.selected_block.get_active_block(),
              self.context,
            )
              .into(),
          ),
          ActiveReadarrBlock::EditAuthorPathInput | ActiveReadarrBlock::EditAuthorTagsInput => {
            self.app.push_navigation_stack(
              (
                self.app.data.readarr_data.selected_block.get_active_block(),
                self.context,
              )
                .into(),
            );
            self.app.ignore_special_keys_for_textbox_input = true;
          }
          ActiveReadarrBlock::EditAuthorToggleMonitored => {
            self
              .app
              .data
              .readarr_data
              .edit_author_modal
              .as_mut()
              .unwrap()
              .monitored = Some(
              !self
                .app
                .data
                .readarr_data
                .edit_author_modal
                .as_mut()
                .unwrap()
                .monitored
                .unwrap_or_default(),
            )
          }
          _ => (),
        }
      }
      ActiveReadarrBlock::EditAuthorSelectMonitorNewItems
      | ActiveReadarrBlock::EditAuthorSelectQualityProfile
      | ActiveReadarrBlock::EditAuthorSelectMetadataProfile => self.app.pop_navigation_stack(),
      ActiveReadarrBlock::EditAuthorPathInput | ActiveReadarrBlock::EditAuthorTagsInput => {
        self.app.pop_navigation_stack();
        self.app.ignore_special_keys_for_textbox_input = false;
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::EditAuthorTagsInput | ActiveReadarrBlock::EditAuthorPathInput => {
        self.app.pop_navigation_stack();
        self.app.ignore_special_keys_for_textbox_input = false;
      }
      ActiveReadarrBlock::EditAuthorPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.readarr_data.edit_author_modal = None;
        self.app.data.readarr_data.prompt_confirm = false;
      }
      ActiveReadarrBlock::EditAuthorSelectMonitorNewItems
      | ActiveReadarrBlock::EditAuthorSelectQualityProfile
      | ActiveReadarrBlock::EditAuthorSelectMetadataProfile => self.app.pop_navigation_stack(),
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_readarr_block {
      ActiveReadarrBlock::EditAuthorPathInput => {
        handle_text_box_keys!(
          self,
          key,
          self
            .app
            .data
            .readarr_data
            .edit_author_modal
            .as_mut()
            .unwrap()
            .path
        )
      }
      ActiveReadarrBlock::EditAuthorTagsInput => {
        handle_text_box_keys!(
          self,
          key,
          self
            .app
            .data
            .readarr_data
            .edit_author_modal
            .as_mut()
            .unwrap()
            .tags
        )
      }
      ActiveReadarrBlock::EditAuthorPrompt
        if self.app.data.readarr_data.selected_block.get_active_block()
          == ActiveReadarrBlock::EditAuthorConfirmPrompt
          && matches_key!(confirm, key) =>
      {
        self.app.data.readarr_data.prompt_confirm = true;
        self.app.data.readarr_data.prompt_confirm_action =
          Some(ReadarrEvent::EditAuthor(self.build_edit_author_params()));
        self.app.should_refresh = true;

        self.app.pop_navigation_stack();
      }
      _ => (),
    }
  }

  fn app_mut(&mut self) -> &mut App<'b> {
    self.app
  }

  fn current_route(&self) -> Route {
    self.app.get_current_route()
  }
}
