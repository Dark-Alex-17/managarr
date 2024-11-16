use std::collections::BTreeMap;

use anyhow::Result;
use indoc::formatdoc;
use log::info;
use managarr_tree_widget::TreeItem;
use serde_json::{json, Value};

use crate::{
  models::{
    servarr_data::sonarr::{modals::EpisodeDetailsModal, sonarr_data::ActiveSonarrBlock},
    sonarr_models::{
      BlocklistResponse, DownloadRecord, Episode, LogResponse, QualityProfile, Series,
      SonarrSerdeable, SystemStatus,
    },
    HorizontallyScrollableText, Route, Scrollable, ScrollableText,
  },
  network::RequestMethod,
  utils::convert_to_gb,
};

use super::{Network, NetworkEvent, NetworkResource};
#[cfg(test)]
#[path = "sonarr_network_tests.rs"]
mod sonarr_network_tests;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SonarrEvent {
  ClearBlocklist,
  DeleteBlocklistItem(Option<i64>),
  GetBlocklist,
  GetEpisodeDetails(Option<i64>),
  GetEpisodes(Option<i64>),
  GetLogs(Option<u64>),
  GetQualityProfiles,
  GetStatus,
  HealthCheck,
  ListSeries,
}

impl NetworkResource for SonarrEvent {
  fn resource(&self) -> &'static str {
    match &self {
      SonarrEvent::ClearBlocklist => "/blocklist/bulk",
      SonarrEvent::DeleteBlocklistItem(_) => "/blocklist",
      SonarrEvent::GetBlocklist => "/blocklist?page=1&pageSize=10000",
      SonarrEvent::GetEpisodes(_) | SonarrEvent::GetEpisodeDetails(_) => "/episode",
      SonarrEvent::GetLogs(_) => "/log",
      SonarrEvent::GetQualityProfiles => "/qualityprofile",
      SonarrEvent::GetStatus => "/system/status",
      SonarrEvent::HealthCheck => "/health",
      SonarrEvent::ListSeries => "/series",
    }
  }
}

impl From<SonarrEvent> for NetworkEvent {
  fn from(sonarr_event: SonarrEvent) -> Self {
    NetworkEvent::Sonarr(sonarr_event)
  }
}

impl<'a, 'b> Network<'a, 'b> {
  pub async fn handle_sonarr_event(
    &mut self,
    sonarr_event: SonarrEvent,
  ) -> Result<SonarrSerdeable> {
    match sonarr_event {
      SonarrEvent::ClearBlocklist => self
        .clear_sonarr_blocklist()
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::DeleteBlocklistItem(blocklist_item_id) => self
        .delete_sonarr_blocklist_item(blocklist_item_id)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::GetBlocklist => self.get_sonarr_blocklist().await.map(SonarrSerdeable::from),
      SonarrEvent::GetEpisodes(series_id) => self
        .get_episodes(series_id)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::GetEpisodeDetails(episode_id) => self
        .get_episode_details(episode_id)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::GetQualityProfiles => self
        .get_sonarr_quality_profiles()
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::GetLogs(events) => self
        .get_sonarr_logs(events)
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::GetStatus => self.get_sonarr_status().await.map(SonarrSerdeable::from),
      SonarrEvent::HealthCheck => self
        .get_sonarr_healthcheck()
        .await
        .map(SonarrSerdeable::from),
      SonarrEvent::ListSeries => self.list_series().await.map(SonarrSerdeable::from),
    }
  }

  async fn clear_sonarr_blocklist(&mut self) -> Result<()> {
    info!("Clearing Sonarr blocklist");
    let event = SonarrEvent::ClearBlocklist;

    let ids = self
      .app
      .lock()
      .await
      .data
      .sonarr_data
      .blocklist
      .items
      .iter()
      .map(|item| item.id)
      .collect::<Vec<i64>>();

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Delete,
        Some(json!({"ids": ids})),
        None,
        None,
      )
      .await;

