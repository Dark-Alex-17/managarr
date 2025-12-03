#[cfg(test)]
mod tests {
  use crate::Cli;
  use crate::cli::{
    Command,
    sonarr::{SonarrCommand, manual_search_command_handler::SonarrManualSearchCommand},
  };
  use clap::CommandFactory;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_sonarr_manual_search_command_from() {
    let command = SonarrManualSearchCommand::Episode { episode_id: 1 };

    let result = Command::from(command.clone());

    assert_eq!(
      result,
      Command::Sonarr(SonarrCommand::ManualSearch(command))
    );
  }

  mod cli {
    use super::*;
    use clap::error::ErrorKind;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_manual_season_search_requires_series_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "manual-search",
        "season",
        "--season-number",
        "1",
      ]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_manual_season_search_requires_season_number() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "manual-search",
        "season",
        "--series-id",
        "1",
      ]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_manual_season_search_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "manual-search",
        "season",
        "--series-id",
        "1",
        "--season-number",
        "1",
      ]);

      assert!(result.is_ok());
    }

    #[test]
    fn test_manual_episode_search_requires_episode_id() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "sonarr", "manual-search", "episode"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_manual_episode_search_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "manual-search",
        "episode",
        "--episode-id",
        "1",
      ]);

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
        CliCommandHandler,
        sonarr::manual_search_command_handler::{
          SonarrManualSearchCommand, SonarrManualSearchCommandHandler,
        },
      },
      models::{Serdeable, sonarr_models::SonarrSerdeable},
      network::{MockNetworkTrait, NetworkEvent, sonarr_network::SonarrEvent},
    };

    #[tokio::test]
    async fn test_manual_episode_search_command() {
      let expected_episode_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::GetEpisodeReleases(expected_episode_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let manual_episode_search_command = SonarrManualSearchCommand::Episode { episode_id: 1 };

      let result = SonarrManualSearchCommandHandler::with(
        &app_arc,
        manual_episode_search_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_manual_season_search_command() {
      let expected_series_id = 1;
      let expected_season_number = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::GetSeasonReleases((expected_series_id, expected_season_number)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let manual_season_search_command = SonarrManualSearchCommand::Season {
        series_id: 1,
        season_number: 1,
      };

      let result = SonarrManualSearchCommandHandler::with(
        &app_arc,
        manual_season_search_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert!(result.is_ok());
    }
  }
}
