#[cfg(test)]
mod tests {
  use crate::cli::{
    sonarr::{list_command_handler::SonarrListCommand, SonarrCommand},
    Command,
  };
  use crate::Cli;
  use clap::CommandFactory;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_sonarr_command_from() {
    let command = SonarrCommand::List(SonarrListCommand::Series);

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Sonarr(command));
  }

  mod cli {
    use super::*;
    use clap::error::ErrorKind;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    fn test_commands_that_have_no_arg_requirements(
      #[values("clear-blocklist", "test-all-indexers")] subcommand: &str,
    ) {
      let result = Cli::command().try_get_matches_from(["managarr", "sonarr", subcommand]);

      assert!(result.is_ok());
    }

    #[test]
    fn test_mark_history_item_as_failed_requires_history_item_id() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "sonarr", "mark-history-item-as-failed"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_mark_history_item_as_failed_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "mark-history-item-as-failed",
        "--history-item-id",
        "1",
      ]);

      assert!(result.is_ok());
    }

    #[test]
    fn test_search_new_series_requires_query() {
      let result = Cli::command().try_get_matches_from(["managarr", "sonarr", "search-new-series"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_search_new_series_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "search-new-series",
        "--query",
        "halo",
      ]);

      assert!(result.is_ok());
    }

    #[test]
    fn test_start_task_requires_task_name() {
      let result = Cli::command().try_get_matches_from(["managarr", "sonarr", "start-task"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_start_task_task_name_validation() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "start-task",
        "--task-name",
        "test",
      ]);

      assert!(result.is_err());
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_start_task_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "start-task",
        "--task-name",
        "application-update-check",
      ]);

      assert!(result.is_ok());
    }

    #[test]
    fn test_test_indexer_requires_indexer_id() {
      let result = Cli::command().try_get_matches_from(["managarr", "sonarr", "test-indexer"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_test_indexer_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "test-indexer",
        "--indexer-id",
        "1",
      ]);

      assert!(result.is_ok());
    }

    #[test]
    fn test_toggle_episode_monitoring_requires_episode_id() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "sonarr", "toggle-episode-monitoring"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_toggle_episode_monitoring_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "toggle-episode-monitoring",
        "--episode-id",
        "1",
      ]);

      assert!(result.is_ok());
    }

    #[test]
    fn test_toggle_season_monitoring_requires_series_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "toggle-season-monitoring",
        "--season-number",
        "1",
      ]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_toggle_season_monitoring_requires_season_number() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "toggle-season-monitoring",
        "--series-id",
        "1",
      ]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_toggle_season_monitoring_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "toggle-season-monitoring",
        "--series-id",
        "1",
        "--season-number",
        "1",
      ]);

      assert!(result.is_ok());
    }
  }

  mod handler {
    use std::sync::Arc;

    use mockall::predicate::eq;
    use serde_json::json;
    use tokio::sync::Mutex;

    use crate::{
      app::App,
      cli::{
        sonarr::{
          add_command_handler::SonarrAddCommand, delete_command_handler::SonarrDeleteCommand,
          download_command_handler::SonarrDownloadCommand, edit_command_handler::SonarrEditCommand,
          get_command_handler::SonarrGetCommand, list_command_handler::SonarrListCommand,
          manual_search_command_handler::SonarrManualSearchCommand,
          refresh_command_handler::SonarrRefreshCommand,
          trigger_automatic_search_command_handler::SonarrTriggerAutomaticSearchCommand,
          SonarrCliHandler, SonarrCommand,
        },
        CliCommandHandler,
      },
      models::{
        sonarr_models::{
          BlocklistItem, BlocklistResponse, IndexerSettings, Series, SonarrReleaseDownloadBody,
          SonarrSerdeable, SonarrTaskName,
        },
        Serdeable,
      },
      network::{sonarr_network::SonarrEvent, MockNetworkTrait, NetworkEvent},
    };

    #[tokio::test]
    async fn test_handle_clear_blocklist_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(SonarrEvent::GetBlocklist.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::BlocklistResponse(
            BlocklistResponse {
              records: vec![BlocklistItem::default()],
            },
          )))
        });
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(SonarrEvent::ClearBlocklist.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let claer_blocklist_command = SonarrCommand::ClearBlocklist;

      let result = SonarrCliHandler::with(&app_arc, claer_blocklist_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_mark_history_item_as_failed_command() {
      let expected_history_item_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::MarkHistoryItemAsFailed(expected_history_item_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let mark_history_item_as_failed_command =
        SonarrCommand::MarkHistoryItemAsFailed { history_item_id: 1 };

      let result = SonarrCliHandler::with(
        &app_arc,
        mark_history_item_as_failed_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_sonarr_cli_handler_delegates_add_commands_to_the_add_command_handler() {
      let expected_tag_name = "test".to_owned();
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::AddTag(expected_tag_name.clone()).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let add_tag_command = SonarrCommand::Add(SonarrAddCommand::Tag {
        name: expected_tag_name,
      });

      let result = SonarrCliHandler::with(&app_arc, add_tag_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_sonarr_cli_handler_delegates_delete_commands_to_the_delete_command_handler() {
      let expected_blocklist_item_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::DeleteBlocklistItem(expected_blocklist_item_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let delete_blocklist_item_command =
        SonarrCommand::Delete(SonarrDeleteCommand::BlocklistItem {
          blocklist_item_id: 1,
        });

      let result =
        SonarrCliHandler::with(&app_arc, delete_blocklist_item_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_sonarr_cli_handler_delegates_download_commands_to_the_download_command_handler() {
      let expected_params = SonarrReleaseDownloadBody {
        guid: "1234".to_owned(),
        indexer_id: 1,
        series_id: Some(1),
        ..SonarrReleaseDownloadBody::default()
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::DownloadRelease(expected_params).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let download_series_release_command =
        SonarrCommand::Download(SonarrDownloadCommand::Series {
          guid: "1234".to_owned(),
          indexer_id: 1,
          series_id: 1,
        });

      let result =
        SonarrCliHandler::with(&app_arc, download_series_release_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_sonarr_cli_handler_delegates_edit_commands_to_the_edit_command_handler() {
      let expected_edit_all_indexer_settings = IndexerSettings {
        id: 1,
        maximum_size: 1,
        minimum_age: 1,
        retention: 1,
        rss_sync_interval: 1,
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::GetAllIndexerSettings.into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::IndexerSettings(
            IndexerSettings {
              id: 1,
              maximum_size: 2,
              minimum_age: 2,
              retention: 2,
              rss_sync_interval: 2,
            },
          )))
        });
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::EditAllIndexerSettings(Some(expected_edit_all_indexer_settings)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let edit_all_indexer_settings_command =
        SonarrCommand::Edit(SonarrEditCommand::AllIndexerSettings {
          maximum_size: Some(1),
          minimum_age: Some(1),
          retention: Some(1),
          rss_sync_interval: Some(1),
        });

      let result = SonarrCliHandler::with(
        &app_arc,
        edit_all_indexer_settings_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_sonarr_cli_handler_delegates_manual_search_commands_to_the_manual_search_command_handler(
    ) {
      let expected_episode_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::GetEpisodeReleases(Some(expected_episode_id)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let manual_episode_search_command =
        SonarrCommand::ManualSearch(SonarrManualSearchCommand::Episode { episode_id: 1 });

      let result =
        SonarrCliHandler::with(&app_arc, manual_episode_search_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_sonarr_cli_handler_delegates_trigger_automatic_search_commands_to_the_trigger_automatic_search_command_handler(
    ) {
      let expected_episode_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::TriggerAutomaticEpisodeSearch(Some(expected_episode_id)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let manual_episode_search_command =
        SonarrCommand::TriggerAutomaticSearch(SonarrTriggerAutomaticSearchCommand::Episode {
          episode_id: 1,
        });

      let result =
        SonarrCliHandler::with(&app_arc, manual_episode_search_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_sonarr_cli_handler_delegates_get_commands_to_the_get_command_handler() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(SonarrEvent::GetStatus.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let get_system_status_command = SonarrCommand::Get(SonarrGetCommand::SystemStatus);

      let result = SonarrCliHandler::with(&app_arc, get_system_status_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_sonarr_cli_handler_delegates_list_commands_to_the_list_command_handler() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(SonarrEvent::ListSeries.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::SeriesVec(vec![
            Series::default(),
          ])))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let list_series_command = SonarrCommand::List(SonarrListCommand::Series);

      let result = SonarrCliHandler::with(&app_arc, list_series_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_sonarr_cli_handler_delegates_refresh_commands_to_the_refresh_command_handler() {
      let expected_series_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::UpdateAndScanSeries(Some(expected_series_id)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let refresh_series_command =
        SonarrCommand::Refresh(SonarrRefreshCommand::Series { series_id: 1 });

      let result = SonarrCliHandler::with(&app_arc, refresh_series_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_search_new_series_command() {
      let expected_search_query = "halo".to_owned();
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::SearchNewSeries(Some(expected_search_query)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let search_new_series_command = SonarrCommand::SearchNewSeries {
        query: "halo".to_owned(),
      };

      let result = SonarrCliHandler::with(&app_arc, search_new_series_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_start_task_command() {
      let expected_task_name = SonarrTaskName::ApplicationUpdateCheck;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::StartTask(Some(expected_task_name)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let start_task_command = SonarrCommand::StartTask {
        task_name: SonarrTaskName::ApplicationUpdateCheck,
      };

      let result = SonarrCliHandler::with(&app_arc, start_task_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_test_indexer_command() {
      let expected_indexer_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::TestIndexer(Some(expected_indexer_id)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let test_indexer_command = SonarrCommand::TestIndexer { indexer_id: 1 };

      let result = SonarrCliHandler::with(&app_arc, test_indexer_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_test_all_indexers_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(SonarrEvent::TestAllIndexers.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let test_all_indexers_command = SonarrCommand::TestAllIndexers;

      let result = SonarrCliHandler::with(&app_arc, test_all_indexers_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_toggle_episode_monitoring_command() {
      let expected_episode_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::ToggleEpisodeMonitoring(Some(expected_episode_id)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let toggle_episode_monitoring_command =
        SonarrCommand::ToggleEpisodeMonitoring { episode_id: 1 };

      let result = SonarrCliHandler::with(
        &app_arc,
        toggle_episode_monitoring_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_toggle_season_monitoring_command() {
      let expected_series_id = 1;
      let expected_season_number = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::ToggleSeasonMonitoring(Some((expected_series_id, expected_season_number)))
            .into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let toggle_season_monitoring_command = SonarrCommand::ToggleSeasonMonitoring {
        series_id: 1,
        season_number: 1,
      };

      let result = SonarrCliHandler::with(
        &app_arc,
        toggle_season_monitoring_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert!(result.is_ok());
    }
  }
}
