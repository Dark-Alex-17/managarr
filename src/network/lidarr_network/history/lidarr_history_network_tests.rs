#[cfg(test)]
mod tests {
  use crate::models::lidarr_models::{LidarrHistoryItem, LidarrHistoryWrapper, LidarrSerdeable};
  use crate::models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock;
  use crate::models::stateful_table::SortOption;
  use crate::network::lidarr_network::LidarrEvent;
  use crate::network::lidarr_network::lidarr_network_test_utils::test_utils::lidarr_history_item;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use rstest::rstest;
  use serde_json::json;

  #[rstest]
  #[tokio::test]
  async fn test_handle_get_lidarr_history_event(#[values(true, false)] use_custom_sorting: bool) {
    let history_json = json!({"records": [{
      "id": 123,
      "sourceTitle": "z album",
      "albumId": 1007,
      "artistId": 1007,
      "quality": { "quality": { "name": "Lossless" } },
      "date": "2023-01-01T00:00:00Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/music/Something/cool.mp3",
        "importedPath": "/nfs/music/Something/Album 1/Cool.mp3"
      }
    },
    {
      "id": 456,
      "sourceTitle": "An Album",
      "albumId": 2001,
      "artistId": 2001,
      "quality": { "quality": { "name": "Lossless" } },
      "date": "2023-01-01T00:00:00Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/music/Something/cool.mp3",
        "importedPath": "/nfs/music/Something/Album 1/Cool.mp3"
      }
    }]});
    let response: LidarrHistoryWrapper = serde_json::from_value(history_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(history_json)
      .query("pageSize=500&sortDirection=descending&sortKey=date")
      .build_for(LidarrEvent::GetHistory(500))
      .await;
    let mut expected_history_items = vec![
      LidarrHistoryItem {
        id: 123,
        album_id: 1007,
        artist_id: 1007,
        source_title: "z album".into(),
        ..lidarr_history_item()
      },
      LidarrHistoryItem {
        id: 456,
        album_id: 2001,
        artist_id: 2001,
        source_title: "An Album".into(),
        ..lidarr_history_item()
      },
    ];
    {
      let mut app_mut = app.lock().await;
      app_mut.server_tabs.set_index(2);
      app_mut.data.lidarr_data.history.sort_asc = true;
    }
    if use_custom_sorting {
      let cmp_fn = |a: &LidarrHistoryItem, b: &LidarrHistoryItem| {
        a.source_title
          .text
          .to_lowercase()
          .cmp(&b.source_title.text.to_lowercase())
      };
      expected_history_items.sort_by(cmp_fn);

      let history_sort_option = SortOption {
        name: "Source Title",
        cmp_fn: Some(cmp_fn),
      };
      app
        .lock()
        .await
        .data
        .lidarr_data
        .history
        .sorting(vec![history_sort_option]);
    }

    let mut network = test_network(&app);

    let result = network
      .handle_lidarr_event(LidarrEvent::GetHistory(500))
      .await;

    mock.assert_async().await;
    assert!(result.is_ok());
    let LidarrSerdeable::LidarrHistoryWrapper(history) = result.unwrap() else {
      panic!("Expected LidarrHistoryWrapper")
    };
    assert_eq!(
      app.lock().await.data.lidarr_data.history.items,
      expected_history_items
    );
    assert!(app.lock().await.data.lidarr_data.history.sort_asc);
    assert_eq!(history, response);
  }

  #[tokio::test]
  async fn test_handle_get_lidarr_history_event_no_op_when_user_is_selecting_sort_options() {
    let history_json = json!({"records": [{
      "id": 123,
      "sourceTitle": "z album",
      "albumId": 1007,
      "artistId": 1007,
      "quality": { "quality": { "name": "Lossless" } },
      "date": "2023-01-01T00:00:00Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/music/Something/cool.mp3",
        "importedPath": "/nfs/music/Something/Album 1/Cool.mp3"
      }
    },
    {
      "id": 456,
      "sourceTitle": "An Album",
      "albumId": 2001,
      "artistId": 2001,
      "quality": { "quality": { "name": "Lossless" } },
      "date": "2023-01-01T00:00:00Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/music/Something/cool.mp3",
        "importedPath": "/nfs/music/Something/Album 1/Cool.mp3"
      }
    }]});
    let response: LidarrHistoryWrapper = serde_json::from_value(history_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(history_json)
      .query("pageSize=500&sortDirection=descending&sortKey=date")
      .build_for(LidarrEvent::GetHistory(500))
      .await;
    app.lock().await.data.lidarr_data.history.sort_asc = true;
    app
      .lock()
      .await
      .push_navigation_stack(ActiveLidarrBlock::HistorySortPrompt.into());
    let cmp_fn = |a: &LidarrHistoryItem, b: &LidarrHistoryItem| {
      a.source_title
        .text
        .to_lowercase()
        .cmp(&b.source_title.text.to_lowercase())
    };
    let history_sort_option = SortOption {
      name: "Source Title",
      cmp_fn: Some(cmp_fn),
    };
    app
      .lock()
      .await
      .data
      .lidarr_data
      .history
      .sorting(vec![history_sort_option]);
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let LidarrSerdeable::LidarrHistoryWrapper(history) = network
      .handle_lidarr_event(LidarrEvent::GetHistory(500))
      .await
      .unwrap()
    else {
      panic!("Expected LidarrHistoryWrapper")
    };
    mock.assert_async().await;
    assert_is_empty!(app.lock().await.data.lidarr_data.history);
    assert!(app.lock().await.data.lidarr_data.history.sort_asc);
    assert_eq!(history, response);
  }

  #[tokio::test]
  async fn test_handle_mark_lidarr_history_item_as_failed_event() {
    let history_item_id = 1234i64;
    let (mock, app, _server) = MockServarrApi::post()
      .returns(json!({}))
      .path("/1234")
      .build_for(LidarrEvent::MarkHistoryItemAsFailed(history_item_id))
      .await;
    app.lock().await.server_tabs.set_index(2);
    let mut network = test_network(&app);

    let result = network
      .handle_lidarr_event(LidarrEvent::MarkHistoryItemAsFailed(history_item_id))
      .await;

    mock.assert_async().await;
    assert_ok!(result);
  }
}
