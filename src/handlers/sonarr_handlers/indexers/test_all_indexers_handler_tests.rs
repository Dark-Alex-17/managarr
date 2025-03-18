#[cfg(test)]
mod tests {
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::app::App;
  use crate::handlers::sonarr_handlers::indexers::test_all_indexers_handler::TestAllIndexersHandler;
  use crate::handlers::KeyEventHandler;
  use crate::models::servarr_data::modals::IndexerTestResultModalItem;
  use crate::models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock;
  use crate::models::stateful_table::StatefulTable;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  mod test_handle_esc {
    use super::*;
    use crate::models::stateful_table::StatefulTable;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    fn test_test_all_indexers_esc(#[values(true, false)] is_ready: bool) {
      let mut app = App::test_default();
      app.is_loading = is_ready;
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveSonarrBlock::TestAllIndexers.into());
      app.data.sonarr_data.indexer_test_all_results = Some(StatefulTable::default());

      TestAllIndexersHandler::new(
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

  #[rstest]
  fn test_test_all_indexers_handler_ignore_alt_navigation(
    #[values(true, false)] should_ignore_quit_key: bool,
  ) {
    let mut app = App::test_default();
    app.should_ignore_quit_key = should_ignore_quit_key;
    let handler = TestAllIndexersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::default(),
      None,
    );

    assert_eq!(handler.ignore_alt_navigation(), should_ignore_quit_key);
  }

  #[test]
  fn test_test_all_indexers_handler_is_not_ready_when_loading() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
    app.is_loading = true;

    let handler = TestAllIndexersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::TestAllIndexers,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_test_all_indexers_handler_is_not_ready_when_results_is_none() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
    app.is_loading = false;

    let handler = TestAllIndexersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::TestAllIndexers,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_test_all_indexers_handler_is_not_ready_when_results_is_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
    app.is_loading = false;
    app.data.sonarr_data.indexer_test_all_results = Some(StatefulTable::default());

    let handler = TestAllIndexersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::TestAllIndexers,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_test_all_indexers_handler_is_ready_when_results_is_not_empty_and_is_loaded() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
    app.is_loading = false;
    let mut indexer_test_results = StatefulTable::default();
    indexer_test_results.set_items(vec![IndexerTestResultModalItem::default()]);
    app.data.sonarr_data.indexer_test_all_results = Some(indexer_test_results);

    let handler = TestAllIndexersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveSonarrBlock::TestAllIndexers,
      None,
    );

    assert!(handler.is_ready());
  }
}
