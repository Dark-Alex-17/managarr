use crate::models::Route;
use crate::models::radarr_models::RadarrHistoryWrapper;
use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
use crate::network::radarr_network::RadarrEvent;
use crate::network::{Network, RequestMethod};
use anyhow::Result;
use log::info;
use serde_json::Value;

#[cfg(test)]
#[path = "radarr_history_network_tests.rs"]
mod radarr_history_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::radarr_network) async fn get_radarr_history(
    &mut self,
    events: u64,
  ) -> Result<RadarrHistoryWrapper> {
    info!("Fetching all Radarr history events");
    let event = RadarrEvent::GetHistory(events);

    let params = format!("pageSize={events}&sortDirection=descending&sortKey=date");
    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, Some(params))
      .await;

    self
      .handle_request::<(), RadarrHistoryWrapper>(request_props, |history_response, mut app| {
        if !matches!(
          app.get_current_route(),
          Route::Radarr(ActiveRadarrBlock::HistorySortPrompt, _)
        ) {
          let mut history_vec = history_response.records;
          history_vec.sort_by(|a, b| a.id.cmp(&b.id));
          app.data.radarr_data.history.set_items(history_vec);
          app.data.radarr_data.history.apply_sorting_toggle(false);
        }
      })
      .await
  }

  pub(in crate::network::radarr_network) async fn mark_radarr_history_item_as_failed(
    &mut self,
    history_item_id: i64,
  ) -> Result<Value> {
    info!("Marking the Radarr history item with ID: {history_item_id} as 'failed'");
    let event = RadarrEvent::MarkHistoryItemAsFailed(history_item_id);

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
