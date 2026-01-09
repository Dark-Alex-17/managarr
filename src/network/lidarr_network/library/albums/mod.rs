use crate::models::lidarr_models::{Album, DeleteParams};
use crate::network::lidarr_network::LidarrEvent;
use crate::network::{Network, RequestMethod};
use anyhow::Result;
use log::{debug, info, warn};
use serde_json::{Value, json};

#[cfg(test)]
#[path = "lidarr_albums_network_tests.rs"]
mod lidarr_albums_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::lidarr_network) async fn get_albums(
    &mut self,
    artist_id: i64,
  ) -> Result<Vec<Album>> {
    info!("Fetching albums for Lidarr artist with ID: {artist_id}");
    let event = LidarrEvent::GetAlbums(artist_id);

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
      .handle_request::<(), Vec<Album>>(request_props, |mut albums_vec, mut app| {
        albums_vec.sort_by(|a, b| a.id.cmp(&b.id));
        app.data.lidarr_data.albums.set_items(albums_vec);
      })
      .await
  }

  pub(in crate::network::lidarr_network) async fn get_album_details(
    &mut self,
    album_id: i64,
  ) -> Result<Album> {
    info!("Fetching details for Lidarr album with ID: {album_id}");
    let event = LidarrEvent::GetAlbumDetails(album_id);

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{album_id}")),
        None,
      )
      .await;

    self
      .handle_request::<(), Album>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::lidarr_network) async fn delete_album(
    &mut self,
    delete_album_params: DeleteParams,
  ) -> Result<()> {
    let event = LidarrEvent::DeleteAlbum(DeleteParams::default());
    let DeleteParams {
      id,
      delete_files,
      add_import_list_exclusion,
    } = delete_album_params;

    info!(
      "Deleting Lidarr album with ID: {id} with deleteFiles={delete_files} and addImportListExclusion={add_import_list_exclusion}"
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

  pub(in crate::network::lidarr_network) async fn toggle_album_monitoring(
    &mut self,
    album_id: i64,
  ) -> Result<()> {
    let event = LidarrEvent::ToggleAlbumMonitoring(album_id);
    info!("Toggling album monitoring for album with ID: {album_id}");
    info!("Fetching album details for album with ID: {album_id}");

    let detail_event = LidarrEvent::GetAlbums(0);
    let request_props = self
      .request_props_from(
        detail_event,
        RequestMethod::Get,
        None::<()>,
        Some(format!("/{album_id}")),
        None,
      )
      .await;

    let mut response = String::new();

    self
      .handle_request::<(), Value>(request_props, |detailed_album_body, _| {
        response = detailed_album_body.to_string()
      })
      .await?;

    info!("Constructing toggle album monitoring body");

    match serde_json::from_str::<Value>(&response) {
      Ok(mut detailed_album_body) => {
        let monitored = detailed_album_body
          .get("monitored")
          .unwrap()
          .as_bool()
          .unwrap();

        *detailed_album_body.get_mut("monitored").unwrap() = json!(!monitored);

        debug!("Toggle album monitoring body: {detailed_album_body:?}");

        let request_props = self
          .request_props_from(
            event,
            RequestMethod::Put,
            Some(detailed_album_body),
            Some(format!("/{album_id}")),
            None,
          )
          .await;

        self
          .handle_request::<Value, ()>(request_props, |_, _| ())
          .await
      }
      Err(_) => {
        warn!("Request for detailed album body was interrupted");
        Ok(())
      }
    }
  }
}
