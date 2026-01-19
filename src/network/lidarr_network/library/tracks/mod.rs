use crate::models::lidarr_models::{LidarrHistoryItem, MediaInfo, Track, TrackFile};
use crate::models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock;
use crate::models::servarr_data::lidarr::modals::AlbumDetailsModal;
use crate::models::{Route, ScrollableText};
use crate::network::lidarr_network::LidarrEvent;
use crate::network::{Network, RequestMethod};
use anyhow::Result;
use indoc::formatdoc;
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
        let album_details_modal = app
          .data
          .lidarr_data
          .album_details_modal
          .get_or_insert_default();

        album_details_modal.tracks.set_items(track_vec);
      })
      .await
  }

  pub(in crate::network::lidarr_network) async fn get_track_details(
    &mut self,
    track_id: i64,
  ) -> Result<Track> {
    let event = LidarrEvent::GetTrackDetails(track_id);
    info!("Fetching Lidarr track details for track with ID: {track_id}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{track_id}")),
        None,
      )
      .await;

    self
      .handle_request::<(), Track>(request_props, |track_response, mut app| {
        if app.cli_mode {
          app.data.lidarr_data.album_details_modal = Some(AlbumDetailsModal::default());
        }

        let Track {
          explicit,
          track_number,
          title,
          duration,
          track_file,
          ..
        } = track_response;
        let duration_secs = duration / 1000;
        let mins = duration_secs / 60;
        let secs = duration_secs % 60;
        let track_length = format!("{mins}:{secs:02}");
        let track_details_modal = app
          .data
          .lidarr_data
          .album_details_modal
          .as_mut()
          .expect("Album details modal is empty")
          .track_details_modal
          .get_or_insert_default();
        let mut details = formatdoc!(
          "
            Title: {title}
            Track Number: {track_number}
            Duration: {track_length}
            Explicit: {explicit}
          "
        );

        if let Some(file) = track_file {
          let TrackFile {
            path,
            size,
            quality,
            date_added,
            media_info,
            ..
          } = file;
          let quality_name = quality.quality.name;
          let size_mb = size as f64 / 1024f64.powi(2);

          details.push_str(&formatdoc!(
            "
              Quality: {quality_name}
              File Path: {path}
              File Size: {size_mb:.2} MB
              Date Added: {date_added}
            "
          ));

          if let Some(info) = media_info {
            let MediaInfo {
              audio_bit_rate,
              audio_channels,
              audio_codec,
              audio_bits,
              audio_sample_rate,
            } = info;

            details.push_str(&formatdoc!(
              "
              Codec: {}
              Channels: {}
              Bits: {}
              Bit Rate: {}
              Sample Rate: {}
            ",
              audio_codec.unwrap_or_default(),
              audio_channels,
              audio_bits.unwrap_or_default(),
              audio_bit_rate.unwrap_or_default(),
              audio_sample_rate.unwrap_or_default()
            ));
          }
        }

        track_details_modal.track_details = ScrollableText::with_string(details);
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
        let album_details_modal = app
          .data
          .lidarr_data
          .album_details_modal
          .get_or_insert_default();

        album_details_modal.track_files.set_items(track_file_vec);
      })
      .await
  }

  pub(in crate::network::lidarr_network) async fn get_lidarr_track_history(
    &mut self,
    artist_id: i64,
    album_id: i64,
    track_id: i64,
  ) -> Result<Vec<LidarrHistoryItem>> {
    let event = LidarrEvent::GetTrackHistory(artist_id, album_id, track_id);
    info!(
      "Fetching history for artist with ID: {artist_id} and album with ID: {album_id} and track with ID: {track_id}"
    );

    let params = format!("artistId={artist_id}&albumId={album_id}");
    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, Some(params))
      .await;

    self
      .handle_request::<(), Vec<LidarrHistoryItem>>(request_props, |history_items, mut app| {
        let is_sorting = matches!(
          app.get_current_route(),
          Route::Lidarr(ActiveLidarrBlock::TrackHistorySortPrompt, _)
        );

        let album_details_modal = app
          .data
          .lidarr_data
          .album_details_modal
          .get_or_insert_default();
        let track_details_modal = album_details_modal
          .track_details_modal
          .get_or_insert_default();

        if !is_sorting {
          let mut history_vec: Vec<LidarrHistoryItem> = history_items
            .into_iter()
            .filter(|it| it.track_id == track_id)
            .collect();
          history_vec.sort_by(|a, b| a.id.cmp(&b.id));
          track_details_modal.track_history.set_items(history_vec);
          track_details_modal
            .track_history
            .apply_sorting_toggle(false);
        }
      })
      .await
  }
}
