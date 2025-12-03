use crate::models::Route;
use crate::models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock;
use crate::models::sonarr_models::SonarrHistoryWrapper;
use crate::network::sonarr_network::SonarrEvent;
use crate::network::{Network, RequestMethod};
use anyhow::Result;
use log::info;
use serde_json::Value;

#[cfg(test)]
#[path = "sonarr_history_network_tests.rs"]
mod sonarr_history_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::sonarr_network) async fn get_sonarr_history(
    &mut self,
    events: u64,
  ) -> Result<SonarrHistoryWrapper> {
    info!("Fetching all Sonarr history events");
    let event = SonarrEvent::GetHistory(events);

    let params = format!("pageSize={events}&sortDirection=descending&sortKey=date");
    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, Some(params))
      .await;

    self
      .handle_request::<(), SonarrHistoryWrapper>(request_props, |history_response, mut app| {
        if !matches!(
          app.get_current_route(),
          Route::Sonarr(ActiveSonarrBlock::HistorySortPrompt, _)
        ) {
          let mut history_vec = history_response.records;
          history_vec.sort_by(|a, b| a.id.cmp(&b.id));
          app.data.sonarr_data.history.set_items(history_vec);
          app.data.sonarr_data.history.apply_sorting_toggle(false);
        }
      })
      .await
  }

  pub(in crate::network::sonarr_network) async fn mark_sonarr_history_item_as_failed(
    &mut self,
    history_item_id: i64,
  ) -> Result<Value> {
    info!("Marking the Sonarr history item with ID: {history_item_id} as 'failed'");
    let event = SonarrEvent::MarkHistoryItemAsFailed(history_item_id);

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Post,
        None,
        Some(format!("/{history_item_id}")),
        None,
      )
      .await;

    self
      .handle_request::<(), Value>(request_props, |_, _| ())
      .await
  }
}
