use crate::app::App;
use crate::event::Key;
use crate::handlers::lidarr_handlers::library::album_details_handler::releases_sorting_options;
use crate::handlers::table_handler::TableHandlingConfig;
use crate::handlers::{handle_prompt_toggle, KeyEventHandler};
use crate::models::servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, TRACK_DETAILS_BLOCKS};
use crate::models::lidarr_models::{LidarrHistoryItem, LidarrRelease, LidarrReleaseDownloadBody};
use crate::network::lidarr_network::LidarrEvent;
use crate::{handle_table_events, matches_key};

#[cfg(test)]
#[path = "track_details_handler_tests.rs"]
mod track_details_handler_tests;

pub(super) struct TrackDetailsHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_lidarr_block: ActiveLidarrBlock,
  _context: Option<ActiveLidarrBlock>,
}

impl TrackDetailsHandler<'_, '_> {
  handle_table_events!(
    self,
    track_history,
    self
      .app
      .data
      .lidarr_data
      .album_details_modal
      .as_mut()
      .expect("Album details modal is undefined")
      .track_details_modal
      .as_mut()
      .expect("Track details modal is undefined")
      .track_history,
    LidarrHistoryItem
  );
  handle_table_events!(
    self,
    track_releases,
    self
      .app
      .data
      .lidarr_data
      .album_details_modal
      .as_mut()
      .expect("Album details modal is undefined")
      .track_details_modal
      .as_mut()
      .expect("Track details modal is undefined")
      .track_releases,
    LidarrRelease
  );

