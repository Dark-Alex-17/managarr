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
    use crate::models::BlockSelectionState;
    use crate::models::servarr_data::sonarr::sonarr_data::{
      ADD_SERIES_SELECTION_BLOCKS, ActiveSonarrBlock, DELETE_SERIES_SELECTION_BLOCKS,
      EDIT_SERIES_SELECTION_BLOCKS,
    };
    use rstest::rstest;

    use crate::ui::DrawUi;
    use crate::ui::sonarr_ui::library::LibraryUi;
    use crate::ui::ui_test_utils::test_utils::{TerminalSize, render_to_string_with_app};

    #[rstest]
    fn test_library_ui_renders(
      #[values(
        ActiveSonarrBlock::Series,
        ActiveSonarrBlock::SeriesSortPrompt,
        ActiveSonarrBlock::SearchSeries,
        ActiveSonarrBlock::SearchSeriesError,
        ActiveSonarrBlock::FilterSeries,
        ActiveSonarrBlock::FilterSeriesError,
        ActiveSonarrBlock::UpdateAllSeriesPrompt
      )]
      active_sonarr_block: ActiveSonarrBlock,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_sonarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        LibraryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(format!("sonarr_library_{active_sonarr_block}"), output);
    }

    #[test]
    fn test_library_ui_renders_loading() {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        LibraryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_library_ui_renders_empty() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        LibraryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_library_ui_renders_series_details_over_series() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveSonarrBlock::SeriesDetails.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        LibraryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_library_ui_renders_season_details_over_series() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveSonarrBlock::SeasonDetails.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        LibraryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_library_ui_renders_episode_details_over_series() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveSonarrBlock::EpisodeDetails.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        LibraryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_library_ui_renders_delete_episode_over_series() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveSonarrBlock::DeleteSeriesPrompt.into());
      app.data.sonarr_data.selected_block =
        BlockSelectionState::new(DELETE_SERIES_SELECTION_BLOCKS);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        LibraryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_library_ui_renders_edit_series_over_series() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveSonarrBlock::EditSeriesPrompt.into());
      app.data.sonarr_data.selected_block = BlockSelectionState::new(EDIT_SERIES_SELECTION_BLOCKS);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        LibraryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_library_ui_renders_add_series_over_series() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveSonarrBlock::AddSeriesPrompt.into());
      app.data.sonarr_data.selected_block = BlockSelectionState::new(ADD_SERIES_SELECTION_BLOCKS);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        LibraryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_library_ui_renders_update_all_series_prompt() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveSonarrBlock::Series.into());
      app.push_navigation_stack(ActiveSonarrBlock::UpdateAllSeriesPrompt.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        LibraryUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
