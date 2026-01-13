#[cfg(test)]
mod tests {
  use clap::{CommandFactory, Parser, error::ErrorKind};

  use crate::{
    Cli,
    cli::{
      Command,
      lidarr::{LidarrCommand, add_command_handler::LidarrAddCommand},
    },
    models::lidarr_models::{MonitorType, NewItemMonitorType},
  };
  use pretty_assertions::assert_eq;

  #[test]
  fn test_lidarr_add_command_from() {
    let command = LidarrAddCommand::Tag {
      name: String::new(),
    };

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Lidarr(LidarrCommand::Add(command)));
  }

  mod cli {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_add_root_folder_requires_arguments() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "lidarr", "add", "root-folder"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_add_root_folder_success() {
      let expected_args = LidarrAddCommand::RootFolder {
        root_folder_path: "/nfs/test".to_owned(),
      };

      let result = Cli::try_parse_from([
        "managarr",
        "lidarr",
        "add",
        "root-folder",
        "--root-folder-path",
        "/nfs/test",
      ]);

      assert_ok!(&result);

      let Some(Command::Lidarr(LidarrCommand::Add(add_command))) = result.unwrap().command else {
        panic!("Unexpected command type");
      };
      assert_eq!(add_command, expected_args);
    }

    #[test]
    fn test_add_tag_requires_arguments() {
      let result = Cli::command().try_get_matches_from(["managarr", "lidarr", "add", "tag"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_add_tag_success() {
      let expected_args = LidarrAddCommand::Tag {
        name: "test".to_owned(),
      };

      let result = Cli::try_parse_from(["managarr", "lidarr", "add", "tag", "--name", "test"]);

      assert_ok!(&result);

      let Some(Command::Lidarr(LidarrCommand::Add(add_command))) = result.unwrap().command else {
        panic!("Unexpected command type")
      };
      assert_eq!(add_command, expected_args);
    }

    #[test]
    fn test_add_artist_requires_arguments() {
      let result = Cli::command().try_get_matches_from(["managarr", "lidarr", "add", "artist"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_add_artist_requires_foreign_artist_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "lidarr",
        "add",
        "artist",
        "--artist-name",
        "Test",
        "--root-folder-path",
        "/music",
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
    fn test_add_artist_requires_artist_name() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "lidarr",
        "add",
        "artist",
        "--foreign-artist-id",
        "test-id",
        "--root-folder-path",
        "/music",
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
    fn test_add_artist_requires_root_folder_path() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "lidarr",
        "add",
        "artist",
        "--foreign-artist-id",
        "test-id",
        "--artist-name",
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
    fn test_add_artist_requires_quality_profile_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "lidarr",
        "add",
        "artist",
        "--foreign-artist-id",
        "test-id",
        "--artist-name",
        "Test",
        "--root-folder-path",
        "/music",
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
    fn test_add_artist_requires_metadata_profile_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "lidarr",
        "add",
        "artist",
        "--foreign-artist-id",
        "test-id",
        "--artist-name",
        "Test",
        "--root-folder-path",
        "/music",
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
    fn test_add_artist_success_with_required_args_only() {
      let expected_args = LidarrAddCommand::Artist {
        foreign_artist_id: "test-id".to_owned(),
        artist_name: "Test Artist".to_owned(),
        root_folder_path: "/music".to_owned(),
        quality_profile_id: 1,
        metadata_profile_id: 1,
        disable_monitoring: false,
        tag: vec![],
        monitor: MonitorType::default(),
        monitor_new_items: NewItemMonitorType::default(),
        no_search_for_missing_albums: false,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "lidarr",
        "add",
        "artist",
        "--foreign-artist-id",
        "test-id",
        "--artist-name",
        "Test Artist",
        "--root-folder-path",
        "/music",
        "--quality-profile-id",
        "1",
        "--metadata-profile-id",
        "1",
      ]);

      assert_ok!(&result);

      let Some(Command::Lidarr(LidarrCommand::Add(add_command))) = result.unwrap().command else {
        panic!("Unexpected command type")
      };
      assert_eq!(add_command, expected_args);
    }

    #[test]
    fn test_add_artist_success_with_all_args() {
      let expected_args = LidarrAddCommand::Artist {
        foreign_artist_id: "test-id".to_owned(),
        artist_name: "Test Artist".to_owned(),
        root_folder_path: "/music".to_owned(),
        quality_profile_id: 1,
        metadata_profile_id: 2,
        disable_monitoring: true,
        tag: vec![1, 2],
        monitor: MonitorType::Future,
        monitor_new_items: NewItemMonitorType::New,
        no_search_for_missing_albums: true,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "lidarr",
        "add",
        "artist",
        "--foreign-artist-id",
        "test-id",
        "--artist-name",
        "Test Artist",
        "--root-folder-path",
        "/music",
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
        "--no-search-for-missing-albums",
      ]);

      assert_ok!(&result);

      let Some(Command::Lidarr(LidarrCommand::Add(add_command))) = result.unwrap().command else {
        panic!("Unexpected command type")
      };
      assert_eq!(add_command, expected_args);
    }

    #[test]
    fn test_add_artist_monitor_type_validation() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "lidarr",
        "add",
        "artist",
        "--foreign-artist-id",
        "test-id",
        "--artist-name",
        "Test Artist",
        "--root-folder-path",
        "/music",
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
    fn test_add_artist_new_item_monitor_type_validation() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "lidarr",
        "add",
        "artist",
        "--foreign-artist-id",
        "test-id",
        "--artist-name",
        "Test Artist",
        "--root-folder-path",
        "/music",
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
    fn test_add_artist_tags_is_repeatable() {
      let expected_args = LidarrAddCommand::Artist {
        foreign_artist_id: "test-id".to_owned(),
        artist_name: "Test Artist".to_owned(),
        root_folder_path: "/music".to_owned(),
        quality_profile_id: 1,
        metadata_profile_id: 2,
        disable_monitoring: false,
        tag: vec![1, 2],
        monitor: MonitorType::default(),
        monitor_new_items: NewItemMonitorType::default(),
        no_search_for_missing_albums: false,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "lidarr",
        "add",
        "artist",
        "--foreign-artist-id",
        "test-id",
        "--artist-name",
        "Test Artist",
        "--root-folder-path",
        "/music",
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

      let Some(Command::Lidarr(LidarrCommand::Add(add_command))) = result.unwrap().command else {
        panic!("Unexpected command type")
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
    use crate::cli::lidarr::add_command_handler::{LidarrAddCommand, LidarrAddCommandHandler};
    use crate::models::Serdeable;
    use crate::models::lidarr_models::{
      AddArtistBody, AddArtistOptions, LidarrSerdeable, MonitorType, NewItemMonitorType,
    };
    use crate::models::servarr_models::AddRootFolderBody;
    use crate::network::lidarr_network::LidarrEvent;
    use crate::{
      app::App,
      network::{MockNetworkTrait, NetworkEvent},
    };

    #[tokio::test]
    async fn test_handle_add_root_folder_command() {
      let expected_root_folder_path = "/nfs/test".to_owned();
      let expected_add_root_folder_body = AddRootFolderBody {
        path: expected_root_folder_path.clone(),
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          LidarrEvent::AddRootFolder(expected_add_root_folder_body.clone()).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let add_root_folder_command = LidarrAddCommand::RootFolder {
        root_folder_path: expected_root_folder_path,
      };

      let result =
        LidarrAddCommandHandler::with(&app_arc, add_root_folder_command, &mut mock_network)
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
          LidarrEvent::AddTag(expected_tag_name.clone()).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let add_tag_command = LidarrAddCommand::Tag {
        name: expected_tag_name,
      };

      let result = LidarrAddCommandHandler::with(&app_arc, add_tag_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_add_artist_command() {
      let expected_body = AddArtistBody {
        foreign_artist_id: "test-id".to_owned(),
        artist_name: "Test Artist".to_owned(),
        monitored: false,
        root_folder_path: "/music".to_owned(),
        quality_profile_id: 1,
        metadata_profile_id: 1,
        tags: vec![1, 2],
        tag_input_string: None,
        add_options: AddArtistOptions {
          monitor: MonitorType::All,
          monitor_new_items: NewItemMonitorType::All,
          search_for_missing_albums: false,
        },
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          LidarrEvent::AddArtist(expected_body).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let add_artist_command = LidarrAddCommand::Artist {
        foreign_artist_id: "test-id".to_owned(),
        artist_name: "Test Artist".to_owned(),
        root_folder_path: "/music".to_owned(),
        quality_profile_id: 1,
        metadata_profile_id: 1,
        disable_monitoring: true,
        tag: vec![1, 2],
        monitor: MonitorType::All,
        monitor_new_items: NewItemMonitorType::All,
        no_search_for_missing_albums: true,
      };

      let result = LidarrAddCommandHandler::with(&app_arc, add_artist_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }
  }
}
