#[cfg(test)]
mod tests {
  use crate::cli::{
    Command,
    readarr::{ReadarrCommand, list_command_handler::ReadarrListCommand},
  };
  use pretty_assertions::assert_eq;

  #[test]
  fn test_readarr_command_from() {
    let command = ReadarrCommand::List(ReadarrListCommand::Authors);

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Readarr(command));
  }

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

    use crate::cli::readarr::add_command_handler::ReadarrAddCommand;
    use crate::cli::readarr::edit_command_handler::ReadarrEditCommand;
    use crate::cli::readarr::get_command_handler::ReadarrGetCommand;
    use crate::cli::readarr::refresh_command_handler::ReadarrRefreshCommand;
    use crate::cli::readarr::trigger_automatic_search_command_handler::ReadarrTriggerAutomaticSearchCommand;
    use crate::models::readarr_models::{
      Author, BlocklistItem, BlocklistResponse, DeleteParams, ReadarrSerdeable, ReadarrTaskName,
    };
    use crate::models::servarr_models::{IndexerSettings, ReleaseDownloadBody};
    use crate::{
      app::App,
      cli::{
        CliCommandHandler,
        readarr::{
          ReadarrCliHandler, ReadarrCommand, delete_command_handler::ReadarrDeleteCommand,
          list_command_handler::ReadarrListCommand,
        },
      },
      models::Serdeable,
      network::{MockNetworkTrait, NetworkEvent, readarr_network::ReadarrEvent},
    };

    #[tokio::test]
    async fn test_readarr_cli_handler_delegates_add_commands_to_the_add_command_handler() {
      let expected_tag_name = "test".to_owned();
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::AddTag(expected_tag_name.clone()).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let add_tag_command = ReadarrCommand::Add(ReadarrAddCommand::Tag {
        name: expected_tag_name,
      });

      let result = ReadarrCliHandler::with(&app_arc, add_tag_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_readarr_cli_handler_delegates_get_commands_to_the_get_command_handler() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(ReadarrEvent::GetStatus.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let get_system_status_command = ReadarrCommand::Get(ReadarrGetCommand::SystemStatus);

      let result = ReadarrCliHandler::with(&app_arc, get_system_status_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_readarr_cli_handler_delegates_delete_commands_to_the_delete_command_handler() {
      let expected_delete_author_params = DeleteParams {
        id: 7,
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
      let delete_author_command = ReadarrCommand::Delete(ReadarrDeleteCommand::Author {
        author_id: 7,
        delete_files_from_disk: true,
        add_list_exclusion: false,
      });

      let result = ReadarrCliHandler::with(&app_arc, delete_author_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_readarr_cli_handler_delegates_edit_commands_to_the_edit_command_handler() {
      let expected_edit_all_indexer_settings = IndexerSettings {
        id: 1,
        maximum_size: 26500,
        minimum_age: 17,
        retention: 43,
        rss_sync_interval: 35,
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::GetAllIndexerSettings.into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::IndexerSettings(
            IndexerSettings {
              id: 9,
              maximum_size: 31200,
              minimum_age: 22,
              retention: 58,
              rss_sync_interval: 90,
            },
          )))
        });
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::EditAllIndexerSettings(expected_edit_all_indexer_settings).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let edit_all_indexer_settings_command =
        ReadarrCommand::Edit(ReadarrEditCommand::AllIndexerSettings {
          maximum_size: Some(26500),
          minimum_age: Some(17),
          retention: Some(43),
          rss_sync_interval: Some(35),
        });

      let result = ReadarrCliHandler::with(
        &app_arc,
        edit_all_indexer_settings_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_readarr_cli_handler_delegates_list_commands_to_the_list_command_handler() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(ReadarrEvent::ListAuthors.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Authors(vec![
            Author::default(),
          ])))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let list_authors_command = ReadarrCommand::List(ReadarrListCommand::Authors);

      let result = ReadarrCliHandler::with(&app_arc, list_authors_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_readarr_cli_handler_delegates_refresh_commands_to_the_refresh_command_handler() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(ReadarrEvent::UpdateAllAuthors.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let refresh_all_authors_command = ReadarrCommand::Refresh(ReadarrRefreshCommand::AllAuthors);

      let result =
        ReadarrCliHandler::with(&app_arc, refresh_all_authors_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_readarr_cli_handler_delegates_trigger_automatic_search_commands_to_the_trigger_automatic_search_command_handler()
     {
      let expected_author_id = 7;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::TriggerAutomaticAuthorSearch(expected_author_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let trigger_automatic_search_command =
        ReadarrCommand::TriggerAutomaticSearch(ReadarrTriggerAutomaticSearchCommand::Author {
          author_id: expected_author_id,
        });

      let result = ReadarrCliHandler::with(
        &app_arc,
        trigger_automatic_search_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert_ok!(&result);
    }

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
