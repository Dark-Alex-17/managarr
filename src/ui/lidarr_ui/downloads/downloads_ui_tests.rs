#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::lidarr::lidarr_data::{ActiveLidarrBlock, DOWNLOADS_BLOCKS};
  use crate::ui::DrawUi;
  use crate::ui::lidarr_ui::downloads::DownloadsUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_downloads_ui_accepts() {
    ActiveLidarrBlock::iter().for_each(|active_lidarr_block| {
      if DOWNLOADS_BLOCKS.contains(&active_lidarr_block) {
        assert!(DownloadsUi::accepts(active_lidarr_block.into()));
      } else {
        assert!(!DownloadsUi::accepts(active_lidarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use crate::ui::ui_test_utils::test_utils::TerminalSize;
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_downloads_ui_renders_loading() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveLidarrBlock::Downloads.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        DownloadsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_downloads_ui_renders_empty_downloads() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveLidarrBlock::Downloads.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        DownloadsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    fn test_downloads_ui_renders(
      #[values(
        ActiveLidarrBlock::Downloads,
        ActiveLidarrBlock::DeleteDownloadPrompt,
        ActiveLidarrBlock::UpdateDownloadsPrompt
      )]
      active_lidarr_block: ActiveLidarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_lidarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        DownloadsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("downloads_ui_{active_lidarr_block}"), output);
    }
  }
}
