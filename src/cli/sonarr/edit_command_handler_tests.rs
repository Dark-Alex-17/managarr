#[cfg(test)]
mod tests {
  use crate::cli::{
    sonarr::{edit_command_handler::SonarrEditCommand, SonarrCommand},
    Command,
  };

  #[test]
  fn test_sonarr_edit_command_from() {
    let command = SonarrEditCommand::AllIndexerSettings {
      maximum_size: None,
      minimum_age: None,
      retention: None,
      rss_sync_interval: None,
    };

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Sonarr(SonarrCommand::Edit(command)));
  }

  mod cli {
    use crate::{models::sonarr_models::SeriesType, Cli};

    use super::*;
    use clap::{error::ErrorKind, CommandFactory, Parser};
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn test_edit_all_indexer_settings_requires_arguments() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "sonarr", "edit", "all-indexer-settings"]);

      assert!(result.is_err());
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
        "sonarr",
        "edit",
        "all-indexer-settings",
        flag,
      ]);

      assert!(result.is_err());
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_edit_all_indexer_settings_only_requires_at_least_one_argument() {
      let expected_args = SonarrEditCommand::AllIndexerSettings {
        maximum_size: Some(1),
        minimum_age: None,
        retention: None,
        rss_sync_interval: None,
      };
      let result = Cli::try_parse_from([
        "managarr",
        "sonarr",
        "edit",
        "all-indexer-settings",
        "--maximum-size",
        "1",
      ]);

      assert!(result.is_ok());

      if let Some(Command::Sonarr(SonarrCommand::Edit(edit_command))) = result.unwrap().command {
        assert_eq!(edit_command, expected_args);
      }
    }

    #[test]
    fn test_edit_all_indexer_settings_all_arguments_defined() {
      let expected_args = SonarrEditCommand::AllIndexerSettings {
        maximum_size: Some(1),
        minimum_age: Some(1),
        retention: Some(1),
        rss_sync_interval: Some(1),
      };
      let result = Cli::try_parse_from([
        "managarr",
        "sonarr",
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

      assert!(result.is_ok());

      if let Some(Command::Sonarr(SonarrCommand::Edit(edit_command))) = result.unwrap().command {
        assert_eq!(edit_command, expected_args);
      }
    }

    #[test]
    fn test_edit_indexer_requires_arguments() {
      let result = Cli::command().try_get_matches_from(["managarr", "sonarr", "edit", "indexer"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_edit_indexer_with_indexer_id_still_requires_arguments() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "edit",
        "indexer",
        "--indexer-id",
        "1",
      ]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_edit_indexer_rss_flags_conflict() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "edit",
        "indexer",
        "--indexer-id",
        "1",
        "--enable-rss",
        "--disable-rss",
      ]);

      assert!(result.is_err());
      assert_eq!(result.unwrap_err().kind(), ErrorKind::ArgumentConflict);
    }

    #[test]
    fn test_edit_indexer_automatic_search_flags_conflict() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "edit",
        "indexer",
        "--indexer-id",
        "1",
        "--enable-automatic-search",
        "--disable-automatic-search",
      ]);

      assert!(result.is_err());
      assert_eq!(result.unwrap_err().kind(), ErrorKind::ArgumentConflict);
    }

    #[test]
    fn test_edit_indexer_interactive_search_flags_conflict() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "edit",
        "indexer",
        "--indexer-id",
        "1",
        "--enable-interactive-search",
        "--disable-interactive-search",
      ]);

      assert!(result.is_err());
      assert_eq!(result.unwrap_err().kind(), ErrorKind::ArgumentConflict);
    }

    #[test]
    fn test_edit_indexer_tag_flags_conflict() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "edit",
        "indexer",
        "--indexer-id",
        "1",
        "--tag",
        "1",
        "--clear-tags",
      ]);

      assert!(result.is_err());
      assert_eq!(result.unwrap_err().kind(), ErrorKind::ArgumentConflict);
    }

    #[rstest]
    fn test_edit_indexer_assert_argument_flags_require_args(
      #[values("--name", "--url", "--api-key", "--seed-ratio", "--tag", "--priority")] flag: &str,
    ) {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "edit",
        "indexer",
        "--indexer-id",
        "1",
        flag,
      ]);

      assert!(result.is_err());
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_edit_indexer_only_requires_at_least_one_argument_plus_indexer_id() {
      let expected_args = SonarrEditCommand::Indexer {
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
        "sonarr",
        "edit",
        "indexer",
        "--indexer-id",
        "1",
        "--name",
        "Test",
      ]);

      assert!(result.is_ok());

      if let Some(Command::Sonarr(SonarrCommand::Edit(edit_command))) = result.unwrap().command {
        assert_eq!(edit_command, expected_args);
      }
    }

    #[test]
    fn test_edit_indexer_tag_argument_is_repeatable() {
      let expected_args = SonarrEditCommand::Indexer {
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
        "sonarr",
        "edit",
        "indexer",
        "--indexer-id",
        "1",
        "--tag",
        "1",
        "--tag",
        "2",
      ]);

      assert!(result.is_ok());

      if let Some(Command::Sonarr(SonarrCommand::Edit(edit_command))) = result.unwrap().command {
        assert_eq!(edit_command, expected_args);
      }
    }

    #[test]
    fn test_edit_indexer_all_arguments_defined() {
      let expected_args = SonarrEditCommand::Indexer {
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
        "sonarr",
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

      assert!(result.is_ok());

      if let Some(Command::Sonarr(SonarrCommand::Edit(edit_command))) = result.unwrap().command {
        assert_eq!(edit_command, expected_args);
      }
    }

    #[test]
    fn test_edit_series_requires_arguments() {
      let result = Cli::command().try_get_matches_from(["managarr", "sonarr", "edit", "series"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_edit_series_with_series_id_still_requires_arguments() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "edit",
        "series",
        "--series-id",
        "1",
      ]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_edit_series_monitoring_flags_conflict() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "edit",
        "series",
        "--series-id",
        "1",
        "--enable-monitoring",
        "--disable-monitoring",
      ]);

      assert!(result.is_err());
      assert_eq!(result.unwrap_err().kind(), ErrorKind::ArgumentConflict);
    }

    #[test]
    fn test_edit_series_season_folders_flags_conflict() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "edit",
        "series",
        "--series-id",
        "1",
        "--enable-season-folders",
        "--disable-season-folders",
      ]);

      assert!(result.is_err());
      assert_eq!(result.unwrap_err().kind(), ErrorKind::ArgumentConflict);
    }

    #[test]
    fn test_edit_series_tag_flags_conflict() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "edit",
        "series",
        "--series-id",
        "1",
        "--tag",
        "1",
        "--clear-tags",
      ]);

      assert!(result.is_err());
      assert_eq!(result.unwrap_err().kind(), ErrorKind::ArgumentConflict);
    }

    #[rstest]
    fn test_edit_series_assert_argument_flags_require_args(
      #[values(
        "--series-type",
        "--quality-profile-id",
        "--language-profile-id",
        "--root-folder-path",
        "--tag"
      )]
      flag: &str,
    ) {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "edit",
        "series",
        "--series-id",
        "1",
        flag,
      ]);

      assert!(result.is_err());
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_edit_series_series_type_validation() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "edit",
        "series",
        "--series-id",
        "1",
        "--series-type",
        "test",
      ]);

      assert!(result.is_err());
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_edit_series_only_requires_at_least_one_argument_plus_series_id() {
      let expected_args = SonarrEditCommand::Series {
        series_id: 1,
        enable_monitoring: false,
        disable_monitoring: false,
        enable_season_folders: false,
        disable_season_folders: false,
        series_type: None,
        quality_profile_id: None,
        language_profile_id: None,
        root_folder_path: Some("/nfs/test".to_owned()),
        tag: None,
        clear_tags: false,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "sonarr",
        "edit",
        "series",
        "--series-id",
        "1",
        "--root-folder-path",
        "/nfs/test",
      ]);

      assert!(result.is_ok());

      if let Some(Command::Sonarr(SonarrCommand::Edit(edit_command))) = result.unwrap().command {
        assert_eq!(edit_command, expected_args);
      }
    }

    #[test]
    fn test_edit_series_tag_argument_is_repeatable() {
      let expected_args = SonarrEditCommand::Series {
        series_id: 1,
        enable_monitoring: false,
        disable_monitoring: false,
        enable_season_folders: false,
        disable_season_folders: false,
        series_type: None,
        quality_profile_id: None,
        language_profile_id: None,
        root_folder_path: None,
        tag: Some(vec![1, 2]),
        clear_tags: false,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "sonarr",
        "edit",
        "series",
        "--series-id",
        "1",
        "--tag",
        "1",
        "--tag",
        "2",
      ]);

      assert!(result.is_ok());

      if let Some(Command::Sonarr(SonarrCommand::Edit(edit_command))) = result.unwrap().command {
        assert_eq!(edit_command, expected_args);
      }
    }

    #[test]
    fn test_edit_series_all_arguments_defined() {
      let expected_args = SonarrEditCommand::Series {
        series_id: 1,
        enable_monitoring: true,
        disable_monitoring: false,
        enable_season_folders: true,
        disable_season_folders: false,
        series_type: Some(SeriesType::Anime),
        quality_profile_id: Some(1),
        language_profile_id: Some(1),
        root_folder_path: Some("/nfs/test".to_owned()),
        tag: Some(vec![1, 2]),
        clear_tags: false,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "sonarr",
        "edit",
        "series",
        "--series-id",
        "1",
        "--enable-monitoring",
        "--enable-season-folders",
        "--series-type",
        "anime",
        "--quality-profile-id",
        "1",
        "--language-profile-id",
        "1",
        "--root-folder-path",
        "/nfs/test",
        "--tag",
        "1",
        "--tag",
        "2",
      ]);

      assert!(result.is_ok());

      if let Some(Command::Sonarr(SonarrCommand::Edit(edit_command))) = result.unwrap().command {
        assert_eq!(edit_command, expected_args);
      }
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
        sonarr::edit_command_handler::{SonarrEditCommand, SonarrEditCommandHandler},
        CliCommandHandler,
      },
      models::{
        servarr_models::EditIndexerParams,
        sonarr_models::{EditSeriesParams, IndexerSettings, SeriesType, SonarrSerdeable},
        Serdeable,
      },
      network::{sonarr_network::SonarrEvent, MockNetworkTrait, NetworkEvent},
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
          SonarrEvent::GetAllIndexerSettings.into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::IndexerSettings(
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
          SonarrEvent::EditAllIndexerSettings(Some(expected_edit_all_indexer_settings)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let edit_all_indexer_settings_command = SonarrEditCommand::AllIndexerSettings {
        maximum_size: Some(1),
        minimum_age: Some(1),
        retention: Some(1),
        rss_sync_interval: Some(1),
      };

      let result = SonarrEditCommandHandler::with(
        &app_arc,
        edit_all_indexer_settings_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert!(result.is_ok());
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
        priority: Some(25),
        clear_tags: false,
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::EditIndexer(Some(expected_edit_indexer_params)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let edit_indexer_command = SonarrEditCommand::Indexer {
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
        SonarrEditCommandHandler::with(&app_arc, edit_indexer_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_edit_series_command() {
      let expected_edit_series_params = EditSeriesParams {
        series_id: 1,
        monitored: Some(true),
        use_season_folders: Some(true),
        series_type: Some(SeriesType::Anime),
        quality_profile_id: Some(1),
        language_profile_id: Some(1),
        root_folder_path: Some("/nfs/test".to_owned()),
        tags: Some(vec![1, 2]),
        clear_tags: false,
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::EditSeries(Some(expected_edit_series_params)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let edit_series_command = SonarrEditCommand::Series {
        series_id: 1,
        enable_monitoring: true,
        disable_monitoring: false,
        enable_season_folders: true,
        disable_season_folders: false,
        series_type: Some(SeriesType::Anime),
        quality_profile_id: Some(1),
        language_profile_id: Some(1),
        root_folder_path: Some("/nfs/test".to_owned()),
        tag: Some(vec![1, 2]),
        clear_tags: false,
      };

      let result = SonarrEditCommandHandler::with(&app_arc, edit_series_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_edit_series_command_handles_disable_monitoring_flag_properly() {
      let expected_edit_series_params = EditSeriesParams {
        series_id: 1,
        monitored: Some(false),
        use_season_folders: Some(false),
        series_type: Some(SeriesType::Anime),
        quality_profile_id: Some(1),
        language_profile_id: Some(1),
        root_folder_path: Some("/nfs/test".to_owned()),
        tags: Some(vec![1, 2]),
        clear_tags: false,
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::EditSeries(Some(expected_edit_series_params)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let edit_series_command = SonarrEditCommand::Series {
        series_id: 1,
        enable_monitoring: false,
        disable_monitoring: true,
        enable_season_folders: false,
        disable_season_folders: true,
        series_type: Some(SeriesType::Anime),
        quality_profile_id: Some(1),
        language_profile_id: Some(1),
        root_folder_path: Some("/nfs/test".to_owned()),
        tag: Some(vec![1, 2]),
        clear_tags: false,
      };

      let result = SonarrEditCommandHandler::with(&app_arc, edit_series_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_edit_series_command_no_monitoring_boolean_flags_returns_none_value() {
      let expected_edit_series_params = EditSeriesParams {
        series_id: 1,
        monitored: None,
        use_season_folders: None,
        series_type: Some(SeriesType::Anime),
        quality_profile_id: Some(1),
        language_profile_id: Some(1),
        root_folder_path: Some("/nfs/test".to_owned()),
        tags: Some(vec![1, 2]),
        clear_tags: false,
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::EditSeries(Some(expected_edit_series_params)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let edit_series_command = SonarrEditCommand::Series {
        series_id: 1,
        enable_monitoring: false,
        disable_monitoring: false,
        enable_season_folders: false,
        disable_season_folders: false,
        series_type: Some(SeriesType::Anime),
        quality_profile_id: Some(1),
        language_profile_id: Some(1),
        root_folder_path: Some("/nfs/test".to_owned()),
        tag: Some(vec![1, 2]),
        clear_tags: false,
      };

      let result = SonarrEditCommandHandler::with(&app_arc, edit_series_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }
  }
}