    self
      .handle_request::<Value, ()>(request_props, |_, _| ())
      .await
  }

  async fn delete_sonarr_blocklist_item(&mut self, blocklist_item_id: Option<i64>) -> Result<()> {
    let event = SonarrEvent::DeleteBlocklistItem(None);
    let id = if let Some(b_id) = blocklist_item_id {
      b_id
    } else {
      self
        .app
        .lock()
        .await
        .data
        .sonarr_data
        .blocklist
        .current_selection()
        .id
    };

    info!("Deleting Sonarr blocklist item for item with id: {id}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Delete,
        None::<()>,
        Some(format!("/{id}")),
        None,
      )
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }

  async fn get_sonarr_healthcheck(&mut self) -> Result<()> {
    info!("Performing Sonarr health check");
    let event = SonarrEvent::HealthCheck;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }

  async fn get_sonarr_blocklist(&mut self) -> Result<BlocklistResponse> {
    info!("Fetching Sonarr blocklist");
    let event = SonarrEvent::GetBlocklist;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), BlocklistResponse>(request_props, |blocklist_resp, mut app| {
        if !matches!(
          app.get_current_route(),
          Route::Sonarr(ActiveSonarrBlock::BlocklistSortPrompt, _)
        ) {
          let mut blocklist_vec = blocklist_resp.records;
          blocklist_vec.sort_by(|a, b| a.id.cmp(&b.id));
          app.data.sonarr_data.blocklist.set_items(blocklist_vec);
          app.data.sonarr_data.blocklist.apply_sorting_toggle(false);
        }
      })
      .await
  }

  async fn get_episodes(&mut self, series_id: Option<i64>) -> Result<Vec<Episode>> {
    let event = SonarrEvent::GetEpisodes(series_id);
    let (id, series_id_param) = self.extract_series_id(series_id).await;
    info!("Fetching episodes for Sonarr series with ID: {id}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(series_id_param),
      )
      .await;

    self
      .handle_request::<(), Vec<Episode>>(request_props, |mut episode_vec, mut app| {
        episode_vec.sort_by(|a, b| a.id.cmp(&b.id));
        if !matches!(
          app.get_current_route(),
          Route::Sonarr(ActiveSonarrBlock::EpisodesTableSortPrompt, _)
        ) {
          app
            .data
            .sonarr_data
            .episodes_table
            .set_items(episode_vec.clone());
          app
            .data
            .sonarr_data
            .episodes_table
            .apply_sorting_toggle(false);
        }

        let mut seasons = BTreeMap::new();

        for episode in episode_vec {
          seasons
            .entry(episode.season_number)
            .or_insert_with(Vec::new)
            .push(episode);
        }

        let tree = seasons
          .into_iter()
          .map(|(season, episodes_vec)| {
            let marker_episode = Episode {
              title: Some(format!("Season {season}")),
              ..Episode::default()
            };
            let children = episodes_vec.into_iter().map(TreeItem::new_leaf).collect();

            TreeItem::new(marker_episode, children).expect("All item identifiers must be unique")
          })
          .collect();

        app.data.sonarr_data.episodes_tree.set_items(tree);
      })
      .await
  }

  async fn get_episode_details(&mut self, episode_id: Option<i64>) -> Result<Episode> {
    info!("Fetching Sonarr episode details");
    let event = SonarrEvent::GetEpisodeDetails(None);
    let id = self.extract_episode_id(episode_id).await;

    info!("Fetching episode details for episode with ID: {id}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{id}")),
        None,
      )
      .await;

    self
      .handle_request::<(), Episode>(request_props, |episode_response, mut app| {
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
        let mut episode_details_modal = EpisodeDetailsModal {
          episode_details: ScrollableText::with_string(formatdoc!(
            "
            Title: {}
            Season: {season_number}
            Episode Number: {episode_number}
            Air Date: {air_date}
            Status: {status}
            Description: {}",
            title.unwrap_or_default(),
            overview.unwrap_or_default(),
          )),
          ..EpisodeDetailsModal::default()
        };
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
            file.language.name,
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
              media_info.video_codec,
              media_info.video_fps.as_f64().unwrap(),
              media_info.resolution,
              media_info.scan_type,
              media_info.run_time,
              media_info.subtitles.unwrap_or_default()
            );
          }
        };

        app.data.sonarr_data.episode_details_modal = Some(episode_details_modal);
      })
      .await
  }

  async fn get_sonarr_logs(&mut self, events: Option<u64>) -> Result<LogResponse> {
    info!("Fetching Sonarr logs");
    let event = SonarrEvent::GetLogs(events);

    let params = format!(
      "pageSize={}&sortDirection=descending&sortKey=time",
      events.unwrap_or(500)
    );
    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, Some(params))
      .await;

    self
      .handle_request::<(), LogResponse>(request_props, |log_response, mut app| {
        let mut logs = log_response.records;
        logs.reverse();

        let log_lines = logs
          .into_iter()
          .map(|log| {
            if log.exception.is_some() {
              HorizontallyScrollableText::from(format!(
                "{}|{}|{}|{}|{}",
                log.time,
                log.level.to_uppercase(),
                log.logger.as_ref().unwrap(),
                log.exception_type.as_ref().unwrap(),
                log.exception.as_ref().unwrap()
              ))
            } else {
              HorizontallyScrollableText::from(format!(
                "{}|{}|{}|{}",
                log.time,
                log.level.to_uppercase(),
                log.logger.as_ref().unwrap(),
                log.message.as_ref().unwrap()
              ))
            }
          })
          .collect();

        app.data.sonarr_data.logs.set_items(log_lines);
        app.data.sonarr_data.logs.scroll_to_bottom();
      })
      .await
  }

  async fn get_sonarr_quality_profiles(&mut self) -> Result<Vec<QualityProfile>> {
    info!("Fetching Sonarr quality profiles");
    let event = SonarrEvent::GetQualityProfiles;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<QualityProfile>>(request_props, |quality_profiles, mut app| {
        app.data.sonarr_data.quality_profile_map = quality_profiles
          .into_iter()
          .map(|profile| (profile.id, profile.name))
          .collect();
      })
      .await
  }

  async fn list_series(&mut self) -> Result<Vec<Series>> {
    info!("Fetching Sonarr library");
    let event = SonarrEvent::ListSeries;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<Series>>(request_props, |mut series_vec, mut app| {
        if !matches!(
          app.get_current_route(),
          Route::Sonarr(ActiveSonarrBlock::SeriesSortPrompt, _)
        ) {
          series_vec.sort_by(|a, b| a.id.cmp(&b.id));
          app.data.sonarr_data.series.set_items(series_vec);
          app.data.sonarr_data.series.apply_sorting_toggle(false);
        }
      })
      .await
  }

  async fn get_sonarr_status(&mut self) -> Result<SystemStatus> {
    info!("Fetching Sonarr system status");
    let event = SonarrEvent::GetStatus;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), SystemStatus>(request_props, |system_status, mut app| {
        app.data.sonarr_data.version = system_status.version;
        app.data.sonarr_data.start_time = system_status.start_time;
      })
      .await
  }

  async fn extract_series_id(&mut self, series_id: Option<i64>) -> (i64, String) {
    let series_id = if let Some(id) = series_id {
      id
    } else {
      self
        .app
        .lock()
        .await
        .data
        .sonarr_data
        .series
        .current_selection()
        .id
    };
    (series_id, format!("seriesId={series_id}"))
  }

  async fn extract_episode_id(&mut self, episode_id: Option<i64>) -> i64 {
    let app = self.app.lock().await;

    let episode_id = if let Some(id) = episode_id {
      id
    } else if matches!(
      app.get_current_route(),
      Route::Sonarr(ActiveSonarrBlock::EpisodesTable, _)
    ) {
      app.data.sonarr_data.episodes_table.current_selection().id
    } else {
      app
        .data
        .sonarr_data
        .episodes_tree
        .current_selection()
        .as_ref()
        .unwrap()
        .id
    };

    episode_id
  }
}

fn get_episode_status(has_file: bool, downloads_vec: &[DownloadRecord], episode_id: i64) -> String {
  if !has_file {
    if let Some(download) = downloads_vec
      .iter()
      .find(|&download| download.episode_id == episode_id)
    {
      if download.status == "downloading" {
        return "Downloading".to_owned();
      }

      if download.status == "completed" {
        return "Awaiting Import".to_owned();
      }
    }

    return "Missing".to_owned();
  }

  "Downloaded".to_owned()
}
