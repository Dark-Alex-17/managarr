#[cfg(test)]
mod tests {
  use clap::{CommandFactory, Parser, error::ErrorKind};

  use crate::{
    Cli,
    cli::{
      Command,
      readarr::{ReadarrCommand, add_command_handler::ReadarrAddCommand},
    },
    models::readarr_models::{MonitorType, NewItemMonitorType},
  };
  use pretty_assertions::assert_eq;

  #[test]
  fn test_readarr_add_command_from() {
    let command = ReadarrAddCommand::Tag {
      name: String::new(),
    };

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Readarr(ReadarrCommand::Add(command)));
  }

  mod cli {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_add_author_requires_arguments() {
      let result = Cli::command().try_get_matches_from(["managarr", "readarr", "add", "author"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_add_author_requires_foreign_author_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "add",
        "author",
        "--author-name",
        "Test",
        "--root-folder-path",
        "/nfs/books",
        "--quality-profile-id",
        "1",
        "--metadata-profile-id",
        "1",
      ]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_add_author_requires_author_name() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "add",
        "author",
        "--foreign-author-id",
        "test-id",
        "--root-folder-path",
        "/nfs/books",
        "--quality-profile-id",
        "1",
        "--metadata-profile-id",
        "1",
      ]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_add_author_requires_root_folder_path() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "add",
        "author",
        "--foreign-author-id",
        "test-id",
        "--author-name",
        "Test",
        "--quality-profile-id",
        "1",
        "--metadata-profile-id",
        "1",
      ]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_add_author_requires_quality_profile_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "add",
        "author",
        "--foreign-author-id",
        "test-id",
        "--author-name",
        "Test",
        "--root-folder-path",
        "/nfs/books",
        "--metadata-profile-id",
        "1",
      ]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_add_author_requires_metadata_profile_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "add",
        "author",
        "--foreign-author-id",
        "test-id",
        "--author-name",
        "Test",
        "--root-folder-path",
        "/nfs/books",
        "--quality-profile-id",
        "1",
      ]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_add_author_success_with_required_args_only() {
      let expected_args = ReadarrAddCommand::Author {
        foreign_author_id: "test-id".to_owned(),
        author_name: "Test Author".to_owned(),
        root_folder_path: "/nfs/books".to_owned(),
        quality_profile_id: 1,
        metadata_profile_id: 1,
        disable_monitoring: false,
        tag: vec![],
        monitor: MonitorType::default(),
        monitor_new_items: NewItemMonitorType::default(),
        no_search_for_missing_books: false,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "readarr",
        "add",
        "author",
        "--foreign-author-id",
        "test-id",
        "--author-name",
        "Test Author",
        "--root-folder-path",
        "/nfs/books",
        "--quality-profile-id",
        "1",
        "--metadata-profile-id",
        "1",
      ]);

      assert_ok!(&result);

      let Some(Command::Readarr(ReadarrCommand::Add(add_command))) = result.unwrap().command else {
        panic!("Unexpected command type")
      };
      assert_eq!(add_command, expected_args);
    }

    #[test]
    fn test_add_author_success_with_all_args() {
      let expected_args = ReadarrAddCommand::Author {
        foreign_author_id: "test-id".to_owned(),
        author_name: "Test Author".to_owned(),
        root_folder_path: "/nfs/books".to_owned(),
        quality_profile_id: 1,
        metadata_profile_id: 2,
        disable_monitoring: true,
        tag: vec![1, 2],
        monitor: MonitorType::Future,
        monitor_new_items: NewItemMonitorType::New,
        no_search_for_missing_books: true,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "readarr",
        "add",
        "author",
        "--foreign-author-id",
        "test-id",
        "--author-name",
        "Test Author",
        "--root-folder-path",
        "/nfs/books",
        "--quality-profile-id",
        "1",
        "--metadata-profile-id",
        "2",
        "--disable-monitoring",
        "--tag",
        "1",
        "--tag",
        "2",
        "--monitor",
        "future",
        "--monitor-new-items",
        "new",
        "--no-search-for-missing-books",
      ]);

      assert_ok!(&result);

      let Some(Command::Readarr(ReadarrCommand::Add(add_command))) = result.unwrap().command else {
        panic!("Unexpected command type")
      };
      assert_eq!(add_command, expected_args);
    }

    #[test]
    fn test_add_author_monitor_type_validation() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "add",
        "author",
        "--foreign-author-id",
        "test-id",
        "--author-name",
        "Test Author",
        "--root-folder-path",
        "/nfs/books",
        "--quality-profile-id",
        "1",
        "--metadata-profile-id",
        "2",
        "--monitor",
        "test",
      ]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_add_author_new_item_monitor_type_validation() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "add",
        "author",
        "--foreign-author-id",
        "test-id",
        "--author-name",
        "Test Author",
        "--root-folder-path",
        "/nfs/books",
        "--quality-profile-id",
        "1",
        "--metadata-profile-id",
        "2",
        "--monitor-new-items",
        "test",
      ]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_add_author_tags_is_repeatable() {
      let expected_args = ReadarrAddCommand::Author {
        foreign_author_id: "test-id".to_owned(),
        author_name: "Test Author".to_owned(),
        root_folder_path: "/nfs/books".to_owned(),
        quality_profile_id: 1,
        metadata_profile_id: 2,
        disable_monitoring: false,
        tag: vec![1, 2],
        monitor: MonitorType::default(),
        monitor_new_items: NewItemMonitorType::default(),
        no_search_for_missing_books: false,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "readarr",
        "add",
        "author",
        "--foreign-author-id",
        "test-id",
        "--author-name",
        "Test Author",
        "--root-folder-path",
        "/nfs/books",
        "--quality-profile-id",
        "1",
        "--metadata-profile-id",
        "2",
        "--tag",
        "1",
        "--tag",
        "2",
      ]);

      assert_ok!(&result);

      let Some(Command::Readarr(ReadarrCommand::Add(add_command))) = result.unwrap().command else {
        panic!("Unexpected command type")
      };
      assert_eq!(add_command, expected_args);
    }

    #[test]
    fn test_add_root_folder_requires_arguments() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "readarr", "add", "root-folder"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_add_root_folder_success() {
      let expected_args = ReadarrAddCommand::RootFolder {
        name: "Books".to_owned(),
        root_folder_path: "/nfs/test".to_owned(),
        quality_profile_id: 1,
        metadata_profile_id: 1,
        monitor: MonitorType::All,
        monitor_new_items: NewItemMonitorType::All,
        tag: vec![],
      };

      let result = Cli::try_parse_from([
        "managarr",
        "readarr",
        "add",
        "root-folder",
        "--name",
        "Books",
        "--root-folder-path",
        "/nfs/test",
        "--quality-profile-id",
        "1",
        "--metadata-profile-id",
        "1",
      ]);

      assert_ok!(&result);

      let Some(Command::Readarr(ReadarrCommand::Add(add_command))) = result.unwrap().command else {
        panic!("Unexpected command type");
      };
      assert_eq!(add_command, expected_args);
    }

    #[test]
    fn test_add_tag_requires_arguments() {
      let result = Cli::command().try_get_matches_from(["managarr", "readarr", "add", "tag"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_add_tag_success() {
      let expected_args = ReadarrAddCommand::Tag {
        name: "test".to_owned(),
      };

      let result = Cli::try_parse_from(["managarr", "readarr", "add", "tag", "--name", "test"]);

      assert_ok!(&result);

      let Some(Command::Readarr(ReadarrCommand::Add(add_command))) = result.unwrap().command else {
        panic!("Unexpected command type");
      };
      assert_eq!(add_command, expected_args);
    }
  }

  mod handler {
    use std::sync::Arc;

    use mockall::predicate::eq;
    use serde_json::json;
    use tokio::sync::Mutex;

    use crate::cli::CliCommandHandler;
    use crate::cli::readarr::add_command_handler::{ReadarrAddCommand, ReadarrAddCommandHandler};
    use crate::models::Serdeable;
    use crate::models::readarr_models::{
      AddAuthorBody, AddAuthorOptions, AddReadarrRootFolderBody, MonitorType, NewItemMonitorType,
      ReadarrSerdeable,
    };
    use crate::network::readarr_network::ReadarrEvent;
    use crate::{
      app::App,
      network::{MockNetworkTrait, NetworkEvent},
    };

    #[tokio::test]
    async fn test_handle_add_author_command() {
      let expected_body = AddAuthorBody {
        foreign_author_id: "test-id".to_owned(),
        author_name: "Test Author".to_owned(),
        monitored: false,
        root_folder_path: "/nfs/books".to_owned(),
        quality_profile_id: 1,
        metadata_profile_id: 1,
        tags: vec![1, 2],
        tag_input_string: None,
        add_options: AddAuthorOptions {
          monitor: MonitorType::All,
          monitor_new_items: NewItemMonitorType::All,
          search_for_missing_books: false,
        },
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::AddAuthor(expected_body).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let add_author_command = ReadarrAddCommand::Author {
        foreign_author_id: "test-id".to_owned(),
        author_name: "Test Author".to_owned(),
        root_folder_path: "/nfs/books".to_owned(),
        quality_profile_id: 1,
        metadata_profile_id: 1,
        disable_monitoring: true,
        tag: vec![1, 2],
        monitor: MonitorType::All,
        monitor_new_items: NewItemMonitorType::All,
        no_search_for_missing_books: true,
      };

      let result = ReadarrAddCommandHandler::with(&app_arc, add_author_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_add_root_folder_command() {
      let expected_root_folder_path = "/nfs/test".to_owned();
      let expected_add_root_folder_body = AddReadarrRootFolderBody {
        name: "Books".to_owned(),
        path: expected_root_folder_path.clone(),
        default_quality_profile_id: 1,
        default_metadata_profile_id: 1,
        default_monitor_option: MonitorType::All,
        default_new_item_monitor_option: NewItemMonitorType::All,
        default_tags: vec![1, 2],
        tag_input_string: None,
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::AddRootFolder(expected_add_root_folder_body.clone()).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let add_root_folder_command = ReadarrAddCommand::RootFolder {
        name: "Books".to_owned(),
        root_folder_path: expected_root_folder_path,
        quality_profile_id: 1,
        metadata_profile_id: 1,
        monitor: MonitorType::All,
        monitor_new_items: NewItemMonitorType::All,
        tag: vec![1, 2],
      };

      let result =
        ReadarrAddCommandHandler::with(&app_arc, add_root_folder_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_add_tag_command() {
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
      let add_tag_command = ReadarrAddCommand::Tag {
        name: expected_tag_name,
      };

      let result = ReadarrAddCommandHandler::with(&app_arc, add_tag_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }
  }
}
