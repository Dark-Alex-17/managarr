#[cfg(test)]
mod tests {
  use crate::cli::{
    sonarr::{get_command_handler::SonarrGetCommand, SonarrCommand},
    Command,
  };
  use crate::Cli;
  use clap::CommandFactory;

  #[test]
  fn test_sonarr_get_command_from() {
    let command = SonarrGetCommand::SystemStatus;

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Sonarr(SonarrCommand::Get(command)));
  }

  mod cli {
    use clap::error::ErrorKind;

    use super::*;

    #[test]
    fn test_system_status_has_no_arg_requirements() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "sonarr", "get", "system-status"]);

      assert!(result.is_ok());
    }

    #[test]
    fn test_episode_details_requires_episode_id() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "sonarr", "get", "episode-details"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_episode_details_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "get",
        "episode-details",
        "--episode-id",
        "1",
      ]);

      assert!(result.is_ok());
    }

    #[test]
    fn test_get_host_config_has_no_arg_requirements() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "sonarr", "get", "host-config"]);

      assert!(result.is_ok());
    }

    #[test]
    fn test_get_security_config_has_no_arg_requirements() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "sonarr", "get", "security-config"]);

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
        sonarr::get_command_handler::{SonarrGetCommand, SonarrGetCommandHandler},
        CliCommandHandler,
      },
      models::{sonarr_models::SonarrSerdeable, Serdeable},
      network::{sonarr_network::SonarrEvent, MockNetworkTrait, NetworkEvent},
    };

    #[tokio::test]
    async fn test_handle_get_episode_details_command() {
      let expected_episode_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::GetEpisodeDetails(Some(expected_episode_id)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let get_episode_details_command = SonarrGetCommand::EpisodeDetails { episode_id: 1 };

      let result =
        SonarrGetCommandHandler::with(&app_arc, get_episode_details_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_get_host_config_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(SonarrEvent::GetHostConfig.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let get_host_config_command = SonarrGetCommand::HostConfig;

      let result =
        SonarrGetCommandHandler::with(&app_arc, get_host_config_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_get_security_config_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(SonarrEvent::GetSecurityConfig.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let get_security_config_command = SonarrGetCommand::SecurityConfig;

      let result =
        SonarrGetCommandHandler::with(&app_arc, get_security_config_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_get_system_status_command() {
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
      let get_system_status_command = SonarrGetCommand::SystemStatus;

      let result =
        SonarrGetCommandHandler::with(&app_arc, get_system_status_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }
  }
}
