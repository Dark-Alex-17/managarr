#[cfg(test)]
mod tests {
  use bimap::BiMap;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, EPISODE_DETAILS_BLOCKS, SEASON_DETAILS_BLOCKS, SERIES_DETAILS_BLOCKS,
  };
  use crate::models::sonarr_models::{Season, Series};
  use crate::models::stateful_table::StatefulTable;
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
    use super::*;

    #[test]
    fn test_series_details_ui_renders_series_details() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::SeriesDetails.into());
      app.data.sonarr_data.quality_profile_map = BiMap::from_iter(vec![(1, "HD-1080p".to_owned())]);
      app.data.sonarr_data.language_profiles_map =
        BiMap::from_iter(vec![(1, "English".to_owned())]);
      app.data.sonarr_data.series = StatefulTable::default();
      app.data.sonarr_data.series.set_items(vec![Series {
        id: 1,
        title: "Test Series".into(),
        seasons: Some(vec![Season::default()]),
        quality_profile_id: 1,
        language_profile_id: 1,
        ..Series::default()
      }]);
      app.data.sonarr_data.series_history = Some(StatefulTable::default());

      let output = render_to_string_with_app(120, 30, &mut app, |f, app| {
        SeriesDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
