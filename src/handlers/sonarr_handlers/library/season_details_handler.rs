use crate::app::App;
use crate::event::Key;
use crate::handlers::sonarr_handlers::history::history_sorting_options;
use crate::handlers::table_handler::TableHandlingConfig;
use crate::handlers::{handle_prompt_toggle, KeyEventHandler};
use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, SEASON_DETAILS_BLOCKS};
use crate::models::servarr_models::Language;
use crate::models::sonarr_models::{
  Episode, SonarrHistoryItem, SonarrRelease, SonarrReleaseDownloadBody,
};
use crate::models::stateful_table::SortOption;
use crate::network::sonarr_network::SonarrEvent;
use crate::{handle_table_events, matches_key};
use serde_json::Number;

#[cfg(test)]
#[path = "season_details_handler_tests.rs"]
mod season_details_handler_tests;

pub(super) struct SeasonDetailsHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_sonarr_block: ActiveSonarrBlock,
  _context: Option<ActiveSonarrBlock>,
}

impl SeasonDetailsHandler<'_, '_> {
  handle_table_events!(
    self,
    episodes,
    self
      .app
      .data
      .sonarr_data
      .season_details_modal
      .as_mut()
      .expect("Season details modal is undefined")
      .episodes,
    Episode
  );
  handle_table_events!(
    self,
    season_history,
    self
      .app
      .data
      .sonarr_data
      .season_details_modal
      .as_mut()
      .expect("Season details modal is undefined")
      .season_history,
    SonarrHistoryItem
  );
  handle_table_events!(
    self,
    season_releases,
    self
      .app
      .data
      .sonarr_data
      .season_details_modal
      .as_mut()
      .expect("Season details modal is undefined")
      .season_releases,
    SonarrRelease
  );

  fn extract_episode_file_id(&self) -> i64 {
    self
      .app
      .data
      .sonarr_data
      .season_details_modal
      .as_ref()
      .expect("Season details have not been loaded")
      .episodes
      .current_selection()
      .episode_file_id
  }

  fn extract_episode_id(&self) -> i64 {
    self
      .app
      .data
      .sonarr_data
      .season_details_modal
      .as_ref()
      .expect("Season details have not been loaded")
      .episodes
      .current_selection()
      .id
  }

