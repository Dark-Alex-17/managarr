#[cfg(test)]
mod tests {
  use crate::models::radarr_models::{RadarrHistoryItem, RadarrHistoryWrapper, RadarrSerdeable};
  use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
  use crate::models::stateful_table::SortOption;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::radarr_network::RadarrEvent;
  use crate::network::radarr_network::radarr_network_test_utils::test_utils::radarr_history_item;
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use serde_json::json;

  #[rstest]
  #[tokio::test]
  async fn test_handle_get_radarr_history_event(#[values(true, false)] use_custom_sorting: bool) {
    let history_json = json!({"records": [{
      "id": 123,
      "sourceTitle": "z movie",
      "movieId": 1007,
      "quality": { "quality": { "name": "HD - 1080p" } },
      "languages": [{ "id": 1, "name": "English" }],
      "date": "2022-12-30T07:37:56Z",
      "eventType": "grabbed",
      "data": {
        "indexer": "DrunkenSlug (Prowlarr)",
        "releaseGroup": "SPARKS",
        "downloadClient": "transmission",
      }
    },
    {
      "id": 456,
      "sourceTitle": "A Movie",
      "movieId": 2001,
      "quality": { "quality": { "name": "HD - 1080p" } },
      "languages": [{ "id": 1, "name": "English" }],
      "date": "2022-12-30T07:37:56Z",
      "eventType": "grabbed",
      "data": {
        "indexer": "DrunkenSlug (Prowlarr)",
        "releaseGroup": "SPARKS",
        "downloadClient": "transmission",
      }
    }]});
    let response: RadarrHistoryWrapper = serde_json::from_value(history_json.clone()).unwrap();
    let mut expected_history_items = vec![
      RadarrHistoryItem {
        id: 123,
        movie_id: 1007,
        source_title: "z movie".into(),
        ..radarr_history_item()
      },
      RadarrHistoryItem {
        id: 456,
        movie_id: 2001,
        source_title: "A Movie".into(),
        ..radarr_history_item()
      },
    ];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(history_json)
      .query("pageSize=500&sortDirection=descending&sortKey=date")
      .build_for(RadarrEvent::GetHistory(500))
      .await;
    app.lock().await.data.radarr_data.history.sort_asc = true;
    if use_custom_sorting {
      let cmp_fn = |a: &RadarrHistoryItem, b: &RadarrHistoryItem| {
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
        .radarr_data
        .history
        .sorting(vec![history_sort_option]);
    }
    let mut network = test_network(&app);

    let RadarrSerdeable::HistoryWrapper(history) = network
      .handle_radarr_event(RadarrEvent::GetHistory(500))
      .await
      .unwrap()
    else {
      panic!("Expected HistoryWrapper")
    };
    mock.assert_async().await;
    assert_eq!(
      app.lock().await.data.radarr_data.history.items,
      expected_history_items
    );
    assert!(app.lock().await.data.radarr_data.history.sort_asc);
    assert_eq!(history, response);
  }

  #[tokio::test]
  async fn test_handle_get_radarr_history_event_no_op_when_user_is_selecting_sort_options() {
    let history_json = json!({"records": [{
      "id": 123,
      "sourceTitle": "z movie",
      "movieId": 1007,
      "quality": { "quality": { "name": "Bluray-1080p" } },
      "languages": [{ "id": 1, "name": "English" }],
      "date": "2024-02-10T07:28:45Z",
      "eventType": "grabbed",
      "data": {
        "indexer": "DrunkenSlug (Prowlarr)",
        "releaseGroup": "SPARKS"
      }
    },
    {
      "id": 456,
      "sourceTitle": "A Movie",
      "movieId": 2001,
      "quality": { "quality": { "name": "Bluray-1080p" } },
      "languages": [{ "id": 1, "name": "English" }],
      "date": "2024-02-10T07:28:45Z",
      "eventType": "grabbed",
      "data": {
        "indexer": "DrunkenSlug (Prowlarr)",
        "releaseGroup": "SPARKS"
      }
    }]});
    let response: RadarrHistoryWrapper = serde_json::from_value(history_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(history_json)
      .query("pageSize=500&sortDirection=descending&sortKey=date")
      .build_for(RadarrEvent::GetHistory(500))
      .await;
    app.lock().await.data.radarr_data.history.sort_asc = true;
    app
      .lock()
      .await
      .push_navigation_stack(ActiveRadarrBlock::HistorySortPrompt.into());
    let cmp_fn = |a: &RadarrHistoryItem, b: &RadarrHistoryItem| {
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
      .radarr_data
      .history
      .sorting(vec![history_sort_option]);
    let mut network = test_network(&app);

    let RadarrSerdeable::HistoryWrapper(history) = network
      .handle_radarr_event(RadarrEvent::GetHistory(500))
      .await
      .unwrap()
    else {
      panic!("Expected HistoryWrapper")
    };
    mock.assert_async().await;
    assert_is_empty!(app.lock().await.data.radarr_data.history);
    assert!(app.lock().await.data.radarr_data.history.sort_asc);
    assert_eq!(history, response);
  }

  #[tokio::test]
  async fn test_handle_mark_radarr_history_item_as_failed_event() {
    let expected_history_item_id = 1;
    let (mock, app, _server) = MockServarrApi::post()
      .returns(json!({}))
      .path("/1")
      .build_for(RadarrEvent::MarkHistoryItemAsFailed(
        expected_history_item_id,
      ))
      .await;
    let mut network = test_network(&app);

    let result = network
      .handle_radarr_event(RadarrEvent::MarkHistoryItemAsFailed(
        expected_history_item_id,
      ))
      .await;

    mock.assert_async().await;
    assert_ok!(result);
  }
}
