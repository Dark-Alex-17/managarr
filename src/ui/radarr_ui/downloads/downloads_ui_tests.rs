#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, DOWNLOADS_BLOCKS};
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::downloads::DownloadsUi;
  use crate::ui::ui_test_utils::test_utils::{TerminalSize, render_to_string_with_app};

  #[test]
  fn test_downloads_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      if DOWNLOADS_BLOCKS.contains(&active_radarr_block) {
        assert!(DownloadsUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!DownloadsUi::accepts(active_radarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use rstest::rstest;
    use super::*;

    #[test]
    fn test_radarr_ui_renders_downloads_tab_loading() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveRadarrBlock::Downloads.into());
      app.is_loading = true;

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        DownloadsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    fn test_radarr_ui_renders_downloads_tab(
      #[values(
        ActiveRadarrBlock::Downloads,
        ActiveRadarrBlock::DeleteDownloadPrompt,
        ActiveRadarrBlock::UpdateDownloadsPrompt,
      )] active_radarr_block: ActiveRadarrBlock
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_radarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        DownloadsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(active_radarr_block.to_string(), output);
    }

    #[test]
    fn test_radarr_ui_renders_downloads_tab_empty() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveRadarrBlock::Downloads.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        DownloadsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
