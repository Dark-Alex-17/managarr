#[cfg(test)]
mod tests {
  mod cli {
    use clap::CommandFactory;
    use clap::error::ErrorKind;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::Cli;

    #[test]
    fn test_readarr_command_requires_a_subcommand() {
      let result = Cli::command().try_get_matches_from(["managarr", "readarr"]);

      assert_err!(&result);
    }

    #[test]
    fn test_mark_history_item_as_failed_requires_history_item_id() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "readarr", "mark-history-item-as-failed"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_download_release_requires_guid() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "download-release",
        "--indexer-id",
        "6",
      ]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_download_release_requires_indexer_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "download-release",
        "--guid",
        "test-release-guid",
      ]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_download_release_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "download-release",
        "--guid",
        "test-release-guid",
        "--indexer-id",
        "6",
      ]);

      assert_ok!(&result);
    }

    #[test]
    fn test_mark_history_item_as_failed_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "mark-history-item-as-failed",
        "--history-item-id",
        "1234",
      ]);

      assert_ok!(&result);
    }

    #[test]
    fn test_search_new_author_requires_query() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "readarr", "search-new-author"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_search_new_author_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "search-new-author",
        "--query",
        "test query",
      ]);

      assert_ok!(&result);
    }

    #[test]
    fn test_start_task_requires_task_name() {
      let result = Cli::command().try_get_matches_from(["managarr", "readarr", "start-task"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_start_task_task_name_validation() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "start-task",
        "--task-name",
        "test",
      ]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_start_task_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "start-task",
        "--task-name",
        "check-health",
      ]);

      assert_ok!(&result);
    }

    #[test]
    fn test_test_indexer_requires_indexer_id() {
      let result = Cli::command().try_get_matches_from(["managarr", "readarr", "test-indexer"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_test_indexer_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "test-indexer",
        "--indexer-id",
        "8",
      ]);

      assert_ok!(&result);
    }

    #[test]
    fn test_toggle_author_monitoring_requires_author_id() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "readarr", "toggle-author-monitoring"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_toggle_author_monitoring_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "toggle-author-monitoring",
        "--author-id",
        "1",
      ]);

      assert_ok!(&result);
    }

    #[test]
    fn test_toggle_book_monitoring_requires_book_id() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "readarr", "toggle-book-monitoring"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[rstest]
    fn test_commands_that_have_no_arg_requirements(
      #[values("clear-blocklist", "test-all-indexers")] subcommand: &str,
    ) {
      let result = Cli::command().try_get_matches_from(["managarr", "readarr", subcommand]);

      assert_ok!(&result);
    }

    #[test]
    fn test_toggle_book_monitoring_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "toggle-book-monitoring",
        "--book-id",
        "1",
      ]);

      assert_ok!(&result);
    }
  }

  mod handler {
    use std::sync::Arc;

    use mockall::predicate::eq;
    use serde_json::json;
    use tokio::sync::Mutex;

    use crate::models::readarr_models::{
      BlocklistItem, BlocklistResponse, ReadarrSerdeable, ReadarrTaskName,
    };
    use crate::models::servarr_models::ReleaseDownloadBody;
    use crate::{
      app::App,
      cli::{
        CliCommandHandler,
        readarr::{ReadarrCliHandler, ReadarrCommand},
      },
      models::Serdeable,
      network::{MockNetworkTrait, NetworkEvent, readarr_network::ReadarrEvent},
    };

    #[tokio::test]
    async fn test_mark_history_item_as_failed_command() {
      let expected_history_item_id = 1234i64;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::MarkHistoryItemAsFailed(expected_history_item_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let mark_history_item_as_failed_command = ReadarrCommand::MarkHistoryItemAsFailed {
        history_item_id: expected_history_item_id,
      };

      let result = ReadarrCliHandler::with(
        &app_arc,
        mark_history_item_as_failed_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_download_release_command() {
      let expected_release_download_body = ReleaseDownloadBody {
        guid: "test-release-guid".to_owned(),
        indexer_id: 6,
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::DownloadRelease(expected_release_download_body).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let download_release_command = ReadarrCommand::DownloadRelease {
        guid: "test-release-guid".to_owned(),
        indexer_id: 6,
      };

      let result = ReadarrCliHandler::with(&app_arc, download_release_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_search_new_author_command() {
      let expected_query = "test author".to_owned();
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::SearchNewAuthor(expected_query.clone()).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let search_new_author_command = ReadarrCommand::SearchNewAuthor {
        query: expected_query,
      };

      let result = ReadarrCliHandler::with(&app_arc, search_new_author_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_start_task_command() {
      let expected_task_name = ReadarrTaskName::CheckHealth;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::StartTask(expected_task_name).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let start_task_command = ReadarrCommand::StartTask {
        task_name: ReadarrTaskName::CheckHealth,
      };

      let result = ReadarrCliHandler::with(&app_arc, start_task_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_test_indexer_command() {
      let expected_indexer_id = 8;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::TestIndexer(expected_indexer_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let test_indexer_command = ReadarrCommand::TestIndexer { indexer_id: 8 };

      let result = ReadarrCliHandler::with(&app_arc, test_indexer_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_test_all_indexers_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(ReadarrEvent::TestAllIndexers.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let test_all_indexers_command = ReadarrCommand::TestAllIndexers;

      let result = ReadarrCliHandler::with(&app_arc, test_all_indexers_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_toggle_author_monitoring_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::ToggleAuthorMonitoring(1).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let toggle_author_monitoring_command =
        ReadarrCommand::ToggleAuthorMonitoring { author_id: 1 };

      let result = ReadarrCliHandler::with(
        &app_arc,
        toggle_author_monitoring_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_toggle_book_monitoring_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::ToggleBookMonitoring(1).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let toggle_book_monitoring_command = ReadarrCommand::ToggleBookMonitoring { book_id: 1 };

      let result =
        ReadarrCliHandler::with(&app_arc, toggle_book_monitoring_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_clear_blocklist_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(ReadarrEvent::GetBlocklist.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::BlocklistResponse(
            BlocklistResponse {
              records: vec![BlocklistItem::default()],
            },
          )))
        });
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(ReadarrEvent::ClearBlocklist.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let clear_blocklist_command = ReadarrCommand::ClearBlocklist;

      let result = ReadarrCliHandler::with(&app_arc, clear_blocklist_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }
  }
}
