use crate::app::App;
use crate::event::Key;
use crate::handlers::{KeyEventHandler, handle_prompt_toggle};
use crate::models::{Route, Scrollable};
use crate::models::lidarr_models::EditArtistParams;
use crate::models::servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, EDIT_ARTIST_BLOCKS};
use crate::models::servarr_data::lidarr::modals::EditArtistModal;
use crate::network::lidarr_network::LidarrEvent;
use crate::{handle_text_box_keys, handle_text_box_left_right_keys, matches_key};

#[cfg(test)]
#[path = "edit_artist_handler_tests.rs"]
mod edit_artist_handler_tests;

pub(in crate::handlers::lidarr_handlers) struct EditArtistHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_lidarr_block: ActiveLidarrBlock,
  context: Option<ActiveLidarrBlock>,
}

impl EditArtistHandler<'_, '_> {
  fn build_edit_artist_params(&mut self) -> EditArtistParams {
    let edit_artist_modal = self
      .app
      .data
      .lidarr_data
      .edit_artist_modal
      .take()
      .expect("EditArtistModal is None");
    let artist_id = self.app.data.lidarr_data.artists.current_selection().id;
    let tags = edit_artist_modal.tags.text;

    let EditArtistModal {
      monitored,
      path,
      monitor_list,
      quality_profile_list,
      metadata_profile_list,
      ..
    } = edit_artist_modal;
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
      .metadata_profile_map
      .iter()
      .filter(|(_, value)| *value == metadata_profile)
      .map(|(key, _)| key)
      .next()
      .unwrap();

    EditArtistParams {
      artist_id,
      monitored,
      monitor_new_items: Some(*monitor_list.current_selection()),
      quality_profile_id: Some(quality_profile_id),
      metadata_profile_id: Some(metadata_profile_id),
      root_folder_path: Some(path.text),
      tag_input_string: Some(tags),
      ..EditArtistParams::default()
    }
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveLidarrBlock> for EditArtistHandler<'a, 'b> {
  fn accepts(active_block: ActiveLidarrBlock) -> bool {
    EDIT_ARTIST_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveLidarrBlock,
    context: Option<ActiveLidarrBlock>,
  ) -> EditArtistHandler<'a, 'b> {
    EditArtistHandler {
      key,
      app,
      active_lidarr_block: active_block,
      context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    !self.app.is_loading && self.app.data.lidarr_data.edit_artist_modal.is_some()
  }

  fn handle_scroll_up(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::EditArtistSelectMonitorNewItems => self
        .app
        .data
        .lidarr_data
        .edit_artist_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .scroll_up(),
      ActiveLidarrBlock::EditArtistSelectQualityProfile => self
        .app
        .data
        .lidarr_data
        .edit_artist_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_up(),
      ActiveLidarrBlock::EditArtistSelectMetadataProfile => self
        .app
        .data
        .lidarr_data
        .edit_artist_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .scroll_up(),
      ActiveLidarrBlock::EditArtistPrompt => self.app.data.lidarr_data.selected_block.up(),
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::EditArtistSelectMonitorNewItems => self
        .app
        .data
        .lidarr_data
        .edit_artist_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .scroll_down(),
      ActiveLidarrBlock::EditArtistSelectQualityProfile => self
        .app
        .data
        .lidarr_data
        .edit_artist_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_down(),
      ActiveLidarrBlock::EditArtistSelectMetadataProfile => self
        .app
        .data
        .lidarr_data
        .edit_artist_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .scroll_down(),
      ActiveLidarrBlock::EditArtistPrompt => self.app.data.lidarr_data.selected_block.down(),
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::EditArtistSelectMonitorNewItems => self
        .app
        .data
        .lidarr_data
        .edit_artist_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .scroll_to_top(),
      ActiveLidarrBlock::EditArtistSelectQualityProfile => self
        .app
        .data
        .lidarr_data
        .edit_artist_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_to_top(),
      ActiveLidarrBlock::EditArtistSelectMetadataProfile => self
        .app
        .data
        .lidarr_data
        .edit_artist_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .scroll_to_top(),
      ActiveLidarrBlock::EditArtistPathInput => self
        .app
        .data
        .lidarr_data
        .edit_artist_modal
        .as_mut()
        .unwrap()
        .path
        .scroll_home(),
      ActiveLidarrBlock::EditArtistTagsInput => self
        .app
        .data
        .lidarr_data
        .edit_artist_modal
        .as_mut()
        .unwrap()
        .tags
        .scroll_home(),
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::EditArtistSelectMonitorNewItems => self
        .app
        .data
        .lidarr_data
        .edit_artist_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .scroll_to_bottom(),
      ActiveLidarrBlock::EditArtistSelectQualityProfile => self
        .app
        .data
        .lidarr_data
        .edit_artist_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_to_bottom(),
      ActiveLidarrBlock::EditArtistSelectMetadataProfile => self
        .app
        .data
        .lidarr_data
        .edit_artist_modal
        .as_mut()
        .unwrap()
        .metadata_profile_list
        .scroll_to_bottom(),
      ActiveLidarrBlock::EditArtistPathInput => self
        .app
        .data
        .lidarr_data
        .edit_artist_modal
        .as_mut()
        .unwrap()
        .path
        .reset_offset(),
      ActiveLidarrBlock::EditArtistTagsInput => self
        .app
        .data
        .lidarr_data
        .edit_artist_modal
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
      ActiveLidarrBlock::EditArtistPrompt => handle_prompt_toggle(self.app, self.key),
      ActiveLidarrBlock::EditArtistPathInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .lidarr_data
            .edit_artist_modal
            .as_mut()
            .unwrap()
            .path
        )
      }
      ActiveLidarrBlock::EditArtistTagsInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .lidarr_data
            .edit_artist_modal
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
      ActiveLidarrBlock::EditArtistPrompt => {
        match self.app.data.lidarr_data.selected_block.get_active_block() {
          ActiveLidarrBlock::EditArtistConfirmPrompt => {
            if self.app.data.lidarr_data.prompt_confirm {
              self.app.data.lidarr_data.prompt_confirm_action =
                Some(LidarrEvent::EditArtist(self.build_edit_artist_params()));
              self.app.should_refresh = true;
            }

            self.app.pop_navigation_stack();
          }
          ActiveLidarrBlock::EditArtistSelectMonitorNewItems
          | ActiveLidarrBlock::EditArtistSelectQualityProfile
          | ActiveLidarrBlock::EditArtistSelectMetadataProfile => self.app.push_navigation_stack(
            (
              self.app.data.lidarr_data.selected_block.get_active_block(),
              self.context,
            )
              .into(),
          ),
          ActiveLidarrBlock::EditArtistPathInput | ActiveLidarrBlock::EditArtistTagsInput => {
            self.app.push_navigation_stack(
              (
                self.app.data.lidarr_data.selected_block.get_active_block(),
                self.context,
              )
                .into(),
            );
            self.app.ignore_special_keys_for_textbox_input = true;
          }
          ActiveLidarrBlock::EditArtistToggleMonitored => {
            self
              .app
              .data
              .lidarr_data
              .edit_artist_modal
              .as_mut()
              .unwrap()
              .monitored = Some(
              !self
                .app
                .data
                .lidarr_data
                .edit_artist_modal
                .as_mut()
                .unwrap()
                .monitored
                .unwrap_or_default(),
            )
          }
          _ => (),
        }
      }
      ActiveLidarrBlock::EditArtistSelectMonitorNewItems
      | ActiveLidarrBlock::EditArtistSelectQualityProfile
      | ActiveLidarrBlock::EditArtistSelectMetadataProfile => self.app.pop_navigation_stack(),
      ActiveLidarrBlock::EditArtistPathInput | ActiveLidarrBlock::EditArtistTagsInput => {
        self.app.pop_navigation_stack();
        self.app.ignore_special_keys_for_textbox_input = false;
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::EditArtistTagsInput | ActiveLidarrBlock::EditArtistPathInput => {
        self.app.pop_navigation_stack();
        self.app.ignore_special_keys_for_textbox_input = false;
      }
      ActiveLidarrBlock::EditArtistPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.lidarr_data.edit_artist_modal = None;
        self.app.data.lidarr_data.prompt_confirm = false;
      }
      ActiveLidarrBlock::EditArtistSelectMonitorNewItems
      | ActiveLidarrBlock::EditArtistSelectQualityProfile
      | ActiveLidarrBlock::EditArtistSelectMetadataProfile => self.app.pop_navigation_stack(),
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_lidarr_block {
      ActiveLidarrBlock::EditArtistPathInput => {
        handle_text_box_keys!(
          self,
          key,
          self
            .app
            .data
            .lidarr_data
            .edit_artist_modal
            .as_mut()
            .unwrap()
            .path
        )
      }
      ActiveLidarrBlock::EditArtistTagsInput => {
        handle_text_box_keys!(
          self,
          key,
          self
            .app
            .data
            .lidarr_data
            .edit_artist_modal
            .as_mut()
            .unwrap()
            .tags
        )
      }
      ActiveLidarrBlock::EditArtistPrompt => {
        if self.app.data.lidarr_data.selected_block.get_active_block()
          == ActiveLidarrBlock::EditArtistConfirmPrompt
          && matches_key!(confirm, key)
        {
          self.app.data.lidarr_data.prompt_confirm = true;
          self.app.data.lidarr_data.prompt_confirm_action =
            Some(LidarrEvent::EditArtist(self.build_edit_artist_params()));
          self.app.should_refresh = true;

          self.app.pop_navigation_stack();
        }
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
