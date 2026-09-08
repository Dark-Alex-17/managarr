#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::readarr::readarr_data::{
    ActiveReadarrBlock, EDIT_INDEXER_BLOCKS, INDEXER_SETTINGS_BLOCKS, INDEXERS_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::readarr_ui::indexers::IndexersUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_indexers_ui_accepts() {
    let mut indexers_blocks = Vec::new();
    indexers_blocks.extend(INDEXERS_BLOCKS);
    indexers_blocks.extend(INDEXER_SETTINGS_BLOCKS);
    indexers_blocks.extend(EDIT_INDEXER_BLOCKS);
    indexers_blocks.push(ActiveReadarrBlock::TestAllIndexers);

    ActiveReadarrBlock::iter().for_each(|active_readarr_block| {
      if indexers_blocks.contains(&active_readarr_block) {
        assert!(IndexersUi::accepts(active_readarr_block.into()));
      } else {
        assert!(!IndexersUi::accepts(active_readarr_block.into()));
      }
    });
  }

  mod test_indexer_flag_text {
    use pretty_assertions::assert_eq;
    use ratatui::text::Text;

    use crate::ui::readarr_ui::indexers::indexer_flag_text;
    use crate::ui::styles::ManagarrStyle;

    #[test]
    fn test_indexer_flag_text_when_the_flag_is_enabled() {
      let flag_text = indexer_flag_text(true);

      assert_eq!(flag_text, Text::from("Enabled").success());
    }

    #[test]
    fn test_indexer_flag_text_when_the_flag_is_disabled() {
      let flag_text = indexer_flag_text(false);

      assert_eq!(flag_text, Text::from("Disabled").failure());
    }

    #[test]
    fn test_indexer_flag_text_styles_the_two_branches_differently() {
      let enabled_flag_text = indexer_flag_text(true);

      assert_ne!(enabled_flag_text, Text::from("Enabled").failure());
    }
  }

  mod snapshot_tests {
    use crate::models::BlockSelectionState;
    use crate::models::Scrollable;
    use crate::models::servarr_data::readarr::readarr_data::{
      EDIT_INDEXER_NZB_SELECTION_BLOCKS, EDIT_INDEXER_TORRENT_SELECTION_BLOCKS,
      INDEXER_SETTINGS_SELECTION_BLOCKS,
    };
    use crate::models::servarr_models::Indexer;
    use crate::network::readarr_network::readarr_network_test_utils::test_utils::{
      indexer, indexer_settings,
    };
    use crate::ui::ui_test_utils::test_utils::TerminalSize;
    use rstest::rstest;
    use serde_json::Number;

    use super::*;

    fn seed_distinct_indexers(app: &mut App<'_>) {
      app
        .data
        .readarr_data
        .tags_map
        .insert(2, "usenet-only".to_owned());
      app.data.readarr_data.indexers.set_items(vec![
        Indexer {
          id: 3,
          name: Some("Decoy Usenet Indexer".to_owned()),
          protocol: "usenet".to_owned(),
          enable_rss: false,
          enable_automatic_search: true,
          enable_interactive_search: true,
          priority: 7,
          tags: vec![Number::from(2)],
          ..indexer()
        },
        Indexer {
          id: 8,
          name: Some("Test Torrent Indexer".to_owned()),
          protocol: "torrent".to_owned(),
          enable_rss: true,
          enable_automatic_search: true,
          enable_interactive_search: false,
          priority: 42,
          tags: vec![Number::from(1)],
          ..indexer()
        },
      ]);
    }

    #[test]
    fn test_indexers_ui_renders_loading() {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::Indexers.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        IndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_indexers_ui_renders_empty_indexers() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Indexers.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        IndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_indexers_ui_renders_loading_test_results() {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::TestIndexer.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        IndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_indexers_ui_renders_loading_test_results_when_indexer_test_errors_is_none() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveReadarrBlock::TestIndexer.into());
      app.data.readarr_data.indexer_test_errors = None;

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        IndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_indexers_ui_renders_test_indexer_success_when_there_are_no_errors() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveReadarrBlock::TestIndexer.into());
      app.data.readarr_data.indexer_test_errors = Some(String::new());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        IndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    fn test_indexers_ui_renders(
      #[values(
        ActiveReadarrBlock::Indexers,
        ActiveReadarrBlock::DeleteIndexerPrompt,
        ActiveReadarrBlock::TestIndexer
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      seed_distinct_indexers(&mut app);
      app.push_navigation_stack(active_readarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        IndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("indexers_ui_{active_readarr_block}"), output);
    }

    #[test]
    fn test_indexers_ui_renders_delete_prompt_for_non_default_selection() {
      let mut app = App::test_default_fully_populated();
      seed_distinct_indexers(&mut app);
      app.data.readarr_data.indexers.scroll_down();
      app.push_navigation_stack(ActiveReadarrBlock::DeleteIndexerPrompt.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        IndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_indexers_ui_renders_edit_usenet_indexer_over_indexers() {
      let mut app = App::test_default_fully_populated();
      seed_distinct_indexers(&mut app);
      app.data.readarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_NZB_SELECTION_BLOCKS);
      app.push_navigation_stack(ActiveReadarrBlock::EditIndexerPrompt.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        IndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_indexers_ui_renders_edit_torrent_indexer_over_indexers() {
      let mut app = App::test_default_fully_populated();
      seed_distinct_indexers(&mut app);
      app.data.readarr_data.indexers.scroll_down();
      app.data.readarr_data.selected_block =
        BlockSelectionState::new(EDIT_INDEXER_TORRENT_SELECTION_BLOCKS);
      app.push_navigation_stack(ActiveReadarrBlock::EditIndexerPrompt.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        IndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_indexers_ui_renders_indexer_settings_over_indexers() {
      let mut app = App::test_default_fully_populated();
      seed_distinct_indexers(&mut app);
      app.data.readarr_data.indexer_settings = Some(indexer_settings());
      app.data.readarr_data.selected_block =
        BlockSelectionState::new(INDEXER_SETTINGS_SELECTION_BLOCKS);
      app.push_navigation_stack(ActiveReadarrBlock::AllIndexerSettingsPrompt.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        IndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_indexers_ui_renders_test_all_indexers_over_indexers() {
      let mut app = App::test_default_fully_populated();
      seed_distinct_indexers(&mut app);
      app.push_navigation_stack(ActiveReadarrBlock::TestAllIndexers.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        IndexersUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
