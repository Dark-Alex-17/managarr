use anyhow::Result;
use log::info;

use crate::models::Route;
use crate::models::readarr_models::ReadarrHistoryWrapper;
use crate::models::servarr_data::readarr::readarr_data::ActiveReadarrBlock;
use crate::network::readarr_network::ReadarrEvent;
use crate::network::{Network, RequestMethod};

#[cfg(test)]
#[path = "readarr_history_network_tests.rs"]
mod readarr_history_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::readarr_network) async fn get_readarr_history(
    &mut self,
    events: u64,
  ) -> Result<ReadarrHistoryWrapper> {
    info!("Fetching all Readarr history events");
    let event = ReadarrEvent::GetHistory(events);

    let params = format!("pageSize={events}&sortDirection=descending&sortKey=date");
    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, Some(params))
      .await;

    self
      .handle_request::<(), ReadarrHistoryWrapper>(request_props, |history_response, mut app| {
        if !matches!(
          app.get_current_route(),
          Route::Readarr(ActiveReadarrBlock::HistorySortPrompt, _)
        ) {
          let mut history_vec = history_response.records;
          history_vec.sort_by_key(|item| item.id);
          app.data.readarr_data.history.set_items(history_vec);
          app.data.readarr_data.history.apply_sorting_toggle(false);
        }
      })
      .await
  }
}
