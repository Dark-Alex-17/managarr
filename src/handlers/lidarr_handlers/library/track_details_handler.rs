use crate::app::App;
use crate::event::Key;
use crate::handlers::KeyEventHandler;
use crate::handlers::lidarr_handlers::history::history_sorting_options;
use crate::handlers::table_handler::{TableHandlingConfig, handle_table};
use crate::matches_key;
use crate::models::Route;
use crate::models::lidarr_models::LidarrHistoryItem;
use crate::models::servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, TRACK_DETAILS_BLOCKS};

#[cfg(test)]
#[path = "track_details_handler_tests.rs"]
mod track_details_handler_tests;

pub(super) struct TrackDetailsHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_lidarr_block: ActiveLidarrBlock,
  _context: Option<ActiveLidarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveLidarrBlock> for TrackDetailsHandler<'a, 'b> {
  fn handle(&mut self) {
    let track_history_table_handling_config =
      TableHandlingConfig::new(ActiveLidarrBlock::TrackHistory.into())
        .sorting_block(ActiveLidarrBlock::TrackHistorySortPrompt.into())
        .sort_options(history_sorting_options())
        .searching_block(ActiveLidarrBlock::SearchTrackHistory.into())
        .search_error_block(ActiveLidarrBlock::SearchTrackHistoryError.into())
        .search_field_fn(|history_item: &LidarrHistoryItem| &history_item.source_title.text)
        .filtering_block(ActiveLidarrBlock::FilterTrackHistory.into())
        .filter_error_block(ActiveLidarrBlock::FilterTrackHistoryError.into())
        .filter_field_fn(|history_item: &LidarrHistoryItem| &history_item.source_title.text);

    if !handle_table(
      self,
      |app| {
        &mut app
          .data
          .lidarr_data
          .album_details_modal
          .as_mut()
          .expect("Album details modal is undefined")
          .track_details_modal
          .as_mut()
          .expect("Track details modal is undefined")
          .track_history
      },
      track_history_table_handling_config,
    ) {
      self.handle_key_event();
    }
  }

  fn accepts(active_block: ActiveLidarrBlock) -> bool {
    TRACK_DETAILS_BLOCKS.contains(&active_block)
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

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn is_ready(&self) -> bool {
    if self.app.is_loading {
      return false;
    }

    let Some(album_details_modal) = self.app.data.lidarr_data.album_details_modal.as_ref() else {
      return false;
    };

    let Some(track_details_modal) = &album_details_modal.track_details_modal else {
      return false;
    };

    match self.active_lidarr_block {
      ActiveLidarrBlock::TrackDetails => !track_details_modal.track_details.is_empty(),
      ActiveLidarrBlock::TrackHistory => !track_details_modal.track_history.is_empty(),
      _ => true,
    }
  }

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::TrackDetails | ActiveLidarrBlock::TrackHistory => match self.key {
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
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    if self.active_lidarr_block == ActiveLidarrBlock::TrackHistory {
      self
        .app
        .push_navigation_stack(ActiveLidarrBlock::TrackHistoryDetails.into());
    }
  }

  fn handle_esc(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::TrackDetails | ActiveLidarrBlock::TrackHistory => {
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
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::TrackDetails | ActiveLidarrBlock::TrackHistory => match self.key {
        _ if matches_key!(refresh, self.key) => {
          self
            .app
            .pop_and_push_navigation_stack(self.active_lidarr_block.into());
        }
        _ => (),
      },
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
