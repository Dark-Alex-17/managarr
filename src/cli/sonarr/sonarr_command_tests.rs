#[cfg(test)]
mod tests {
  use crate::cli::{
    sonarr::{list_command_handler::SonarrListCommand, SonarrCommand},
    Command,
  };
  use crate::Cli;
  use clap::CommandFactory;

  #[test]
  fn test_sonarr_command_from() {
    let command = SonarrCommand::List(SonarrListCommand::Series);

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Sonarr(command));
  }

  mod cli {
    use super::*;
    use clap::error::ErrorKind;
    use rstest::rstest;

    #[rstest]
    fn test_commands_that_have_no_arg_requirements(
      #[values("clear-blocklist", "test-all-indexers")] subcommand: &str,
    ) {
      let result = Cli::command().try_get_matches_from(["managarr", "sonarr", subcommand]);

      assert!(result.is_ok());
    }

    #[rstest]
    fn test_mark_history_item_as_failed_requires_history_item_id() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "sonarr", "mark-history-item-as-failed"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[rstest]
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

    #[rstest]
    fn test_manual_season_search_requires_series_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "manual-season-search",
        "--season-number",
        "1",
      ]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[rstest]
    fn test_manual_season_search_requires_season_number() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "manual-season-search",
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
    fn test_manual_season_search_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "manual-season-search",
        "--series-id",
        "1",
        "--season-number",
        "1",
      ]);

      assert!(result.is_ok());
    }

    #[rstest]
    fn test_manual_episode_search_requires_episode_id() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "sonarr", "manual-episode-search"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_manual_episode_search_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "manual-episode-search",
        "--episode-id",
        "1",
      ]);

      assert!(result.is_ok());
    }

    #[rstest]
    fn test_start_task_requires_task_name() {
      let result = Cli::command().try_get_matches_from(["managarr", "sonarr", "start-task"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[rstest]
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

    #[rstest]
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

    #[rstest]
    fn test_trigger_automatic_series_search_requires_series_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "trigger-automatic-series-search",
      ]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_trigger_automatic_series_search_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "trigger-automatic-series-search",
        "--series-id",
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
          get_command_handler::SonarrGetCommand, list_command_handler::SonarrListCommand,
          SonarrCliHandler, SonarrCommand,
        },
        CliCommandHandler,
      },
      models::{
        sonarr_models::{
          BlocklistItem, BlocklistResponse, Series, SonarrSerdeable, SonarrTaskName,
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
    async fn test_manual_episode_search_command() {
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
      let manual_episode_search_command = SonarrCommand::ManualEpisodeSearch { episode_id: 1 };

      let result =
        SonarrCliHandler::with(&app_arc, manual_episode_search_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_manual_season_search_command() {
      let expected_series_id = 1;
      let expected_season_number = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::GetSeasonReleases(Some((expected_series_id, expected_season_number))).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let manual_season_search_command = SonarrCommand::ManualSeasonSearch {
        series_id: 1,
        season_number: 1,
      };

      let result =
        SonarrCliHandler::with(&app_arc, manual_season_search_command, &mut mock_network)
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
          SonarrEvent::DeleteBlocklistItem(Some(expected_blocklist_item_id)).into(),
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
    async fn test_trigger_automatic_series_search_command() {
      let expected_series_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::TriggerAutomaticSeriesSearch(Some(expected_series_id)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let trigger_automatic_series_search_command =
        SonarrCommand::TriggerAutomaticSeriesSearch { series_id: 1 };

      let result = SonarrCliHandler::with(
        &app_arc,
        trigger_automatic_series_search_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert!(result.is_ok());
    }
  }
}