  fn extract_series_id_season_number_tuple(&self) -> (i64, i64) {
    let series_id = self.app.data.sonarr_data.series.current_selection().id;
    let season_number = self
      .app
      .data
      .sonarr_data
      .seasons
      .current_selection()
      .season_number;
    (series_id, season_number)
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveSonarrBlock> for SeasonDetailsHandler<'a, 'b> {
  fn handle(&mut self) {
    let episodes_table_handling_config =
      TableHandlingConfig::new(ActiveSonarrBlock::SeasonDetails.into())
        .searching_block(ActiveSonarrBlock::SearchEpisodes.into())
        .search_error_block(ActiveSonarrBlock::SearchEpisodesError.into())
        .search_field_fn(|episode: &Episode| &episode.title);
    let season_history_table_handling_config =
      TableHandlingConfig::new(ActiveSonarrBlock::SeasonHistory.into())
        .sorting_block(ActiveSonarrBlock::SeasonHistorySortPrompt.into())
        .sort_options(history_sorting_options())
        .sort_by_fn(|a: &SonarrHistoryItem, b: &SonarrHistoryItem| a.id.cmp(&b.id))
        .searching_block(ActiveSonarrBlock::SearchSeasonHistory.into())
        .search_error_block(ActiveSonarrBlock::SearchSeasonHistoryError.into())
        .search_field_fn(|history_item: &SonarrHistoryItem| &history_item.source_title.text)
        .filtering_block(ActiveSonarrBlock::FilterSeasonHistory.into())
        .filter_error_block(ActiveSonarrBlock::FilterSeasonHistoryError.into())
        .filter_field_fn(|history_item: &SonarrHistoryItem| &history_item.source_title.text);
    let season_releases_table_handling_config =
      TableHandlingConfig::new(ActiveSonarrBlock::ManualSeasonSearch.into())
        .sorting_block(ActiveSonarrBlock::ManualSeasonSearchSortPrompt.into())
        .sort_options(releases_sorting_options());

    if !self.handle_episodes_table_events(episodes_table_handling_config)
      && !self.handle_season_history_table_events(season_history_table_handling_config)
      && !self.handle_season_releases_table_events(season_releases_table_handling_config)
    {
      self.handle_key_event();
    }
  }

  fn accepts(active_block: ActiveSonarrBlock) -> bool {
    SEASON_DETAILS_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_block: ActiveSonarrBlock,
    context: Option<ActiveSonarrBlock>,
  ) -> SeasonDetailsHandler<'a, 'b> {
    SeasonDetailsHandler {
      key,
      app,
      active_sonarr_block: active_block,
      _context: context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    !self.app.is_loading
      && if let Some(season_details_modal) = &self.app.data.sonarr_data.season_details_modal {
        match self.active_sonarr_block {
          ActiveSonarrBlock::SeasonDetails => !season_details_modal.episodes.is_empty(),
          ActiveSonarrBlock::SeasonHistory => !season_details_modal.season_history.is_empty(),
          ActiveSonarrBlock::ManualSeasonSearch => !season_details_modal.season_releases.is_empty(),
          _ => true,
        }
      } else {
        false
      }
  }

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {
    if self.active_sonarr_block == ActiveSonarrBlock::SeasonDetails {
      self
        .app
        .push_navigation_stack(ActiveSonarrBlock::DeleteEpisodeFilePrompt.into());
    }
  }

  fn handle_left_right_action(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::SeasonDetails
      | ActiveSonarrBlock::SeasonHistory
      | ActiveSonarrBlock::ManualSeasonSearch => match self.key {
        _ if matches_key!(left, self.key) => {
          self
            .app
            .data
            .sonarr_data
            .season_details_modal
            .as_mut()
            .unwrap()
            .season_details_tabs
            .previous();
          self.app.pop_and_push_navigation_stack(
            self
              .app
              .data
              .sonarr_data
              .season_details_modal
              .as_ref()
              .unwrap()
              .season_details_tabs
              .get_active_route(),
          );
        }
        _ if matches_key!(right, self.key) => {
          self
            .app
            .data
            .sonarr_data
            .season_details_modal
            .as_mut()
            .unwrap()
            .season_details_tabs
            .next();
          self.app.pop_and_push_navigation_stack(
            self
              .app
              .data
              .sonarr_data
              .season_details_modal
              .as_ref()
              .unwrap()
              .season_details_tabs
              .get_active_route(),
          );
        }
        _ => (),
      },
      ActiveSonarrBlock::AutomaticallySearchSeasonPrompt
      | ActiveSonarrBlock::ManualSeasonSearchConfirmPrompt
      | ActiveSonarrBlock::DeleteEpisodeFilePrompt => {
        handle_prompt_toggle(self.app, self.key);
      }
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::SeasonDetails
        if self.app.data.sonarr_data.season_details_modal.is_some()
          && !self
            .app
            .data
            .sonarr_data
            .season_details_modal
            .as_ref()
            .unwrap()
            .episodes
            .is_empty() =>
      {
        self
          .app
          .push_navigation_stack(ActiveSonarrBlock::EpisodeDetails.into())
      }
      ActiveSonarrBlock::SeasonHistory => self
        .app
        .push_navigation_stack(ActiveSonarrBlock::SeasonHistoryDetails.into()),
      ActiveSonarrBlock::DeleteEpisodeFilePrompt => {
        if self.app.data.sonarr_data.prompt_confirm {
          self.app.data.sonarr_data.prompt_confirm_action = Some(SonarrEvent::DeleteEpisodeFile(
            self.extract_episode_file_id(),
          ));
        }

        self.app.pop_navigation_stack();
      }
      ActiveSonarrBlock::AutomaticallySearchSeasonPrompt => {
        if self.app.data.sonarr_data.prompt_confirm {
          self.app.data.sonarr_data.prompt_confirm_action = Some(
            SonarrEvent::TriggerAutomaticSeasonSearch(self.extract_series_id_season_number_tuple()),
          );
        }

        self.app.pop_navigation_stack();
      }
      ActiveSonarrBlock::ManualSeasonSearch => {
        self
          .app
          .push_navigation_stack(ActiveSonarrBlock::ManualSeasonSearchConfirmPrompt.into());
      }
      ActiveSonarrBlock::ManualSeasonSearchConfirmPrompt => {
        if self.app.data.sonarr_data.prompt_confirm {
          let SonarrRelease {
            guid, indexer_id, ..
          } = self
            .app
            .data
            .sonarr_data
            .season_details_modal
            .as_ref()
            .unwrap()
            .season_releases
            .current_selection();
          let series_id = self.app.data.sonarr_data.series.current_selection().id;
          let season_number = self
            .app
            .data
            .sonarr_data
            .seasons
            .current_selection()
            .season_number;
          let params = SonarrReleaseDownloadBody {
            guid: guid.clone(),
            indexer_id: *indexer_id,
            series_id: Some(series_id),
            season_number: Some(season_number),
            ..SonarrReleaseDownloadBody::default()
          };
          self.app.data.sonarr_data.prompt_confirm_action =
            Some(SonarrEvent::DownloadRelease(params));
        }

        self.app.pop_navigation_stack();
      }
      _ => (),
    }
  }

  fn handle_esc(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::SeasonDetails | ActiveSonarrBlock::ManualSeasonSearch => {
        self.app.pop_navigation_stack();
        self.app.data.sonarr_data.season_details_modal = None;
      }
      ActiveSonarrBlock::SeasonHistoryDetails => {
        self.app.pop_navigation_stack();
      }
      ActiveSonarrBlock::SeasonHistory => {
        if self
          .app
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .season_history
          .filtered_items
          .is_some()
        {
          self
            .app
            .data
            .sonarr_data
            .season_details_modal
            .as_mut()
            .unwrap()
            .season_history
            .filtered_items = None;
        } else {
          self.app.pop_navigation_stack();
          self.app.data.sonarr_data.season_details_modal = None;
        }
      }
      ActiveSonarrBlock::AutomaticallySearchSeasonPrompt
      | ActiveSonarrBlock::ManualSeasonSearchConfirmPrompt
      | ActiveSonarrBlock::DeleteEpisodeFilePrompt => {
        self.app.pop_navigation_stack();
        self.app.data.sonarr_data.prompt_confirm = false;
      }
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_sonarr_block {
      ActiveSonarrBlock::SeasonDetails if matches_key!(toggle_monitoring, self.key) => {
        self.app.data.sonarr_data.prompt_confirm = true;
        self.app.data.sonarr_data.prompt_confirm_action = Some(
          SonarrEvent::ToggleEpisodeMonitoring(self.extract_episode_id()),
        );

        self
          .app
          .pop_and_push_navigation_stack(self.active_sonarr_block.into());
      }
      ActiveSonarrBlock::SeasonDetails
      | ActiveSonarrBlock::SeasonHistory
      | ActiveSonarrBlock::ManualSeasonSearch => match self.key {
        _ if matches_key!(refresh, self.key) => {
          self
            .app
            .pop_and_push_navigation_stack(self.active_sonarr_block.into());
        }
        _ if matches_key!(auto_search, self.key) => {
          self
            .app
            .push_navigation_stack(ActiveSonarrBlock::AutomaticallySearchSeasonPrompt.into());
        }
        _ => (),
      },
      ActiveSonarrBlock::AutomaticallySearchSeasonPrompt if matches_key!(confirm, key) => {
        self.app.data.sonarr_data.prompt_confirm = true;
        self.app.data.sonarr_data.prompt_confirm_action = Some(
          SonarrEvent::TriggerAutomaticSeasonSearch(self.extract_series_id_season_number_tuple()),
        );

        self.app.pop_navigation_stack();
      }
      ActiveSonarrBlock::DeleteEpisodeFilePrompt if matches_key!(confirm, key) => {
        self.app.data.sonarr_data.prompt_confirm = true;
        self.app.data.sonarr_data.prompt_confirm_action = Some(SonarrEvent::DeleteEpisodeFile(
          self.extract_episode_file_id(),
        ));

        self.app.pop_navigation_stack();
      }
      ActiveSonarrBlock::ManualSeasonSearchConfirmPrompt if matches_key!(confirm, key) => {
        self.app.data.sonarr_data.prompt_confirm = true;
        let SonarrRelease {
          guid, indexer_id, ..
        } = self
          .app
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .season_releases
          .current_selection();
        let series_id = self.app.data.sonarr_data.series.current_selection().id;
        let season_number = self
          .app
          .data
          .sonarr_data
          .seasons
          .current_selection()
          .season_number;
        let params = SonarrReleaseDownloadBody {
          guid: guid.clone(),
          indexer_id: *indexer_id,
          series_id: Some(series_id),
          season_number: Some(season_number),
          ..SonarrReleaseDownloadBody::default()
        };
        self.app.data.sonarr_data.prompt_confirm_action =
          Some(SonarrEvent::DownloadRelease(params));

        self.app.pop_navigation_stack();
      }
      _ => (),
    }
  }
}

pub(in crate::handlers::sonarr_handlers::library) fn releases_sorting_options(
) -> Vec<SortOption<SonarrRelease>> {
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
