#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, BLOCKLIST_BLOCKS};
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::blocklist::BlocklistUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_blocklist_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if BLOCKLIST_BLOCKS.contains(&active_radarr_block) {
        assert!(BlocklistUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!BlocklistUi::accepts(active_radarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use crate::ui::ui_test_utils::test_utils::TerminalSize;
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_blocklist_ui_renders_blocklist_tab_loading() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveRadarrBlock::Blocklist.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        BlocklistUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_blocklist_ui_renders_empty_blocklist() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::Blocklist.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        BlocklistUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    fn test_blocklist_ui_renders_blocklist_tab(
      #[values(
        ActiveRadarrBlock::Blocklist,
        ActiveRadarrBlock::BlocklistSortPrompt,
        ActiveRadarrBlock::DeleteBlocklistItemPrompt,
        ActiveRadarrBlock::BlocklistClearAllItemsPrompt
      )]
      active_radarr_block: ActiveRadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_radarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        BlocklistUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(active_radarr_block.to_string(), output);
    }
  }
}
