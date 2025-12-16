#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::BlockSelectionState;
  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, EDIT_INDEXER_BLOCKS, EDIT_INDEXER_TORRENT_SELECTION_BLOCKS,
  };
  use crate::models::servarr_models::Indexer;
  use crate::ui::DrawUi;
  use crate::ui::sonarr_ui::indexers::edit_indexer_ui::EditIndexerUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_edit_indexer_ui_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if EDIT_INDEXER_BLOCKS.contains(&active_sonarr_block) {
        assert!(EditIndexerUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!EditIndexerUi::accepts(active_sonarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use crate::models::servarr_data::sonarr::sonarr_data::EDIT_INDEXER_NZB_SELECTION_BLOCKS;
    use crate::network::sonarr_network::sonarr_network_test_utils::test_utils::indexer;
    use crate::ui::ui_test_utils::test_utils::TerminalSize;

    use super::*;

    #[test]
    fn test_edit_indexer_ui_renders_loading() {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.data.sonarr_data.edit_indexer_modal = None;
      app.push_navigation_stack(ActiveSonarrBlock::EditIndexerPrompt.into());
      app.data.sonarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        EditIndexerUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_edit_indexer_ui_renders_edit_torrent_indexer() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveSonarrBlock::EditIndexerPrompt.into());
      app.data.sonarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        EditIndexerUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_edit_indexer_ui_renders_edit_usenet_indexer() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveSonarrBlock::EditIndexerPrompt.into());
      app.data.sonarr_data.indexers.set_items(vec![Indexer {
        protocol: "usenet".into(),
        ..indexer()
      }]);
      app.data.sonarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_NZB_SELECTION_BLOCKS);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        EditIndexerUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
