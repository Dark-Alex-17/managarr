#[cfg(test)]
mod tests {
  use crate::models::lidarr_models::{LidarrSerdeable, SystemStatus};
  use crate::models::servarr_models::{DiskSpace, HostConfig, SecurityConfig};
  use crate::network::lidarr_network::LidarrEvent;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use pretty_assertions::assert_eq;
  use serde_json::json;

  #[tokio::test]
  async fn test_handle_get_diskspace_event() {
    let diskspace_json = json!([
      {
        "freeSpace": 1111,
        "totalSpace": 2222,
      },
      {
        "freeSpace": 3333,
        "totalSpace": 4444
      }
    ]);
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
  async fn test_handle_get_host_config_event() {
    let host_config_json = json!({
      "bindAddress": "*",
      "port": 8686,
      "urlBase": "some.test.site/lidarr",
      "instanceName": "Lidarr",
      "applicationUrl": "https://some.test.site:8686/lidarr",
      "enableSsl": true,
      "sslPort": 6868,
      "sslCertPath": "/app/lidarr.pfx",
      "sslCertPassword": "test"
    });
    let response: HostConfig = serde_json::from_value(host_config_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(host_config_json)
      .build_for(LidarrEvent::GetHostConfig)
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let result = network
      .handle_lidarr_event(LidarrEvent::GetHostConfig)
      .await;

    mock.assert_async().await;

    let LidarrSerdeable::HostConfig(host_config) = result.unwrap() else {
      panic!("Expected HostConfig");
    };

    assert_eq!(host_config, response);
  }

  #[tokio::test]
  async fn test_handle_get_security_config_event() {
    let security_config_json = json!({
      "authenticationMethod": "forms",
      "authenticationRequired": "disabledForLocalAddresses",
      "username": "test",
      "password": "some password",
      "apiKey": "someApiKey12345",
      "certificateValidation": "disabledForLocalAddresses"
    });
    let response: SecurityConfig = serde_json::from_value(security_config_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(security_config_json)
      .build_for(LidarrEvent::GetSecurityConfig)
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let result = network
      .handle_lidarr_event(LidarrEvent::GetSecurityConfig)
      .await;

    mock.assert_async().await;

    let LidarrSerdeable::SecurityConfig(security_config) = result.unwrap() else {
      panic!("Expected SecurityConfig");
    };

    assert_eq!(security_config, response);
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
