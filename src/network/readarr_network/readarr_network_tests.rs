#[cfg(test)]
mod tests {
  use crate::app::App;
  use crate::models::readarr_models::{
    AddAuthorBody, AddReadarrRootFolderBody, DeleteParams, EditAuthorParams, ReadarrSerdeable,
  };
  use crate::models::servarr_models::{
    EditIndexerParams, IndexerSettings, MetadataProfile, QualityProfile, ReleaseDownloadBody, Tag,
  };
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::{NetworkEvent, NetworkResource, readarr_network::ReadarrEvent};
  use bimap::BiMap;
  use pretty_assertions::{assert_eq, assert_str_eq};
  use rstest::rstest;
  use serde_json::json;
  use std::sync::Arc;
  use tokio::sync::Mutex;

  #[rstest]
  #[case(ReadarrEvent::AddAuthor(AddAuthorBody::default()), "/author")]
  #[case(ReadarrEvent::DeleteAuthor(DeleteParams::default()), "/author")]
  #[case(ReadarrEvent::EditAuthor(EditAuthorParams::default()), "/author")]
  #[case(ReadarrEvent::GetAuthorDetails(1), "/author")]
  #[case(ReadarrEvent::ListAuthors, "/author")]
  #[case(ReadarrEvent::ToggleAuthorMonitoring(1), "/author")]
  #[case(ReadarrEvent::SearchNewAuthor(String::new()), "/author/lookup")]
  #[case(ReadarrEvent::DeleteBlocklistItem(1), "/blocklist")]
  #[case(ReadarrEvent::ClearBlocklist, "/blocklist/bulk")]
  #[case(ReadarrEvent::GetBlocklist, "/blocklist?page=1&pageSize=10000")]
  #[case(ReadarrEvent::DeleteBook(DeleteParams::default()), "/book")]
  #[case(ReadarrEvent::GetBookDetails(1), "/book")]
  #[case(ReadarrEvent::GetBooks(1), "/book")]
  #[case(ReadarrEvent::ToggleBookMonitoring(1), "/book")]
  #[case(ReadarrEvent::DeleteBookFile(1), "/bookfile")]
  #[case(ReadarrEvent::GetBookFiles(1), "/bookfile")]
  #[case(ReadarrEvent::GetQueuedEvents, "/command")]
  #[case(ReadarrEvent::StartTask(Default::default()), "/command")]
  #[case(ReadarrEvent::TriggerAutomaticAuthorSearch(1), "/command")]
  #[case(ReadarrEvent::TriggerAutomaticBookSearch(1), "/command")]
  #[case(ReadarrEvent::UpdateAllAuthors, "/command")]
  #[case(ReadarrEvent::UpdateAndScanAuthor(1), "/command")]
  #[case(ReadarrEvent::UpdateDownloads, "/command")]
  #[case(ReadarrEvent::GetHostConfig, "/config/host")]
  #[case(ReadarrEvent::GetSecurityConfig, "/config/host")]
  #[case(ReadarrEvent::GetAllIndexerSettings, "/config/indexer")]
  #[case(
    ReadarrEvent::EditAllIndexerSettings(IndexerSettings::default()),
    "/config/indexer"
  )]
  #[case(ReadarrEvent::GetDiskSpace, "/diskspace")]
  #[case(ReadarrEvent::GetBookEditions(1), "/edition")]
  #[case(ReadarrEvent::HealthCheck, "/health")]
  #[case(ReadarrEvent::GetHistory(500), "/history")]
  #[case(ReadarrEvent::GetAuthorHistory(1), "/history/author")]
  #[case(ReadarrEvent::GetBookHistory(1, 2), "/history/author")]
  #[case(ReadarrEvent::MarkHistoryItemAsFailed(1), "/history/failed")]
  #[case(ReadarrEvent::DeleteIndexer(1), "/indexer")]
  #[case(ReadarrEvent::EditIndexer(EditIndexerParams::default()), "/indexer")]
  #[case(ReadarrEvent::GetIndexers, "/indexer")]
  #[case(ReadarrEvent::TestIndexer(8), "/indexer/test")]
  #[case(ReadarrEvent::TestAllIndexers, "/indexer/testall")]
  #[case(ReadarrEvent::GetLogs(500), "/log")]
  #[case(ReadarrEvent::GetMetadataProfiles, "/metadataprofile")]
  #[case(ReadarrEvent::GetQualityProfiles, "/qualityprofile")]
  #[case(ReadarrEvent::DeleteDownload(1), "/queue")]
  #[case(ReadarrEvent::GetDownloads(500), "/queue")]
  #[case(
    ReadarrEvent::DownloadRelease(ReleaseDownloadBody::default()),
    "/release"
  )]
  #[case(ReadarrEvent::GetAuthorReleases(1), "/release")]
  #[case(ReadarrEvent::GetBookReleases(1), "/release")]
  #[case(
    ReadarrEvent::AddRootFolder(AddReadarrRootFolderBody::default()),
    "/rootfolder"
  )]
  #[case(ReadarrEvent::DeleteRootFolder(1), "/rootfolder")]
  #[case(ReadarrEvent::GetRootFolders, "/rootfolder")]
  #[case(ReadarrEvent::GetStatus, "/system/status")]
  #[case(ReadarrEvent::GetTasks, "/system/task")]
  #[case(ReadarrEvent::AddTag(String::new()), "/tag")]
  #[case(ReadarrEvent::DeleteTag(1), "/tag")]
  #[case(ReadarrEvent::GetTags, "/tag")]
  #[case(ReadarrEvent::GetUpdates, "/update")]
  fn test_resource(#[case] event: ReadarrEvent, #[case] expected_uri: &str) {
    assert_str_eq!(event.resource(), expected_uri);
  }

  #[test]
  fn test_from_readarr_event() {
    assert_eq!(
      NetworkEvent::Readarr(ReadarrEvent::HealthCheck),
      NetworkEvent::from(ReadarrEvent::HealthCheck)
    );
  }

  #[tokio::test]
  async fn test_handle_get_readarr_healthcheck_event() {
    let (mock, app, _server) = MockServarrApi::get()
      .build_for(ReadarrEvent::HealthCheck)
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let _ = network
      .handle_readarr_event(ReadarrEvent::HealthCheck)
      .await;

    mock.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_get_readarr_healthcheck_event_failure() {
    let (mock, app, _server) = MockServarrApi::get()
      .status(500)
      .build_for(ReadarrEvent::HealthCheck)
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::HealthCheck)
      .await;

    mock.assert_async().await;
    assert_err!(result);
  }

  #[tokio::test]
  async fn test_handle_get_metadata_profiles_event() {
    let metadata_profiles_json = json!([
      {
        "id": 1,
        "name": "Standard"
      },
      {
        "id": 2,
        "name": "None"
      }
    ]);
    let response: Vec<MetadataProfile> =
      serde_json::from_value(metadata_profiles_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(metadata_profiles_json)
      .build_for(ReadarrEvent::GetMetadataProfiles)
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetMetadataProfiles)
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::MetadataProfiles(metadata_profiles) = result.unwrap() else {
      panic!("Expected MetadataProfiles");
    };

    assert_eq!(metadata_profiles, response);

    let app = app.lock().await;

    assert_some_eq_x!(
      app.data.readarr_data.metadata_profile_map.get_by_left(&1),
      &"Standard".to_owned()
    );
    assert_some_eq_x!(
      app.data.readarr_data.metadata_profile_map.get_by_left(&2),
      &"None".to_owned()
    );
  }

  #[tokio::test]
  async fn test_handle_get_metadata_profiles_event_empty_response() {
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!([]))
      .build_for(ReadarrEvent::GetMetadataProfiles)
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.data.readarr_data.metadata_profile_map =
        BiMap::from_iter([(99i64, "Stale Profile".to_owned())]);
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetMetadataProfiles)
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::MetadataProfiles(metadata_profiles) = result.unwrap() else {
      panic!("Expected MetadataProfiles");
    };

    assert_is_empty!(metadata_profiles);
    assert_is_empty!(app.lock().await.data.readarr_data.metadata_profile_map);
  }

  #[tokio::test]
  async fn test_handle_get_metadata_profiles_event_failure() {
    let seeded_map = BiMap::from_iter([(99i64, "Stale Profile".to_owned())]);
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!({}))
      .status(500)
      .build_for(ReadarrEvent::GetMetadataProfiles)
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.data.readarr_data.metadata_profile_map = seeded_map.clone();
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetMetadataProfiles)
      .await;

    mock.assert_async().await;
    assert_err!(result);
    assert_eq!(
      app.lock().await.data.readarr_data.metadata_profile_map,
      seeded_map
    );
  }

  #[tokio::test]
  async fn test_handle_get_quality_profiles_event() {
    let quality_profiles_json = json!([
      {
        "id": 1,
        "name": "eBook"
      },
      {
        "id": 2,
        "name": "Spoken"
      }
    ]);
    let response: Vec<QualityProfile> =
      serde_json::from_value(quality_profiles_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(quality_profiles_json)
      .build_for(ReadarrEvent::GetQualityProfiles)
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetQualityProfiles)
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::QualityProfiles(quality_profiles) = result.unwrap() else {
      panic!("Expected QualityProfiles");
    };

    assert_eq!(quality_profiles, response);

    let app = app.lock().await;

    assert_some_eq_x!(
      app.data.readarr_data.quality_profile_map.get_by_left(&1),
      &"eBook".to_owned()
    );
    assert_some_eq_x!(
      app.data.readarr_data.quality_profile_map.get_by_left(&2),
      &"Spoken".to_owned()
    );
  }

  #[tokio::test]
  async fn test_handle_get_quality_profiles_event_empty_response() {
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!([]))
      .build_for(ReadarrEvent::GetQualityProfiles)
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.data.readarr_data.quality_profile_map =
        BiMap::from_iter([(99i64, "Stale Profile".to_owned())]);
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetQualityProfiles)
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::QualityProfiles(quality_profiles) = result.unwrap() else {
      panic!("Expected QualityProfiles");
    };

    assert_is_empty!(quality_profiles);
    assert_is_empty!(app.lock().await.data.readarr_data.quality_profile_map);
  }

  #[tokio::test]
  async fn test_handle_get_quality_profiles_event_failure() {
    let seeded_map = BiMap::from_iter([(99i64, "Stale Profile".to_owned())]);
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!({}))
      .status(500)
      .build_for(ReadarrEvent::GetQualityProfiles)
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.data.readarr_data.quality_profile_map = seeded_map.clone();
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetQualityProfiles)
      .await;

    mock.assert_async().await;
    assert_err!(result);
    assert_eq!(
      app.lock().await.data.readarr_data.quality_profile_map,
      seeded_map
    );
  }

  #[tokio::test]
  async fn test_handle_get_tags_event() {
    let tags_json = json!([
      {
        "id": 1,
        "label": "huntarr-missing"
      }
    ]);
    let response: Vec<Tag> = serde_json::from_value(tags_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(tags_json)
      .build_for(ReadarrEvent::GetTags)
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network.handle_readarr_event(ReadarrEvent::GetTags).await;

    mock.assert_async().await;

    let ReadarrSerdeable::Tags(tags) = result.unwrap() else {
      panic!("Expected Tags");
    };

    assert_eq!(tags, response);
    assert_some_eq_x!(
      app.lock().await.data.readarr_data.tags_map.get_by_left(&1),
      &"huntarr-missing".to_owned()
    );
  }

  #[tokio::test]
  async fn test_handle_get_tags_event_empty_response() {
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!([]))
      .build_for(ReadarrEvent::GetTags)
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.data.readarr_data.tags_map = BiMap::from_iter([(99i64, "stale-tag".to_owned())]);
    }
    let mut network = test_network(&app);

    let result = network.handle_readarr_event(ReadarrEvent::GetTags).await;

    mock.assert_async().await;

    let ReadarrSerdeable::Tags(tags) = result.unwrap() else {
      panic!("Expected Tags");
    };

    assert_is_empty!(tags);
    assert_is_empty!(app.lock().await.data.readarr_data.tags_map);
  }

  #[tokio::test]
  async fn test_handle_get_tags_event_failure() {
    let seeded_map = BiMap::from_iter([(99i64, "stale-tag".to_owned())]);
    let (mock, app, _server) = MockServarrApi::get()
      .returns(json!({}))
      .status(500)
      .build_for(ReadarrEvent::GetTags)
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.data.readarr_data.tags_map = seeded_map.clone();
    }
    let mut network = test_network(&app);

    let result = network.handle_readarr_event(ReadarrEvent::GetTags).await;

    mock.assert_async().await;
    assert_err!(result);
    assert_eq!(app.lock().await.data.readarr_data.tags_map, seeded_map);
  }

  #[tokio::test]
  async fn test_handle_add_readarr_tag_event() {
    let tag_json = json!({
      "id": 2,
      "label": "managarr-verify"
    });
    let response: Tag = serde_json::from_value(tag_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(json!({ "label": "managarr-verify" }))
      .returns(tag_json)
      .build_for(ReadarrEvent::AddTag("managarr-verify".to_owned()))
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.data.readarr_data.tags_map = BiMap::from_iter([(1i64, "huntarr-missing".to_owned())]);
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::AddTag("managarr-verify".to_owned()))
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::Tag(tag) = result.unwrap() else {
      panic!("Expected Tag");
    };

    assert_eq!(tag, response);
    assert_eq!(
      app.lock().await.data.readarr_data.tags_map,
      BiMap::from_iter([
        (1i64, "huntarr-missing".to_owned()),
        (2i64, "managarr-verify".to_owned())
      ])
    );
  }

  #[tokio::test]
  async fn test_handle_add_readarr_tag_event_failure() {
    let seeded_map = BiMap::from_iter([(1i64, "huntarr-missing".to_owned())]);
    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(json!({ "label": "managarr-verify" }))
      .returns(json!({}))
      .status(500)
      .build_for(ReadarrEvent::AddTag("managarr-verify".to_owned()))
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.data.readarr_data.tags_map = seeded_map.clone();
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::AddTag("managarr-verify".to_owned()))
      .await;

    mock.assert_async().await;
    assert_err!(result);
    assert_eq!(app.lock().await.data.readarr_data.tags_map, seeded_map);
  }

  #[tokio::test]
  async fn test_handle_delete_readarr_tag_event() {
    let (mock, app, _server) = MockServarrApi::delete()
      .path("/1")
      .build_for(ReadarrEvent::DeleteTag(1))
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::DeleteTag(1))
      .await;

    mock.assert_async().await;
    assert_ok!(result);
  }

  #[tokio::test]
  async fn test_handle_delete_readarr_tag_event_failure() {
    let (mock, app, _server) = MockServarrApi::delete()
      .path("/1")
      .status(500)
      .build_for(ReadarrEvent::DeleteTag(1))
      .await;
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::DeleteTag(1))
      .await;

    mock.assert_async().await;
    assert_err!(result);
  }

  #[tokio::test]
  async fn test_extract_and_add_readarr_tag_ids_vec() {
    let app_arc = Arc::new(Mutex::new(App::test_default()));
    let tags = "    test,HI ,, usenet ";
    {
      let mut app = app_arc.lock().await;
      app.data.readarr_data.tags_map = BiMap::from_iter([
        (1, "usenet".to_owned()),
        (2, "test".to_owned()),
        (3, "hi".to_owned()),
      ]);
    }
    app_arc.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app_arc);

    assert_eq!(
      network.extract_and_add_readarr_tag_ids_vec(tags).await,
      vec![2, 3, 1]
    );
  }

  #[tokio::test]
  async fn test_extract_and_add_readarr_tag_ids_vec_add_missing_tags_first() {
    let (mock, app, _server) = MockServarrApi::post()
      .with_request_body(json!({ "label": "TESTING" }))
      .returns(json!({ "id": 3, "label": "testing" }))
      .build_for(ReadarrEvent::GetTags)
      .await;
    let tags = "usenet, test, TESTING";
    {
      let mut app_guard = app.lock().await;
      app_guard.data.readarr_data.tags_map =
        BiMap::from_iter([(1, "usenet".to_owned()), (2, "test".to_owned())]);
    }
    app.lock().await.server_tabs.set_index(3);
    let mut network = test_network(&app);

    let tag_ids_vec = network.extract_and_add_readarr_tag_ids_vec(tags).await;

    mock.assert_async().await;
    assert_eq!(tag_ids_vec, vec![1, 2, 3]);
    assert_eq!(
      app.lock().await.data.readarr_data.tags_map,
      BiMap::from_iter([
        (1, "usenet".to_owned()),
        (2, "test".to_owned()),
        (3, "testing".to_owned())
      ])
    );
  }
}
