#[cfg(test)]
mod tests {
  use crate::models::servarr_data::sonarr::sonarr_data::{
    ADD_SERIES_BLOCKS, ActiveSonarrBlock, DELETE_SERIES_BLOCKS, EDIT_SERIES_BLOCKS,
    EPISODE_DETAILS_BLOCKS, SEASON_DETAILS_BLOCKS, SERIES_DETAILS_BLOCKS,
  };
  use crate::models::{
    servarr_data::sonarr::sonarr_data::LIBRARY_BLOCKS, sonarr_models::SeriesStatus,
  };
  use crate::ui::DrawUi;
  use crate::ui::sonarr_ui::library::LibraryUi;
  use crate::ui::styles::ManagarrStyle;
  use pretty_assertions::assert_eq;
  use ratatui::widgets::{Cell, Row};
  use strum::IntoEnumIterator;

  use crate::models::sonarr_models::{Season, SeasonStatistics};
  use crate::{
    models::sonarr_models::Series, ui::sonarr_ui::library::decorate_series_row_with_style,
  };

  #[test]
  fn test_library_ui_accepts() {
    let mut library_ui_blocks = Vec::new();
    library_ui_blocks.extend(LIBRARY_BLOCKS);
    library_ui_blocks.extend(ADD_SERIES_BLOCKS);
    library_ui_blocks.extend(DELETE_SERIES_BLOCKS);
    library_ui_blocks.extend(EDIT_SERIES_BLOCKS);
    library_ui_blocks.extend(SERIES_DETAILS_BLOCKS);
    library_ui_blocks.extend(SEASON_DETAILS_BLOCKS);
    library_ui_blocks.extend(EPISODE_DETAILS_BLOCKS);

    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if library_ui_blocks.contains(&active_sonarr_block) {
        assert!(LibraryUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!LibraryUi::accepts(active_sonarr_block.into()));
      }
    });
  }

  #[test]
  fn test_decorate_row_with_style_unmonitored() {
    let series = Series::default();
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_series_row_with_style(&series, row.clone());

    assert_eq!(style, row.unmonitored());
  }

  #[test]
  fn test_decorate_row_with_style_downloaded_when_ended_and_all_monitored_episodes_are_present() {
    let seasons = vec![
      Season {
        monitored: false,
        statistics: Some(SeasonStatistics {
          episode_file_count: 1,
          episode_count: 3,
          ..SeasonStatistics::default()
        }),
        ..Season::default()
      },
      Season {
        monitored: true,
        statistics: Some(SeasonStatistics {
          episode_file_count: 3,
          episode_count: 3,
          ..SeasonStatistics::default()
        }),
        ..Season::default()
      },
    ];
    let series = Series {
      monitored: true,
      status: SeriesStatus::Ended,
      seasons: Some(seasons),
      ..Series::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_series_row_with_style(&series, row.clone());

    assert_eq!(style, row.downloaded());
  }

  #[test]
  fn test_decorate_row_with_style_missing_when_ended_and_episodes_are_missing() {
    let seasons = vec![
      Season {
        monitored: true,
        statistics: Some(SeasonStatistics {
          episode_file_count: 1,
          episode_count: 3,
          ..SeasonStatistics::default()
        }),
        ..Season::default()
      },
      Season {
        monitored: true,
        statistics: Some(SeasonStatistics {
          episode_file_count: 3,
          episode_count: 3,
          ..SeasonStatistics::default()
        }),
        ..Season::default()
      },
    ];
    let series = Series {
      monitored: true,
      status: SeriesStatus::Ended,
      seasons: Some(seasons),
      ..Series::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_series_row_with_style(&series, row.clone());

    assert_eq!(style, row.missing());
  }

  #[test]
  fn test_decorate_row_with_style_indeterminate_when_ended_and_seasons_is_empty() {
    let series = Series {
      monitored: true,
      status: SeriesStatus::Ended,
      ..Series::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_series_row_with_style(&series, row.clone());

    assert_eq!(style, row.indeterminate());
  }

  #[test]
  fn test_decorate_row_with_style_unreleased_when_continuing_and_all_monitored_episodes_are_present()
   {
    let seasons = vec![
      Season {
        monitored: false,
        statistics: Some(SeasonStatistics {
          episode_file_count: 1,
          episode_count: 3,
          ..SeasonStatistics::default()
        }),
        ..Season::default()
      },
      Season {
        monitored: true,
        statistics: Some(SeasonStatistics {
          episode_file_count: 3,
          episode_count: 3,
          ..SeasonStatistics::default()
        }),
        ..Season::default()
      },
    ];
    let series = Series {
      monitored: true,
      status: SeriesStatus::Continuing,
      seasons: Some(seasons),
      ..Series::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_series_row_with_style(&series, row.clone());

    assert_eq!(style, row.unreleased());
  }

  #[test]
  fn test_decorate_row_with_style_missing_when_continuing_and_episodes_are_missing() {
    let seasons = vec![
      Season {
        monitored: true,
        statistics: Some(SeasonStatistics {
          episode_file_count: 1,
          episode_count: 3,
          ..SeasonStatistics::default()
        }),
        ..Season::default()
      },
      Season {
        monitored: true,
        statistics: Some(SeasonStatistics {
          episode_file_count: 3,
          episode_count: 3,
          ..SeasonStatistics::default()
        }),
        ..Season::default()
      },
    ];
    let series = Series {
      monitored: true,
      status: SeriesStatus::Continuing,
      seasons: Some(seasons),
      ..Series::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_series_row_with_style(&series, row.clone());

    assert_eq!(style, row.missing());
  }

  #[test]
  fn test_decorate_row_with_style_indeterminate_when_continuing_and_seasons_is_empty() {
    let series = Series {
      monitored: true,
      status: SeriesStatus::Continuing,
      ..Series::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_series_row_with_style(&series, row.clone());

    assert_eq!(style, row.indeterminate());
  }

  #[test]
  fn test_decorate_row_with_style_unreleased_when_upcoming() {
    let series = Series {
      monitored: true,
      status: SeriesStatus::Upcoming,
      ..Series::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_series_row_with_style(&series, row.clone());

    assert_eq!(style, row.unreleased());
  }

  #[test]
  fn test_decorate_row_with_style_defaults_to_indeterminate() {
    let series = Series {
      monitored: true,
      status: SeriesStatus::Deleted,
      ..Series::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_series_row_with_style(&series, row.clone());

    assert_eq!(style, row.indeterminate());
  }

  mod snapshot_tests {

    use crate::app::App;
    use crate::models::servarr_data::sonarr::sonarr_data::ActiveSonarrBlock;

    use crate::models::stateful_table::StatefulTable;
    use crate::ui::DrawUi;
    use crate::ui::sonarr_ui::library::LibraryUi;
    use crate::ui::ui_test_utils::test_utils::{TerminalSize, render_to_string_with_app};

    #[test]
    fn test_library_ui_renders_loading_state() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        LibraryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_library_ui_renders_empty_series() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.data.sonarr_data.series = StatefulTable::default();

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        LibraryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_library_ui_renders_with_series() {
      use crate::models::sonarr_models::{Series, SeriesStatus, SeriesType};
      use crate::models::stateful_table::StatefulTable;
      use bimap::BiMap;

      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());

      // Set up quality profile and language profile maps
      let mut quality_profile_map = BiMap::new();
      quality_profile_map.insert(1, "HD-1080p".to_owned());
      quality_profile_map.insert(2, "Any".to_owned());
      app.data.sonarr_data.quality_profile_map = quality_profile_map;

      let mut language_profiles_map = BiMap::new();
      language_profiles_map.insert(1, "English".to_owned());
      language_profiles_map.insert(2, "Any".to_owned());
      app.data.sonarr_data.language_profiles_map = language_profiles_map;

      // Create series with data
      let mut series_table = StatefulTable::default();
      series_table.set_items(vec![
        Series {
          id: 1,
          title: "Breaking Bad".into(),
          year: 2008,
          network: Some("AMC".to_owned()),
          status: SeriesStatus::Ended,
          monitored: true,
          series_type: SeriesType::Standard,
          quality_profile_id: 1,
          language_profile_id: 1,
          seasons: Some(vec![]),
          ..Series::default()
        },
        Series {
          id: 2,
          title: "The Wire".into(),
          year: 2002,
          network: Some("HBO".to_owned()),
          status: SeriesStatus::Continuing,
          monitored: true,
          series_type: SeriesType::Standard,
          quality_profile_id: 2,
          language_profile_id: 1,
          seasons: Some(vec![]),
          ..Series::default()
        },
      ]);
      app.data.sonarr_data.series = series_table;

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        LibraryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_library_ui_renders_update_all_series_prompt() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::UpdateAllSeriesPrompt.into());
      app.data.sonarr_data.series = StatefulTable::default();

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        LibraryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
