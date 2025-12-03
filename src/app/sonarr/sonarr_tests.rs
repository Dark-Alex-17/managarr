#[cfg(test)]
mod tests {
  mod sonarr_tests {
    use pretty_assertions::{assert_eq, assert_str_eq};
    use tokio::sync::mpsc;

    use crate::models::servarr_data::sonarr::sonarr_data::sonarr_test_utils::utils::create_test_sonarr_data;
    use crate::models::servarr_models::Indexer;
    use crate::models::sonarr_models::Episode;
    use crate::{
      app::App,
      models::{
        servarr_data::sonarr::{
          modals::{EpisodeDetailsModal, SeasonDetailsModal},
          sonarr_data::ActiveSonarrBlock,
        },
        sonarr_models::{Season, Series, SonarrRelease},
      },
      network::{NetworkEvent, sonarr_network::SonarrEvent},
    };

    #[tokio::test]
    async fn test_dispatch_by_blocklist_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::Blocklist)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::ListSeries.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetBlocklist.into()
      );
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_series_history_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();
      app.data.sonarr_data.series.set_items(vec![Series {
        id: 1,
        ..Series::default()
      }]);

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::SeriesHistory)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetSeriesHistory(1).into()
      );
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_series_details_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app.data.sonarr_data.series.set_items(vec![Series {
        seasons: Some(vec![Season::default()]),
        ..Series::default()
      }]);

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::SeriesDetails)
        .await;

      assert!(!app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::ListSeries.into()
      );
      assert!(!app.data.sonarr_data.seasons.items.is_empty());
      assert_eq!(app.tick_count, 0);
      assert!(!app.data.sonarr_data.prompt_confirm);
    }

    #[tokio::test]
    async fn test_dispatch_by_season_details_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();
      app.data.sonarr_data.series.set_items(vec![Series {
        id: 1,
        ..Series::default()
      }]);

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::SeasonDetails)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetEpisodes(1).into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetEpisodeFiles(1).into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetDownloads(500).into()
      );
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_season_history_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();
      app.data.sonarr_data.series.set_items(vec![Series {
        id: 1,
        ..Series::default()
      }]);
      app.data.sonarr_data.seasons.set_items(vec![Season {
        season_number: 1,
        ..Season::default()
      }]);

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::SeasonHistory)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetSeasonHistory((1, 1)).into()
      );
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_season_history_block_no_op_when_seasons_table_is_empty() {
      let (mut app, _) = construct_app_unit();
      app.data.sonarr_data.series.set_items(vec![Series {
        id: 1,
        ..Series::default()
      }]);

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::SeasonHistory)
        .await;

      assert!(!app.is_loading);
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_manual_season_search_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();
      app.data.sonarr_data.season_details_modal = Some(SeasonDetailsModal::default());
      app.data.sonarr_data.series.set_items(vec![Series {
        id: 1,
        ..Series::default()
      }]);
      app.data.sonarr_data.seasons.set_items(vec![Season {
        season_number: 1,
        ..Season::default()
      }]);

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::ManualSeasonSearch)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetSeasonReleases((1, 1)).into()
      );
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_manual_season_search_block_is_loading() {
      let mut app = App {
        is_loading: true,
        ..App::test_default()
      };

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::ManualSeasonSearch)
        .await;

      assert!(app.is_loading);
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_manual_season_search_block_season_releases_non_empty() {
      let mut app = App::test_default();
      let mut season_details_modal = SeasonDetailsModal::default();
      season_details_modal
        .season_releases
        .set_items(vec![SonarrRelease::default()]);
      app.data.sonarr_data.season_details_modal = Some(season_details_modal);

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::ManualSeasonSearch)
        .await;

      assert!(!app.is_loading);
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_episode_details_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();
      app.data.sonarr_data = create_test_sonarr_data();

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::EpisodeDetails)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetEpisodeDetails(0).into()
      );
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_episode_file_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();
      app.data.sonarr_data = create_test_sonarr_data();

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::EpisodeFile)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetEpisodeDetails(0).into()
      );
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_episode_history_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();
      let mut season_details_modal = SeasonDetailsModal::default();
      season_details_modal.episodes.set_items(vec![Episode {
        id: 1,
        ..Episode::default()
      }]);
      app.data.sonarr_data.season_details_modal = Some(season_details_modal);

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::EpisodeHistory)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetEpisodeHistory(1).into()
      );
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_manual_episode_search_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();
      let mut season_details_modal = SeasonDetailsModal {
        episode_details_modal: Some(EpisodeDetailsModal::default()),
        ..SeasonDetailsModal::default()
      };
      season_details_modal.episodes.set_items(vec![Episode {
        id: 1,
        ..Episode::default()
      }]);
      app.data.sonarr_data.season_details_modal = Some(season_details_modal);

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::ManualEpisodeSearch)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetEpisodeReleases(1).into()
      );
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_manual_episode_search_block_is_loading() {
      let mut app = App {
        is_loading: true,
        ..App::test_default()
      };

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::ManualEpisodeSearch)
        .await;

      assert!(app.is_loading);
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_manual_episode_search_block_episode_releases_non_empty() {
      let mut app = App::test_default();
      let mut episode_details_modal = EpisodeDetailsModal::default();
      episode_details_modal
        .episode_releases
        .set_items(vec![SonarrRelease::default()]);
      let season_details_modal = SeasonDetailsModal {
        episode_details_modal: Some(episode_details_modal),
        ..SeasonDetailsModal::default()
      };
      app.data.sonarr_data.season_details_modal = Some(season_details_modal);

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::ManualEpisodeSearch)
        .await;

      assert!(!app.is_loading);
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_history_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::History)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetHistory(500).into()
      );
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_downloads_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::Downloads)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetDownloads(500).into()
      );
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_root_folders_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::RootFolders)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetRootFolders.into()
      );
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_series_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::Series)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetQualityProfiles.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetLanguageProfiles.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetTags.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::ListSeries.into()
      );
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_indexers_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::Indexers)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetTags.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetIndexers.into()
      );
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_all_indexer_settings_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::AllIndexerSettingsPrompt)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetAllIndexerSettings.into()
      );
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_test_indexer_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();
      app.data.sonarr_data.indexers.set_items(vec![Indexer {
        id: 1,
        ..Indexer::default()
      }]);

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::TestIndexer)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::TestIndexer(1).into()
      );
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_test_all_indexers_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::TestAllIndexers)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::TestAllIndexers.into()
      );
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_system_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::System)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetTasks.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetQueuedEvents.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetLogs(500).into()
      );
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_system_updates_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::SystemUpdates)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetUpdates.into()
      );
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_add_series_search_results_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();
      app.data.sonarr_data.add_series_search = Some("test search".into());

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::AddSeriesSearchResults)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::SearchNewSeries("test search".into()).into()
      );
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_check_for_sonarr_prompt_action_no_prompt_confirm() {
      let mut app = App::test_default();
      app.data.sonarr_data.prompt_confirm = false;

      app.check_for_sonarr_prompt_action().await;

      assert!(!app.data.sonarr_data.prompt_confirm);
      assert!(!app.should_refresh);
    }

    #[tokio::test]
    async fn test_check_for_sonarr_prompt_action() {
      let (mut app, mut sync_network_rx) = construct_app_unit();
      app.data.sonarr_data.prompt_confirm_action = Some(SonarrEvent::GetStatus);

      app.check_for_sonarr_prompt_action().await;

      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetStatus.into()
      );
      assert!(app.should_refresh);
      assert_eq!(app.data.sonarr_data.prompt_confirm_action, None);
    }

    #[tokio::test]
    async fn test_sonarr_refresh_metadata() {
      let (mut app, mut sync_network_rx) = construct_app_unit();
      app.is_routing = true;

      app.refresh_sonarr_metadata().await;

      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetQualityProfiles.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetLanguageProfiles.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetTags.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetRootFolders.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetDownloads(500).into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetDiskSpace.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetStatus.into()
      );
      assert!(app.is_loading);
    }

    #[tokio::test]
    async fn test_sonarr_on_tick_first_render() {
      let (mut app, mut sync_network_rx) = construct_app_unit();
      app.is_first_render = true;

      app.sonarr_on_tick(ActiveSonarrBlock::Downloads).await;

      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetQualityProfiles.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetLanguageProfiles.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetTags.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetRootFolders.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetDownloads(500).into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetDiskSpace.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetStatus.into()
      );
      assert!(app.is_loading);
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert!(!app.is_first_render);
    }

    #[tokio::test]
    async fn test_sonarr_on_tick_routing() {
      let (mut app, mut sync_network_rx) = construct_app_unit();
      app.is_routing = true;
      app.should_refresh = true;

      app.sonarr_on_tick(ActiveSonarrBlock::Downloads).await;

      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetDownloads(500).into()
      );
      assert!(!app.data.sonarr_data.prompt_confirm);
    }

    #[tokio::test]
    async fn test_sonarr_on_tick_routing_while_long_request_is_running_should_cancel_request() {
      let (mut app, _) = construct_app_unit();
      app.is_routing = true;
      app.should_refresh = false;

      app.sonarr_on_tick(ActiveSonarrBlock::Downloads).await;

      assert!(app.cancellation_token.is_cancelled());
    }

    #[tokio::test]
    async fn test_sonarr_on_tick_should_refresh() {
      let (mut app, mut sync_network_rx) = construct_app_unit();
      app.should_refresh = true;

      app.sonarr_on_tick(ActiveSonarrBlock::Downloads).await;

      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetDownloads(500).into()
      );
      assert!(app.should_refresh);
      assert!(!app.data.sonarr_data.prompt_confirm);
    }

    #[tokio::test]
    async fn test_sonarr_on_tick_should_refresh_does_not_cancel_prompt_requests() {
      let (mut app, mut sync_network_rx) = construct_app_unit();
      app.is_loading = true;
      app.is_routing = true;
      app.should_refresh = true;

      app.sonarr_on_tick(ActiveSonarrBlock::Downloads).await;

      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetDownloads(500).into()
      );
      assert!(app.is_loading);
      assert!(app.should_refresh);
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert!(!app.cancellation_token.is_cancelled());
    }

    #[tokio::test]
    async fn test_sonarr_on_tick_network_tick_frequency() {
      let (mut app, mut sync_network_rx) = construct_app_unit();
      app.tick_count = 2;
      app.tick_until_poll = 2;

      app.sonarr_on_tick(ActiveSonarrBlock::Downloads).await;

      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetQualityProfiles.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetLanguageProfiles.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetTags.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetRootFolders.into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetDownloads(500).into()
      );
      assert!(app.is_loading);
    }

    #[tokio::test]
    async fn test_populate_seasons_table_unfiltered() {
      let mut app = App::test_default();
      app.data.sonarr_data.series.set_items(vec![Series {
        seasons: Some(vec![Season::default()]),
        ..Series::default()
      }]);

      app.populate_seasons_table().await;

      assert!(!app.data.sonarr_data.seasons.items.is_empty());
      assert_str_eq!(
        app.data.sonarr_data.seasons.items[0]
          .title
          .as_ref()
          .unwrap(),
        "Season 0"
      );
    }

    #[tokio::test]
    async fn test_populate_seasons_table_filtered() {
      let mut app = App::test_default();
      app.data.sonarr_data.series.set_filtered_items(vec![Series {
        seasons: Some(vec![Season::default()]),
        ..Series::default()
      }]);

      app.populate_seasons_table().await;

      assert!(!app.data.sonarr_data.seasons.items.is_empty());
      assert_str_eq!(
        app.data.sonarr_data.seasons.items[0]
          .title
          .as_ref()
          .unwrap(),
        "Season 0"
      );
    }

    #[tokio::test]
    async fn test_extract_episode_id() {
      let mut app = App::test_default();
      let mut season_details_modal = SeasonDetailsModal::default();
      season_details_modal.episodes.set_items(vec![Episode {
        id: 1,
        ..Episode::default()
      }]);
      app.data.sonarr_data.season_details_modal = Some(season_details_modal);

      assert_eq!(app.extract_episode_id().await, 1);
    }

    #[tokio::test]
    #[should_panic(expected = "Season details have not been loaded")]
    async fn test_extract_episode_id_requires_season_details_modal_to_be_some() {
      let app = App::test_default();

      assert_eq!(app.extract_episode_id().await, 0);
    }

    #[tokio::test]
    async fn test_extract_series_id() {
      let mut app = App::test_default();
      app.data.sonarr_data.series.set_items(vec![Series {
        id: 1,
        ..Series::default()
      }]);

      assert_eq!(app.extract_series_id().await, 1);
    }

    #[tokio::test]
    async fn test_extract_series_id_season_number_tuple() {
      let mut app = App::test_default();
      app.data.sonarr_data.series.set_items(vec![Series {
        id: 1,
        ..Series::default()
      }]);
      app.data.sonarr_data.seasons.set_items(vec![Season {
        season_number: 1,
        ..Season::default()
      }]);

      assert_eq!(app.extract_series_id_season_number_tuple().await, (1, 1));
    }

    #[tokio::test]
    async fn test_extract_add_new_series_search_query() {
      let mut app = App::test_default();
      app.data.sonarr_data.add_series_search = Some("test search".into());

      assert_str_eq!(
        app.extract_add_new_series_search_query().await,
        "test search"
      );
    }

    #[tokio::test]
    #[should_panic(expected = "Add series search is empty")]
    async fn test_extract_add_new_series_search_query_panics_when_the_query_is_not_set() {
      let app = App::test_default();

      app.extract_add_new_series_search_query().await;
    }

    #[tokio::test]
    async fn test_extract_sonarr_indexer_id() {
      let mut app = App::test_default();
      app.data.sonarr_data.indexers.set_items(vec![Indexer {
        id: 1,
        ..Indexer::default()
      }]);

      assert_eq!(app.extract_sonarr_indexer_id().await, 1);
    }

    fn construct_app_unit<'a>() -> (App<'a>, mpsc::Receiver<NetworkEvent>) {
      let (sync_network_tx, sync_network_rx) = mpsc::channel::<NetworkEvent>(500);
      let mut app = App {
        network_tx: Some(sync_network_tx),
        tick_count: 1,
        is_first_render: false,
        ..App::test_default()
      };
      app.data.sonarr_data.prompt_confirm = true;

      (app, sync_network_rx)
    }
  }
}
