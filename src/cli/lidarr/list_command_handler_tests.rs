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
  fn test_lidarr_list_command_from() {
    let command = LidarrListCommand::Artists;

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Lidarr(LidarrCommand::List(command)));
  }

  mod cli {
    use super::*;
    use clap::{Parser, error::ErrorKind};
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    fn test_list_commands_have_no_arg_requirements(
      #[values(
        "artists",
        "indexers",
        "metadata-profiles",
        "quality-profiles",
        "queued-events",
        "tags",
        "tasks",
        "updates",
        "root-folders"
      )]
      subcommand: &str,
    ) {
      let result = Cli::command().try_get_matches_from(["managarr", "lidarr", "list", subcommand]);

      assert_ok!(&result);
    }

    #[test]
    fn test_list_albums_requires_artist_id() {
      let result = Cli::command().try_get_matches_from(["managarr", "lidarr", "list", "albums"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_list_albums_with_artist_id() {
      let expected_args = LidarrListCommand::Albums { artist_id: 1 };
      let result =
        Cli::try_parse_from(["managarr", "lidarr", "list", "albums", "--artist-id", "1"]);

      assert_ok!(&result);

      let Some(Command::Lidarr(LidarrCommand::List(album_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(album_command, expected_args);
    }

    #[test]
    fn test_list_downloads_count_flag_requires_arguments() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "lidarr", "list", "downloads", "--count"]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_list_downloads_default_values() {
      let expected_args = LidarrListCommand::Downloads { count: 500 };
      let result = Cli::try_parse_from(["managarr", "lidarr", "list", "downloads"]);

      assert_ok!(&result);

      let Some(Command::Lidarr(LidarrCommand::List(downloads_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(downloads_command, expected_args);
    }

    #[test]
    fn test_list_history_events_flag_requires_arguments() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "lidarr", "list", "history", "--events"]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_list_history_default_values() {
      let expected_args = LidarrListCommand::History { events: 500 };
      let result = Cli::try_parse_from(["managarr", "lidarr", "list", "history"]);

      assert_ok!(&result);

      let Some(Command::Lidarr(LidarrCommand::List(history_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(history_command, expected_args);
    }

    #[test]
    fn test_list_logs_events_flag_requires_arguments() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "lidarr", "list", "logs", "--events"]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_list_logs_default_values() {
      let expected_args = LidarrListCommand::Logs {
        events: 500,
        output_in_log_format: false,
      };
      let result = Cli::try_parse_from(["managarr", "lidarr", "list", "logs"]);

      assert_ok!(&result);

      let Some(Command::Lidarr(LidarrCommand::List(logs_command))) = result.unwrap().command else {
        panic!("Unexpected command type");
      };
      assert_eq!(logs_command, expected_args);
    }
  }

  mod handler {
    use std::sync::Arc;

    use mockall::predicate::eq;
    use rstest::rstest;
    use serde_json::json;
    use tokio::sync::Mutex;

    use crate::cli::CliCommandHandler;
    use crate::cli::lidarr::list_command_handler::{LidarrListCommand, LidarrListCommandHandler};
    use crate::models::Serdeable;
    use crate::models::lidarr_models::LidarrSerdeable;
    use crate::network::lidarr_network::LidarrEvent;
    use crate::{
      app::App,
      network::{MockNetworkTrait, NetworkEvent},
    };

    #[rstest]
    #[case(LidarrListCommand::Artists, LidarrEvent::ListArtists)]
    #[case(LidarrListCommand::Indexers, LidarrEvent::GetIndexers)]
    #[case(LidarrListCommand::MetadataProfiles, LidarrEvent::GetMetadataProfiles)]
    #[case(LidarrListCommand::QualityProfiles, LidarrEvent::GetQualityProfiles)]
    #[case(LidarrListCommand::QueuedEvents, LidarrEvent::GetQueuedEvents)]
    #[case(LidarrListCommand::RootFolders, LidarrEvent::GetRootFolders)]
    #[case(LidarrListCommand::Tags, LidarrEvent::GetTags)]
    #[case(LidarrListCommand::Tasks, LidarrEvent::GetTasks)]
    #[case(LidarrListCommand::Updates, LidarrEvent::GetUpdates)]
    #[tokio::test]
    async fn test_handle_list_command(
      #[case] list_command: LidarrListCommand,
      #[case] expected_lidarr_event: LidarrEvent,
    ) {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(expected_lidarr_event.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));

      let result = LidarrListCommandHandler::with(&app_arc, list_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_list_albums_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(LidarrEvent::GetAlbums(1).into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let list_command = LidarrListCommand::Albums { artist_id: 1 };

      let result = LidarrListCommandHandler::with(&app_arc, list_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_list_downloads_command() {
      let expected_count = 1000;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          LidarrEvent::GetDownloads(expected_count).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let list_downloads_command = LidarrListCommand::Downloads { count: 1000 };

      let result =
        LidarrListCommandHandler::with(&app_arc, list_downloads_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_list_history_command() {
      let expected_events = 1000;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          LidarrEvent::GetHistory(expected_events).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let list_history_command = LidarrListCommand::History { events: 1000 };

      let result =
        LidarrListCommandHandler::with(&app_arc, list_history_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_list_logs_command() {
      let expected_events = 1000;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          LidarrEvent::GetLogs(expected_events).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let list_logs_command = LidarrListCommand::Logs {
        events: 1000,
        output_in_log_format: false,
      };

      let result = LidarrListCommandHandler::with(&app_arc, list_logs_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }
  }
}
