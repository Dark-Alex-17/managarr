#[cfg(test)]
mod tests {
  use clap::{CommandFactory, Parser, error::ErrorKind};
  use pretty_assertions::assert_eq;

  use crate::{
    Cli,
    cli::{
      Command,
      sonarr::{SonarrCommand, add_command_handler::SonarrAddCommand},
    },
  };

  #[test]
  fn test_sonarr_add_command_from() {
    let command = SonarrAddCommand::Tag {
      name: String::new(),
    };

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Sonarr(SonarrCommand::Add(command)));
  }

  mod cli {
    use crate::models::sonarr_models::{SeriesMonitor, SeriesType};

    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn test_add_root_folder_requires_arguments() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "sonarr", "add", "root-folder"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_add_root_folder_success() {
      let expected_args = SonarrAddCommand::RootFolder {
        root_folder_path: "/nfs/test".to_owned(),
      };

      let result = Cli::try_parse_from([
        "managarr",
        "sonarr",
        "add",
        "root-folder",
        "--root-folder-path",
        "/nfs/test",
      ]);

      assert_ok!(&result);

      let Some(Command::Sonarr(SonarrCommand::Add(add_command))) = result.unwrap().command else {
        panic!("Unexpected command type");
      };
      assert_eq!(add_command, expected_args);
    }

    #[test]
    fn test_add_series_requires_arguments() {
      let result = Cli::command().try_get_matches_from(["managarr", "sonarr", "add", "series"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_add_series_requires_tvdb_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "add",
        "series",
        "--root-folder-path",
        "test",
        "--quality-profile-id",
        "1",
        "--language-profile-id",
        "1",
        "--title",
        "test",
      ]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_add_series_requires_title() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "add",
        "series",
        "--tvdb-id",
        "1",
        "--root-folder-path",
        "test",
        "--quality-profile-id",
        "1",
        "--language-profile-id",
        "1",
      ]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_add_series_requires_root_folder_path() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "add",
        "series",
        "--tvdb-id",
        "1",
        "--quality-profile-id",
        "1",
        "--language-profile-id",
        "1",
        "--title",
        "test",
      ]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_add_series_requires_quality_profile_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "add",
        "series",
        "--tvdb-id",
        "1",
        "--root-folder-path",
        "test",
        "--language-profile-id",
        "1",
        "--title",
        "test",
      ]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_add_series_requires_language_profile_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "add",
        "series",
        "--tvdb-id",
        "1",
        "--root-folder-path",
        "test",
        "--quality-profile-id",
        "1",
        "--title",
        "test",
      ]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[rstest]
    fn test_add_series_assert_argument_flags_require_args(
      #[values("--series-type", "--tag", "--monitor")] flag: &str,
    ) {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "add",
        "series",
        "--tvdb-id",
        "1",
        "--title",
        "test",
        "--root-folder-path",
        "/test",
        "--quality-profile-id",
        "1",
        "--language-profile-id",
        "1",
        flag,
      ]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_add_series_all_arguments_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "add",
        "series",
        "--title",
        "test",
        "--root-folder-path",
        "/test",
        "--quality-profile-id",
        "1",
        "--language-profile-id",
        "1",
        "--tvdb-id",
        "1",
      ]);

      assert_ok!(&result);
    }

    #[test]
    fn test_add_series_series_type_validation() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "add",
        "series",
        "--root-folder-path",
        "/test",
        "--quality-profile-id",
        "1",
        "--language-profile-id",
        "1",
        "--tvdb-id",
        "1",
        "--title",
        "test",
        "--series-type",
        "test",
      ]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_add_series_monitor_validation() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "add",
        "series",
        "--root-folder-path",
        "/test",
        "--quality-profile-id",
        "1",
        "--language-profile-id",
        "1",
        "--tvdb-id",
        "--title",
        "test",
        "1",
        "--monitor",
        "test",
      ]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_add_series_defaults() {
      let expected_args = SonarrAddCommand::Series {
        tvdb_id: 1,
        title: "test".to_owned(),
        root_folder_path: "/test".to_owned(),
        quality_profile_id: 1,
        language_profile_id: 1,
        series_type: SeriesType::default(),
        disable_monitoring: false,
        disable_season_folders: false,
        tag: vec![],
        monitor: SeriesMonitor::default(),
        no_search_for_series: false,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "sonarr",
        "add",
        "series",
        "--root-folder-path",
        "/test",
        "--quality-profile-id",
        "1",
        "--language-profile-id",
        "1",
        "--title",
        "test",
        "--tvdb-id",
        "1",
      ]);

      assert_ok!(&result);
      let Some(Command::Sonarr(SonarrCommand::Add(add_command))) = result.unwrap().command else {
        panic!("Unexpected command type");
      };
      assert_eq!(add_command, expected_args);
    }

    #[test]
    fn test_add_series_tags_is_repeatable() {
      let expected_args = SonarrAddCommand::Series {
        tvdb_id: 1,
        title: "test".to_owned(),
        root_folder_path: "/test".to_owned(),
        quality_profile_id: 1,
        language_profile_id: 1,
        series_type: SeriesType::default(),
        disable_monitoring: false,
        disable_season_folders: false,
        tag: vec![1, 2],
        monitor: SeriesMonitor::default(),
        no_search_for_series: false,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "sonarr",
        "add",
        "series",
        "--root-folder-path",
        "/test",
        "--quality-profile-id",
        "1",
        "--language-profile-id",
        "1",
        "--tvdb-id",
        "1",
        "--title",
        "test",
        "--tag",
        "1",
        "--tag",
        "2",
      ]);

      assert_ok!(&result);
      let Some(Command::Sonarr(SonarrCommand::Add(add_command))) = result.unwrap().command else {
        panic!("Unexpected command type");
      };
      assert_eq!(add_command, expected_args);
    }

    #[test]
    fn test_add_series_all_args_defined() {
      let expected_args = SonarrAddCommand::Series {
        tvdb_id: 1,
        title: "test".to_owned(),
        root_folder_path: "/test".to_owned(),
        quality_profile_id: 1,
        language_profile_id: 1,
        series_type: SeriesType::Anime,
        disable_monitoring: true,
        disable_season_folders: true,
        tag: vec![1, 2],
        monitor: SeriesMonitor::Future,
        no_search_for_series: true,
      };

      let result = Cli::try_parse_from([
        "managarr",
        "sonarr",
        "add",
        "series",
        "--root-folder-path",
        "/test",
        "--quality-profile-id",
        "1",
        "--language-profile-id",
        "1",
        "--series-type",
        "anime",
        "--disable-monitoring",
        "--disable-season-folders",
        "--tvdb-id",
        "1",
        "--title",
        "test",
        "--tag",
        "1",
        "--tag",
        "2",
        "--monitor",
        "future",
        "--no-search-for-series",
      ]);

      assert_ok!(&result);
      let Some(Command::Sonarr(SonarrCommand::Add(add_command))) = result.unwrap().command else {
        panic!("Unexpected command type");
      };
      assert_eq!(add_command, expected_args);
    }

    #[test]
    fn test_add_tag_requires_arguments() {
      let result = Cli::command().try_get_matches_from(["managarr", "sonarr", "add", "tag"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_add_tag_success() {
      let expected_args = SonarrAddCommand::Tag {
        name: "test".to_owned(),
      };

      let result = Cli::try_parse_from(["managarr", "sonarr", "add", "tag", "--name", "test"]);

      assert_ok!(&result);

      let Some(Command::Sonarr(SonarrCommand::Add(add_command))) = result.unwrap().command else {
        panic!("Unexpected command type");
      };
      assert_eq!(add_command, expected_args);
    }
  }

  mod handler {
    use std::sync::Arc;

    use crate::{
      app::App,
      cli::{CliCommandHandler, sonarr::add_command_handler::SonarrAddCommandHandler},
      models::{
        Serdeable,
        sonarr_models::{
          AddSeriesBody, AddSeriesOptions, SeriesMonitor, SeriesType, SonarrSerdeable,
        },
      },
      network::{MockNetworkTrait, NetworkEvent, sonarr_network::SonarrEvent},
    };

    use super::*;
    use mockall::predicate::eq;

    use crate::models::servarr_models::AddRootFolderBody;
    use serde_json::json;
    use tokio::sync::Mutex;

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
          SonarrEvent::AddRootFolder(expected_add_root_folder_body.clone()).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let add_root_folder_command = SonarrAddCommand::RootFolder {
        root_folder_path: expected_root_folder_path,
      };

      let result =
        SonarrAddCommandHandler::with(&app_arc, add_root_folder_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_add_series_command() {
      let expected_add_series_body = AddSeriesBody {
        tvdb_id: 1,
        title: "test".to_owned(),
        root_folder_path: "/test".to_owned(),
        quality_profile_id: 1,
        language_profile_id: 1,
        series_type: SeriesType::Anime,
        monitored: false,
        tags: vec![1, 2],
        tag_input_string: None,
        season_folder: false,
        add_options: AddSeriesOptions {
          monitor: SeriesMonitor::Future,
          search_for_cutoff_unmet_episodes: false,
          search_for_missing_episodes: false,
        },
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::AddSeries(expected_add_series_body).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let add_series_command = SonarrAddCommand::Series {
        tvdb_id: 1,
        title: "test".to_owned(),
        root_folder_path: "/test".to_owned(),
        quality_profile_id: 1,
        language_profile_id: 1,
        series_type: SeriesType::Anime,
        disable_monitoring: true,
        disable_season_folders: true,
        tag: vec![1, 2],
        monitor: SeriesMonitor::Future,
        no_search_for_series: true,
      };

      let result = SonarrAddCommandHandler::with(&app_arc, add_series_command, &mut mock_network)
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
          SonarrEvent::AddTag(expected_tag_name.clone()).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let add_tag_command = SonarrAddCommand::Tag {
        name: expected_tag_name,
      };

      let result = SonarrAddCommandHandler::with(&app_arc, add_tag_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }
  }
}
