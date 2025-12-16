#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, EDIT_INDEXER_BLOCKS, INDEXER_SETTINGS_BLOCKS, INDEXERS_BLOCKS,
  };
  use crate::models::servarr_models::Indexer;
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
    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::sonarr::sonarr_data::{
      EDIT_INDEXER_NZB_SELECTION_BLOCKS, EDIT_INDEXER_TORRENT_SELECTION_BLOCKS,
    };
    use crate::network::sonarr_network::sonarr_network_test_utils::test_utils::indexer;
    use crate::ui::ui_test_utils::test_utils::TerminalSize;
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_indexers_ui_renders_loading() {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        IndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_indexers_ui_renders_loading_test_results() {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::TestIndexer.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        IndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_indexers_ui_renders_loading_test_results_when_indexer_test_errors_is_none() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveSonarrBlock::TestIndexer.into());
      app.data.sonarr_data.indexer_test_errors = None;

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        IndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_indexers_ui_renders_empty_indexers() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Indexers.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        IndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    fn test_indexers_ui_renders(
      #[values(
        ActiveSonarrBlock::DeleteIndexerPrompt,
        ActiveSonarrBlock::Indexers,
        ActiveSonarrBlock::TestIndexer
      )]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_sonarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        IndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("indexers_ui_{active_sonarr_block}"), output);
    }

    #[test]
    fn test_indexers_ui_renders_test_all_over_indexers() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveSonarrBlock::TestAllIndexers.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        IndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_indexers_ui_renders_edit_usenet_indexer_over_indexers() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveSonarrBlock::EditIndexerPrompt.into());
      app.data.sonarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_NZB_SELECTION_BLOCKS);
      app.data.sonarr_data.indexers.set_items(vec![Indexer {
        protocol: "usenet".into(),
        ..indexer()
      }]);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        IndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_indexers_ui_renders_edit_torrent_indexer_over_indexers() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveSonarrBlock::EditIndexerPrompt.into());
      app.data.sonarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        IndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
