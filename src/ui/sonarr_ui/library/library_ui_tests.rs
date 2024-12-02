#[cfg(test)]
mod tests {
  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, ADD_SERIES_BLOCKS, DELETE_SERIES_BLOCKS, EDIT_SERIES_BLOCKS,
  };
  use crate::models::{
    servarr_data::sonarr::sonarr_data::LIBRARY_BLOCKS, sonarr_models::SeriesStatus,
  };
  use crate::ui::sonarr_ui::library::LibraryUi;
  use crate::ui::styles::ManagarrStyle;
  use crate::ui::DrawUi;
  use pretty_assertions::assert_eq;
  use ratatui::widgets::{Cell, Row};
  use rstest::rstest;
  use strum::IntoEnumIterator;

  use crate::{
    models::sonarr_models::{Series, SeriesStatistics},
    ui::sonarr_ui::library::decorate_series_row_with_style,
  };

  #[test]
  fn test_library_ui_accepts() {
    let mut library_ui_blocks = Vec::new();
    library_ui_blocks.extend(LIBRARY_BLOCKS);
    library_ui_blocks.extend(ADD_SERIES_BLOCKS);
    library_ui_blocks.extend(DELETE_SERIES_BLOCKS);
    library_ui_blocks.extend(EDIT_SERIES_BLOCKS);

    ActiveSonarrBlock::iter().for_each(|active_radarr_block| {
      if library_ui_blocks.contains(&active_radarr_block) {
        assert!(LibraryUi::accepts(active_radarr_block.into()));
      } else {
        assert!(!LibraryUi::accepts(active_radarr_block.into()));
      }
    });
  }

  #[rstest]
  #[case(SeriesStatus::Ended, None, RowStyle::Missing)]
  #[case(SeriesStatus::Ended, Some(59.0), RowStyle::Missing)]
  #[case(SeriesStatus::Ended, Some(100.0), RowStyle::Downloaded)]
  #[case(SeriesStatus::Continuing, None, RowStyle::Missing)]
  #[case(SeriesStatus::Continuing, Some(59.0), RowStyle::Missing)]
  #[case(SeriesStatus::Continuing, Some(100.0), RowStyle::Unreleased)]
  #[case(SeriesStatus::Upcoming, None, RowStyle::Unreleased)]
  #[case(SeriesStatus::Deleted, None, RowStyle::Missing)]
  fn test_decorate_series_row_with_style(
    #[case] series_status: SeriesStatus,
    #[case] percent_of_episodes: Option<f64>,
    #[case] expected_row_style: RowStyle,
  ) {
    let mut series = Series {
      status: series_status,
      ..Series::default()
    };
    if let Some(percentage) = percent_of_episodes {
      series.statistics = Some(SeriesStatistics {
        percent_of_episodes: percentage,
        ..SeriesStatistics::default()
      });
    }

    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_series_row_with_style(&series, row.clone());

    match expected_row_style {
      RowStyle::Downloaded => assert_eq!(style, row.downloaded()),
      RowStyle::Missing => assert_eq!(style, row.missing()),
      RowStyle::Unreleased => assert_eq!(style, row.unreleased()),
    }
  }

  enum RowStyle {
    Downloaded,
    Missing,
    Unreleased,
  }
}
