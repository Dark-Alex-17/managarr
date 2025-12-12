#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::BlockSelectionState;
  use crate::models::radarr_models::IndexerSettings;
  use crate::models::servarr_data::radarr::radarr_data::{
    ActiveRadarrBlock, INDEXER_SETTINGS_BLOCKS, INDEXER_SETTINGS_SELECTION_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::indexers::indexer_settings_ui::IndexerSettingsUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_indexer_settings_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if INDEXER_SETTINGS_BLOCKS.contains(&active_radarr_block) {
        assert!(IndexerSettingsUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!IndexerSettingsUi::accepts(active_radarr_block.into()));
      }
    });
  }

  #[test]
  fn test_indexer_settings_ui_renders_indexer_settings() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveRadarrBlock::IndexerSettingsMinimumAgeInput.into());
    app.data.radarr_data.selected_block =
      BlockSelectionState::new(INDEXER_SETTINGS_SELECTION_BLOCKS);
    app.data.radarr_data.indexer_settings = Some(IndexerSettings::default());

    let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
      IndexerSettingsUi::draw(f, app, f.area());
    });

    insta::assert_snapshot!(output);
  }
}
