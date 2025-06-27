use crate::{
  models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock,
  network::lidarr_network::LidarrEvent,
};

use super::App;

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
      ActiveLidarrBlock::ArtistDetails => {
        self
          .dispatch_network_event(LidarrEvent::ListArtists.into())
          .await;
        self.is_loading = true;
        self.populate_albums_table().await;
        self.is_loading = false;
      }
      ActiveLidarrBlock::ArtistHistory => {
        self
          .dispatch_network_event(
            LidarrEvent::GetArtistHistory(self.extract_artist_id().await).into(),
          )
          .await;
      }
      ActiveLidarrBlock::AlbumDetails => {
        self
          .dispatch_network_event(LidarrEvent::GetTracks(self.extract_artist_id().await).into())
          .await;
        self
          .dispatch_network_event(
            LidarrEvent::GetTrackFiles(self.extract_artist_id().await).into(),
          )
          .await;
        self
          .dispatch_network_event(LidarrEvent::GetDownloads.into())
          .await;
      }
      ActiveLidarrBlock::AlbumHistory => {
        if !self.data.lidarr_data.albums.is_empty() {
          self
            .dispatch_network_event(
              LidarrEvent::GetAlbumHistory(self.extract_artist_id_album_id_tuple().await)
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
                LidarrEvent::GetAlbumReleases(self.extract_artist_id_album_id_tuple().await)
                  .into(),
              )
              .await;
          }
          _ => (),
        }
      }
      ActiveLidarrBlock::TrackDetails | ActiveLidarrBlock::TrackFile => {
        self
          .dispatch_network_event(
            LidarrEvent::GetTrackDetails(self.extract_track_id().await).into(),
          )
          .await;
      }
      ActiveLidarrBlock::TrackHistory => {
        self
          .dispatch_network_event(
            LidarrEvent::GetTrackHistory(self.extract_track_id().await).into(),
          )
          .await;
      }
      ActiveLidarrBlock::ManualTrackSearch => {
        if let Some(album_details_modal) = self.data.lidarr_data.album_details_modal.as_ref() {
          if let Some(track_details_modal) = album_details_modal.track_details_modal.as_ref() {
            if track_details_modal.track_releases.is_empty() {
              self
                .dispatch_network_event(
                  LidarrEvent::GetTrackReleases(self.extract_track_id().await).into(),
                )
                .await;
            }
          }
        }
      }
      ActiveLidarrBlock::Downloads => {
        self
          .dispatch_network_event(LidarrEvent::GetDownloads.into())
          .await;
      }
      ActiveLidarrBlock::Blocklist => {
        self
          .dispatch_network_event(LidarrEvent::ListArtists.into())
          .await;
        self
          .dispatch_network_event(LidarrEvent::GetBlocklist.into())
          .await;
      }
      ActiveLidarrBlock::History => {
        self
          .dispatch_network_event(LidarrEvent::GetHistory(500).into())
          .await;
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
      ActiveLidarrBlock::AddArtistSearchResults => {
        self
          .dispatch_network_event(
            LidarrEvent::SearchNewArtist(self.extract_add_new_artist_search_query().await).into(),
          )
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

    if self.tick_count % self.tick_until_poll == 0 {
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
      .dispatch_network_event(LidarrEvent::GetDownloads.into())
      .await;
    self
      .dispatch_network_event(LidarrEvent::GetDiskSpace.into())
      .await;
    self
      .dispatch_network_event(LidarrEvent::GetStatus.into())
      .await;
  }

  async fn populate_albums_table(&mut self) {
    let albums = self
      .data
      .lidarr_data
      .artists
      .current_selection()
      .clone()
      .albums
      .unwrap_or_default()
      .into_iter()
      .map(|mut album| {
        album.title = album.title;
        album
      })
      .collect();
    self.data.lidarr_data.albums.set_items(albums);
  }

  async fn extract_track_id(&self) -> i64 {
    self
      .data
      .lidarr_data
      .album_details_modal
      .as_ref()
      .expect("Album details have not been loaded")
      .tracks
      .current_selection()
      .id
  }

  async fn extract_artist_id(&self) -> i64 {
    self.data.lidarr_data.artists.current_selection().id
  }

  async fn extract_artist_id_album_id_tuple(&self) -> (i64, i64) {
    let artist_id = self.data.lidarr_data.artists.current_selection().id;
    let album_id = self
      .data
      .lidarr_data
      .albums
      .current_selection()
      .id;
    (artist_id, album_id)
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

  async fn extract_lidarr_indexer_id(&self) -> i64 {
    self.data.lidarr_data.indexers.current_selection().id
  }
}
