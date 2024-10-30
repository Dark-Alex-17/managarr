#[cfg(test)]
mod tests {
  use clap::{error::ErrorKind, CommandFactory, Parser};

  use crate::{
    cli::{
      radarr::{add_command_handler::RadarrAddCommand, RadarrCommand},
      Command,
    },
    models::radarr_models::{MinimumAvailability, Monitor},
    Cli,
  };

  #[test]
  fn test_radarr_add_command_from() {
    let command = RadarrAddCommand::Tag {
      name: String::new(),
    };

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Radarr(RadarrCommand::Add(command)));
  }

  mod cli {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn test_add_movie_requires_arguments() {
      let result = Cli::command().try_get_matches_from(["managarr", "radarr", "add", "movie"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_add_movie_requires_root_folder_path() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "radarr",
        "add",
        "movie",
        "--tmdb-id",
        "1",
        "--quality-profile-id",
        "1",
      ]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_add_movie_requires_quality_profile_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "radarr",
        "add",
        "movie",
        "--tmdb-id",
        "1",
        "--root-folder-path",
        "/test",
      ]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_add_movie_requires_tmdb_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "radarr",
        "add",
        "movie",
        "--root-folder-path",
        "/test",
        "--quality-profile-id",
        "1",
      ]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[rstest]
    fn test_add_movie_assert_argument_flags_require_args(
      #[values("--minimum-availability", "--tag", "--monitor")] flag: &str,
    ) {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "radarr",
        "add",
        "movie",
        "--root-folder-path",
        "/test",
        "--quality-profile-id",
        "1",
        flag,
      ]);

      assert!(result.is_err());
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_add_movie_all_arguments_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "radarr",
        "add",
        "movie",
        "--root-folder-path",
        "/test",
        "--quality-profile-id",
        "1",
        "--tmdb-id",
        "1",
      ]);

      assert!(result.is_ok());
    }

    #[test]
    fn test_add_movie_minimum_availability_validation() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "radarr",
        "add",
        "movie",
        "--root-folder-path",
        "/test",
        "--quality-profile-id",
        "1",
        "--tmdb-id",
        "1",
        "--minimum-availability",
        "test",
      ]);

      assert!(result.is_err());
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_add_movie_monitor_validation() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "radarr",
        "add",
        "movie",
        "--root-folder-path",
        "/test",
        "--quality-profile-id",
        "1",
        "--tmdb-id",
        "1",
        "--monitor",
        "test",
      ]);

      assert!(result.is_err());
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_add_movie_defaults() {
      let expected_args = RadarrAddCommand::Movie {
        tmdb_id: 1,
        root_folder_path: "/test".to_owned(),
        quality_profile_id: 1,
        minimum_availability: MinimumAvailability::default(),
        disable_monitoring: false,
        tag: vec![],
        monitor: Monitor::default(),
        no_search_for_movie: false,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "radarr",
        "add",
        "movie",
        "--root-folder-path",
        "/test",
        "--quality-profile-id",
        "1",
        "--tmdb-id",
        "1",
      ]);

      assert!(result.is_ok());
      if let Some(Command::Radarr(RadarrCommand::Add(add_command))) = result.unwrap().command {
        assert_eq!(add_command, expected_args);
      }
    }

    #[test]
    fn test_add_movie_tags_is_repeatable() {
      let expected_args = RadarrAddCommand::Movie {
        tmdb_id: 1,
        root_folder_path: "/test".to_owned(),
        quality_profile_id: 1,
        minimum_availability: MinimumAvailability::default(),
        disable_monitoring: false,
        tag: vec![1, 2],
        monitor: Monitor::default(),
        no_search_for_movie: false,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "radarr",
        "add",
        "movie",
        "--root-folder-path",
        "/test",
        "--quality-profile-id",
        "1",
        "--tmdb-id",
        "1",
        "--tag",
        "1",
        "--tag",
        "2",
      ]);

      assert!(result.is_ok());
      if let Some(Command::Radarr(RadarrCommand::Add(add_command))) = result.unwrap().command {
        assert_eq!(add_command, expected_args);
      }
    }

    #[test]
    fn test_add_movie_all_args_defined() {
      let expected_args = RadarrAddCommand::Movie {
        tmdb_id: 1,
        root_folder_path: "/test".to_owned(),
        quality_profile_id: 1,
        minimum_availability: MinimumAvailability::Released,
        disable_monitoring: true,
        tag: vec![1, 2],
        monitor: Monitor::MovieAndCollection,
        no_search_for_movie: true,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "radarr",
        "add",
        "movie",
        "--root-folder-path",
        "/test",
        "--quality-profile-id",
        "1",
        "--minimum-availability",
        "released",
        "--disable-monitoring",
        "--tmdb-id",
        "1",
        "--tag",
        "1",
        "--tag",
        "2",
        "--monitor",
        "movie-and-collection",
        "--no-search-for-movie",
      ]);

      assert!(result.is_ok());
      if let Some(Command::Radarr(RadarrCommand::Add(add_command))) = result.unwrap().command {
        assert_eq!(add_command, expected_args);
      }
    }

    #[test]
    fn test_add_root_folder_requires_arguments() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "radarr", "add", "root-folder"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_add_root_folder_success() {
      let expected_args = RadarrAddCommand::RootFolder {
        root_folder_path: "/nfs/test".to_owned(),
      };

      let result = Cli::try_parse_from([
        "managarr",
        "radarr",
        "add",
        "root-folder",
        "--root-folder-path",
        "/nfs/test",
      ]);

      assert!(result.is_ok());

      if let Some(Command::Radarr(RadarrCommand::Add(add_command))) = result.unwrap().command {
        assert_eq!(add_command, expected_args);
      }
    }

    #[test]
    fn test_add_tag_requires_arguments() {
      let result = Cli::command().try_get_matches_from(["managarr", "radarr", "add", "tag"]);

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_add_tag_success() {
      let expected_args = RadarrAddCommand::Tag {
        name: "test".to_owned(),
      };

      let result = Cli::try_parse_from(["managarr", "radarr", "add", "tag", "--name", "test"]);

      assert!(result.is_ok());

      if let Some(Command::Radarr(RadarrCommand::Add(add_command))) = result.unwrap().command {
        assert_eq!(add_command, expected_args);
      }
    }
  }

  mod handler {
    use std::sync::Arc;

    use crate::{
      app::App,
      cli::{radarr::add_command_handler::RadarrAddCommandHandler, CliCommandHandler},
      models::{
        radarr_models::{AddMovieBody, AddOptions, RadarrSerdeable},
        Serdeable,
      },
      network::{radarr_network::RadarrEvent, MockNetworkTrait, NetworkEvent},
    };

    use super::*;
    use mockall::predicate::eq;

    use serde_json::json;
    use tokio::sync::Mutex;

    #[tokio::test]
    async fn test_handle_add_movie_command() {
      let expected_add_movie_body = AddMovieBody {
        tmdb_id: 1,
        title: String::new(),
        root_folder_path: "/test".to_owned(),
        quality_profile_id: 1,
        minimum_availability: "released".to_owned(),
        monitored: false,
        tags: vec![1, 2],
        add_options: AddOptions {
          monitor: "movieAndCollection".to_owned(),
          search_for_movie: false,
        },
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::AddMovie(Some(expected_add_movie_body)).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let add_movie_command = RadarrAddCommand::Movie {
        tmdb_id: 1,
        root_folder_path: "/test".to_owned(),
        quality_profile_id: 1,
        minimum_availability: MinimumAvailability::Released,
        disable_monitoring: true,
        tag: vec![1, 2],
        monitor: Monitor::MovieAndCollection,
        no_search_for_movie: true,
      };

      let result = RadarrAddCommandHandler::with(&app_arc, add_movie_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_add_root_folder_command() {
      let expected_root_folder_path = "/nfs/test".to_owned();
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          RadarrEvent::AddRootFolder(Some(expected_root_folder_path.clone())).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Radarr(RadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let add_root_folder_command = RadarrAddCommand::RootFolder {
        root_folder_path: expected_root_folder_path,
      };

      let result =
        RadarrAddCommandHandler::with(&app_arc, add_root_folder_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_add_tag_command() {
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
      let add_tag_command = RadarrAddCommand::Tag {
        name: expected_tag_name,
      };

      let result = RadarrAddCommandHandler::with(&app_arc, add_tag_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }
  }
}
