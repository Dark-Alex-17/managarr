#[cfg(test)]
mod tests {
  use crate::{
    cli::{
      radarr::{delete_command_handler::RadarrDeleteCommand, RadarrCommand},
      Command,
    },
    Cli,
  };
  use clap::{error::ErrorKind, CommandFactory, Parser};

  #[test]
  fn test_radarr_delete_command_from() {
    let command = RadarrDeleteCommand::BlocklistItem {
      blocklist_item_id: 1,
    };

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Radarr(RadarrCommand::Delete(command)));
  }

  mod cli {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_delete_blocklist_item_requires_arguments() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "radarr", "delete", "blocklist-item"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_delete_blocklist_item_success() {
      let expected_args = RadarrDeleteCommand::BlocklistItem {
        blocklist_item_id: 1,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "radarr",
        "delete",
        "blocklist-item",
        "--blocklist-item-id",
        "1",
      ]);

      assert!(result.is_ok());

      if let Some(Command::Radarr(RadarrCommand::Delete(delete_command))) = result.unwrap().command
      {
        assert_eq!(delete_command, expected_args);
      }
    }

    #[test]
    fn test_delete_download_requires_arguments() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "radarr", "delete", "download"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_delete_download_success() {
      let expected_args = RadarrDeleteCommand::Download { download_id: 1 };

      let result = Cli::try_parse_from([
        "managarr",
        "radarr",
        "delete",
        "download",
        "--download-id",
        "1",
      ]);

      assert!(result.is_ok());

      if let Some(Command::Radarr(RadarrCommand::Delete(delete_command))) = result.unwrap().command
      {
        assert_eq!(delete_command, expected_args);
      }
    }

    #[test]
    fn test_delete_indexer_requires_arguments() {
      let result = Cli::command().try_get_matches_from(["managarr", "radarr", "delete", "indexer"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_delete_indexer_success() {
      let expected_args = RadarrDeleteCommand::Indexer { indexer_id: 1 };

      let result = Cli::try_parse_from([
        "managarr",
        "radarr",
        "delete",
        "indexer",
        "--indexer-id",
        "1",
      ]);

      assert!(result.is_ok());

      if let Some(Command::Radarr(RadarrCommand::Delete(delete_command))) = result.unwrap().command
      {
        assert_eq!(delete_command, expected_args);
      }
    }

    #[test]
    fn test_delete_movie_requires_arguments() {
      let result = Cli::command().try_get_matches_from(["managarr", "radarr", "delete", "movie"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_delete_movie_defaults() {
      let expected_args = RadarrDeleteCommand::Movie {
        movie_id: 1,
        delete_files_from_disk: false,
        add_list_exclusion: false,
      };

      let result =
        Cli::try_parse_from(["managarr", "radarr", "delete", "movie", "--movie-id", "1"]);

      assert!(result.is_ok());

      if let Some(Command::Radarr(RadarrCommand::Delete(delete_command))) = result.unwrap().command
      {
        assert_eq!(delete_command, expected_args);
      }
    }

    #[test]
    fn test_delete_movie_all_args_defined() {
      let expected_args = RadarrDeleteCommand::Movie {
        movie_id: 1,
        delete_files_from_disk: true,
        add_list_exclusion: true,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "radarr",
        "delete",
        "movie",
        "--movie-id",
        "1",
        "--delete-files-from-disk",
        "--add-list-exclusion",
      ]);

      assert!(result.is_ok());

      if let Some(Command::Radarr(RadarrCommand::Delete(delete_command))) = result.unwrap().command
      {
        assert_eq!(delete_command, expected_args);
      }
    }

    #[test]
    fn test_delete_root_folder_requires_arguments() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "radarr", "delete", "root-folder"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_delete_root_folder_success() {
      let expected_args = RadarrDeleteCommand::RootFolder { root_folder_id: 1 };

      let result = Cli::try_parse_from([
        "managarr",
        "radarr",
        "delete",
        "root-folder",
        "--root-folder-id",
        "1",
      ]);

      assert!(result.is_ok());

      if let Some(Command::Radarr(RadarrCommand::Delete(delete_command))) = result.unwrap().command
      {
        assert_eq!(delete_command, expected_args);
      }
    }

    #[test]
    fn test_delete_tag_requires_arguments() {
      let result = Cli::command().try_get_matches_from(["managarr", "radarr", "delete", "tag"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_delete_tag_success() {
      let expected_args = RadarrDeleteCommand::Tag { tag_id: 1 };

      let result = Cli::try_parse_from(["managarr", "radarr", "delete", "tag", "--tag-id", "1"]);

      assert!(result.is_ok());

      if let Some(Command::Radarr(RadarrCommand::Delete(delete_command))) = result.unwrap().command
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
        radarr::delete_command_handler::{RadarrDeleteCommand, RadarrDeleteCommandHandler},
        CliCommandHandler,
      },
      models::{
        radarr_models::{DeleteMovieParams, RadarrSerdeable},
        Serdeable,
      },
      network::{radarr_network::RadarrEvent, MockNetworkTrait, NetworkEvent},
    };

    #[tokio::test]
    async fn test_handle_delete_blocklist_item_command() {
      let expected_blocklist_item_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::DeleteBlocklistItem(Some(expected_blocklist_item_id)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let delete_blocklist_item_command = RadarrDeleteCommand::BlocklistItem {
        blocklist_item_id: 1,
      };

      let result = RadarrDeleteCommandHandler::with(
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
          RadarrEvent::DeleteDownload(Some(expected_download_id)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let delete_download_command = RadarrDeleteCommand::Download { download_id: 1 };

      let result =
        RadarrDeleteCommandHandler::with(&app_arc, delete_download_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_delete_indexer_command() {
      let expected_indexer_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::DeleteIndexer(Some(expected_indexer_id)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let delete_indexer_command = RadarrDeleteCommand::Indexer { indexer_id: 1 };

      let result =
        RadarrDeleteCommandHandler::with(&app_arc, delete_indexer_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_delete_movie_command() {
      let expected_delete_movie_params = DeleteMovieParams {
        id: 1,
        delete_movie_files: true,
        add_list_exclusion: true,
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::DeleteMovie(Some(expected_delete_movie_params)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let delete_movie_command = RadarrDeleteCommand::Movie {
        movie_id: 1,
        delete_files_from_disk: true,
        add_list_exclusion: true,
      };

      let result =
        RadarrDeleteCommandHandler::with(&app_arc, delete_movie_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_delete_root_folder_command() {
      let expected_root_folder_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::DeleteRootFolder(Some(expected_root_folder_id)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let delete_root_folder_command = RadarrDeleteCommand::RootFolder { root_folder_id: 1 };

      let result =
        RadarrDeleteCommandHandler::with(&app_arc, delete_root_folder_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_delete_tag_command() {
      let expected_tag_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::DeleteTag(expected_tag_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let delete_tag_command = RadarrDeleteCommand::Tag { tag_id: 1 };

      let result =
        RadarrDeleteCommandHandler::with(&app_arc, delete_tag_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }
  }
}
