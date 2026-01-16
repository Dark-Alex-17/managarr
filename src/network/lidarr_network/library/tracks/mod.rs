use crate::models::lidarr_models::{Track, TrackFile};
use crate::models::servarr_data::lidarr::modals::AlbumDetailsModal;
use crate::network::lidarr_network::LidarrEvent;
use crate::network::{Network, RequestMethod};
use anyhow::Result;
use log::info;

#[cfg(test)]
#[path = "lidarr_tracks_network_tests.rs"]
mod lidarr_tracks_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::lidarr_network) async fn delete_lidarr_track_file(
    &mut self,
    track_file_id: i64,
  ) -> Result<()> {
    let event = LidarrEvent::DeleteTrackFile(track_file_id);
    info!("Deleting Lidarr track file for track file with id: {track_file_id}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Delete,
        None::<()>,
        Some(format!("/{track_file_id}")),
        None,
      )
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::lidarr_network) async fn get_tracks(
    &mut self,
    artist_id: i64,
    album_id: i64,
  ) -> Result<Vec<Track>> {
    let event = LidarrEvent::GetTracks(artist_id, album_id);
    info!("Fetching tracks for Lidarr artist with ID: {artist_id} and album with ID: {album_id}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("artistId={artist_id}&albumId={album_id}")),
      )
      .await;

    self
      .handle_request::<(), Vec<Track>>(request_props, |mut track_vec, mut app| {
        track_vec.sort_by(|a, b| a.id.cmp(&b.id));
        if app.data.lidarr_data.album_details_modal.is_none() {
          app.data.lidarr_data.album_details_modal = Some(AlbumDetailsModal::default());
        }

        app
          .data
          .lidarr_data
          .album_details_modal
          .as_mut()
          .unwrap()
          .tracks
          .set_items(track_vec);
      })
      .await
  }

  pub(in crate::network::lidarr_network) async fn get_track_files(
    &mut self,
    album_id: i64,
  ) -> Result<Vec<TrackFile>> {
    let event = LidarrEvent::GetTrackFiles(album_id);
    info!("Fetching tracks files for Lidarr album with ID: {album_id}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("albumId={album_id}")),
      )
      .await;

    self
      .handle_request::<(), Vec<TrackFile>>(request_props, |track_file_vec, mut app| {
        if app.data.lidarr_data.album_details_modal.is_none() {
          app.data.lidarr_data.album_details_modal = Some(AlbumDetailsModal::default());
        }

        app
          .data
          .lidarr_data
          .album_details_modal
          .as_mut()
          .unwrap()
          .track_files
          .set_items(track_file_vec);
      })
      .await
  }
}
