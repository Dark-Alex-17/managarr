use anyhow::Result;
use log::info;

use crate::models::lidarr_models::{Artist, DeleteArtistParams};
use crate::models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock;
use crate::models::Route;
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
}
