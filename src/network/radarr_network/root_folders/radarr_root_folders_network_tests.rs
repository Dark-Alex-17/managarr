#[cfg(test)]
mod tests {
  use crate::models::radarr_models::RadarrSerdeable;
  use crate::models::servarr_models::{AddRootFolderBody, RootFolder};
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::radarr_network::RadarrEvent;
  use crate::network::radarr_network::radarr_network_test_utils::test_utils::root_folder;
  use pretty_assertions::assert_eq;
  use serde_json::json;

  #[tokio::test]
  async fn test_handle_add_radarr_root_folder_event() {
    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(json!({
        "path": "/nfs/test"
      }))
      .returns(json!({}))
      .build_for(RadarrEvent::AddRootFolder(AddRootFolderBody::default()))
      .await;
    let add_root_folder_body = AddRootFolderBody {
      path: "/nfs/test".to_owned(),
    };
    let mut network = test_network(&app);

    assert!(
      network
        .handle_radarr_event(RadarrEvent::AddRootFolder(add_root_folder_body))
        .await
        .is_ok()
    );

    mock.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_delete_radarr_root_folder_event() {
    let (mock, app, _server) = MockServarrApi::delete()
      .path("/1")
      .build_for(RadarrEvent::DeleteRootFolder(1))
      .await;
    let mut network = test_network(&app);

    assert!(
      network
        .handle_radarr_event(RadarrEvent::DeleteRootFolder(1))
        .await
        .is_ok()
    );

    mock.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_get_radarr_root_folders_event() {
    let root_folder_json = json!([{
      "id": 1,
      "path": "/nfs",
      "accessible": true,
      "freeSpace": 219902325555200u64,
    }]);
    let response: Vec<RootFolder> = serde_json::from_value(root_folder_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(root_folder_json)
      .build_for(RadarrEvent::GetRootFolders)
      .await;
    let mut network = test_network(&app);

    let RadarrSerdeable::RootFolders(root_folders) = network
      .handle_radarr_event(RadarrEvent::GetRootFolders)
      .await
      .unwrap()
    else {
      panic!("Expected RootFolders")
    };
    mock.assert_async().await;
    assert_eq!(
      app.lock().await.data.radarr_data.root_folders.items,
      vec![root_folder()]
    );
    assert_eq!(root_folders, response);
  }
}
