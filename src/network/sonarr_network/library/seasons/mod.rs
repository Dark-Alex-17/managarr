use crate::models::Route;
use crate::models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock;
use crate::models::sonarr_models::{SonarrCommandBody, SonarrHistoryItem, SonarrRelease};
use crate::network::sonarr_network::SonarrEvent;
use crate::network::{Network, RequestMethod};
use anyhow::Result;
use log::{debug, info, warn};
use serde_json::{Value, json};

#[cfg(test)]
#[path = "sonarr_seasons_network_tests.rs"]
mod sonarr_seasons_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::sonarr_network) async fn toggle_sonarr_season_monitoring(
    &mut self,
    series_id_season_number_tuple: (i64, i64),
  ) -> Result<()> {
    let event = SonarrEvent::ToggleSeasonMonitoring(series_id_season_number_tuple);
    let (series_id, season_number) = series_id_season_number_tuple;

    let detail_event = SonarrEvent::GetSeriesDetails(series_id);
    info!("Toggling season monitoring for season {season_number} in series with ID: {series_id}");
    info!("Fetching series details for series with ID: {series_id}");

    let request_props = self
      .request_props_from(
        detail_event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{series_id}")),
        None,
      )
      .await;

    let mut response = String::new();

    self
      .handle_request::<(), Value>(request_props, |detailed_series_body, _| {
        response = detailed_series_body.to_string()
      })
      .await?;

    info!("Constructing toggle season monitoring body");

    match serde_json::from_str::<Value>(&response) {
      Ok(mut detailed_series_body) => {
        let monitored = detailed_series_body
          .get("seasons")
          .unwrap()
          .as_array()
          .unwrap()
          .iter()
          .find(|season| season["seasonNumber"] == season_number)
          .unwrap()
          .get("monitored")
          .unwrap()
          .as_bool()
          .unwrap();

        *detailed_series_body
          .get_mut("seasons")
          .unwrap()
          .as_array_mut()
          .unwrap()
          .iter_mut()
          .find(|season| season["seasonNumber"] == season_number)
          .unwrap()
          .get_mut("monitored")
          .unwrap() = json!(!monitored);

        debug!("Toggle season monitoring body: {detailed_series_body:?}");

        let request_props = self
          .request_props_from(
            event,
            RequestMethod::Put,
            Some(detailed_series_body),
            Some(format!("/{series_id}")),
            None,
          )
          .await;

        self
          .handle_request::<Value, ()>(request_props, |_, _| ())
          .await
      }
      Err(_) => {
        warn!("Request for detailed series body was interrupted");
        Ok(())
      }
    }
  }

  pub(in crate::network::sonarr_network) async fn get_season_releases(
    &mut self,
    series_season_id_tuple: (i64, i64),
  ) -> Result<Vec<SonarrRelease>> {
    let event = SonarrEvent::GetSeasonReleases(series_season_id_tuple);
    let (series_id, season_number) = series_season_id_tuple;
    info!("Fetching releases for series with ID: {series_id} and season number: {season_number}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("seriesId={series_id}&seasonNumber={season_number}")),
      )
      .await;

    self
      .handle_request::<(), Vec<SonarrRelease>>(request_props, |release_vec, mut app| {
        let season_details_modal = app
          .data
          .sonarr_data
          .season_details_modal
          .get_or_insert_default();

        let season_releases_vec = release_vec
          .into_iter()
          .filter(|release| release.full_season)
          .collect();

        season_details_modal
          .season_releases
          .set_items(season_releases_vec);
      })
      .await
  }

  pub(in crate::network::sonarr_network) async fn get_sonarr_season_history(
    &mut self,
    series_season_id_tuple: (i64, i64),
  ) -> Result<Vec<SonarrHistoryItem>> {
    let event = SonarrEvent::GetSeasonHistory(series_season_id_tuple);
    let (series_id, season_number) = series_season_id_tuple;
    info!("Fetching history for series with ID: {series_id} and season number: {season_number}");

    let params = format!("seriesId={series_id}&seasonNumber={season_number}",);
    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, Some(params))
      .await;

    self
      .handle_request::<(), Vec<SonarrHistoryItem>>(request_props, |history_items, mut app| {
        let is_sorting = matches!(
          app.get_current_route(),
          Route::Sonarr(ActiveSonarrBlock::SeasonHistorySortPrompt, _)
        );

        let season_details_modal = app
          .data
          .sonarr_data
          .season_details_modal
          .get_or_insert_default();

        if !is_sorting {
          let mut history_vec = history_items;
          history_vec.sort_by(|a, b| a.id.cmp(&b.id));
          season_details_modal.season_history.set_items(history_vec);
          season_details_modal
            .season_history
            .apply_sorting_toggle(false);
        }
      })
      .await
  }

  pub(in crate::network::sonarr_network) async fn trigger_automatic_season_search(
    &mut self,
    series_season_id_tuple: (i64, i64),
  ) -> Result<Value> {
    let event = SonarrEvent::TriggerAutomaticSeasonSearch(series_season_id_tuple);
    let (series_id, season_number) = series_season_id_tuple;
    info!("Searching indexers for series with ID: {series_id} and season number: {season_number}");

    let body = SonarrCommandBody {
      name: "SeasonSearch".to_owned(),
      season_number: Some(season_number),
      series_id: Some(series_id),
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
