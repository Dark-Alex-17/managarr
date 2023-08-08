use std::cmp::Ordering;

use serde_json::Number;
use strum::IntoEnumIterator;

use crate::app::key_binding::DEFAULT_KEYBINDINGS;
use crate::app::radarr::{ActiveRadarrBlock, EDIT_MOVIE_SELECTION_BLOCKS, MOVIE_DETAILS_BLOCKS};
use crate::app::App;
use crate::event::Key;
use crate::handlers::{handle_prompt_toggle, KeyEventHandler};
use crate::models::radarr_models::{Language, Release, ReleaseField};
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

  fn handle_scroll_up(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::MovieDetails => self.app.data.radarr_data.movie_details.scroll_up(),
      ActiveRadarrBlock::MovieHistory => self.app.data.radarr_data.movie_history.scroll_up(),
      ActiveRadarrBlock::Cast => self.app.data.radarr_data.movie_cast.scroll_up(),
      ActiveRadarrBlock::Crew => self.app.data.radarr_data.movie_crew.scroll_up(),
      ActiveRadarrBlock::ManualSearch => self.app.data.radarr_data.movie_releases.scroll_up(),
      ActiveRadarrBlock::ManualSearchSortPrompt => {
        self.app.data.radarr_data.movie_releases_sort.scroll_up()
      }
      _ => (),
    }
  }

  fn handle_scroll_down(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::MovieDetails => self.app.data.radarr_data.movie_details.scroll_down(),
      ActiveRadarrBlock::MovieHistory => self.app.data.radarr_data.movie_history.scroll_down(),
      ActiveRadarrBlock::Cast => self.app.data.radarr_data.movie_cast.scroll_down(),
      ActiveRadarrBlock::Crew => self.app.data.radarr_data.movie_crew.scroll_down(),
      ActiveRadarrBlock::ManualSearch => self.app.data.radarr_data.movie_releases.scroll_down(),
      ActiveRadarrBlock::ManualSearchSortPrompt => {
        self.app.data.radarr_data.movie_releases_sort.scroll_down()
      }
      _ => (),
    }
  }

  fn handle_home(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::MovieDetails => self.app.data.radarr_data.movie_details.scroll_to_top(),
      ActiveRadarrBlock::MovieHistory => self.app.data.radarr_data.movie_history.scroll_to_top(),
      ActiveRadarrBlock::Cast => self.app.data.radarr_data.movie_cast.scroll_to_top(),
      ActiveRadarrBlock::Crew => self.app.data.radarr_data.movie_crew.scroll_to_top(),
      ActiveRadarrBlock::ManualSearch => self.app.data.radarr_data.movie_releases.scroll_to_top(),
      ActiveRadarrBlock::ManualSearchSortPrompt => self
        .app
        .data
        .radarr_data
        .movie_releases_sort
        .scroll_to_top(),
      _ => (),
    }
  }

  fn handle_end(&mut self) {
    match self.active_radarr_block {
      ActiveRadarrBlock::MovieDetails => self.app.data.radarr_data.movie_details.scroll_to_bottom(),
      ActiveRadarrBlock::MovieHistory => self.app.data.radarr_data.movie_history.scroll_to_bottom(),
      ActiveRadarrBlock::Cast => self.app.data.radarr_data.movie_cast.scroll_to_bottom(),
      ActiveRadarrBlock::Crew => self.app.data.radarr_data.movie_crew.scroll_to_bottom(),
      ActiveRadarrBlock::ManualSearch => {
        self.app.data.radarr_data.movie_releases.scroll_to_bottom()
      }
      ActiveRadarrBlock::ManualSearchSortPrompt => self
        .app
        .data
        .radarr_data
        .movie_releases_sort
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
            Some(RadarrEvent::TriggerAutomaticSearch);
        }

        self.app.pop_navigation_stack();
      }
      ActiveRadarrBlock::UpdateAndScanPrompt => {
        if self.app.data.radarr_data.prompt_confirm {
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::UpdateAndScan);
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
          self.app.data.radarr_data.prompt_confirm_action = Some(RadarrEvent::DownloadRelease);
        }

        self.app.pop_navigation_stack();
      }
      ActiveRadarrBlock::ManualSearchSortPrompt => {
        let movie_releases = self.app.data.radarr_data.movie_releases.items.clone();
        let field = self
          .app
          .data
          .radarr_data
          .movie_releases_sort
          .current_selection();
        let sort_ascending = !self.app.data.radarr_data.sort_ascending.unwrap();
        self.app.data.radarr_data.sort_ascending = Some(sort_ascending);

        self
          .app
          .data
          .radarr_data
          .movie_releases
          .set_items(sort_releases_by_selected_field(
            movie_releases,
            *field,
            sort_ascending,
          ));
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
          self.app.data.radarr_data.populate_edit_movie_fields();
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
            .movie_releases_sort
            .set_items(Vec::from_iter(ReleaseField::iter()));
          let sort_ascending = self.app.data.radarr_data.sort_ascending;
          self.app.data.radarr_data.sort_ascending =
            Some(sort_ascending.is_some() && sort_ascending.unwrap());
          self
            .app
            .push_navigation_stack(ActiveRadarrBlock::ManualSearchSortPrompt.into());
        }
        _ => (),
      },
      _ => (),
    }
  }
}

fn sort_releases_by_selected_field(
  mut releases: Vec<Release>,
  field: ReleaseField,
  sort_ascending: bool,
) -> Vec<Release> {
  let cmp_fn: fn(&Release, &Release) -> Ordering = match field {
    ReleaseField::Source => |release_a, release_b| release_a.protocol.cmp(&release_b.protocol),
    ReleaseField::Age => |release_a, release_b| release_a.age.as_u64().cmp(&release_b.age.as_u64()),
    ReleaseField::Rejected => |release_a, release_b| release_a.rejected.cmp(&release_b.rejected),
    ReleaseField::Title => |release_a, release_b| release_a.title.text.cmp(&release_b.title.text),
    ReleaseField::Indexer => |release_a, release_b| release_a.indexer.cmp(&release_b.indexer),
    ReleaseField::Size => |release_a, release_b| {
      release_a
        .size
        .as_u64()
        .as_ref()
        .unwrap()
        .cmp(release_b.size.as_u64().as_ref().unwrap())
    },
    ReleaseField::Peers => |release_a, release_b| {
      let default_number = Number::from(i64::max_value());
      let seeder_a = release_a
        .seeders
        .as_ref()
        .unwrap_or(&default_number)
        .as_u64()
        .unwrap();
      let seeder_b = release_b
        .seeders
        .as_ref()
        .unwrap_or(&default_number)
        .as_u64()
        .unwrap();

      seeder_a.cmp(&seeder_b)
    },
    ReleaseField::Language => |release_a, release_b| {
      let default_language_vec = vec![Language {
        name: "_".to_owned(),
      }];
      let language_a = &release_a
        .languages
        .as_ref()
        .unwrap_or(&default_language_vec)[0];
      let language_b = &release_b
        .languages
        .as_ref()
        .unwrap_or(&default_language_vec)[0];

      language_a.cmp(language_b)
    },
    ReleaseField::Quality => |release_a, release_b| release_a.quality.cmp(&release_b.quality),
  };

  if !sort_ascending {
    releases.sort_by(|release_a, release_b| cmp_fn(release_a, release_b).reverse());
  } else {
    releases.sort_by(cmp_fn);
  }

  releases
}
