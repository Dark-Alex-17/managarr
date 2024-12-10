#[cfg(test)]
mod tests {
  mod sonarr_tests {
    use pretty_assertions::{assert_eq, assert_str_eq};
    use tokio::sync::mpsc;

    use crate::{
      app::App,
      models::{
        servarr_data::sonarr::sonarr_data::ActiveSonarrBlock,
        sonarr_models::{Season, Series},
      },
      network::{sonarr_network::SonarrEvent, NetworkEvent},
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

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::SeriesHistory)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetSeriesHistory(None).into()
      );
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_series_details_block() {
      let (mut app, _) = construct_app_unit();

      app.data.sonarr_data.series.set_items(vec![Series {
        seasons: Some(vec![Season::default()]),
        ..Series::default()
      }]);

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::SeriesDetails)
        .await;

      assert!(!app.is_loading);
      assert!(!app.data.sonarr_data.seasons.items.is_empty());
      assert_eq!(app.tick_count, 0);
      assert!(!app.data.sonarr_data.prompt_confirm);
    }

    #[tokio::test]
    async fn test_dispatch_by_season_details_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::SeasonDetails)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetEpisodes(None).into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetEpisodeFiles(None).into()
      );
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetDownloads.into()
      );
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_season_history_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::SeasonHistory)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetSeasonHistory(None).into()
      );
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_manual_season_search_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::ManualSeasonSearch)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetSeasonReleases(None).into()
      );
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_episode_details_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::EpisodeDetails)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetEpisodeDetails(None).into()
      );
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_episode_file_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::EpisodeFile)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetEpisodeDetails(None).into()
      );
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_episode_history_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::EpisodeHistory)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetEpisodeHistory(None).into()
      );
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_dispatch_by_manual_episode_search_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::ManualEpisodeSearch)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::GetEpisodeReleases(None).into()
      );
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
        SonarrEvent::GetHistory(None).into()
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
        SonarrEvent::GetDownloads.into()
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

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::TestIndexer)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::TestIndexer(None).into()
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
        SonarrEvent::GetLogs(None).into()
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
    async fn test_dispatch_by_add_movie_search_results_block() {
      let (mut app, mut sync_network_rx) = construct_app_unit();

      app
        .dispatch_by_sonarr_block(&ActiveSonarrBlock::AddSeriesSearchResults)
        .await;

      assert!(app.is_loading);
      assert_eq!(
        sync_network_rx.recv().await.unwrap(),
        SonarrEvent::SearchNewSeries(None).into()
      );
      assert!(!app.data.sonarr_data.prompt_confirm);
      assert_eq!(app.tick_count, 0);
    }

    #[tokio::test]
    async fn test_check_for_sonarr_prompt_action_no_prompt_confirm() {
      let mut app = App::default();
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
        SonarrEvent::GetDownloads.into()
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
        SonarrEvent::GetDownloads.into()
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
        SonarrEvent::GetDownloads.into()
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
        SonarrEvent::GetDownloads.into()
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
        SonarrEvent::GetDownloads.into()
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
        SonarrEvent::GetDownloads.into()
      );
      assert!(app.is_loading);
    }

    #[tokio::test]
    async fn test_populate_seasons_table_unfiltered() {
      let mut app = App::default();
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
      let mut app = App::default();
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

    fn construct_app_unit<'a>() -> (App<'a>, mpsc::Receiver<NetworkEvent>) {
      let (sync_network_tx, sync_network_rx) = mpsc::channel::<NetworkEvent>(500);
      let mut app = App {
        network_tx: Some(sync_network_tx),
        tick_count: 1,
        is_first_render: false,
        ..App::default()
      };
      app.data.sonarr_data.prompt_confirm = true;

      (app, sync_network_rx)
    }
  }
}
