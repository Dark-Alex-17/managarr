use crate::app::App;
use crate::event::Key;
use crate::handlers::lidarr_handlers::history::history_sorting_options;
use crate::handlers::table_handler::TableHandlingConfig;
use crate::handlers::{handle_prompt_toggle, KeyEventHandler};
use crate::models::lidarr_models::{Album, LidarrHistoryItem};
use crate::models::servarr_data::lidarr::lidarr_data::{
  ActiveLidarrBlock, ARTIST_DETAILS_BLOCKS, EDIT_ARTIST_SELECTION_BLOCKS,
};
use crate::models::BlockSelectionState;
use crate::network::lidarr_network::LidarrEvent;
use crate::{handle_table_events, matches_key};

#[cfg(test)]
#[path = "artist_details_handler_tests.rs"]
mod artist_details_handler_tests;

pub(super) struct ArtistDetailsHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_lidarr_block: ActiveLidarrBlock,
  _context: Option<ActiveLidarrBlock>,
}

impl ArtistDetailsHandler<'_, '_> {
  handle_table_events!(self, album, self.app.data.lidarr_data.albums, Album);
  handle_table_events!(
    self,
    artist_history,
    self
      .app
      .data
      .lidarr_data
      .artist_history
      .as_mut()
      .expect("Artist history is undefined"),
    LidarrHistoryItem
  );

  fn extract_artist_id_album_id_tuple(&self) -> (i64, i64) {
    let artist_id = self.app.data.lidarr_data.artists.current_selection().id;
    let album_id = self
      .app
      .data
      .lidarr_data
      .albums
      .current_selection()
      .id;

    (artist_id, album_id)
  }

  fn extract_artist_id(&self) -> i64 {
    self.app.data.lidarr_data.artists.current_selection().id
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveLidarrBlock> for ArtistDetailsHandler<'a, 'b> {
  fn handle(&mut self) {
    let album_table_handling_config =
      TableHandlingConfig::new(ActiveLidarrBlock::ArtistDetails.into())
        .searching_block(ActiveLidarrBlock::SearchAlbum.into())
        .search_error_block(ActiveLidarrBlock::SearchAlbumError.into())
        .search_field_fn(|album: &Album| {
          &album
            .title
            .text
        });
    let artist_history_table_handling_config =
      TableHandlingConfig::new(ActiveLidarrBlock::ArtistHistory.into())
        .sorting_block(ActiveLidarrBlock::ArtistHistorySortPrompt.into())
        .sort_options(history_sorting_options())
        .sort_by_fn(|a: &LidarrHistoryItem, b: &LidarrHistoryItem| a.id.cmp(&b.id))
        .searching_block(ActiveLidarrBlock::SearchArtistHistory.into())
        .search_error_block(ActiveLidarrBlock::SearchArtistHistoryError.into())
        .search_field_fn(|history_item: &LidarrHistoryItem| &history_item.source_title.text)
        .filtering_block(ActiveLidarrBlock::FilterArtistHistory.into())
        .filter_error_block(ActiveLidarrBlock::FilterArtistHistoryError.into())
        .filter_field_fn(|history_item: &LidarrHistoryItem| &history_item.source_title.text);

    if !self.handle_album_table_events(album_table_handling_config)
      && !self.handle_artist_history_table_events(artist_history_table_handling_config)
    {
      self.handle_key_event();
    }
  }

  fn accepts(active_block: ActiveLidarrBlock) -> bool {
    ARTIST_DETAILS_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveLidarrBlock,
    _context: Option<ActiveLidarrBlock>,
  ) -> ArtistDetailsHandler<'a, 'b> {
    ArtistDetailsHandler {
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

  fn handle_delete(&mut self) {}

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
        handle_prompt_toggle(self.app, self.key)
      }
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::ArtistDetails if !self.app.data.lidarr_data.albums.is_empty() => {
        self
          .app
          .push_navigation_stack(ActiveLidarrBlock::AlbumDetails.into());
      }
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
          self.app.data.lidarr_data.prompt_confirm = true;
          self.app.data.lidarr_data.prompt_confirm_action = Some(
            LidarrEvent::ToggleAlbumMonitoring(self.extract_artist_id_album_id_tuple()),
          );

          self
            .app
            .pop_and_push_navigation_stack(self.active_lidarr_block.into());
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
        if self.app.data.lidarr_data.prompt_confirm {
          self.app.data.lidarr_data.prompt_confirm_action =
            Some(LidarrEvent::UpdateAndScanArtist(self.extract_artist_id()));
        }

        self.app.pop_navigation_stack();
      }
      _ => (),
    }
  }
}
