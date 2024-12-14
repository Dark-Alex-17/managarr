#[cfg(test)]
mod tests {
  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, ADD_SERIES_BLOCKS, DELETE_SERIES_BLOCKS, EDIT_SERIES_BLOCKS,
    EPISODE_DETAILS_BLOCKS, SEASON_DETAILS_BLOCKS, SERIES_DETAILS_BLOCKS,
  };
  use crate::models::{
    servarr_data::sonarr::sonarr_data::LIBRARY_BLOCKS, sonarr_models::SeriesStatus,
  };
  use crate::ui::sonarr_ui::library::LibraryUi;
  use crate::ui::styles::ManagarrStyle;
  use crate::ui::DrawUi;
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
        statistics: SeasonStatistics {
          episode_file_count: 1,
          episode_count: 3,
          ..SeasonStatistics::default()
        },
        ..Season::default()
      },
      Season {
        monitored: true,
        statistics: SeasonStatistics {
          episode_file_count: 3,
          episode_count: 3,
          ..SeasonStatistics::default()
        },
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
        statistics: SeasonStatistics {
          episode_file_count: 1,
          episode_count: 3,
          ..SeasonStatistics::default()
        },
        ..Season::default()
      },
      Season {
        monitored: true,
        statistics: SeasonStatistics {
          episode_file_count: 3,
          episode_count: 3,
          ..SeasonStatistics::default()
        },
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
  fn test_decorate_row_with_style_unreleased_when_continuing_and_all_monitored_episodes_are_present(
  ) {
    let seasons = vec![
      Season {
        monitored: false,
        statistics: SeasonStatistics {
          episode_file_count: 1,
          episode_count: 3,
          ..SeasonStatistics::default()
        },
        ..Season::default()
      },
      Season {
        monitored: true,
        statistics: SeasonStatistics {
          episode_file_count: 3,
          episode_count: 3,
          ..SeasonStatistics::default()
        },
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
        statistics: SeasonStatistics {
          episode_file_count: 1,
          episode_count: 3,
          ..SeasonStatistics::default()
        },
        ..Season::default()
      },
      Season {
        monitored: true,
        statistics: SeasonStatistics {
          episode_file_count: 3,
          episode_count: 3,
          ..SeasonStatistics::default()
        },
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
}
