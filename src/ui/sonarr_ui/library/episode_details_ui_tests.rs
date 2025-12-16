#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::sonarr::modals::{EpisodeDetailsModal, SeasonDetailsModal};
  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, EPISODE_DETAILS_BLOCKS,
  };
  use crate::models::sonarr_models::{Episode, Season, Series};
  use crate::models::stateful_table::StatefulTable;
  use crate::ui::DrawUi;
  use crate::ui::sonarr_ui::library::episode_details_ui::EpisodeDetailsUi;
  use crate::ui::ui_test_utils::test_utils::render_to_string_with_app;

  #[test]
  fn test_episode_details_ui_accepts() {
    ActiveSonarrBlock::iter().for_each(|active_sonarr_block| {
      if EPISODE_DETAILS_BLOCKS.contains(&active_sonarr_block) {
        assert!(EpisodeDetailsUi::accepts(active_sonarr_block.into()));
      } else {
        assert!(!EpisodeDetailsUi::accepts(active_sonarr_block.into()));
      }
    });
  }

  mod snapshot_tests {
    use crate::ui::ui_test_utils::test_utils::TerminalSize;

    use super::*;

    #[test]
    fn test_episode_details_ui_renders_loading_state() {
      let mut app = App::test_default();
      app.is_loading = true;
      app.push_navigation_stack(ActiveSonarrBlock::EpisodeDetails.into());
      app.data.sonarr_data.series = StatefulTable::default();
      app.data.sonarr_data.series.set_items(vec![Series {
        seasons: Some(vec![Season {
          season_number: 1,
          ..Season::default()
        }]),
        ..Series::default()
      }]);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        EpisodeDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_episode_details_ui_renders_episode_details_tab() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::EpisodeDetails.into());
      app.data.sonarr_data.series = StatefulTable::default();
      app.data.sonarr_data.series.set_items(vec![Series {
        seasons: Some(vec![Season {
          season_number: 1,
          ..Season::default()
        }]),
        ..Series::default()
      }]);
      let mut season_details_modal = SeasonDetailsModal::default();
      season_details_modal
        .episodes
        .set_items(vec![Episode::default()]);
      season_details_modal.episode_details_modal = Some(EpisodeDetailsModal::default());
      app.data.sonarr_data.season_details_modal = Some(season_details_modal);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        EpisodeDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_episode_details_ui_renders_episode_history_tab() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::EpisodeDetails.into());
      app.data.sonarr_data.series = StatefulTable::default();
      app.data.sonarr_data.series.set_items(vec![Series {
        seasons: Some(vec![Season {
          season_number: 1,
          ..Season::default()
        }]),
        ..Series::default()
      }]);
      let mut season_details_modal = SeasonDetailsModal::default();
      season_details_modal
        .episodes
        .set_items(vec![Episode::default()]);
      let mut episode_details_modal = EpisodeDetailsModal::default();
      episode_details_modal.episode_details_tabs.set_index(1);
      season_details_modal.episode_details_modal = Some(episode_details_modal);
      app.data.sonarr_data.season_details_modal = Some(season_details_modal);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        EpisodeDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }

    #[test]
    fn test_episode_details_ui_renders_manual_search_tab() {
      let mut app = App::test_default();
      app.push_navigation_stack(ActiveSonarrBlock::EpisodeDetails.into());
      app.data.sonarr_data.series = StatefulTable::default();
      app.data.sonarr_data.series.set_items(vec![Series {
        seasons: Some(vec![Season {
          season_number: 1,
          ..Season::default()
        }]),
        ..Series::default()
      }]);
      let mut season_details_modal = SeasonDetailsModal::default();
      season_details_modal
        .episodes
        .set_items(vec![Episode::default()]);
      let mut episode_details_modal = EpisodeDetailsModal::default();
      episode_details_modal.episode_details_tabs.set_index(3);
      season_details_modal.episode_details_modal = Some(episode_details_modal);
      app.data.sonarr_data.season_details_modal = Some(season_details_modal);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        EpisodeDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
