use serde_json::Number;

use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::App;
use crate::event::Key;
use crate::handlers::{handle_prompt_toggle, KeyEventHandler};
use crate::models::radarr_models::RadarrRelease;
use crate::models::servarr_data::radarr::radarr_data::{
  ActiveRadarrBlock, EDIT_MOVIE_SELECTION_BLOCKS, MOVIE_DETAILS_BLOCKS,
};
use crate::models::servarr_models::Language;
use crate::models::stateful_table::SortOption;
use crate::models::{BlockSelectionState, Scrollable};
use crate::network::radarr_network::RadarrEvent;

#[cfg(test)]
#[path = "movie_details_handler_tests.rs"]
mod movie_details_handler_tests;

pub(super) struct MovieDetailsHandler<'a, 'b> {
  key: &'a Key,
  app: &'a mut App<'b>,
  active_radarr_block: &'a ActiveRadarrBlock,
  _context: &'a Option<ActiveRadarrBlock>,
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveRadarrBlock> for MovieDetailsHandler<'a, 'b> {
  fn accepts(active_block: &'a ActiveRadarrBlock) -> bool {
    MOVIE_DETAILS_BLOCKS.contains(active_block)
  }

  fn with(
    key: &'a Key,
    app: &'a mut App<'b>,
    active_block: &'a ActiveRadarrBlock,
    _context: &'a Option<ActiveRadarrBlock>,
  ) -> MovieDetailsHandler<'a, 'b> {
    MovieDetailsHandler {
      key,
      app,
      active_radarr_block: active_block,
      _context,
    }
  }

  fn get_key(&self) -> &Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    if let Some(movie_details_modal) = &self.app.data.radarr_data.movie_details_modal {
      match self.active_radarr_block {
        ActiveRadarrBlock::MovieDetails => {
          !self.app.is_loading && !movie_details_modal.movie_details.is_empty()
        }
        ActiveRadarrBlock::MovieHistory => {
          !self.app.is_loading && !movie_details_modal.movie_history.is_empty()
        }
        ActiveRadarrBlock::Cast => {
          !self.app.is_loading && !movie_details_modal.movie_cast.is_empty()
        }
        ActiveRadarrBlock::Crew => {
          !self.app.is_loading && !movie_details_modal.movie_crew.is_empty()
        }
        ActiveRadarrBlock::ManualSearch => {
          !self.app.is_loading && !movie_details_modal.movie_releases.is_empty()
        }
        _ => !self.app.is_loading,
      }
    } else {
      false
    }
  }

