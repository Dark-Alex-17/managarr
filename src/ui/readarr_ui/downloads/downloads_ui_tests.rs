#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::readarr::readarr_data::{ActiveReadarrBlock, DOWNLOADS_BLOCKS};
  use crate::ui::DrawUi;
  use crate::ui::readarr_ui::downloads::DownloadsUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_downloads_ui_accepts() {
    ActiveReadarrBlock::iter().for_each(|active_readarr_block| {
      if DOWNLOADS_BLOCKS.contains(&active_readarr_block) {
        assert!(DownloadsUi::accepts(active_readarr_block.into()));
      } else {
        assert!(!DownloadsUi::accepts(active_readarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use rstest::rstest;
    use serde_json::Number;

    use crate::models::readarr_models::{DownloadRecord, DownloadStatus};
    use crate::ui::ui_test_utils::test_utils::TerminalSize;

    use super::*;

    #[test]
    fn test_downloads_ui_renders_loading() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveReadarrBlock::Downloads.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        DownloadsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_downloads_ui_renders_empty() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveReadarrBlock::Downloads.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        DownloadsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[rstest]
    fn test_downloads_ui_renders(
      #[values(
        ActiveReadarrBlock::Downloads,
        ActiveReadarrBlock::DeleteDownloadPrompt,
        ActiveReadarrBlock::UpdateDownloadsPrompt
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_readarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        DownloadsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("downloads_ui_{active_readarr_block}"), output);
    }

    #[rstest]
    fn test_downloads_ui_renders_second_item_selected(
      #[values(
        ActiveReadarrBlock::Downloads,
        ActiveReadarrBlock::DeleteDownloadPrompt
      )]
      active_readarr_block: ActiveReadarrBlock,
    ) {
      let mut app = App::test_default();
      app.data.readarr_data.downloads.set_items(downloads_vec());
      app.data.readarr_data.downloads.select_index(Some(1));
      app.push_navigation_stack(active_readarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        DownloadsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("downloads_second_item_selected_{active_readarr_block}"),
        output
      );
    }

    fn downloads_vec() -> Vec<DownloadRecord> {
      vec![
        DownloadRecord {
          id: 1,
          title: "First Download Title".to_owned(),
          status: DownloadStatus::Downloading,
          book_id: Some(Number::from(1)),
          author_id: Some(Number::from(1)),
          size: 3543348019f64,
          sizeleft: 1771674009f64,
          output_path: Some("/nfs/books/First Author".into()),
          indexer: "DrunkenSlug".to_owned(),
          download_client: Some("sabnzbd".to_owned()),
        },
        DownloadRecord {
          id: 2,
          title: "Second Download Title".to_owned(),
          status: DownloadStatus::Paused,
          book_id: Some(Number::from(2)),
          author_id: Some(Number::from(2)),
          size: 1073741824f64,
          sizeleft: 268435456f64,
          output_path: Some("/nfs/books/Second Author".into()),
          indexer: "Nyaa".to_owned(),
          download_client: Some("transmission".to_owned()),
        },
      ]
    }
  }
}
