use crate::{
  models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock,
  network::sonarr_network::SonarrEvent,
};

use super::App;

pub mod sonarr_context_clues;

#[cfg(test)]
#[path = "sonarr_tests.rs"]
mod sonarr_tests;

impl<'a> App<'a> {
  pub(super) async fn dispatch_by_sonarr_block(&mut self, active_sonarr_block: &ActiveSonarrBlock) {
    match active_sonarr_block {
      ActiveSonarrBlock::Series => {
        self
          .dispatch_network_event(SonarrEvent::ListSeries.into())
          .await;
      }
      ActiveSonarrBlock::SeriesDetails => {
        self.is_loading = true;
        self.populate_seasons_table().await;
        self.is_loading = false;
      }
      ActiveSonarrBlock::SeriesHistory => {
        self
          .dispatch_network_event(SonarrEvent::GetSeriesHistory(None).into())
          .await;
      }
      ActiveSonarrBlock::SeasonDetails => {
        self
          .dispatch_network_event(SonarrEvent::GetEpisodes(None).into())
          .await;
      }
      ActiveSonarrBlock::SeasonHistory => {
        self
          .dispatch_network_event(SonarrEvent::GetSeasonHistory(None).into())
          .await;
      }
      ActiveSonarrBlock::ManualSeasonSearch => {
        self
          .dispatch_network_event(SonarrEvent::GetSeasonReleases(None).into())
          .await;
      }
      ActiveSonarrBlock::EpisodeDetails | ActiveSonarrBlock::EpisodeFile => {
        self
          .dispatch_network_event(SonarrEvent::GetEpisodeDetails(None).into())
          .await;
      }
      ActiveSonarrBlock::EpisodeHistory => {
        self
          .dispatch_network_event(SonarrEvent::GetEpisodeHistory(None).into())
          .await;
      }
      ActiveSonarrBlock::ManualEpisodeSearch => {
        self
          .dispatch_network_event(SonarrEvent::GetEpisodeReleases(None).into())
          .await;
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
          .dispatch_network_event(SonarrEvent::GetHistory(None).into())
          .await;
      }
      ActiveSonarrBlock::RootFolders => {
        self
          .dispatch_network_event(SonarrEvent::GetRootFolders.into())
          .await;
      }
      ActiveSonarrBlock::Indexers => {
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
          .dispatch_network_event(SonarrEvent::TestIndexer(None).into())
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
          .dispatch_network_event(SonarrEvent::GetLogs(None).into())
          .await;
      }
      ActiveSonarrBlock::AddSeriesSearchResults => {
        self
          .dispatch_network_event(SonarrEvent::SearchNewSeries(None).into())
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
      if let Some(sonarr_event) = &self.data.sonarr_data.prompt_confirm_action {
        self
          .dispatch_network_event(sonarr_event.clone().into())
          .await;
        self.should_refresh = true;
        self.data.sonarr_data.prompt_confirm_action = None;
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
}
