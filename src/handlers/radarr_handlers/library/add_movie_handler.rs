use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::handlers::table_handler::TableHandlingConfig;
use crate::handlers::{handle_prompt_toggle, KeyEventHandler};
use crate::models::radarr_models::AddMovieSearchResult;
use crate::models::servarr_data::radarr::radarr_data::{
  ActiveRadarrBlock, ADD_MOVIE_BLOCKS, ADD_MOVIE_SELECTION_BLOCKS,
};
use crate::models::{BlockSelectionState, Scrollable};
use crate::network::radarr_network::RadarrEvent;
use crate::{handle_table_events, handle_text_box_keys, handle_text_box_left_right_keys, App, Key};

#[cfg(test)]
#[path = "add_movie_handler_tests.rs"]
mod add_movie_handler_tests;

pub(super) struct AddMovieHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_radarr_block: ActiveRadarrBlock,
  context: Option<ActiveRadarrBlock>,
}

impl<'a, 'b> AddMovieHandler<'a, 'b> {
  handle_table_events!(
    self,
    add_movie_search_results,
    self
      .app
      .data
      .radarr_data
      .add_searched_movies
      .as_mut()
      .unwrap(),
    AddMovieSearchResult
  );
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for AddMovieHandler<'a, 'b> {
  fn handle(&mut self) {
    let add_movie_table_handling_config =
      TableHandlingConfig::new(ActiveRadarrBlock::AddMovieSearchResults.into());

    if !self.handle_add_movie_search_results_table_events(add_movie_table_handling_config) {
      self.handle_key_event();
    }
  }

  fn accepts(active_block: ActiveRadarrBlock) -> bool {
    ADD_MOVIE_BLOCKS.contains(&active_block)
  }

  fn with(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveRadarrBlock,
    context: Option<ActiveRadarrBlock>,
  ) -> AddMovieHandler<'a, 'b> {
    AddMovieHandler {
      key,
      app,
      active_radarr_block: active_block,
      context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    !self.app.is_loading
  }

  fn handle_scroll_up(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::AddMovieSelectMonitor => self
        .app
        .data
        .radarr_data
        .add_movie_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .scroll_up(),
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability => self
        .app
        .data
        .radarr_data
        .add_movie_modal
        .as_mut()
        .unwrap()
        .minimum_availability_list
        .scroll_up(),
      ActiveRadarrBlock::AddMovieSelectQualityProfile => self
        .app
        .data
        .radarr_data
        .add_movie_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_up(),
      ActiveRadarrBlock::AddMovieSelectRootFolder => self
        .app
        .data
        .radarr_data
        .add_movie_modal
        .as_mut()
        .unwrap()
        .root_folder_list
        .scroll_up(),
      ActiveRadarrBlock::AddMoviePrompt => self.app.data.radarr_data.selected_block.up(),
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::AddMovieSelectMonitor => self
        .app
        .data
        .radarr_data
        .add_movie_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .scroll_down(),
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability => self
        .app
        .data
        .radarr_data
        .add_movie_modal
        .as_mut()
        .unwrap()
        .minimum_availability_list
        .scroll_down(),
      ActiveRadarrBlock::AddMovieSelectQualityProfile => self
        .app
        .data
        .radarr_data
        .add_movie_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_down(),
      ActiveRadarrBlock::AddMovieSelectRootFolder => self
        .app
        .data
        .radarr_data
        .add_movie_modal
        .as_mut()
        .unwrap()
        .root_folder_list
        .scroll_down(),
      ActiveRadarrBlock::AddMoviePrompt => self.app.data.radarr_data.selected_block.down(),
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::AddMovieSelectMonitor => self
        .app
        .data
        .radarr_data
        .add_movie_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .scroll_to_top(),
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability => self
        .app
        .data
        .radarr_data
        .add_movie_modal
        .as_mut()
        .unwrap()
        .minimum_availability_list
        .scroll_to_top(),
      ActiveRadarrBlock::AddMovieSelectQualityProfile => self
        .app
        .data
        .radarr_data
        .add_movie_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_to_top(),
      ActiveRadarrBlock::AddMovieSelectRootFolder => self
        .app
        .data
        .radarr_data
        .add_movie_modal
        .as_mut()
        .unwrap()
        .root_folder_list
        .scroll_to_top(),
      ActiveRadarrBlock::AddMovieSearchInput => self
        .app
        .data
        .radarr_data
        .add_movie_search
        .as_mut()
        .unwrap()
        .scroll_home(),
      ActiveRadarrBlock::AddMovieTagsInput => self
        .app
        .data
        .radarr_data
        .add_movie_modal
        .as_mut()
        .unwrap()
        .tags
        .scroll_home(),
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::AddMovieSelectMonitor => self
        .app
        .data
        .radarr_data
        .add_movie_modal
        .as_mut()
        .unwrap()
        .monitor_list
        .scroll_to_bottom(),
      ActiveRadarrBlock::AddMovieSelectMinimumAvailability => self
        .app
        .data
        .radarr_data
        .add_movie_modal
        .as_mut()
        .unwrap()
        .minimum_availability_list
        .scroll_to_bottom(),
      ActiveRadarrBlock::AddMovieSelectQualityProfile => self
        .app
        .data
        .radarr_data
        .add_movie_modal
        .as_mut()
        .unwrap()
        .quality_profile_list
        .scroll_to_bottom(),
      ActiveRadarrBlock::AddMovieSelectRootFolder => self
        .app
        .data
        .radarr_data
        .add_movie_modal
        .as_mut()
        .unwrap()
        .root_folder_list
        .scroll_to_bottom(),
      ActiveRadarrBlock::AddMovieSearchInput => self
        .app
        .data
        .radarr_data
        .add_movie_search
        .as_mut()
        .unwrap()
        .reset_offset(),
      ActiveRadarrBlock::AddMovieTagsInput => self
        .app
        .data
        .radarr_data
        .add_movie_modal
        .as_mut()
        .unwrap()
        .tags
        .reset_offset(),
      _ => (),
    }
  }

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::AddMoviePrompt => handle_prompt_toggle(self.app, self.key),
      ActiveRadarrBlock::AddMovieSearchInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self.app.data.radarr_data.add_movie_search.as_mut().unwrap()
        )
      }
      ActiveRadarrBlock::AddMovieTagsInput => {
        handle_text_box_left_right_keys!(
          self,
          self.key,
          self
            .app
            .data
            .radarr_data
            .add_movie_modal
            .as_mut()
            .unwrap()
            .tags
        )
      }
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_radarr_block {
      _ if self.active_radarr_block == ActiveRadarrBlock::AddMovieSearchInput
        && !self
          .app
          .data
          .radarr_data
          .add_movie_search
          .as_mut()
          .unwrap()
          .text
          .is_empty() =>
      {
        self
          .app
          .push_navigation_stack(ActiveRadarrBlock::AddMovieSearchResults.into());
        self.app.should_ignore_quit_key = false;
      }
      _ if self.active_radarr_block == ActiveRadarrBlock::AddMovieSearchResults
        && self.app.data.radarr_data.add_searched_movies.is_some() =>
      {
        let tmdb_id = self
          .app
          .data
          .radarr_data
          .add_searched_movies
          .as_ref()
          .unwrap()
          .current_selection()
          .tmdb_id;

        if self
          .app
          .data
          .radarr_data
          .movies
          .items
          .iter()
          .any(|movie| movie.tmdb_id == tmdb_id)
        {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::AddMovieAlreadyInLibrary.into());
        } else {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::AddMoviePrompt.into());
          self.app.data.radarr_data.add_movie_modal = Some((&self.app.data.radarr_data).into());
          self.app.data.radarr_data.selected_block =
            BlockSelectionState::new(ADD_MOVIE_SELECTION_BLOCKS);
        }
      }
      ActiveRadarrBlock::AddMoviePrompt => {
        match self.app.data.radarr_data.selected_block.get_active_block() {
          ActiveRadarrBlock::AddMovieConfirmPrompt => {
            if self.app.data.radarr_data.prompt_confirm {
              self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::AddMovie(None));
            }

            self.app.pop_navigation_stack();
          }
          ActiveRadarrBlock::AddMovieSelectMonitor
          | ActiveRadarrBlock::AddMovieSelectMinimumAvailability
          | ActiveRadarrBlock::AddMovieSelectQualityProfile
          | ActiveRadarrBlock::AddMovieSelectRootFolder => self.app.push_navigation_stack(
            (
              self.app.data.radarr_data.selected_block.get_active_block(),
              self.context,
            )
              .into(),
          ),
          ActiveRadarrBlock::AddMovieTagsInput => {
            self.app.push_navigation_stack(
              (
                self.app.data.radarr_data.selected_block.get_active_block(),
                self.context,
              )
                .into(),
            );
            self.app.should_ignore_quit_key = true;
          }
          _ => (),
        }
      }
      ActiveRadarrBlock::AddMovieSelectMonitor
      | ActiveRadarrBlock::AddMovieSelectMinimumAvailability
      | ActiveRadarrBlock::AddMovieSelectQualityProfile
      | ActiveRadarrBlock::AddMovieSelectRootFolder => self.app.pop_navigation_stack(),
      ActiveRadarrBlock::AddMovieTagsInput => {
        self.app.pop_navigation_stack();
        self.app.should_ignore_quit_key = false;
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::AddMovieSearchInput => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.add_movie_search = None;
        self.app.should_ignore_quit_key = false;
      }
      ActiveRadarrBlock::AddMovieSearchResults | ActiveRadarrBlock::AddMovieEmptySearchResults => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.add_searched_movies = None;
        self.app.should_ignore_quit_key = true;
      }
      ActiveRadarrBlock::AddMoviePrompt => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.add_movie_modal = None;
        self.app.data.radarr_data.prompt_confirm = false;
      }
      ActiveRadarrBlock::AddMovieSelectMonitor
      | ActiveRadarrBlock::AddMovieSelectMinimumAvailability
      | ActiveRadarrBlock::AddMovieSelectQualityProfile
      | ActiveRadarrBlock::AddMovieAlreadyInLibrary
      | ActiveRadarrBlock::AddMovieSelectRootFolder => self.app.pop_navigation_stack(),
      ActiveRadarrBlock::AddMovieTagsInput => {
        self.app.pop_navigation_stack();
        self.app.should_ignore_quit_key = false;
      }
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_radarr_block {
      ActiveRadarrBlock::AddMovieSearchInput => {
        handle_text_box_keys!(
          self,
          key,
          self.app.data.radarr_data.add_movie_search.as_mut().unwrap()
        )
      }
      ActiveRadarrBlock::AddMovieTagsInput => {
        handle_text_box_keys!(
          self,
          key,
          self
            .app
            .data
            .radarr_data
            .add_movie_modal
            .as_mut()
            .unwrap()
            .tags
        )
      }
      ActiveRadarrBlock::AddMoviePrompt => {
        if self.app.data.radarr_data.selected_block.get_active_block()
          == ActiveRadarrBlock::AddMovieConfirmPrompt
          && key == DEFAULT_KEYBINDINGS.confirm.key
        {
          self.app.data.radarr_data.prompt_confirm = true;
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::AddMovie(None));
          self.app.pop_navigation_stack();
        }
      }
      _ => (),
    }
  }
}
