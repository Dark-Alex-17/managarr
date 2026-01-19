use crate::models::Route;
use crate::models::lidarr_models::{BlocklistItem, BlocklistResponse};
use crate::models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock;
use crate::network::lidarr_network::LidarrEvent;
use crate::network::{Network, RequestMethod};
use anyhow::Result;
use log::info;
use serde_json::{Value, json};

#[cfg(test)]
#[path = "lidarr_blocklist_network_tests.rs"]
mod lidarr_blocklist_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::lidarr_network) async fn clear_lidarr_blocklist(&mut self) -> Result<()> {
    info!("Clearing Lidarr blocklist");
    let event = LidarrEvent::ClearBlocklist;

    let ids = self
      .app
      .lock()
      .await
      .data
      .lidarr_data
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

  pub(in crate::network::lidarr_network) async fn delete_lidarr_blocklist_item(
    &mut self,
    blocklist_item_id: i64,
  ) -> Result<()> {
    let event = LidarrEvent::DeleteBlocklistItem(blocklist_item_id);
    info!("Deleting Lidarr blocklist item for item with id: {blocklist_item_id}");

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

  pub(in crate::network::lidarr_network) async fn get_lidarr_blocklist(
    &mut self,
  ) -> Result<BlocklistResponse> {
    info!("Fetching Lidarr blocklist");
    let event = LidarrEvent::GetBlocklist;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), BlocklistResponse>(request_props, |blocklist_resp, mut app| {
        if !matches!(
          app.get_current_route(),
          Route::Lidarr(ActiveLidarrBlock::BlocklistSortPrompt, _)
        ) {
          let mut blocklist_vec: Vec<BlocklistItem> = blocklist_resp.records;
          blocklist_vec.sort_by(|a, b| a.id.cmp(&b.id));
          app.data.lidarr_data.blocklist.set_items(blocklist_vec);
          app.data.lidarr_data.blocklist.apply_sorting_toggle(false);
        }
      })
      .await
  }
}
