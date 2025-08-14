use crate::models::servarr_data::sonarr::modals::{EpisodeDetailsModal, SeasonDetailsModal};
use crate::models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock;
use crate::models::servarr_models::Language;
use crate::models::sonarr_models::{
  DownloadRecord, DownloadStatus, Episode, EpisodeFile, MonitorEpisodeBody, SonarrCommandBody,
  SonarrHistoryWrapper, SonarrRelease,
};
use crate::models::{Route, ScrollableText};
use crate::network::sonarr_network::SonarrEvent;
use crate::network::{Network, RequestMethod};
use crate::utils::convert_to_gb;
use anyhow::Result;
use indoc::formatdoc;
use log::info;
use serde_json::{Number, Value};

#[cfg(test)]
#[path = "sonarr_episodes_network_tests.rs"]
mod sonarr_episodes_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::sonarr_network) async fn delete_sonarr_episode_file(
    &mut self,
    episode_file_id: i64,
  ) -> Result<()> {
    let event = SonarrEvent::DeleteEpisodeFile(episode_file_id);
    info!("Deleting Sonarr episode file for episode file with id: {episode_file_id}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Delete,
        None::<()>,
        Some(format!("/{episode_file_id}")),
        None,
      )
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::sonarr_network) async fn get_episodes(
    &mut self,
    series_id: i64,
  ) -> Result<Vec<Episode>> {
    let event = SonarrEvent::GetEpisodes(series_id);
    info!("Fetching episodes for Sonarr series with ID: {series_id}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("seriesId={series_id}")),
      )
      .await;

    self
      .handle_request::<(), Vec<Episode>>(request_props, |mut episode_vec, mut app| {
        episode_vec.sort_by(|a, b| a.id.cmp(&b.id));
        if !matches!(
          app.get_current_route(),
          Route::Sonarr(ActiveSonarrBlock::EpisodesSortPrompt, _)
        ) {
          if app.data.sonarr_data.season_details_modal.is_none() {
            app.data.sonarr_data.season_details_modal = Some(SeasonDetailsModal::default());
          }

          let season_episodes_vec = if !app.data.sonarr_data.seasons.is_empty() {
            let season_number = app
              .data
              .sonarr_data
              .seasons
              .current_selection()
              .season_number;

            episode_vec
              .into_iter()
              .filter(|episode| episode.season_number == season_number)
              .collect()
          } else {
            episode_vec
          };

          app
            .data
            .sonarr_data
            .season_details_modal
            .as_mut()
            .unwrap()
            .episodes
            .set_items(season_episodes_vec);
          app
            .data
            .sonarr_data
            .season_details_modal
            .as_mut()
            .unwrap()
            .episodes
            .apply_sorting_toggle(false);
        }
      })
      .await
  }

  pub(in crate::network::sonarr_network) async fn get_episode_files(
    &mut self,
    series_id: i64,
  ) -> Result<Vec<EpisodeFile>> {
    let event = SonarrEvent::GetEpisodeFiles(series_id);
    info!("Fetching episodes files for Sonarr series with ID: {series_id}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("seriesId={series_id}")),
      )
      .await;

    self
      .handle_request::<(), Vec<EpisodeFile>>(request_props, |episode_file_vec, mut app| {
        if app.data.sonarr_data.season_details_modal.is_none() {
          app.data.sonarr_data.season_details_modal = Some(SeasonDetailsModal::default());
        }

        app
          .data
          .sonarr_data
          .season_details_modal
          .as_mut()
          .unwrap()
          .episode_files
          .set_items(episode_file_vec);
      })
      .await
  }

  pub(in crate::network::sonarr_network) async fn get_sonarr_episode_history(
    &mut self,
    episode_id: i64,
  ) -> Result<SonarrHistoryWrapper> {
    info!("Fetching Sonarr history for episode with ID: {episode_id}");
    let event = SonarrEvent::GetEpisodeHistory(episode_id);

    let params =
      format!("episodeId={episode_id}&pageSize=1000&sortDirection=descending&sortKey=date");
    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, Some(params))
      .await;

    self
      .handle_request::<(), SonarrHistoryWrapper>(request_props, |history_response, mut app| {
        if app.data.sonarr_data.season_details_modal.is_none() {
          app.data.sonarr_data.season_details_modal = Some(SeasonDetailsModal::default());
        }

        if app
          .data
          .sonarr_data
          .season_details_modal
          .as_ref()
          .unwrap()
          .episode_details_modal
          .is_none()
        {
          app
            .data
            .sonarr_data
            .season_details_modal
            .as_mut()
            .unwrap()
            .episode_details_modal = Some(EpisodeDetailsModal::default());
        }

        let mut history_vec = history_response.records;
        history_vec.sort_by(|a, b| a.id.cmp(&b.id));
        app
          .data
          .sonarr_data
          .season_details_modal
          .as_mut()
          .unwrap()
          .episode_details_modal
          .as_mut()
          .unwrap()
          .episode_history
          .set_items(history_vec);
        app
          .data
          .sonarr_data
          .season_details_modal
          .as_mut()
          .unwrap()
          .episode_details_modal
          .as_mut()
          .unwrap()
          .episode_history
          .apply_sorting_toggle(false);
      })
      .await
  }

  pub(in crate::network::sonarr_network) async fn get_episode_details(
    &mut self,
    episode_id: i64,
  ) -> Result<Episode> {
    info!("Fetching Sonarr episode details");
    let event = SonarrEvent::GetEpisodeDetails(episode_id);

    info!("Fetching episode details for episode with ID: {episode_id}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{episode_id}")),
        None,
      )
      .await;

    self
      .handle_request::<(), Episode>(request_props, |episode_response, mut app| {
        if app.cli_mode {
          app.data.sonarr_data.season_details_modal = Some(SeasonDetailsModal::default());
        }

        if app
          .data
          .sonarr_data
          .season_details_modal
          .as_mut()
          .expect("Season details modal is empty")
          .episode_details_modal
          .is_none()
        {
          app
            .data
            .sonarr_data
            .season_details_modal
            .as_mut()
            .unwrap()
            .episode_details_modal = Some(EpisodeDetailsModal::default());
        }

        let Episode {
          id,
          title,
          air_date_utc,
          overview,
          has_file,
          season_number,
          episode_number,
          episode_file,
          ..
        } = episode_response;
        let status = get_episode_status(has_file, &app.data.sonarr_data.downloads.items, id);
        let air_date = if let Some(air_date) = air_date_utc {
          format!("{air_date}")
        } else {
          String::new()
        };
        let episode_details_modal = app
          .data
          .sonarr_data
          .season_details_modal
          .as_mut()
          .unwrap()
          .episode_details_modal
          .as_mut()
          .unwrap();
        episode_details_modal.episode_details = ScrollableText::with_string(formatdoc!(
          "
            Title: {}
            Season: {season_number}
            Episode Number: {episode_number}
            Air Date: {air_date}
            Status: {status}
            Description: {}",
          title,
          overview.unwrap_or_default(),
        ));
        if let Some(file) = episode_file {
          let size = convert_to_gb(file.size);
          episode_details_modal.file_details = formatdoc!(
            "
            Relative Path: {}
            Absolute Path: {}
            Size: {size:.2} GB
            Language: {}
            Date Added: {}",
            file.relative_path,
            file.path,
            file.languages.first().unwrap_or(&Language::default()).name,
            file.date_added,
          );

          if let Some(media_info) = file.media_info {
            episode_details_modal.audio_details = formatdoc!(
              "
              Bitrate: {}
              Channels: {:.1}
              Codec: {}
              Languages: {}
              Stream Count: {}",
              media_info.audio_bitrate,
              media_info.audio_channels.as_f64().unwrap(),
              media_info.audio_codec.unwrap_or_default(),
              media_info.audio_languages.unwrap_or_default(),
              media_info.audio_stream_count
            );

            episode_details_modal.video_details = formatdoc!(
              "
              Bit Depth: {}
              Bitrate: {}
              Codec: {}
              FPS: {}
              Resolution: {}
              Scan Type: {}
              Runtime: {}
              Subtitles: {}",
              media_info.video_bit_depth,
              media_info.video_bitrate,
              media_info.video_codec.unwrap_or_default(),
              media_info.video_fps.as_f64().unwrap(),
              media_info.resolution,
              media_info.scan_type,
              media_info.run_time,
              media_info.subtitles.unwrap_or_default()
            );
          }
        };
      })
      .await
  }

  pub(in crate::network::sonarr_network) async fn get_episode_releases(
    &mut self,
    episode_id: i64,
  ) -> Result<Vec<SonarrRelease>> {
    let event = SonarrEvent::GetEpisodeReleases(episode_id);
    info!("Fetching releases for episode with ID: {episode_id}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("episodeId={episode_id}")),
      )
      .await;

    self
      .handle_request::<(), Vec<SonarrRelease>>(request_props, |release_vec, mut app| {
        if app.data.sonarr_data.season_details_modal.is_none() {
          app.data.sonarr_data.season_details_modal = Some(SeasonDetailsModal::default());
        }

        if app
          .data
          .sonarr_data
          .season_details_modal
          .as_mut()
          .unwrap()
          .episode_details_modal
          .is_none()
        {
          app
            .data
            .sonarr_data
            .season_details_modal
            .as_mut()
            .unwrap()
            .episode_details_modal = Some(EpisodeDetailsModal::default());
        }

        app
          .data
          .sonarr_data
          .season_details_modal
          .as_mut()
          .unwrap()
          .episode_details_modal
          .as_mut()
          .unwrap()
          .episode_releases
          .set_items(release_vec);
      })
      .await
  }

  pub(in crate::network::sonarr_network) async fn toggle_sonarr_episode_monitoring(
    &mut self,
    episode_id: i64,
  ) -> Result<()> {
    let event = SonarrEvent::ToggleEpisodeMonitoring(episode_id);
    let detail_event = SonarrEvent::GetEpisodeDetails(0);

    let monitored = {
      info!("Fetching episode details for episode id: {episode_id}");
      let request_props = self
        .request_props_from(
          detail_event,
          RequestMethod::Get,
          None::<()>,
          Some(format!("/{episode_id}")),
          None,
        )
        .await;

      let mut monitored = false;

      self
        .handle_request::<(), Value>(request_props, |detailed_episode_body, _| {
          monitored = detailed_episode_body
            .get("monitored")
            .unwrap()
            .as_bool()
            .unwrap();
        })
        .await?;

      monitored
    };

    info!("Toggling monitoring for episode id: {episode_id}");

    let body = MonitorEpisodeBody {
      episode_ids: vec![episode_id],
      monitored: !monitored,
    };

    let request_props = self
      .request_props_from(event, RequestMethod::Put, Some(body), None, None)
      .await;

    self
      .handle_request::<MonitorEpisodeBody, ()>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::sonarr_network) async fn trigger_automatic_episode_search(
    &mut self,
    episode_id: i64,
  ) -> Result<Value> {
    let event = SonarrEvent::TriggerAutomaticEpisodeSearch(episode_id);
    info!("Searching indexers for episode with ID: {episode_id}");

    let body = SonarrCommandBody {
      name: "EpisodeSearch".to_owned(),
      episode_ids: Some(vec![episode_id]),
      ..SonarrCommandBody::default()
    };

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(body), None, None)
      .await;

    self
      .handle_request::<SonarrCommandBody, Value>(request_props, |_, _| ())
      .await
  }
}

fn get_episode_status(has_file: bool, downloads_vec: &[DownloadRecord], episode_id: i64) -> String {
  if !has_file {
    let default_episode_id = Number::from(-1i64);
    if let Some(download) = downloads_vec.iter().find(|&download| {
      download
        .episode_id
        .as_ref()
        .unwrap_or(&default_episode_id)
        .as_i64()
        .unwrap()
        == episode_id
    }) {
      if download.status == DownloadStatus::Downloading {
        return "Downloading".to_owned();
      }

      if download.status == DownloadStatus::Completed {
        return "Awaiting Import".to_owned();
      }
    }

    return "Missing".to_owned();
  }

  "Downloaded".to_owned()
}
