#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, BLOCKLIST_BLOCKS};
  use crate::ui::DrawUi;
  use crate::ui::lidarr_ui::blocklist::BlocklistUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_blocklist_ui_accepts() {
    ActiveLidarrBlock::iter().for_each(|active_lidarr_block| {
      if BLOCKLIST_BLOCKS.contains(&active_lidarr_block) {
        assert!(BlocklistUi::accepts(active_lidarr_block.into()));
      } else {
        assert!(!BlocklistUi::accepts(active_lidarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use crate::ui::ui_test_utils::test_utils::TerminalSize;
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_blocklist_ui_renders_loading() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveLidarrBlock::Blocklist.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        BlocklistUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_blocklist_ui_renders_empty() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Blocklist.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        BlocklistUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    fn test_blocklist_ui_renders(
      #[values(
        ActiveLidarrBlock::Blocklist,
        ActiveLidarrBlock::BlocklistItemDetails,
        ActiveLidarrBlock::DeleteBlocklistItemPrompt,
        ActiveLidarrBlock::BlocklistClearAllItemsPrompt,
        ActiveLidarrBlock::BlocklistSortPrompt
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_lidarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        BlocklistUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("blocklist_tab_{active_lidarr_block}"), output);
    }
  }
}
