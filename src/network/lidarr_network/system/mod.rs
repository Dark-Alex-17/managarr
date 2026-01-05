use anyhow::Result;
use log::info;

use crate::models::lidarr_models::{DownloadsResponse, MetadataProfile, SystemStatus};
use crate::models::servarr_models::{DiskSpace, QualityProfile, RootFolder, Tag};
use crate::network::lidarr_network::LidarrEvent;
use crate::network::{Network, RequestMethod};

#[cfg(test)]
#[path = "lidarr_system_network_tests.rs"]
mod lidarr_system_network_tests;

impl Network<'_, '_> {
  pub(in crate::network::lidarr_network) async fn get_lidarr_healthcheck(&mut self) -> Result<()> {
    info!("Performing Lidarr health check");
    let event = LidarrEvent::HealthCheck;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), ()>(request_props, |_, _| ())
      .await
  }

  pub(in crate::network::lidarr_network) async fn get_lidarr_metadata_profiles(
    &mut self,
  ) -> Result<Vec<MetadataProfile>> {
    info!("Fetching Lidarr metadata profiles");
    let event = LidarrEvent::GetMetadataProfiles;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<MetadataProfile>>(request_props, |metadata_profiles, mut app| {
        app.data.lidarr_data.metadata_profile_map = metadata_profiles
          .into_iter()
          .map(|profile| (profile.id, profile.name))
          .collect();
      })
      .await
  }

  pub(in crate::network::lidarr_network) async fn get_lidarr_quality_profiles(
    &mut self,
  ) -> Result<Vec<QualityProfile>> {
    info!("Fetching Lidarr quality profiles");
    let event = LidarrEvent::GetQualityProfiles;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<QualityProfile>>(request_props, |quality_profiles, mut app| {
        app.data.lidarr_data.quality_profile_map = quality_profiles
          .into_iter()
          .map(|profile| (profile.id, profile.name))
          .collect();
      })
      .await
  }

  pub(in crate::network::lidarr_network) async fn get_lidarr_tags(&mut self) -> Result<Vec<Tag>> {
    info!("Fetching Lidarr tags");
    let event = LidarrEvent::GetTags;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<Tag>>(request_props, |tags_vec, mut app| {
        app.data.lidarr_data.tags_map = tags_vec
          .into_iter()
          .map(|tag| (tag.id, tag.label))
          .collect();
      })
      .await
  }

  pub(in crate::network::lidarr_network) async fn get_lidarr_diskspace(
    &mut self,
  ) -> Result<Vec<DiskSpace>> {
    info!("Fetching Lidarr disk space");
    let event = LidarrEvent::GetDiskSpace;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<DiskSpace>>(request_props, |disk_space_vec, mut app| {
        app.data.lidarr_data.disk_space_vec = disk_space_vec;
      })
      .await
  }

  pub(in crate::network::lidarr_network) async fn get_lidarr_downloads(
    &mut self,
    count: u64,
  ) -> Result<DownloadsResponse> {
    info!("Fetching Lidarr downloads");
    let event = LidarrEvent::GetDownloads(count);

    let request_props = self
      .request_props_from(
        event,
        RequestMethod::Get,
        None::<()>,
        None,
        Some(format!("pageSize={count}")),
      )
      .await;

    self
      .handle_request::<(), DownloadsResponse>(request_props, |queue_response, mut app| {
        app
          .data
          .lidarr_data
          .downloads
          .set_items(queue_response.records);
      })
      .await
  }

  pub(in crate::network::lidarr_network) async fn get_lidarr_root_folders(
    &mut self,
  ) -> Result<Vec<RootFolder>> {
    info!("Fetching Lidarr root folders");
    let event = LidarrEvent::GetRootFolders;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), Vec<RootFolder>>(request_props, |root_folders, mut app| {
        app.data.lidarr_data.root_folders.set_items(root_folders);
      })
      .await
  }

  pub(in crate::network::lidarr_network) async fn get_lidarr_status(
    &mut self,
  ) -> Result<SystemStatus> {
    info!("Fetching Lidarr system status");
    let event = LidarrEvent::GetStatus;

    let request_props = self
      .request_props_from(event, RequestMethod::Get, None::<()>, None, None)
      .await;

    self
      .handle_request::<(), SystemStatus>(request_props, |system_status, mut app| {
        app.data.lidarr_data.version = system_status.version;
        app.data.lidarr_data.start_time = system_status.start_time;
      })
      .await
  }
}
