#[cfg(test)]
mod tests {
  use crate::{
    Cli,
    cli::{
      Command,
      lidarr::{LidarrCommand, delete_command_handler::LidarrDeleteCommand},
    },
  };
  use clap::{CommandFactory, Parser, error::ErrorKind};
  use pretty_assertions::assert_eq;

  #[test]
  fn test_lidarr_delete_command_from() {
    let command = LidarrDeleteCommand::Artist {
      artist_id: 1,
      delete_files_from_disk: false,
      add_list_exclusion: false,
    };

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Lidarr(LidarrCommand::Delete(command)));
  }

  mod cli {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_delete_album_requires_arguments() {
      let result = Cli::command().try_get_matches_from(["managarr", "lidarr", "delete", "album"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_delete_album_defaults() {
      let expected_args = LidarrDeleteCommand::Album {
        album_id: 1,
        delete_files_from_disk: false,
        add_list_exclusion: false,
      };

      let result =
        Cli::try_parse_from(["managarr", "lidarr", "delete", "album", "--album-id", "1"]);

      assert_ok!(&result);

      let Some(Command::Lidarr(LidarrCommand::Delete(delete_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(delete_command, expected_args);
    }

    #[test]
    fn test_delete_album_all_args_defined() {
      let expected_args = LidarrDeleteCommand::Album {
        album_id: 1,
        delete_files_from_disk: true,
        add_list_exclusion: true,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "lidarr",
        "delete",
        "album",
        "--album-id",
        "1",
        "--delete-files-from-disk",
        "--add-list-exclusion",
      ]);

      assert_ok!(&result);

      let Some(Command::Lidarr(LidarrCommand::Delete(delete_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(delete_command, expected_args);
    }

    #[test]
    fn test_delete_artist_requires_arguments() {
      let result = Cli::command().try_get_matches_from(["managarr", "lidarr", "delete", "artist"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_delete_artist_defaults() {
      let expected_args = LidarrDeleteCommand::Artist {
        artist_id: 1,
        delete_files_from_disk: false,
        add_list_exclusion: false,
      };

      let result =
        Cli::try_parse_from(["managarr", "lidarr", "delete", "artist", "--artist-id", "1"]);

      assert_ok!(&result);

      let Some(Command::Lidarr(LidarrCommand::Delete(delete_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(delete_command, expected_args);
    }

    #[test]
    fn test_delete_artist_all_args_defined() {
      let expected_args = LidarrDeleteCommand::Artist {
        artist_id: 1,
        delete_files_from_disk: true,
        add_list_exclusion: true,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "lidarr",
        "delete",
        "artist",
        "--artist-id",
        "1",
        "--delete-files-from-disk",
        "--add-list-exclusion",
      ]);

      assert_ok!(&result);

      let Some(Command::Lidarr(LidarrCommand::Delete(delete_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(delete_command, expected_args);
    }

    #[test]
    fn test_delete_download_requires_arguments() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "lidarr", "delete", "download"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_delete_download_success() {
      let expected_args = LidarrDeleteCommand::Download { download_id: 1 };

      let result = Cli::try_parse_from([
        "managarr",
        "lidarr",
        "delete",
        "download",
        "--download-id",
        "1",
      ]);

      assert_ok!(&result);

      let Some(Command::Lidarr(LidarrCommand::Delete(delete_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(delete_command, expected_args);
    }

    #[test]
    fn test_delete_root_folder_requires_arguments() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "lidarr", "delete", "root-folder"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_delete_root_folder_success() {
      let expected_args = LidarrDeleteCommand::RootFolder { root_folder_id: 1 };

      let result = Cli::try_parse_from([
        "managarr",
        "lidarr",
        "delete",
        "root-folder",
        "--root-folder-id",
        "1",
      ]);

      assert_ok!(&result);

      let Some(Command::Lidarr(LidarrCommand::Delete(delete_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(delete_command, expected_args);
    }

    #[test]
    fn test_delete_tag_requires_arguments() {
      let result = Cli::command().try_get_matches_from(["managarr", "lidarr", "delete", "tag"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_delete_tag_success() {
      let expected_args = LidarrDeleteCommand::Tag { tag_id: 1 };

      let result = Cli::try_parse_from(["managarr", "lidarr", "delete", "tag", "--tag-id", "1"]);

      assert_ok!(&result);

      let Some(Command::Lidarr(LidarrCommand::Delete(delete_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(delete_command, expected_args);
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
        lidarr::delete_command_handler::{LidarrDeleteCommand, LidarrDeleteCommandHandler},
      },
      models::{
        Serdeable,
        lidarr_models::{DeleteParams, LidarrSerdeable},
      },
      network::{MockNetworkTrait, NetworkEvent, lidarr_network::LidarrEvent},
    };

    #[tokio::test]
    async fn test_handle_delete_album_command() {
      let expected_delete_album_params = DeleteParams {
        id: 1,
        delete_files: true,
        add_import_list_exclusion: true,
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          LidarrEvent::DeleteAlbum(expected_delete_album_params).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let delete_album_command = LidarrDeleteCommand::Album {
        album_id: 1,
        delete_files_from_disk: true,
        add_list_exclusion: true,
      };

      let result =
        LidarrDeleteCommandHandler::with(&app_arc, delete_album_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_delete_artist_command() {
      let expected_delete_artist_params = DeleteParams {
        id: 1,
        delete_files: true,
        add_import_list_exclusion: true,
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          LidarrEvent::DeleteArtist(expected_delete_artist_params).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let delete_artist_command = LidarrDeleteCommand::Artist {
        artist_id: 1,
        delete_files_from_disk: true,
        add_list_exclusion: true,
      };

      let result =
        LidarrDeleteCommandHandler::with(&app_arc, delete_artist_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_delete_download_command() {
      let expected_download_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          LidarrEvent::DeleteDownload(expected_download_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let delete_download_command = LidarrDeleteCommand::Download { download_id: 1 };

      let result =
        LidarrDeleteCommandHandler::with(&app_arc, delete_download_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_delete_root_folder_command() {
      let expected_root_folder_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          LidarrEvent::DeleteRootFolder(expected_root_folder_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let delete_root_folder_command = LidarrDeleteCommand::RootFolder { root_folder_id: 1 };

      let result =
        LidarrDeleteCommandHandler::with(&app_arc, delete_root_folder_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_delete_tag_command() {
      let expected_tag_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          LidarrEvent::DeleteTag(expected_tag_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let delete_tag_command = LidarrDeleteCommand::Tag { tag_id: 1 };

      let result =
        LidarrDeleteCommandHandler::with(&app_arc, delete_tag_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }
  }
}
