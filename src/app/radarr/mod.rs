use crate::app::App;
use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
use crate::network::radarr_network::RadarrEvent;

pub mod radarr_context_clues;

#[cfg(test)]
#[path = "radarr_tests.rs"]
mod radarr_tests;

impl<'a> App<'a> {
  pub(super) async fn dispatch_by_radarr_block(&mut self, active_radarr_block: &ActiveRadarrBlock) {
    match active_radarr_block {
      ActiveRadarrBlock::Blocklist => {
        self
          .dispatch_network_event(RadarrEvent::GetBlocklist.into())
          .await;
      }
      ActiveRadarrBlock::Collections => {
        self
          .dispatch_network_event(RadarrEvent::GetCollections.into())
          .await;
      }
      ActiveRadarrBlock::CollectionDetails => {
        self.is_loading = true;
        self.populate_movie_collection_table().await;
        self.is_loading = false;
      }
      ActiveRadarrBlock::Downloads => {
        self
          .dispatch_network_event(RadarrEvent::GetDownloads.into())
          .await;
      }
      ActiveRadarrBlock::RootFolders => {
        self
          .dispatch_network_event(RadarrEvent::GetRootFolders.into())
          .await;
      }
      ActiveRadarrBlock::Movies => {
        self
          .dispatch_network_event(RadarrEvent::GetMovies.into())
          .await;
        self
          .dispatch_network_event(RadarrEvent::GetDownloads.into())
          .await;
      }
      ActiveRadarrBlock::Indexers => {
        self
          .dispatch_network_event(RadarrEvent::GetIndexers.into())
          .await;
      }
      ActiveRadarrBlock::AllIndexerSettingsPrompt => {
        self
          .dispatch_network_event(RadarrEvent::GetAllIndexerSettings.into())
          .await;
      }
      ActiveRadarrBlock::TestIndexer => {
        self
          .dispatch_network_event(RadarrEvent::TestIndexer(None).into())
          .await;
      }
      ActiveRadarrBlock::TestAllIndexers => {
        self
          .dispatch_network_event(RadarrEvent::TestAllIndexers.into())
          .await;
      }
      ActiveRadarrBlock::System => {
        self
          .dispatch_network_event(RadarrEvent::GetTasks.into())
          .await;
        self
          .dispatch_network_event(RadarrEvent::GetQueuedEvents.into())
          .await;
        self
          .dispatch_network_event(RadarrEvent::GetLogs(None).into())
          .await;
      }
      ActiveRadarrBlock::SystemUpdates => {
        self
          .dispatch_network_event(RadarrEvent::GetUpdates.into())
          .await;
      }
      ActiveRadarrBlock::AddMovieSearchResults => {
        self
          .dispatch_network_event(RadarrEvent::SearchNewMovie(None).into())
          .await;
      }
      ActiveRadarrBlock::MovieDetails | ActiveRadarrBlock::FileInfo => {
        self
          .dispatch_network_event(RadarrEvent::GetMovieDetails(None).into())
          .await;
      }
      ActiveRadarrBlock::MovieHistory => {
        self
          .dispatch_network_event(RadarrEvent::GetMovieHistory(None).into())
          .await;
      }
      ActiveRadarrBlock::Cast | ActiveRadarrBlock::Crew => {
        match self.data.radarr_data.movie_details_modal.as_ref() {
          Some(movie_details_modal)
            if movie_details_modal.movie_cast.items.is_empty()
              || movie_details_modal.movie_crew.items.is_empty() =>
          {
            self
              .dispatch_network_event(RadarrEvent::GetMovieCredits(None).into())
              .await;
          }
          _ => (),
        }
      }
      ActiveRadarrBlock::ManualSearch => match self.data.radarr_data.movie_details_modal.as_ref() {
        Some(movie_details_modal) if movie_details_modal.movie_releases.items.is_empty() => {
          self
            .dispatch_network_event(RadarrEvent::GetReleases(None).into())
            .await;
        }
        _ => (),
      },
      _ => (),
    }

    self.check_for_prompt_action().await;
    self.reset_tick_count();
  }

  async fn check_for_prompt_action(&mut self) {
    if self.data.radarr_data.prompt_confirm {
      self.data.radarr_data.prompt_confirm = false;
      if let Some(radarr_event) = &self.data.radarr_data.prompt_confirm_action {
        self
          .dispatch_network_event(radarr_event.clone().into())
          .await;
        self.should_refresh = true;
        self.data.radarr_data.prompt_confirm_action = None;
      }
    }
  }

  pub(super) async fn radarr_on_tick(
    &mut self,
    active_radarr_block: ActiveRadarrBlock,
    is_first_render: bool,
  ) {
    if is_first_render {
      self
        .dispatch_network_event(RadarrEvent::GetQualityProfiles.into())
        .await;
      self
        .dispatch_network_event(RadarrEvent::GetTags.into())
        .await;
      self
        .dispatch_network_event(RadarrEvent::GetRootFolders.into())
        .await;
      self
        .dispatch_network_event(RadarrEvent::GetOverview.into())
        .await;
      self
        .dispatch_network_event(RadarrEvent::GetStatus.into())
        .await;
      self.dispatch_by_radarr_block(&active_radarr_block).await;
    }

    if self.should_refresh {
      self.dispatch_by_radarr_block(&active_radarr_block).await;
    }

    if self.is_routing {
      if !self.should_refresh {
        self.cancellation_token.cancel();
      }

      self.dispatch_by_radarr_block(&active_radarr_block).await;
      self.refresh_metadata().await;
    }

    if self.tick_count % self.tick_until_poll == 0 {
      self.refresh_metadata().await;
    }
  }

  async fn refresh_metadata(&mut self) {
    self
      .dispatch_network_event(RadarrEvent::GetQualityProfiles.into())
      .await;
    self
      .dispatch_network_event(RadarrEvent::GetTags.into())
      .await;
    self
      .dispatch_network_event(RadarrEvent::GetRootFolders.into())
      .await;
    self
      .dispatch_network_event(RadarrEvent::GetDownloads.into())
      .await;
  }

  async fn populate_movie_collection_table(&mut self) {
    let collection_movies = self
      .data
      .radarr_data
      .collections
      .current_selection()
      .clone()
      .movies
      .unwrap_or_default();
    self
      .data
      .radarr_data
      .collection_movies
      .set_items(collection_movies);
  }
}
