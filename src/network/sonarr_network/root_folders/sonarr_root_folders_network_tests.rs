#[cfg(test)]
mod tests {
  use crate::models::servarr_models::{AddRootFolderBody, RootFolder};
  use crate::models::sonarr_models::SonarrSerdeable;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::sonarr_network::SonarrEvent;
  use crate::network::sonarr_network::sonarr_network_test_utils::test_utils::root_folder;
  use pretty_assertions::assert_eq;
  use serde_json::json;

  #[tokio::test]
  async fn test_handle_add_sonarr_root_folder_event() {
    let expected_add_root_folder_body = AddRootFolderBody {
      path: "/nfs/test".to_owned(),
    };
    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(json!({
        "path": "/nfs/test"
      }))
      .returns(json!({}))
      .build_for(SonarrEvent::AddRootFolder(
        expected_add_root_folder_body.clone(),
      ))
      .await;
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    assert!(
      network
        .handle_sonarr_event(SonarrEvent::AddRootFolder(expected_add_root_folder_body))
        .await
        .is_ok()
    );

    mock.assert_async().await;
    assert!(app.lock().await.data.sonarr_data.edit_root_folder.is_none());
  }

  #[tokio::test]
  async fn test_handle_delete_sonarr_root_folder_event() {
    let (mock, app, _server) = MockServarrApi::delete()
      .path("/1")
      .build_for(SonarrEvent::DeleteRootFolder(1))
      .await;
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    assert!(
      network
        .handle_sonarr_event(SonarrEvent::DeleteRootFolder(1))
        .await
        .is_ok()
    );

    mock.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_get_sonarr_root_folders_event() {
    let root_folder_json = json!([{
      "id": 1,
      "path": "/nfs",
      "accessible": true,
      "freeSpace": 219902325555200u64,
    }]);
    let response: Vec<RootFolder> = serde_json::from_value(root_folder_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(root_folder_json)
      .build_for(SonarrEvent::GetRootFolders)
      .await;
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    let SonarrSerdeable::RootFolders(root_folders) = network
      .handle_sonarr_event(SonarrEvent::GetRootFolders)
      .await
      .unwrap()
    else {
      panic!("Expected RootFolders")
    };
    mock.assert_async().await;
    assert_eq!(
      app.lock().await.data.sonarr_data.root_folders.items,
      vec![root_folder()]
    );
    assert_eq!(root_folders, response);
  }
}
