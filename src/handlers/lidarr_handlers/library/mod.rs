use crate::{
  app::App,
  event::Key,
  handlers::{KeyEventHandler, handle_clear_errors, handle_prompt_toggle},
  matches_key,
  models::{
    BlockSelectionState, HorizontallyScrollableText,
    lidarr_models::Artist,
    servarr_data::lidarr::lidarr_data::{
      ActiveLidarrBlock, DELETE_ARTIST_SELECTION_BLOCKS, EDIT_ARTIST_SELECTION_BLOCKS,
      LIBRARY_BLOCKS,
    },
    stateful_table::SortOption,
  },
  network::lidarr_network::LidarrEvent,
};

use super::handle_change_tab_left_right_keys;
use crate::handlers::table_handler::{TableHandlingConfig, handle_table};

mod add_artist_handler;
mod delete_artist_handler;
mod edit_artist_handler;

use crate::models::Route;
pub(in crate::handlers::lidarr_handlers) use add_artist_handler::AddArtistHandler;
pub(in crate::handlers::lidarr_handlers) use delete_artist_handler::DeleteArtistHandler;
pub(in crate::handlers::lidarr_handlers) use edit_artist_handler::EditArtistHandler;

#[cfg(test)]
#[path = "library_handler_tests.rs"]
mod library_handler_tests;

pub(super) struct LibraryHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_lidarr_block: ActiveLidarrBlock,
  context: Option<ActiveLidarrBlock>,
}

