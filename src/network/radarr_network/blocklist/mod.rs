use crate::models::Route;
use crate::models::radarr_models::BlocklistResponse;
use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
use crate::network::radarr_network::RadarrEvent;
use crate::network::{Network, RequestMethod};
use anyhow::Result;
use log::info;
use serde_json::{Value, json};

#[cfg(test)]
#[path = "radarr_blocklist_network_tests.rs"]
mod radarr_blocklist_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::radarr_network) async fn clear_radarr_blocklist(&mut self) -> Result<()> {
    info!("Clearing Radarr blocklist");
    let event = RadarrEvent::ClearBlocklist;

    let ids = self
      .app
      .lock()
      .await
      .data
      .radarr_data
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

  pub(in crate::network::radarr_network) async fn delete_radarr_blocklist_item(
    &mut self,
    blocklist_item_id: i64,
  ) -> Result<()> {
    let event = RadarrEvent::DeleteBlocklistItem(blocklist_item_id);

    info!("Deleting Radarr blocklist item for item with id: {blocklist_item_id}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Delete,
        None::<()>,
        Some(format!("/{blocklist_item_id}")),
        None,
      )
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::radarr_network) async fn get_radarr_blocklist(
    &mut self,
  ) -> Result<BlocklistResponse> {
    info!("Fetching Radarr blocklist");
    let event = RadarrEvent::GetBlocklist;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), BlocklistResponse>(request_props, |blocklist_resp, mut app| {
        if !matches!(
          app.get_current_route(),
          Route::Radarr(ActiveRadarrBlock::BlocklistSortPrompt, _)
        ) {
          let mut blocklist_vec = blocklist_resp.records;
          blocklist_vec.sort_by(|a, b| a.id.cmp(&b.id));
          app.data.radarr_data.blocklist.set_items(blocklist_vec);
          app.data.radarr_data.blocklist.apply_sorting_toggle(false);
        }
      })
      .await
  }
}
