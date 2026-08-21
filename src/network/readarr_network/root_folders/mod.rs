use crate::models::servarr_models::RootFolder;
use crate::network::readarr_network::ReadarrEvent;
use crate::network::{Network, RequestMethod};
use anyhow::Result;
use log::info;

#[cfg(test)]
#[path = "readarr_root_folders_network_tests.rs"]
mod readarr_root_folders_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::readarr_network) async fn get_readarr_root_folders(
    &mut self,
  ) -> Result<Vec<RootFolder>> {
    info!("Fetching Readarr root folders");
    let event = ReadarrEvent::GetRootFolders;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<RootFolder>>(request_props, |root_folders, mut app| {
        app.data.readarr_data.root_folders.set_items(root_folders);
      })
      .await
  }
}
