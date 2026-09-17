#[cfg(test)]
mod tests {
  use chrono::{Duration, Utc};
  use pretty_assertions::assert_eq;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, EPISODE_DETAILS_BLOCKS,
  };
  use crate::models::sonarr_models::{DownloadRecord, DownloadStatus, Episode};
  use crate::ui::DrawUi;
  use crate::ui::sonarr_ui::library::episode_details_ui::{EpisodeDetailsUi, style_from_status};
  use crate::ui::styles::{
    awaiting_import_style, downloaded_style, downloading_style, missing_style,
    unmonitored_missing_style, unmonitored_style, unreleased_style,
  };
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

  #[test]
  fn test_style_from_status_downloading_when_episode_has_no_file_and_download_is_downloading() {
    let download = DownloadRecord {
      status: DownloadStatus::Downloading,
      ..DownloadRecord::default()
    };
    let episode = Episode {
      has_file: false,
      monitored: true,
      ..Episode::default()
    };

    let style = style_from_status(Some(&download), &episode);

    assert_eq!(style, downloading_style());
  }

  #[test]
  fn test_style_from_status_awaiting_import_when_episode_has_no_file_and_download_is_completed() {
    let download = DownloadRecord {
      status: DownloadStatus::Completed,
      ..DownloadRecord::default()
    };
    let episode = Episode {
      has_file: false,
      monitored: true,
      ..Episode::default()
    };

    let style = style_from_status(Some(&download), &episode);

    assert_eq!(style, awaiting_import_style());
  }

  #[test]
  fn test_style_from_status_ignores_download_that_is_neither_downloading_nor_completed() {
    let download = DownloadRecord {
      status: DownloadStatus::Queued,
      ..DownloadRecord::default()
    };
    let episode = Episode {
      has_file: false,
      monitored: true,
      air_date_utc: None,
      ..Episode::default()
    };

    let style = style_from_status(Some(&download), &episode);

    assert_eq!(style, missing_style());
  }

  #[test]
  fn test_style_from_status_unmonitored_missing_when_episode_has_no_file_and_is_unmonitored() {
    let episode = Episode {
      has_file: false,
      monitored: false,
      air_date_utc: Some(Utc::now() + Duration::days(1)),
      ..Episode::default()
    };

    let style = style_from_status(None, &episode);

    assert_eq!(style, unmonitored_missing_style());
  }

  #[test]
  fn test_style_from_status_unreleased_when_episode_has_no_file_and_air_date_is_future() {
    let episode = Episode {
      has_file: false,
      monitored: true,
      air_date_utc: Some(Utc::now() + Duration::days(1)),
      ..Episode::default()
    };

    let style = style_from_status(None, &episode);

    assert_eq!(style, unreleased_style());
  }

  #[test]
  fn test_style_from_status_missing_when_episode_has_no_file_and_air_date_has_passed() {
    let episode = Episode {
      has_file: false,
      monitored: true,
      air_date_utc: Some(Utc::now() - Duration::days(1)),
      ..Episode::default()
    };

    let style = style_from_status(None, &episode);

    assert_eq!(style, missing_style());
  }

  #[test]
  fn test_style_from_status_missing_when_episode_has_no_file_and_no_air_date() {
    let episode = Episode {
      has_file: false,
      monitored: true,
      air_date_utc: None,
      ..Episode::default()
    };

    let style = style_from_status(None, &episode);

    assert_eq!(style, missing_style());
  }

  #[test]
  fn test_style_from_status_unmonitored_when_episode_has_file_and_is_unmonitored() {
    let episode = Episode {
      has_file: true,
      monitored: false,
      ..Episode::default()
    };

    let style = style_from_status(None, &episode);

    assert_eq!(style, unmonitored_style());
  }

  #[test]
  fn test_style_from_status_downloaded_when_episode_has_file_and_is_monitored() {
    let episode = Episode {
      has_file: true,
      monitored: true,
      ..Episode::default()
    };

    let style = style_from_status(None, &episode);

    assert_eq!(style, downloaded_style());
  }

  #[test]
  fn test_style_from_status_ignores_download_when_episode_has_file() {
    let download = DownloadRecord {
      status: DownloadStatus::Downloading,
      ..DownloadRecord::default()
    };
    let episode = Episode {
      has_file: true,
      monitored: true,
      ..Episode::default()
    };

    let style = style_from_status(Some(&download), &episode);

    assert_eq!(style, downloaded_style());
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
