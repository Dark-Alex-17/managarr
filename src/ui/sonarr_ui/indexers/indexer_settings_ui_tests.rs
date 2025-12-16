#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::BlockSelectionState;
  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, INDEXER_SETTINGS_BLOCKS, INDEXER_SETTINGS_SELECTION_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::sonarr_ui::indexers::indexer_settings_ui::IndexerSettingsUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_indexer_settings_ui_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if INDEXER_SETTINGS_BLOCKS.contains(&active_sonarr_block) {
        assert!(IndexerSettingsUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!IndexerSettingsUi::accepts(active_sonarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use crate::ui::ui_test_utils::test_utils::TerminalSize;

    use super::*;

    #[test]
    fn test_indexer_settings_ui_renders_indexer_settings() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveSonarrBlock::IndexerSettingsMinimumAgeInput.into());
      app.data.sonarr_data.selected_block =
        BlockSelectionState::new(INDEXER_SETTINGS_SELECTION_BLOCKS);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        IndexerSettingsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
