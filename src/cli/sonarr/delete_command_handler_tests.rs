#[cfg(test)]
mod tests {
  use crate::{
    cli::{
      sonarr::{delete_command_handler::SonarrDeleteCommand, SonarrCommand},
      Command,
    },
    Cli,
  };
  use clap::{error::ErrorKind, CommandFactory, Parser};

  #[test]
  fn test_sonarr_delete_command_from() {
    let command = SonarrDeleteCommand::BlocklistItem {
      blocklist_item_id: 1,
    };

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Sonarr(SonarrCommand::Delete(command)));
  }

  mod cli {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_delete_blocklist_item_requires_arguments() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "sonarr", "delete", "blocklist-item"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_delete_blocklist_item_success() {
      let expected_args = SonarrDeleteCommand::BlocklistItem {
        blocklist_item_id: 1,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "sonarr",
        "delete",
        "blocklist-item",
        "--blocklist-item-id",
        "1",
      ]);

      assert!(result.is_ok());

      if let Some(Command::Sonarr(SonarrCommand::Delete(delete_command))) = result.unwrap().command
      {
        assert_eq!(delete_command, expected_args);
      }
    }

    #[test]
    fn test_delete_download_requires_arguments() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "sonarr", "delete", "download"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_delete_download_success() {
      let expected_args = SonarrDeleteCommand::Download { download_id: 1 };

      let result = Cli::try_parse_from([
        "managarr",
        "sonarr",
        "delete",
        "download",
        "--download-id",
        "1",
      ]);

      assert!(result.is_ok());

      if let Some(Command::Sonarr(SonarrCommand::Delete(delete_command))) = result.unwrap().command
      {
        assert_eq!(delete_command, expected_args);
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
        sonarr::delete_command_handler::{SonarrDeleteCommand, SonarrDeleteCommandHandler},
        CliCommandHandler,
      },
      models::{sonarr_models::SonarrSerdeable, Serdeable},
      network::{sonarr_network::SonarrEvent, MockNetworkTrait, NetworkEvent},
    };

    #[tokio::test]
    async fn test_handle_delete_blocklist_item_command() {
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
      let delete_blocklist_item_command = SonarrDeleteCommand::BlocklistItem {
        blocklist_item_id: 1,
      };

      let result = SonarrDeleteCommandHandler::with(
        &app_arc,
        delete_blocklist_item_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_delete_download_command() {
      let expected_download_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::DeleteDownload(Some(expected_download_id)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let delete_download_command = SonarrDeleteCommand::Download { download_id: 1 };

      let result =
        SonarrDeleteCommandHandler::with(&app_arc, delete_download_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }
  }
}
