use crate::models::Route;
use crate::models::lidarr_models::LidarrHistoryWrapper;
use crate::models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock;
use crate::network::lidarr_network::LidarrEvent;
use crate::network::{Network, RequestMethod};
use anyhow::Result;
use log::info;
use serde_json::Value;

#[cfg(test)]
#[path = "lidarr_history_network_tests.rs"]
mod lidarr_history_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::lidarr_network) async fn get_lidarr_history(
    &mut self,
    events: u64,
  ) -> Result<LidarrHistoryWrapper> {
    info!("Fetching all Lidarr history events");
    let event = LidarrEvent::GetHistory(events);

    let params = format!("pageSize={events}&sortDirection=descending&sortKey=date");
    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, Some(params))
      .await;

    self
      .handle_request::<(), LidarrHistoryWrapper>(request_props, |history_response, mut app| {
        if !matches!(
          app.get_current_route(),
          Route::Lidarr(ActiveLidarrBlock::HistorySortPrompt, _)
        ) {
          let mut history_vec = history_response.records;
          history_vec.sort_by(|a, b| a.id.cmp(&b.id));
          app.data.lidarr_data.history.set_items(history_vec);
          app.data.lidarr_data.history.apply_sorting_toggle(false);
        }
      })
      .await
  }

  pub(in crate::network::lidarr_network) async fn mark_lidarr_history_item_as_failed(
    &mut self,
    history_item_id: i64,
  ) -> Result<Value> {
    info!("Marking the Lidarr history item with ID: {history_item_id} as 'failed'");
    let event = LidarrEvent::MarkHistoryItemAsFailed(history_item_id);

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
