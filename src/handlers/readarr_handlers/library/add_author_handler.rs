use crate::handlers::table_handler::{TableHandlingConfig, handle_table};
use crate::handlers::{KeyEventHandler, handle_prompt_toggle};
use crate::models::readarr_models::{AddAuthorBody, AddAuthorOptions, AddAuthorSearchResult};
use crate::models::servarr_data::readarr::modals::AddAuthorModal;
use crate::models::servarr_data::readarr::readarr_data::{
  ADD_AUTHOR_BLOCKS, ADD_AUTHOR_SELECTION_BLOCKS, ActiveReadarrBlock,
};
use crate::models::{BlockSelectionState, Route, Scrollable};
use crate::network::readarr_network::ReadarrEvent;
use crate::{App, Key, handle_text_box_keys, handle_text_box_left_right_keys, matches_key};

#[cfg(test)]
#[path = "add_author_handler_tests.rs"]
mod add_author_handler_tests;

pub(in crate::handlers::readarr_handlers) struct AddAuthorHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_readarr_block: ActiveReadarrBlock,
  _context: Option<ActiveReadarrBlock>,
}

impl AddAuthorHandler<'_, '_> {
  fn build_add_author_body(&mut self) -> AddAuthorBody {
    let add_author_modal = self
      .app
      .data
      .readarr_data
      .add_author_modal
      .take()
      .expect("AddAuthorModal is None");
    let tags = add_author_modal.tags.text;
    let AddAuthorModal {
      root_folder_list,
      monitor_list,
      monitor_new_items_list,
      quality_profile_list,
      metadata_profile_list,
      ..
    } = add_author_modal;
    let (foreign_author_id, author_name) = {
      let AddAuthorSearchResult {
        foreign_author_id,
        author_name,
        ..
      } = self
        .app
        .data
        .readarr_data
        .add_searched_authors
        .as_ref()
        .unwrap()
        .current_selection();
      (foreign_author_id.clone(), author_name.text.clone())
    };
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

    let path = root_folder_list.current_selection().path.clone();
    let monitor = *monitor_list.current_selection();
    let monitor_new_items = *monitor_new_items_list.current_selection();

    AddAuthorBody {
      foreign_author_id,
      author_name,
      monitored: true,
      root_folder_path: path,
      quality_profile_id,
      metadata_profile_id,
      tags: Vec::new(),
      tag_input_string: Some(tags),
      add_options: AddAuthorOptions {
        monitor,
        monitor_new_items,
        search_for_missing_books: true,
      },
    }
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveReadarrBlock> for AddAuthorHandler<'a, 'b> {
  fn handle(&mut self) {
    let add_author_table_handling_config =
      TableHandlingConfig::new(ActiveReadarrBlock::AddAuthorSearchResults.into());

    if !handle_table(
      self,
      |app| {
        app
          .data
          .readarr_data
          .add_searched_authors
          .as_mut()
          .expect("add_searched_authors should be initialized")
      },
      add_author_table_handling_config,
    ) {
      self.handle_key_event();
    }
  }

  fn accepts(active_block: ActiveReadarrBlock) -> bool {
    ADD_AUTHOR_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveReadarrBlock,
    context: Option<ActiveReadarrBlock>,
  ) -> AddAuthorHandler<'a, 'b> {
    AddAuthorHandler {
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
  }

  fn handle_scroll_up(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::AddAuthorSelectMonitor => self
        .app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .scroll_up(),
      ActiveReadarrBlock::AddAuthorSelectMonitorNewItems => self
        .app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .monitor_new_items_list
        .scroll_up(),
      ActiveReadarrBlock::AddAuthorSelectQualityProfile => self
        .app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_up(),
      ActiveReadarrBlock::AddAuthorSelectMetadataProfile => self
        .app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .scroll_up(),
      ActiveReadarrBlock::AddAuthorSelectRootFolder => self
        .app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .root_folder_list
        .scroll_up(),
      ActiveReadarrBlock::AddAuthorPrompt => self.app.data.readarr_data.selected_block.up(),
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::AddAuthorSelectMonitor => self
        .app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .scroll_down(),
      ActiveReadarrBlock::AddAuthorSelectMonitorNewItems => self
        .app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .monitor_new_items_list
        .scroll_down(),
      ActiveReadarrBlock::AddAuthorSelectQualityProfile => self
        .app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_down(),
      ActiveReadarrBlock::AddAuthorSelectMetadataProfile => self
        .app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .scroll_down(),
      ActiveReadarrBlock::AddAuthorSelectRootFolder => self
        .app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .root_folder_list
        .scroll_down(),
      ActiveReadarrBlock::AddAuthorPrompt => self.app.data.readarr_data.selected_block.down(),
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::AddAuthorSelectMonitor => self
        .app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .scroll_to_top(),
      ActiveReadarrBlock::AddAuthorSelectMonitorNewItems => self
        .app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .monitor_new_items_list
        .scroll_to_top(),
      ActiveReadarrBlock::AddAuthorSelectQualityProfile => self
        .app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_to_top(),
      ActiveReadarrBlock::AddAuthorSelectMetadataProfile => self
        .app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .scroll_to_top(),
      ActiveReadarrBlock::AddAuthorSelectRootFolder => self
        .app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .root_folder_list
        .scroll_to_top(),
      ActiveReadarrBlock::AddAuthorSearchInput => self
        .app
        .data
        .readarr_data
        .add_author_search
        .as_mut()
        .unwrap()
        .scroll_home(),
      ActiveReadarrBlock::AddAuthorTagsInput => self
        .app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .tags
        .scroll_home(),
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::AddAuthorSelectMonitor => self
        .app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .scroll_to_bottom(),
      ActiveReadarrBlock::AddAuthorSelectMonitorNewItems => self
        .app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .monitor_new_items_list
        .scroll_to_bottom(),
      ActiveReadarrBlock::AddAuthorSelectQualityProfile => self
        .app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_to_bottom(),
      ActiveReadarrBlock::AddAuthorSelectMetadataProfile => self
        .app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .scroll_to_bottom(),
      ActiveReadarrBlock::AddAuthorSelectRootFolder => self
        .app
        .data
        .readarr_data
        .add_author_modal
        .as_mut()
        .unwrap()
        .root_folder_list
        .scroll_to_bottom(),
      ActiveReadarrBlock::AddAuthorSearchInput => self
        .app
        .data
        .readarr_data
        .add_author_search
        .as_mut()
        .unwrap()
        .reset_offset(),
      ActiveReadarrBlock::AddAuthorTagsInput => self
        .app
        .data
        .readarr_data
        .add_author_modal
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
      ActiveReadarrBlock::AddAuthorPrompt => handle_prompt_toggle(self.app, self.key),
      ActiveReadarrBlock::AddAuthorSearchInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .readarr_data
            .add_author_search
            .as_mut()
            .unwrap()
        )
      }
      ActiveReadarrBlock::AddAuthorTagsInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .readarr_data
            .add_author_modal
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
      ActiveReadarrBlock::AddAuthorSearchInput
        if !self
          .app
          .data
          .readarr_data
          .add_author_search
          .as_ref()
          .unwrap()
          .text
          .is_empty() =>
      {
        self
          .app
          .push_navigation_stack(ActiveReadarrBlock::AddAuthorSearchResults.into());
        self.app.ignore_special_keys_for_textbox_input = false;
      }
      ActiveReadarrBlock::AddAuthorSearchResults
        if self.app.data.readarr_data.add_searched_authors.is_some() =>
      {
        let foreign_author_id = self
          .app
          .data
          .readarr_data
          .add_searched_authors
          .as_ref()
          .unwrap()
          .current_selection()
          .foreign_author_id
          .clone();

        if self
          .app
          .data
          .readarr_data
          .authors
          .items
          .iter()
          .any(|author| author.foreign_author_id == foreign_author_id)
        {
          self
            .app
            .push_navigation_stack(ActiveReadarrBlock::AddAuthorAlreadyInLibrary.into());
        } else {
          self
            .app
            .push_navigation_stack(ActiveReadarrBlock::AddAuthorPrompt.into());
          self.app.data.readarr_data.add_author_modal = Some((&self.app.data.readarr_data).into());
          self.app.data.readarr_data.selected_block =
            BlockSelectionState::new(ADD_AUTHOR_SELECTION_BLOCKS);
        }
      }
      ActiveReadarrBlock::AddAuthorPrompt => {
        match self.app.data.readarr_data.selected_block.get_active_block() {
          ActiveReadarrBlock::AddAuthorConfirmPrompt => {
            if self.app.data.readarr_data.prompt_confirm {
              self.app.data.readarr_data.prompt_confirm_action =
                Some(ReadarrEvent::AddAuthor(self.build_add_author_body()));
            }

            self.app.pop_navigation_stack();
          }
          ActiveReadarrBlock::AddAuthorSelectMonitor
          | ActiveReadarrBlock::AddAuthorSelectMonitorNewItems
          | ActiveReadarrBlock::AddAuthorSelectQualityProfile
          | ActiveReadarrBlock::AddAuthorSelectMetadataProfile
          | ActiveReadarrBlock::AddAuthorSelectRootFolder => self.app.push_navigation_stack(
            self
              .app
              .data
              .readarr_data
              .selected_block
              .get_active_block()
              .into(),
          ),
          ActiveReadarrBlock::AddAuthorTagsInput => {
            self.app.push_navigation_stack(
              self
                .app
                .data
                .readarr_data
                .selected_block
                .get_active_block()
                .into(),
            );
            self.app.ignore_special_keys_for_textbox_input = true;
          }
          _ => (),
        }
      }
      ActiveReadarrBlock::AddAuthorSelectMonitor
      | ActiveReadarrBlock::AddAuthorSelectMonitorNewItems
      | ActiveReadarrBlock::AddAuthorSelectQualityProfile
      | ActiveReadarrBlock::AddAuthorSelectMetadataProfile
      | ActiveReadarrBlock::AddAuthorSelectRootFolder => self.app.pop_navigation_stack(),
      ActiveReadarrBlock::AddAuthorTagsInput => {
        self.app.pop_navigation_stack();
        self.app.ignore_special_keys_for_textbox_input = false;
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_readarr_block {
      ActiveReadarrBlock::AddAuthorSearchInput => {
        self.app.pop_navigation_stack();
        self.app.data.readarr_data.add_author_search = None;
        self.app.ignore_special_keys_for_textbox_input = false;
      }
      ActiveReadarrBlock::AddAuthorSearchResults
      | ActiveReadarrBlock::AddAuthorEmptySearchResults => {
        self.app.pop_navigation_stack();
        self.app.data.readarr_data.add_searched_authors = None;
        self.app.ignore_special_keys_for_textbox_input = true;
      }
      ActiveReadarrBlock::AddAuthorPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.readarr_data.add_author_modal = None;
        self.app.data.readarr_data.prompt_confirm = false;
      }
      ActiveReadarrBlock::AddAuthorSelectMonitor
      | ActiveReadarrBlock::AddAuthorSelectMonitorNewItems
      | ActiveReadarrBlock::AddAuthorSelectQualityProfile
      | ActiveReadarrBlock::AddAuthorSelectMetadataProfile
      | ActiveReadarrBlock::AddAuthorAlreadyInLibrary
      | ActiveReadarrBlock::AddAuthorSelectRootFolder => self.app.pop_navigation_stack(),
      ActiveReadarrBlock::AddAuthorTagsInput => {
        self.app.pop_navigation_stack();
        self.app.ignore_special_keys_for_textbox_input = false;
      }
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_readarr_block {
      ActiveReadarrBlock::AddAuthorSearchInput => {
        handle_text_box_keys!(
          self,
          key,
          self
            .app
            .data
            .readarr_data
            .add_author_search
            .as_mut()
            .unwrap()
        )
      }
      ActiveReadarrBlock::AddAuthorTagsInput => {
        handle_text_box_keys!(
          self,
          key,
          self
            .app
            .data
            .readarr_data
            .add_author_modal
            .as_mut()
            .unwrap()
            .tags
        )
      }
      ActiveReadarrBlock::AddAuthorPrompt
        if self.app.data.readarr_data.selected_block.get_active_block()
          == ActiveReadarrBlock::AddAuthorConfirmPrompt
          && matches_key!(confirm, key) =>
      {
        self.app.data.readarr_data.prompt_confirm = true;
        self.app.data.readarr_data.prompt_confirm_action =
          Some(ReadarrEvent::AddAuthor(self.build_add_author_body()));
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
