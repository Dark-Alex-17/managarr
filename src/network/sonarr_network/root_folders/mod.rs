use crate::models::servarr_models::{AddRootFolderBody, RootFolder};
use crate::network::sonarr_network::SonarrEvent;
use crate::network::{Network, RequestMethod};
use anyhow::Result;
use log::{debug, info};
use serde_json::Value;

#[cfg(test)]
#[path = "sonarr_root_folders_network_tests.rs"]
mod sonarr_root_folders_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::sonarr_network) async fn add_sonarr_root_folder(
    &mut self,
    add_root_folder_body: AddRootFolderBody,
  ) -> Result<Value> {
    info!("Adding new root folder to Sonarr");
    let event = SonarrEvent::AddRootFolder(AddRootFolderBody::default());

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

  pub(in crate::network::sonarr_network) async fn delete_sonarr_root_folder(
    &mut self,
    root_folder_id: i64,
  ) -> Result<()> {
    let event = SonarrEvent::DeleteRootFolder(root_folder_id);
    info!("Deleting Sonarr root folder for folder with id: {root_folder_id}");

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

  pub(in crate::network::sonarr_network) async fn get_sonarr_root_folders(
    &mut self,
  ) -> Result<Vec<RootFolder>> {
    info!("Fetching Sonarr root folders");
    let event = SonarrEvent::GetRootFolders;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<RootFolder>>(request_props, |root_folders, mut app| {
        app.data.sonarr_data.root_folders.set_items(root_folders);
      })
      .await
  }
}
