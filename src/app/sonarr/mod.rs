use crate::{
  models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock,
  network::sonarr_network::SonarrEvent,
};

use super::App;

pub mod sonarr_context_clues;

#[cfg(test)]
#[path = "sonarr_tests.rs"]
mod sonarr_tests;

impl App<'_> {
  pub(super) async fn dispatch_by_sonarr_block(&mut self, active_sonarr_block: &ActiveSonarrBlock) {
    match active_sonarr_block {
      ActiveSonarrBlock::Series => {
        self
          .dispatch_network_event(SonarrEvent::GetQualityProfiles.into())
          .await;
        self
          .dispatch_network_event(SonarrEvent::GetLanguageProfiles.into())
          .await;
        self
          .dispatch_network_event(SonarrEvent::GetTags.into())
          .await;
        self
          .dispatch_network_event(SonarrEvent::ListSeries.into())
          .await;
      }
      ActiveSonarrBlock::SeriesDetails => {
        self
          .dispatch_network_event(SonarrEvent::ListSeries.into())
          .await;
        self.is_loading = true;
        self.populate_seasons_table().await;
        self.is_loading = false;
      }
      ActiveSonarrBlock::SeriesHistory => {
        self
          .dispatch_network_event(
            SonarrEvent::GetSeriesHistory(self.extract_series_id().await).into(),
          )
          .await;
      }
      ActiveSonarrBlock::SeasonDetails => {
        self
          .dispatch_network_event(SonarrEvent::GetEpisodes(self.extract_series_id().await).into())
          .await;
        self
          .dispatch_network_event(
            SonarrEvent::GetEpisodeFiles(self.extract_series_id().await).into(),
          )
          .await;
        self
          .dispatch_network_event(SonarrEvent::GetDownloads.into())
          .await;
      }
      ActiveSonarrBlock::SeasonHistory => {
        if !self.data.sonarr_data.seasons.is_empty() {
          self
            .dispatch_network_event(
              SonarrEvent::GetSeasonHistory(self.extract_series_id_season_number_tuple().await)
                .into(),
            )
            .await;
        }
      }
      ActiveSonarrBlock::ManualSeasonSearch => {
        match self.data.sonarr_data.season_details_modal.as_ref() {
          Some(season_details_modal) if season_details_modal.season_releases.is_empty() => {
            self
              .dispatch_network_event(
                SonarrEvent::GetSeasonReleases(self.extract_series_id_season_number_tuple().await)
                  .into(),
              )
              .await;
          }
          _ => (),
        }
      }
      ActiveSonarrBlock::EpisodeDetails | ActiveSonarrBlock::EpisodeFile => {
        self
          .dispatch_network_event(
            SonarrEvent::GetEpisodeDetails(self.extract_episode_id().await).into(),
          )
          .await;
      }
      ActiveSonarrBlock::EpisodeHistory => {
        self
          .dispatch_network_event(
            SonarrEvent::GetEpisodeHistory(self.extract_episode_id().await).into(),
          )
          .await;
      }
      ActiveSonarrBlock::ManualEpisodeSearch => {
        if let Some(season_details_modal) = self.data.sonarr_data.season_details_modal.as_ref() {
          if let Some(episode_details_modal) = season_details_modal.episode_details_modal.as_ref() {
            if episode_details_modal.episode_releases.is_empty() {
              self
                .dispatch_network_event(
                  SonarrEvent::GetEpisodeReleases(self.extract_episode_id().await).into(),
                )
                .await;
            }
          }
        }
      }
      ActiveSonarrBlock::Downloads => {
        self
          .dispatch_network_event(SonarrEvent::GetDownloads.into())
          .await;
      }
      ActiveSonarrBlock::Blocklist => {
        self
          .dispatch_network_event(SonarrEvent::ListSeries.into())
          .await;
        self
          .dispatch_network_event(SonarrEvent::GetBlocklist.into())
          .await;
      }
      ActiveSonarrBlock::History => {
        self
          .dispatch_network_event(SonarrEvent::GetHistory(500).into())
          .await;
      }
      ActiveSonarrBlock::RootFolders => {
        self
          .dispatch_network_event(SonarrEvent::GetRootFolders.into())
          .await;
      }
      ActiveSonarrBlock::Indexers => {
        self
          .dispatch_network_event(SonarrEvent::GetTags.into())
          .await;
        self
          .dispatch_network_event(SonarrEvent::GetIndexers.into())
          .await;
      }
      ActiveSonarrBlock::AllIndexerSettingsPrompt => {
        self
          .dispatch_network_event(SonarrEvent::GetAllIndexerSettings.into())
          .await;
      }
      ActiveSonarrBlock::TestIndexer => {
        self
          .dispatch_network_event(
            SonarrEvent::TestIndexer(self.extract_sonarr_indexer_id().await).into(),
          )
          .await;
      }
      ActiveSonarrBlock::TestAllIndexers => {
        self
          .dispatch_network_event(SonarrEvent::TestAllIndexers.into())
          .await;
      }
      ActiveSonarrBlock::System => {
        self
          .dispatch_network_event(SonarrEvent::GetTasks.into())
          .await;
        self
          .dispatch_network_event(SonarrEvent::GetQueuedEvents.into())
          .await;
        self
          .dispatch_network_event(SonarrEvent::GetLogs(500).into())
          .await;
      }
      ActiveSonarrBlock::AddSeriesSearchResults => {
        self
          .dispatch_network_event(
            SonarrEvent::SearchNewSeries(self.extract_add_new_series_search_query().await).into(),
          )
          .await;
      }
      ActiveSonarrBlock::SystemUpdates => {
        self
          .dispatch_network_event(SonarrEvent::GetUpdates.into())
          .await;
      }
      _ => (),
    }

    self.check_for_sonarr_prompt_action().await;
    self.reset_tick_count();
  }

  async fn check_for_sonarr_prompt_action(&mut self) {
    if self.data.sonarr_data.prompt_confirm {
      self.data.sonarr_data.prompt_confirm = false;
      if let Some(sonarr_event) = self.data.sonarr_data.prompt_confirm_action.take() {
        self.dispatch_network_event(sonarr_event.into()).await;
        self.should_refresh = true;
      }
    }
  }

  pub(super) async fn sonarr_on_tick(&mut self, active_sonarr_block: ActiveSonarrBlock) {
    if self.is_first_render {
      self.refresh_sonarr_metadata().await;
      self.dispatch_by_sonarr_block(&active_sonarr_block).await;
      self.is_first_render = false;
      return;
    }

    if self.should_refresh {
      self.dispatch_by_sonarr_block(&active_sonarr_block).await;
      self.refresh_sonarr_metadata().await;
    }

    if self.is_routing {
      if !self.should_refresh {
        self.cancellation_token.cancel();
      } else {
        self.dispatch_by_sonarr_block(&active_sonarr_block).await;
      }
    }

    if self.tick_count % self.tick_until_poll == 0 {
      self.refresh_sonarr_metadata().await;
    }
  }

  async fn refresh_sonarr_metadata(&mut self) {
    self
      .dispatch_network_event(SonarrEvent::GetQualityProfiles.into())
      .await;
    self
      .dispatch_network_event(SonarrEvent::GetLanguageProfiles.into())
      .await;
    self
      .dispatch_network_event(SonarrEvent::GetTags.into())
      .await;
    self
      .dispatch_network_event(SonarrEvent::GetRootFolders.into())
      .await;
    self
      .dispatch_network_event(SonarrEvent::GetDownloads.into())
      .await;
    self
      .dispatch_network_event(SonarrEvent::GetDiskSpace.into())
      .await;
    self
      .dispatch_network_event(SonarrEvent::GetStatus.into())
      .await;
  }

  async fn populate_seasons_table(&mut self) {
    let seasons = self
      .data
      .sonarr_data
      .series
      .current_selection()
      .clone()
      .seasons
      .unwrap_or_default()
      .into_iter()
      .map(|mut season| {
        season.title = Some(format!("Season {}", season.season_number));
        season
      })
      .collect();
    self.data.sonarr_data.seasons.set_items(seasons);
  }

  async fn extract_episode_id(&self) -> i64 {
    self
      .data
      .sonarr_data
      .season_details_modal
      .as_ref()
      .expect("Season details have not been loaded")
      .episodes
      .current_selection()
      .id
  }

  async fn extract_series_id(&self) -> i64 {
    self.data.sonarr_data.series.current_selection().id
  }

  async fn extract_series_id_season_number_tuple(&self) -> (i64, i64) {
    let series_id = self.data.sonarr_data.series.current_selection().id;
    let season_number = self
      .data
      .sonarr_data
      .seasons
      .current_selection()
      .season_number;
    (series_id, season_number)
  }

  async fn extract_add_new_series_search_query(&self) -> String {
    self
      .data
      .sonarr_data
      .add_series_search
      .as_ref()
      .expect("Add series search is empty")
      .text
      .clone()
  }

  async fn extract_sonarr_indexer_id(&self) -> i64 {
    self.data.sonarr_data.indexers.current_selection().id
  }
}
