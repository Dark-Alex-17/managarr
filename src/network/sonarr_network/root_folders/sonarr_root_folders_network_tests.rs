#[cfg(test)]
mod tests {
  use crate::models::servarr_models::{AddRootFolderBody, RootFolder};
  use crate::models::sonarr_models::SonarrSerdeable;
  use crate::network::network_tests::test_utils::mock_servarr_api;
  use crate::network::sonarr_network::sonarr_network_test_utils::test_utils::root_folder;
  use crate::network::sonarr_network::SonarrEvent;
  use crate::network::{Network, RequestMethod};
  use pretty_assertions::assert_eq;
  use reqwest::Client;
  use serde_json::json;
  use tokio_util::sync::CancellationToken;

  #[tokio::test]
  async fn test_handle_add_sonarr_root_folder_event() {
    let expected_add_root_folder_body = AddRootFolderBody {
      path: "/nfs/test".to_owned(),
    };
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "path": "/nfs/test"
      })),
      Some(json!({})),
      None,
      SonarrEvent::AddRootFolder(expected_add_root_folder_body.clone()),
      None,
      None,
    )
    .await;
    app_arc.lock().await.server_tabs.next();
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::AddRootFolder(expected_add_root_folder_body))
      .await
      .is_ok());

    async_server.assert_async().await;
    assert!(app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .edit_root_folder
      .is_none());
  }

  #[tokio::test]
  async fn test_handle_delete_sonarr_root_folder_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Delete,
      None,
      None,
      None,
      SonarrEvent::DeleteRootFolder(1),
      Some("/1"),
      None,
    )
    .await;
    app_arc.lock().await.server_tabs.next();
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::DeleteRootFolder(1))
      .await
      .is_ok());

    async_server.assert_async().await;
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
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(root_folder_json),
      None,
      SonarrEvent::GetRootFolders,
      None,
      None,
    )
    .await;
    app_arc.lock().await.server_tabs.next();
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::RootFolders(root_folders) = network
      .handle_sonarr_event(SonarrEvent::GetRootFolders)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.sonarr_data.root_folders.items,
        vec![root_folder()]
      );
      assert_eq!(root_folders, response);
    }
  }
}
