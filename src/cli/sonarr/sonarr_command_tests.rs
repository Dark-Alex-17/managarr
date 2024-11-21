#[cfg(test)]
mod tests {
  use crate::cli::{
    sonarr::{list_command_handler::SonarrListCommand, SonarrCommand},
    Command,
  };
  use crate::Cli;
  use clap::CommandFactory;

  #[test]
  fn test_sonarr_command_from() {
    let command = SonarrCommand::List(SonarrListCommand::Series);

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Sonarr(command));
  }

  mod cli {
    use super::*;
    use clap::error::ErrorKind;
    use rstest::rstest;

    #[rstest]
    fn test_commands_that_have_no_arg_requirements(#[values("clear-blocklist")] subcommand: &str) {
      let result = Cli::command().try_get_matches_from(["managarr", "sonarr", subcommand]);

      assert!(result.is_ok());
    }

    #[rstest]
    fn test_manual_season_search_requires_series_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "manual-season-search",
        "--season-number",
        "1",
      ]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[rstest]
    fn test_manual_season_search_requires_season_number() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "manual-season-search",
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
        "manual-season-search",
        "--series-id",
        "1",
        "--season-number",
        "1",
      ]);

      assert!(result.is_ok());
    }

    #[rstest]
    fn test_manual_episode_search_requires_episode_id() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "sonarr", "manual-episode-search"]);

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
        "manual-episode-search",
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
        sonarr::{
          delete_command_handler::SonarrDeleteCommand, get_command_handler::SonarrGetCommand,
          list_command_handler::SonarrListCommand, SonarrCliHandler, SonarrCommand,
        },
        CliCommandHandler,
      },
      models::{
        sonarr_models::{BlocklistItem, BlocklistResponse, Series, SonarrSerdeable},
        Serdeable,
      },
      network::{sonarr_network::SonarrEvent, MockNetworkTrait, NetworkEvent},
    };

    #[tokio::test]
    async fn test_handle_clear_blocklist_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(SonarrEvent::GetBlocklist.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::BlocklistResponse(
            BlocklistResponse {
              records: vec![BlocklistItem::default()],
            },
          )))
        });
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(SonarrEvent::ClearBlocklist.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let claer_blocklist_command = SonarrCommand::ClearBlocklist;

      let result = SonarrCliHandler::with(&app_arc, claer_blocklist_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_manual_episode_search_command() {
      let expected_episode_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::GetEpisodeReleases(Some(expected_episode_id)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let manual_episode_search_command = SonarrCommand::ManualEpisodeSearch { episode_id: 1 };

      let result =
        SonarrCliHandler::with(&app_arc, manual_episode_search_command, &mut mock_network)
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
          SonarrEvent::GetSeasonReleases(Some((expected_series_id, expected_season_number))).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let manual_season_search_command = SonarrCommand::ManualSeasonSearch {
        series_id: 1,
        season_number: 1,
      };

      let result =
        SonarrCliHandler::with(&app_arc, manual_season_search_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_sonarr_cli_handler_delegates_delete_commands_to_the_delete_command_handler() {
      let expected_blocklist_item_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::DeleteBlocklistItem(Some(expected_blocklist_item_id)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let delete_blocklist_item_command =
        SonarrCommand::Delete(SonarrDeleteCommand::BlocklistItem {
          blocklist_item_id: 1,
        });

      let result =
        SonarrCliHandler::with(&app_arc, delete_blocklist_item_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_sonarr_cli_handler_delegates_get_commands_to_the_get_command_handler() {
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
      let get_system_status_command = SonarrCommand::Get(SonarrGetCommand::SystemStatus);

      let result = SonarrCliHandler::with(&app_arc, get_system_status_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_sonarr_cli_handler_delegates_list_commands_to_the_list_command_handler() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(SonarrEvent::ListSeries.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::SeriesVec(vec![
            Series::default(),
          ])))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let list_series_command = SonarrCommand::List(SonarrListCommand::Series);

      let result = SonarrCliHandler::with(&app_arc, list_series_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }
  }
}