  fn handle_scroll_up(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::MovieDetails => self
        .app
        .data
        .radarr_data
        .movie_details_modal
        .as_mut()
        .unwrap()
        .movie_details
        .scroll_up(),
      ActiveRadarrBlock::MovieHistory => self
        .app
        .data
        .radarr_data
        .movie_details_modal
        .as_mut()
        .unwrap()
        .movie_history
        .scroll_up(),
      ActiveRadarrBlock::Cast => self
        .app
        .data
        .radarr_data
        .movie_details_modal
        .as_mut()
        .unwrap()
        .movie_cast
        .scroll_up(),
      ActiveRadarrBlock::Crew => self
        .app
        .data
        .radarr_data
        .movie_details_modal
        .as_mut()
        .unwrap()
        .movie_crew
        .scroll_up(),
      ActiveRadarrBlock::ManualSearch => self
        .app
        .data
        .radarr_data
        .movie_details_modal
        .as_mut()
        .unwrap()
        .movie_releases
        .scroll_up(),
      ActiveRadarrBlock::ManualSearchSortPrompt => self
        .app
        .data
        .radarr_data
        .movie_details_modal
        .as_mut()
        .unwrap()
        .movie_releases
        .sort
        .as_mut()
        .unwrap()
        .scroll_up(),
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::MovieDetails => self
        .app
        .data
        .radarr_data
        .movie_details_modal
        .as_mut()
        .unwrap()
        .movie_details
        .scroll_down(),
      ActiveRadarrBlock::MovieHistory => self
        .app
        .data
        .radarr_data
        .movie_details_modal
        .as_mut()
        .unwrap()
        .movie_history
        .scroll_down(),
      ActiveRadarrBlock::Cast => self
        .app
        .data
        .radarr_data
        .movie_details_modal
        .as_mut()
        .unwrap()
        .movie_cast
        .scroll_down(),
      ActiveRadarrBlock::Crew => self
        .app
        .data
        .radarr_data
        .movie_details_modal
        .as_mut()
        .unwrap()
        .movie_crew
        .scroll_down(),
      ActiveRadarrBlock::ManualSearch => self
        .app
        .data
        .radarr_data
        .movie_details_modal
        .as_mut()
        .unwrap()
        .movie_releases
        .scroll_down(),
      ActiveRadarrBlock::ManualSearchSortPrompt => self
        .app
        .data
        .radarr_data
        .movie_details_modal
        .as_mut()
        .unwrap()
        .movie_releases
        .sort
        .as_mut()
        .unwrap()
        .scroll_down(),
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::MovieDetails => self
        .app
        .data
        .radarr_data
        .movie_details_modal
        .as_mut()
        .unwrap()
        .movie_details
        .scroll_to_top(),
      ActiveRadarrBlock::MovieHistory => self
        .app
        .data
        .radarr_data
        .movie_details_modal
        .as_mut()
        .unwrap()
        .movie_history
        .scroll_to_top(),
      ActiveRadarrBlock::Cast => self
        .app
        .data
        .radarr_data
        .movie_details_modal
        .as_mut()
        .unwrap()
        .movie_cast
        .scroll_to_top(),
      ActiveRadarrBlock::Crew => self
        .app
        .data
        .radarr_data
        .movie_details_modal
        .as_mut()
        .unwrap()
        .movie_crew
        .scroll_to_top(),
      ActiveRadarrBlock::ManualSearch => self
        .app
        .data
        .radarr_data
        .movie_details_modal
        .as_mut()
        .unwrap()
        .movie_releases
        .scroll_to_top(),
      ActiveRadarrBlock::ManualSearchSortPrompt => self
        .app
        .data
        .radarr_data
        .movie_details_modal
        .as_mut()
        .unwrap()
        .movie_releases
        .sort
        .as_mut()
        .unwrap()
        .scroll_to_top(),
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::MovieDetails => self
        .app
        .data
        .radarr_data
        .movie_details_modal
        .as_mut()
        .unwrap()
        .movie_details
        .scroll_to_bottom(),
      ActiveRadarrBlock::MovieHistory => self
        .app
        .data
        .radarr_data
        .movie_details_modal
        .as_mut()
        .unwrap()
        .movie_history
        .scroll_to_bottom(),
      ActiveRadarrBlock::Cast => self
        .app
        .data
        .radarr_data
        .movie_details_modal
        .as_mut()
        .unwrap()
        .movie_cast
        .scroll_to_bottom(),
      ActiveRadarrBlock::Crew => self
        .app
        .data
        .radarr_data
        .movie_details_modal
        .as_mut()
        .unwrap()
        .movie_crew
        .scroll_to_bottom(),
      ActiveRadarrBlock::ManualSearch => self
        .app
        .data
        .radarr_data
        .movie_details_modal
        .as_mut()
        .unwrap()
        .movie_releases
        .scroll_to_bottom(),
      ActiveRadarrBlock::ManualSearchSortPrompt => self
        .app
        .data
        .radarr_data
        .movie_details_modal
        .as_mut()
        .unwrap()
        .movie_releases
        .sort
        .as_mut()
        .unwrap()
        .scroll_to_bottom(),
      _ => (),
    }
  }

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::MovieDetails
      | ActiveRadarrBlock::MovieHistory
      | ActiveRadarrBlock::FileInfo
      | ActiveRadarrBlock::Cast
      | ActiveRadarrBlock::Crew
      | ActiveRadarrBlock::ManualSearch => match self.key {
        _ if *self.key == DEFAULT_KEYBINDINGS.left.key => {
          self.app.data.radarr_data.movie_info_tabs.previous();
          self.app.pop_and_push_navigation_stack(
            *self.app.data.radarr_data.movie_info_tabs.get_active_route(),
          );
        }
        _ if *self.key == DEFAULT_KEYBINDINGS.right.key => {
          self.app.data.radarr_data.movie_info_tabs.next();
          self.app.pop_and_push_navigation_stack(
            *self.app.data.radarr_data.movie_info_tabs.get_active_route(),
          );
        }
        _ => (),
      },
      ActiveRadarrBlock::AutomaticallySearchMoviePrompt
      | ActiveRadarrBlock::UpdateAndScanPrompt
      | ActiveRadarrBlock::ManualSearchConfirmPrompt => handle_prompt_toggle(self.app, self.key),
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::AutomaticallySearchMoviePrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action =
            Some(RadarrEvent::TriggerAutomaticSearch(None));
        }

        self.app.pop_navigation_stack();
      }
      ActiveRadarrBlock::UpdateAndScanPrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::UpdateAndScan(None));
        }

        self.app.pop_navigation_stack();
      }
      ActiveRadarrBlock::ManualSearch => {
        self
          .app
          .push_navigation_stack(ActiveRadarrBlock::ManualSearchConfirmPrompt.into());
      }
      ActiveRadarrBlock::ManualSearchConfirmPrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action =
            Some(RadarrEvent::DownloadRelease(None));
        }

        self.app.pop_navigation_stack();
      }
      ActiveRadarrBlock::ManualSearchSortPrompt => {
        self
          .app
          .data
          .radarr_data
          .movie_details_modal
          .as_mut()
          .unwrap()
          .movie_releases
          .apply_sorting();
        self.app.pop_navigation_stack();
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::MovieDetails
      | ActiveRadarrBlock::MovieHistory
      | ActiveRadarrBlock::FileInfo
      | ActiveRadarrBlock::Cast
      | ActiveRadarrBlock::Crew
      | ActiveRadarrBlock::ManualSearch => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.reset_movie_info_tabs();
      }
      ActiveRadarrBlock::AutomaticallySearchMoviePrompt
      | ActiveRadarrBlock::UpdateAndScanPrompt
      | ActiveRadarrBlock::ManualSearchConfirmPrompt
      | ActiveRadarrBlock::ManualSearchSortPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.radarr_data.prompt_confirm = false;
      }
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match *self.active_radarr_block {
      ActiveRadarrBlock::MovieDetails
      | ActiveRadarrBlock::MovieHistory
      | ActiveRadarrBlock::FileInfo
      | ActiveRadarrBlock::Cast
      | ActiveRadarrBlock::Crew
      | ActiveRadarrBlock::ManualSearch => match self.key {
        _ if *key == DEFAULT_KEYBINDINGS.search.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::AutomaticallySearchMoviePrompt.into());
        }
        _ if *key == DEFAULT_KEYBINDINGS.edit.key => {
          self.app.push_navigation_stack(
            (
              ActiveRadarrBlock::EditMoviePrompt,
              Some(*self.active_radarr_block),
            )
              .into(),
          );
          self.app.data.radarr_data.edit_movie_modal = Some((&self.app.data.radarr_data).into());
          self.app.data.radarr_data.selected_block =
            BlockSelectionState::new(&EDIT_MOVIE_SELECTION_BLOCKS);
        }
        _ if *key == DEFAULT_KEYBINDINGS.update.key => {
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::UpdateAndScanPrompt.into());
        }
        _ if *key == DEFAULT_KEYBINDINGS.refresh.key => {
          self
            .app
            .pop_and_push_navigation_stack((*self.active_radarr_block).into());
        }
        _ if *key == DEFAULT_KEYBINDINGS.sort.key => {
          self
            .app
            .data
            .radarr_data
            .movie_details_modal
            .as_mut()
            .unwrap()
            .movie_releases
            .sorting(releases_sorting_options());
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::ManualSearchSortPrompt.into());
        }
        _ => (),
      },
      ActiveRadarrBlock::AutomaticallySearchMoviePrompt => {
        if *key == DEFAULT_KEYBINDINGS.confirm.key {
          self.app.data.radarr_data.prompt_confirm = true;
          self.app.data.radarr_data.prompt_confirm_action =
            Some(RadarrEvent::TriggerAutomaticSearch(None));

          self.app.pop_navigation_stack();
        }
      }
      ActiveRadarrBlock::UpdateAndScanPrompt => {
        if *key == DEFAULT_KEYBINDINGS.confirm.key {
          self.app.data.radarr_data.prompt_confirm = true;
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::UpdateAndScan(None));

          self.app.pop_navigation_stack();
        }
      }
      ActiveRadarrBlock::ManualSearchConfirmPrompt => {
        if *key == DEFAULT_KEYBINDINGS.confirm.key {
          self.app.data.radarr_data.prompt_confirm = true;
          self.app.data.radarr_data.prompt_confirm_action =
            Some(RadarrEvent::DownloadRelease(None));

          self.app.pop_navigation_stack();
        }
      }
      _ => (),
    }
  }
}

fn releases_sorting_options() -> Vec<SortOption<RadarrRelease>> {
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
      name: "Language",
      cmp_fn: Some(|a, b| {
        let default_language_vec = vec![Language {
          id: 1,
          name: "_".to_owned(),
        }];
        let language_a = &a.languages.as_ref().unwrap_or(&default_language_vec)[0];
        let language_b = &b.languages.as_ref().unwrap_or(&default_language_vec)[0];

        language_a.cmp(language_b)
      }),
    },
    SortOption {
      name: "Quality",
      cmp_fn: Some(|a, b| a.quality.cmp(&b.quality)),
    },
  ]
}
