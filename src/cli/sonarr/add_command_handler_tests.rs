#[cfg(test)]
mod tests {
  use clap::{error::ErrorKind, CommandFactory, Parser};
  use pretty_assertions::assert_eq;

  use crate::{
    cli::{
      sonarr::{add_command_handler::SonarrAddCommand, SonarrCommand},
      Command,
    },
    Cli,
  };

  #[test]
  fn test_sonarr_add_command_from() {
    let command = SonarrAddCommand::Tag {
      name: String::new(),
    };

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Sonarr(SonarrCommand::Add(command)));
  }

  mod cli {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_add_root_folder_requires_arguments() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "sonarr", "add", "root-folder"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_add_root_folder_success() {
      let expected_args = SonarrAddCommand::RootFolder {
        root_folder_path: "/nfs/test".to_owned(),
      };

      let result = Cli::try_parse_from([
        "managarr",
        "sonarr",
        "add",
        "root-folder",
        "--root-folder-path",
        "/nfs/test",
      ]);

      assert!(result.is_ok());

      if let Some(Command::Sonarr(SonarrCommand::Add(add_command))) = result.unwrap().command {
        assert_eq!(add_command, expected_args);
      }
    }

    #[test]
    fn test_add_tag_requires_arguments() {
      let result = Cli::command().try_get_matches_from(["managarr", "sonarr", "add", "tag"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_add_tag_success() {
      let expected_args = SonarrAddCommand::Tag {
        name: "test".to_owned(),
      };

      let result = Cli::try_parse_from(["managarr", "sonarr", "add", "tag", "--name", "test"]);

      assert!(result.is_ok());

      if let Some(Command::Sonarr(SonarrCommand::Add(add_command))) = result.unwrap().command {
        assert_eq!(add_command, expected_args);
      }
    }
  }

  mod handler {
    use std::sync::Arc;

    use crate::{
      app::App,
      cli::{sonarr::add_command_handler::SonarrAddCommandHandler, CliCommandHandler},
      models::{sonarr_models::SonarrSerdeable, Serdeable},
      network::{sonarr_network::SonarrEvent, MockNetworkTrait, NetworkEvent},
    };

    use super::*;
    use mockall::predicate::eq;

    use serde_json::json;
    use tokio::sync::Mutex;

    #[tokio::test]
    async fn test_handle_add_root_folder_command() {
      let expected_root_folder_path = "/nfs/test".to_owned();
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::AddRootFolder(Some(expected_root_folder_path.clone())).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let add_root_folder_command = SonarrAddCommand::RootFolder {
        root_folder_path: expected_root_folder_path,
      };

      let result =
        SonarrAddCommandHandler::with(&app_arc, add_root_folder_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_add_tag_command() {
      let expected_tag_name = "test".to_owned();
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::AddTag(expected_tag_name.clone()).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let add_tag_command = SonarrAddCommand::Tag {
        name: expected_tag_name,
      };

      let result = SonarrAddCommandHandler::with(&app_arc, add_tag_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }
  }
}
