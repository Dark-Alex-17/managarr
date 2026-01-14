#[cfg(test)]
mod tests {
  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::assert_navigation_popped;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::lidarr_handlers::indexers::test_all_indexers_handler::TestAllIndexersHandler;
  use crate::models::servarr_data::lidarr::lidarr_data::ActiveLidarrBlock;
  use crate::models::servarr_data::modals::IndexerTestResultModalItem;
  use crate::models::stateful_table::StatefulTable;
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  mod test_handle_esc {
    use super::*;

    const ESC_KEY: crate::event::Key = DEFAULT_KEYBINDINGS.esc.key;

    #[rstest]
    fn test_test_all_indexers_prompt_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.push_navigation_stack(ActiveLidarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveLidarrBlock::TestAllIndexers.into());
      app.data.lidarr_data.indexer_test_all_results = Some(StatefulTable::default());

      TestAllIndexersHandler::new(ESC_KEY, &mut app, ActiveLidarrBlock::TestAllIndexers, None)
        .handle();

      assert_navigation_popped!(app, ActiveLidarrBlock::Indexers.into());
      assert_none!(app.data.lidarr_data.indexer_test_all_results);
    }
  }

  #[test]
  fn test_test_all_indexers_handler_accepts() {
    ActiveLidarrBlock::iter().for_each(|active_lidarr_block| {
      if active_lidarr_block == ActiveLidarrBlock::TestAllIndexers {
        assert!(TestAllIndexersHandler::accepts(active_lidarr_block));
      } else {
        assert!(!TestAllIndexersHandler::accepts(active_lidarr_block));
      }
    })
  }

  #[rstest]
  fn test_test_all_indexers_handler_ignore_special_keys(
    #[values(true, false)] ignore_special_keys_for_textbox_input: bool,
  ) {
    let mut app = App::test_default();
    app.ignore_special_keys_for_textbox_input = ignore_special_keys_for_textbox_input;
    let handler = TestAllIndexersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::default(),
      None,
    );

    assert_eq!(
      handler.ignore_special_keys(),
      ignore_special_keys_for_textbox_input
    );
  }

  #[test]
  fn test_test_all_indexers_handler_not_ready_when_loading() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::TestAllIndexers.into());
    app.is_loading = true;

    let handler = TestAllIndexersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::TestAllIndexers,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_test_all_indexers_handler_not_ready_when_results_is_none() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::TestAllIndexers.into());
    app.is_loading = false;

    let handler = TestAllIndexersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::TestAllIndexers,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_test_all_indexers_handler_not_ready_when_results_is_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::TestAllIndexers.into());
    app.is_loading = false;
    app.data.lidarr_data.indexer_test_all_results = Some(StatefulTable::default());

    let handler = TestAllIndexersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::TestAllIndexers,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_test_all_indexers_handler_ready_when_not_loading_and_results_is_not_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveLidarrBlock::TestAllIndexers.into());
    app.is_loading = false;
    let mut results = StatefulTable::default();
    results.set_items(vec![IndexerTestResultModalItem::default()]);
    app.data.lidarr_data.indexer_test_all_results = Some(results);

    let handler = TestAllIndexersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveLidarrBlock::TestAllIndexers,
      None,
    );

    assert!(handler.is_ready());
  }
}
