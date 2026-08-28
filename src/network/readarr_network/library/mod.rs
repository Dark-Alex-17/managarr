use crate::models::servarr_data::Notification;
use crate::models::servarr_models::ReleaseDownloadBody;
use crate::network::readarr_network::ReadarrEvent;
use crate::network::{Network, RequestMethod};
use anyhow::Result;
use log::info;
use serde_json::Value;

mod authors;
mod books;

#[cfg(test)]
#[path = "readarr_library_network_tests.rs"]
mod readarr_library_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::readarr_network) async fn download_readarr_release(
    &mut self,
    release_download_body: ReleaseDownloadBody,
  ) -> Result<Value> {
    let event = ReadarrEvent::DownloadRelease(ReleaseDownloadBody::default());
    info!("Downloading Readarr release with params: {release_download_body:?}");

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Post,
        Some(release_download_body),
        None,
        None,
      )
      .await;

    let result = self
      .handle_request::<ReleaseDownloadBody, Value>(request_props, |_, mut app| {
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
