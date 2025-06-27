use crate::handlers::table_handler::TableHandlingConfig;
use crate::handlers::{handle_prompt_toggle, KeyEventHandler};
use crate::models::servarr_data::lidarr::lidarr_data::{
  ActiveLidarrBlock, ADD_ARTIST_BLOCKS, ADD_ARTIST_SELECTION_BLOCKS,
};
use crate::models::lidarr_models::{AddArtistBody, AddArtistOptions, AddArtistSearchResult};
use crate::models::stateful_table::StatefulTable;
use crate::models::{BlockSelectionState, Scrollable};
use crate::network::lidarr_network::LidarrEvent;
use crate::{
  handle_table_events, handle_text_box_keys, handle_text_box_left_right_keys, matches_key, App, Key,
};
use crate::models::servarr_data::lidarr::modals::add_artist_modal::AddArtistModal;

#[cfg(test)]
#[path = "add_artist_handler_tests.rs"]
mod add_artist_handler_tests;

pub(super) struct AddArtistHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_lidarr_block: ActiveLidarrBlock,
  _context: Option<ActiveLidarrBlock>,
}

impl AddArtistHandler<'_, '_> {
  handle_table_events!(
    self,
    add_searched_artist,
    self
      .app
      .data
      .lidarr_data
      .add_searched_artist
      .as_mut()
      .unwrap_or(&mut StatefulTable::default()),
    AddArtistSearchResult
  );

