#[cfg(test)]
mod tests {
  use crate::{
    cli::{
      radarr::{edit_command_handler::RadarrEditCommand, RadarrCommand},
      Command,
    },
    Cli,
  };
  use clap::{error::ErrorKind, CommandFactory, Parser};
  use pretty_assertions::assert_eq;

  #[test]
  fn test_radarr_edit_command_from() {
    let command = RadarrEditCommand::AllIndexerSettings {
      allow_hardcoded_subs: true,
      disable_allow_hardcoded_subs: false,
      availability_delay: None,
      maximum_size: None,
      minimum_age: None,
      prefer_indexer_flags: true,
      disable_prefer_indexer_flags: false,
      retention: None,
      rss_sync_interval: None,
      whitelisted_subtitle_tags: None,
    };

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Radarr(RadarrCommand::Edit(command)));
  }

  mod cli {
    use crate::models::radarr_models::MinimumAvailability;

    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn test_edit_all_indexer_settings_requires_arguments() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "radarr", "edit", "all-indexer-settings"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_edit_all_indexer_settings_hardcoded_subs_flags_conflict() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "radarr",
        "edit",
        "all-indexer-settings",
        "--allow-hardcoded-subs",
        "--disable-allow-hardcoded-subs",
      ]);

      assert!(result.is_err());
      assert_eq!(result.unwrap_err().kind(), ErrorKind::ArgumentConflict);
    }

    #[test]
    fn test_edit_all_indexer_settings_prefer_indexer_flags_conflict() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "radarr",
        "edit",
        "all-indexer-settings",
        "--prefer-indexer-flags",
        "--disable-prefer-indexer-flags",
      ]);

      assert!(result.is_err());
      assert_eq!(result.unwrap_err().kind(), ErrorKind::ArgumentConflict);
    }

    #[rstest]
    fn test_edit_all_indexer_settings_assert_argument_flags_require_args(
      #[values(
        "--availability-delay",
        "--maximum-size",
        "--minimum-age",
        "--retention",
        "--rss-sync-interval",
        "--whitelisted-subtitle-tags"
      )]
      flag: &str,
    ) {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "radarr",
        "edit",
        "all-indexer-settings",
        flag,
      ]);

      assert!(result.is_err());
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_edit_all_indexer_settings_only_requires_at_least_one_argument() {
      let expected_args = RadarrEditCommand::AllIndexerSettings {
        allow_hardcoded_subs: false,
        disable_allow_hardcoded_subs: false,
        availability_delay: Some(1),
        maximum_size: None,
        minimum_age: None,
        prefer_indexer_flags: false,
        disable_prefer_indexer_flags: false,
        retention: None,
        rss_sync_interval: None,
        whitelisted_subtitle_tags: None,
      };
      let result = Cli::try_parse_from([
        "managarr",
        "radarr",
        "edit",
        "all-indexer-settings",
        "--availability-delay",
        "1",
      ]);

      assert!(result.is_ok());

      if let Some(Command::Radarr(RadarrCommand::Edit(edit_command))) = result.unwrap().command {
        assert_eq!(edit_command, expected_args);
      }
    }

    #[test]
    fn test_edit_all_indexer_settings_all_arguments_defined() {
      let expected_args = RadarrEditCommand::AllIndexerSettings {
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
      };
      let result = Cli::try_parse_from([
        "managarr",
        "radarr",
        "edit",
        "all-indexer-settings",
        "--allow-hardcoded-subs",
        "--availability-delay",
        "1",
        "--maximum-size",
        "1",
        "--minimum-age",
        "1",
        "--prefer-indexer-flags",
        "--retention",
        "1",
        "--rss-sync-interval",
        "1",
        "--whitelisted-subtitle-tags",
        "test",
      ]);

      assert!(result.is_ok());

      if let Some(Command::Radarr(RadarrCommand::Edit(edit_command))) = result.unwrap().command {
        assert_eq!(edit_command, expected_args);
      }
    }

    #[test]
    fn test_edit_collection_requires_arguments() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "radarr", "edit", "collection"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_edit_collection_with_collection_id_still_requires_arguments() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "radarr",
        "edit",
        "collection",
        "--collection-id",
        "1",
      ]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_edit_collection_monitoring_flags_conflict() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "radarr",
        "edit",
        "collection",
        "--collection-id",
        "1",
        "--enable-monitoring",
        "--disable-monitoring",
      ]);

      assert!(result.is_err());
      assert_eq!(result.unwrap_err().kind(), ErrorKind::ArgumentConflict);
    }

    #[test]
    fn test_edit_collection_search_on_add_flags_conflict() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "radarr",
        "edit",
        "collection",
        "--collection-id",
        "1",
        "--search-on-add",
        "--disable-search-on-add",
      ]);

      assert!(result.is_err());
      assert_eq!(result.unwrap_err().kind(), ErrorKind::ArgumentConflict);
    }

    #[test]
    fn test_edit_collection_minimum_availability_validation() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "radarr",
        "edit",
        "collection",
        "--collection-id",
        "1",
        "--minimum-availability",
        "test",
      ]);

      assert!(result.is_err());
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[rstest]
    fn test_edit_collection_assert_argument_flags_require_args(
      #[values("--minimum-availability", "--quality-profile-id", "--root-folder-path")] flag: &str,
    ) {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "radarr",
        "edit",
        "collection",
        "--collection-id",
        "1",
        flag,
      ]);

      assert!(result.is_err());
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_edit_collection_only_requires_at_least_one_argument_plus_collection_id() {
      let expected_args = RadarrEditCommand::Collection {
        collection_id: 1,
        enable_monitoring: false,
        disable_monitoring: false,
        minimum_availability: None,
        quality_profile_id: None,
        root_folder_path: Some("/test".to_owned()),
        search_on_add: false,
        disable_search_on_add: false,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "radarr",
        "edit",
        "collection",
        "--collection-id",
        "1",
        "--root-folder-path",
        "/test",
      ]);

      assert!(result.is_ok());

      if let Some(Command::Radarr(RadarrCommand::Edit(edit_command))) = result.unwrap().command {
        assert_eq!(edit_command, expected_args);
      }
    }

    #[test]
    fn test_edit_collection_all_arguments_defined() {
      let expected_args = RadarrEditCommand::Collection {
        collection_id: 1,
        enable_monitoring: true,
        disable_monitoring: false,
        minimum_availability: Some(MinimumAvailability::Released),
        quality_profile_id: Some(1),
        root_folder_path: Some("/test".to_owned()),
        search_on_add: true,
        disable_search_on_add: false,
      };
      let result = Cli::try_parse_from([
        "managarr",
        "radarr",
        "edit",
        "collection",
        "--collection-id",
        "1",
        "--enable-monitoring",
        "--minimum-availability",
        "released",
        "--quality-profile-id",
        "1",
        "--root-folder-path",
        "/test",
        "--search-on-add",
      ]);

      assert!(result.is_ok());

      if let Some(Command::Radarr(RadarrCommand::Edit(edit_command))) = result.unwrap().command {
        assert_eq!(edit_command, expected_args);
      }
    }

    #[test]
    fn test_edit_indexer_requires_arguments() {
      let result = Cli::command().try_get_matches_from(["managarr", "radarr", "edit", "indexer"]);

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
        "radarr",
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
        "radarr",
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
        "radarr",
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
        "radarr",
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
        "radarr",
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
        "radarr",
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
      let expected_args = RadarrEditCommand::Indexer {
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
        "radarr",
        "edit",
        "indexer",
        "--indexer-id",
        "1",
        "--name",
        "Test",
      ]);

      assert!(result.is_ok());

      if let Some(Command::Radarr(RadarrCommand::Edit(edit_command))) = result.unwrap().command {
        assert_eq!(edit_command, expected_args);
      }
    }

    #[test]
    fn test_edit_indexer_tag_argument_is_repeatable() {
      let expected_args = RadarrEditCommand::Indexer {
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
        "radarr",
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

      if let Some(Command::Radarr(RadarrCommand::Edit(edit_command))) = result.unwrap().command {
        assert_eq!(edit_command, expected_args);
      }
    }

    #[test]
    fn test_edit_indexer_all_arguments_defined() {
      let expected_args = RadarrEditCommand::Indexer {
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
        "radarr",
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

      if let Some(Command::Radarr(RadarrCommand::Edit(edit_command))) = result.unwrap().command {
        assert_eq!(edit_command, expected_args);
      }
    }

    #[test]
    fn test_edit_movie_requires_arguments() {
      let result = Cli::command().try_get_matches_from(["managarr", "radarr", "edit", "movie"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_edit_movie_with_movie_id_still_requires_arguments() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "radarr",
        "edit",
        "movie",
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
    fn test_edit_movie_monitoring_flags_conflict() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "radarr",
        "edit",
        "movie",
        "--movie-id",
        "1",
        "--enable-monitoring",
        "--disable-monitoring",
      ]);

      assert!(result.is_err());
      assert_eq!(result.unwrap_err().kind(), ErrorKind::ArgumentConflict);
    }

    #[test]
    fn test_edit_movie_tag_flags_conflict() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "radarr",
        "edit",
        "movie",
        "--movie-id",
        "1",
        "--tag",
        "1",
        "--clear-tags",
      ]);

      assert!(result.is_err());
      assert_eq!(result.unwrap_err().kind(), ErrorKind::ArgumentConflict);
    }

    #[rstest]
    fn test_edit_movie_assert_argument_flags_require_args(
      #[values(
        "--minimum-availability",
        "--quality-profile-id",
        "--root-folder-path",
        "--tag"
      )]
      flag: &str,
    ) {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "radarr",
        "edit",
        "movie",
        "--movie-id",
        "1",
        flag,
      ]);

      assert!(result.is_err());
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_edit_movie_minimum_availability_validation() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "radarr",
        "edit",
        "movie",
        "--movie-id",
        "1",
        "--minimum-availability",
        "test",
      ]);

      assert!(result.is_err());
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_edit_movie_only_requires_at_least_one_argument_plus_movie_id() {
      let expected_args = RadarrEditCommand::Movie {
        movie_id: 1,
        enable_monitoring: false,
        disable_monitoring: false,
        minimum_availability: None,
        quality_profile_id: None,
        root_folder_path: Some("/nfs/test".to_owned()),
        tag: None,
        clear_tags: false,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "radarr",
        "edit",
        "movie",
        "--movie-id",
        "1",
        "--root-folder-path",
        "/nfs/test",
      ]);

      assert!(result.is_ok());

      if let Some(Command::Radarr(RadarrCommand::Edit(edit_command))) = result.unwrap().command {
        assert_eq!(edit_command, expected_args);
      }
    }

    #[test]
    fn test_edit_movie_tag_argument_is_repeatable() {
      let expected_args = RadarrEditCommand::Movie {
        movie_id: 1,
        enable_monitoring: false,
        disable_monitoring: false,
        minimum_availability: None,
        quality_profile_id: None,
        root_folder_path: None,
        tag: Some(vec![1, 2]),
        clear_tags: false,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "radarr",
        "edit",
        "movie",
        "--movie-id",
        "1",
        "--tag",
        "1",
        "--tag",
        "2",
      ]);

      assert!(result.is_ok());

      if let Some(Command::Radarr(RadarrCommand::Edit(edit_command))) = result.unwrap().command {
        assert_eq!(edit_command, expected_args);
      }
    }

    #[test]
    fn test_edit_movie_all_arguments_defined() {
      let expected_args = RadarrEditCommand::Movie {
        movie_id: 1,
        enable_monitoring: true,
        disable_monitoring: false,
        minimum_availability: Some(MinimumAvailability::Released),
        quality_profile_id: Some(1),
        root_folder_path: Some("/nfs/test".to_owned()),
        tag: Some(vec![1, 2]),
        clear_tags: false,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "radarr",
        "edit",
        "movie",
        "--movie-id",
        "1",
        "--enable-monitoring",
        "--minimum-availability",
        "released",
        "--quality-profile-id",
        "1",
        "--root-folder-path",
        "/nfs/test",
        "--tag",
        "1",
        "--tag",
        "2",
      ]);

      assert!(result.is_ok());

      if let Some(Command::Radarr(RadarrCommand::Edit(edit_command))) = result.unwrap().command {
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
        radarr::edit_command_handler::{RadarrEditCommand, RadarrEditCommandHandler},
        CliCommandHandler,
      },
      models::{
        radarr_models::{
          EditCollectionParams, EditMovieParams, IndexerSettings, MinimumAvailability,
          RadarrSerdeable,
        },
        servarr_models::EditIndexerParams,
        Serdeable,
      },
      network::{radarr_network::RadarrEvent, MockNetworkTrait, NetworkEvent},
    };

    #[tokio::test]
    async fn test_handle_edit_all_indexer_settings_command() {
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
          RadarrEvent::EditAllIndexerSettings(expected_edit_all_indexer_settings).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let edit_all_indexer_settings_command = RadarrEditCommand::AllIndexerSettings {
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
      };

      let result = RadarrEditCommandHandler::with(
        &app_arc,
        edit_all_indexer_settings_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_edit_all_indexer_settings_command_disable_flags_function_properly() {
      let expected_edit_all_indexer_settings = IndexerSettings {
        allow_hardcoded_subs: false,
        availability_delay: 1,
        id: 1,
        maximum_size: 1,
        minimum_age: 1,
        prefer_indexer_flags: false,
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
              allow_hardcoded_subs: true,
              availability_delay: 2,
              id: 1,
              maximum_size: 2,
              minimum_age: 2,
              prefer_indexer_flags: true,
              retention: 2,
              rss_sync_interval: 2,
              whitelisted_hardcoded_subs: "testing".into(),
            },
          )))
        });
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::EditAllIndexerSettings(expected_edit_all_indexer_settings).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let edit_all_indexer_settings_command = RadarrEditCommand::AllIndexerSettings {
        allow_hardcoded_subs: false,
        disable_allow_hardcoded_subs: true,
        availability_delay: Some(1),
        maximum_size: Some(1),
        minimum_age: Some(1),
        prefer_indexer_flags: false,
        disable_prefer_indexer_flags: true,
        retention: Some(1),
        rss_sync_interval: Some(1),
        whitelisted_subtitle_tags: Some("test".to_owned()),
      };

      let result = RadarrEditCommandHandler::with(
        &app_arc,
        edit_all_indexer_settings_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_edit_all_indexer_settings_command_unprovided_values_default_to_previous_values(
    ) {
      let expected_edit_all_indexer_settings = IndexerSettings {
        allow_hardcoded_subs: true,
        availability_delay: 2,
        id: 1,
        maximum_size: 2,
        minimum_age: 2,
        prefer_indexer_flags: true,
        retention: 2,
        rss_sync_interval: 2,
        whitelisted_hardcoded_subs: "testing".into(),
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
              allow_hardcoded_subs: true,
              availability_delay: 2,
              id: 1,
              maximum_size: 2,
              minimum_age: 2,
              prefer_indexer_flags: true,
              retention: 2,
              rss_sync_interval: 2,
              whitelisted_hardcoded_subs: "testing".into(),
            },
          )))
        });
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::EditAllIndexerSettings(expected_edit_all_indexer_settings).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let edit_all_indexer_settings_command = RadarrEditCommand::AllIndexerSettings {
        allow_hardcoded_subs: false,
        disable_allow_hardcoded_subs: false,
        availability_delay: None,
        maximum_size: None,
        minimum_age: None,
        prefer_indexer_flags: false,
        disable_prefer_indexer_flags: false,
        retention: None,
        rss_sync_interval: None,
        whitelisted_subtitle_tags: None,
      };

      let result = RadarrEditCommandHandler::with(
        &app_arc,
        edit_all_indexer_settings_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_edit_collection_command() {
      let expected_edit_collection_params = EditCollectionParams {
        collection_id: 1,
        monitored: Some(true),
        minimum_availability: Some(MinimumAvailability::Released),
        quality_profile_id: Some(1),
        root_folder_path: Some("/nfs/test".to_owned()),
        search_on_add: Some(true),
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::EditCollection(expected_edit_collection_params).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let edit_collection_command = RadarrEditCommand::Collection {
        collection_id: 1,
        enable_monitoring: true,
        disable_monitoring: false,
        minimum_availability: Some(MinimumAvailability::Released),
        quality_profile_id: Some(1),
        root_folder_path: Some("/nfs/test".to_owned()),
        search_on_add: true,
        disable_search_on_add: false,
      };

      let result =
        RadarrEditCommandHandler::with(&app_arc, edit_collection_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_edit_collection_command_handles_disable_flags_properly() {
      let expected_edit_collection_params = EditCollectionParams {
        collection_id: 1,
        monitored: Some(false),
        minimum_availability: Some(MinimumAvailability::Released),
        quality_profile_id: Some(1),
        root_folder_path: Some("/nfs/test".to_owned()),
        search_on_add: Some(false),
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::EditCollection(expected_edit_collection_params).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let edit_collection_command = RadarrEditCommand::Collection {
        collection_id: 1,
        enable_monitoring: false,
        disable_monitoring: true,
        minimum_availability: Some(MinimumAvailability::Released),
        quality_profile_id: Some(1),
        root_folder_path: Some("/nfs/test".to_owned()),
        search_on_add: false,
        disable_search_on_add: true,
      };

      let result =
        RadarrEditCommandHandler::with(&app_arc, edit_collection_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_edit_collection_command_no_boolean_flags_returns_none_value() {
      let expected_edit_collection_params = EditCollectionParams {
        collection_id: 1,
        monitored: None,
        minimum_availability: Some(MinimumAvailability::Released),
        quality_profile_id: Some(1),
        root_folder_path: Some("/nfs/test".to_owned()),
        search_on_add: None,
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::EditCollection(expected_edit_collection_params).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let edit_collection_command = RadarrEditCommand::Collection {
        collection_id: 1,
        enable_monitoring: false,
        disable_monitoring: false,
        minimum_availability: Some(MinimumAvailability::Released),
        quality_profile_id: Some(1),
        root_folder_path: Some("/nfs/test".to_owned()),
        search_on_add: false,
        disable_search_on_add: false,
      };

      let result =
        RadarrEditCommandHandler::with(&app_arc, edit_collection_command, &mut mock_network)
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
          RadarrEvent::EditIndexer(Some(expected_edit_indexer_params)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let edit_indexer_command = RadarrEditCommand::Indexer {
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
        RadarrEditCommandHandler::with(&app_arc, edit_indexer_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_edit_indexer_command_handles_disable_flags_properly() {
      let expected_edit_indexer_params = EditIndexerParams {
        indexer_id: 1,
        name: Some("Test".to_owned()),
        enable_rss: Some(false),
        enable_automatic_search: Some(false),
        enable_interactive_search: Some(false),
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
          RadarrEvent::EditIndexer(Some(expected_edit_indexer_params)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let edit_indexer_command = RadarrEditCommand::Indexer {
        indexer_id: 1,
        name: Some("Test".to_owned()),
        enable_rss: false,
        disable_rss: true,
        enable_automatic_search: false,
        disable_automatic_search: true,
        enable_interactive_search: false,
        disable_interactive_search: true,
        url: Some("http://test.com".to_owned()),
        api_key: Some("testKey".to_owned()),
        seed_ratio: Some("1.2".to_owned()),
        tag: Some(vec![1, 2]),
        priority: Some(25),
        clear_tags: false,
      };

      let result =
        RadarrEditCommandHandler::with(&app_arc, edit_indexer_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_edit_indexer_command_no_boolean_flags_returns_none_value() {
      let expected_edit_indexer_params = EditIndexerParams {
        indexer_id: 1,
        name: Some("Test".to_owned()),
        enable_rss: None,
        enable_automatic_search: None,
        enable_interactive_search: None,
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
          RadarrEvent::EditIndexer(Some(expected_edit_indexer_params)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let edit_indexer_command = RadarrEditCommand::Indexer {
        indexer_id: 1,
        name: Some("Test".to_owned()),
        enable_rss: false,
        disable_rss: false,
        enable_automatic_search: false,
        disable_automatic_search: false,
        enable_interactive_search: false,
        disable_interactive_search: false,
        url: Some("http://test.com".to_owned()),
        api_key: Some("testKey".to_owned()),
        seed_ratio: Some("1.2".to_owned()),
        tag: Some(vec![1, 2]),
        priority: Some(25),
        clear_tags: false,
      };

      let result =
        RadarrEditCommandHandler::with(&app_arc, edit_indexer_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_edit_movie_command() {
      let expected_edit_movie_params = EditMovieParams {
        movie_id: 1,
        monitored: Some(true),
        minimum_availability: Some(MinimumAvailability::Released),
        quality_profile_id: Some(1),
        root_folder_path: Some("/nfs/test".to_owned()),
        tags: Some(vec![1, 2]),
        clear_tags: false,
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::EditMovie(Some(expected_edit_movie_params)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let edit_movie_command = RadarrEditCommand::Movie {
        movie_id: 1,
        enable_monitoring: true,
        disable_monitoring: false,
        minimum_availability: Some(MinimumAvailability::Released),
        quality_profile_id: Some(1),
        root_folder_path: Some("/nfs/test".to_owned()),
        tag: Some(vec![1, 2]),
        clear_tags: false,
      };

      let result = RadarrEditCommandHandler::with(&app_arc, edit_movie_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_edit_movie_command_handles_disable_monitoring_flag_properly() {
      let expected_edit_movie_params = EditMovieParams {
        movie_id: 1,
        monitored: Some(false),
        minimum_availability: Some(MinimumAvailability::Released),
        quality_profile_id: Some(1),
        root_folder_path: Some("/nfs/test".to_owned()),
        tags: Some(vec![1, 2]),
        clear_tags: false,
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::EditMovie(Some(expected_edit_movie_params)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let edit_movie_command = RadarrEditCommand::Movie {
        movie_id: 1,
        enable_monitoring: false,
        disable_monitoring: true,
        minimum_availability: Some(MinimumAvailability::Released),
        quality_profile_id: Some(1),
        root_folder_path: Some("/nfs/test".to_owned()),
        tag: Some(vec![1, 2]),
        clear_tags: false,
      };

      let result = RadarrEditCommandHandler::with(&app_arc, edit_movie_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_edit_movie_command_no_monitoring_boolean_flags_returns_none_value() {
      let expected_edit_movie_params = EditMovieParams {
        movie_id: 1,
        monitored: None,
        minimum_availability: Some(MinimumAvailability::Released),
        quality_profile_id: Some(1),
        root_folder_path: Some("/nfs/test".to_owned()),
        tags: Some(vec![1, 2]),
        clear_tags: false,
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::EditMovie(Some(expected_edit_movie_params)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let edit_movie_command = RadarrEditCommand::Movie {
        movie_id: 1,
        enable_monitoring: false,
        disable_monitoring: false,
        minimum_availability: Some(MinimumAvailability::Released),
        quality_profile_id: Some(1),
        root_folder_path: Some("/nfs/test".to_owned()),
        tag: Some(vec![1, 2]),
        clear_tags: false,
      };

      let result = RadarrEditCommandHandler::with(&app_arc, edit_movie_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }
  }
}
