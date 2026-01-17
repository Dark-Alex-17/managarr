use super::App;
use crate::{
  models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock,
  network::lidarr_network::LidarrEvent,
};

pub mod lidarr_context_clues;

#[cfg(test)]
#[path = "lidarr_tests.rs"]
mod lidarr_tests;

impl App<'_> {
  pub(super) async fn dispatch_by_lidarr_block(&mut self, active_lidarr_block: &ActiveLidarrBlock) {
    match active_lidarr_block {
      ActiveLidarrBlock::Artists => {
        self
          .dispatch_network_event(LidarrEvent::GetQualityProfiles.into())
          .await;
        self
          .dispatch_network_event(LidarrEvent::GetMetadataProfiles.into())
          .await;
        self
          .dispatch_network_event(LidarrEvent::GetTags.into())
          .await;
        self
          .dispatch_network_event(LidarrEvent::ListArtists.into())
          .await;
      }
      ActiveLidarrBlock::Downloads => {
        self
          .dispatch_network_event(LidarrEvent::GetDownloads(500).into())
          .await;
      }
      ActiveLidarrBlock::ArtistDetails => {
        self
          .dispatch_network_event(LidarrEvent::GetAlbums(self.extract_artist_id().await).into())
          .await;
      }
      ActiveLidarrBlock::ArtistHistory => {
        self
          .dispatch_network_event(
            LidarrEvent::GetArtistHistory(self.extract_artist_id().await).into(),
          )
          .await;
      }
      ActiveLidarrBlock::ManualArtistSearch => {
        if self.data.lidarr_data.discography_releases.is_empty() {
          self
            .dispatch_network_event(
              LidarrEvent::GetDiscographyReleases(self.extract_artist_id().await).into(),
            )
            .await;
        }
      }
      ActiveLidarrBlock::AlbumDetails => {
        let artist_id = self.extract_artist_id().await;
        let album_id = self.extract_album_id().await;
        self
          .dispatch_network_event(LidarrEvent::GetTracks(artist_id, album_id).into())
          .await;
        self
          .dispatch_network_event(LidarrEvent::GetTrackFiles(album_id).into())
          .await;
        self
          .dispatch_network_event(LidarrEvent::GetDownloads(500).into())
          .await;
      }
      ActiveLidarrBlock::AlbumHistory => {
        if !self.data.lidarr_data.albums.is_empty() {
          self
            .dispatch_network_event(
              LidarrEvent::GetAlbumHistory(
                self.extract_artist_id().await,
                self.extract_album_id().await,
              )
              .into(),
            )
            .await;
        }
      }
      ActiveLidarrBlock::ManualAlbumSearch => {
        match self.data.lidarr_data.album_details_modal.as_ref() {
          Some(album_details_modal) if album_details_modal.album_releases.is_empty() => {
            self
              .dispatch_network_event(
                LidarrEvent::GetAlbumReleases(
                  self.extract_artist_id().await,
                  self.extract_album_id().await,
                )
                .into(),
              )
              .await;
          }
          _ => (),
        }
      }
      ActiveLidarrBlock::AddArtistSearchResults => {
        self
          .dispatch_network_event(
            LidarrEvent::SearchNewArtist(self.extract_add_new_artist_search_query().await).into(),
          )
          .await;
      }
      ActiveLidarrBlock::History => {
        self
          .dispatch_network_event(LidarrEvent::GetHistory(500).into())
          .await
      }
      ActiveLidarrBlock::RootFolders => {
        self
          .dispatch_network_event(LidarrEvent::GetRootFolders.into())
          .await;
      }
      ActiveLidarrBlock::Indexers => {
        self
          .dispatch_network_event(LidarrEvent::GetTags.into())
          .await;
        self
          .dispatch_network_event(LidarrEvent::GetIndexers.into())
          .await;
      }
      ActiveLidarrBlock::AllIndexerSettingsPrompt => {
        self
          .dispatch_network_event(LidarrEvent::GetAllIndexerSettings.into())
          .await;
      }
      ActiveLidarrBlock::TestIndexer => {
        self
          .dispatch_network_event(
            LidarrEvent::TestIndexer(self.extract_lidarr_indexer_id().await).into(),
          )
          .await;
      }
      ActiveLidarrBlock::TestAllIndexers => {
        self
          .dispatch_network_event(LidarrEvent::TestAllIndexers.into())
          .await;
      }
      ActiveLidarrBlock::System => {
        self
          .dispatch_network_event(LidarrEvent::GetTasks.into())
          .await;
        self
          .dispatch_network_event(LidarrEvent::GetQueuedEvents.into())
          .await;
        self
          .dispatch_network_event(LidarrEvent::GetLogs(500).into())
          .await;
      }
      ActiveLidarrBlock::SystemUpdates => {
        self
          .dispatch_network_event(LidarrEvent::GetUpdates.into())
          .await;
      }
      _ => (),
    }

    self.check_for_lidarr_prompt_action().await;
    self.reset_tick_count();
  }

  async fn extract_add_new_artist_search_query(&self) -> String {
    self
      .data
      .lidarr_data
      .add_artist_search
      .as_ref()
      .expect("Add artist search is empty")
      .text
      .clone()
  }

  async fn extract_artist_id(&self) -> i64 {
    self.data.lidarr_data.artists.current_selection().id
  }

  async fn extract_album_id(&self) -> i64 {
    self.data.lidarr_data.albums.current_selection().id
  }

  async fn extract_lidarr_indexer_id(&self) -> i64 {
    self.data.lidarr_data.indexers.current_selection().id
  }

  async fn check_for_lidarr_prompt_action(&mut self) {
    if self.data.lidarr_data.prompt_confirm {
      self.data.lidarr_data.prompt_confirm = false;
      if let Some(lidarr_event) = self.data.lidarr_data.prompt_confirm_action.take() {
        self.dispatch_network_event(lidarr_event.into()).await;
        self.should_refresh = true;
      }
    }
  }

  pub(super) async fn lidarr_on_tick(&mut self, active_lidarr_block: ActiveLidarrBlock) {
    if self.is_first_render {
      self.refresh_lidarr_metadata().await;
      self.dispatch_by_lidarr_block(&active_lidarr_block).await;
      self.is_first_render = false;
      return;
    }

    if self.should_refresh {
      self.dispatch_by_lidarr_block(&active_lidarr_block).await;
      self.refresh_lidarr_metadata().await;
    }

    if self.is_routing {
      if !self.should_refresh {
        self.cancellation_token.cancel();
      } else {
        self.dispatch_by_lidarr_block(&active_lidarr_block).await;
      }
    }

    if self.tick_count.is_multiple_of(self.tick_until_poll) {
      self.refresh_lidarr_metadata().await;
    }
  }

  async fn refresh_lidarr_metadata(&mut self) {
    self
      .dispatch_network_event(LidarrEvent::GetQualityProfiles.into())
      .await;
    self
      .dispatch_network_event(LidarrEvent::GetMetadataProfiles.into())
      .await;
    self
      .dispatch_network_event(LidarrEvent::GetTags.into())
      .await;
    self
      .dispatch_network_event(LidarrEvent::GetRootFolders.into())
      .await;
    self
      .dispatch_network_event(LidarrEvent::GetDownloads(500).into())
      .await;
    self
      .dispatch_network_event(LidarrEvent::GetDiskSpace.into())
      .await;
    self
      .dispatch_network_event(LidarrEvent::GetStatus.into())
      .await;
  }
}
