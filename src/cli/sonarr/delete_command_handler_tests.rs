#[cfg(test)]
mod tests {
  use crate::{
    Cli,
    cli::{
      Command,
      sonarr::{SonarrCommand, delete_command_handler::SonarrDeleteCommand},
    },
  };
  use clap::{CommandFactory, Parser, error::ErrorKind};
  use pretty_assertions::assert_eq;

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

      assert_err!(&result);
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

      assert_ok!(&result);

      let Some(Command::Sonarr(SonarrCommand::Delete(delete_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(delete_command, expected_args);
    }

    #[test]
    fn test_delete_download_requires_arguments() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "sonarr", "delete", "download"]);

      assert_err!(&result);
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

      assert_ok!(&result);

      let Some(Command::Sonarr(SonarrCommand::Delete(delete_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(delete_command, expected_args);
    }

    #[test]
    fn test_delete_episode_file_requires_arguments() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "sonarr", "delete", "episode-file"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_delete_episode_file_success() {
      let expected_args = SonarrDeleteCommand::EpisodeFile { episode_file_id: 1 };

      let result = Cli::try_parse_from([
        "managarr",
        "sonarr",
        "delete",
        "episode-file",
        "--episode-file-id",
        "1",
      ]);

      assert_ok!(&result);

      let Some(Command::Sonarr(SonarrCommand::Delete(delete_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(delete_command, expected_args);
    }

    #[test]
    fn test_delete_indexer_requires_arguments() {
      let result = Cli::command().try_get_matches_from(["managarr", "sonarr", "delete", "indexer"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_delete_indexer_success() {
      let expected_args = SonarrDeleteCommand::Indexer { indexer_id: 1 };

      let result = Cli::try_parse_from([
        "managarr",
        "sonarr",
        "delete",
        "indexer",
        "--indexer-id",
        "1",
      ]);

      assert_ok!(&result);

      let Some(Command::Sonarr(SonarrCommand::Delete(delete_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(delete_command, expected_args);
    }

    #[test]
    fn test_delete_root_folder_requires_arguments() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "sonarr", "delete", "root-folder"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_delete_root_folder_success() {
      let expected_args = SonarrDeleteCommand::RootFolder { root_folder_id: 1 };

      let result = Cli::try_parse_from([
        "managarr",
        "sonarr",
        "delete",
        "root-folder",
        "--root-folder-id",
        "1",
      ]);

      assert_ok!(&result);

      let Some(Command::Sonarr(SonarrCommand::Delete(delete_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(delete_command, expected_args);
    }

    #[test]
    fn test_delete_series_requires_arguments() {
      let result = Cli::command().try_get_matches_from(["managarr", "sonarr", "delete", "series"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_delete_series_defaults() {
      let expected_args = SonarrDeleteCommand::Series {
        series_id: 1,
        delete_files_from_disk: false,
        add_list_exclusion: false,
      };

      let result =
        Cli::try_parse_from(["managarr", "sonarr", "delete", "series", "--series-id", "1"]);

      assert_ok!(&result);

      let Some(Command::Sonarr(SonarrCommand::Delete(delete_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(delete_command, expected_args);
    }

    #[test]
    fn test_delete_series_all_args_defined() {
      let expected_args = SonarrDeleteCommand::Series {
        series_id: 1,
        delete_files_from_disk: true,
        add_list_exclusion: true,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "sonarr",
        "delete",
        "series",
        "--series-id",
        "1",
        "--delete-files-from-disk",
        "--add-list-exclusion",
      ]);

      assert_ok!(&result);

      let Some(Command::Sonarr(SonarrCommand::Delete(delete_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(delete_command, expected_args);
    }

    #[test]
    fn test_delete_tag_requires_arguments() {
      let result = Cli::command().try_get_matches_from(["managarr", "sonarr", "delete", "tag"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_delete_tag_success() {
      let expected_args = SonarrDeleteCommand::Tag { tag_id: 1 };

      let result = Cli::try_parse_from(["managarr", "sonarr", "delete", "tag", "--tag-id", "1"]);

      assert_ok!(&result);

      let Some(Command::Sonarr(SonarrCommand::Delete(delete_command))) = result.unwrap().command
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
        sonarr::delete_command_handler::{SonarrDeleteCommand, SonarrDeleteCommandHandler},
      },
      models::{
        Serdeable,
        sonarr_models::{DeleteSeriesParams, SonarrSerdeable},
      },
      network::{MockNetworkTrait, NetworkEvent, sonarr_network::SonarrEvent},
    };

    #[tokio::test]
    async fn test_handle_delete_blocklist_item_command() {
      let expected_blocklist_item_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::DeleteBlocklistItem(expected_blocklist_item_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
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

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_delete_download_command() {
      let expected_download_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::DeleteDownload(expected_download_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let delete_download_command = SonarrDeleteCommand::Download { download_id: 1 };

      let result =
        SonarrDeleteCommandHandler::with(&app_arc, delete_download_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_delete_episode_file_command() {
      let expected_episode_file_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::DeleteEpisodeFile(expected_episode_file_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let delete_episode_file_command = SonarrDeleteCommand::EpisodeFile { episode_file_id: 1 };

      let result =
        SonarrDeleteCommandHandler::with(&app_arc, delete_episode_file_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_delete_indexer_command() {
      let expected_indexer_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::DeleteIndexer(expected_indexer_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let delete_indexer_command = SonarrDeleteCommand::Indexer { indexer_id: 1 };

      let result =
        SonarrDeleteCommandHandler::with(&app_arc, delete_indexer_command, &mut mock_network)
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
          SonarrEvent::DeleteRootFolder(expected_root_folder_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let delete_root_folder_command = SonarrDeleteCommand::RootFolder { root_folder_id: 1 };

      let result =
        SonarrDeleteCommandHandler::with(&app_arc, delete_root_folder_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_delete_series_command() {
      let expected_delete_series_params = DeleteSeriesParams {
        id: 1,
        delete_series_files: true,
        add_list_exclusion: true,
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::DeleteSeries(expected_delete_series_params).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let delete_series_command = SonarrDeleteCommand::Series {
        series_id: 1,
        delete_files_from_disk: true,
        add_list_exclusion: true,
      };

      let result =
        SonarrDeleteCommandHandler::with(&app_arc, delete_series_command, &mut mock_network)
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
          SonarrEvent::DeleteTag(expected_tag_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let delete_tag_command = SonarrDeleteCommand::Tag { tag_id: 1 };

      let result =
        SonarrDeleteCommandHandler::with(&app_arc, delete_tag_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }
  }
}
