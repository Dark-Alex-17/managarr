#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::readarr::readarr_data::ActiveReadarrBlock;
  use crate::ui::DrawUi;
  use crate::ui::readarr_ui::indexers::test_all_indexers_ui::TestAllIndexersUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_test_all_indexers_ui_accepts() {
    ActiveReadarrBlock::iter().for_each(|active_readarr_block| {
      if active_readarr_block == ActiveReadarrBlock::TestAllIndexers {
        assert!(TestAllIndexersUi::accepts(active_readarr_block.into()));
      } else {
        assert!(!TestAllIndexersUi::accepts(active_readarr_block.into()));
      }
    });
  }

  mod test_decorate_test_result_row_with_style {
    use pretty_assertions::assert_eq;
    use ratatui::widgets::{Cell, Row};

    use crate::models::servarr_data::modals::IndexerTestResultModalItem;
    use crate::ui::readarr_ui::indexers::test_all_indexers_ui::decorate_test_result_row_with_style;
    use crate::ui::styles::ManagarrStyle;

    fn indexer_test_result(is_valid: bool) -> IndexerTestResultModalItem {
      IndexerTestResultModalItem {
        name: "Test Indexer".to_owned(),
        is_valid,
        validation_failures: "Unable to connect to indexer".into(),
      }
    }

    #[test]
    fn test_decorate_test_result_row_with_style_when_the_result_is_valid() {
      let row = Row::new(vec![Cell::from("Test Indexer")]);

      let decorated_row =
        decorate_test_result_row_with_style(&indexer_test_result(true), row.clone());

      assert_eq!(decorated_row, row.success());
    }

    #[test]
    fn test_decorate_test_result_row_with_style_when_the_result_is_invalid() {
      let row = Row::new(vec![Cell::from("Test Indexer")]);

      let decorated_row =
        decorate_test_result_row_with_style(&indexer_test_result(false), row.clone());

      assert_eq!(decorated_row, row.failure());
    }

    #[test]
    fn test_decorate_test_result_row_with_style_styles_the_two_branches_differently() {
      let row = Row::new(vec![Cell::from("Test Indexer")]);

      let decorated_row =
        decorate_test_result_row_with_style(&indexer_test_result(true), row.clone());

      assert_ne!(decorated_row, row.failure());
    }
  }

  mod snapshot_tests {
    use crate::models::Scrollable;
    use crate::models::stateful_table::StatefulTable;
    use crate::network::readarr_network::readarr_network_test_utils::test_utils::indexer_test_results;
    use crate::ui::ui_test_utils::test_utils::TerminalSize;

    use super::*;

    fn seed_distinct_test_results(app: &mut App<'_>) {
      let mut indexer_test_all_results = StatefulTable::default();
      indexer_test_all_results.set_items(indexer_test_results());
      app.data.readarr_data.indexer_test_all_results = Some(indexer_test_all_results);
    }

    #[test]
    fn test_test_all_indexers_ui_renders_loading() {
      let mut app = App::test_default_fully_populated();
      seed_distinct_test_results(&mut app);
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::TestAllIndexers.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        TestAllIndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_test_all_indexers_ui_renders_loading_when_results_are_none() {
      let mut app = App::test_default_fully_populated();
      app.data.readarr_data.indexer_test_all_results = None;
      app.push_navigation_stack(ActiveReadarrBlock::TestAllIndexers.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        TestAllIndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_test_all_indexers_ui_renders_empty_results() {
      let mut app = App::test_default_fully_populated();
      app.data.readarr_data.indexer_test_all_results = Some(StatefulTable::default());
      app.push_navigation_stack(ActiveReadarrBlock::TestAllIndexers.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        TestAllIndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_test_all_indexers_ui_renders() {
      let mut app = App::test_default_fully_populated();
      seed_distinct_test_results(&mut app);
      app.push_navigation_stack(ActiveReadarrBlock::TestAllIndexers.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        TestAllIndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_test_all_indexers_ui_renders_for_non_default_selection() {
      let mut app = App::test_default_fully_populated();
      seed_distinct_test_results(&mut app);
      app
        .data
        .readarr_data
        .indexer_test_all_results
        .as_mut()
        .expect("indexer_test_all_results must exist in this context")
        .scroll_down();
      app.push_navigation_stack(ActiveReadarrBlock::TestAllIndexers.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        TestAllIndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
