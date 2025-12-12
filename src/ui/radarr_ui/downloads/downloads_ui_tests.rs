#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::radarr_models::DownloadRecord;
  use crate::models::servarr_data::radarr::radarr_data::{ActiveRadarrBlock, DOWNLOADS_BLOCKS};
  use crate::models::stateful_table::StatefulTable;
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::downloads::DownloadsUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

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

  #[test]
  fn test_downloads_ui_renders_loading_state() {
    let mut app = App::test_default();
    app.is_loading = true;
    app.push_navigation_stack(ActiveRadarrBlock::Downloads.into());

    let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
      DownloadsUi::draw(f, app, f.area());
    });

    insta::assert_snapshot!(output);
  }

  #[test]
  fn test_downloads_ui_renders_empty_downloads() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveRadarrBlock::Downloads.into());
    app.data.radarr_data.downloads = StatefulTable::default();

    let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
      DownloadsUi::draw(f, app, f.area());
    });

    insta::assert_snapshot!(output);
  }

  #[test]
  fn test_downloads_ui_renders_with_downloads() {
    let mut app = App::test_default();
    app.push_navigation_stack(ActiveRadarrBlock::Downloads.into());
    app.data.radarr_data.downloads = StatefulTable::default();
    app.data.radarr_data.downloads.set_items(vec![
      DownloadRecord {
        id: 1,
        movie_id: 1,
        title: "Test Movie Download".to_owned(),
        status: "downloading".to_owned(),
        size: 1024 * 1024 * 1024,
        sizeleft: 512 * 1024 * 1024,
        ..DownloadRecord::default()
      },
      DownloadRecord {
        id: 2,
        movie_id: 2,
        title: "Another Movie Download".to_owned(),
        status: "completed".to_owned(),
        size: 2048 * 1024 * 1024,
        sizeleft: 0,
        ..DownloadRecord::default()
      },
    ]);

    let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
      DownloadsUi::draw(f, app, f.area());
    });

    insta::assert_snapshot!(output);
  }
}
