#[cfg(test)]
mod tests {
  use crate::app::App;
  use crate::app::key_binding::DEFAULT_KEYBINDINGS;
  use crate::assert_navigation_popped;
  use crate::handlers::KeyEventHandler;
  use crate::handlers::readarr_handlers::indexers::test_all_indexers_handler::TestAllIndexersHandler;
  use crate::models::servarr_data::modals::IndexerTestResultModalItem;
  use crate::models::servarr_data::readarr::readarr_data::ActiveReadarrBlock;
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
      app.push_navigation_stack(ActiveReadarrBlock::Indexers.into());
      app.push_navigation_stack(ActiveReadarrBlock::TestAllIndexers.into());
      app.data.readarr_data.indexer_test_all_results = Some(StatefulTable::default());

      TestAllIndexersHandler::new(ESC_KEY, &mut app, ActiveReadarrBlock::TestAllIndexers, None)
        .handle();

      assert_navigation_popped!(app, ActiveReadarrBlock::Indexers.into());
      assert_none!(app.data.readarr_data.indexer_test_all_results);
    }
  }

  #[test]
  fn test_test_all_indexers_handler_accepts() {
    ActiveReadarrBlock::iter().for_each(|active_readarr_block| {
      if active_readarr_block == ActiveReadarrBlock::TestAllIndexers {
        assert!(TestAllIndexersHandler::accepts(active_readarr_block));
      } else {
        assert!(!TestAllIndexersHandler::accepts(active_readarr_block));
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
      ActiveReadarrBlock::default(),
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
    app.push_navigation_stack(ActiveReadarrBlock::TestAllIndexers.into());
    app.is_loading = true;

    let handler = TestAllIndexersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::TestAllIndexers,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_test_all_indexers_handler_not_ready_when_results_is_none() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::TestAllIndexers.into());
    app.is_loading = false;

    let handler = TestAllIndexersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::TestAllIndexers,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_test_all_indexers_handler_not_ready_when_results_is_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::TestAllIndexers.into());
    app.is_loading = false;
    app.data.readarr_data.indexer_test_all_results = Some(StatefulTable::default());

    let handler = TestAllIndexersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::TestAllIndexers,
      None,
    );

    assert!(!handler.is_ready());
  }

  #[test]
  fn test_test_all_indexers_handler_ready_when_not_loading_and_results_is_not_empty() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveReadarrBlock::TestAllIndexers.into());
    app.is_loading = false;
    let mut results = StatefulTable::default();
    results.set_items(vec![IndexerTestResultModalItem::default()]);
    app.data.readarr_data.indexer_test_all_results = Some(results);

    let handler = TestAllIndexersHandler::new(
      DEFAULT_KEYBINDINGS.esc.key,
      &mut app,
      ActiveReadarrBlock::TestAllIndexers,
      None,
    );

    assert!(handler.is_ready());
  }
}
