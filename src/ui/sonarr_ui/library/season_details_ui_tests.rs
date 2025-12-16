#[cfg(test)]
mod tests {
  use bimap::BiMap;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::sonarr::modals::SeasonDetailsModal;
  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, EPISODE_DETAILS_BLOCKS, SEASON_DETAILS_BLOCKS,
  };
  use crate::models::sonarr_models::{Season, Series};
  use crate::models::stateful_table::StatefulTable;
  use crate::ui::DrawUi;
  use crate::ui::sonarr_ui::library::season_details_ui::SeasonDetailsUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_season_details_ui_accepts() {
    let mut blocks = SEASON_DETAILS_BLOCKS.clone().to_vec();
    blocks.extend(EPISODE_DETAILS_BLOCKS);

    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if blocks.contains(&active_sonarr_block) {
        assert!(SeasonDetailsUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!SeasonDetailsUi::accepts(active_sonarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use crate::ui::ui_test_utils::test_utils::TerminalSize;

    use super::*;

    #[test]
    fn test_season_details_ui_renders_loading_state() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::SeasonDetails.into());
      app.data.sonarr_data.series = StatefulTable::default();
      app.data.sonarr_data.series.set_items(vec![Series {
        seasons: Some(vec![Season::default()]),
        ..Series::default()
      }]);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SeasonDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_season_details_ui_renders_episodes_tab() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::SeasonDetails.into());
      app.data.sonarr_data.series = StatefulTable::default();
      app.data.sonarr_data.series.set_items(vec![Series {
        seasons: Some(vec![Season::default()]),
        ..Series::default()
      }]);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SeasonDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_season_details_ui_renders_manual_search_tab() {
      use crate::models::sonarr_models::{Episode, EpisodeFile, SonarrRelease};

      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::SeasonDetails.into());
      app.data.sonarr_data.quality_profile_map = BiMap::from_iter(vec![(0, "Any".to_owned())]);
      app.data.sonarr_data.language_profiles_map =
        BiMap::from_iter(vec![(0, "English".to_owned())]);
      app.data.sonarr_data.series = StatefulTable::default();
      app.data.sonarr_data.series.set_items(vec![Series {
        seasons: Some(vec![Season::default()]),
        ..Series::default()
      }]);
      app
        .data
        .sonarr_data
        .seasons
        .set_items(vec![Season::default()]);
      let mut season_details_modal = SeasonDetailsModal::default();
      season_details_modal.season_details_tabs.set_index(2);
      season_details_modal
        .episodes
        .set_items(vec![Episode::default()]);
      season_details_modal
        .episode_files
        .set_items(vec![EpisodeFile::default()]);
      season_details_modal
        .season_releases
        .set_items(vec![SonarrRelease::default()]);
      app.data.sonarr_data.season_details_modal = Some(season_details_modal);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SeasonDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_season_details_ui_renders_season_history_tab() {
      use crate::models::sonarr_models::{Episode, EpisodeFile, SonarrHistoryItem};

      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::SeasonDetails.into());
      app.data.sonarr_data.quality_profile_map = BiMap::from_iter(vec![(0, "Any".to_owned())]);
      app.data.sonarr_data.language_profiles_map =
        BiMap::from_iter(vec![(0, "English".to_owned())]);
      app.data.sonarr_data.series = StatefulTable::default();
      app.data.sonarr_data.series.set_items(vec![Series {
        seasons: Some(vec![Season::default()]),
        ..Series::default()
      }]);
      app
        .data
        .sonarr_data
        .seasons
        .set_items(vec![Season::default()]);
      let mut season_details_modal = SeasonDetailsModal::default();
      season_details_modal.season_details_tabs.set_index(1);
      season_details_modal
        .episodes
        .set_items(vec![Episode::default()]);
      season_details_modal
        .episode_files
        .set_items(vec![EpisodeFile::default()]);
      season_details_modal
        .season_history
        .set_items(vec![SonarrHistoryItem::default()]);
      app.data.sonarr_data.season_details_modal = Some(season_details_modal);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SeasonDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