impl LibraryHandler<'_, '_> {
  fn extract_artist_id(&self) -> i64 {
    self.app.data.lidarr_data.artists.current_selection().id
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveLidarrBlock> for LibraryHandler<'a, 'b> {
  fn handle(&mut self) {
    let artists_table_handling_config = TableHandlingConfig::new(ActiveLidarrBlock::Artists.into())
      .sorting_block(ActiveLidarrBlock::ArtistsSortPrompt.into())
      .sort_options(artists_sorting_options())
      .searching_block(ActiveLidarrBlock::SearchArtists.into())
      .search_error_block(ActiveLidarrBlock::SearchArtistsError.into())
      .search_field_fn(|artist| &artist.artist_name.text)
      .filtering_block(ActiveLidarrBlock::FilterArtists.into())
      .filter_error_block(ActiveLidarrBlock::FilterArtistsError.into())
      .filter_field_fn(|artist| &artist.artist_name.text);

    if !handle_table(
      self,
      |app| &mut app.data.lidarr_data.artists,
      artists_table_handling_config,
    ) {
      match self.active_lidarr_block {
        _ if AddArtistHandler::accepts(self.active_lidarr_block) => {
          AddArtistHandler::new(self.key, self.app, self.active_lidarr_block, self.context)
            .handle();
        }
        _ if DeleteArtistHandler::accepts(self.active_lidarr_block) => {
          DeleteArtistHandler::new(self.key, self.app, self.active_lidarr_block, self.context)
            .handle();
        }
        _ if EditArtistHandler::accepts(self.active_lidarr_block) => {
          EditArtistHandler::new(self.key, self.app, self.active_lidarr_block, self.context)
            .handle();
        }
        _ => self.handle_key_event(),
      }
    }
  }

  fn accepts(active_block: ActiveLidarrBlock) -> bool {
    AddArtistHandler::accepts(active_block)
      || DeleteArtistHandler::accepts(active_block)
      || EditArtistHandler::accepts(active_block)
      || LIBRARY_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveLidarrBlock,
    context: Option<ActiveLidarrBlock>,
  ) -> LibraryHandler<'a, 'b> {
    LibraryHandler {
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
    !self.app.is_loading && !self.app.data.lidarr_data.artists.is_empty()
  }

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {
    if self.active_lidarr_block == ActiveLidarrBlock::Artists {
      self
        .app
        .push_navigation_stack(ActiveLidarrBlock::DeleteArtistPrompt.into());
      self.app.data.lidarr_data.selected_block =
        BlockSelectionState::new(DELETE_ARTIST_SELECTION_BLOCKS);
    }
  }

  fn handle_left_right_action(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::Artists => handle_change_tab_left_right_keys(self.app, self.key),
      ActiveLidarrBlock::UpdateAllArtistsPrompt => handle_prompt_toggle(self.app, self.key),
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    if self.active_lidarr_block == ActiveLidarrBlock::UpdateAllArtistsPrompt {
      if self.app.data.lidarr_data.prompt_confirm {
        self.app.data.lidarr_data.prompt_confirm_action = Some(LidarrEvent::UpdateAllArtists);
      }

      self.app.pop_navigation_stack();
    }
  }

  fn handle_esc(&mut self) {
    match self.active_lidarr_block {
      ActiveLidarrBlock::UpdateAllArtistsPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.lidarr_data.prompt_confirm = false;
      }
      _ => {
        handle_clear_errors(self.app);
      }
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_lidarr_block {
      ActiveLidarrBlock::Artists => match key {
        _ if matches_key!(add, key) => {
          self
            .app
            .push_navigation_stack(ActiveLidarrBlock::AddArtistSearchInput.into());
          self.app.data.lidarr_data.add_artist_search = Some(HorizontallyScrollableText::default());
          self.app.ignore_special_keys_for_textbox_input = true;
        }
        _ if matches_key!(toggle_monitoring, key) => {
          self.app.data.lidarr_data.prompt_confirm = true;
          self.app.data.lidarr_data.prompt_confirm_action = Some(
            LidarrEvent::ToggleArtistMonitoring(self.extract_artist_id()),
          );

          self
            .app
            .pop_and_push_navigation_stack(self.active_lidarr_block.into());
        }
        _ if matches_key!(edit, key) => {
          self.app.data.lidarr_data.edit_artist_modal = Some((&self.app.data.lidarr_data).into());
          self
            .app
            .push_navigation_stack(ActiveLidarrBlock::EditArtistPrompt.into());
          self.app.data.lidarr_data.selected_block =
            BlockSelectionState::new(EDIT_ARTIST_SELECTION_BLOCKS);
        }
        _ if matches_key!(update, key) => {
          self
            .app
            .push_navigation_stack(ActiveLidarrBlock::UpdateAllArtistsPrompt.into());
        }
        _ if matches_key!(refresh, key) => {
          self.app.should_refresh = true;
        }
        _ => (),
      },
      ActiveLidarrBlock::UpdateAllArtistsPrompt => {
        if matches_key!(confirm, key) {
          self.app.data.lidarr_data.prompt_confirm = true;
          self.app.data.lidarr_data.prompt_confirm_action = Some(LidarrEvent::UpdateAllArtists);

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

fn artists_sorting_options() -> Vec<SortOption<Artist>> {
  vec![
    SortOption {
      name: "Name",
      cmp_fn: Some(|a, b| {
        a.artist_name
          .text
          .to_lowercase()
          .cmp(&b.artist_name.text.to_lowercase())
      }),
    },
    SortOption {
      name: "Type",
      cmp_fn: Some(|a, b| {
        a.artist_type
          .as_ref()
          .unwrap_or(&String::new())
          .to_lowercase()
          .cmp(
            &b.artist_type
              .as_ref()
              .unwrap_or(&String::new())
              .to_lowercase(),
          )
      }),
    },
    SortOption {
      name: "Status",
      cmp_fn: Some(|a, b| {
        a.status
          .to_string()
          .to_lowercase()
          .cmp(&b.status.to_string().to_lowercase())
      }),
    },
    SortOption {
      name: "Quality Profile",
      cmp_fn: Some(|a, b| a.quality_profile_id.cmp(&b.quality_profile_id)),
    },
    SortOption {
      name: "Metadata Profile",
      cmp_fn: Some(|a, b| a.metadata_profile_id.cmp(&b.metadata_profile_id)),
    },
    SortOption {
      name: "Albums",
      cmp_fn: Some(|a, b| {
        a.statistics
          .as_ref()
          .map_or(0, |stats| stats.album_count)
          .cmp(&b.statistics.as_ref().map_or(0, |stats| stats.album_count))
      }),
    },
    SortOption {
      name: "Tracks",
      cmp_fn: Some(|a, b| {
        a.statistics
          .as_ref()
          .map_or(0, |stats| stats.track_count)
          .cmp(&b.statistics.as_ref().map_or(0, |stats| stats.track_count))
      }),
    },
    SortOption {
      name: "Size",
      cmp_fn: Some(|a, b| {
        a.statistics
          .as_ref()
          .map_or(0, |stats| stats.size_on_disk)
          .cmp(&b.statistics.as_ref().map_or(0, |stats| stats.size_on_disk))
      }),
    },
    SortOption {
      name: "Monitored",
      cmp_fn: Some(|a, b| a.monitored.cmp(&b.monitored)),
    },
    SortOption {
      name: "Tags",
      cmp_fn: Some(|a, b| {
        let a_str = a
          .tags
          .iter()
          .map(|tag| tag.as_i64().unwrap().to_string())
          .collect::<Vec<String>>()
          .join(",");
        let b_str = b
          .tags
          .iter()
          .map(|tag| tag.as_i64().unwrap().to_string())
          .collect::<Vec<String>>()
          .join(",");

        a_str.cmp(&b_str)
      }),
    },
  ]
}
