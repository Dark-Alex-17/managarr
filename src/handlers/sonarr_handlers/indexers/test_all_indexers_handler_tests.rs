#[cfg(test)]
mod tests {
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::event::Key;
  use crate::handlers::sonarr_handlers::indexers::test_all_indexers_handler::TestAllIndexersHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::servarr_data::modals::IndexerTestResultModalItem;
  use crate::models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock;
  use crate::models::stateful_table::StatefulTable;
  use strum::IntoEnumIterator;

  mod test_handle_scroll_up_and_down {
    use pretty_assertions::assert_str_eq;
    use rstest::rstest;

    use crate::models::servarr_data::modals::IndexerTestResultModalItem;
    use crate::models::stateful_table::StatefulTable;
    use crate::simple_stateful_iterable_vec;

    use super::*;

    #[rstest]
    fn test_test_all_indexers_results_scroll(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
      let mut indexer_test_results = StatefulTable::default();
      indexer_test_results.set_items(simple_stateful_iterable_vec!(
        IndexerTestResultModalItem,
        String,
        name
      ));
      app.data.sonarr_data.indexer_test_all_results = Some(indexer_test_results);

      TestAllIndexersHandler::with(key, &mut app, ActiveSonarrBlock::TestAllIndexers, None)
        .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .indexer_test_all_results
          .as_ref()
          .unwrap()
          .current_selection()
          .name,
        "Test 2"
      );

      TestAllIndexersHandler::with(key, &mut app, ActiveSonarrBlock::TestAllIndexers, None)
        .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .indexer_test_all_results
          .as_ref()
          .unwrap()
          .current_selection()
          .name,
        "Test 1"
      );
    }

    #[rstest]
    fn test_test_all_indexers_results_scroll_no_op_when_not_ready(
      #[values(DEFAULT_KEYBINDINGS.up.key, DEFAULT_KEYBINDINGS.down.key)] key: Key,
    ) {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
      app.is_loading = true;
      let mut indexer_test_results = StatefulTable::default();
      indexer_test_results.set_items(simple_stateful_iterable_vec!(
        IndexerTestResultModalItem,
        String,
        name
      ));
      app.data.sonarr_data.indexer_test_all_results = Some(indexer_test_results);

      TestAllIndexersHandler::with(key, &mut app, ActiveSonarrBlock::TestAllIndexers, None)
        .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .indexer_test_all_results
          .as_ref()
          .unwrap()
          .current_selection()
          .name,
        "Test 1"
      );

      TestAllIndexersHandler::with(key, &mut app, ActiveSonarrBlock::TestAllIndexers, None)
        .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .indexer_test_all_results
          .as_ref()
          .unwrap()
          .current_selection()
          .name,
        "Test 1"
      );
    }
  }

  mod test_handle_home_end {
    use crate::extended_stateful_iterable_vec;
    use crate::models::servarr_data::modals::IndexerTestResultModalItem;
    use crate::models::stateful_table::StatefulTable;
    use pretty_assertions::assert_str_eq;

    use super::*;

    #[test]
    fn test_test_all_indexers_results_home_end() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
      let mut indexer_test_results = StatefulTable::default();
      indexer_test_results.set_items(extended_stateful_iterable_vec!(
        IndexerTestResultModalItem,
        String,
        name
      ));
      app.data.sonarr_data.indexer_test_all_results = Some(indexer_test_results);

      TestAllIndexersHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::TestAllIndexers,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .indexer_test_all_results
          .as_ref()
          .unwrap()
          .current_selection()
          .name,
        "Test 3"
      );

      TestAllIndexersHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::TestAllIndexers,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .indexer_test_all_results
          .as_ref()
          .unwrap()
          .current_selection()
          .name,
        "Test 1"
      );
    }

    #[test]
    fn test_test_all_indexers_results_home_end_no_op_when_not_ready() {
      let mut app = App::default();
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
      app.is_loading = true;
      let mut indexer_test_results = StatefulTable::default();
      indexer_test_results.set_items(extended_stateful_iterable_vec!(
        IndexerTestResultModalItem,
        String,
        name
      ));
      app.data.sonarr_data.indexer_test_all_results = Some(indexer_test_results);

      TestAllIndexersHandler::with(
        DEFAULT_KEYBINDINGS.end.key,
        &mut app,
        ActiveSonarrBlock::TestAllIndexers,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .indexer_test_all_results
          .as_ref()
          .unwrap()
          .current_selection()
          .name,
        "Test 1"
      );

      TestAllIndexersHandler::with(
        DEFAULT_KEYBINDINGS.home.key,
        &mut app,
        ActiveSonarrBlock::TestAllIndexers,
        None,
      )
      .handle();

      assert_str_eq!(
        app
          .data
          .sonarr_data
          .indexer_test_all_results
          .as_ref()
          .unwrap()
          .current_selection()
          .name,
        "Test 1"
      );
    }
  }

  mod test_handle_esc {
    use super::*;
    use crate::models::stateful_table::StatefulTable;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    fn test_test_all_indexers_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::default();
      app.is_loading = is_ready;
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveSonarrBlock::TestAllIndexers.into());
      app.data.sonarr_data.indexer_test_all_results = Some(StatefulTable::default());

      TestAllIndexersHandler::with(
        DEFAULT_KEYBINDINGS.esc.key,
        &mut app,
        ActiveSonarrBlock::TestAllIndexers,
        None,
      )
      .handle();

      assert_eq!(app.get_current_route(), ActiveSonarrBlock::Indexers.into());
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert!(app.data.sonarr_data.indexer_test_all_results.is_none());
    }
  }

  #[test]
  fn test_test_all_indexers_handler_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if active_sonarr_block == ActiveSonarrBlock::TestAllIndexers {
        assert!(TestAllIndexersHandler::accepts(active_sonarr_block));
      } else {
        assert!(!TestAllIndexersHandler::accepts(active_sonarr_block));
      }
    });
  }

  #[test]
  fn test_test_all_indexers_handler_is_not_ready_when_loading() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
    app.is_loading = true;

    let handler = TestAllIndexersHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::TestAllIndexers,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_test_all_indexers_handler_is_not_ready_when_results_is_none() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
    app.is_loading = false;

    let handler = TestAllIndexersHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::TestAllIndexers,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_test_all_indexers_handler_is_not_ready_when_results_is_empty() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
    app.is_loading = false;
    app.data.sonarr_data.indexer_test_all_results = Some(StatefulTable::default());

    let handler = TestAllIndexersHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::TestAllIndexers,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_test_all_indexers_handler_is_ready_when_results_is_not_empty_and_is_loaded() {
    let mut app = App::default();
    app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
    app.is_loading = false;
    let mut indexer_test_results = StatefulTable::default();
    indexer_test_results.set_items(vec![IndexerTestResultModalItem::default()]);
    app.data.sonarr_data.indexer_test_all_results = Some(indexer_test_results);

    let handler = TestAllIndexersHandler::with(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::TestAllIndexers,
      None,
    );

    assert!(handler.is_ready());
  }
}
