use anyhow::Result;

use super::{NetworkEvent, NetworkResource};
use crate::models::lidarr_models::LidarrSerdeable;
use crate::network::Network;

mod library;
mod system;

#[cfg(test)]
#[path = "lidarr_network_tests.rs"]
mod lidarr_network_tests;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum LidarrEvent {
  GetDiskSpace,
  GetDownloads(u64),
  GetMetadataProfiles,
  GetQualityProfiles,
  GetRootFolders,
  GetStatus,
  GetTags,
  HealthCheck,
  ListArtists,
}

impl NetworkResource for LidarrEvent {
  fn resource(&self) -> &'static str {
    match &self {
      LidarrEvent::GetDiskSpace => "/diskspace",
      LidarrEvent::GetDownloads(_) => "/queue",
      LidarrEvent::GetMetadataProfiles => "/metadataprofile",
      LidarrEvent::GetQualityProfiles => "/qualityprofile",
      LidarrEvent::GetRootFolders => "/rootfolder",
      LidarrEvent::GetStatus => "/system/status",
      LidarrEvent::GetTags => "/tag",
      LidarrEvent::HealthCheck => "/health",
      LidarrEvent::ListArtists => "/artist",
    }
  }
}

impl From<LidarrEvent> for NetworkEvent {
  fn from(lidarr_event: LidarrEvent) -> Self {
    NetworkEvent::Lidarr(lidarr_event)
  }
}

impl Network<'_, '_> {
  pub async fn handle_lidarr_event(
    &mut self,
    lidarr_event: LidarrEvent,
  ) -> Result<LidarrSerdeable> {
    match lidarr_event {
      LidarrEvent::GetDiskSpace => self
        .get_lidarr_diskspace()
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetDownloads(count) => self
        .get_lidarr_downloads(count)
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetMetadataProfiles => self
        .get_lidarr_metadata_profiles()
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetQualityProfiles => self
        .get_lidarr_quality_profiles()
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetRootFolders => self
        .get_lidarr_root_folders()
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetStatus => self
        .get_lidarr_status()
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::GetTags => self.get_lidarr_tags().await.map(LidarrSerdeable::from),
      LidarrEvent::HealthCheck => self
        .get_lidarr_healthcheck()
        .await
        .map(LidarrSerdeable::from),
      LidarrEvent::ListArtists => self.list_artists().await.map(LidarrSerdeable::from),
    }
  }
}
