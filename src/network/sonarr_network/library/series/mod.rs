use crate::models::Route;
use crate::models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock;
use crate::models::sonarr_models::{
  AddSeriesBody, AddSeriesSearchResult, DeleteSeriesParams, EditSeriesParams, Series,
  SonarrCommandBody, SonarrHistoryItem,
};
use crate::models::stateful_table::StatefulTable;
use crate::network::sonarr_network::SonarrEvent;
use crate::network::{Network, RequestMethod};
use anyhow::Result;
use log::{debug, info, warn};
use serde_json::{Value, json};
use urlencoding::encode;

#[cfg(test)]
#[path = "sonarr_series_network_tests.rs"]
mod sonarr_series_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::sonarr_network) async fn add_sonarr_series(
    &mut self,
    mut add_series_body: AddSeriesBody,
  ) -> anyhow::Result<Value> {
    info!("Adding new series to Sonarr");
    let event = SonarrEvent::AddSeries(AddSeriesBody::default());
    if let Some(tag_input_str) = add_series_body.tag_input_string.as_ref() {
      let tag_ids_vec = self.extract_and_add_sonarr_tag_ids_vec(tag_input_str).await;
      add_series_body.tags = tag_ids_vec;
    }

    debug!("Add series body: {add_series_body:?}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Post,
        Some(add_series_body),
        None,
        None,
      )
      .await;

    self
      .handle_request::<AddSeriesBody, Value>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::sonarr_network) async fn delete_series(
    &mut self,
    delete_series_params: DeleteSeriesParams,
  ) -> Result<()> {
    let event = SonarrEvent::DeleteSeries(DeleteSeriesParams::default());
    let DeleteSeriesParams {
      id,
      delete_series_files,
      add_list_exclusion,
    } = delete_series_params;

    info!(
      "Deleting Sonarr series with ID: {id} with deleteFiles={delete_series_files} and addImportExclusion={add_list_exclusion}"
    );

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Delete,
        None::<()>,
        Some(format!("/{id}")),
        Some(format!(
          "deleteFiles={delete_series_files}&addImportExclusion={add_list_exclusion}"
        )),
      )
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::sonarr_network) async fn edit_sonarr_series(
    &mut self,
    mut edit_series_params: EditSeriesParams,
  ) -> Result<()> {
    info!("Editing Sonarr series");
    if let Some(tag_input_str) = edit_series_params.tag_input_string.as_ref() {
      let tag_ids_vec = self.extract_and_add_sonarr_tag_ids_vec(tag_input_str).await;
      edit_series_params.tags = Some(tag_ids_vec);
    }
    let series_id = edit_series_params.series_id;
    let detail_event = SonarrEvent::GetSeriesDetails(series_id);
    let event = SonarrEvent::EditSeries(EditSeriesParams::default());
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

    info!("Constructing edit series body");

    let mut detailed_series_body: Value = serde_json::from_str(&response)?;
    let (
      monitored,
      use_season_folders,
      series_type,
      quality_profile_id,
      language_profile_id,
      root_folder_path,
      tags,
    ) = {
      let monitored = edit_series_params.monitored.unwrap_or(
        detailed_series_body["monitored"]
          .as_bool()
          .expect("Unable to deserialize 'monitored'"),
      );
      let use_season_folders = edit_series_params.use_season_folders.unwrap_or(
        detailed_series_body["seasonFolder"]
          .as_bool()
          .expect("Unable to deserialize 'season_folder'"),
      );
      let series_type = edit_series_params
        .series_type
        .unwrap_or_else(|| {
          serde_json::from_value(detailed_series_body["seriesType"].clone())
            .expect("Unable to deserialize 'seriesType'")
        })
        .to_string();
      let quality_profile_id = edit_series_params.quality_profile_id.unwrap_or_else(|| {
        detailed_series_body["qualityProfileId"]
          .as_i64()
          .expect("Unable to deserialize 'qualityProfileId'")
      });
      let language_profile_id = edit_series_params.language_profile_id.unwrap_or_else(|| {
        detailed_series_body["languageProfileId"]
          .as_i64()
          .expect("Unable to deserialize 'languageProfileId'")
      });
      let root_folder_path = edit_series_params.root_folder_path.unwrap_or_else(|| {
        detailed_series_body["path"]
          .as_str()
          .expect("Unable to deserialize 'path'")
          .to_owned()
      });
      let tags = if edit_series_params.clear_tags {
        vec![]
      } else {
        edit_series_params.tags.unwrap_or(
          detailed_series_body["tags"]
            .as_array()
            .expect("Unable to deserialize 'tags'")
            .iter()
            .map(|item| item.as_i64().expect("Unable to deserialize tag ID"))
            .collect(),
        )
      };

      (
        monitored,
        use_season_folders,
        series_type,
        quality_profile_id,
        language_profile_id,
        root_folder_path,
        tags,
      )
    };

    *detailed_series_body.get_mut("monitored").unwrap() = json!(monitored);
    *detailed_series_body.get_mut("seasonFolder").unwrap() = json!(use_season_folders);
    *detailed_series_body.get_mut("seriesType").unwrap() = json!(series_type);
    *detailed_series_body.get_mut("qualityProfileId").unwrap() = json!(quality_profile_id);
    *detailed_series_body.get_mut("languageProfileId").unwrap() = json!(language_profile_id);
    *detailed_series_body.get_mut("path").unwrap() = json!(root_folder_path);
    *detailed_series_body.get_mut("tags").unwrap() = json!(tags);

    debug!("Edit series body: {detailed_series_body:?}");

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

  pub(in crate::network::sonarr_network) async fn toggle_sonarr_series_monitoring(
    &mut self,
    series_id: i64,
  ) -> Result<()> {
    let event = SonarrEvent::ToggleSeriesMonitoring(series_id);

    let detail_event = SonarrEvent::GetSeriesDetails(series_id);
    info!("Toggling series monitoring for series with ID: {series_id}");
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

    info!("Constructing toggle series monitoring body");

    match serde_json::from_str::<Value>(&response) {
      Ok(mut detailed_series_body) => {
        let monitored = detailed_series_body
          .get("monitored")
          .unwrap()
          .as_bool()
          .unwrap();

        *detailed_series_body.get_mut("monitored").unwrap() = json!(!monitored);

        debug!("Toggle series monitoring body: {detailed_series_body:?}");

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

  pub(in crate::network::sonarr_network) async fn get_series_details(
    &mut self,
    series_id: i64,
  ) -> Result<Series> {
    info!("Fetching details for Sonarr series with ID: {series_id}");
    let event = SonarrEvent::GetSeriesDetails(series_id);

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{series_id}")),
        None,
      )
      .await;

    self
      .handle_request::<(), Series>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::sonarr_network) async fn get_sonarr_series_history(
    &mut self,
    series_id: i64,
  ) -> Result<Vec<SonarrHistoryItem>> {
    info!("Fetching Sonarr series history for series with ID: {series_id}");
    let event = SonarrEvent::GetSeriesHistory(series_id);

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
      .handle_request::<(), Vec<SonarrHistoryItem>>(request_props, |mut history_vec, mut app| {
        let is_sorting = matches!(
          app.get_current_route(),
          Route::Sonarr(ActiveSonarrBlock::SeriesHistorySortPrompt, _)
        );

        let series_history = app
          .data
          .sonarr_data
          .series_history
          .get_or_insert_default();

        if !is_sorting {
          history_vec.sort_by(|a, b| a.id.cmp(&b.id));
          series_history.set_items(history_vec);
          series_history.apply_sorting_toggle(false);
        }
      })
      .await
  }

  pub(in crate::network::sonarr_network) async fn list_series(&mut self) -> Result<Vec<Series>> {
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

  pub(in crate::network::sonarr_network) async fn search_sonarr_series(
    &mut self,
    query: String,
  ) -> Result<Vec<AddSeriesSearchResult>> {
    info!("Searching for specific Sonarr series");
    let event = SonarrEvent::SearchNewSeries(String::new());

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("term={}", encode(&query))),
      )
      .await;

    self
      .handle_request::<(), Vec<AddSeriesSearchResult>>(request_props, |series_vec, mut app| {
        if series_vec.is_empty() {
          app.pop_and_push_navigation_stack(ActiveSonarrBlock::AddSeriesEmptySearchResults.into());
        } else if let Some(add_searched_seriess) = app.data.sonarr_data.add_searched_series.as_mut()
        {
          add_searched_seriess.set_items(series_vec);
        } else {
          let mut add_searched_seriess = StatefulTable::default();
          add_searched_seriess.set_items(series_vec);
          app.data.sonarr_data.add_searched_series = Some(add_searched_seriess);
        }
      })
      .await
  }

  pub(in crate::network::sonarr_network) async fn trigger_automatic_series_search(
    &mut self,
    series_id: i64,
  ) -> Result<Value> {
    let event = SonarrEvent::TriggerAutomaticSeriesSearch(series_id);
    info!("Searching indexers for series with ID: {series_id}");

    let body = SonarrCommandBody {
      name: "SeriesSearch".to_owned(),
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

  pub(in crate::network::sonarr_network) async fn update_all_series(&mut self) -> Result<Value> {
    info!("Updating all series");
    let event = SonarrEvent::UpdateAllSeries;
    let body = SonarrCommandBody {
      name: "RefreshSeries".to_owned(),
      ..SonarrCommandBody::default()
    };

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(body), None, None)
      .await;

    self
      .handle_request::<SonarrCommandBody, Value>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::sonarr_network) async fn update_and_scan_series(
    &mut self,
    series_id: i64,
  ) -> Result<Value> {
    let event = SonarrEvent::UpdateAndScanSeries(series_id);
    info!("Updating and scanning series with ID: {series_id}");
    let body = SonarrCommandBody {
      name: "RefreshSeries".to_owned(),
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
