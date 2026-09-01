#[cfg(test)]
mod tests {
  use crate::models::readarr_models::{BlocklistItem, BlocklistResponse, ReadarrSerdeable};
  use crate::models::servarr_data::readarr::readarr_data::ActiveReadarrBlock;
  use crate::models::stateful_table::SortOption;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::readarr_network::ReadarrEvent;
  use crate::network::readarr_network::readarr_network_test_utils::test_utils::{
    AUTHOR_JSON, blocklist_item, stale_blocklist_item,
  };
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use serde_json::{Number, Value, json};

  #[rstest]
  #[tokio::test]
  async fn test_handle_get_readarr_blocklist_event(
    #[values(true, false)] use_custom_sorting: bool,
  ) {
    let blocklist_json = json!({"records": [
      {
        "id": 456,
        "authorId": 2001,
        "bookIds": [42020],
        "sourceTitle": "An Anthology",
        "quality": { "quality": { "name": "AZW3" } },
        "date": "2023-01-01T00:00:00Z",
        "protocol": "usenet",
        "indexer": "DrunkenSlug",
        "message": "test message",
        "author": serde_json::from_str::<Value>(AUTHOR_JSON).unwrap()
      },
      {
        "id": 123,
        "authorId": 1007,
        "bookIds": [42018],
        "sourceTitle": "z book",
        "quality": { "quality": { "name": "AZW3" } },
        "date": "2023-01-01T00:00:00Z",
        "protocol": "usenet",
        "indexer": "DrunkenSlug",
        "message": "test message",
        "author": serde_json::from_str::<Value>(AUTHOR_JSON).unwrap()
      }
    ]});
    let anthology_blocklist_item = BlocklistItem {
      id: 456,
      author_id: 2001,
      book_ids: Some(vec![Number::from(42020)]),
      source_title: "An Anthology".to_owned(),
      ..blocklist_item()
    };
    let z_book_blocklist_item = BlocklistItem {
      id: 123,
      author_id: 1007,
      book_ids: Some(vec![Number::from(42018)]),
      source_title: "z book".to_owned(),
      ..blocklist_item()
    };
    let expected_response = BlocklistResponse {
      records: vec![
        anthology_blocklist_item.clone(),
        z_book_blocklist_item.clone(),
      ],
    };
    let mut expected_blocklist_items = vec![z_book_blocklist_item, anthology_blocklist_item];
    let (mock, app, _server) = MockServarrApi::get()
      .returns(blocklist_json)
      .build_for(ReadarrEvent::GetBlocklist)
      .await;
    if use_custom_sorting {
      let cmp_fn = |a: &BlocklistItem, b: &BlocklistItem| {
        a.source_title
          .to_lowercase()
          .cmp(&b.source_title.to_lowercase())
      };
      expected_blocklist_items.sort_by(cmp_fn);

      let blocklist_sort_option = SortOption {
        name: "Source Title",
        cmp_fn: Some(cmp_fn),
      };
      app
        .lock()
        .await
        .data
        .readarr_data
        .blocklist
        .sorting(vec![blocklist_sort_option]);
    }
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.data.readarr_data.blocklist.sort_asc = true;
      app
        .data
        .readarr_data
        .blocklist
        .set_items(vec![stale_blocklist_item()]);
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetBlocklist)
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::BlocklistResponse(blocklist) = result.unwrap() else {
      panic!("Expected BlocklistResponse")
    };

    assert_eq!(blocklist, expected_response);
    let app = app.lock().await;
    assert_eq!(
      app.data.readarr_data.blocklist.items,
      expected_blocklist_items
    );
    assert!(app.data.readarr_data.blocklist.sort_asc);
  }

  #[tokio::test]
  async fn test_handle_get_readarr_blocklist_event_no_op_when_user_is_selecting_sort_options() {
    let stale_blocklist_items = vec![stale_blocklist_item()];
    let blocklist_json = json!({"records": [
      {
        "id": 456,
        "authorId": 2001,
        "bookIds": [42020],
        "sourceTitle": "An Anthology",
        "quality": { "quality": { "name": "AZW3" } },
        "date": "2023-01-01T00:00:00Z",
        "protocol": "usenet",
        "indexer": "DrunkenSlug",
        "message": "test message",
        "author": serde_json::from_str::<Value>(AUTHOR_JSON).unwrap()
      }
    ]});
    let expected_response = BlocklistResponse {
      records: vec![BlocklistItem {
        id: 456,
        author_id: 2001,
        book_ids: Some(vec![Number::from(42020)]),
        source_title: "An Anthology".to_owned(),
        ..blocklist_item()
      }],
    };
    let (mock, app, _server) = MockServarrApi::get()
      .returns(blocklist_json)
      .build_for(ReadarrEvent::GetBlocklist)
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app.data.readarr_data.blocklist.sort_asc = true;
      app
        .data
        .readarr_data
        .blocklist
        .set_items(stale_blocklist_items.clone());
      app.push_navigation_stack(ActiveReadarrBlock::BlocklistSortPrompt.into());
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetBlocklist)
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::BlocklistResponse(blocklist) = result.unwrap() else {
      panic!("Expected BlocklistResponse")
    };

    assert_eq!(blocklist, expected_response);
    let app = app.lock().await;
    assert_eq!(app.data.readarr_data.blocklist.items, stale_blocklist_items);
    assert!(app.data.readarr_data.blocklist.sort_asc);
  }

  #[tokio::test]
  async fn test_handle_get_readarr_blocklist_event_failure() {
    let stale_blocklist_items = vec![stale_blocklist_item()];
    let blocklist_json = json!({"records": [
      {
        "id": 456,
        "authorId": 2001,
        "bookIds": [42020],
        "sourceTitle": "An Anthology",
        "quality": { "quality": { "name": "AZW3" } },
        "date": "2023-01-01T00:00:00Z",
        "protocol": "usenet",
        "indexer": "DrunkenSlug",
        "message": "test message",
        "author": serde_json::from_str::<Value>(AUTHOR_JSON).unwrap()
      }
    ]});
    let (mock, app, _server) = MockServarrApi::get()
      .returns(blocklist_json)
      .status(500)
      .build_for(ReadarrEvent::GetBlocklist)
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app
        .data
        .readarr_data
        .blocklist
        .set_items(stale_blocklist_items.clone());
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetBlocklist)
      .await;

    mock.assert_async().await;

    assert_err!(result);
    assert_eq!(
      app.lock().await.data.readarr_data.blocklist.items,
      stale_blocklist_items
    );
  }
}
