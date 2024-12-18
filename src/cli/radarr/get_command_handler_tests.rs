#[cfg(test)]
mod tests {
  use clap::error::ErrorKind;
  use clap::CommandFactory;

  use crate::cli::radarr::get_command_handler::RadarrGetCommand;
  use crate::cli::radarr::RadarrCommand;
  use crate::cli::Command;
  use crate::Cli;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_radarr_get_command_from() {
    let command = RadarrGetCommand::AllIndexerSettings;

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Radarr(RadarrCommand::Get(command)));
  }

  mod cli {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_all_indexer_settings_has_no_arg_requirements() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "radarr", "get", "all-indexer-settings"]);

      assert!(result.is_ok());
    }

    #[test]
    fn test_get_host_config_has_no_arg_requirements() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "radarr", "get", "host-config"]);

      assert!(result.is_ok());
    }

    #[test]
    fn test_movie_details_requires_movie_id() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "radarr", "get", "movie-details"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_movie_details_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "radarr",
        "get",
        "movie-details",
        "--movie-id",
        "1",
      ]);

      assert!(result.is_ok());
    }

    #[test]
    fn test_movie_history_requires_movie_id() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "radarr", "get", "movie-history"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_movie_history_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "radarr",
        "get",
        "movie-history",
        "--movie-id",
        "1",
      ]);

      assert!(result.is_ok());
    }

    #[test]
    fn test_get_security_config_has_no_arg_requirements() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "radarr", "get", "security-config"]);

      assert!(result.is_ok());
    }

    #[test]
    fn test_system_status_has_no_arg_requirements() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "radarr", "get", "system-status"]);

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
        radarr::get_command_handler::{RadarrGetCommand, RadarrGetCommandHandler},
        CliCommandHandler,
      },
      models::{radarr_models::RadarrSerdeable, Serdeable},
      network::{radarr_network::RadarrEvent, MockNetworkTrait, NetworkEvent},
    };

    #[tokio::test]
    async fn test_handle_get_all_indexer_settings_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::GetAllIndexerSettings.into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let get_all_indexer_settings_command = RadarrGetCommand::AllIndexerSettings;

      let result = RadarrGetCommandHandler::with(
        &app_arc,
        get_all_indexer_settings_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_get_host_config_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(RadarrEvent::GetHostConfig.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let get_host_config_command = RadarrGetCommand::HostConfig;

      let result =
        RadarrGetCommandHandler::with(&app_arc, get_host_config_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_get_movie_details_command() {
      let expected_movie_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::GetMovieDetails(expected_movie_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let get_movie_details_command = RadarrGetCommand::MovieDetails { movie_id: 1 };

      let result =
        RadarrGetCommandHandler::with(&app_arc, get_movie_details_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_get_movie_history_command() {
      let expected_movie_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::GetMovieHistory(Some(expected_movie_id)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let get_movie_history_command = RadarrGetCommand::MovieHistory { movie_id: 1 };

      let result =
        RadarrGetCommandHandler::with(&app_arc, get_movie_history_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_get_security_config_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(RadarrEvent::GetSecurityConfig.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let get_security_config_command = RadarrGetCommand::SecurityConfig;

      let result =
        RadarrGetCommandHandler::with(&app_arc, get_security_config_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_get_system_status_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(RadarrEvent::GetStatus.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let get_system_status_command = RadarrGetCommand::SystemStatus;

      let result =
        RadarrGetCommandHandler::with(&app_arc, get_system_status_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }
  }
}
