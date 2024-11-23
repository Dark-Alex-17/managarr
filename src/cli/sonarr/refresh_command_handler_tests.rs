#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;

  use crate::cli::{
    sonarr::{refresh_command_handler::SonarrRefreshCommand, SonarrCommand},
    Command,
  };
  use crate::Cli;
  use clap::CommandFactory;

  #[test]
  fn test_sonarr_refresh_command_from() {
    let command = SonarrRefreshCommand::AllSeries;

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Sonarr(SonarrCommand::Refresh(command)));
  }

  mod cli {
    use super::*;
    use clap::Parser;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    fn test_refresh_commands_have_no_arg_requirements(#[values("all-series")] subcommand: &str) {
      let result =
        Cli::command().try_get_matches_from(["managarr", "sonarr", "refresh", subcommand]);

      assert!(result.is_ok());
    }
  }

  mod handler {
    use rstest::rstest;
    use std::sync::Arc;

    use mockall::predicate::eq;
    use serde_json::json;
    use tokio::sync::Mutex;

    use crate::{
      cli::{sonarr::refresh_command_handler::SonarrRefreshCommand, CliCommandHandler},
      network::sonarr_network::SonarrEvent,
    };
    use crate::{
      models::{sonarr_models::SonarrSerdeable, Serdeable},
      network::{MockNetworkTrait, NetworkEvent},
    };

    #[rstest]
    #[case(SonarrRefreshCommand::AllSeries, SonarrEvent::UpdateAllSeries)]
    #[tokio::test]
    async fn test_handle_refresh_command(
      #[case] refresh_command: SonarrRefreshCommand,
      #[case] expected_sonarr_event: SonarrEvent,
    ) {
      use crate::{app::App, cli::sonarr::refresh_command_handler::SonarrRefreshCommandHandler};

      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(expected_sonarr_event.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));

      let result = SonarrRefreshCommandHandler::with(&app_arc, refresh_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }
  }
}
