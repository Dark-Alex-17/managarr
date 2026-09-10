#[cfg(test)]
mod tests {
  use chrono::{Duration, Utc};
  use pretty_assertions::assert_eq;
  use ratatui::widgets::{Cell, Row};
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, EPISODE_DETAILS_BLOCKS, SEASON_DETAILS_BLOCKS, SERIES_DETAILS_BLOCKS,
    SERIES_OVERVIEW_BLOCKS,
  };
  use crate::models::sonarr_models::{Season, SeasonStatistics};
  use crate::ui::DrawUi;
  use crate::ui::sonarr_ui::library::series_details_ui::{
    SeriesDetailsUi, decorate_season_row_with_style,
  };
  use crate::ui::styles::ManagarrStyle;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_series_details_ui_accepts() {
    let mut blocks = SERIES_DETAILS_BLOCKS.clone().to_vec();
    blocks.extend(SEASON_DETAILS_BLOCKS);
    blocks.extend(EPISODE_DETAILS_BLOCKS);
    blocks.extend(SERIES_OVERVIEW_BLOCKS);

    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if blocks.contains(&active_sonarr_block) {
        assert!(SeriesDetailsUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!SeriesDetailsUi::accepts(active_sonarr_block.into()));
      }
    });
  }

  #[test]
  fn test_decorate_season_row_with_style_unmonitored() {
    let season = Season::default();
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_season_row_with_style(&season, row.clone());

    assert_eq!(style, row.unmonitored());
  }

  #[test]
  fn test_decorate_season_row_with_style_downloaded_when_all_episodes_present() {
    let season = Season {
      monitored: true,
      statistics: Some(SeasonStatistics {
        episode_file_count: 3,
        episode_count: 3,
        ..SeasonStatistics::default()
      }),
      ..Season::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_season_row_with_style(&season, row.clone());

    assert_eq!(style, row.downloaded());
  }

  #[test]
  fn test_decorate_season_row_with_style_downloaded_when_no_statistics() {
    let season = Season {
      monitored: true,
      statistics: None,
      ..Season::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_season_row_with_style(&season, row.clone());

    assert_eq!(style, row.downloaded());
  }

  #[test]
  fn test_decorate_season_row_with_style_unreleased_when_episodes_are_missing_and_next_airing_is_future()
   {
    let season = Season {
      monitored: true,
      statistics: Some(SeasonStatistics {
        episode_file_count: 0,
        episode_count: 3,
        next_airing: Some(Utc::now() + Duration::days(1)),
        ..SeasonStatistics::default()
      }),
      ..Season::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_season_row_with_style(&season, row.clone());

    assert_eq!(style, row.unreleased());
  }

  #[test]
  fn test_decorate_season_row_with_style_missing_when_episodes_are_missing_and_next_airing_has_passed()
   {
    let season = Season {
      monitored: true,
      statistics: Some(SeasonStatistics {
        episode_file_count: 1,
        episode_count: 3,
        next_airing: Some(Utc::now() - Duration::days(1)),
        ..SeasonStatistics::default()
      }),
      ..Season::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_season_row_with_style(&season, row.clone());

    assert_eq!(style, row.missing());
  }

  #[test]
  fn test_decorate_season_row_with_style_missing_when_episodes_are_missing_and_no_next_airing() {
    let season = Season {
      monitored: true,
      statistics: Some(SeasonStatistics {
        episode_file_count: 1,
        episode_count: 3,
        next_airing: None,
        ..SeasonStatistics::default()
      }),
      ..Season::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_season_row_with_style(&season, row.clone());

    assert_eq!(style, row.missing());
  }

  mod snapshot_tests {
    use crate::models::stateful_table::StatefulTable;
    use crate::ui::ui_test_utils::test_utils::TerminalSize;
    use pretty_assertions::assert_str_eq;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(ActiveSonarrBlock::SeriesDetails, 0)]
    #[case(ActiveSonarrBlock::SeriesHistory, 1)]
    #[case(ActiveSonarrBlock::SearchSeason, 0)]
    #[case(ActiveSonarrBlock::SearchSeasonError, 0)]
    #[case(ActiveSonarrBlock::UpdateAndScanSeriesPrompt, 0)]
    #[case(ActiveSonarrBlock::UpdateAndScanSeriesPrompt, 1)]
    #[case(ActiveSonarrBlock::AutomaticallySearchSeriesPrompt, 0)]
    #[case(ActiveSonarrBlock::AutomaticallySearchSeriesPrompt, 1)]
    #[case(ActiveSonarrBlock::SearchSeriesHistory, 1)]
    #[case(ActiveSonarrBlock::SearchSeriesHistoryError, 1)]
    #[case(ActiveSonarrBlock::FilterSeriesHistory, 1)]
    #[case(ActiveSonarrBlock::FilterSeriesHistoryError, 1)]
    #[case(ActiveSonarrBlock::SeriesHistorySortPrompt, 1)]
    #[case(ActiveSonarrBlock::SeriesHistoryDetails, 1)]
    fn test_series_details_ui_renders_series_details(
      #[case] active_sonarr_block: ActiveSonarrBlock,
      #[case] index: usize,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_sonarr_block.into());
      app.data.sonarr_data.series_info_tabs.set_index(index);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SeriesDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("series_details_ui_{active_sonarr_block}_{index}"),
        output
      );
    }

    #[rstest]
    #[case(ActiveSonarrBlock::SeriesDetails, 0)]
    #[case(ActiveSonarrBlock::SeriesHistory, 1)]
    fn test_series_details_ui_renders_series_details_loading(
      #[case] active_sonarr_block: ActiveSonarrBlock,
      #[case] index: usize,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_sonarr_block.into());
      app.data.sonarr_data.series_info_tabs.set_index(index);
      app.is_loading = true;

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SeriesDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("loading_series_details_{active_sonarr_block}"),
        output
      );
    }

    #[rstest]
    #[case(ActiveSonarrBlock::SeriesDetails, 0)]
    #[case(ActiveSonarrBlock::SeriesHistory, 1)]
    #[case(ActiveSonarrBlock::SeriesHistoryDetails, 1)]
    fn test_series_details_ui_renders_series_details_empty(
      #[case] active_sonarr_block: ActiveSonarrBlock,
      #[case] index: usize,
    ) {
      let mut app = App::test_default_fully_populated();
      app.data.sonarr_data.seasons = StatefulTable::default();
      app.data.sonarr_data.series_history = Some(StatefulTable::default());
      app.data.sonarr_data.series_info_tabs.set_index(index);
      app.push_navigation_stack(active_sonarr_block.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SeriesDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("empty_series_details_{active_sonarr_block}"),
        output
      );
    }

    #[test]
    fn test_series_details_ui_renders_season_details_over_series_details() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveSonarrBlock::SeasonDetails.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SeriesDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_series_details_ui_renders_episode_details_over_series_details() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveSonarrBlock::EpisodeDetails.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SeriesDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_series_details_ui_renders_series_overview_over_series_details() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveSonarrBlock::SeriesDetails.into());
      app.push_navigation_stack(ActiveSonarrBlock::SeriesOverview.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SeriesDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_scrolling_the_series_overview_does_not_scroll_the_series_details_background() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveSonarrBlock::SeriesDetails.into());
      let before = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SeriesDetailsUi::draw(f, app, f.area());
      });

      app
        .data
        .sonarr_data
        .series_overview_modal
        .as_mut()
        .unwrap()
        .overview
        .offset = 3;

      let after = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SeriesDetailsUi::draw(f, app, f.area());
      });

      assert_str_eq!(before, after);
      assert_contains!(before, "Overview: Blah blah blah");
    }
  }
}
