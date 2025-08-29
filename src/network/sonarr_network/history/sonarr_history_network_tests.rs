#[cfg(test)]
mod tests {
  use crate::models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock;
  use crate::models::sonarr_models::{SonarrHistoryItem, SonarrHistoryWrapper, SonarrSerdeable};
  use crate::models::stateful_table::SortOption;
  use crate::network::network_tests::test_utils::mock_servarr_api;
  use crate::network::sonarr_network::sonarr_network_test_utils::test_utils::history_item;
  use crate::network::sonarr_network::SonarrEvent;
  use crate::network::{Network, RequestMethod};
  use pretty_assertions::assert_eq;
  use reqwest::Client;
  use rstest::rstest;
  use serde_json::json;
  use tokio_util::sync::CancellationToken;

  #[rstest]
  #[tokio::test]
  async fn test_handle_get_sonarr_history_event(#[values(true, false)] use_custom_sorting: bool) {
    let history_json = json!({"records": [{
      "id": 123,
      "sourceTitle": "z episode",
      "episodeId": 1007,
      "quality": { "quality": { "name": "Bluray-1080p" } },
      "languages": [{ "id": 1, "name": "English" }],
      "date": "2024-02-10T07:28:45Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/series/Coolness/something.cool.mkv",
        "importedPath": "/nfs/tv/Coolness/Season 1/Coolness - S01E01 - Something Cool Bluray-1080p.mkv"
      }
    },
    {
      "id": 456,
      "sourceTitle": "A Episode",
      "episodeId": 2001,
      "quality": { "quality": { "name": "Bluray-1080p" } },
      "languages": [{ "id": 1, "name": "English" }],
      "date": "2024-02-10T07:28:45Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/series/Coolness/something.cool.mkv",
        "importedPath": "/nfs/tv/Coolness/Season 1/Coolness - S01E01 - Something Cool Bluray-1080p.mkv"
      }
    }]});
    let response: SonarrHistoryWrapper = serde_json::from_value(history_json.clone()).unwrap();
    let mut expected_history_items = vec![
      SonarrHistoryItem {
        id: 123,
        episode_id: 1007,
        source_title: "z episode".into(),
        ..history_item()
      },
      SonarrHistoryItem {
        id: 456,
        episode_id: 2001,
        source_title: "A Episode".into(),
        ..history_item()
      },
    ];
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(history_json),
      None,
      SonarrEvent::GetHistory(500),
      None,
      Some("pageSize=500&sortDirection=descending&sortKey=date"),
    )
    .await;
    app_arc.lock().await.data.sonarr_data.history.sort_asc = true;
    if use_custom_sorting {
      let cmp_fn = |a: &SonarrHistoryItem, b: &SonarrHistoryItem| {
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
      app_arc
        .lock()
        .await
        .data
        .sonarr_data
        .history
        .sorting(vec![history_sort_option]);
    }
    app_arc.lock().await.server_tabs.next();
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::SonarrHistoryWrapper(history) = network
      .handle_sonarr_event(SonarrEvent::GetHistory(500))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert_eq!(
        app_arc.lock().await.data.sonarr_data.history.items,
        expected_history_items
      );
      assert!(app_arc.lock().await.data.sonarr_data.history.sort_asc);
      assert_eq!(history, response);
    }
  }

  #[tokio::test]
  async fn test_handle_get_sonarr_history_event_no_op_when_user_is_selecting_sort_options() {
    let history_json = json!({"records": [{
      "id": 123,
      "sourceTitle": "z episode",
      "episodeId": 1007,
      "quality": { "quality": { "name": "Bluray-1080p" } },
      "languages": [{ "id": 1, "name": "English" }],
      "date": "2024-02-10T07:28:45Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/series/Coolness/something.cool.mkv",
        "importedPath": "/nfs/tv/Coolness/Season 1/Coolness - S01E01 - Something Cool Bluray-1080p.mkv"
      }
    },
    {
      "id": 456,
      "sourceTitle": "A Episode",
      "episodeId": 2001,
      "quality": { "quality": { "name": "Bluray-1080p" } },
      "languages": [{ "id": 1, "name": "English" }],
      "date": "2024-02-10T07:28:45Z",
      "eventType": "grabbed",
      "data": {
        "droppedPath": "/nfs/nzbget/completed/series/Coolness/something.cool.mkv",
        "importedPath": "/nfs/tv/Coolness/Season 1/Coolness - S01E01 - Something Cool Bluray-1080p.mkv"
      }
    }]});
    let response: SonarrHistoryWrapper = serde_json::from_value(history_json.clone()).unwrap();
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Get,
      None,
      Some(history_json),
      None,
      SonarrEvent::GetHistory(500),
      None,
      Some("pageSize=500&sortDirection=descending&sortKey=date"),
    )
    .await;
    app_arc.lock().await.data.sonarr_data.history.sort_asc = true;
    app_arc
      .lock()
      .await
      .push_navigation_stack(ActiveSonarrBlock::HistorySortPrompt.into());
    let cmp_fn = |a: &SonarrHistoryItem, b: &SonarrHistoryItem| {
      a.source_title
        .text
        .to_lowercase()
        .cmp(&b.source_title.text.to_lowercase())
    };
    let history_sort_option = SortOption {
      name: "Source Title",
      cmp_fn: Some(cmp_fn),
    };
    app_arc
      .lock()
      .await
      .data
      .sonarr_data
      .history
      .sorting(vec![history_sort_option]);
    app_arc.lock().await.server_tabs.next();
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    if let SonarrSerdeable::SonarrHistoryWrapper(history) = network
      .handle_sonarr_event(SonarrEvent::GetHistory(500))
      .await
      .unwrap()
    {
      async_server.assert_async().await;
      assert!(app_arc.lock().await.data.sonarr_data.history.is_empty());
      assert!(app_arc.lock().await.data.sonarr_data.history.sort_asc);
      assert_eq!(history, response);
    }
  }

  #[tokio::test]
  async fn test_handle_mark_sonarr_history_item_as_failed_event() {
    let expected_history_item_id = 1;
    let (async_server, app_arc, _server) = mock_servarr_api(
      RequestMethod::Post,
      None,
      Some(json!({})),
      None,
      SonarrEvent::MarkHistoryItemAsFailed(expected_history_item_id),
      Some("/1"),
      None,
    )
    .await;
    app_arc.lock().await.server_tabs.next();
    let mut network = Network::new(&app_arc, CancellationToken::new(), Client::new());

    assert!(network
      .handle_sonarr_event(SonarrEvent::MarkHistoryItemAsFailed(
        expected_history_item_id
      ))
      .await
      .is_ok());
    async_server.assert_async().await;
  }
}
