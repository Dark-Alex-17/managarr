#[cfg(test)]
mod tests {
  use crate::cli::{
    sonarr::{
      trigger_automatic_search_command_handler::SonarrTriggerAutomaticSearchCommand, SonarrCommand,
    },
    Command,
  };
  use crate::Cli;
  use clap::CommandFactory;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_sonarr_trigger_automatic_search_command_from() {
    let command = SonarrTriggerAutomaticSearchCommand::Episode { episode_id: 1 };

    let result = Command::from(command.clone());

    assert_eq!(
      result,
      Command::Sonarr(SonarrCommand::TriggerAutomaticSearch(command))
    );
  }

  mod cli {
    use super::*;
    use clap::error::ErrorKind;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_trigger_automatic_series_search_requires_series_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "trigger-automatic-search",
        "series",
      ]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_trigger_automatic_series_search_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "trigger-automatic-search",
        "series",
        "--series-id",
        "1",
      ]);

      assert!(result.is_ok());
    }

    #[test]
    fn test_trigger_automatic_season_search_requires_series_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "trigger-automatic-search",
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
    fn test_trigger_automatic_season_search_requires_season_number() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "trigger-automatic-search",
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
    fn test_trigger_automatic_season_search_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "trigger-automatic-search",
        "season",
        "--series-id",
        "1",
        "--season-number",
        "1",
      ]);

      assert!(result.is_ok());
    }

    #[test]
    fn test_trigger_automatic_episode_search_requires_episode_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "trigger-automatic-search",
        "episode",
      ]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_trigger_automatic_episode_search_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "trigger-automatic-search",
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
        sonarr::trigger_automatic_search_command_handler::{
          SonarrTriggerAutomaticSearchCommand, SonarrTriggerAutomaticSearchCommandHandler,
        },
        CliCommandHandler,
      },
      models::{sonarr_models::SonarrSerdeable, Serdeable},
      network::{sonarr_network::SonarrEvent, MockNetworkTrait, NetworkEvent},
    };

    #[tokio::test]
    async fn test_trigger_automatic_series_search_command() {
      let expected_series_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::TriggerAutomaticSeriesSearch(Some(expected_series_id)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let trigger_automatic_series_search_command =
        SonarrTriggerAutomaticSearchCommand::Series { series_id: 1 };

      let result = SonarrTriggerAutomaticSearchCommandHandler::with(
        &app_arc,
        trigger_automatic_series_search_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_trigger_automatic_season_search_command() {
      let expected_series_id = 1;
      let expected_season_number = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::TriggerAutomaticSeasonSearch(Some((
            expected_series_id,
            expected_season_number,
          )))
          .into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let trigger_automatic_season_search_command = SonarrTriggerAutomaticSearchCommand::Season {
        series_id: 1,
        season_number: 1,
      };

      let result = SonarrTriggerAutomaticSearchCommandHandler::with(
        &app_arc,
        trigger_automatic_season_search_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_trigger_automatic_episode_search_command() {
      let expected_episode_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::TriggerAutomaticEpisodeSearch(expected_episode_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let trigger_automatic_episode_search_command =
        SonarrTriggerAutomaticSearchCommand::Episode { episode_id: 1 };

      let result = SonarrTriggerAutomaticSearchCommandHandler::with(
        &app_arc,
        trigger_automatic_episode_search_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert!(result.is_ok());
    }
  }
}
