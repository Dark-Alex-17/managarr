use crate::app::App;
use crate::event::Key;
use crate::handlers::lidarr_handlers::history::history_sorting_options;
use crate::handlers::table_handler::{TableHandlingConfig, handle_table};
use crate::handlers::{KeyEventHandler, handle_prompt_toggle};
use crate::matches_key;
use crate::models::Route;
use crate::models::lidarr_models::{
  LidarrHistoryItem, LidarrRelease, LidarrReleaseDownloadBody, Track,
};
use crate::models::servarr_data::lidarr::lidarr_data::{ALBUM_DETAILS_BLOCKS, ActiveLidarrBlock};
use crate::models::stateful_table::SortOption;
use crate::network::lidarr_network::LidarrEvent;
use serde_json::Number;

#[cfg(test)]
#[path = "album_details_handler_tests.rs"]
mod album_details_handler_tests;

pub(in crate::handlers::lidarr_handlers) struct AlbumDetailsHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_lidarr_block: ActiveLidarrBlock,
  _context: Option<ActiveLidarrBlock>,
}

impl AlbumDetailsHandler<'_, '_> {
  fn extract_track_file_id(&self) -> i64 {
    self
      .app
      .data
      .lidarr_data
      .album_details_modal
      .as_ref()
      .expect("Album details have not been loaded")
      .tracks
      .current_selection()
      .track_file_id
  }

