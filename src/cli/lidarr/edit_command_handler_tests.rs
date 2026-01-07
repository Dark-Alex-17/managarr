#[cfg(test)]
mod tests {
  use crate::cli::{
    Command,
    lidarr::{LidarrCommand, edit_command_handler::LidarrEditCommand},
  };

  #[test]
  fn test_lidarr_edit_command_from() {
    let command = LidarrEditCommand::Artist {
      artist_id: 1,
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

    assert_eq!(result, Command::Lidarr(LidarrCommand::Edit(command)));
  }

  mod cli {
    use crate::{Cli, models::lidarr_models::NewItemMonitorType};

    use super::*;
    use clap::{CommandFactory, Parser, error::ErrorKind};
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn test_edit_artist_requires_arguments() {
      let result = Cli::command().try_get_matches_from(["managarr", "lidarr", "edit", "artist"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_edit_artist_with_artist_id_still_requires_arguments() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "lidarr",
        "edit",
        "artist",
        "--artist-id",
        "1",
      ]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_edit_artist_monitoring_flags_conflict() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "lidarr",
        "edit",
        "artist",
        "--artist-id",
        "1",
        "--enable-monitoring",
        "--disable-monitoring",
      ]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::ArgumentConflict);
    }

    #[test]
    fn test_edit_artist_tag_flags_conflict() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "lidarr",
        "edit",
        "artist",
        "--artist-id",
        "1",
        "--tag",
        "1",
        "--clear-tags",
      ]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::ArgumentConflict);
    }

    #[rstest]
    fn test_edit_artist_assert_argument_flags_require_args(
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
        "lidarr",
        "edit",
        "artist",
        "--artist-id",
        "1",
        flag,
      ]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_edit_artist_monitor_new_items_validation() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "lidarr",
        "edit",
        "artist",
        "--artist-id",
        "1",
        "--monitor-new-items",
        "test",
      ]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_edit_artist_only_requires_at_least_one_argument_plus_artist_id() {
      let expected_args = LidarrEditCommand::Artist {
        artist_id: 1,
        enable_monitoring: false,
        disable_monitoring: false,
        monitor_new_items: None,
        quality_profile_id: None,
        metadata_profile_id: None,
        root_folder_path: Some("/nfs/test".to_owned()),
        tag: None,
        clear_tags: false,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "lidarr",
        "edit",
        "artist",
        "--artist-id",
        "1",
        "--root-folder-path",
        "/nfs/test",
      ]);

      assert_ok!(&result);

      let Some(Command::Lidarr(LidarrCommand::Edit(edit_command))) = result.unwrap().command else {
        panic!("Unexpected command type");
      };
      assert_eq!(edit_command, expected_args);
    }

    #[test]
    fn test_edit_artist_tag_argument_is_repeatable() {
      let expected_args = LidarrEditCommand::Artist {
        artist_id: 1,
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
        "lidarr",
        "edit",
        "artist",
        "--artist-id",
        "1",
        "--tag",
        "1",
        "--tag",
        "2",
      ]);

      assert_ok!(&result);

      let Some(Command::Lidarr(LidarrCommand::Edit(edit_command))) = result.unwrap().command else {
        panic!("Unexpected command type");
      };
      assert_eq!(edit_command, expected_args);
    }

    #[test]
    fn test_edit_artist_all_arguments_defined() {
      let expected_args = LidarrEditCommand::Artist {
        artist_id: 1,
        enable_monitoring: true,
        disable_monitoring: false,
        monitor_new_items: Some(NewItemMonitorType::New),
        quality_profile_id: Some(1),
        metadata_profile_id: Some(1),
        root_folder_path: Some("/nfs/test".to_owned()),
        tag: Some(vec![1, 2]),
        clear_tags: false,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "lidarr",
        "edit",
        "artist",
        "--artist-id",
        "1",
        "--enable-monitoring",
        "--monitor-new-items",
        "new",
        "--quality-profile-id",
        "1",
        "--metadata-profile-id",
        "1",
        "--root-folder-path",
        "/nfs/test",
        "--tag",
        "1",
        "--tag",
        "2",
      ]);

      assert_ok!(&result);

      let Some(Command::Lidarr(LidarrCommand::Edit(edit_command))) = result.unwrap().command else {
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

    use crate::{
      app::App,
      cli::{
        CliCommandHandler,
        lidarr::edit_command_handler::{LidarrEditCommand, LidarrEditCommandHandler},
      },
      models::{
        Serdeable,
        lidarr_models::{EditArtistParams, LidarrSerdeable, NewItemMonitorType},
      },
      network::{MockNetworkTrait, NetworkEvent, lidarr_network::LidarrEvent},
    };

    #[tokio::test]
    async fn test_handle_edit_artist_command() {
      let expected_edit_artist_params = EditArtistParams {
        artist_id: 1,
        monitored: Some(true),
        monitor_new_items: Some(NewItemMonitorType::New),
        quality_profile_id: Some(1),
        metadata_profile_id: Some(1),
        root_folder_path: Some("/nfs/test".to_owned()),
        tags: Some(vec![1, 2]),
        tag_input_string: None,
        clear_tags: false,
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          LidarrEvent::EditArtist(expected_edit_artist_params).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let edit_artist_command = LidarrEditCommand::Artist {
        artist_id: 1,
        enable_monitoring: true,
        disable_monitoring: false,
        monitor_new_items: Some(NewItemMonitorType::New),
        quality_profile_id: Some(1),
        metadata_profile_id: Some(1),
        root_folder_path: Some("/nfs/test".to_owned()),
        tag: Some(vec![1, 2]),
        clear_tags: false,
      };

      let result = LidarrEditCommandHandler::with(&app_arc, edit_artist_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_edit_artist_command_handles_disable_monitoring_flag_properly() {
      let expected_edit_artist_params = EditArtistParams {
        artist_id: 1,
        monitored: Some(false),
        monitor_new_items: Some(NewItemMonitorType::None),
        quality_profile_id: Some(1),
        metadata_profile_id: Some(1),
        root_folder_path: Some("/nfs/test".to_owned()),
        tags: Some(vec![1, 2]),
        tag_input_string: None,
        clear_tags: false,
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          LidarrEvent::EditArtist(expected_edit_artist_params).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let edit_artist_command = LidarrEditCommand::Artist {
        artist_id: 1,
        enable_monitoring: false,
        disable_monitoring: true,
        monitor_new_items: Some(NewItemMonitorType::None),
        quality_profile_id: Some(1),
        metadata_profile_id: Some(1),
        root_folder_path: Some("/nfs/test".to_owned()),
        tag: Some(vec![1, 2]),
        clear_tags: false,
      };

      let result = LidarrEditCommandHandler::with(&app_arc, edit_artist_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_edit_artist_command_no_monitoring_boolean_flags_returns_none_value() {
      let expected_edit_artist_params = EditArtistParams {
        artist_id: 1,
        monitored: None,
        monitor_new_items: Some(NewItemMonitorType::All),
        quality_profile_id: Some(1),
        metadata_profile_id: Some(1),
        root_folder_path: Some("/nfs/test".to_owned()),
        tags: Some(vec![1, 2]),
        tag_input_string: None,
        clear_tags: false,
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          LidarrEvent::EditArtist(expected_edit_artist_params).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let edit_artist_command = LidarrEditCommand::Artist {
        artist_id: 1,
        enable_monitoring: false,
        disable_monitoring: false,
        monitor_new_items: Some(NewItemMonitorType::All),
        quality_profile_id: Some(1),
        metadata_profile_id: Some(1),
        root_folder_path: Some("/nfs/test".to_owned()),
        tag: Some(vec![1, 2]),
        clear_tags: false,
      };

      let result = LidarrEditCommandHandler::with(&app_arc, edit_artist_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }
  }
}
