#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::readarr::readarr_data::{
    ActiveReadarrBlock, EDIT_INDEXER_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::readarr_ui::indexers::edit_indexer_ui::{
    EditIndexerPromptHighlights, EditIndexerUi, edit_indexer_prompt_highlights,
  };
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_edit_indexer_ui_accepts() {
    ActiveReadarrBlock::iter().for_each(|active_readarr_block| {
      if EDIT_INDEXER_BLOCKS.contains(&active_readarr_block) {
        assert!(EditIndexerUi::accepts(active_readarr_block.into()));
      } else {
        assert!(!EditIndexerUi::accepts(active_readarr_block.into()));
      }
    });
  }

  #[rstest]
  #[case(
    ActiveReadarrBlock::EditIndexerNameInput,
    EditIndexerPromptHighlights {
      name: true,
      url: false,
      api_key: false,
      seed_ratio: false,
      tags: false,
      priority: false,
      rss: false,
      automatic_search: false,
      interactive_search: false,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::EditIndexerUrlInput,
    EditIndexerPromptHighlights {
      name: false,
      url: true,
      api_key: false,
      seed_ratio: false,
      tags: false,
      priority: false,
      rss: false,
      automatic_search: false,
      interactive_search: false,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::EditIndexerApiKeyInput,
    EditIndexerPromptHighlights {
      name: false,
      url: false,
      api_key: true,
      seed_ratio: false,
      tags: false,
      priority: false,
      rss: false,
      automatic_search: false,
      interactive_search: false,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::EditIndexerSeedRatioInput,
    EditIndexerPromptHighlights {
      name: false,
      url: false,
      api_key: false,
      seed_ratio: true,
      tags: false,
      priority: false,
      rss: false,
      automatic_search: false,
      interactive_search: false,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::EditIndexerTagsInput,
    EditIndexerPromptHighlights {
      name: false,
      url: false,
      api_key: false,
      seed_ratio: false,
      tags: true,
      priority: false,
      rss: false,
      automatic_search: false,
      interactive_search: false,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::EditIndexerPriorityInput,
    EditIndexerPromptHighlights {
      name: false,
      url: false,
      api_key: false,
      seed_ratio: false,
      tags: false,
      priority: true,
      rss: false,
      automatic_search: false,
      interactive_search: false,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::EditIndexerToggleEnableRss,
    EditIndexerPromptHighlights {
      name: false,
      url: false,
      api_key: false,
      seed_ratio: false,
      tags: false,
      priority: false,
      rss: true,
      automatic_search: false,
      interactive_search: false,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::EditIndexerToggleEnableAutomaticSearch,
    EditIndexerPromptHighlights {
      name: false,
      url: false,
      api_key: false,
      seed_ratio: false,
      tags: false,
      priority: false,
      rss: false,
      automatic_search: true,
      interactive_search: false,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::EditIndexerToggleEnableInteractiveSearch,
    EditIndexerPromptHighlights {
      name: false,
      url: false,
      api_key: false,
      seed_ratio: false,
      tags: false,
      priority: false,
      rss: false,
      automatic_search: false,
      interactive_search: true,
      confirm: false,
    }
  )]
  #[case(
    ActiveReadarrBlock::EditIndexerConfirmPrompt,
    EditIndexerPromptHighlights {
      name: false,
      url: false,
      api_key: false,
      seed_ratio: false,
      tags: false,
      priority: false,
      rss: false,
      automatic_search: false,
      interactive_search: false,
      confirm: true,
    }
  )]
  #[case(
    ActiveReadarrBlock::EditIndexerPrompt,
    EditIndexerPromptHighlights {
      name: false,
      url: false,
      api_key: false,
      seed_ratio: false,
      tags: false,
      priority: false,
      rss: false,
      automatic_search: false,
      interactive_search: false,
      confirm: false,
    }
  )]
  fn test_edit_indexer_prompt_highlights(
    #[case] selected_block: ActiveReadarrBlock,
    #[case] expected_highlights: EditIndexerPromptHighlights,
  ) {
    let highlights = edit_indexer_prompt_highlights(selected_block);

    assert_eq!(highlights, expected_highlights);
  }

  mod snapshot_tests {
    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::readarr::readarr_data::{
      EDIT_INDEXER_NZB_SELECTION_BLOCKS, EDIT_INDEXER_TORRENT_SELECTION_BLOCKS,
    };
    use crate::models::servarr_models::Indexer;
    use crate::network::readarr_network::readarr_network_test_utils::test_utils::indexer;
    use crate::ui::ui_test_utils::test_utils::TerminalSize;

    use super::*;

    fn seed_asymmetric_enable_flags(
      app: &mut App<'_>,
      enable_rss: bool,
      enable_automatic_search: bool,
      enable_interactive_search: bool,
    ) {
      let edit_indexer_modal = app
        .data
        .readarr_data
        .edit_indexer_modal
        .as_mut()
        .expect("edit_indexer_modal must exist in this context");
      edit_indexer_modal.enable_rss = Some(enable_rss);
      edit_indexer_modal.enable_automatic_search = Some(enable_automatic_search);
      edit_indexer_modal.enable_interactive_search = Some(enable_interactive_search);
    }

    fn seed_indexer_protocol(app: &mut App<'_>, protocol: &str) {
      app.data.readarr_data.indexers.set_items(vec![Indexer {
        protocol: protocol.to_owned(),
        ..indexer()
      }]);
    }

    #[test]
    fn test_edit_indexer_ui_renders_loading() {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.data.readarr_data.edit_indexer_modal = None;
      app.data.readarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);
      app.push_navigation_stack(ActiveReadarrBlock::EditIndexerPrompt.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        EditIndexerUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    #[case(ActiveReadarrBlock::EditIndexerPrompt, 0, 0)]
    #[case(ActiveReadarrBlock::EditIndexerNameInput, 0, 0)]
    #[case(ActiveReadarrBlock::EditIndexerUrlInput, 1, 0)]
    #[case(ActiveReadarrBlock::EditIndexerToggleEnableRss, 0, 1)]
    #[case(ActiveReadarrBlock::EditIndexerApiKeyInput, 1, 1)]
    #[case(ActiveReadarrBlock::EditIndexerToggleEnableAutomaticSearch, 0, 2)]
    #[case(ActiveReadarrBlock::EditIndexerSeedRatioInput, 1, 2)]
    #[case(ActiveReadarrBlock::EditIndexerToggleEnableInteractiveSearch, 0, 3)]
    #[case(ActiveReadarrBlock::EditIndexerTagsInput, 1, 3)]
    #[case(ActiveReadarrBlock::EditIndexerPriorityInput, 0, 4)]
    #[case(ActiveReadarrBlock::EditIndexerConfirmPrompt, 1, 4)]
    fn test_edit_indexer_ui_renders_torrent_grid(
      #[case] active_readarr_block: ActiveReadarrBlock,
      #[case] x: usize,
      #[case] y: usize,
    ) {
      let mut app = App::test_default_fully_populated();
      seed_asymmetric_enable_flags(&mut app, true, false, true);
      seed_indexer_protocol(&mut app, "torrent");
      let mut selected_block = BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);
      selected_block.set_index(x, y);
      app.data.readarr_data.selected_block = selected_block;
      app.push_navigation_stack(active_readarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        EditIndexerUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("edit_indexer_torrent_{active_readarr_block}"),
        output
      );
    }

    #[test]
    fn test_edit_indexer_ui_renders_usenet_grid() {
      let mut app = App::test_default_fully_populated();
      seed_asymmetric_enable_flags(&mut app, false, true, true);
      seed_indexer_protocol(&mut app, "usenet");
      app.data.readarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_NZB_SELECTION_BLOCKS);
      app.push_navigation_stack(ActiveReadarrBlock::EditIndexerPrompt.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        EditIndexerUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
