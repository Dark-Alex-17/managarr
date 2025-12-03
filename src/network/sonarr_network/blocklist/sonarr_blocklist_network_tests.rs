#[cfg(test)]
mod tests {
  use crate::models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock;
  use crate::models::sonarr_models::{BlocklistItem, BlocklistResponse, Series, SonarrSerdeable};
  use crate::models::stateful_table::SortOption;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::sonarr_network::SonarrEvent;
  use crate::network::sonarr_network::sonarr_network_test_utils::test_utils::{
    blocklist_item, series,
  };
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use serde_json::{Number, json};

  #[tokio::test]
  async fn test_handle_clear_sonarr_blocklist_event() {
    let blocklist_items = vec![
      BlocklistItem {
        id: 1,
        ..blocklist_item()
      },
      BlocklistItem {
        id: 2,
        ..blocklist_item()
      },
      BlocklistItem {
        id: 3,
        ..blocklist_item()
      },
    ];
    let expected_request_json = json!({ "ids": [1, 2, 3]});
    let (mock, app, _server) = MockServarrApi::delete()
      .with_request_body(expected_request_json)
      .build_for(SonarrEvent::ClearBlocklist)
      .await;
    app
      .lock()
      .await
      .data
      .sonarr_data
      .blocklist
      .set_items(blocklist_items);
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    assert!(
      network
        .handle_sonarr_event(SonarrEvent::ClearBlocklist)
        .await
        .is_ok()
    );

    mock.assert_async().await;
  }

  #[tokio::test]
  async fn test_handle_delete_sonarr_blocklist_item_event() {
    let (mock, app, _server) = MockServarrApi::delete()
      .path("/1")
      .build_for(SonarrEvent::DeleteBlocklistItem(1))
      .await;
    app
      .lock()
      .await
      .data
      .sonarr_data
      .blocklist
      .set_items(vec![blocklist_item()]);
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    assert!(
      network
        .handle_sonarr_event(SonarrEvent::DeleteBlocklistItem(1))
        .await
        .is_ok()
    );

    mock.assert_async().await;
  }

  #[rstest]
  #[tokio::test]
  async fn test_handle_get_sonarr_blocklist_event(#[values(true, false)] use_custom_sorting: bool) {
    let blocklist_json = json!({"records": [{
        "seriesId": 1007,
        "episodeIds": [42020],
        "sourceTitle": "z series",
        "languages": [{ "id": 1, "name": "English" }],
        "quality": { "quality": { "name": "Bluray-1080p" }},
        "date": "2024-02-10T07:28:45Z",
        "protocol": "usenet",
        "indexer": "NZBgeek (Prowlarr)",
        "message": "test message",
        "id": 123
    },
    {
        "seriesId": 2001,
        "episodeIds": [42018],
        "sourceTitle": "A Series",
        "languages": [{ "id": 1, "name": "English" }],
        "quality": { "quality": { "name": "Bluray-1080p" }},
        "date": "2024-02-10T07:28:45Z",
        "protocol": "usenet",
        "indexer": "NZBgeek (Prowlarr)",
        "message": "test message",
        "id": 456
    }]});
    let response: BlocklistResponse = serde_json::from_value(blocklist_json.clone()).unwrap();
    let mut expected_blocklist = vec![
      BlocklistItem {
        id: 123,
        series_id: 1007,
        series_title: Some("Z Series".into()),
        source_title: "z series".into(),
        episode_ids: vec![Number::from(42020)],
        ..blocklist_item()
      },
      BlocklistItem {
        id: 456,
        series_id: 2001,
        source_title: "A Series".into(),
        episode_ids: vec![Number::from(42018)],
        ..blocklist_item()
      },
    ];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(blocklist_json)
      .build_for(SonarrEvent::GetBlocklist)
      .await;
    app
      .lock()
      .await
      .data
      .sonarr_data
      .series
      .set_items(vec![Series {
        id: 1007,
        title: "Z Series".into(),
        ..series()
      }]);
    app.lock().await.data.sonarr_data.blocklist.sort_asc = true;
    if use_custom_sorting {
      let cmp_fn = |a: &BlocklistItem, b: &BlocklistItem| {
        a.source_title
          .to_lowercase()
          .cmp(&b.source_title.to_lowercase())
      };
      expected_blocklist.sort_by(cmp_fn);

      let blocklist_sort_option = SortOption {
        name: "Source Title",
        cmp_fn: Some(cmp_fn),
      };
      app
        .lock()
        .await
        .data
        .sonarr_data
        .blocklist
        .sorting(vec![blocklist_sort_option]);
    }
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    let SonarrSerdeable::BlocklistResponse(blocklist) = network
      .handle_sonarr_event(SonarrEvent::GetBlocklist)
      .await
      .unwrap()
    else {
      panic!("Expected BlocklistResponse")
    };
    mock.assert_async().await;
    assert_eq!(
      app.lock().await.data.sonarr_data.blocklist.items,
      expected_blocklist
    );
    assert!(app.lock().await.data.sonarr_data.blocklist.sort_asc);
    assert_eq!(blocklist, response);
  }

  #[tokio::test]
  async fn test_handle_get_sonarr_blocklist_event_no_op_when_user_is_selecting_sort_options() {
    let blocklist_json = json!({"records": [{
        "seriesId": 1007,
        "episodeIds": [42020],
        "sourceTitle": "z series",
        "languages": [{ "id": 1, "name": "English" }],
        "quality": { "quality": { "name": "Bluray-1080p" }},
        "date": "2024-02-10T07:28:45Z",
        "protocol": "usenet",
        "indexer": "NZBgeek (Prowlarr)",
        "message": "test message",
        "id": 123
    },
    {
        "seriesId": 2001,
        "episodeIds": [42018],
        "sourceTitle": "A Series",
        "languages": [{ "id": 1, "name": "English" }],
        "quality": { "quality": { "name": "Bluray-1080p" }},
        "date": "2024-02-10T07:28:45Z",
        "protocol": "usenet",
        "indexer": "NZBgeek (Prowlarr)",
        "message": "test message",
        "id": 456
    }]});
    let response: BlocklistResponse = serde_json::from_value(blocklist_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(blocklist_json)
      .build_for(SonarrEvent::GetBlocklist)
      .await;
    app.lock().await.data.sonarr_data.blocklist.sort_asc = true;
    app
      .lock()
      .await
      .push_navigation_stack(ActiveSonarrBlock::BlocklistSortPrompt.into());
    let cmp_fn = |a: &BlocklistItem, b: &BlocklistItem| {
      a.source_title
        .to_lowercase()
        .cmp(&b.source_title.to_lowercase())
    };
    let blocklist_sort_option = SortOption {
      name: "Source Title",
      cmp_fn: Some(cmp_fn),
    };
    app
      .lock()
      .await
      .data
      .sonarr_data
      .blocklist
      .sorting(vec![blocklist_sort_option]);
    app.lock().await.server_tabs.next();
    let mut network = test_network(&app);

    let SonarrSerdeable::BlocklistResponse(blocklist) = network
      .handle_sonarr_event(SonarrEvent::GetBlocklist)
      .await
      .unwrap()
    else {
      panic!("Expected BlocklistResponse")
    };
    mock.assert_async().await;
    assert!(app.lock().await.data.sonarr_data.blocklist.is_empty());
    assert!(app.lock().await.data.sonarr_data.blocklist.sort_asc);
    assert_eq!(blocklist, response);
  }
}
