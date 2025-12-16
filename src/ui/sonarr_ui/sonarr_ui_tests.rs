#[cfg(test)]
mod tests {
  use bimap::BiMap;
  use strum::IntoEnumIterator;

  use crate::{
    app::App,
    models::{
      servarr_data::sonarr::sonarr_data::ActiveSonarrBlock,
      sonarr_models::{DownloadRecord, DownloadStatus, Series, SonarrHistoryItem},
      stateful_table::StatefulTable,
    },
    ui::{DrawUi, sonarr_ui::SonarrUi, ui_test_utils::test_utils::render_to_string_with_app},
  };

  #[test]
  fn test_sonarr_ui_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      assert!(SonarrUi::accepts(active_sonarr_block.into()));
    });
  }

  mod snapshot_tests {
    use crate::ui::ui_test_utils::test_utils::TerminalSize;

    use super::*;

    #[test]
    fn test_sonarr_ui_renders_downloads_tab() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.quality_profile_map = BiMap::from_iter(vec![(0, "Any".to_owned())]);
      app.data.sonarr_data.language_profiles_map =
        BiMap::from_iter(vec![(0, "English".to_owned())]);
      app.data.sonarr_data.main_tabs.set_index(1); // Downloads tab
      app.data.sonarr_data.downloads = StatefulTable::default();
      app.data.sonarr_data.downloads.set_items(vec![
        DownloadRecord {
          id: 1,
          title: "Test Series S01E01".to_owned(),
          status: DownloadStatus::Downloading,
          size: 1500000000.0,
          sizeleft: 500000000.0,
          ..DownloadRecord::default()
        },
        DownloadRecord {
          id: 2,
          title: "Another Series S02E05".to_owned(),
          status: DownloadStatus::Downloading,
          size: 1200000000.0,
          sizeleft: 400000000.0,
          ..DownloadRecord::default()
        },
      ]);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SonarrUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_sonarr_ui_renders_history_tab() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.quality_profile_map = BiMap::from_iter(vec![(0, "Any".to_owned())]);
      app.data.sonarr_data.language_profiles_map =
        BiMap::from_iter(vec![(0, "English".to_owned())]);
      app.data.sonarr_data.main_tabs.set_index(2); // History tab
      app.data.sonarr_data.history = StatefulTable::default();
      app
        .data
        .sonarr_data
        .history
        .set_items(vec![SonarrHistoryItem::default()]);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SonarrUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_sonarr_ui_renders_library_tab() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.quality_profile_map = BiMap::from_iter(vec![(0, "Any".to_owned())]);
      app.data.sonarr_data.language_profiles_map =
        BiMap::from_iter(vec![(0, "English".to_owned())]);
      app.data.sonarr_data.main_tabs.set_index(0); // Library tab
      app.data.sonarr_data.series = StatefulTable::default();
      app.data.sonarr_data.series.set_items(vec![Series {
        id: 1,
        title: "Test Series".into(),
        quality_profile_id: 0,
        ..Series::default()
      }]);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SonarrUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
