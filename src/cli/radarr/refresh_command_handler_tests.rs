#[cfg(test)]
mod tests {
  use clap::error::ErrorKind;
  use clap::CommandFactory;

  use crate::cli::radarr::refresh_command_handler::RadarrRefreshCommand;
  use crate::cli::radarr::RadarrCommand;
  use crate::cli::Command;
  use crate::Cli;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_radarr_refresh_command_from() {
    let command = RadarrRefreshCommand::AllMovies;

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Radarr(RadarrCommand::Refresh(command)));
  }

  mod cli {
    use super::*;
    use clap::Parser;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    fn test_refresh_commands_have_no_arg_requirements(
      #[values("all-movies", "collections", "downloads")] subcommand: &str,
    ) {
      let result =
        Cli::command().try_get_matches_from(["managarr", "radarr", "refresh", subcommand]);

      assert!(result.is_ok());
    }

    #[test]
    fn test_refresh_movie_requires_movie_id() {
      let result = Cli::command().try_get_matches_from(["managarr", "radarr", "refresh", "movie"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_refresh_movie_success() {
      let expected_args = RadarrRefreshCommand::Movie { movie_id: 1 };
      let result =
        Cli::try_parse_from(["managarr", "radarr", "refresh", "movie", "--movie-id", "1"]);

      assert!(result.is_ok());

      if let Some(Command::Radarr(RadarrCommand::Refresh(refresh_command))) =
        result.unwrap().command
      {
        assert_eq!(refresh_command, expected_args);
      }
    }
  }

  mod handler {
    use rstest::rstest;
    use std::sync::Arc;

    use mockall::predicate::eq;
    use serde_json::json;
    use tokio::sync::Mutex;

    use crate::cli::CliCommandHandler;
    use crate::{
      app::App,
      cli::radarr::refresh_command_handler::{RadarrRefreshCommand, RadarrRefreshCommandHandler},
      models::{radarr_models::RadarrSerdeable, Serdeable},
      network::{radarr_network::RadarrEvent, MockNetworkTrait, NetworkEvent},
    };

    #[rstest]
    #[case(RadarrRefreshCommand::AllMovies, RadarrEvent::UpdateAllMovies)]
    #[case(RadarrRefreshCommand::Collections, RadarrEvent::UpdateCollections)]
    #[case(RadarrRefreshCommand::Downloads, RadarrEvent::UpdateDownloads)]
    #[tokio::test]
    async fn test_handle_refresh_command(
      #[case] refresh_command: RadarrRefreshCommand,
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

      let result = RadarrRefreshCommandHandler::with(&app_arc, refresh_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_refresh_movie_command() {
      let expected_movie_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::UpdateAndScan(expected_movie_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let refresh_movie_command = RadarrRefreshCommand::Movie { movie_id: 1 };

      let result =
        RadarrRefreshCommandHandler::with(&app_arc, refresh_movie_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }
  }
}
