use crate::models::servarr_models::Indexer;
use crate::network::readarr_network::ReadarrEvent;
use crate::network::{Network, RequestMethod};
use anyhow::Result;
use log::info;

#[cfg(test)]
#[path = "readarr_indexers_network_tests.rs"]
mod readarr_indexers_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::readarr_network) async fn get_readarr_indexers(
    &mut self,
  ) -> Result<Vec<Indexer>> {
    info!("Fetching Readarr indexers");
    let event = ReadarrEvent::GetIndexers;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<Indexer>>(request_props, |indexers, mut app| {
        app.data.readarr_data.indexers.set_items(indexers);
      })
      .await
  }
}
