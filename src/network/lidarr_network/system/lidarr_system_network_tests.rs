#[cfg(test)]
mod tests {
  use crate::models::lidarr_models::{LidarrSerdeable, SystemStatus};
  use crate::models::servarr_models::DiskSpace;
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
