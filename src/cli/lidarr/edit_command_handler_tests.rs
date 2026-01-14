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
    fn test_edit_all_indexer_settings_requires_arguments() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "lidarr", "edit", "all-indexer-settings"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[rstest]
    fn test_edit_all_indexer_settings_assert_argument_flags_require_args(
      #[values(
        "--maximum-size",
        "--minimum-age",
        "--retention",
        "--rss-sync-interval"
      )]
      flag: &str,
    ) {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "lidarr",
        "edit",
        "all-indexer-settings",
        flag,
      ]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_edit_all_indexer_settings_only_requires_at_least_one_argument() {
      let expected_args = LidarrEditCommand::AllIndexerSettings {
        maximum_size: Some(1),
        minimum_age: None,
        retention: None,
        rss_sync_interval: None,
      };
      let result = Cli::try_parse_from([
        "managarr",
        "lidarr",
        "edit",
        "all-indexer-settings",
        "--maximum-size",
        "1",
      ]);

      assert_ok!(&result);

      let Some(Command::Lidarr(LidarrCommand::Edit(edit_command))) = result.unwrap().command else {
        panic!("Unexpected command type");
      };
      assert_eq!(edit_command, expected_args);
    }

    #[test]
    fn test_edit_all_indexer_settings_all_arguments_defined() {
      let expected_args = LidarrEditCommand::AllIndexerSettings {
        maximum_size: Some(1),
        minimum_age: Some(1),
        retention: Some(1),
        rss_sync_interval: Some(1),
      };
      let result = Cli::try_parse_from([
        "managarr",
        "lidarr",
        "edit",
        "all-indexer-settings",
        "--maximum-size",
        "1",
        "--minimum-age",
        "1",
        "--retention",
        "1",
        "--rss-sync-interval",
        "1",
      ]);

      assert_ok!(&result);

      let Some(Command::Lidarr(LidarrCommand::Edit(edit_command))) = result.unwrap().command else {
        panic!("Unexpected command type");
      };
      assert_eq!(edit_command, expected_args);
    }

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

    #[test]
    fn test_edit_indexer_requires_arguments() {
      let result = Cli::command().try_get_matches_from(["managarr", "lidarr", "edit", "indexer"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_edit_indexer_with_indexer_id_still_requires_arguments() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "lidarr",
        "edit",
        "indexer",
        "--indexer-id",
        "1",
      ]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_edit_indexer_rss_flags_conflict() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "lidarr",
        "edit",
        "indexer",
        "--indexer-id",
        "1",
        "--enable-rss",
        "--disable-rss",
      ]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::ArgumentConflict);
    }

    #[test]
    fn test_edit_indexer_automatic_search_flags_conflict() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "lidarr",
        "edit",
        "indexer",
        "--indexer-id",
        "1",
        "--enable-automatic-search",
        "--disable-automatic-search",
      ]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::ArgumentConflict);
    }

    #[test]
    fn test_edit_indexer_interactive_search_flags_conflict() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "lidarr",
        "edit",
        "indexer",
        "--indexer-id",
        "1",
        "--enable-interactive-search",
        "--disable-interactive-search",
      ]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::ArgumentConflict);
    }

    #[test]
    fn test_edit_indexer_tag_flags_conflict() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "lidarr",
        "edit",
        "indexer",
        "--indexer-id",
        "1",
        "--tag",
        "1",
        "--clear-tags",
      ]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::ArgumentConflict);
    }

    #[rstest]
    fn test_edit_indexer_assert_argument_flags_require_args(
      #[values("--name", "--url", "--api-key", "--seed-ratio", "--tag", "--priority")] flag: &str,
    ) {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "lidarr",
        "edit",
        "indexer",
        "--indexer-id",
        "1",
        flag,
      ]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_edit_indexer_only_requires_at_least_one_argument_plus_indexer_id() {
      let expected_args = LidarrEditCommand::Indexer {
        indexer_id: 1,
        name: Some("Test".to_owned()),
        enable_rss: false,
        disable_rss: false,
        enable_automatic_search: false,
        disable_automatic_search: false,
        enable_interactive_search: false,
        disable_interactive_search: false,
        url: None,
        api_key: None,
        seed_ratio: None,
        tag: None,
        priority: None,
        clear_tags: false,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "lidarr",
        "edit",
        "indexer",
        "--indexer-id",
        "1",
        "--name",
        "Test",
      ]);

      assert_ok!(&result);

      let Some(Command::Lidarr(LidarrCommand::Edit(edit_command))) = result.unwrap().command else {
        panic!("Unexpected command type");
      };
      assert_eq!(edit_command, expected_args);
    }

    #[test]
    fn test_edit_indexer_tag_argument_is_repeatable() {
      let expected_args = LidarrEditCommand::Indexer {
        indexer_id: 1,
        name: None,
        enable_rss: false,
        disable_rss: false,
        enable_automatic_search: false,
        disable_automatic_search: false,
        enable_interactive_search: false,
        disable_interactive_search: false,
        url: None,
        api_key: None,
        seed_ratio: None,
        tag: Some(vec![1, 2]),
        priority: None,
        clear_tags: false,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "lidarr",
        "edit",
        "indexer",
        "--indexer-id",
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
    fn test_edit_indexer_all_arguments_defined() {
      let expected_args = LidarrEditCommand::Indexer {
        indexer_id: 1,
        name: Some("Test".to_owned()),
        enable_rss: true,
        disable_rss: false,
        enable_automatic_search: true,
        disable_automatic_search: false,
        enable_interactive_search: true,
        disable_interactive_search: false,
        url: Some("http://test.com".to_owned()),
        api_key: Some("testKey".to_owned()),
        seed_ratio: Some("1.2".to_owned()),
        tag: Some(vec![1, 2]),
        priority: Some(25),
        clear_tags: false,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "lidarr",
        "edit",
        "indexer",
        "--indexer-id",
        "1",
        "--name",
        "Test",
        "--enable-rss",
        "--enable-automatic-search",
        "--enable-interactive-search",
        "--url",
        "http://test.com",
        "--api-key",
        "testKey",
        "--seed-ratio",
        "1.2",
        "--tag",
        "1",
        "--tag",
        "2",
        "--priority",
        "25",
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

    use crate::models::servarr_models::{EditIndexerParams, IndexerSettings};
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
    async fn test_handle_edit_all_indexer_settings_command() {
      let expected_edit_all_indexer_settings = IndexerSettings {
        id: 1,
        maximum_size: 1,
        minimum_age: 1,
        retention: 1,
        rss_sync_interval: 1,
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          LidarrEvent::GetAllIndexerSettings.into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::IndexerSettings(
            IndexerSettings {
              id: 1,
              maximum_size: 2,
              minimum_age: 2,
              retention: 2,
              rss_sync_interval: 2,
            },
          )))
        });
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          LidarrEvent::EditAllIndexerSettings(expected_edit_all_indexer_settings).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let edit_all_indexer_settings_command = LidarrEditCommand::AllIndexerSettings {
        maximum_size: Some(1),
        minimum_age: Some(1),
        retention: Some(1),
        rss_sync_interval: Some(1),
      };

      let result = LidarrEditCommandHandler::with(
        &app_arc,
        edit_all_indexer_settings_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert_ok!(&result);
    }

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

    #[tokio::test]
    async fn test_handle_edit_indexer_command() {
      let expected_edit_indexer_params = EditIndexerParams {
        indexer_id: 1,
        name: Some("Test".to_owned()),
        enable_rss: Some(true),
        enable_automatic_search: Some(true),
        enable_interactive_search: Some(true),
        url: Some("http://test.com".to_owned()),
        api_key: Some("testKey".to_owned()),
        seed_ratio: Some("1.2".to_owned()),
        tags: Some(vec![1, 2]),
        tag_input_string: None,
        priority: Some(25),
        clear_tags: false,
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          LidarrEvent::EditIndexer(expected_edit_indexer_params).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let edit_indexer_command = LidarrEditCommand::Indexer {
        indexer_id: 1,
        name: Some("Test".to_owned()),
        enable_rss: true,
        disable_rss: false,
        enable_automatic_search: true,
        disable_automatic_search: false,
        enable_interactive_search: true,
        disable_interactive_search: false,
        url: Some("http://test.com".to_owned()),
        api_key: Some("testKey".to_owned()),
        seed_ratio: Some("1.2".to_owned()),
        tag: Some(vec![1, 2]),
        priority: Some(25),
        clear_tags: false,
      };

      let result =
        LidarrEditCommandHandler::with(&app_arc, edit_indexer_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }
  }
}
