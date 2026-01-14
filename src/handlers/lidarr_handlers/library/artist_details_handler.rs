use crate::app::App;
use crate::event::Key;
use crate::handlers::lidarr_handlers::history::history_sorting_options;
use crate::handlers::lidarr_handlers::library::delete_album_handler::DeleteAlbumHandler;
use crate::handlers::table_handler::{TableHandlingConfig, handle_table};
use crate::handlers::{KeyEventHandler, handle_prompt_toggle};
use crate::matches_key;
use crate::models::lidarr_models::{Album, LidarrHistoryItem};
use crate::models::servarr_data::lidarr::lidarr_data::{
  ARTIST_DETAILS_BLOCKS, ActiveLidarrBlock, DELETE_ALBUM_SELECTION_BLOCKS,
  EDIT_ARTIST_SELECTION_BLOCKS,
};
use crate::models::{BlockSelectionState, Route};
use crate::network::lidarr_network::LidarrEvent;

#[cfg(test)]
#[path = "artist_details_handler_tests.rs"]
mod artist_details_handler_tests;

pub struct ArtistDetailsHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_lidarr_block: ActiveLidarrBlock,
  context: Option<ActiveLidarrBlock>,
}

impl ArtistDetailsHandler<'_, '_> {
  fn extract_artist_id(&self) -> i64 {
    self.app.data.lidarr_data.artists.current_selection().id
  }

