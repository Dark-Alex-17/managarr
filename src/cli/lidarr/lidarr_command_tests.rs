#[cfg(test)]
mod tests {
  use crate::Cli;
  use crate::cli::{
    Command,
    lidarr::{LidarrCommand, list_command_handler::LidarrListCommand},
  };
  use clap::CommandFactory;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_lidarr_command_from() {
    let command = LidarrCommand::List(LidarrListCommand::Artists);

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Lidarr(command));
  }

  mod cli {
    use super::*;
    use clap::error::ErrorKind;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    fn test_commands_that_have_no_arg_requirements(
      #[values("test-all-indexers")] subcommand: &str,
    ) {
      let result = Cli::command().try_get_matches_from(["managarr", "lidarr", subcommand]);

      assert_ok!(&result);
    }

    #[test]
    fn test_list_artists_has_no_arg_requirements() {
      let result = Cli::command().try_get_matches_from(["managarr", "lidarr", "list", "artists"]);

      assert_ok!(&result);
    }

    #[test]
    fn test_lidarr_list_subcommand_requires_subcommand() {
      let result = Cli::command().try_get_matches_from(["managarr", "lidarr", "list"]);

      assert_err!(&result);
    }

    #[test]
    fn test_lidarr_add_subcommand_requires_subcommand() {
      let result = Cli::command().try_get_matches_from(["managarr", "lidarr", "add"]);

      assert_err!(&result);
    }

    #[test]
    fn test_lidarr_delete_subcommand_requires_subcommand() {
      let result = Cli::command().try_get_matches_from(["managarr", "lidarr", "delete"]);

      assert_err!(&result);
    }

    #[test]
    fn test_toggle_artist_monitoring_requires_artist_id() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "lidarr", "toggle-artist-monitoring"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_toggle_artist_monitoring_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "lidarr",
        "toggle-artist-monitoring",
        "--artist-id",
        "1",
      ]);

      assert_ok!(&result);
    }

    #[test]
    fn test_toggle_album_monitoring_requires_album_id() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "lidarr", "toggle-album-monitoring"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_toggle_album_monitoring_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "lidarr",
        "toggle-album-monitoring",
        "--album-id",
        "1",
      ]);

      assert_ok!(&result);
    }

    #[test]
    fn test_search_new_artist_requires_query() {
      let result = Cli::command().try_get_matches_from(["managarr", "lidarr", "search-new-artist"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_search_new_artist_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "lidarr",
        "search-new-artist",
        "--query",
        "test query",
      ]);

      assert_ok!(&result);
    }

    #[test]
    fn test_start_task_requires_task_name() {
      let result = Cli::command().try_get_matches_from(["managarr", "lidarr", "start-task"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_start_task_task_name_validation() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "lidarr",
        "start-task",
        "--task-name",
        "test",
      ]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_start_task_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "lidarr",
        "start-task",
        "--task-name",
        "application-update-check",
      ]);

      assert_ok!(&result);
    }

    #[test]
    fn test_mark_history_item_as_failed_requires_history_item_id() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "lidarr", "mark-history-item-as-failed"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_mark_history_item_as_failed_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "lidarr",
        "mark-history-item-as-failed",
        "--history-item-id",
        "1",
      ]);

      assert_ok!(&result);
    }

    #[test]
    fn test_test_indexer_requires_indexer_id() {
      let result = Cli::command().try_get_matches_from(["managarr", "lidarr", "test-indexer"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_test_indexer_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "lidarr",
        "test-indexer",
        "--indexer-id",
        "1",
      ]);

      assert_ok!(&result);
    }
  }

  mod handler {
    use std::sync::Arc;

    use mockall::predicate::eq;
    use serde_json::json;
    use tokio::sync::Mutex;

    use crate::cli::lidarr::add_command_handler::LidarrAddCommand;
    use crate::cli::lidarr::edit_command_handler::LidarrEditCommand;
    use crate::cli::lidarr::get_command_handler::LidarrGetCommand;
    use crate::cli::lidarr::manual_search_command_handler::LidarrManualSearchCommand;
    use crate::cli::lidarr::refresh_command_handler::LidarrRefreshCommand;
    use crate::cli::lidarr::trigger_automatic_search_command_handler::LidarrTriggerAutomaticSearchCommand;
    use crate::models::lidarr_models::LidarrTaskName;
    use crate::models::servarr_models::IndexerSettings;
    use crate::{
      app::App,
      cli::{
        CliCommandHandler,
        lidarr::{
          LidarrCliHandler, LidarrCommand, delete_command_handler::LidarrDeleteCommand,
          list_command_handler::LidarrListCommand,
        },
      },
      models::{
        Serdeable,
        lidarr_models::{Artist, DeleteParams, LidarrSerdeable},
      },
      network::{MockNetworkTrait, NetworkEvent, lidarr_network::LidarrEvent},
    };

    #[tokio::test]
    async fn test_lidarr_cli_handler_delegates_add_commands_to_the_add_command_handler() {
      let expected_tag_name = "test".to_owned();
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          LidarrEvent::AddTag(expected_tag_name.clone()).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let add_tag_command = LidarrCommand::Add(LidarrAddCommand::Tag {
        name: expected_tag_name,
      });

      let result = LidarrCliHandler::with(&app_arc, add_tag_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_lidarr_cli_handler_delegates_get_commands_to_the_get_command_handler() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(LidarrEvent::GetStatus.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let get_system_status_command = LidarrCommand::Get(LidarrGetCommand::SystemStatus);

      let result = LidarrCliHandler::with(&app_arc, get_system_status_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_lidarr_cli_handler_delegates_delete_commands_to_the_delete_command_handler() {
      let expected_delete_artist_params = DeleteParams {
        id: 1,
        delete_files: true,
        add_import_list_exclusion: true,
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          LidarrEvent::DeleteArtist(expected_delete_artist_params).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let delete_artist_command = LidarrCommand::Delete(LidarrDeleteCommand::Artist {
        artist_id: 1,
        delete_files_from_disk: true,
        add_list_exclusion: true,
      });

      let result = LidarrCliHandler::with(&app_arc, delete_artist_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_lidarr_cli_handler_delegates_edit_commands_to_the_edit_command_handler() {
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
          LidarrEvent::GetAllIndexerSettings.into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::IndexerSettings(
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
          LidarrEvent::EditAllIndexerSettings(expected_edit_all_indexer_settings).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let edit_all_indexer_settings_command =
        LidarrCommand::Edit(LidarrEditCommand::AllIndexerSettings {
          maximum_size: Some(1),
          minimum_age: Some(1),
          retention: Some(1),
          rss_sync_interval: Some(1),
        });

      let result = LidarrCliHandler::with(
        &app_arc,
        edit_all_indexer_settings_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_lidarr_cli_handler_delegates_list_commands_to_the_list_command_handler() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(LidarrEvent::ListArtists.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Artists(vec![
            Artist::default(),
          ])))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let list_artists_command = LidarrCommand::List(LidarrListCommand::Artists);

      let result = LidarrCliHandler::with(&app_arc, list_artists_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_lidarr_cli_handler_delegates_refresh_commands_to_the_refresh_command_handler() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(LidarrEvent::UpdateAllArtists.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let refresh_series_command = LidarrCommand::Refresh(LidarrRefreshCommand::AllArtists);

      let result = LidarrCliHandler::with(&app_arc, refresh_series_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_lidarr_cli_handler_delegates_manual_search_commands_to_the_manual_search_command_handler()
     {
      let expected_artist_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          LidarrEvent::GetDiscographyReleases(expected_artist_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let manual_episode_search_command =
        LidarrCommand::ManualSearch(LidarrManualSearchCommand::Discography { artist_id: 1 });

      let result =
        LidarrCliHandler::with(&app_arc, manual_episode_search_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_lidarr_cli_handler_delegates_trigger_automatic_search_commands_to_the_trigger_automatic_search_command_handler()
     {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          LidarrEvent::TriggerAutomaticArtistSearch(1).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let trigger_automatic_search_command =
        LidarrCommand::TriggerAutomaticSearch(LidarrTriggerAutomaticSearchCommand::Artist {
          artist_id: 1,
        });

      let result = LidarrCliHandler::with(
        &app_arc,
        trigger_automatic_search_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_toggle_artist_monitoring_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          LidarrEvent::ToggleArtistMonitoring(1).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let toggle_artist_monitoring_command = LidarrCommand::ToggleArtistMonitoring { artist_id: 1 };

      let result = LidarrCliHandler::with(
        &app_arc,
        toggle_artist_monitoring_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_search_new_artist_command() {
      let expected_query = "test artist".to_owned();
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          LidarrEvent::SearchNewArtist(expected_query.clone()).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let search_new_artist_command = LidarrCommand::SearchNewArtist {
        query: expected_query,
      };

      let result = LidarrCliHandler::with(&app_arc, search_new_artist_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_start_task_command() {
      let expected_task_name = LidarrTaskName::ApplicationUpdateCheck;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          LidarrEvent::StartTask(expected_task_name).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let start_task_command = LidarrCommand::StartTask {
        task_name: LidarrTaskName::ApplicationUpdateCheck,
      };

      let result = LidarrCliHandler::with(&app_arc, start_task_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_test_indexer_command() {
      let expected_indexer_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          LidarrEvent::TestIndexer(expected_indexer_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let test_indexer_command = LidarrCommand::TestIndexer { indexer_id: 1 };

      let result = LidarrCliHandler::with(&app_arc, test_indexer_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_test_all_indexers_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(LidarrEvent::TestAllIndexers.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let test_all_indexers_command = LidarrCommand::TestAllIndexers;

      let result = LidarrCliHandler::with(&app_arc, test_all_indexers_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_mark_history_item_as_failed_command() {
      let expected_history_item_id = 1i64;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          LidarrEvent::MarkHistoryItemAsFailed(expected_history_item_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let mark_history_item_as_failed_command = LidarrCommand::MarkHistoryItemAsFailed {
        history_item_id: expected_history_item_id,
      };

      let result = LidarrCliHandler::with(
        &app_arc,
        mark_history_item_as_failed_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert_ok!(&result);
    }
  }
}
