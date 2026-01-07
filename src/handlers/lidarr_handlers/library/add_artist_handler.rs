use crate::handlers::KeyEventHandler;
use crate::handlers::table_handler::{TableHandlingConfig, handle_table};
use crate::models::Route;
use crate::models::servarr_data::lidarr::lidarr_data::{ADD_ARTIST_BLOCKS, ActiveLidarrBlock};
use crate::{App, Key, handle_text_box_keys, handle_text_box_left_right_keys};

#[cfg(test)]
#[path = "add_artist_handler_tests.rs"]
mod add_artist_handler_tests;

pub struct AddArtistHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_lidarr_block: ActiveLidarrBlock,
  _context: Option<ActiveLidarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveLidarrBlock> for AddArtistHandler<'a, 'b> {
  fn handle(&mut self) {
    let add_artist_table_handling_config =
      TableHandlingConfig::new(ActiveLidarrBlock::AddArtistSearchResults.into());

    if !handle_table(
      self,
      |app| {
        app
          .data
          .lidarr_data
          .add_searched_artists
          .as_mut()
          .expect("add_searched_artists should be initialized")
      },
      add_artist_table_handling_config,
    ) {
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
    context: Option<ActiveLidarrBlock>,
  ) -> AddArtistHandler<'a, 'b> {
    AddArtistHandler {
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
    !self.app.is_loading
  }

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {
    if self.active_lidarr_block == ActiveLidarrBlock::AddArtistSearchInput {
      self
        .app
        .data
        .lidarr_data
        .add_artist_search
        .as_mut()
        .unwrap()
        .scroll_home();
    }
  }

  fn handle_end(&mut self) {
    if self.active_lidarr_block == ActiveLidarrBlock::AddArtistSearchInput {
      self
        .app
        .data
        .lidarr_data
        .add_artist_search
        .as_mut()
        .unwrap()
        .reset_offset();
    }
  }

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    if self.active_lidarr_block == ActiveLidarrBlock::AddArtistSearchInput {
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
  }

  fn handle_submit(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::AddArtistSearchInput => {
        let search_text = &self
          .app
          .data
          .lidarr_data
          .add_artist_search
          .as_ref()
          .unwrap()
          .text;

        if !search_text.is_empty() {
          self
            .app
            .push_navigation_stack(ActiveLidarrBlock::AddArtistSearchResults.into());
          self.app.ignore_special_keys_for_textbox_input = false;
        }
      }
      ActiveLidarrBlock::AddArtistSearchResults => {}
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
        self.app.data.lidarr_data.add_searched_artists = None;
        self.app.ignore_special_keys_for_textbox_input = true;
      }
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    if self.active_lidarr_block == ActiveLidarrBlock::AddArtistSearchInput {
      handle_text_box_keys!(
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
  }

  fn app_mut(&mut self) -> &mut App<'b> {
    self.app
  }

  fn current_route(&self) -> Route {
    self.app.get_current_route()
  }
}
