#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, DOWNLOADS_BLOCKS};
  use crate::ui::DrawUi;
  use crate::ui::sonarr_ui::downloads::DownloadsUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_downloads_ui_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if DOWNLOADS_BLOCKS.contains(&active_sonarr_block) {
        assert!(DownloadsUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!DownloadsUi::accepts(active_sonarr_block.into()));
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
      app.push_navigation_stack(ActiveSonarrBlock::Downloads.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        DownloadsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_downloads_ui_renders_empty_downloads() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Downloads.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        DownloadsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    fn test_downloads_ui_renders(
      #[values(
        ActiveSonarrBlock::Downloads,
        ActiveSonarrBlock::DeleteDownloadPrompt,
        ActiveSonarrBlock::UpdateDownloadsPrompt
      )]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_sonarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        DownloadsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("downloads_ui_{active_sonarr_block}"), output);
    }
  }
}
