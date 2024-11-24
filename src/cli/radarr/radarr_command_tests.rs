#[cfg(test)]
mod tests {
  use clap::error::ErrorKind;
  use clap::CommandFactory;

  use crate::cli::radarr::RadarrCommand;
  use crate::cli::Command;
  use crate::Cli;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_radarr_command_from() {
    let command = RadarrCommand::TestAllIndexers;

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Radarr(command));
  }

  mod cli {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    fn test_commands_that_have_no_arg_requirements(
      #[values("clear-blocklist", "test-all-indexers")] subcommand: &str,
    ) {
      let result = Cli::command().try_get_matches_from(["managarr", "radarr", subcommand]);

      assert!(result.is_ok());
    }

    #[rstest]
    fn test_download_release_requires_movie_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "radarr",
        "download-release",
        "--indexer-id",
        "1",
        "--guid",
        "1",
      ]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[rstest]
    fn test_download_release_requires_guid() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "radarr",
        "download-release",
        "--indexer-id",
        "1",
        "--movie-id",
        "1",
      ]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[rstest]
    fn test_download_release_requires_indexer_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "radarr",
        "download-release",
        "--guid",
        "1",
        "--movie-id",
        "1",
      ]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_download_release_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "radarr",
        "download-release",
        "--guid",
        "1",
        "--movie-id",
        "1",
        "--indexer-id",
        "1",
      ]);

      assert!(result.is_ok());
    }

    #[rstest]
    fn test_manual_search_requires_movie_id() {
      let result = Cli::command().try_get_matches_from(["managarr", "radarr", "manual-search"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_manual_search_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "radarr",
        "manual-search",
        "--movie-id",
        "1",
      ]);

      assert!(result.is_ok());
    }

    #[test]
    fn test_search_new_movie_requires_query() {
      let result = Cli::command().try_get_matches_from(["managarr", "radarr", "search-new-movie"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_search_new_movie_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "radarr",
        "search-new-movie",
        "--query",
        "halo",
      ]);

      assert!(result.is_ok());
    }

    #[test]
    fn test_start_task_requires_task_name() {
      let result = Cli::command().try_get_matches_from(["managarr", "radarr", "start-task"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[rstest]
    fn test_start_task_task_name_validation() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "radarr",
        "start-task",
        "--task-name",
        "test",
      ]);

      assert!(result.is_err());
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_start_task_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "radarr",
        "start-task",
        "--task-name",
        "application-check-update",
      ]);

      assert!(result.is_ok());
    }

    #[rstest]
    fn test_test_indexer_requires_indexer_id() {
      let result = Cli::command().try_get_matches_from(["managarr", "radarr", "test-indexer"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_test_indexer_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "radarr",
        "test-indexer",
        "--indexer-id",
        "1",
      ]);

      assert!(result.is_ok());
    }

    #[rstest]
    fn test_trigger_automatic_search_requires_movie_id() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "radarr", "trigger-automatic-search"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_trigger_automatic_search_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "radarr",
        "trigger-automatic-search",
        "--movie-id",
        "1",
      ]);

      assert!(result.is_ok());
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
        radarr::{
          add_command_handler::RadarrAddCommand, delete_command_handler::RadarrDeleteCommand,
          edit_command_handler::RadarrEditCommand, get_command_handler::RadarrGetCommand,
          list_command_handler::RadarrListCommand, refresh_command_handler::RadarrRefreshCommand,
          RadarrCliHandler, RadarrCommand,
        },
        CliCommandHandler,
      },
      models::{
        radarr_models::{
          BlocklistItem, BlocklistResponse, IndexerSettings, RadarrReleaseDownloadBody,
          RadarrSerdeable, RadarrTaskName,
        },
        Serdeable,
      },
      network::{radarr_network::RadarrEvent, MockNetworkTrait, NetworkEvent},
    };

    #[tokio::test]
    async fn test_handle_clear_blocklist_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(RadarrEvent::GetBlocklist.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::BlocklistResponse(
            BlocklistResponse {
              records: vec![BlocklistItem::default()],
            },
          )))
        });
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(RadarrEvent::ClearBlocklist.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let claer_blocklist_command = RadarrCommand::ClearBlocklist;

      let result = RadarrCliHandler::with(&app_arc, claer_blocklist_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_download_release_command() {
      let expected_release_download_body = RadarrReleaseDownloadBody {
        guid: "guid".to_owned(),
        indexer_id: 1,
        movie_id: 1,
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::DownloadRelease(Some(expected_release_download_body)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let download_release_command = RadarrCommand::DownloadRelease {
        guid: "guid".to_owned(),
        indexer_id: 1,
        movie_id: 1,
      };

      let result = RadarrCliHandler::with(&app_arc, download_release_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_manual_search_command() {
      let expected_movie_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::GetReleases(Some(expected_movie_id)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let manual_search_command = RadarrCommand::ManualSearch { movie_id: 1 };

      let result = RadarrCliHandler::with(&app_arc, manual_search_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_search_new_movie_command() {
      let expected_search_query = "halo".to_owned();
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::SearchNewMovie(Some(expected_search_query)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let search_new_movie_command = RadarrCommand::SearchNewMovie {
        query: "halo".to_owned(),
      };

      let result = RadarrCliHandler::with(&app_arc, search_new_movie_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_start_task_command() {
      let expected_task_name = RadarrTaskName::ApplicationCheckUpdate;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::StartTask(Some(expected_task_name)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let start_task_command = RadarrCommand::StartTask {
        task_name: RadarrTaskName::ApplicationCheckUpdate,
      };

      let result = RadarrCliHandler::with(&app_arc, start_task_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_test_indexer_command() {
      let expected_indexer_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::TestIndexer(Some(expected_indexer_id)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let test_indexer_command = RadarrCommand::TestIndexer { indexer_id: 1 };

      let result = RadarrCliHandler::with(&app_arc, test_indexer_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_test_all_indexers_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(RadarrEvent::TestAllIndexers.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let test_all_indexers_command = RadarrCommand::TestAllIndexers;

      let result = RadarrCliHandler::with(&app_arc, test_all_indexers_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_trigger_automatic_search_command() {
      let expected_movie_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::TriggerAutomaticSearch(Some(expected_movie_id)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let trigger_automatic_search_command = RadarrCommand::TriggerAutomaticSearch { movie_id: 1 };

      let result = RadarrCliHandler::with(
        &app_arc,
        trigger_automatic_search_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_radarr_cli_handler_delegates_add_commands_to_the_add_command_handler() {
      let expected_tag_name = "test".to_owned();
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::AddTag(expected_tag_name.clone()).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let add_tag_command = RadarrCommand::Add(RadarrAddCommand::Tag {
        name: expected_tag_name,
      });

      let result = RadarrCliHandler::with(&app_arc, add_tag_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_radarr_cli_handler_delegates_delete_commands_to_the_delete_command_handler() {
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
      let delete_blocklist_item_command =
        RadarrCommand::Delete(RadarrDeleteCommand::BlocklistItem {
          blocklist_item_id: 1,
        });

      let result =
        RadarrCliHandler::with(&app_arc, delete_blocklist_item_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_radarr_cli_handler_delegates_edit_commands_to_the_edit_command_handler() {
      let expected_edit_all_indexer_settings = IndexerSettings {
        allow_hardcoded_subs: true,
        availability_delay: 1,
        id: 1,
        maximum_size: 1,
        minimum_age: 1,
        prefer_indexer_flags: true,
        retention: 1,
        rss_sync_interval: 1,
        whitelisted_hardcoded_subs: "test".into(),
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::GetAllIndexerSettings.into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::IndexerSettings(
            IndexerSettings {
              allow_hardcoded_subs: false,
              availability_delay: 2,
              id: 1,
              maximum_size: 2,
              minimum_age: 2,
              prefer_indexer_flags: false,
              retention: 2,
              rss_sync_interval: 2,
              whitelisted_hardcoded_subs: "testing".into(),
            },
          )))
        });
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::EditAllIndexerSettings(Some(expected_edit_all_indexer_settings)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let edit_all_indexer_settings_command =
        RadarrCommand::Edit(RadarrEditCommand::AllIndexerSettings {
          allow_hardcoded_subs: true,
          disable_allow_hardcoded_subs: false,
          availability_delay: Some(1),
          maximum_size: Some(1),
          minimum_age: Some(1),
          prefer_indexer_flags: true,
          disable_prefer_indexer_flags: false,
          retention: Some(1),
          rss_sync_interval: Some(1),
          whitelisted_subtitle_tags: Some("test".to_owned()),
        });

      let result = RadarrCliHandler::with(
        &app_arc,
        edit_all_indexer_settings_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_radarr_cli_handler_delegates_get_commands_to_the_get_command_handler() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::GetAllIndexerSettings.into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let get_all_indexer_settings_command =
        RadarrCommand::Get(RadarrGetCommand::AllIndexerSettings);

      let result = RadarrCliHandler::with(
        &app_arc,
        get_all_indexer_settings_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_radarr_cli_handler_delegates_list_commands_to_the_list_command_handler() {
      let expected_movie_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::GetMovieCredits(Some(expected_movie_id)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let list_movie_credits_command =
        RadarrCommand::List(RadarrListCommand::MovieCredits { movie_id: 1 });

      let result = RadarrCliHandler::with(&app_arc, list_movie_credits_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_radarr_cli_handler_delegates_refresh_commands_to_the_refresh_command_handler() {
      let expected_movie_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::UpdateAndScan(Some(expected_movie_id)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let refresh_movie_command =
        RadarrCommand::Refresh(RadarrRefreshCommand::Movie { movie_id: 1 });

      let result = RadarrCliHandler::with(&app_arc, refresh_movie_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }
  }
}
