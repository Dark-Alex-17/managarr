#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::sonarr::sonarr_data::{ActiveSonarrBlock, DOWNLOADS_BLOCKS};
  use crate::models::sonarr_models::DownloadRecord;
  use crate::models::stateful_table::StatefulTable;
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

    use super::*;

    #[test]
    fn test_downloads_ui_renders_loading_state() {
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
      app.data.sonarr_data.downloads = StatefulTable::default();

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        DownloadsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_downloads_ui_renders_with_downloads() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Downloads.into());
      app.data.sonarr_data.downloads = StatefulTable::default();
      app.data.sonarr_data.downloads.set_items(vec![
        DownloadRecord {
          id: 1,
          title: "Test Series Download".to_owned(),
          status: Default::default(),
          size: 1024.0 * 1024.0 * 1024.0,
          sizeleft: 512.0 * 1024.0 * 1024.0,
          ..DownloadRecord::default()
        },
        DownloadRecord {
          id: 2,
          title: "Another Series Download".to_owned(),
          status: Default::default(),
          size: 2048.0 * 1024.0 * 1024.0,
          sizeleft: 0.0,
          ..DownloadRecord::default()
        },
      ]);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        DownloadsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
