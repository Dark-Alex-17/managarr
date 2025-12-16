use crate::models::Route;
use crate::models::radarr_models::{Collection, EditCollectionParams};
use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
use crate::models::servarr_models::CommandBody;
use crate::network::radarr_network::RadarrEvent;
use crate::network::{Network, RequestMethod};
use anyhow::Result;
use log::{debug, info};
use serde_json::{Value, json};

#[cfg(test)]
#[path = "radarr_collections_network_tests.rs"]
mod radarr_collections_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::radarr_network) async fn edit_collection(
    &mut self,
    edit_collection_params: EditCollectionParams,
  ) -> Result<()> {
    info!("Editing Radarr collection");
    let detail_event = RadarrEvent::GetCollections;
    let event = RadarrEvent::EditCollection(EditCollectionParams::default());
    info!("Fetching collection details");
    let collection_id = edit_collection_params.collection_id;

    let request_props = self
      .request_props_from(
        detail_event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{collection_id}")),
        None,
      )
      .await;

    let mut response = String::new();

    self
      .handle_request::<(), Value>(request_props, |detailed_collection_body, _| {
        response = detailed_collection_body.to_string()
      })
      .await?;

    info!("Constructing edit collection body");

    let mut detailed_collection_body: Value = serde_json::from_str(&response)?;
    let (monitored, minimum_availability, quality_profile_id, root_folder_path, search_on_add) = {
      let monitored = edit_collection_params.monitored.unwrap_or_else(|| {
        detailed_collection_body["monitored"]
          .as_bool()
          .expect("Unable to deserialize 'monitored' bool")
      });
      let minimum_availability = edit_collection_params
        .minimum_availability
        .unwrap_or_else(|| {
          serde_json::from_value(detailed_collection_body["minimumAvailability"].clone())
            .expect("Unable to deserialize 'minimumAvailability'")
        })
        .to_string();
      let quality_profile_id = edit_collection_params
        .quality_profile_id
        .unwrap_or_else(|| {
          detailed_collection_body["qualityProfileId"]
            .as_i64()
            .expect("Unable to deserialize 'qualityProfileId'")
        });
      let root_folder_path = edit_collection_params.root_folder_path.unwrap_or_else(|| {
        detailed_collection_body["rootFolderPath"]
          .as_str()
          .expect("Unable to deserialize 'rootFolderPath'")
          .to_owned()
      });
      let search_on_add = edit_collection_params.search_on_add.unwrap_or_else(|| {
        detailed_collection_body["searchOnAdd"]
          .as_bool()
          .expect("Unable to deserialize 'searchOnAdd'")
      });

      (
        monitored,
        minimum_availability,
        quality_profile_id,
        root_folder_path,
        search_on_add,
      )
    };

    *detailed_collection_body.get_mut("monitored").unwrap() = json!(monitored);
    *detailed_collection_body
      .get_mut("minimumAvailability")
      .unwrap() = json!(minimum_availability);
    *detailed_collection_body
      .get_mut("qualityProfileId")
      .unwrap() = json!(quality_profile_id);
    *detailed_collection_body.get_mut("rootFolderPath").unwrap() = json!(root_folder_path);
    *detailed_collection_body.get_mut("searchOnAdd").unwrap() = json!(search_on_add);

    debug!("Edit collection body: {detailed_collection_body:?}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Put,
        Some(detailed_collection_body),
        Some(format!("/{collection_id}")),
        None,
      )
      .await;

    self
      .handle_request::<Value, ()>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::radarr_network) async fn get_collections(
    &mut self,
  ) -> Result<Vec<Collection>> {
    info!("Fetching Radarr collections");
    let event = RadarrEvent::GetCollections;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<Collection>>(request_props, |mut collections_vec, mut app| {
        if !matches!(
          app.get_current_route(),
          Route::Radarr(ActiveRadarrBlock::CollectionsSortPrompt, _)
        ) {
          collections_vec.sort_by(|a, b| a.id.cmp(&b.id));
          app.data.radarr_data.collections.set_items(collections_vec);
          app.data.radarr_data.collections.apply_sorting_toggle(false);
        }
      })
      .await
  }

  pub(in crate::network::radarr_network) async fn update_collections(&mut self) -> Result<Value> {
    info!("Updating collections");
    let event = RadarrEvent::UpdateCollections;
    let body = CommandBody {
      name: "RefreshCollections".to_owned(),
    };

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(body), None, None)
      .await;

    self
      .handle_request::<CommandBody, Value>(request_props, |_, _| ())
      .await
  }
}