  fn extract_album_id(&self) -> i64 {
    self.app.data.lidarr_data.albums.current_selection().id
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveLidarrBlock> for AlbumDetailsHandler<'a, 'b> {
  fn handle(&mut self) {
    let tracks_table_handling_config =
      TableHandlingConfig::new(ActiveLidarrBlock::AlbumDetails.into())
        .searching_block(ActiveLidarrBlock::SearchTracks.into())
        .search_error_block(ActiveLidarrBlock::SearchTracksError.into())
        .search_field_fn(|track: &Track| &track.title);
    let album_history_table_handling_config =
      TableHandlingConfig::new(ActiveLidarrBlock::AlbumHistory.into())
        .sorting_block(ActiveLidarrBlock::AlbumHistorySortPrompt.into())
        .sort_options(history_sorting_options())
        .searching_block(ActiveLidarrBlock::SearchAlbumHistory.into())
        .search_error_block(ActiveLidarrBlock::SearchAlbumHistoryError.into())
        .search_field_fn(|history_item: &LidarrHistoryItem| &history_item.source_title.text)
        .filtering_block(ActiveLidarrBlock::FilterAlbumHistory.into())
        .filter_error_block(ActiveLidarrBlock::FilterAlbumHistoryError.into())
        .filter_field_fn(|history_item: &LidarrHistoryItem| &history_item.source_title.text);
    let album_releases_table_handling_config =
      TableHandlingConfig::new(ActiveLidarrBlock::ManualAlbumSearch.into())
        .sorting_block(ActiveLidarrBlock::ManualAlbumSearchSortPrompt.into())
        .sort_options(releases_sorting_options());

    if !handle_table(
      self,
      |app| {
        &mut app
          .data
          .lidarr_data
          .album_details_modal
          .as_mut()
          .expect("Album details modal is undefined")
          .tracks
      },
      tracks_table_handling_config,
    ) && !handle_table(
      self,
      |app| {
        &mut app
          .data
          .lidarr_data
          .album_details_modal
          .as_mut()
          .expect("Album details modal is undefined")
          .album_history
      },
      album_history_table_handling_config,
    ) && !handle_table(
      self,
      |app| {
        &mut app
          .data
          .lidarr_data
          .album_details_modal
          .as_mut()
          .expect("Album details modal is undefined")
          .album_releases
      },
      album_releases_table_handling_config,
    ) {
      self.handle_key_event();
    }
  }

  fn accepts(active_block: ActiveLidarrBlock) -> bool {
    ALBUM_DETAILS_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveLidarrBlock,
    context: Option<ActiveLidarrBlock>,
  ) -> AlbumDetailsHandler<'a, 'b> {
    AlbumDetailsHandler {
      key,
      app,
      active_lidarr_block: active_block,
      _context: context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    if self.app.is_loading {
      return false;
    }

    let Some(album_details_modal) = &self.app.data.lidarr_data.album_details_modal else {
      return false;
    };

    match self.active_lidarr_block {
      ActiveLidarrBlock::AlbumDetails => !album_details_modal.tracks.is_empty(),
      ActiveLidarrBlock::AlbumHistory => !album_details_modal.album_history.is_empty(),
      ActiveLidarrBlock::ManualAlbumSearch => !album_details_modal.album_releases.is_empty(),
      _ => true,
    }
  }

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {
    if self.active_lidarr_block == ActiveLidarrBlock::AlbumDetails {
      self
        .app
        .push_navigation_stack(ActiveLidarrBlock::DeleteTrackFilePrompt.into());
    }
  }

  fn handle_left_right_action(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::AlbumDetails
      | ActiveLidarrBlock::AlbumHistory
      | ActiveLidarrBlock::ManualAlbumSearch => match self.key {
        _ if matches_key!(left, self.key) => {
          self
            .app
            .data
            .lidarr_data
            .album_details_modal
            .as_mut()
            .unwrap()
            .album_details_tabs
            .previous();
          self.app.pop_and_push_navigation_stack(
            self
              .app
              .data
              .lidarr_data
              .album_details_modal
              .as_ref()
              .unwrap()
              .album_details_tabs
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
            .album_details_tabs
            .next();
          self.app.pop_and_push_navigation_stack(
            self
              .app
              .data
              .lidarr_data
              .album_details_modal
              .as_ref()
              .unwrap()
              .album_details_tabs
              .get_active_route(),
          );
        }
        _ => (),
      },
      ActiveLidarrBlock::AutomaticallySearchAlbumPrompt
      | ActiveLidarrBlock::ManualAlbumSearchConfirmPrompt
      | ActiveLidarrBlock::DeleteTrackFilePrompt => {
        handle_prompt_toggle(self.app, self.key);
      }
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::AlbumDetails
        if self.app.data.lidarr_data.album_details_modal.is_some()
          && !self
            .app
            .data
            .lidarr_data
            .album_details_modal
            .as_ref()
            .unwrap()
            .tracks
            .is_empty() =>
      {
        self
          .app
          .push_navigation_stack(ActiveLidarrBlock::TrackDetails.into())
      }
      ActiveLidarrBlock::AlbumHistory => self
        .app
        .push_navigation_stack(ActiveLidarrBlock::AlbumHistoryDetails.into()),
      ActiveLidarrBlock::DeleteTrackFilePrompt => {
        if self.app.data.lidarr_data.prompt_confirm {
          self.app.data.lidarr_data.prompt_confirm_action =
            Some(LidarrEvent::DeleteTrackFile(self.extract_track_file_id()));
        }

        self.app.pop_navigation_stack();
      }
      ActiveLidarrBlock::AutomaticallySearchAlbumPrompt => {
        if self.app.data.lidarr_data.prompt_confirm {
          self.app.data.lidarr_data.prompt_confirm_action = Some(
            LidarrEvent::TriggerAutomaticAlbumSearch(self.extract_album_id()),
          );
        }

        self.app.pop_navigation_stack();
      }
      ActiveLidarrBlock::ManualAlbumSearch => {
        self
          .app
          .push_navigation_stack(ActiveLidarrBlock::ManualAlbumSearchConfirmPrompt.into());
      }
      ActiveLidarrBlock::ManualAlbumSearchConfirmPrompt => {
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
            .album_releases
            .current_selection();
          let params = LidarrReleaseDownloadBody {
            guid: guid.clone(),
            indexer_id: *indexer_id,
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
      ActiveLidarrBlock::AlbumDetails | ActiveLidarrBlock::ManualAlbumSearch => {
        self.app.pop_navigation_stack();
        self.app.data.lidarr_data.album_details_modal = None;
      }
      ActiveLidarrBlock::AlbumHistoryDetails => {
        self.app.pop_navigation_stack();
      }
      ActiveLidarrBlock::AlbumHistory => {
        if self
          .app
          .data
          .lidarr_data
          .album_details_modal
          .as_ref()
          .unwrap()
          .album_history
          .filtered_items
          .is_some()
        {
          self
            .app
            .data
            .lidarr_data
            .album_details_modal
            .as_mut()
            .unwrap()
            .album_history
            .filtered_items = None;
        } else {
          self.app.pop_navigation_stack();
          self.app.data.lidarr_data.album_details_modal = None;
        }
      }
      ActiveLidarrBlock::AutomaticallySearchAlbumPrompt
      | ActiveLidarrBlock::ManualAlbumSearchConfirmPrompt
      | ActiveLidarrBlock::DeleteTrackFilePrompt => {
        self.app.pop_navigation_stack();
        self.app.data.lidarr_data.prompt_confirm = false;
      }
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_lidarr_block {
      ActiveLidarrBlock::AlbumDetails
      | ActiveLidarrBlock::AlbumHistory
      | ActiveLidarrBlock::ManualAlbumSearch => match self.key {
        _ if matches_key!(refresh, self.key) => {
          self
            .app
            .pop_and_push_navigation_stack(self.active_lidarr_block.into());
        }
        _ if matches_key!(auto_search, self.key) => {
          self
            .app
            .push_navigation_stack(ActiveLidarrBlock::AutomaticallySearchAlbumPrompt.into());
        }
        _ => (),
      },
      ActiveLidarrBlock::AutomaticallySearchAlbumPrompt if matches_key!(confirm, key) => {
        self.app.data.lidarr_data.prompt_confirm = true;
        self.app.data.lidarr_data.prompt_confirm_action = Some(
          LidarrEvent::TriggerAutomaticAlbumSearch(self.extract_album_id()),
        );

        self.app.pop_navigation_stack();
      }
      ActiveLidarrBlock::DeleteTrackFilePrompt if matches_key!(confirm, key) => {
        self.app.data.lidarr_data.prompt_confirm = true;
        self.app.data.lidarr_data.prompt_confirm_action =
          Some(LidarrEvent::DeleteTrackFile(self.extract_track_file_id()));

        self.app.pop_navigation_stack();
      }
      ActiveLidarrBlock::ManualAlbumSearchConfirmPrompt if matches_key!(confirm, key) => {
        self.app.data.lidarr_data.prompt_confirm = true;
        let LidarrRelease {
          guid, indexer_id, ..
        } = self
          .app
          .data
          .lidarr_data
          .album_details_modal
          .as_ref()
          .unwrap()
          .album_releases
          .current_selection();
        let params = LidarrReleaseDownloadBody {
          guid: guid.clone(),
          indexer_id: *indexer_id,
        };
        self.app.data.lidarr_data.prompt_confirm_action =
          Some(LidarrEvent::DownloadRelease(params));

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

pub(in crate::handlers::lidarr_handlers::library) fn releases_sorting_options()
-> Vec<SortOption<LidarrRelease>> {
  vec![
    SortOption {
      name: "Source",
      cmp_fn: Some(|a, b| a.protocol.cmp(&b.protocol)),
    },
    SortOption {
      name: "Age",
      cmp_fn: Some(|a, b| a.age.cmp(&b.age)),
    },
    SortOption {
      name: "Rejected",
      cmp_fn: Some(|a, b| a.rejected.cmp(&b.rejected)),
    },
    SortOption {
      name: "Title",
      cmp_fn: Some(|a, b| {
        a.title
          .text
          .to_lowercase()
          .cmp(&b.title.text.to_lowercase())
      }),
    },
    SortOption {
      name: "Indexer",
      cmp_fn: Some(|a, b| a.indexer.to_lowercase().cmp(&b.indexer.to_lowercase())),
    },
    SortOption {
      name: "Size",
      cmp_fn: Some(|a, b| a.size.cmp(&b.size)),
    },
    SortOption {
      name: "Peers",
      cmp_fn: Some(|a, b| {
        let default_number = Number::from(i64::MAX);
        let seeder_a = a
          .seeders
          .as_ref()
          .unwrap_or(&default_number)
          .as_u64()
          .unwrap();
        let seeder_b = b
          .seeders
          .as_ref()
          .unwrap_or(&default_number)
          .as_u64()
          .unwrap();

        seeder_a.cmp(&seeder_b)
      }),
    },
    SortOption {
      name: "Quality",
      cmp_fn: Some(|a, b| a.quality.cmp(&b.quality)),
    },
  ]
}
