use anyhow::Result;
use log::info;

use crate::models::Route;
use crate::models::readarr_models::{BlocklistItem, BlocklistResponse};
use crate::models::servarr_data::readarr::readarr_data::ActiveReadarrBlock;
use crate::network::readarr_network::ReadarrEvent;
use crate::network::{Network, RequestMethod};

#[cfg(test)]
#[path = "readarr_blocklist_network_tests.rs"]
mod readarr_blocklist_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::readarr_network) async fn get_readarr_blocklist(
    &mut self,
  ) -> Result<BlocklistResponse> {
    info!("Fetching Readarr blocklist");
    let event = ReadarrEvent::GetBlocklist;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), BlocklistResponse>(request_props, |blocklist_resp, mut app| {
        if !matches!(
          app.get_current_route(),
          Route::Readarr(ActiveReadarrBlock::BlocklistSortPrompt, _)
        ) {
          let mut blocklist_vec: Vec<BlocklistItem> = blocklist_resp.records;
          blocklist_vec.sort_by_key(|item| item.id);
          app.data.readarr_data.blocklist.set_items(blocklist_vec);
          app.data.readarr_data.blocklist.apply_sorting_toggle(false);
        }
      })
      .await
  }
}
