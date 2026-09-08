#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::BlockSelectionState;
  use crate::models::servarr_data::readarr::readarr_data::{
    ActiveReadarrBlock, INDEXER_SETTINGS_BLOCKS, INDEXER_SETTINGS_SELECTION_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::readarr_ui::indexers::indexer_settings_ui::{
    IndexerSettingsPromptHighlights, IndexerSettingsUi, indexer_settings_prompt_highlights,
  };
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_indexer_settings_ui_accepts() {
    ActiveReadarrBlock::iter().for_each(|active_readarr_block| {
      if INDEXER_SETTINGS_BLOCKS.contains(&active_readarr_block) {
        assert!(IndexerSettingsUi::accepts(active_readarr_block.into()));
      } else {
        assert!(!IndexerSettingsUi::accepts(active_readarr_block.into()));
      }
    });
  }

  #[rstest]
  #[case(
    ActiveReadarrBlock::IndexerSettingsMinimumAgeInput,
    IndexerSettingsPromptHighlights {
      minimum_age: true,
      retention: false,
      maximum_size: false,
      rss_sync_interval: false,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::IndexerSettingsRetentionInput,
    IndexerSettingsPromptHighlights {
      minimum_age: false,
      retention: true,
      maximum_size: false,
      rss_sync_interval: false,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::IndexerSettingsMaximumSizeInput,
    IndexerSettingsPromptHighlights {
      minimum_age: false,
      retention: false,
      maximum_size: true,
      rss_sync_interval: false,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::IndexerSettingsRssSyncIntervalInput,
    IndexerSettingsPromptHighlights {
      minimum_age: false,
      retention: false,
      maximum_size: false,
      rss_sync_interval: true,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::IndexerSettingsConfirmPrompt,
    IndexerSettingsPromptHighlights {
      minimum_age: false,
      retention: false,
      maximum_size: false,
      rss_sync_interval: false,
      confirm: true,
    }
  )]
  #[case(
    ActiveReadarrBlock::AllIndexerSettingsPrompt,
    IndexerSettingsPromptHighlights {
      minimum_age: false,
      retention: false,
      maximum_size: false,
      rss_sync_interval: false,
      confirm: false,
    }
  )]
  fn test_indexer_settings_prompt_highlights(
    #[case] selected_block: ActiveReadarrBlock,
    #[case] expected_highlights: IndexerSettingsPromptHighlights,
  ) {
    let highlights = indexer_settings_prompt_highlights(selected_block);

    assert_eq!(highlights, expected_highlights);
  }

  #[test]
  fn test_indexer_settings_prompt_highlights_covers_every_selection_step() {
    for step in INDEXER_SETTINGS_SELECTION_BLOCKS {
      let highlights = indexer_settings_prompt_highlights(step[0]);

      assert_ne!(
        highlights,
        indexer_settings_prompt_highlights(ActiveReadarrBlock::AllIndexerSettingsPrompt)
      );
    }
  }

  mod snapshot_tests {
    use crate::network::readarr_network::readarr_network_test_utils::test_utils::indexer_settings;
    use crate::ui::ui_test_utils::test_utils::TerminalSize;

    use super::*;

    fn seed_distinct_indexer_settings(app: &mut App<'_>) {
      app.data.readarr_data.indexer_settings = Some(indexer_settings());
    }

    #[test]
    fn test_indexer_settings_ui_renders_loading() {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.data.readarr_data.indexer_settings = None;
      app.data.readarr_data.selected_block =
        BlockSelectionState::new(INDEXER_SETTINGS_SELECTION_BLOCKS);
      app.push_navigation_stack(ActiveReadarrBlock::AllIndexerSettingsPrompt.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        IndexerSettingsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    fn test_indexer_settings_ui_renders(
      #[values(
        ActiveReadarrBlock::AllIndexerSettingsPrompt,
        ActiveReadarrBlock::IndexerSettingsConfirmPrompt,
        ActiveReadarrBlock::IndexerSettingsMaximumSizeInput,
        ActiveReadarrBlock::IndexerSettingsMinimumAgeInput,
        ActiveReadarrBlock::IndexerSettingsRetentionInput,
        ActiveReadarrBlock::IndexerSettingsRssSyncIntervalInput
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      seed_distinct_indexer_settings(&mut app);
      app.data.readarr_data.selected_block =
        BlockSelectionState::new(INDEXER_SETTINGS_SELECTION_BLOCKS);
      app.push_navigation_stack(active_readarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        IndexerSettingsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("indexer_settings_{active_readarr_block}"), output);
    }
  }
}
