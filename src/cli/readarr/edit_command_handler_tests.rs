#[cfg(test)]
mod tests {
  use clap::{CommandFactory, Parser, error::ErrorKind};

  use crate::{
    Cli,
    cli::{
      Command,
      readarr::{ReadarrCommand, edit_command_handler::ReadarrEditCommand},
    },
  };
  use pretty_assertions::assert_eq;

  #[test]
  fn test_readarr_edit_command_from() {
    let command = ReadarrEditCommand::Author {
      author_id: 1,
      enable_monitoring: false,
      disable_monitoring: false,
      monitor_new_items: None,
      quality_profile_id: None,
      metadata_profile_id: None,
      root_folder_path: None,
      tag: None,
      clear_tags: false,
    };

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Readarr(ReadarrCommand::Edit(command)));
  }

  mod cli {
    use super::*;
    use crate::models::readarr_models::NewItemMonitorType;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn test_edit_author_requires_arguments() {
      let result = Cli::command().try_get_matches_from(["managarr", "readarr", "edit", "author"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_edit_author_with_author_id_still_requires_arguments() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "edit",
        "author",
        "--author-id",
        "1",
      ]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_edit_author_author_id_requires_a_number() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "edit",
        "author",
        "--author-id",
        "not_a_number",
        "--enable-monitoring",
      ]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::ValueValidation);
    }

    #[test]
    fn test_edit_author_monitoring_flags_conflict() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "edit",
        "author",
        "--author-id",
        "1",
        "--enable-monitoring",
        "--disable-monitoring",
      ]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::ArgumentConflict);
    }

    #[test]
    fn test_edit_author_tag_flags_conflict() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "edit",
        "author",
        "--author-id",
        "1",
        "--tag",
        "1",
        "--clear-tags",
      ]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::ArgumentConflict);
    }

    #[rstest]
    fn test_edit_author_assert_argument_flags_require_args(
      #[values(
        "--monitor-new-items",
        "--quality-profile-id",
        "--metadata-profile-id",
        "--root-folder-path",
        "--tag"
      )]
      flag: &str,
    ) {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "edit",
        "author",
        "--author-id",
        "1",
        flag,
      ]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_edit_author_monitor_new_items_validation() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "edit",
        "author",
        "--author-id",
        "1",
        "--monitor-new-items",
        "test",
      ]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_edit_author_only_requires_at_least_one_argument_plus_author_id() {
      let expected_args = ReadarrEditCommand::Author {
        author_id: 1,
        enable_monitoring: false,
        disable_monitoring: false,
        monitor_new_items: None,
        quality_profile_id: None,
        metadata_profile_id: None,
        root_folder_path: Some("/nfs/books".to_owned()),
        tag: None,
        clear_tags: false,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "readarr",
        "edit",
        "author",
        "--author-id",
        "1",
        "--root-folder-path",
        "/nfs/books",
      ]);

      assert_ok!(&result);

      let Some(Command::Readarr(ReadarrCommand::Edit(edit_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(edit_command, expected_args);
    }

    #[test]
    fn test_edit_author_tag_argument_is_repeatable() {
      let expected_args = ReadarrEditCommand::Author {
        author_id: 1,
        enable_monitoring: false,
        disable_monitoring: false,
        monitor_new_items: None,
        quality_profile_id: None,
        metadata_profile_id: None,
        root_folder_path: None,
        tag: Some(vec![1, 2]),
        clear_tags: false,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "readarr",
        "edit",
        "author",
        "--author-id",
        "1",
        "--tag",
        "1",
        "--tag",
        "2",
      ]);

      assert_ok!(&result);

      let Some(Command::Readarr(ReadarrCommand::Edit(edit_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(edit_command, expected_args);
    }

    #[test]
    fn test_edit_author_all_arguments_defined() {
      let expected_args = ReadarrEditCommand::Author {
        author_id: 1,
        enable_monitoring: true,
        disable_monitoring: false,
        monitor_new_items: Some(NewItemMonitorType::New),
        quality_profile_id: Some(1),
        metadata_profile_id: Some(1),
        root_folder_path: Some("/nfs/books".to_owned()),
        tag: Some(vec![1, 2]),
        clear_tags: false,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "readarr",
        "edit",
        "author",
        "--author-id",
        "1",
        "--enable-monitoring",
        "--monitor-new-items",
        "new",
        "--quality-profile-id",
        "1",
        "--metadata-profile-id",
        "1",
        "--root-folder-path",
        "/nfs/books",
        "--tag",
        "1",
        "--tag",
        "2",
      ]);

      assert_ok!(&result);

      let Some(Command::Readarr(ReadarrCommand::Edit(edit_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(edit_command, expected_args);
    }
  }

  mod handler {
    use std::sync::Arc;

    use mockall::predicate::eq;
    use serde_json::json;
    use tokio::sync::Mutex;

    use crate::cli::CliCommandHandler;
    use crate::cli::readarr::edit_command_handler::{
      ReadarrEditCommand, ReadarrEditCommandHandler,
    };
    use crate::models::Serdeable;
    use crate::models::readarr_models::{EditAuthorParams, NewItemMonitorType, ReadarrSerdeable};
    use crate::network::readarr_network::ReadarrEvent;
    use crate::{
      app::App,
      network::{MockNetworkTrait, NetworkEvent},
    };

    #[tokio::test]
    async fn test_handle_edit_author_command() {
      let expected_edit_author_params = EditAuthorParams {
        author_id: 1,
        monitored: Some(true),
        monitor_new_items: Some(NewItemMonitorType::New),
        quality_profile_id: Some(1),
        metadata_profile_id: Some(1),
        root_folder_path: Some("/nfs/books".to_owned()),
        tags: Some(vec![1, 2]),
        tag_input_string: None,
        clear_tags: false,
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::EditAuthor(expected_edit_author_params).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let edit_author_command = ReadarrEditCommand::Author {
        author_id: 1,
        enable_monitoring: true,
        disable_monitoring: false,
        monitor_new_items: Some(NewItemMonitorType::New),
        quality_profile_id: Some(1),
        metadata_profile_id: Some(1),
        root_folder_path: Some("/nfs/books".to_owned()),
        tag: Some(vec![1, 2]),
        clear_tags: false,
      };

      let result =
        ReadarrEditCommandHandler::with(&app_arc, edit_author_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_edit_author_command_handles_disable_monitoring_flag_properly() {
      let expected_edit_author_params = EditAuthorParams {
        author_id: 1,
        monitored: Some(false),
        monitor_new_items: Some(NewItemMonitorType::None),
        quality_profile_id: Some(1),
        metadata_profile_id: Some(1),
        root_folder_path: Some("/nfs/books".to_owned()),
        tags: Some(vec![1, 2]),
        tag_input_string: None,
        clear_tags: false,
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::EditAuthor(expected_edit_author_params).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let edit_author_command = ReadarrEditCommand::Author {
        author_id: 1,
        enable_monitoring: false,
        disable_monitoring: true,
        monitor_new_items: Some(NewItemMonitorType::None),
        quality_profile_id: Some(1),
        metadata_profile_id: Some(1),
        root_folder_path: Some("/nfs/books".to_owned()),
        tag: Some(vec![1, 2]),
        clear_tags: false,
      };

      let result =
        ReadarrEditCommandHandler::with(&app_arc, edit_author_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_edit_author_command_no_monitoring_boolean_flags_returns_none_value() {
      let expected_edit_author_params = EditAuthorParams {
        author_id: 1,
        monitored: None,
        monitor_new_items: Some(NewItemMonitorType::All),
        quality_profile_id: Some(1),
        metadata_profile_id: Some(1),
        root_folder_path: Some("/nfs/books".to_owned()),
        tags: Some(vec![1, 2]),
        tag_input_string: None,
        clear_tags: false,
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::EditAuthor(expected_edit_author_params).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let edit_author_command = ReadarrEditCommand::Author {
        author_id: 1,
        enable_monitoring: false,
        disable_monitoring: false,
        monitor_new_items: Some(NewItemMonitorType::All),
        quality_profile_id: Some(1),
        metadata_profile_id: Some(1),
        root_folder_path: Some("/nfs/books".to_owned()),
        tag: Some(vec![1, 2]),
        clear_tags: false,
      };

      let result =
        ReadarrEditCommandHandler::with(&app_arc, edit_author_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_edit_author_command_clear_tags() {
      let expected_edit_author_params = EditAuthorParams {
        author_id: 1,
        monitored: None,
        monitor_new_items: None,
        quality_profile_id: None,
        metadata_profile_id: None,
        root_folder_path: None,
        tags: None,
        tag_input_string: None,
        clear_tags: true,
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::EditAuthor(expected_edit_author_params).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let edit_author_command = ReadarrEditCommand::Author {
        author_id: 1,
        enable_monitoring: false,
        disable_monitoring: false,
        monitor_new_items: None,
        quality_profile_id: None,
        metadata_profile_id: None,
        root_folder_path: None,
        tag: None,
        clear_tags: true,
      };

      let result =
        ReadarrEditCommandHandler::with(&app_arc, edit_author_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }
  }
}
