#[cfg(test)]
mod tests {
  use crate::Cli;
  use crate::cli::{
    Command,
    readarr::{ReadarrCommand, get_command_handler::ReadarrGetCommand},
  };
  use clap::CommandFactory;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_readarr_get_command_from() {
    let command = ReadarrGetCommand::SystemStatus;

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Readarr(ReadarrCommand::Get(command)));
  }

  mod cli {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_get_commands_have_no_arg_requirements(
      #[values("host-config", "security-config", "system-status")] subcommand: &str,
    ) {
      let result = Cli::command().try_get_matches_from(["managarr", "readarr", "get", subcommand]);

      assert_ok!(&result);
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
        CliCommandHandler,
        readarr::get_command_handler::{ReadarrGetCommand, ReadarrGetCommandHandler},
      },
      models::{Serdeable, readarr_models::ReadarrSerdeable},
      network::{MockNetworkTrait, NetworkEvent, readarr_network::ReadarrEvent},
    };

    #[tokio::test]
    async fn test_handle_get_host_config_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(ReadarrEvent::GetHostConfig.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let get_host_config_command = ReadarrGetCommand::HostConfig;

      let result =
        ReadarrGetCommandHandler::with(&app_arc, get_host_config_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_get_security_config_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(ReadarrEvent::GetSecurityConfig.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let get_security_config_command = ReadarrGetCommand::SecurityConfig;

      let result =
        ReadarrGetCommandHandler::with(&app_arc, get_security_config_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_get_system_status_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(ReadarrEvent::GetStatus.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let get_system_status_command = ReadarrGetCommand::SystemStatus;

      let result =
        ReadarrGetCommandHandler::with(&app_arc, get_system_status_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }
  }
}
