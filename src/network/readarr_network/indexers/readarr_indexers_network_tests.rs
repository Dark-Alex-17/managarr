#[cfg(test)]
mod tests {
  use crate::models::readarr_models::ReadarrSerdeable;
  use crate::models::servarr_models::Indexer;
  use crate::network::network_tests::test_utils::{MockServarrApi, test_network};
  use crate::network::readarr_network::ReadarrEvent;
  use crate::network::readarr_network::readarr_network_test_utils::test_utils::{
    INDEXER_JSON, indexer, stale_indexer,
  };
  use pretty_assertions::assert_eq;
  use serde_json::{Value, json};

  #[tokio::test]
  async fn test_handle_get_readarr_indexers_event() {
    let indexers_response_json = json!([serde_json::from_str::<Value>(INDEXER_JSON).unwrap()]);
    let response: Vec<Indexer> = serde_json::from_value(indexers_response_json.clone()).unwrap();
    let (mock, app, _server) = MockServarrApi::get()
      .returns(indexers_response_json)
      .build_for(ReadarrEvent::GetIndexers)
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app
        .data
        .readarr_data
        .indexers
        .set_items(vec![stale_indexer()]);
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetIndexers)
      .await;

    mock.assert_async().await;

    let ReadarrSerdeable::Indexers(indexers) = result.unwrap() else {
      panic!("Expected Indexers")
    };

    assert_eq!(indexers, response);
    assert_eq!(
      app.lock().await.data.readarr_data.indexers.items,
      vec![indexer()]
    );
  }

  #[tokio::test]
  async fn test_handle_get_readarr_indexers_event_failure() {
    let stale_indexers = vec![stale_indexer()];
    let indexers_response_json = json!([serde_json::from_str::<Value>(INDEXER_JSON).unwrap()]);
    let (mock, app, _server) = MockServarrApi::get()
      .returns(indexers_response_json)
      .status(500)
      .build_for(ReadarrEvent::GetIndexers)
      .await;
    {
      let mut app = app.lock().await;
      app.server_tabs.set_index(3);
      app
        .data
        .readarr_data
        .indexers
        .set_items(stale_indexers.clone());
    }
    let mut network = test_network(&app);

    let result = network
      .handle_readarr_event(ReadarrEvent::GetIndexers)
      .await;

    mock.assert_async().await;

    assert_err!(result);
    assert_eq!(
      app.lock().await.data.readarr_data.indexers.items,
      stale_indexers
    );
  }
}
