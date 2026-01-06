use anyhow::Result;
use log::{debug, info, warn};
use serde_json::{Value, json};

use crate::models::Route;
use crate::models::lidarr_models::{Artist, DeleteArtistParams};
use crate::models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock;
use crate::network::lidarr_network::LidarrEvent;
use crate::network::{Network, RequestMethod};

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
}
