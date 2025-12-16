#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, EPISODE_DETAILS_BLOCKS, SEASON_DETAILS_BLOCKS,
  };
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
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(ActiveSonarrBlock::SeasonDetails, 0)]
    #[case(ActiveSonarrBlock::SeasonHistory, 1)]
    #[case(ActiveSonarrBlock::SearchEpisodes, 0)]
    #[case(ActiveSonarrBlock::SearchEpisodesError, 0)]
    #[case(ActiveSonarrBlock::AutomaticallySearchSeasonPrompt, 0)]
    #[case(ActiveSonarrBlock::AutomaticallySearchSeasonPrompt, 1)]
    #[case(ActiveSonarrBlock::AutomaticallySearchSeasonPrompt, 2)]
    #[case(ActiveSonarrBlock::SearchSeasonHistory, 1)]
    #[case(ActiveSonarrBlock::SearchSeasonHistoryError, 1)]
    #[case(ActiveSonarrBlock::FilterSeasonHistory, 1)]
    #[case(ActiveSonarrBlock::FilterSeasonHistoryError, 1)]
    #[case(ActiveSonarrBlock::SeasonHistorySortPrompt, 1)]
    #[case(ActiveSonarrBlock::SeasonHistoryDetails, 1)]
    #[case(ActiveSonarrBlock::ManualSeasonSearch, 2)]
    #[case(ActiveSonarrBlock::ManualSeasonSearchConfirmPrompt, 2)]
    #[case(ActiveSonarrBlock::ManualSeasonSearchSortPrompt, 2)]
    #[case(ActiveSonarrBlock::DeleteEpisodeFilePrompt, 0)]
    fn test_season_details_ui_renders(
      #[case] active_sonarr_block: ActiveSonarrBlock,
      #[case] index: usize,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_sonarr_block.into());
      app
        .data
        .sonarr_data
        .season_details_modal
        .as_mut()
        .unwrap()
        .season_details_tabs
        .set_index(index);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SeasonDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("season_details_renders_{active_sonarr_block}_{index}"),
        output
      );
    }

    #[rstest]
    #[case(ActiveSonarrBlock::SeasonDetails, 0)]
    #[case(ActiveSonarrBlock::SeasonHistory, 1)]
    #[case(ActiveSonarrBlock::SeasonHistoryDetails, 1)]
    #[case(ActiveSonarrBlock::ManualSeasonSearch, 2)]
    fn test_season_details_ui_renders_loading(
      #[case] active_sonarr_block: ActiveSonarrBlock,
      #[case] index: usize,
    ) {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(active_sonarr_block.into());
      {
        let season_details_modal = app.data.sonarr_data.season_details_modal.as_mut().unwrap();
        season_details_modal.season_releases = StatefulTable::default();
        season_details_modal.season_history = StatefulTable::default();
        season_details_modal.episodes = StatefulTable::default();
        season_details_modal.season_details_tabs.set_index(index);
      }

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SeasonDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("loading_season_details_{active_sonarr_block}_{index}"),
        output
      );
    }

    #[rstest]
    #[case(ActiveSonarrBlock::SeasonDetails, 0)]
    #[case(ActiveSonarrBlock::SeasonHistory, 1)]
    #[case(ActiveSonarrBlock::SeasonHistoryDetails, 1)]
    #[case(ActiveSonarrBlock::ManualSeasonSearch, 2)]
    fn test_season_details_ui_renders_empty(
      #[case] active_sonarr_block: ActiveSonarrBlock,
      #[case] index: usize,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_sonarr_block.into());
      {
        let season_details_modal = app.data.sonarr_data.season_details_modal.as_mut().unwrap();
        season_details_modal.season_releases = StatefulTable::default();
        season_details_modal.season_history = StatefulTable::default();
        season_details_modal.episodes = StatefulTable::default();
        season_details_modal.season_details_tabs.set_index(index);
      }

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SeasonDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("empty_season_details_{active_sonarr_block}_{index}"),
        output
      );
    }

    #[test]
    fn test_season_details_ui_renders_episode_details_over_season_details() {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(ActiveSonarrBlock::EpisodeDetails.into());

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        SeasonDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(output);
    }
  }
}
