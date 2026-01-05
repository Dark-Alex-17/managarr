use anyhow::Result;
use log::info;

use crate::models::servarr_models::RootFolder;
use crate::network::lidarr_network::LidarrEvent;
use crate::network::{Network, RequestMethod};

#[cfg(test)]
#[path = "lidarr_root_folders_network_tests.rs"]
mod lidarr_root_folders_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::lidarr_network) async fn get_lidarr_root_folders(
    &mut self,
  ) -> Result<Vec<RootFolder>> {
    info!("Fetching Lidarr root folders");
    let event = LidarrEvent::GetRootFolders;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<RootFolder>>(request_props, |root_folders, mut app| {
        app.data.lidarr_data.root_folders.set_items(root_folders);
      })
      .await
  }
}
