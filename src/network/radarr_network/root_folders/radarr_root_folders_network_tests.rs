#[cfg(test)]
mod tests {
  use crate::models::radarr_models::RadarrSerdeable;
  use crate::models::servarr_models::{AddRootFolderBody, RootFolder};
  use crate::network::network_tests::test_utils::mock_servarr_api;
  use crate::network::radarr_network::radarr_network_test_utils::test_utils::root_folder;
  use crate::network::radarr_network::RadarrEvent;
  use crate::network::{Network, RequestMethod};
  use pretty_assertions::assert_eq;
  use reqwest::Client;
  use serde_json::json;
  use tokio_util::sync::CancellationToken;

  #[tokio::test]
  async fn test_handle_add_radarr_root_folder_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      Some(json!({
        "path": "/nfs/test"
      })),
      Some(json!({})),
      None,
      RadarrEvent::AddRootFolder(AddRootFolderBody::default()),
      None,
      None,
    )
    .await;
    let add_root_folder_body = AddRootFolderBody {
      path: "/nfs/test".to_owned(),
    };
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::AddRootFolder(add_root_folder_body))
      .await
      .is_ok());

    async_server.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_delete_radarr_root_folder_event() {
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Delete,
      None,
      None,
      None,
      RadarrEvent::DeleteRootFolder(1),
      Some("/1"),
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_radarr_event(RadarrEvent::DeleteRootFolder(1))
      .await
      .is_ok());

    async_server.assert_async().await;
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
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(root_folder_json),
      None,
      RadarrEvent::GetRootFolders,
      None,
      None,
    )
    .await;
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let RadarrSerdeable::RootFolders(root_folders) = network
      .handle_radarr_event(RadarrEvent::GetRootFolders)
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.radarr_data.root_folders.items,
        vec![root_folder()]
      );
      assert_eq!(root_folders, response);
    }
  }
}