  fn extract_track_id(&self) -> i64 {
    self
      .app
      .data
      .lidarr_data
      .album_details_modal
      .as_ref()
      .expect("Album details modal is undefined")
      .tracks
      .current_selection()
      .id
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveLidarrBlock> for TrackDetailsHandler<'a, 'b> {
  fn handle(&mut self) {
    let track_history_table_handling_config =
      TableHandlingConfig::new(ActiveLidarrBlock::TrackHistory.into());
    let track_releases_table_handling_config =
      TableHandlingConfig::new(ActiveLidarrBlock::ManualTrackSearch.into())
        .sorting_block(ActiveLidarrBlock::ManualTrackSearchSortPrompt.into())
        .sort_options(releases_sorting_options());

    if !self.handle_track_history_table_events(track_history_table_handling_config)
      && !self.handle_track_releases_table_events(track_releases_table_handling_config)
    {
      self.handle_key_event();
    }
  }

  fn accepts(active_block: ActiveLidarrBlock) -> bool {
    TRACK_DETAILS_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_lidarr_block: ActiveLidarrBlock,
    _context: Option<ActiveLidarrBlock>,
  ) -> Self {
    Self {
      key,
      app,
      active_lidarr_block,
      _context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    !self.app.is_loading
      && if let Some(album_details_modal) = self.app.data.lidarr_data.album_details_modal.as_ref()
      {
        if let Some(track_details_modal) = &album_details_modal.track_details_modal {
          match self.active_lidarr_block {
            ActiveLidarrBlock::TrackHistory => !track_details_modal.track_history.is_empty(),
            ActiveLidarrBlock::ManualTrackSearch => {
              !track_details_modal.track_releases.is_empty()
            }
            _ => true,
          }
        } else {
          false
        }
      } else {
        false
      }
  }

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::TrackDetails
      | ActiveLidarrBlock::TrackHistory
      | ActiveLidarrBlock::TrackFile
      | ActiveLidarrBlock::ManualTrackSearch => match self.key {
        _ if matches_key!(left, self.key) => {
          self
            .app
            .data
            .lidarr_data
            .album_details_modal
            .as_mut()
            .unwrap()
            .track_details_modal
            .as_mut()
            .unwrap()
            .track_details_tabs
            .previous();
          self.app.pop_and_push_navigation_stack(
            self
              .app
              .data
              .lidarr_data
              .album_details_modal
              .as_ref()
              .unwrap()
              .track_details_modal
              .as_ref()
              .unwrap()
              .track_details_tabs
              .get_active_route(),
          );
        }
        _ if matches_key!(right, self.key) => {
          self
            .app
            .data
            .lidarr_data
            .album_details_modal
            .as_mut()
            .unwrap()
            .track_details_modal
            .as_mut()
            .unwrap()
            .track_details_tabs
            .next();
          self.app.pop_and_push_navigation_stack(
            self
              .app
              .data
              .lidarr_data
              .album_details_modal
              .as_ref()
              .unwrap()
              .track_details_modal
              .as_ref()
              .unwrap()
              .track_details_tabs
              .get_active_route(),
          );
        }
        _ => (),
      },
      ActiveLidarrBlock::AutomaticallySearchTrackPrompt
      | ActiveLidarrBlock::ManualTrackSearchConfirmPrompt => {
        handle_prompt_toggle(self.app, self.key);
      }
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::TrackHistory => {
        self
          .app
          .push_navigation_stack(ActiveLidarrBlock::TrackHistoryDetails.into());
      }
      ActiveLidarrBlock::AutomaticallySearchTrackPrompt => {
        if self.app.data.lidarr_data.prompt_confirm {
          self.app.data.lidarr_data.prompt_confirm_action = Some(
            LidarrEvent::TriggerAutomaticTrackSearch(self.extract_track_id()),
          );
        }

        self.app.pop_navigation_stack();
      }
      ActiveLidarrBlock::ManualTrackSearch => {
        self
          .app
          .push_navigation_stack(ActiveLidarrBlock::ManualTrackSearchConfirmPrompt.into());
      }
      ActiveLidarrBlock::ManualTrackSearchConfirmPrompt => {
        if self.app.data.lidarr_data.prompt_confirm {
          let LidarrRelease {
            guid, indexer_id, ..
          } = self
            .app
            .data
            .lidarr_data
            .album_details_modal
            .as_ref()
            .unwrap()
            .track_details_modal
            .as_ref()
            .unwrap()
            .track_releases
            .current_selection();
          let track_id = self
            .app
            .data
            .lidarr_data
            .album_details_modal
            .as_ref()
            .unwrap()
            .tracks
            .current_selection()
            .id;
          let params = LidarrReleaseDownloadBody {
            guid: guid.clone(),
            indexer_id: *indexer_id,
            track_id: Some(track_id),
            ..LidarrReleaseDownloadBody::default()
          };
          self.app.data.lidarr_data.prompt_confirm_action =
            Some(LidarrEvent::DownloadRelease(params));
        }

        self.app.pop_navigation_stack();
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::TrackDetails
      | ActiveLidarrBlock::TrackFile
      | ActiveLidarrBlock::TrackHistory
      | ActiveLidarrBlock::ManualTrackSearch => {
        self.app.pop_navigation_stack();
        self
          .app
          .data
          .lidarr_data
          .album_details_modal
          .as_mut()
          .unwrap()
          .track_details_modal = None;
      }
      ActiveLidarrBlock::TrackHistoryDetails => {
        self.app.pop_navigation_stack();
      }
      ActiveLidarrBlock::AutomaticallySearchTrackPrompt
      | ActiveLidarrBlock::ManualTrackSearchConfirmPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.lidarr_data.prompt_confirm = false;
      }
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_lidarr_block {
      ActiveLidarrBlock::TrackDetails
      | ActiveLidarrBlock::TrackHistory
      | ActiveLidarrBlock::TrackFile
      | ActiveLidarrBlock::ManualTrackSearch => match self.key {
        _ if matches_key!(refresh, self.key) => {
          self
            .app
            .pop_and_push_navigation_stack(self.active_lidarr_block.into());
        }
        _ if matches_key!(auto_search, self.key) => {
          self
            .app
            .push_navigation_stack(ActiveLidarrBlock::AutomaticallySearchTrackPrompt.into());
        }
        _ => (),
      },
      ActiveLidarrBlock::AutomaticallySearchTrackPrompt if matches_key!(confirm, key) => {
        self.app.data.lidarr_data.prompt_confirm = true;
        self.app.data.lidarr_data.prompt_confirm_action = Some(
          LidarrEvent::TriggerAutomaticTrackSearch(self.extract_track_id()),
        );

        self.app.pop_navigation_stack();
      }
      ActiveLidarrBlock::ManualTrackSearchConfirmPrompt if matches_key!(confirm, key) => {
        if self.app.data.lidarr_data.prompt_confirm {
          let LidarrRelease {
            guid, indexer_id, ..
          } = self
            .app
            .data
            .lidarr_data
            .album_details_modal
            .as_ref()
            .unwrap()
            .track_details_modal
            .as_ref()
            .unwrap()
            .track_releases
            .current_selection();
          let track_id = self
            .app
            .data
            .lidarr_data
            .album_details_modal
            .as_ref()
            .unwrap()
            .tracks
            .current_selection()
            .id;
          let params = LidarrReleaseDownloadBody {
            guid: guid.clone(),
            indexer_id: *indexer_id,
            track_id: Some(track_id),
            ..LidarrReleaseDownloadBody::default()
          };
          self.app.data.lidarr_data.prompt_confirm_action =
            Some(LidarrEvent::DownloadRelease(params));
        }

        self.app.pop_navigation_stack();
      }
      _ => (),
    }
  }
}
