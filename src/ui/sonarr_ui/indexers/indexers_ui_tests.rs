#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, EDIT_INDEXER_BLOCKS, INDEXER_SETTINGS_BLOCKS, INDEXERS_BLOCKS,
  };
  use crate::models::servarr_models::Indexer;
  use crate::models::stateful_table::StatefulTable;
  use crate::ui::DrawUi;
  use crate::ui::sonarr_ui::indexers::IndexersUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_indexers_ui_accepts() {
    let mut indexers_blocks = Vec::new();
    indexers_blocks.extend(INDEXERS_BLOCKS);
    indexers_blocks.extend(INDEXER_SETTINGS_BLOCKS);
    indexers_blocks.extend(EDIT_INDEXER_BLOCKS);
    indexers_blocks.push(ActiveSonarrBlock::TestAllIndexers);

    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if indexers_blocks.contains(&active_sonarr_block) {
        assert!(IndexersUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!IndexersUi::accepts(active_sonarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use super::*;

    #[test]
    fn test_indexers_ui_renders_loading_state() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());

      let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
        IndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_indexers_ui_renders_empty_indexers() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
      app.data.sonarr_data.indexers = StatefulTable::default();

      let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
        IndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_indexers_ui_renders_with_indexers() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());
      app.data.sonarr_data.indexers = StatefulTable::default();
      app.data.sonarr_data.indexers.set_items(vec![
        Indexer {
          id: 1,
          name: Some("Test Indexer 1".to_owned()),
          enable_rss: true,
          enable_automatic_search: true,
          enable_interactive_search: true,
          priority: 25,
          ..Indexer::default()
        },
        Indexer {
          id: 2,
          name: Some("Test Indexer 2".to_owned()),
          enable_rss: false,
          ..Indexer::default()
        },
      ]);

      let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
        IndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
