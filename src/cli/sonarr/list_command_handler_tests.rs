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
    use rstest::rstest;

    #[rstest]
    fn test_list_commands_have_no_arg_requirements(#[values("series")] subcommand: &str) {
      let result = Cli::command().try_get_matches_from(["managarr", "sonarr", "list", subcommand]);

      assert!(result.is_ok());
    }
  }

  mod handler {

    use std::sync::Arc;

    use mockall::predicate::eq;
    use rstest::rstest;
    use serde_json::json;
    use tokio::sync::Mutex;

    use crate::cli::sonarr::list_command_handler::SonarrListCommand;
    use crate::cli::CliCommandHandler;
    use crate::network::sonarr_network::SonarrEvent;
    use crate::{
      app::App,
      models::{radarr_models::RadarrSerdeable, Serdeable},
      network::{MockNetworkTrait, NetworkEvent},
    };

    #[rstest]
    #[case(SonarrListCommand::Series, SonarrEvent::ListSeries)]
    #[tokio::test]
    async fn test_handle_list_command(
      #[case] list_command: SonarrListCommand,
      #[case] expected_sonarr_event: SonarrEvent,
    ) {
      use crate::cli::sonarr::list_command_handler::SonarrListCommandHandler;

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
  }
}
