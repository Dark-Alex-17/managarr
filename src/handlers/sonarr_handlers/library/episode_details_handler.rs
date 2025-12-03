use crate::app::App;
use crate::event::Key;
use crate::handlers::sonarr_handlers::library::season_details_handler::releases_sorting_options;
use crate::handlers::table_handler::TableHandlingConfig;
use crate::handlers::{KeyEventHandler, handle_prompt_toggle};
use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, EPISODE_DETAILS_BLOCKS};
use crate::models::sonarr_models::{SonarrHistoryItem, SonarrRelease, SonarrReleaseDownloadBody};
use crate::network::sonarr_network::SonarrEvent;
use crate::{handle_table_events, matches_key};

#[cfg(test)]
#[path = "episode_details_handler_tests.rs"]
mod episode_details_handler_tests;

pub(super) struct EpisodeDetailsHandler<'a, 'b> {
  key: Key,
  app: &'a mut App<'b>,
  active_sonarr_block: ActiveSonarrBlock,
  _context: Option<ActiveSonarrBlock>,
}

impl EpisodeDetailsHandler<'_, '_> {
  handle_table_events!(
    self,
    episode_history,
    self
      .app
      .data
      .sonarr_data
      .season_details_modal
      .as_mut()
      .expect("Season details modal is undefined")
      .episode_details_modal
      .as_mut()
      .expect("Episode details modal is undefined")
      .episode_history,
    SonarrHistoryItem
  );
  handle_table_events!(
    self,
    episode_releases,
    self
      .app
      .data
      .sonarr_data
      .season_details_modal
      .as_mut()
      .expect("Season details modal is undefined")
      .episode_details_modal
      .as_mut()
      .expect("Episode details modal is undefined")
      .episode_releases,
    SonarrRelease
  );

  fn extract_episode_id(&self) -> i64 {
    self
      .app
      .data
      .sonarr_data
      .season_details_modal
      .as_ref()
      .expect("Season details modal is undefined")
      .episodes
      .current_selection()
      .id
  }
}

impl<'a, 'b> KeyEventHandler<'a, 'b, ActiveSonarrBlock> for EpisodeDetailsHandler<'a, 'b> {
  fn handle(&mut self) {
    let episode_history_table_handling_config =
      TableHandlingConfig::new(ActiveSonarrBlock::EpisodeHistory.into());
    let episode_releases_table_handling_config =
      TableHandlingConfig::new(ActiveSonarrBlock::ManualEpisodeSearch.into())
        .sorting_block(ActiveSonarrBlock::ManualEpisodeSearchSortPrompt.into())
        .sort_options(releases_sorting_options());

    if !self.handle_episode_history_table_events(episode_history_table_handling_config)
      && !self.handle_episode_releases_table_events(episode_releases_table_handling_config)
    {
      self.handle_key_event();
    }
  }

  fn accepts(active_block: ActiveSonarrBlock) -> bool {
    EPISODE_DETAILS_BLOCKS.contains(&active_block)
  }

  fn ignore_special_keys(&self) -> bool {
    self.app.ignore_special_keys_for_textbox_input
  }

  fn new(
    key: Key,
    app: &'a mut App<'b>,
    active_sonarr_block: ActiveSonarrBlock,
    _context: Option<ActiveSonarrBlock>,
  ) -> Self {
    Self {
      key,
      app,
      active_sonarr_block,
      _context,
    }
  }

  fn get_key(&self) -> Key {
    self.key
  }

  fn is_ready(&self) -> bool {
    if self.app.is_loading {
      return false;
    }

    let Some(season_details_modal) = self.app.data.sonarr_data.season_details_modal.as_ref() else {
      return false;
    };

    let Some(episode_details_modal) = &season_details_modal.episode_details_modal else {
      return false;
    };

    match self.active_sonarr_block {
      ActiveSonarrBlock::EpisodeHistory => !episode_details_modal.episode_history.is_empty(),
      ActiveSonarrBlock::ManualEpisodeSearch => !episode_details_modal.episode_releases.is_empty(),
      _ => true,
    }
  }

  fn handle_scroll_up(&mut self) {}

  fn handle_scroll_down(&mut self) {}

  fn handle_home(&mut self) {}

  fn handle_end(&mut self) {}

  fn handle_delete(&mut self) {}

