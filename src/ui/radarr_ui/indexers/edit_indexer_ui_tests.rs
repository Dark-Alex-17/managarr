#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::BlockSelectionState;
  use crate::models::servarr_data::radarr::radarr_data::{
    ActiveRadarrBlock, EDIT_INDEXER_BLOCKS, EDIT_INDEXER_TORRENT_SELECTION_BLOCKS,
  };
  use crate::models::servarr_models::{Indexer};
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::indexers::edit_indexer_ui::EditIndexerUi;
  use crate::ui::ui_test_utils::test_utils::{TerminalSize, render_to_string_with_app};

  #[test]
  fn test_edit_indexer_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if EDIT_INDEXER_BLOCKS.contains(&active_radarr_block) {
        assert!(EditIndexerUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!EditIndexerUi::accepts(active_radarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use crate::models::servarr_data::radarr::radarr_data::EDIT_INDEXER_NZB_SELECTION_BLOCKS;
    use crate::network::radarr_network::radarr_network_test_utils::test_utils::indexer;
    use super::*;

    #[test]
    fn test_edit_indexer_ui_renders_edit_indexer_modal_torrent() {
      let mut app = App::test_default_fully_populated();
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);
      app.push_navigation_stack(ActiveRadarrBlock::EditIndexerPrompt.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        EditIndexerUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_edit_indexer_ui_renders_edit_indexer_modal_usenet() {
      let mut app = App::test_default_fully_populated();
      app.data.radarr_data.indexers.set_items(vec![Indexer {
        protocol: "usenet".to_owned(),
        ..indexer()
      }]);
      app.data.radarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_NZB_SELECTION_BLOCKS);
      app.push_navigation_stack(ActiveRadarrBlock::EditIndexerPrompt.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        EditIndexerUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}