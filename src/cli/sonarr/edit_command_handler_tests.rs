#[cfg(test)]
mod tests {
  use crate::cli::{
    sonarr::{edit_command_handler::SonarrEditCommand, SonarrCommand},
    Command,
  };

  #[test]
  fn test_sonarr_edit_command_from() {
    let command = SonarrEditCommand::AllIndexerSettings {
      maximum_size: None,
      minimum_age: None,
      retention: None,
      rss_sync_interval: None,
    };

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Sonarr(SonarrCommand::Edit(command)));
  }

  mod cli {
    use crate::Cli;

    use super::*;
    use clap::{error::ErrorKind, CommandFactory, Parser};
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn test_edit_all_indexer_settings_requires_arguments() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "sonarr", "edit", "all-indexer-settings"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[rstest]
    fn test_edit_all_indexer_settings_assert_argument_flags_require_args(
      #[values(
        "--maximum-size",
        "--minimum-age",
        "--retention",
        "--rss-sync-interval"
      )]
      flag: &str,
    ) {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "edit",
        "all-indexer-settings",
        flag,
      ]);

      assert!(result.is_err());
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_edit_all_indexer_settings_only_requires_at_least_one_argument() {
      let expected_args = SonarrEditCommand::AllIndexerSettings {
        maximum_size: Some(1),
        minimum_age: None,
        retention: None,
        rss_sync_interval: None,
      };
      let result = Cli::try_parse_from([
        "managarr",
        "sonarr",
        "edit",
        "all-indexer-settings",
        "--maximum-size",
        "1",
      ]);

      assert!(result.is_ok());

      if let Some(Command::Sonarr(SonarrCommand::Edit(edit_command))) = result.unwrap().command {
        assert_eq!(edit_command, expected_args);
      }
    }

    #[test]
    fn test_edit_all_indexer_settings_all_arguments_defined() {
      let expected_args = SonarrEditCommand::AllIndexerSettings {
        maximum_size: Some(1),
        minimum_age: Some(1),
        retention: Some(1),
        rss_sync_interval: Some(1),
      };
      let result = Cli::try_parse_from([
        "managarr",
        "sonarr",
        "edit",
        "all-indexer-settings",
        "--maximum-size",
        "1",
        "--minimum-age",
        "1",
        "--retention",
        "1",
        "--rss-sync-interval",
        "1",
      ]);

      assert!(result.is_ok());

      if let Some(Command::Sonarr(SonarrCommand::Edit(edit_command))) = result.unwrap().command {
        assert_eq!(edit_command, expected_args);
      }
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
        sonarr::edit_command_handler::{SonarrEditCommand, SonarrEditCommandHandler},
        CliCommandHandler,
      },
      models::{
        sonarr_models::{IndexerSettings, SonarrSerdeable},
        Serdeable,
      },
      network::{sonarr_network::SonarrEvent, MockNetworkTrait, NetworkEvent},
    };

    #[tokio::test]
    async fn test_handle_edit_all_indexer_settings_command() {
      let expected_edit_all_indexer_settings = IndexerSettings {
        id: 1,
        maximum_size: 1,
        minimum_age: 1,
        retention: 1,
        rss_sync_interval: 1,
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::GetAllIndexerSettings.into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::IndexerSettings(
            IndexerSettings {
              id: 1,
              maximum_size: 2,
              minimum_age: 2,
              retention: 2,
              rss_sync_interval: 2,
            },
          )))
        });
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::EditAllIndexerSettings(Some(expected_edit_all_indexer_settings)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let edit_all_indexer_settings_command = SonarrEditCommand::AllIndexerSettings {
        maximum_size: Some(1),
        minimum_age: Some(1),
        retention: Some(1),
        rss_sync_interval: Some(1),
      };

      let result = SonarrEditCommandHandler::with(
        &app_arc,
        edit_all_indexer_settings_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert!(result.is_ok());
    }
  }
}
