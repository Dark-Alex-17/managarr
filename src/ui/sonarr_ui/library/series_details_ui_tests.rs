#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, EPISODE_DETAILS_BLOCKS, SEASON_DETAILS_BLOCKS, SERIES_DETAILS_BLOCKS,
  };
  use crate::ui::DrawUi;
  use crate::ui::sonarr_ui::library::series_details_ui::SeriesDetailsUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_series_details_ui_accepts() {
    let mut blocks = SERIES_DETAILS_BLOCKS.clone().to_vec();
    blocks.extend(SEASON_DETAILS_BLOCKS);
    blocks.extend(EPISODE_DETAILS_BLOCKS);

    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if blocks.contains(&active_sonarr_block) {
        assert!(SeriesDetailsUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!SeriesDetailsUi::accepts(active_sonarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use crate::models::stateful_table::StatefulTable;
    use crate::ui::ui_test_utils::test_utils::TerminalSize;
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
  }
}
