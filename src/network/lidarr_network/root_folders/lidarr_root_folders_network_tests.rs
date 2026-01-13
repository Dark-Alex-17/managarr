#[cfg(test)]
mod tests {
  use crate::models::lidarr_models::LidarrSerdeable;
  use crate::models::servarr_models::{AddRootFolderBody, RootFolder};
  use crate::network::lidarr_network::LidarrEvent;
  use crate::network::lidarr_network::lidarr_network_test_utils::test_utils::root_folder;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use pretty_assertions::assert_eq;
  use serde_json::json;

  #[tokio::test]
  async fn test_handle_add_lidarr_root_folder_event() {
    let expected_add_root_folder_body = AddRootFolderBody {
      path: "/nfs/test".to_owned(),
    };
    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(json!({
        "path": "/nfs/test"
      }))
      .returns(json!({}))
      .build_for(LidarrEvent::AddRootFolder(
        expected_add_root_folder_body.clone(),
      ))
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    assert!(
      network
        .handle_lidarr_event(LidarrEvent::AddRootFolder(expected_add_root_folder_body))
        .await
        .is_ok()
    );

    mock.assert_async().await;
    assert!(app.lock().await.data.lidarr_data.edit_root_folder.is_none());
  }

  #[tokio::test]
  async fn test_handle_delete_lidarr_root_folder_event() {
    let (mock, app, _server) = MockServarrApi::delete()
      .path("/1")
      .build_for(LidarrEvent::DeleteRootFolder(1))
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    assert!(
      network
        .handle_lidarr_event(LidarrEvent::DeleteRootFolder(1))
        .await
        .is_ok()
    );

    mock.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_get_root_folders_event() {
    let root_folders_json = json!([{
      "id": 1,
      "path": "/nfs",
      "accessible": true,
      "freeSpace": 219902325555200i64
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
    assert_eq!(
      app.lock().await.data.lidarr_data.root_folders.items,
      vec![root_folder()]
    );
  }
}
