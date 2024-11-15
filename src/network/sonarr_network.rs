use std::collections::BTreeMap;

use anyhow::Result;
use log::info;
use managarr_tree_widget::TreeItem;
use serde_json::{json, Value};

use crate::{
  models::{
    servarr_data::sonarr::sonarr_data::ActiveSonarrBlock,
    sonarr_models::{
      BlocklistResponse, Episode, LogResponse, Series, SonarrSerdeable, SystemStatus,
    },
    HorizontallyScrollableText, Route, Scrollable,
  },
  network::RequestMethod,
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
  GetEpisodes(Option<i64>),
  GetLogs(Option<u64>),
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
      SonarrEvent::GetEpisodes(_) => "/episode",
      SonarrEvent::GetLogs(_) => "/log",
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

        app.data.sonarr_data.episodes.set_items(tree);
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
}
