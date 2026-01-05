#[cfg(test)]
mod tests {
  use crate::models::lidarr_models::{
    DownloadsResponse, LidarrSerdeable, MetadataProfile, SystemStatus,
  };
  use crate::models::servarr_models::{DiskSpace, QualityProfile, RootFolder, Tag};
  use crate::network::lidarr_network::LidarrEvent;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use pretty_assertions::assert_eq;
  use serde_json::json;

  #[tokio::test]
  async fn test_handle_get_lidarr_healthcheck_event() {
    let (mock, app, _server) = MockServarrApi::get()
      .build_for(LidarrEvent::HealthCheck)
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let _ = network.handle_lidarr_event(LidarrEvent::HealthCheck).await;

    mock.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_get_metadata_profiles_event() {
    let metadata_profiles_json = json!([{
      "id": 1,
      "name": "Standard"
    }]);
    let response: Vec<MetadataProfile> =
      serde_json::from_value(metadata_profiles_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(metadata_profiles_json)
      .build_for(LidarrEvent::GetMetadataProfiles)
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let result = network
      .handle_lidarr_event(LidarrEvent::GetMetadataProfiles)
      .await;

    mock.assert_async().await;

    let LidarrSerdeable::MetadataProfiles(metadata_profiles) = result.unwrap() else {
      panic!("Expected MetadataProfiles");
    };

    assert_eq!(metadata_profiles, response);
    assert_eq!(
      app
        .lock()
        .await
        .data
        .lidarr_data
        .metadata_profile_map
        .get_by_left(&1),
      Some(&"Standard".to_owned())
    );
  }

  #[tokio::test]
  async fn test_handle_get_quality_profiles_event() {
    let quality_profiles_json = json!([{
      "id": 1,
      "name": "Lossless"
    }]);
    let response: Vec<QualityProfile> =
      serde_json::from_value(quality_profiles_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(quality_profiles_json)
      .build_for(LidarrEvent::GetQualityProfiles)
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let result = network
      .handle_lidarr_event(LidarrEvent::GetQualityProfiles)
      .await;

    mock.assert_async().await;

    let LidarrSerdeable::QualityProfiles(quality_profiles) = result.unwrap() else {
      panic!("Expected QualityProfiles");
    };

    assert_eq!(quality_profiles, response);
    assert_eq!(
      app
        .lock()
        .await
        .data
        .lidarr_data
        .quality_profile_map
        .get_by_left(&1),
      Some(&"Lossless".to_owned())
    );
  }

  #[tokio::test]
  async fn test_handle_get_tags_event() {
    let tags_json = json!([{
      "id": 1,
      "label": "usenet"
    }]);
    let response: Vec<Tag> = serde_json::from_value(tags_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(tags_json)
      .build_for(LidarrEvent::GetTags)
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let result = network.handle_lidarr_event(LidarrEvent::GetTags).await;

    mock.assert_async().await;

    let LidarrSerdeable::Tags(tags) = result.unwrap() else {
      panic!("Expected Tags");
    };

    assert_eq!(tags, response);
    assert_eq!(
      app.lock().await.data.lidarr_data.tags_map.get_by_left(&1),
      Some(&"usenet".to_owned())
    );
  }

  #[tokio::test]
  async fn test_handle_get_diskspace_event() {
    let diskspace_json = json!([{
      "freeSpace": 50000000000i64,
      "totalSpace": 100000000000i64
    }]);
    let response: Vec<DiskSpace> = serde_json::from_value(diskspace_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(diskspace_json)
      .build_for(LidarrEvent::GetDiskSpace)
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let result = network.handle_lidarr_event(LidarrEvent::GetDiskSpace).await;

    mock.assert_async().await;

    let LidarrSerdeable::DiskSpaces(disk_spaces) = result.unwrap() else {
      panic!("Expected DiskSpaces");
    };

    assert_eq!(disk_spaces, response);
    assert!(!app.lock().await.data.lidarr_data.disk_space_vec.is_empty());
  }

  #[tokio::test]
  async fn test_handle_get_downloads_event() {
    let downloads_json = json!({
      "records": [{
        "title": "Test Album",
        "status": "downloading",
        "id": 1,
        "size": 100.0,
        "sizeleft": 50.0,
        "indexer": "test-indexer"
      }]
    });
    let response: DownloadsResponse = serde_json::from_value(downloads_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(downloads_json)
      .query("pageSize=500")
      .build_for(LidarrEvent::GetDownloads(500))
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let result = network
      .handle_lidarr_event(LidarrEvent::GetDownloads(500))
      .await;

    mock.assert_async().await;

    let LidarrSerdeable::DownloadsResponse(downloads_response) = result.unwrap() else {
      panic!("Expected DownloadsResponse");
    };

    assert_eq!(downloads_response, response);
    assert!(!app.lock().await.data.lidarr_data.downloads.is_empty());
  }

  #[tokio::test]
  async fn test_handle_get_root_folders_event() {
    let root_folders_json = json!([{
      "id": 1,
      "path": "/music",
      "accessible": true,
      "freeSpace": 50000000000i64
    }]);
    let response: Vec<RootFolder> = serde_json::from_value(root_folders_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(root_folders_json)
      .build_for(LidarrEvent::GetRootFolders)
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let result = network
      .handle_lidarr_event(LidarrEvent::GetRootFolders)
      .await;

    mock.assert_async().await;

    let LidarrSerdeable::RootFolders(root_folders) = result.unwrap() else {
      panic!("Expected RootFolders");
    };

    assert_eq!(root_folders, response);
    assert!(!app.lock().await.data.lidarr_data.root_folders.is_empty());
  }

  #[tokio::test]
  async fn test_handle_get_status_event() {
    let status_json = json!({
      "version": "1.0.0",
      "startTime": "2023-01-01T00:00:00Z"
    });
    let response: SystemStatus = serde_json::from_value(status_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(status_json)
      .build_for(LidarrEvent::GetStatus)
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let result = network.handle_lidarr_event(LidarrEvent::GetStatus).await;

    mock.assert_async().await;

    let LidarrSerdeable::SystemStatus(status) = result.unwrap() else {
      panic!("Expected SystemStatus");
    };

    assert_eq!(status, response);
    assert_eq!(app.lock().await.data.lidarr_data.version, "1.0.0");
  }
}
