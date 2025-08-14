use crate::models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock;
use crate::models::sonarr_models::{BlocklistItem, BlocklistResponse};
use crate::models::Route;
use crate::network::sonarr_network::SonarrEvent;
use crate::network::{Network, RequestMethod};
use anyhow::Result;
use log::info;
use serde_json::{json, Value};

#[cfg(test)]
#[path = "sonarr_blocklist_network_tests.rs"]
mod sonarr_blocklist_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::sonarr_network) async fn clear_sonarr_blocklist(&mut self) -> Result<()> {
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

  pub(in crate::network::sonarr_network) async fn delete_sonarr_blocklist_item(
    &mut self,
    blocklist_item_id: i64,
  ) -> Result<()> {
    let event = SonarrEvent::DeleteBlocklistItem(blocklist_item_id);
    info!("Deleting Sonarr blocklist item for item with id: {blocklist_item_id}");

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

  pub(in crate::network::sonarr_network) async fn get_sonarr_blocklist(
    &mut self,
  ) -> Result<BlocklistResponse> {
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
          let mut blocklist_vec: Vec<BlocklistItem> = blocklist_resp
            .records
            .into_iter()
            .map(|item| {
              if let Some(series) = app
                .data
                .sonarr_data
                .series
                .items
                .iter()
                .find(|it| it.id == item.series_id)
              {
                BlocklistItem {
                  series_title: Some(series.title.text.clone()),
                  ..item
                }
              } else {
                item
              }
            })
            .collect();
          blocklist_vec.sort_by(|a, b| a.id.cmp(&b.id));
          app.data.sonarr_data.blocklist.set_items(blocklist_vec);
          app.data.sonarr_data.blocklist.apply_sorting_toggle(false);
        }
      })
      .await
  }
}
