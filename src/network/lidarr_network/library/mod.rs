use crate::models::lidarr_models::LidarrReleaseDownloadBody;
use crate::models::servarr_data::Notification;
use crate::network::lidarr_network::LidarrEvent;
use crate::network::{Network, RequestMethod};
use anyhow::Result;
use log::info;
use serde_json::Value;

mod albums;
mod artists;
mod tracks;

#[cfg(test)]
#[path = "lidarr_library_network_tests.rs"]
mod lidarr_library_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::lidarr_network) async fn download_lidarr_release(
    &mut self,
    lidarr_release_download_body: LidarrReleaseDownloadBody,
  ) -> Result<Value> {
    let event = LidarrEvent::DownloadRelease(LidarrReleaseDownloadBody::default());
    info!("Downloading Lidarr release with params: {lidarr_release_download_body:?}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Post,
        Some(lidarr_release_download_body),
        None,
        None,
      )
      .await;

    let result = self
      .handle_request::<LidarrReleaseDownloadBody, Value>(request_props, |_, mut app| {
        app.notification = Some(Notification::new(
          "Download Result".to_owned(),
          "Download request sent successfully".to_owned(),
          true,
        ));
      })
      .await;

    if result.is_err() {
      let mut app = self.app.lock().await;
      std::mem::take(&mut app.error.text);
      app.notification = Some(Notification::new(
        "Download Failed".to_owned(),
        "Download request failed. Check the logs for more details.".to_owned(),
        false,
      ));
    }

    result
  }
}