  fn extract_album_id(&self) -> i64 {
    self.app.data.lidarr_data.albums.current_selection().id
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveLidarrBlock> for ArtistDetailsHandler<'a, 'b> {
  fn handle(&mut self) {
    let albums_table_handling_config =
      TableHandlingConfig::new(ActiveLidarrBlock::ArtistDetails.into())
        .searching_block(ActiveLidarrBlock::SearchAlbums.into())
        .search_error_block(ActiveLidarrBlock::SearchAlbumsError.into())
        .search_field_fn(|album: &Album| &album.title.text);

    let artist_history_table_handling_config =
      TableHandlingConfig::new(ActiveLidarrBlock::ArtistHistory.into())
        .sorting_block(ActiveLidarrBlock::ArtistHistorySortPrompt.into())
        .sort_options(history_sorting_options())
        .searching_block(ActiveLidarrBlock::SearchArtistHistory.into())
        .search_error_block(ActiveLidarrBlock::SearchArtistHistoryError.into())
        .search_field_fn(|history_item: &LidarrHistoryItem| &history_item.source_title.text)
        .filtering_block(ActiveLidarrBlock::FilterArtistHistory.into())
        .filter_error_block(ActiveLidarrBlock::FilterArtistHistoryError.into())
        .filter_field_fn(|history_item: &LidarrHistoryItem| &history_item.source_title.text);

    if !handle_table(
      self,
      |app| &mut app.data.lidarr_data.albums,
      albums_table_handling_config,
    ) && !handle_table(
      self,
      |app| {
        app
          .data
          .lidarr_data
          .artist_history
          .as_mut()
          .expect("Artist history is undefined")
      },
      artist_history_table_handling_config,
    ) {
      match self.active_lidarr_block {
        _ if DeleteAlbumHandler::accepts(self.active_lidarr_block) => {
          DeleteAlbumHandler::new(self.key, self.app, self.active_lidarr_block, self.context)
            .handle();
        }
        _ => self.handle_key_event(),
      };
    }
  }

  fn accepts(active_block: ActiveLidarrBlock) -> bool {
    DeleteAlbumHandler::accepts(active_block) || ARTIST_DETAILS_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveLidarrBlock,
    context: Option<ActiveLidarrBlock>,
  ) -> ArtistDetailsHandler<'a, 'b> {
    ArtistDetailsHandler {
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
    if self.active_lidarr_block == ActiveLidarrBlock::ArtistHistory {
      !self.app.is_loading && self.app.data.lidarr_data.artist_history.is_some()
    } else {
      !self.app.is_loading
    }
  }

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {
    if self.active_lidarr_block == ActiveLidarrBlock::ArtistDetails {
      self
        .app
        .push_navigation_stack(ActiveLidarrBlock::DeleteAlbumPrompt.into());
      self.app.data.lidarr_data.selected_block =
        BlockSelectionState::new(DELETE_ALBUM_SELECTION_BLOCKS);
    }
  }

  fn handle_left_right_action(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::ArtistDetails | ActiveLidarrBlock::ArtistHistory => match self.key {
        _ if matches_key!(left, self.key) => {
          self.app.data.lidarr_data.artist_info_tabs.previous();
          self.app.pop_and_push_navigation_stack(
            self
              .app
              .data
              .lidarr_data
              .artist_info_tabs
              .get_active_route(),
          );
        }
        _ if matches_key!(right, self.key) => {
          self.app.data.lidarr_data.artist_info_tabs.next();
          self.app.pop_and_push_navigation_stack(
            self
              .app
              .data
              .lidarr_data
              .artist_info_tabs
              .get_active_route(),
          );
        }
        _ => (),
      },
      ActiveLidarrBlock::UpdateAndScanArtistPrompt
      | ActiveLidarrBlock::AutomaticallySearchArtistPrompt => {
        handle_prompt_toggle(self.app, self.key);
      }
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::ArtistHistory
        if !self
          .app
          .data
          .lidarr_data
          .artist_history
          .as_ref()
          .expect("Artist history should be Some")
          .is_empty() =>
      {
        self
          .app
          .push_navigation_stack(ActiveLidarrBlock::ArtistHistoryDetails.into());
      }
      ActiveLidarrBlock::AutomaticallySearchArtistPrompt => {
        if self.app.data.lidarr_data.prompt_confirm {
          self.app.data.lidarr_data.prompt_confirm_action = Some(
            LidarrEvent::TriggerAutomaticArtistSearch(self.extract_artist_id()),
          );
        }

        self.app.pop_navigation_stack();
      }
      ActiveLidarrBlock::UpdateAndScanArtistPrompt => {
        if self.app.data.lidarr_data.prompt_confirm {
          self.app.data.lidarr_data.prompt_confirm_action =
            Some(LidarrEvent::UpdateAndScanArtist(self.extract_artist_id()));
        }

        self.app.pop_navigation_stack();
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::UpdateAndScanArtistPrompt
      | ActiveLidarrBlock::AutomaticallySearchArtistPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.lidarr_data.prompt_confirm = false;
      }
      ActiveLidarrBlock::ArtistHistoryDetails => {
        self.app.pop_navigation_stack();
      }
      ActiveLidarrBlock::ArtistHistory => {
        if self
          .app
          .data
          .lidarr_data
          .artist_history
          .as_ref()
          .expect("Artist history is not populated")
          .filtered_items
          .is_some()
        {
          self
            .app
            .data
            .lidarr_data
            .artist_history
            .as_mut()
            .expect("Artist history is not populated")
            .reset_filter();
        } else {
          self.app.pop_navigation_stack();
          self.app.data.lidarr_data.reset_artist_info_tabs();
        }
      }
      ActiveLidarrBlock::ArtistDetails => {
        self.app.pop_navigation_stack();
        self.app.data.lidarr_data.reset_artist_info_tabs();
      }
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_lidarr_block {
      ActiveLidarrBlock::ArtistDetails => match self.key {
        _ if matches_key!(refresh, key) => self
          .app
          .pop_and_push_navigation_stack(self.active_lidarr_block.into()),
        _ if matches_key!(auto_search, key) => {
          self
            .app
            .push_navigation_stack(ActiveLidarrBlock::AutomaticallySearchArtistPrompt.into());
        }
        _ if matches_key!(update, key) => {
          self
            .app
            .push_navigation_stack(ActiveLidarrBlock::UpdateAndScanArtistPrompt.into());
        }
        _ if matches_key!(edit, key) => {
          self.app.push_navigation_stack(
            (
              ActiveLidarrBlock::EditArtistPrompt,
              Some(self.active_lidarr_block),
            )
              .into(),
          );
          self.app.data.lidarr_data.edit_artist_modal = Some((&self.app.data.lidarr_data).into());
          self.app.data.lidarr_data.selected_block =
            BlockSelectionState::new(EDIT_ARTIST_SELECTION_BLOCKS);
        }
        _ if matches_key!(toggle_monitoring, key) => {
          if !self.app.data.lidarr_data.albums.is_empty() {
            self.app.data.lidarr_data.prompt_confirm = true;
            self.app.data.lidarr_data.prompt_confirm_action =
              Some(LidarrEvent::ToggleAlbumMonitoring(self.extract_album_id()));

            self
              .app
              .pop_and_push_navigation_stack(self.active_lidarr_block.into());
          }
        }
        _ => (),
      },
      ActiveLidarrBlock::ArtistHistory => match self.key {
        _ if matches_key!(refresh, key) => self
          .app
          .pop_and_push_navigation_stack(self.active_lidarr_block.into()),
        _ if matches_key!(auto_search, key) => {
          self
            .app
            .push_navigation_stack(ActiveLidarrBlock::AutomaticallySearchArtistPrompt.into());
        }
        _ if matches_key!(edit, key) => {
          self.app.push_navigation_stack(
            (
              ActiveLidarrBlock::EditArtistPrompt,
              Some(self.active_lidarr_block),
            )
              .into(),
          );
          self.app.data.lidarr_data.edit_artist_modal = Some((&self.app.data.lidarr_data).into());
          self.app.data.lidarr_data.selected_block =
            BlockSelectionState::new(EDIT_ARTIST_SELECTION_BLOCKS);
        }
        _ if matches_key!(update, key) => {
          self
            .app
            .push_navigation_stack(ActiveLidarrBlock::UpdateAndScanArtistPrompt.into());
        }
        _ => (),
      },
      ActiveLidarrBlock::AutomaticallySearchArtistPrompt => {
        if matches_key!(confirm, key) {
          self.app.data.lidarr_data.prompt_confirm = true;
          self.app.data.lidarr_data.prompt_confirm_action = Some(
            LidarrEvent::TriggerAutomaticArtistSearch(self.extract_artist_id()),
          );

          self.app.pop_navigation_stack();
        }
      }
      ActiveLidarrBlock::UpdateAndScanArtistPrompt => {
        if matches_key!(confirm, key) {
          self.app.data.lidarr_data.prompt_confirm = true;
          self.app.data.lidarr_data.prompt_confirm_action =
            Some(LidarrEvent::UpdateAndScanArtist(self.extract_artist_id()));

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
