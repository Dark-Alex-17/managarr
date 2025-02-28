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
    use clap::{error::ErrorKind, Parser};
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    fn test_refresh_commands_have_no_arg_requirements(
      #[values("all-series", "downloads")] subcommand: &str,
    ) {
      let result =
        Cli::command().try_get_matches_from(["managarr", "sonarr", "refresh", subcommand]);

      assert!(result.is_ok());
    }

    #[test]
    fn test_refresh_series_requires_series_id() {
      let result = Cli::command().try_get_matches_from(["managarr", "sonarr", "refresh", "series"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_refresh_series_success() {
      let expected_args = SonarrRefreshCommand::Series { series_id: 1 };
      let result = Cli::try_parse_from([
        "managarr",
        "sonarr",
        "refresh",
        "series",
        "--series-id",
        "1",
      ]);

      assert!(result.is_ok());

      if let Some(Command::Sonarr(SonarrCommand::Refresh(refresh_command))) =
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

    use crate::{app::App, cli::sonarr::refresh_command_handler::SonarrRefreshCommandHandler};
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
    #[case(SonarrRefreshCommand::Downloads, SonarrEvent::UpdateDownloads)]
    #[tokio::test]
    async fn test_handle_refresh_command(
      #[case] refresh_command: SonarrRefreshCommand,
      #[case] expected_sonarr_event: SonarrEvent,
    ) {
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
      let app_arc = Arc::new(Mutex::new(App::test_default()));

      let result = SonarrRefreshCommandHandler::with(&app_arc, refresh_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_refresh_series_command() {
      let expected_series_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::UpdateAndScanSeries(expected_series_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let refresh_series_command = SonarrRefreshCommand::Series { series_id: 1 };

      let result =
        SonarrRefreshCommandHandler::with(&app_arc, refresh_series_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }
  }
}
