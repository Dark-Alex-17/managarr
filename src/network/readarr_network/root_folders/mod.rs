use crate::models::readarr_models::AddReadarrRootFolderBody;
use crate::models::servarr_models::RootFolder;
use crate::network::readarr_network::ReadarrEvent;
use crate::network::{Network, RequestMethod};
use anyhow::Result;
use log::{debug, info};
use serde_json::Value;

#[cfg(test)]
#[path = "readarr_root_folders_network_tests.rs"]
mod readarr_root_folders_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::readarr_network) async fn add_readarr_root_folder(
    &mut self,
    mut add_root_folder_body: AddReadarrRootFolderBody,
  ) -> Result<Value> {
    info!("Adding new root folder to Readarr");
    if let Some(tag_input_str) = add_root_folder_body.tag_input_string.as_ref() {
      let tag_ids_vec = self
        .extract_and_add_readarr_tag_ids_vec(tag_input_str)
        .await;
      add_root_folder_body.default_tags = tag_ids_vec;
    }
    let event = ReadarrEvent::AddRootFolder(AddReadarrRootFolderBody::default());

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
      .handle_request::<AddReadarrRootFolderBody, Value>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::readarr_network) async fn delete_readarr_root_folder(
    &mut self,
    root_folder_id: i64,
  ) -> Result<()> {
    info!("Deleting Readarr root folder with ID: {root_folder_id}");
    let event = ReadarrEvent::DeleteRootFolder(root_folder_id);

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
