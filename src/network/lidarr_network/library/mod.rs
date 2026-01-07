use anyhow::Result;
use log::{debug, info, warn};
use serde_json::{Value, json};

use crate::models::Route;
use crate::models::lidarr_models::{
  AddArtistSearchResult, Artist, DeleteArtistParams, EditArtistParams,
};
use crate::models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock;
use crate::models::servarr_models::CommandBody;
use crate::models::stateful_table::StatefulTable;
use crate::network::lidarr_network::LidarrEvent;
use crate::network::{Network, RequestMethod};
use urlencoding::encode;

#[cfg(test)]
#[path = "lidarr_library_network_tests.rs"]
mod lidarr_library_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::lidarr_network) async fn delete_artist(
    &mut self,
    delete_artist_params: DeleteArtistParams,
  ) -> Result<()> {
    let event = LidarrEvent::DeleteArtist(DeleteArtistParams::default());
    let DeleteArtistParams {
      id,
      delete_files,
      add_import_list_exclusion,
    } = delete_artist_params;

    info!(
      "Deleting Lidarr artist with ID: {id} with deleteFiles={delete_files} and addImportListExclusion={add_import_list_exclusion}"
    );

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Delete,
        None::<()>,
        Some(format!("/{id}")),
        Some(format!(
          "deleteFiles={delete_files}&addImportListExclusion={add_import_list_exclusion}"
        )),
      )
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::lidarr_network) async fn list_artists(&mut self) -> Result<Vec<Artist>> {
    info!("Fetching Lidarr artists");
    let event = LidarrEvent::ListArtists;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<Artist>>(request_props, |mut artists_vec, mut app| {
        if !matches!(
          app.get_current_route(),
          Route::Lidarr(ActiveLidarrBlock::ArtistsSortPrompt, _)
        ) {
          artists_vec.sort_by(|a, b| a.id.cmp(&b.id));
          app.data.lidarr_data.artists.set_items(artists_vec);
          app.data.lidarr_data.artists.apply_sorting_toggle(false);
        }
      })
      .await
  }

  pub(in crate::network::lidarr_network) async fn get_artist_details(
    &mut self,
    artist_id: i64,
  ) -> Result<Artist> {
    info!("Fetching details for Lidarr artist with ID: {artist_id}");
    let event = LidarrEvent::GetArtistDetails(artist_id);

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{artist_id}")),
        None,
      )
      .await;

    self
      .handle_request::<(), Artist>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::lidarr_network) async fn toggle_artist_monitoring(
    &mut self,
    artist_id: i64,
  ) -> Result<()> {
    let event = LidarrEvent::ToggleArtistMonitoring(artist_id);

    let detail_event = LidarrEvent::GetArtistDetails(artist_id);
    info!("Toggling artist monitoring for artist with ID: {artist_id}");
    info!("Fetching artist details for artist with ID: {artist_id}");

    let request_props = self
      .request_props_from(
        detail_event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{artist_id}")),
        None,
      )
      .await;

    let mut response = String::new();

    self
      .handle_request::<(), Value>(request_props, |detailed_artist_body, _| {
        response = detailed_artist_body.to_string()
      })
      .await?;

    info!("Constructing toggle artist monitoring body");

    match serde_json::from_str::<Value>(&response) {
      Ok(mut detailed_artist_body) => {
        let monitored = detailed_artist_body
          .get("monitored")
          .unwrap()
          .as_bool()
          .unwrap();

        *detailed_artist_body.get_mut("monitored").unwrap() = json!(!monitored);

        debug!("Toggle artist monitoring body: {detailed_artist_body:?}");

        let request_props = self
          .request_props_from(
            event,
            RequestMethod::Put,
            Some(detailed_artist_body),
            Some(format!("/{artist_id}")),
            None,
          )
          .await;

        self
          .handle_request::<Value, ()>(request_props, |_, _| ())
          .await
      }
      Err(_) => {
        warn!("Request for detailed artist body was interrupted");
        Ok(())
      }
    }
  }

  pub(in crate::network::lidarr_network) async fn update_all_artists(&mut self) -> Result<Value> {
    info!("Updating all artists");
    let event = LidarrEvent::UpdateAllArtists;
    let body = CommandBody {
      name: "RefreshArtist".to_owned(),
    };

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(body), None, None)
      .await;

    self
      .handle_request::<CommandBody, Value>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::lidarr_network) async fn search_artist(
    &mut self,
    query: String,
  ) -> Result<Vec<AddArtistSearchResult>> {
    info!("Searching for artist: {query}");
    let event = LidarrEvent::SearchNewArtist(String::new());

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("term={}", encode(&query))),
      )
      .await;

    let result = self
      .handle_request::<(), Vec<AddArtistSearchResult>>(request_props, |artist_vec, mut app| {
        if artist_vec.is_empty() {
          app.pop_and_push_navigation_stack(ActiveLidarrBlock::AddArtistEmptySearchResults.into());
        } else if let Some(add_searched_artists) =
          app.data.lidarr_data.add_searched_artists.as_mut()
        {
          add_searched_artists.set_items(artist_vec);
        } else {
          let mut add_searched_artists = StatefulTable::default();
          add_searched_artists.set_items(artist_vec);
          app.data.lidarr_data.add_searched_artists = Some(add_searched_artists);
        }
      })
      .await;

    if result.is_err() {
      self.app.lock().await.data.lidarr_data.add_searched_artists = Some(StatefulTable::default());
    }

    result
  }

  pub(in crate::network::lidarr_network) async fn edit_artist(
    &mut self,
    mut edit_artist_params: EditArtistParams,
  ) -> Result<()> {
    info!("Editing Lidarr artist");
    if let Some(tag_input_str) = edit_artist_params.tag_input_string.as_ref() {
      let tag_ids_vec = self.extract_and_add_lidarr_tag_ids_vec(tag_input_str).await;
      edit_artist_params.tags = Some(tag_ids_vec);
    }
    let artist_id = edit_artist_params.artist_id;
    let detail_event = LidarrEvent::GetArtistDetails(artist_id);
    let event = LidarrEvent::EditArtist(EditArtistParams::default());
    info!("Fetching artist details for artist with ID: {artist_id}");

    let request_props = self
      .request_props_from(
        detail_event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{artist_id}")),
        None,
      )
      .await;

    let mut response = String::new();

    self
      .handle_request::<(), Value>(request_props, |detailed_artist_body, _| {
        response = detailed_artist_body.to_string()
      })
      .await?;

    info!("Constructing edit artist body");

    let mut detailed_artist_body: Value = serde_json::from_str(&response)?;
    let (
      monitored,
      monitor_new_items,
      quality_profile_id,
      metadata_profile_id,
      root_folder_path,
      tags,
    ) = {
      let monitored = edit_artist_params.monitored.unwrap_or(
        detailed_artist_body["monitored"]
          .as_bool()
          .expect("Unable to deserialize 'monitored'"),
      );
      let monitor_new_items = edit_artist_params.monitor_new_items.unwrap_or_else(|| {
        serde_json::from_value(detailed_artist_body["monitorNewItems"].clone())
          .expect("Unable to deserialize 'monitorNewItems'")
      });
      let quality_profile_id = edit_artist_params.quality_profile_id.unwrap_or_else(|| {
        detailed_artist_body["qualityProfileId"]
          .as_i64()
          .expect("Unable to deserialize 'qualityProfileId'")
      });
      let metadata_profile_id = edit_artist_params.metadata_profile_id.unwrap_or_else(|| {
        detailed_artist_body["metadataProfileId"]
          .as_i64()
          .expect("Unable to deserialize 'metadataProfileId'")
      });
      let root_folder_path = edit_artist_params.root_folder_path.unwrap_or_else(|| {
        detailed_artist_body["path"]
          .as_str()
          .expect("Unable to deserialize 'path'")
          .to_owned()
      });
      let tags = if edit_artist_params.clear_tags {
        vec![]
      } else {
        edit_artist_params.tags.unwrap_or(
          detailed_artist_body["tags"]
            .as_array()
            .expect("Unable to deserialize 'tags'")
            .iter()
            .map(|item| item.as_i64().expect("Unable to deserialize tag ID"))
            .collect(),
        )
      };

      (
        monitored,
        monitor_new_items,
        quality_profile_id,
        metadata_profile_id,
        root_folder_path,
        tags,
      )
    };

    *detailed_artist_body.get_mut("monitored").unwrap() = json!(monitored);
    *detailed_artist_body.get_mut("monitorNewItems").unwrap() = json!(monitor_new_items);
    *detailed_artist_body.get_mut("qualityProfileId").unwrap() = json!(quality_profile_id);
    *detailed_artist_body.get_mut("metadataProfileId").unwrap() = json!(metadata_profile_id);
    *detailed_artist_body.get_mut("path").unwrap() = json!(root_folder_path);
    *detailed_artist_body.get_mut("tags").unwrap() = json!(tags);

    debug!("Edit artist body: {detailed_artist_body:?}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Put,
        Some(detailed_artist_body),
        Some(format!("/{artist_id}")),
        None,
      )
      .await;

    self
      .handle_request::<Value, ()>(request_props, |_, _| ())
      .await
  }
}