  fn handle_left_right_action(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::EpisodeDetails
      | ActiveSonarrBlock::EpisodeHistory
      | ActiveSonarrBlock::EpisodeFile
      | ActiveSonarrBlock::ManualEpisodeSearch => match self.key {
        _ if matches_key!(left, self.key) => {
          self
            .app
            .data
            .sonarr_data
            .season_details_modal
            .as_mut()
            .unwrap()
            .episode_details_modal
            .as_mut()
            .unwrap()
            .episode_details_tabs
            .previous();
          self.app.pop_and_push_navigation_stack(
            self
              .app
              .data
              .sonarr_data
              .season_details_modal
              .as_ref()
              .unwrap()
              .episode_details_modal
              .as_ref()
              .unwrap()
              .episode_details_tabs
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
            .episode_details_modal
            .as_mut()
            .unwrap()
            .episode_details_tabs
            .next();
          self.app.pop_and_push_navigation_stack(
            self
              .app
              .data
              .sonarr_data
              .season_details_modal
              .as_ref()
              .unwrap()
              .episode_details_modal
              .as_ref()
              .unwrap()
              .episode_details_tabs
              .get_active_route(),
          );
        }
        _ => (),
      },
      ActiveSonarrBlock::AutomaticallySearchEpisodePrompt
      | ActiveSonarrBlock::ManualEpisodeSearchConfirmPrompt => {
        handle_prompt_toggle(self.app, self.key);
      }
      _ => (),
    }
  }

  fn handle_submit(&mut self) {
    match self.active_sonarr_block {
      ActiveSonarrBlock::EpisodeHistory => {
        self
          .app
          .push_navigation_stack(ActiveSonarrBlock::EpisodeHistoryDetails.into());
      }
      ActiveSonarrBlock::AutomaticallySearchEpisodePrompt => {
        if self.app.data.sonarr_data.prompt_confirm {
          self.app.data.sonarr_data.prompt_confirm_action = Some(
            SonarrEvent::TriggerAutomaticEpisodeSearch(self.extract_episode_id()),
          );
        }

        self.app.pop_navigation_stack();
      }
      ActiveSonarrBlock::ManualEpisodeSearch => {
        self
          .app
          .push_navigation_stack(ActiveSonarrBlock::ManualEpisodeSearchConfirmPrompt.into());
      }
      ActiveSonarrBlock::ManualEpisodeSearchConfirmPrompt => {
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
            .episode_details_modal
            .as_ref()
            .unwrap()
            .episode_releases
            .current_selection();
          let episode_id = self
            .app
            .data
            .sonarr_data
            .season_details_modal
            .as_ref()
            .unwrap()
            .episodes
            .current_selection()
            .id;
          let params = SonarrReleaseDownloadBody {
            guid: guid.clone(),
            indexer_id: *indexer_id,
            episode_id: Some(episode_id),
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
      ActiveSonarrBlock::EpisodeDetails
      | ActiveSonarrBlock::EpisodeFile
      | ActiveSonarrBlock::EpisodeHistory
      | ActiveSonarrBlock::ManualEpisodeSearch => {
        self.app.pop_navigation_stack();
        self
          .app
          .data
          .sonarr_data
          .season_details_modal
          .as_mut()
          .unwrap()
          .episode_details_modal = None;
      }
      ActiveSonarrBlock::EpisodeHistoryDetails => {
        self.app.pop_navigation_stack();
      }
      ActiveSonarrBlock::AutomaticallySearchEpisodePrompt
      | ActiveSonarrBlock::ManualEpisodeSearchConfirmPrompt => {
        self.app.pop_navigation_stack();
        self.app.data.sonarr_data.prompt_confirm = false;
      }
      _ => (),
    }
  }

  fn handle_char_key_event(&mut self) {
    let key = self.key;
    match self.active_sonarr_block {
      ActiveSonarrBlock::EpisodeDetails
      | ActiveSonarrBlock::EpisodeHistory
      | ActiveSonarrBlock::EpisodeFile
      | ActiveSonarrBlock::ManualEpisodeSearch => match self.key {
        _ if matches_key!(refresh, self.key) => {
          self
            .app
            .pop_and_push_navigation_stack(self.active_sonarr_block.into());
        }
        _ if matches_key!(auto_search, self.key) => {
          self
            .app
            .push_navigation_stack(ActiveSonarrBlock::AutomaticallySearchEpisodePrompt.into());
        }
        _ => (),
      },
      ActiveSonarrBlock::AutomaticallySearchEpisodePrompt if matches_key!(confirm, key) => {
        self.app.data.sonarr_data.prompt_confirm = true;
        self.app.data.sonarr_data.prompt_confirm_action = Some(
          SonarrEvent::TriggerAutomaticEpisodeSearch(self.extract_episode_id()),
        );

        self.app.pop_navigation_stack();
      }
      ActiveSonarrBlock::ManualEpisodeSearchConfirmPrompt if matches_key!(confirm, key) => {
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
            .episode_details_modal
            .as_ref()
            .unwrap()
            .episode_releases
            .current_selection();
          let episode_id = self
            .app
            .data
            .sonarr_data
            .season_details_modal
            .as_ref()
            .unwrap()
            .episodes
            .current_selection()
            .id;
          let params = SonarrReleaseDownloadBody {
            guid: guid.clone(),
            indexer_id: *indexer_id,
            episode_id: Some(episode_id),
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
}
