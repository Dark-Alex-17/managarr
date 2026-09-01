#[cfg(test)]
mod tests {
  use clap::{CommandFactory, Parser, error::ErrorKind};

  use crate::{
    Cli,
    cli::{
      Command,
      readarr::{ReadarrCommand, delete_command_handler::ReadarrDeleteCommand},
    },
  };
  use pretty_assertions::assert_eq;

  #[test]
  fn test_readarr_delete_command_from() {
    let command = ReadarrDeleteCommand::Tag { tag_id: 1 };

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Readarr(ReadarrCommand::Delete(command)));
  }

  mod cli {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_delete_author_requires_arguments() {
      let result = Cli::command().try_get_matches_from(["managarr", "readarr", "delete", "author"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_delete_author_author_id_requires_a_number() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "delete",
        "author",
        "--author-id",
        "not_a_number",
      ]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::ValueValidation);
    }

    #[test]
    fn test_delete_author_defaults() {
      let expected_args = ReadarrDeleteCommand::Author {
        author_id: 1,
        delete_files_from_disk: false,
        add_list_exclusion: false,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "readarr",
        "delete",
        "author",
        "--author-id",
        "1",
      ]);

      assert_ok!(&result);

      let Some(Command::Readarr(ReadarrCommand::Delete(delete_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(delete_command, expected_args);
    }

    #[test]
    fn test_delete_author_all_args_defined() {
      let expected_args = ReadarrDeleteCommand::Author {
        author_id: 1,
        delete_files_from_disk: true,
        add_list_exclusion: true,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "readarr",
        "delete",
        "author",
        "--author-id",
        "1",
        "--delete-files-from-disk",
        "--add-list-exclusion",
      ]);

      assert_ok!(&result);

      let Some(Command::Readarr(ReadarrCommand::Delete(delete_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(delete_command, expected_args);
    }

    #[test]
    fn test_delete_blocklist_item_requires_arguments() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "readarr", "delete", "blocklist-item"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_delete_blocklist_item_blocklist_item_id_requires_a_number() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "delete",
        "blocklist-item",
        "--blocklist-item-id",
        "not_a_number",
      ]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::ValueValidation);
    }

    #[test]
    fn test_delete_blocklist_item_success() {
      let expected_args = ReadarrDeleteCommand::BlocklistItem {
        blocklist_item_id: 7,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "readarr",
        "delete",
        "blocklist-item",
        "--blocklist-item-id",
        "7",
      ]);

      assert_ok!(&result);

      let Some(Command::Readarr(ReadarrCommand::Delete(delete_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(delete_command, expected_args);
    }

    #[test]
    fn test_delete_book_requires_arguments() {
      let result = Cli::command().try_get_matches_from(["managarr", "readarr", "delete", "book"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_delete_book_book_id_requires_a_number() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "delete",
        "book",
        "--book-id",
        "not_a_number",
      ]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::ValueValidation);
    }

    #[test]
    fn test_delete_book_defaults() {
      let expected_args = ReadarrDeleteCommand::Book {
        book_id: 1,
        delete_files_from_disk: false,
        add_list_exclusion: false,
      };

      let result = Cli::try_parse_from(["managarr", "readarr", "delete", "book", "--book-id", "1"]);

      assert_ok!(&result);

      let Some(Command::Readarr(ReadarrCommand::Delete(delete_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(delete_command, expected_args);
    }

    #[test]
    fn test_delete_book_all_args_defined() {
      let expected_args = ReadarrDeleteCommand::Book {
        book_id: 1,
        delete_files_from_disk: true,
        add_list_exclusion: true,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "readarr",
        "delete",
        "book",
        "--book-id",
        "1",
        "--delete-files-from-disk",
        "--add-list-exclusion",
      ]);

      assert_ok!(&result);

      let Some(Command::Readarr(ReadarrCommand::Delete(delete_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(delete_command, expected_args);
    }

    #[test]
    fn test_delete_book_file_requires_arguments() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "readarr", "delete", "book-file"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_delete_book_file_book_file_id_requires_a_number() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "delete",
        "book-file",
        "--book-file-id",
        "not_a_number",
      ]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::ValueValidation);
    }

    #[test]
    fn test_delete_book_file_success() {
      let expected_args = ReadarrDeleteCommand::BookFile { book_file_id: 1 };

      let result = Cli::try_parse_from([
        "managarr",
        "readarr",
        "delete",
        "book-file",
        "--book-file-id",
        "1",
      ]);

      assert_ok!(&result);

      let Some(Command::Readarr(ReadarrCommand::Delete(delete_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(delete_command, expected_args);
    }

    #[test]
    fn test_delete_download_requires_arguments() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "readarr", "delete", "download"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_delete_download_download_id_requires_a_number() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "delete",
        "download",
        "--download-id",
        "not_a_number",
      ]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::ValueValidation);
    }

    #[test]
    fn test_delete_download_success() {
      let expected_args = ReadarrDeleteCommand::Download { download_id: 1 };

      let result = Cli::try_parse_from([
        "managarr",
        "readarr",
        "delete",
        "download",
        "--download-id",
        "1",
      ]);

      assert_ok!(&result);

      let Some(Command::Readarr(ReadarrCommand::Delete(delete_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(delete_command, expected_args);
    }

    #[test]
    fn test_delete_root_folder_requires_arguments() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "readarr", "delete", "root-folder"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_delete_root_folder_root_folder_id_requires_a_number() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "delete",
        "root-folder",
        "--root-folder-id",
        "not_a_number",
      ]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::ValueValidation);
    }

    #[test]
    fn test_delete_root_folder_success() {
      let expected_args = ReadarrDeleteCommand::RootFolder { root_folder_id: 1 };

      let result = Cli::try_parse_from([
        "managarr",
        "readarr",
        "delete",
        "root-folder",
        "--root-folder-id",
        "1",
      ]);

      assert_ok!(&result);

      let Some(Command::Readarr(ReadarrCommand::Delete(delete_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(delete_command, expected_args);
    }

    #[test]
    fn test_delete_tag_requires_arguments() {
      let result = Cli::command().try_get_matches_from(["managarr", "readarr", "delete", "tag"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_delete_tag_tag_id_requires_a_number() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "delete",
        "tag",
        "--tag-id",
        "not_a_number",
      ]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::ValueValidation);
    }

    #[test]
    fn test_delete_tag_success() {
      let expected_args = ReadarrDeleteCommand::Tag { tag_id: 1 };

      let result = Cli::try_parse_from(["managarr", "readarr", "delete", "tag", "--tag-id", "1"]);

      assert_ok!(&result);

      let Some(Command::Readarr(ReadarrCommand::Delete(delete_command))) = result.unwrap().command
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

    use crate::cli::CliCommandHandler;
    use crate::cli::readarr::delete_command_handler::{
      ReadarrDeleteCommand, ReadarrDeleteCommandHandler,
    };
    use crate::models::Serdeable;
    use crate::models::readarr_models::{DeleteParams, ReadarrSerdeable};
    use crate::network::readarr_network::ReadarrEvent;
    use crate::{
      app::App,
      network::{MockNetworkTrait, NetworkEvent},
    };

    #[tokio::test]
    async fn test_handle_delete_author_command() {
      let expected_delete_author_params = DeleteParams {
        id: 1,
        delete_files: true,
        add_import_list_exclusion: false,
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::DeleteAuthor(expected_delete_author_params).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let delete_author_command = ReadarrDeleteCommand::Author {
        author_id: 1,
        delete_files_from_disk: true,
        add_list_exclusion: false,
      };

      let result =
        ReadarrDeleteCommandHandler::with(&app_arc, delete_author_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_delete_book_command() {
      let expected_delete_book_params = DeleteParams {
        id: 1,
        delete_files: true,
        add_import_list_exclusion: false,
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::DeleteBook(expected_delete_book_params).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let delete_book_command = ReadarrDeleteCommand::Book {
        book_id: 1,
        delete_files_from_disk: true,
        add_list_exclusion: false,
      };

      let result =
        ReadarrDeleteCommandHandler::with(&app_arc, delete_book_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_delete_blocklist_item_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::DeleteBlocklistItem(7).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let delete_blocklist_item_command = ReadarrDeleteCommand::BlocklistItem {
        blocklist_item_id: 7,
      };

      let result = ReadarrDeleteCommandHandler::with(
        &app_arc,
        delete_blocklist_item_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_delete_book_file_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(ReadarrEvent::DeleteBookFile(1).into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let delete_book_file_command = ReadarrDeleteCommand::BookFile { book_file_id: 1 };

      let result =
        ReadarrDeleteCommandHandler::with(&app_arc, delete_book_file_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_delete_download_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(ReadarrEvent::DeleteDownload(1).into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let delete_download_command = ReadarrDeleteCommand::Download { download_id: 1 };

      let result =
        ReadarrDeleteCommandHandler::with(&app_arc, delete_download_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_delete_root_folder_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(ReadarrEvent::DeleteRootFolder(1).into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let delete_root_folder_command = ReadarrDeleteCommand::RootFolder { root_folder_id: 1 };

      let result =
        ReadarrDeleteCommandHandler::with(&app_arc, delete_root_folder_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_delete_tag_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(ReadarrEvent::DeleteTag(1).into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let delete_tag_command = ReadarrDeleteCommand::Tag { tag_id: 1 };

      let result =
        ReadarrDeleteCommandHandler::with(&app_arc, delete_tag_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }
  }
}
