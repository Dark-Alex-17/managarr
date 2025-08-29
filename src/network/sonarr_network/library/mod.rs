use crate::models::sonarr_models::SonarrReleaseDownloadBody;
use crate::network::sonarr_network::SonarrEvent;
use crate::network::{Network, RequestMethod};
use anyhow::Result;
use log::info;
use serde_json::Value;

mod episodes;
mod seasons;
mod series;

#[cfg(test)]
#[path = "sonarr_library_network_tests.rs"]
mod sonarr_library_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::sonarr_network) async fn download_sonarr_release(
    &mut self,
    sonarr_release_download_body: SonarrReleaseDownloadBody,
  ) -> Result<Value> {
    let event = SonarrEvent::DownloadRelease(SonarrReleaseDownloadBody::default());
    info!("Downloading Sonarr release with params: {sonarr_release_download_body:?}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Post,
        Some(sonarr_release_download_body),
        None,
        None,
      )
      .await;

    self
      .handle_request::<SonarrReleaseDownloadBody, Value>(request_props, |_, _| ())
      .await
  }
}
