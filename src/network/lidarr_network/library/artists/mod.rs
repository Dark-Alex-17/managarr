use anyhow::Result;
use log::{debug, info, warn};
use serde_json::{Value, json};

use crate::models::Route;
use crate::models::lidarr_models::{
  AddArtistBody, AddArtistSearchResult, Artist, DeleteParams, EditArtistParams, LidarrCommandBody,
  LidarrHistoryItem, LidarrRelease,
};
use crate::models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock;
use crate::models::stateful_table::StatefulTable;
use crate::network::lidarr_network::LidarrEvent;
use crate::network::{Network, RequestMethod};
use urlencoding::encode;

#[cfg(test)]
#[path = "lidarr_artists_network_tests.rs"]
mod lidarr_artists_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::lidarr_network) async fn delete_artist(
    &mut self,
    delete_artist_params: DeleteParams,
  ) -> Result<()> {
    let event = LidarrEvent::DeleteArtist(DeleteParams::default());
    let DeleteParams {
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
    let body = LidarrCommandBody {
      name: "RefreshArtist".to_owned(),
      ..LidarrCommandBody::default()
    };

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(body), None, None)
      .await;

    self
      .handle_request::<LidarrCommandBody, Value>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::lidarr_network) async fn update_and_scan_artist(
    &mut self,
    artist_id: i64,
  ) -> Result<Value> {
    let event = LidarrEvent::UpdateAndScanArtist(artist_id);
    info!("Updating and scanning artist with ID: {artist_id}");
    let body = LidarrCommandBody {
      name: "RefreshArtist".to_owned(),
      artist_id: Some(artist_id),
      ..LidarrCommandBody::default()
    };

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(body), None, None)
      .await;

    self
      .handle_request::<LidarrCommandBody, Value>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::lidarr_network) async fn trigger_automatic_artist_search(
    &mut self,
    artist_id: i64,
  ) -> Result<Value> {
    let event = LidarrEvent::TriggerAutomaticArtistSearch(artist_id);
    info!("Searching indexers for artist with ID: {artist_id}");
    let body = LidarrCommandBody {
      name: "ArtistSearch".to_owned(),
      artist_id: Some(artist_id),
      ..LidarrCommandBody::default()
    };

    let request_props = self
      .request_props_from(event, RequestMethod::Post, Some(body), None, None)
      .await;

    self
      .handle_request::<LidarrCommandBody, Value>(request_props, |_, _| ())
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

  pub(in crate::network::lidarr_network) async fn add_artist(
    &mut self,
    mut add_artist_body: AddArtistBody,
  ) -> Result<Value> {
    info!("Adding Lidarr artist: {}", add_artist_body.artist_name);
    if let Some(tag_input_str) = add_artist_body.tag_input_string.as_ref() {
      let tag_ids_vec = self.extract_and_add_lidarr_tag_ids_vec(tag_input_str).await;
      add_artist_body.tags = tag_ids_vec;
    }
    let event = LidarrEvent::AddArtist(AddArtistBody::default());

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Post,
        Some(add_artist_body),
        None,
        None,
      )
      .await;

    self
      .handle_request::<AddArtistBody, Value>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::lidarr_network) async fn get_lidarr_artist_history(
    &mut self,
    artist_id: i64,
  ) -> Result<Vec<LidarrHistoryItem>> {
    info!("Fetching Lidarr artist history for artist with ID: {artist_id}");
    let event = LidarrEvent::GetArtistHistory(artist_id);

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("artistId={artist_id}")),
      )
      .await;

    self
      .handle_request::<(), Vec<LidarrHistoryItem>>(request_props, |mut history_vec, mut app| {
        let is_sorting = matches!(
          app.get_current_route(),
          Route::Lidarr(ActiveLidarrBlock::ArtistHistorySortPrompt, _)
        );

        let artist_history = &mut app.data.lidarr_data.artist_history;

        if !is_sorting {
          history_vec.sort_by(|a, b| a.id.cmp(&b.id));
          artist_history.set_items(history_vec);
          artist_history.apply_sorting_toggle(false);
        }
      })
      .await
  }

  pub(in crate::network::lidarr_network) async fn get_artist_discography_releases(
    &mut self,
    artist_id: i64,
  ) -> Result<Vec<LidarrRelease>> {
    let event = LidarrEvent::GetDiscographyReleases(artist_id);
    info!("Fetching discography releases for artist with ID: {artist_id}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("artistId={artist_id}")),
      )
      .await;

    self
      .handle_request::<(), Vec<LidarrRelease>>(request_props, |release_vec, mut app| {
        let artist_releases_vec = release_vec
          .into_iter()
          .filter(|release| release.discography)
          .collect();

        app
          .data
          .lidarr_data
          .discography_releases
          .set_items(artist_releases_vec);
      })
      .await
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
