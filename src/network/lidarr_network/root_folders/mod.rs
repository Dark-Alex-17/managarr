use crate::models::lidarr_models::AddLidarrRootFolderBody;
use crate::models::servarr_models::RootFolder;
use crate::network::lidarr_network::LidarrEvent;
use crate::network::{Network, RequestMethod};
use anyhow::Result;
use log::{debug, info};
use serde_json::Value;

#[cfg(test)]
#[path = "lidarr_root_folders_network_tests.rs"]
mod lidarr_root_folders_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::lidarr_network) async fn add_lidarr_root_folder(
    &mut self,
    mut add_root_folder_body: AddLidarrRootFolderBody,
  ) -> Result<Value> {
    info!("Adding new root folder to Lidarr");
    if let Some(tag_input_str) = add_root_folder_body.tag_input_string.as_ref() {
      let tag_ids_vec = self.extract_and_add_lidarr_tag_ids_vec(tag_input_str).await;
      add_root_folder_body.default_tags = tag_ids_vec;
    }
    let event = LidarrEvent::AddRootFolder(AddLidarrRootFolderBody::default());

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
      .handle_request::<AddLidarrRootFolderBody, Value>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::lidarr_network) async fn delete_lidarr_root_folder(
    &mut self,
    root_folder_id: i64,
  ) -> Result<()> {
    let event = LidarrEvent::DeleteRootFolder(root_folder_id);
    info!("Deleting Lidarr root folder for folder with id: {root_folder_id}");

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
