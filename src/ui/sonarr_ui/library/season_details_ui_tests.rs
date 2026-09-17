#[cfg(test)]
mod tests {
  use chrono::{Duration, Utc};
  use pretty_assertions::assert_eq;
  use ratatui::widgets::{Cell, Row};
  use serde_json::Number;
  use strum::IntoEnumIterator;

  use crate::app::App;
  use crate::models::servarr_data::sonarr::sonarr_data::{
    ActiveSonarrBlock, EPISODE_DETAILS_BLOCKS, SEASON_DETAILS_BLOCKS,
  };
  use crate::models::sonarr_models::{DownloadRecord, DownloadStatus, Episode};
  use crate::models::stateful_table::StatefulTable;
  use crate::network::sonarr_network::sonarr_network_test_utils::test_utils::download_record;
  use crate::ui::DrawUi;
  use crate::ui::sonarr_ui::library::season_details_ui::{
    SeasonDetailsUi, decorate_with_row_style,
  };
  use crate::ui::styles::ManagarrStyle;
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

  #[test]
  fn test_decorate_with_row_style_downloaded_when_episode_has_file() {
    let episode = Episode {
      id: 5,
      has_file: true,
      monitored: true,
      ..Episode::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_with_row_style(&[], &episode, row.clone());

    assert_eq!(style, row.downloaded());
  }

  #[test]
  fn test_decorate_with_row_style_unmonitored_when_episode_has_file_and_is_unmonitored() {
    let episode = Episode {
      id: 5,
      has_file: true,
      monitored: false,
      ..Episode::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_with_row_style(&[], &episode, row.clone());

    assert_eq!(style, row.unmonitored());
  }

  #[test]
  fn test_decorate_with_row_style_missing_when_episode_has_no_file() {
    let episode = Episode {
      id: 5,
      has_file: false,
      monitored: true,
      ..Episode::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_with_row_style(&[], &episode, row.clone());

    assert_eq!(style, row.missing());
  }

  #[test]
  fn test_decorate_with_row_style_unmonitored_missing_when_episode_has_no_file_and_is_unmonitored()
  {
    let episode = Episode {
      id: 5,
      has_file: false,
      monitored: false,
      ..Episode::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_with_row_style(&[], &episode, row.clone());

    assert_eq!(style, row.unmonitored_missing());
  }

  #[test]
  fn test_decorate_with_row_style_unreleased_when_episode_has_no_file_and_air_date_is_in_the_future()
   {
    let episode = Episode {
      id: 5,
      has_file: false,
      monitored: true,
      air_date_utc: Some(Utc::now() + Duration::days(1)),
      ..Episode::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_with_row_style(&[], &episode, row.clone());

    assert_eq!(style, row.unreleased());
  }

  #[test]
  fn test_decorate_with_row_style_missing_when_episode_has_no_file_and_air_date_is_in_the_past() {
    let episode = Episode {
      id: 5,
      has_file: false,
      monitored: true,
      air_date_utc: Some(Utc::now() - Duration::days(1)),
      ..Episode::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_with_row_style(&[], &episode, row.clone());

    assert_eq!(style, row.missing());
  }

  #[test]
  fn test_decorate_with_row_style_unmonitored_missing_when_unmonitored_episode_has_no_file_and_air_date_is_in_the_future()
   {
    let episode = Episode {
      id: 5,
      has_file: false,
      monitored: false,
      air_date_utc: Some(Utc::now() + Duration::days(1)),
      ..Episode::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_with_row_style(&[], &episode, row.clone());

    assert_eq!(style, row.unmonitored_missing());
  }

  #[test]
  fn test_decorate_with_row_style_downloading_when_episode_has_no_file_and_is_downloading() {
    let downloads_vec = vec![episode_download(5, DownloadStatus::Downloading)];
    let episode = Episode {
      id: 5,
      has_file: false,
      monitored: true,
      ..Episode::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_with_row_style(&downloads_vec, &episode, row.clone());

    assert_eq!(style, row.downloading());
  }

  #[test]
  fn test_decorate_with_row_style_awaiting_import_when_episode_has_no_file_and_download_is_completed()
   {
    let downloads_vec = vec![episode_download(5, DownloadStatus::Completed)];
    let episode = Episode {
      id: 5,
      has_file: false,
      monitored: true,
      ..Episode::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_with_row_style(&downloads_vec, &episode, row.clone());

    assert_eq!(style, row.awaiting_import());
  }

  #[test]
  fn test_decorate_with_row_style_missing_when_episode_download_is_neither_downloading_nor_completed()
   {
    let downloads_vec = vec![episode_download(5, DownloadStatus::Queued)];
    let episode = Episode {
      id: 5,
      has_file: false,
      monitored: true,
      ..Episode::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_with_row_style(&downloads_vec, &episode, row.clone());

    assert_eq!(style, row.missing());
  }

  #[test]
  fn test_decorate_with_row_style_missing_when_only_a_different_episode_is_downloading() {
    let downloads_vec = vec![episode_download(6, DownloadStatus::Downloading)];
    let episode = Episode {
      id: 5,
      has_file: false,
      monitored: true,
      ..Episode::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_with_row_style(&downloads_vec, &episode, row.clone());

    assert_eq!(style, row.missing());
  }

  #[test]
  fn test_decorate_with_row_style_missing_when_download_has_no_episode_id() {
    let downloads_vec = vec![DownloadRecord {
      episode_id: None,
      ..episode_download(5, DownloadStatus::Downloading)
    }];
    let episode = Episode {
      id: 5,
      has_file: false,
      monitored: true,
      ..Episode::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_with_row_style(&downloads_vec, &episode, row.clone());

    assert_eq!(style, row.missing());
  }

  #[test]
  fn test_decorate_with_row_style_downloading_when_unmonitored_episode_has_no_file_and_is_downloading()
   {
    let downloads_vec = vec![episode_download(5, DownloadStatus::Downloading)];
    let episode = Episode {
      id: 5,
      has_file: false,
      monitored: false,
      ..Episode::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_with_row_style(&downloads_vec, &episode, row.clone());

    assert_eq!(style, row.downloading());
  }

  #[test]
  fn test_decorate_with_row_style_downloaded_when_episode_has_file_and_is_downloading() {
    let downloads_vec = vec![episode_download(5, DownloadStatus::Downloading)];
    let episode = Episode {
      id: 5,
      has_file: true,
      monitored: true,
      ..Episode::default()
    };
    let row = Row::new(vec![Cell::from("test".to_owned())]);

    let style = decorate_with_row_style(&downloads_vec, &episode, row.clone());

    assert_eq!(style, row.downloaded());
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

  fn episode_download(episode_id: i64, status: DownloadStatus) -> DownloadRecord {
    DownloadRecord {
      status,
      id: 9,
      episode_id: Some(Number::from(episode_id)),
      ..download_record()
    }
  }
}
