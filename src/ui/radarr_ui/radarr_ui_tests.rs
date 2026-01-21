#[cfg(test)]
mod tests {
  use crate::models::radarr_models::{DownloadRecord, Movie};
  use pretty_assertions::assert_eq;
  use ratatui::widgets::{Cell, Row};
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::radarr::radarr_data::ActiveRadarrBlock;
  use crate::ui::DrawUi;
  use crate::ui::radarr_ui::{RadarrUi, decorate_with_row_style};
  use crate::ui::styles::ManagarrStyle;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_radarr_ui_accepts() {
    ActiveRadarrBlock::iter().for_each(|active_radarr_block| {
      assert!(RadarrUi::accepts(active_radarr_block.into()));
    });
  }

  mod snapshot_tests {
    use super::*;
    use crate::ui::ui_test_utils::test_utils::TerminalSize;

    #[rstest]
    #[case(ActiveRadarrBlock::Movies, 0)]
    #[case(ActiveRadarrBlock::Collections, 1)]
    #[case(ActiveRadarrBlock::Downloads, 2)]
    #[case(ActiveRadarrBlock::Blocklist, 3)]
    #[case(ActiveRadarrBlock::History, 4)]
    #[case(ActiveRadarrBlock::RootFolders, 5)]
    #[case(ActiveRadarrBlock::Indexers, 6)]
    #[case(ActiveRadarrBlock::System, 7)]
    fn test_radarr_ui_renders_radarr_tabs(
      #[case] active_radarr_block: ActiveRadarrBlock,
      #[case] index: usize,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_radarr_block.into());
      app.data.radarr_data.main_tabs.set_index(index);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        RadarrUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(active_radarr_block.to_string(), output);
    }
  }

  #[rstest]
  #[case(false, Some("downloading"), false, "", RowStyle::Downloading)]
  #[case(false, Some("completed"), false, "", RowStyle::AwaitingImport)]
  #[case(false, None, false, "", RowStyle::UnmonitoredMissing)]
  #[case(false, None, true, "", RowStyle::Unreleased)]
  #[case(false, None, true, "released", RowStyle::Missing)]
  #[case(true, None, false, "", RowStyle::Unmonitored)]
  #[case(true, None, true, "", RowStyle::Downloaded)]
  fn test_decorate_with_row_style(
    #[case] has_file: bool,
    #[case] download_status: Option<&str>,
    #[case] is_monitored: bool,
    #[case] movie_status: String,
    #[case] expected_style: RowStyle,
  ) {
    let downloads_vec = if let Some(download_status) = download_status {
      vec![DownloadRecord {
        movie_id: 1,
        status: download_status.to_owned(),
        ..DownloadRecord::default()
      }]
    } else {
      vec![]
    };
    let movie = Movie {
      id: 1,
      has_file,
      monitored: is_monitored,
      status: movie_status,
      ..Movie::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_with_row_style(&downloads_vec, &movie, row.clone());

    match expected_style {
      RowStyle::AwaitingImport => assert_eq!(style, row.awaiting_import()),
      RowStyle::Downloaded => assert_eq!(style, row.downloaded()),
      RowStyle::Downloading => assert_eq!(style, row.downloading()),
      RowStyle::Missing => assert_eq!(style, row.missing()),
      RowStyle::Unmonitored => assert_eq!(style, row.unmonitored()),
      RowStyle::UnmonitoredMissing => assert_eq!(style, row.unmonitored_missing()),
      RowStyle::Unreleased => assert_eq!(style, row.unreleased()),
    }
  }

  enum RowStyle {
    AwaitingImport,
    Downloaded,
    Downloading,
    Missing,
    Unmonitored,
    UnmonitoredMissing,
    Unreleased,
  }
}
