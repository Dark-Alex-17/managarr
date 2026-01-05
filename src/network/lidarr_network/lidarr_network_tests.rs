#[cfg(test)]
mod tests {
  use crate::network::{NetworkEvent, NetworkResource, lidarr_network::LidarrEvent};
  use pretty_assertions::assert_str_eq;
  use rstest::rstest;

  #[rstest]
  #[case(LidarrEvent::GetDiskSpace, "/diskspace")]
  #[case(LidarrEvent::GetDownloads(500), "/queue")]
  #[case(LidarrEvent::GetMetadataProfiles, "/metadataprofile")]
  #[case(LidarrEvent::GetQualityProfiles, "/qualityprofile")]
  #[case(LidarrEvent::GetRootFolders, "/rootfolder")]
  #[case(LidarrEvent::GetStatus, "/system/status")]
  #[case(LidarrEvent::GetTags, "/tag")]
  #[case(LidarrEvent::HealthCheck, "/health")]
  #[case(LidarrEvent::ListArtists, "/artist")]
  fn test_resource(#[case] event: LidarrEvent, #[case] expected_uri: &str) {
    assert_str_eq!(event.resource(), expected_uri);
  }

  #[test]
  fn test_from_lidarr_event() {
    assert_eq!(
      NetworkEvent::Lidarr(LidarrEvent::HealthCheck),
      NetworkEvent::from(LidarrEvent::HealthCheck)
    );
  }
}
