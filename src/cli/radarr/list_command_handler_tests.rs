#[cfg(test)]
mod tests {
  use clap::error::ErrorKind;
  use clap::CommandFactory;

  use crate::cli::radarr::list_command_handler::RadarrListCommand;
  use crate::cli::radarr::RadarrCommand;
  use crate::cli::Command;
  use crate::Cli;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_radarr_list_command_from() {
    let command = RadarrListCommand::Movies;

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Radarr(RadarrCommand::List(command)));
  }

  mod cli {
    use super::*;
    use clap::Parser;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    fn test_list_commands_have_no_arg_requirements(
      #[values(
        "blocklist",
        "collections",
        "downloads",
        "disk-space",
        "indexers",
        "movies",
        "quality-profiles",
        "queued-events",
        "root-folders",
        "tags",
        "tasks",
        "updates"
      )]
      subcommand: &str,
    ) {
      let result = Cli::command().try_get_matches_from(["managarr", "radarr", "list", subcommand]);

      assert!(result.is_ok());
    }

    #[test]
    fn test_list_movie_credits_requires_movie_id() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "radarr", "list", "movie-credits"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_list_logs_events_flag_requires_arguments() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "radarr", "list", "logs", "--events"]);

      assert!(result.is_err());
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_list_movie_credits_success() {
      let expected_args = RadarrListCommand::MovieCredits { movie_id: 1 };
      let result = Cli::try_parse_from([
        "managarr",
        "radarr",
        "list",
        "movie-credits",
        "--movie-id",
        "1",
      ]);

      assert!(result.is_ok());

      if let Some(Command::Radarr(RadarrCommand::List(credits_command))) = result.unwrap().command {
        assert_eq!(credits_command, expected_args);
      }
    }

    #[test]
    fn test_list_logs_default_values() {
      let expected_args = RadarrListCommand::Logs {
        events: 500,
        output_in_log_format: false,
      };
      let result = Cli::try_parse_from(["managarr", "radarr", "list", "logs"]);

      assert!(result.is_ok());

      if let Some(Command::Radarr(RadarrCommand::List(refresh_command))) = result.unwrap().command {
        assert_eq!(refresh_command, expected_args);
      }
    }
  }

  mod handler {
    use std::sync::Arc;

    use mockall::predicate::eq;
    use rstest::rstest;
    use serde_json::json;
    use tokio::sync::Mutex;

    use crate::cli::CliCommandHandler;
    use crate::{
      app::App,
      cli::radarr::list_command_handler::{RadarrListCommand, RadarrListCommandHandler},
      models::{radarr_models::RadarrSerdeable, Serdeable},
      network::{radarr_network::RadarrEvent, MockNetworkTrait, NetworkEvent},
    };

    #[rstest]
    #[case(RadarrListCommand::Blocklist, RadarrEvent::GetBlocklist)]
    #[case(RadarrListCommand::Collections, RadarrEvent::GetCollections)]
    #[case(RadarrListCommand::Downloads, RadarrEvent::GetDownloads)]
    #[case(RadarrListCommand::DiskSpace, RadarrEvent::GetDiskSpace)]
    #[case(RadarrListCommand::Indexers, RadarrEvent::GetIndexers)]
    #[case(RadarrListCommand::Movies, RadarrEvent::GetMovies)]
    #[case(RadarrListCommand::QualityProfiles, RadarrEvent::GetQualityProfiles)]
    #[case(RadarrListCommand::QueuedEvents, RadarrEvent::GetQueuedEvents)]
    #[case(RadarrListCommand::RootFolders, RadarrEvent::GetRootFolders)]
    #[case(RadarrListCommand::Tags, RadarrEvent::GetTags)]
    #[case(RadarrListCommand::Tasks, RadarrEvent::GetTasks)]
    #[case(RadarrListCommand::Updates, RadarrEvent::GetUpdates)]
    #[tokio::test]
    async fn test_handle_list_command(
      #[case] list_command: RadarrListCommand,
      #[case] expected_radarr_event: RadarrEvent,
    ) {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(expected_radarr_event.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));

      let result = RadarrListCommandHandler::with(&app_arc, list_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_list_movie_credits_command() {
      let expected_movie_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::GetMovieCredits(Some(expected_movie_id)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let list_movie_credits_command = RadarrListCommand::MovieCredits { movie_id: 1 };

      let result =
        RadarrListCommandHandler::with(&app_arc, list_movie_credits_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_list_logs_command() {
      let expected_events = 1000;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::GetLogs(expected_events).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let list_logs_command = RadarrListCommand::Logs {
        events: 1000,
        output_in_log_format: false,
      };

      let result = RadarrListCommandHandler::with(&app_arc, list_logs_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }
  }
}
