#[cfg(test)]
mod tests {
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, EPISODE_DETAILS_BLOCKS,
  };
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
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(ActiveSonarrBlock::EpisodeDetails, 0)]
    #[case(ActiveSonarrBlock::EpisodeHistory, 1)]
    #[case(ActiveSonarrBlock::EpisodeHistoryDetails, 1)]
    #[case(ActiveSonarrBlock::EpisodeFile, 2)]
    #[case(ActiveSonarrBlock::ManualEpisodeSearch, 3)]
    #[case(ActiveSonarrBlock::ManualEpisodeSearchConfirmPrompt, 3)]
    #[case(ActiveSonarrBlock::ManualEpisodeSearchSortPrompt, 3)]
    #[case(ActiveSonarrBlock::AutomaticallySearchEpisodePrompt, 0)]
    #[case(ActiveSonarrBlock::AutomaticallySearchEpisodePrompt, 1)]
    #[case(ActiveSonarrBlock::AutomaticallySearchEpisodePrompt, 2)]
    #[case(ActiveSonarrBlock::AutomaticallySearchEpisodePrompt, 3)]
    fn test_episode_details_ui_renders(
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
        .episode_details_modal
        .as_mut()
        .unwrap()
        .episode_details_tabs
        .set_index(index);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        EpisodeDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("episode_details_{active_sonarr_block}_{index}"),
        output
      );
    }

    #[rstest]
    #[case(ActiveSonarrBlock::EpisodeDetails, 0)]
    #[case(ActiveSonarrBlock::EpisodeHistory, 1)]
    #[case(ActiveSonarrBlock::EpisodeFile, 2)]
    #[case(ActiveSonarrBlock::ManualEpisodeSearch, 3)]
    fn test_episode_details_ui_renders_loading(
      #[case] active_sonarr_block: ActiveSonarrBlock,
      #[case] index: usize,
    ) {
      let mut app = App::test_default_fully_populated();
      app.is_loading = true;
      app.push_navigation_stack(active_sonarr_block.into());
      app
        .data
        .sonarr_data
        .season_details_modal
        .as_mut()
        .unwrap()
        .episode_details_modal
        .as_mut()
        .unwrap()
        .episode_details_tabs
        .set_index(index);

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        EpisodeDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("loading_episode_details_{active_sonarr_block}_{index}"),
        output
      );
    }

    #[rstest]
    #[case(ActiveSonarrBlock::EpisodeDetails, 0)]
    #[case(ActiveSonarrBlock::EpisodeHistory, 1)]
    #[case(ActiveSonarrBlock::EpisodeHistoryDetails, 1)]
    #[case(ActiveSonarrBlock::EpisodeFile, 2)]
    #[case(ActiveSonarrBlock::ManualEpisodeSearch, 3)]
    fn test_episode_details_ui_renders_empty(
      #[case] active_sonarr_block: ActiveSonarrBlock,
      #[case] index: usize,
    ) {
      let mut app = App::test_default_fully_populated();
      app.push_navigation_stack(active_sonarr_block.into());
      {
        let episode_details_modal = app
          .data
          .sonarr_data
          .season_details_modal
          .as_mut()
          .unwrap()
          .episode_details_modal
          .as_mut()
          .unwrap();
        episode_details_modal.episode_details_tabs.set_index(index);
        episode_details_modal.episode_details = Default::default();
        episode_details_modal.episode_history = Default::default();
        episode_details_modal.file_details = Default::default();
        episode_details_modal.episode_releases = Default::default();
      }

      let output = render_to_string_with_app(TerminalSize::Large, &mut app, |f, app| {
        EpisodeDetailsUi::draw(f, app, f.area());
      });

      insta::assert_snapshot!(
        format!("empty_episode_details_{active_sonarr_block}_{index}"),
        output
      );
    }
  }
}
