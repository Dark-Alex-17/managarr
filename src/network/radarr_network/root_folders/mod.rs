use crate::models::servarr_models::{AddRootFolderBody, RootFolder};
use crate::network::radarr_network::RadarrEvent;
use crate::network::{Network, RequestMethod};
use anyhow::Result;
use log::{debug, info};
use serde_json::Value;

#[cfg(test)]
#[path = "radarr_root_folders_network_tests.rs"]
mod radarr_root_folders_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::radarr_network) async fn add_radarr_root_folder(
    &mut self,
    add_root_folder_body: AddRootFolderBody,
  ) -> Result<Value> {
    info!("Adding new root folder to Radarr");
    let event = RadarrEvent::AddRootFolder(AddRootFolderBody::default());

    debug!("Add root folder body: {add_root_folder_body:?}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Post,
        Some(add_root_folder_body),
        None,
        None,
      )
      .await;

    self
      .handle_request::<AddRootFolderBody, Value>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::radarr_network) async fn delete_radarr_root_folder(
    &mut self,
    root_folder_id: i64,
  ) -> Result<()> {
    let event = RadarrEvent::DeleteRootFolder(root_folder_id);
    info!("Deleting Radarr root folder for folder with id: {root_folder_id}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Delete,
        None::<()>,
        Some(format!("/{root_folder_id}")),
        None,
      )
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::radarr_network) async fn get_radarr_root_folders(
    &mut self,
  ) -> Result<Vec<RootFolder>> {
    info!("Fetching Radarr root folders");
    let event = RadarrEvent::GetRootFolders;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<RootFolder>>(request_props, |root_folders, mut app| {
        app.data.radarr_data.root_folders.set_items(root_folders);
      })
      .await
  }
}
