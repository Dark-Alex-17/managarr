#[cfg(test)]
mod tests {
  use crate::Cli;
  use crate::cli::{
    Command,
    readarr::{ReadarrCommand, list_command_handler::ReadarrListCommand},
  };
  use clap::CommandFactory;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_readarr_list_command_from() {
    let command = ReadarrListCommand::DiskSpace;

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Readarr(ReadarrCommand::List(command)));
  }

  mod cli {
    use super::*;
    use clap::{Parser, error::ErrorKind};
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    fn test_list_commands_have_no_arg_requirements(
      #[values(
        "authors",
        "blocklist",
        "disk-space",
        "indexers",
        "metadata-profiles",
        "quality-profiles",
        "queued-events",
        "root-folders",
        "tags",
        "tasks",
        "updates"
      )]
      subcommand: &str,
    ) {
      let result = Cli::command().try_get_matches_from(["managarr", "readarr", "list", subcommand]);

      assert_ok!(&result);
    }

    #[test]
    fn test_list_logs_events_flag_requires_arguments() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "readarr", "list", "logs", "--events"]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_list_downloads_count_flag_requires_arguments() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "list",
        "downloads",
        "--count",
      ]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_list_history_events_flag_requires_arguments() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "readarr", "list", "history", "--events"]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_list_author_history_requires_author_id() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "readarr", "list", "author-history"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_list_author_history_success() {
      let expected_args = ReadarrListCommand::AuthorHistory { author_id: 1 };
      let result = Cli::try_parse_from([
        "managarr",
        "readarr",
        "list",
        "author-history",
        "--author-id",
        "1",
      ]);

      assert_ok!(&result);

      let Some(Command::Readarr(ReadarrCommand::List(author_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(author_command, expected_args);
    }

    #[test]
    fn test_list_books_requires_author_id() {
      let result = Cli::command().try_get_matches_from(["managarr", "readarr", "list", "books"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_list_book_editions_requires_book_id() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "readarr", "list", "book-editions"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_list_book_editions_success() {
      let expected_args = ReadarrListCommand::BookEditions { book_id: 1 };
      let result = Cli::try_parse_from([
        "managarr",
        "readarr",
        "list",
        "book-editions",
        "--book-id",
        "1",
      ]);

      assert_ok!(&result);

      let Some(Command::Readarr(ReadarrCommand::List(book_editions_command))) =
        result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(book_editions_command, expected_args);
    }

    #[test]
    fn test_list_book_files_requires_book_id() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "readarr", "list", "book-files"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_list_book_files_success() {
      let expected_args = ReadarrListCommand::BookFiles { book_id: 1 };
      let result = Cli::try_parse_from([
        "managarr",
        "readarr",
        "list",
        "book-files",
        "--book-id",
        "1",
      ]);

      assert_ok!(&result);

      let Some(Command::Readarr(ReadarrCommand::List(book_files_command))) =
        result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(book_files_command, expected_args);
    }

    #[test]
    fn test_list_book_history_requires_author_id_and_book_id() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "readarr", "list", "book-history"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_list_book_history_success() {
      let expected_args = ReadarrListCommand::BookHistory {
        author_id: 1,
        book_id: 2,
      };
      let result = Cli::try_parse_from([
        "managarr",
        "readarr",
        "list",
        "book-history",
        "--author-id",
        "1",
        "--book-id",
        "2",
      ]);

      assert_ok!(&result);

      let Some(Command::Readarr(ReadarrCommand::List(book_history_command))) =
        result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(book_history_command, expected_args);
    }

    #[test]
    fn test_list_books_success() {
      let expected_args = ReadarrListCommand::Books { author_id: 1 };
      let result =
        Cli::try_parse_from(["managarr", "readarr", "list", "books", "--author-id", "1"]);

      assert_ok!(&result);

      let Some(Command::Readarr(ReadarrCommand::List(books_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(books_command, expected_args);
    }

    #[test]
    fn test_list_releases_requires_book_id() {
      let result = Cli::command().try_get_matches_from(["managarr", "readarr", "list", "releases"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_list_author_releases_requires_author_id() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "readarr", "list", "author-releases"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_list_author_releases_success() {
      let expected_args = ReadarrListCommand::AuthorReleases { author_id: 3 };
      let result = Cli::try_parse_from([
        "managarr",
        "readarr",
        "list",
        "author-releases",
        "--author-id",
        "3",
      ]);

      assert_ok!(&result);

      let Some(Command::Readarr(ReadarrCommand::List(author_releases_command))) =
        result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(author_releases_command, expected_args);
    }

    #[test]
    fn test_list_releases_success() {
      let expected_args = ReadarrListCommand::Releases { book_id: 7 };
      let result =
        Cli::try_parse_from(["managarr", "readarr", "list", "releases", "--book-id", "7"]);

      assert_ok!(&result);

      let Some(Command::Readarr(ReadarrCommand::List(releases_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(releases_command, expected_args);
    }

    #[test]
    fn test_list_logs_default_values() {
      let expected_args = ReadarrListCommand::Logs {
        events: 500,
        output_in_log_format: false,
      };
      let result = Cli::try_parse_from(["managarr", "readarr", "list", "logs"]);

      assert_ok!(&result);

      let Some(Command::Readarr(ReadarrCommand::List(logs_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(logs_command, expected_args);
    }

    #[test]
    fn test_list_downloads_default_values() {
      let expected_args = ReadarrListCommand::Downloads { count: 500 };
      let result = Cli::try_parse_from(["managarr", "readarr", "list", "downloads"]);

      assert_ok!(&result);

      let Some(Command::Readarr(ReadarrCommand::List(downloads_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(downloads_command, expected_args);
    }

    #[test]
    fn test_list_downloads_success() {
      let expected_args = ReadarrListCommand::Downloads { count: 1000 };
      let result = Cli::try_parse_from([
        "managarr",
        "readarr",
        "list",
        "downloads",
        "--count",
        "1000",
      ]);

      assert_ok!(&result);

      let Some(Command::Readarr(ReadarrCommand::List(downloads_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(downloads_command, expected_args);
    }

    #[test]
    fn test_list_history_default_values() {
      let expected_args = ReadarrListCommand::History { events: 500 };
      let result = Cli::try_parse_from(["managarr", "readarr", "list", "history"]);

      assert_ok!(&result);

      let Some(Command::Readarr(ReadarrCommand::List(history_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(history_command, expected_args);
    }

    #[test]
    fn test_list_history_success() {
      let expected_args = ReadarrListCommand::History { events: 1000 };
      let result =
        Cli::try_parse_from(["managarr", "readarr", "list", "history", "--events", "1000"]);

      assert_ok!(&result);

      let Some(Command::Readarr(ReadarrCommand::List(history_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(history_command, expected_args);
    }
  }

  mod handler {
    use std::sync::Arc;

    use mockall::predicate::eq;
    use pretty_assertions::assert_str_eq;
    use serde_json::json;
    use tokio::sync::Mutex;

    use crate::{
      app::App,
      cli::{
        CliCommandHandler,
        readarr::list_command_handler::{ReadarrListCommand, ReadarrListCommandHandler},
      },
      models::{HorizontallyScrollableText, Serdeable, readarr_models::ReadarrSerdeable},
      network::{MockNetworkTrait, NetworkEvent, readarr_network::ReadarrEvent},
    };

    #[tokio::test]
    async fn test_handle_list_authors_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(ReadarrEvent::ListAuthors.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let list_authors_command = ReadarrListCommand::Authors;

      let result =
        ReadarrListCommandHandler::with(&app_arc, list_authors_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_list_blocklist_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(ReadarrEvent::GetBlocklist.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let list_blocklist_command = ReadarrListCommand::Blocklist;

      let result =
        ReadarrListCommandHandler::with(&app_arc, list_blocklist_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_list_author_history_command() {
      let expected_author_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::GetAuthorHistory(expected_author_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let list_author_history_command = ReadarrListCommand::AuthorHistory { author_id: 1 };

      let result =
        ReadarrListCommandHandler::with(&app_arc, list_author_history_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_list_books_command() {
      let expected_author_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::GetBooks(expected_author_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let list_books_command = ReadarrListCommand::Books { author_id: 1 };

      let result = ReadarrListCommandHandler::with(&app_arc, list_books_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_list_book_editions_command() {
      let expected_book_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::GetBookEditions(expected_book_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let list_book_editions_command = ReadarrListCommand::BookEditions { book_id: 1 };

      let result =
        ReadarrListCommandHandler::with(&app_arc, list_book_editions_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_list_book_files_command() {
      let expected_book_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::GetBookFiles(expected_book_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let list_book_files_command = ReadarrListCommand::BookFiles { book_id: 1 };

      let result =
        ReadarrListCommandHandler::with(&app_arc, list_book_files_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_list_book_history_command() {
      let expected_author_id = 1;
      let expected_book_id = 2;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::GetBookHistory(expected_author_id, expected_book_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let list_book_history_command = ReadarrListCommand::BookHistory {
        author_id: 1,
        book_id: 2,
      };

      let result =
        ReadarrListCommandHandler::with(&app_arc, list_book_history_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_list_disk_space_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(ReadarrEvent::GetDiskSpace.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let list_disk_space_command = ReadarrListCommand::DiskSpace;

      let result =
        ReadarrListCommandHandler::with(&app_arc, list_disk_space_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_list_downloads_command() {
      let expected_count = 1000;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::GetDownloads(expected_count).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let list_downloads_command = ReadarrListCommand::Downloads { count: 1000 };

      let result =
        ReadarrListCommandHandler::with(&app_arc, list_downloads_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_list_history_command() {
      let expected_events = 1000;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::GetHistory(expected_events).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let list_history_command = ReadarrListCommand::History { events: 1000 };

      let result =
        ReadarrListCommandHandler::with(&app_arc, list_history_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_list_indexers_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(ReadarrEvent::GetIndexers.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let list_indexers_command = ReadarrListCommand::Indexers;

      let result =
        ReadarrListCommandHandler::with(&app_arc, list_indexers_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_list_logs_command() {
      let expected_events = 1000;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::GetLogs(expected_events).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let list_logs_command = ReadarrListCommand::Logs {
        events: 1000,
        output_in_log_format: false,
      };

      let result = ReadarrListCommandHandler::with(&app_arc, list_logs_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_list_logs_command_output_in_log_format() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(ReadarrEvent::GetLogs(1000).into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      app_arc
        .lock()
        .await
        .data
        .readarr_data
        .logs
        .set_items(vec![HorizontallyScrollableText::from("readarr log line")]);
      let list_logs_command = ReadarrListCommand::Logs {
        events: 1000,
        output_in_log_format: true,
      };

      let result = ReadarrListCommandHandler::with(&app_arc, list_logs_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
      assert_str_eq!(result.unwrap(), "[\n  \"readarr log line\"\n]");
    }

    #[tokio::test]
    async fn test_handle_list_metadata_profiles_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(ReadarrEvent::GetMetadataProfiles.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let list_metadata_profiles_command = ReadarrListCommand::MetadataProfiles;

      let result = ReadarrListCommandHandler::with(
        &app_arc,
        list_metadata_profiles_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_list_quality_profiles_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(ReadarrEvent::GetQualityProfiles.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let list_quality_profiles_command = ReadarrListCommand::QualityProfiles;

      let result =
        ReadarrListCommandHandler::with(&app_arc, list_quality_profiles_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_list_queued_events_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(ReadarrEvent::GetQueuedEvents.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let list_queued_events_command = ReadarrListCommand::QueuedEvents;

      let result =
        ReadarrListCommandHandler::with(&app_arc, list_queued_events_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_list_releases_command() {
      let expected_book_id = 7;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::GetBookReleases(expected_book_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let list_releases_command = ReadarrListCommand::Releases { book_id: 7 };

      let result =
        ReadarrListCommandHandler::with(&app_arc, list_releases_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_list_author_releases_command() {
      let expected_author_id = 3;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::GetAuthorReleases(expected_author_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let list_author_releases_command = ReadarrListCommand::AuthorReleases { author_id: 3 };

      let result =
        ReadarrListCommandHandler::with(&app_arc, list_author_releases_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_list_root_folders_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(ReadarrEvent::GetRootFolders.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let list_root_folders_command = ReadarrListCommand::RootFolders;

      let result =
        ReadarrListCommandHandler::with(&app_arc, list_root_folders_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_list_tags_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(ReadarrEvent::GetTags.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let list_tags_command = ReadarrListCommand::Tags;

      let result = ReadarrListCommandHandler::with(&app_arc, list_tags_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_list_tasks_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(ReadarrEvent::GetTasks.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let list_tasks_command = ReadarrListCommand::Tasks;

      let result = ReadarrListCommandHandler::with(&app_arc, list_tasks_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_list_updates_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(ReadarrEvent::GetUpdates.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let list_updates_command = ReadarrListCommand::Updates;

      let result =
        ReadarrListCommandHandler::with(&app_arc, list_updates_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }
  }
}
