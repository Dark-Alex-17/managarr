#[cfg(test)]
mod tests {
  use crate::cli::{
    sonarr::{list_command_handler::SonarrListCommand, SonarrCommand},
    Command,
  };
  use crate::Cli;
  use clap::CommandFactory;

  #[test]
  fn test_sonarr_list_command_from() {
    let command = SonarrListCommand::Series;

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Sonarr(SonarrCommand::List(command)));
  }

  mod cli {
    use super::*;
    use clap::{error::ErrorKind, Parser};
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    fn test_list_commands_have_no_arg_requirements(
      #[values("blocklist", "series")] subcommand: &str,
    ) {
      let result = Cli::command().try_get_matches_from(["managarr", "sonarr", "list", subcommand]);

      assert!(result.is_ok());
    }

    #[test]
    fn test_list_logs_events_flag_requires_arguments() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "sonarr", "list", "logs", "--events"]);

      assert!(result.is_err());
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_list_logs_default_values() {
      let expected_args = SonarrListCommand::Logs {
        events: 500,
        output_in_log_format: false,
      };
      let result = Cli::try_parse_from(["managarr", "sonarr", "list", "logs"]);

      assert!(result.is_ok());

      if let Some(Command::Sonarr(SonarrCommand::List(refresh_command))) = result.unwrap().command {
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

    use crate::cli::sonarr::list_command_handler::{SonarrListCommand, SonarrListCommandHandler};
    use crate::cli::CliCommandHandler;
    use crate::models::sonarr_models::SonarrSerdeable;
    use crate::network::sonarr_network::SonarrEvent;
    use crate::{
      app::App,
      models::{radarr_models::RadarrSerdeable, Serdeable},
      network::{MockNetworkTrait, NetworkEvent},
    };

    #[rstest]
    #[case(SonarrListCommand::Blocklist, SonarrEvent::GetBlocklist)]
    #[case(SonarrListCommand::Series, SonarrEvent::ListSeries)]
    #[tokio::test]
    async fn test_handle_list_command(
      #[case] list_command: SonarrListCommand,
      #[case] expected_sonarr_event: SonarrEvent,
    ) {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(expected_sonarr_event.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));

      let result = SonarrListCommandHandler::with(&app_arc, list_command, &mut mock_network)
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
          SonarrEvent::GetLogs(Some(expected_events)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let list_logs_command = SonarrListCommand::Logs {
        events: 1000,
        output_in_log_format: false,
      };

      let result = SonarrListCommandHandler::with(&app_arc, list_logs_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }
  }
}