  fn build_add_artist_body(&mut self) -> AddArtistBody {
    let add_artist_modal = self
      .app
      .data
      .lidarr_data
      .add_artist_modal
      .take()
      .expect("AddArtistModal is None");
    let tags = add_artist_modal.tags.text;
    let AddArtistModal {
      root_folder_list,
      monitor_list,
      quality_profile_list,
      metadata_profile_list,
      ..
    } = add_artist_modal;
    let (foreign_artist_id, artist_name) = {
      let AddArtistSearchResult { foreign_artist_id, artist_name, .. } = self
        .app
        .data
        .lidarr_data
        .add_searched_artist
        .as_ref()
        .unwrap()
        .current_selection();
      (foreign_artist_id.clone(), artist_name.clone().text)
    };
    let quality_profile = quality_profile_list.current_selection();
    let quality_profile_id = *self
      .app
      .data
      .lidarr_data
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
      .lidarr_data
      .metadata_profiles_map
      .iter()
      .filter(|(_, value)| *value == metadata_profile)
      .map(|(key, _)| key)
      .next()
      .unwrap();

    let path = root_folder_list.current_selection().path.clone();
    let monitor = monitor_list.current_selection().to_string();

    AddArtistBody {
      foreign_artist_id,
      quality_profile_id,
      monitored: true,
      root_folder_path: path,
      metadata_profile_id,
      tags: Vec::new(),
      tag_input_string: Some(tags),
      add_options: AddArtistOptions {
        monitor,
        search_for_missing_albums: true,
      },
    }
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveLidarrBlock> for AddArtistHandler<'a, 'b> {
  fn handle(&mut self) {
    let add_artist_table_handling_config =
      TableHandlingConfig::new(ActiveLidarrBlock::AddArtistSearchResults.into());

    if !self.handle_add_searched_artist_table_events(add_artist_table_handling_config) {
      self.handle_key_event();
    }
  }

  fn accepts(active_block: ActiveLidarrBlock) -> bool {
    ADD_ARTIST_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveLidarrBlock,
    _context: Option<ActiveLidarrBlock>,
  ) -> AddArtistHandler<'a, 'b> {
    AddArtistHandler {
      key,
      app,
      active_lidarr_block: active_block,
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
    match self.active_lidarr_block {
      ActiveLidarrBlock::AddArtistSelectMonitor => self
        .app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .scroll_up(),
      ActiveLidarrBlock::AddArtistSelectQualityProfile => self
        .app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_up(),
      ActiveLidarrBlock::AddArtistSelectMetadataProfile => self
        .app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .scroll_up(),
      ActiveLidarrBlock::AddArtistSelectRootFolder => self
        .app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .root_folder_list
        .scroll_up(),
      ActiveLidarrBlock::AddArtistPrompt => self.app.data.lidarr_data.selected_block.up(),
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::AddArtistSelectMonitor => self
        .app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .scroll_down(),
      ActiveLidarrBlock::AddArtistSelectQualityProfile => self
        .app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_down(),
      ActiveLidarrBlock::AddArtistSelectMetadataProfile => self
        .app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .scroll_down(),
      ActiveLidarrBlock::AddArtistSelectRootFolder => self
        .app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .root_folder_list
        .scroll_down(),
      ActiveLidarrBlock::AddArtistPrompt => self.app.data.lidarr_data.selected_block.down(),
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::AddArtistSelectMonitor => self
        .app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .scroll_to_top(),
      ActiveLidarrBlock::AddArtistSelectQualityProfile => self
        .app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_to_top(),
      ActiveLidarrBlock::AddArtistSelectMetadataProfile => self
        .app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .scroll_to_top(),
      ActiveLidarrBlock::AddArtistSelectRootFolder => self
        .app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .root_folder_list
        .scroll_to_top(),
      ActiveLidarrBlock::AddArtistSearchInput => self
        .app
        .data
        .lidarr_data
        .add_artist_search
        .as_mut()
        .unwrap()
        .scroll_home(),
      ActiveLidarrBlock::AddArtistTagsInput => self
        .app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .tags
        .scroll_home(),
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::AddArtistSelectMonitor => self
        .app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .scroll_to_bottom(),
      ActiveLidarrBlock::AddArtistSelectQualityProfile => self
        .app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_to_bottom(),
      ActiveLidarrBlock::AddArtistSelectMetadataProfile => self
        .app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .scroll_to_bottom(),
      ActiveLidarrBlock::AddArtistSelectRootFolder => self
        .app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .root_folder_list
        .scroll_to_bottom(),
      ActiveLidarrBlock::AddArtistSearchInput => self
        .app
        .data
        .lidarr_data
        .add_artist_search
        .as_mut()
        .unwrap()
        .reset_offset(),
      ActiveLidarrBlock::AddArtistTagsInput => self
        .app
        .data
        .lidarr_data
        .add_artist_modal
        .as_mut()
        .unwrap()
        .tags
        .reset_offset(),
      _ => (),
    }
  }

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::AddArtistPrompt => handle_prompt_toggle(self.app, self.key),
      ActiveLidarrBlock::AddArtistSearchInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .lidarr_data
            .add_artist_search
            .as_mut()
            .unwrap()
        )
      }
      ActiveLidarrBlock::AddArtistTagsInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .lidarr_data
            .add_artist_modal
            .as_mut()
            .unwrap()
            .tags
        )
      }
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_lidarr_block {
      _ if self.active_lidarr_block == ActiveLidarrBlock::AddArtistSearchInput
        && !self
          .app
          .data
          .lidarr_data
          .add_artist_search
          .as_mut()
          .unwrap()
          .text
          .is_empty() =>
      {
        self
          .app
          .push_navigation_stack(ActiveLidarrBlock::AddArtistSearchResults.into());
        self.app.ignore_special_keys_for_textbox_input = false;
      }
      _ if self.active_lidarr_block == ActiveLidarrBlock::AddArtistSearchResults
        && self.app.data.lidarr_data.add_searched_artist.is_some() =>
      {
        let foreign_artist_id = self
          .app
          .data
          .lidarr_data
          .add_searched_artist
          .as_ref()
          .unwrap()
          .current_selection()
          .clone()
          .foreign_artist_id;

        if self
          .app
          .data
          .lidarr_data
          .artists
          .items
          .iter()
          .any(|artist| artist.foreign_artist_id == foreign_artist_id)
        {
          self
            .app
            .push_navigation_stack(ActiveLidarrBlock::AddArtistAlreadyInLibrary.into());
        } else {
          self
            .app
            .push_navigation_stack(ActiveLidarrBlock::AddArtistPrompt.into());
          self.app.data.lidarr_data.add_artist_modal = Some((&self.app.data.lidarr_data).into());
          self.app.data.lidarr_data.selected_block =
            BlockSelectionState::new(ADD_ARTIST_SELECTION_BLOCKS);
        }
      }
      ActiveLidarrBlock::AddArtistPrompt => {
        match self.app.data.lidarr_data.selected_block.get_active_block() {
          ActiveLidarrBlock::AddArtistConfirmPrompt => {
            if self.app.data.lidarr_data.prompt_confirm {
              self.app.data.lidarr_data.prompt_confirm_action =
                Some(LidarrEvent::AddArtist(self.build_add_artist_body()));
            }

            self.app.pop_navigation_stack();
          }
          ActiveLidarrBlock::AddArtistSelectMonitor
          | ActiveLidarrBlock::AddArtistSelectQualityProfile
          | ActiveLidarrBlock::AddArtistSelectMetadataProfile
          | ActiveLidarrBlock::AddArtistSelectRootFolder => self.app.push_navigation_stack(
            self
              .app
              .data
              .lidarr_data
              .selected_block
              .get_active_block()
              .into(),
          ),
          ActiveLidarrBlock::AddArtistTagsInput => {
            self.app.push_navigation_stack(
              self
                .app
                .data
                .lidarr_data
                .selected_block
                .get_active_block()
                .into(),
            );
            self.app.ignore_special_keys_for_textbox_input = true;
          }
          _ => (),
        }
      }
      ActiveLidarrBlock::AddArtistSelectMonitor
      | ActiveLidarrBlock::AddArtistSelectQualityProfile
      | ActiveLidarrBlock::AddArtistSelectMetadataProfile
      | ActiveLidarrBlock::AddArtistSelectRootFolder => self.app.pop_navigation_stack(),
      ActiveLidarrBlock::AddArtistTagsInput => {
        self.app.pop_navigation_stack();
        self.app.ignore_special_keys_for_textbox_input = false;
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::AddArtistSearchInput => {
        self.app.pop_navigation_stack();
        self.app.data.lidarr_data.add_artist_search = None;
        self.app.ignore_special_keys_for_textbox_input = false;
      }
      ActiveLidarrBlock::AddArtistSearchResults
      | ActiveLidarrBlock::AddArtistEmptySearchResults => {
        self.app.pop_navigation_stack();
        self.app.data.lidarr_data.add_searched_artist = None;
        self.app.ignore_special_keys_for_textbox_input = true;
      }
      ActiveLidarrBlock::AddArtistPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.lidarr_data.add_artist_modal = None;
        self.app.data.lidarr_data.prompt_confirm = false;
      }
      ActiveLidarrBlock::AddArtistSelectMonitor
      | ActiveLidarrBlock::AddArtistSelectQualityProfile
      | ActiveLidarrBlock::AddArtistSelectMetadataProfile
      | ActiveLidarrBlock::AddArtistAlreadyInLibrary
      | ActiveLidarrBlock::AddArtistSelectRootFolder => self.app.pop_navigation_stack(),
      ActiveLidarrBlock::AddArtistTagsInput => {
        self.app.pop_navigation_stack();
        self.app.ignore_special_keys_for_textbox_input = false;
      }
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_lidarr_block {
      ActiveLidarrBlock::AddArtistSearchInput => {
        handle_text_box_keys!(
          self,
          key,
          self
            .app
            .data
            .lidarr_data
            .add_artist_search
            .as_mut()
            .unwrap()
        )
      }
      ActiveLidarrBlock::AddArtistTagsInput => {
        handle_text_box_keys!(
          self,
          key,
          self
            .app
            .data
            .lidarr_data
            .add_artist_modal
            .as_mut()
            .unwrap()
            .tags
        )
      }
      ActiveLidarrBlock::AddArtistPrompt => {
        if self.app.data.lidarr_data.selected_block.get_active_block()
          == ActiveLidarrBlock::AddArtistConfirmPrompt
          && matches_key!(confirm, key)
        {
          self.app.data.lidarr_data.prompt_confirm = true;
          self.app.data.lidarr_data.prompt_confirm_action =
            Some(LidarrEvent::AddArtist(self.build_add_artist_body()));
          self.app.pop_navigation_stack();
        }
      }
      _ => (),
    }
  }
}
