#[cfg(test)]
mod tests {
  use crate::cli::{
    sonarr::{list_command_handler::SonarrListCommand, SonarrCommand},
    Command,
  };
  use crate::Cli;
  use clap::CommandFactory;
  use pretty_assertions::assert_eq;

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
      #[values(
        "blocklist",
        "series",
        "downloads",
        "disk-space",
        "quality-profiles",
        "indexers",
        "queued-events",
        "root-folders",
        "tags",
        "tasks",
        "updates",
        "language-profiles"
      )]
      subcommand: &str,
    ) {
      let result = Cli::command().try_get_matches_from(["managarr", "sonarr", "list", subcommand]);

      assert!(result.is_ok());
    }

    #[test]
    fn test_list_episodes_requires_series_id() {
      let result = Cli::command().try_get_matches_from(["managarr", "sonarr", "list", "episodes"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_list_episode_history_requires_series_id() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "sonarr", "list", "episode-history"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_list_episode_history_success() {
      let expected_args = SonarrListCommand::EpisodeHistory { episode_id: 1 };
      let result = Cli::try_parse_from([
        "managarr",
        "sonarr",
        "list",
        "episode-history",
        "--episode-id",
        "1",
      ]);

      assert!(result.is_ok());

      if let Some(Command::Sonarr(SonarrCommand::List(episode_history_command))) =
        result.unwrap().command
      {
        assert_eq!(episode_history_command, expected_args);
      }
    }

    #[test]
    fn test_list_history_events_flag_requires_arguments() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "sonarr", "list", "history", "--events"]);

      assert!(result.is_err());
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_list_history_default_values() {
      let expected_args = SonarrListCommand::History { events: 500 };
      let result = Cli::try_parse_from(["managarr", "sonarr", "list", "history"]);

      assert!(result.is_ok());

      if let Some(Command::Sonarr(SonarrCommand::List(history_command))) = result.unwrap().command {
        assert_eq!(history_command, expected_args);
      }
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

      if let Some(Command::Sonarr(SonarrCommand::List(logs_command))) = result.unwrap().command {
        assert_eq!(logs_command, expected_args);
      }
    }

    #[test]
    fn test_list_episodes_success() {
      let expected_args = SonarrListCommand::Episodes { series_id: 1 };
      let result =
        Cli::try_parse_from(["managarr", "sonarr", "list", "episodes", "--series-id", "1"]);

      assert!(result.is_ok());

      if let Some(Command::Sonarr(SonarrCommand::List(episodes_command))) = result.unwrap().command
      {
        assert_eq!(episodes_command, expected_args);
      }
    }

    #[test]
    fn test_list_series_history_requires_series_id() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "sonarr", "list", "series-history"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_list_series_history_success() {
      let expected_args = SonarrListCommand::SeriesHistory { series_id: 1 };
      let result = Cli::try_parse_from([
        "managarr",
        "sonarr",
        "list",
        "series-history",
        "--series-id",
        "1",
      ]);

      assert!(result.is_ok());

      if let Some(Command::Sonarr(SonarrCommand::List(series_command))) = result.unwrap().command {
        assert_eq!(series_command, expected_args);
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
    #[case(SonarrListCommand::Downloads, SonarrEvent::GetDownloads)]
    #[case(SonarrListCommand::DiskSpace, SonarrEvent::GetDiskSpace)]
    #[case(SonarrListCommand::Indexers, SonarrEvent::GetIndexers)]
    #[case(SonarrListCommand::QualityProfiles, SonarrEvent::GetQualityProfiles)]
    #[case(SonarrListCommand::QueuedEvents, SonarrEvent::GetQueuedEvents)]
    #[case(SonarrListCommand::RootFolders, SonarrEvent::GetRootFolders)]
    #[case(SonarrListCommand::Series, SonarrEvent::ListSeries)]
    #[case(SonarrListCommand::Tags, SonarrEvent::GetTags)]
    #[case(SonarrListCommand::Tasks, SonarrEvent::GetTasks)]
    #[case(SonarrListCommand::Updates, SonarrEvent::GetUpdates)]
    #[case(SonarrListCommand::LanguageProfiles, SonarrEvent::GetLanguageProfiles)]
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
    async fn test_handle_list_episodes_command() {
      let expected_series_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::GetEpisodes(Some(expected_series_id)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let list_episodes_command = SonarrListCommand::Episodes { series_id: 1 };

      let result =
        SonarrListCommandHandler::with(&app_arc, list_episodes_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_list_history_command() {
      let expected_events = 1000;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::GetHistory(Some(expected_events)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let list_history_command = SonarrListCommand::History { events: 1000 };

      let result =
        SonarrListCommandHandler::with(&app_arc, list_history_command, &mut mock_network)
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

    #[tokio::test]
    async fn test_handle_list_series_history_command() {
      let expected_series_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::GetSeriesHistory(Some(expected_series_id)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let list_series_history_command = SonarrListCommand::SeriesHistory { series_id: 1 };

      let result =
        SonarrListCommandHandler::with(&app_arc, list_series_history_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_list_episode_history_command() {
      let expected_episode_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::GetEpisodeHistory(Some(expected_episode_id)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let list_episode_history_command = SonarrListCommand::EpisodeHistory { episode_id: 1 };

      let result =
        SonarrListCommandHandler::with(&app_arc, list_episode_history_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }
  }
}
