#[cfg(test)]
mod tests {
  use crate::models::readarr_models::ReadarrSerdeable;
  use crate::models::servarr_models::RootFolder;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::readarr_network::ReadarrEvent;
  use pretty_assertions::assert_eq;
  use serde_json::json;

  fn stale_root_folder() -> RootFolder {
    RootFolder {
      id: 99,
      path: "/stale".to_owned(),
      accessible: false,
      free_space: 1,
      unmapped_folders: None,
    }
  }

  #[tokio::test]
  async fn test_handle_get_readarr_root_folders_event() {
    let root_folders_json = json!([
      {
        "name": "Books",
        "path": "/nfs/books",
        "defaultMetadataProfileId": 1,
        "defaultQualityProfileId": 1,
        "defaultMonitorOption": "all",
        "defaultNewItemMonitorOption": "all",
        "defaultTags": [],
        "isCalibreLibrary": false,
        "port": 8080,
        "outputProfile": "default",
        "useSsl": false,
        "accessible": true,
        "freeSpace": 11039514558464i64,
        "totalSpace": 117810875334656i64,
        "id": 1
      },
      {
        "name": "Audiobooks",
        "path": "/nfs/audiobooks",
        "defaultMetadataProfileId": 1,
        "defaultQualityProfileId": 2,
        "defaultMonitorOption": "all",
        "defaultNewItemMonitorOption": "all",
        "defaultTags": [],
        "isCalibreLibrary": false,
        "port": 8080,
        "outputProfile": "default",
        "useSsl": false,
        "accessible": true,
        "freeSpace": 11039514558464i64,
        "totalSpace": 117810875334656i64,
        "id": 2
      }
    ]);
    let response: Vec<RootFolder> = serde_json::from_value(root_folders_json.clone()).unwrap();
    let expected_root_folders = vec![
      RootFolder {
        id: 1,
        path: "/nfs/books".to_owned(),
        accessible: true,
        free_space: 11039514558464,
        unmapped_folders: None,
      },
      RootFolder {
        id: 2,
        path: "/nfs/audiobooks".to_owned(),
        accessible: true,
        free_space: 11039514558464,
        unmapped_folders: None,
      },
    ];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(root_folders_json)
      .build_for(ReadarrEvent::GetRootFolders)
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app
        .data
        .readarr_data
        .root_folders
        .set_items(vec![stale_root_folder()]);
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetRootFolders)
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::RootFolders(root_folders) = result.unwrap() else {
      panic!("Expected RootFolders")
    };

    assert_eq!(root_folders, response);
    assert_eq!(
      app.lock().await.data.readarr_data.root_folders.items,
      expected_root_folders
    );
  }

  #[tokio::test]
  async fn test_handle_get_readarr_root_folders_event_empty_response() {
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!([]))
      .build_for(ReadarrEvent::GetRootFolders)
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app
        .data
        .readarr_data
        .root_folders
        .set_items(vec![stale_root_folder()]);
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetRootFolders)
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::RootFolders(root_folders) = result.unwrap() else {
      panic!("Expected RootFolders")
    };

    assert_is_empty!(root_folders);
    assert_is_empty!(app.lock().await.data.readarr_data.root_folders);
  }

  #[tokio::test]
  async fn test_handle_get_readarr_root_folders_event_failure() {
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!({}))
      .status(500)
      .build_for(ReadarrEvent::GetRootFolders)
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app
        .data
        .readarr_data
        .root_folders
        .set_items(vec![stale_root_folder()]);
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetRootFolders)
      .await;

    mock.assert_async().await;
    assert_err!(result);
    assert_eq!(
      app.lock().await.data.readarr_data.root_folders.items,
      vec![stale_root_folder()]
    );
  }
}
