#[cfg(test)]
mod tests {
  use crate::models::lidarr_models::LidarrSerdeable;
  use crate::models::servarr_models::RootFolder;
  use crate::network::lidarr_network::LidarrEvent;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use pretty_assertions::assert_eq;
  use serde_json::json;

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
}
