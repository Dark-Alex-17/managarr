#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::radarr::radarr_data::{
    ActiveRadarrBlock, EDIT_INDEXER_BLOCKS, INDEXER_SETTINGS_BLOCKS, INDEXERS_BLOCKS,
  };
  use crate::models::stateful_table::StatefulTable;
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::indexers::IndexersUi;
  use crate::ui::ui_test_utils::test_utils::{TerminalSize, render_to_string_with_app};

  #[test]
  fn test_indexers_ui_accepts() {
    let mut indexers_blocks = Vec::new();
    indexers_blocks.extend(INDEXERS_BLOCKS);
    indexers_blocks.extend(INDEXER_SETTINGS_BLOCKS);
    indexers_blocks.extend(EDIT_INDEXER_BLOCKS);
    indexers_blocks.push(ActiveRadarrBlock::TestAllIndexers);

    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if indexers_blocks.contains(&active_radarr_block) {
        assert!(IndexersUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!IndexersUi::accepts(active_radarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use super::*;

    #[test]
    fn test_indexers_ui_renders_indexers_tab_loading() {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        IndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_indexers_ui_renders_indexers_tab_empty_indexers() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());
      app.data.radarr_data.indexers = StatefulTable::default();

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        IndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_indexers_ui_renders_indexers_tab() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveRadarrBlock::Indexers.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        IndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_indexers_ui_renders_test_indexer_loading() {
      let mut app = App::test_default_fully_populated();
      app.data.radarr_data.indexer_test_errors = None;
      app.push_navigation_stack(ActiveRadarrBlock::TestIndexer.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        IndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_indexers_ui_renders_test_indexer_success() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveRadarrBlock::TestIndexer.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        IndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_indexers_ui_renders_test_indexer_error() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveRadarrBlock::TestIndexer.into());

      app.data.radarr_data.indexer_test_errors =
        Some("Connection timeout: Unable to reach indexer".to_owned());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        IndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_indexers_ui_renders_delete_indexer_prompt() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveRadarrBlock::DeleteIndexerPrompt.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        IndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
