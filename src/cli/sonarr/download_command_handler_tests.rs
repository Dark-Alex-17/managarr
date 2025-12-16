#[cfg(test)]
mod tests {
  use crate::{
    Cli,
    cli::{
      Command,
      sonarr::{SonarrCommand, download_command_handler::SonarrDownloadCommand},
    },
  };
  use clap::CommandFactory;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_sonarr_download_command_from() {
    let command = SonarrDownloadCommand::Series {
      guid: "Test".to_owned(),
      indexer_id: 1,
      series_id: 1,
    };

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Sonarr(SonarrCommand::Download(command)));
  }

  mod cli {
    use super::*;
    use clap::error::ErrorKind;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_download_series_requires_series_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "download",
        "series",
        "--indexer-id",
        "1",
        "--guid",
        "1",
      ]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_download_series_requires_guid() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "download",
        "series",
        "--indexer-id",
        "1",
        "--series-id",
        "1",
      ]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_download_series_requires_indexer_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "download",
        "series",
        "--guid",
        "1",
        "--series-id",
        "1",
      ]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_download_series_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "download",
        "series",
        "--guid",
        "1",
        "--series-id",
        "1",
        "--indexer-id",
        "1",
      ]);

      assert_ok!(&result);
    }

    #[test]
    fn test_download_season_requires_series_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "download",
        "season",
        "--indexer-id",
        "1",
        "--season-number",
        "1",
        "--guid",
        "1",
      ]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_download_season_requires_season_number() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "download",
        "season",
        "--indexer-id",
        "1",
        "--series-id",
        "1",
        "--guid",
        "1",
      ]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_download_season_requires_guid() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "download",
        "season",
        "--indexer-id",
        "1",
        "--season-number",
        "1",
        "--series-id",
        "1",
      ]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_download_season_requires_indexer_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "download",
        "season",
        "--guid",
        "1",
        "--season-number",
        "1",
        "--series-id",
        "1",
      ]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_download_season_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "download",
        "season",
        "--guid",
        "1",
        "--series-id",
        "1",
        "--season-number",
        "1",
        "--indexer-id",
        "1",
      ]);

      assert_ok!(&result);
    }

    #[test]
    fn test_download_episode_requires_episode_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "download",
        "episode",
        "--indexer-id",
        "1",
        "--guid",
        "1",
      ]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_download_episode_requires_guid() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "download",
        "episode",
        "--indexer-id",
        "1",
        "--episode-id",
        "1",
      ]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_download_episode_requires_indexer_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "download",
        "episode",
        "--guid",
        "1",
        "--episode-id",
        "1",
      ]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_download_episode_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "sonarr",
        "download",
        "episode",
        "--guid",
        "1",
        "--episode-id",
        "1",
        "--indexer-id",
        "1",
      ]);

      assert_ok!(&result);
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
        sonarr::download_command_handler::{SonarrDownloadCommand, SonarrDownloadCommandHandler},
      },
      models::{
        Serdeable,
        sonarr_models::{SonarrReleaseDownloadBody, SonarrSerdeable},
      },
      network::{MockNetworkTrait, NetworkEvent, sonarr_network::SonarrEvent},
    };

    #[tokio::test]
    async fn test_download_series_release_command() {
      let expected_release_download_body = SonarrReleaseDownloadBody {
        guid: "guid".to_owned(),
        indexer_id: 1,
        series_id: Some(1),
        ..SonarrReleaseDownloadBody::default()
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::DownloadRelease(expected_release_download_body).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let download_release_command = SonarrDownloadCommand::Series {
        guid: "guid".to_owned(),
        indexer_id: 1,
        series_id: 1,
      };

      let result =
        SonarrDownloadCommandHandler::with(&app_arc, download_release_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_download_season_release_command() {
      let expected_release_download_body = SonarrReleaseDownloadBody {
        guid: "guid".to_owned(),
        indexer_id: 1,
        series_id: Some(1),
        season_number: Some(1),
        ..SonarrReleaseDownloadBody::default()
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::DownloadRelease(expected_release_download_body).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let download_release_command = SonarrDownloadCommand::Season {
        guid: "guid".to_owned(),
        indexer_id: 1,
        series_id: 1,
        season_number: 1,
      };

      let result =
        SonarrDownloadCommandHandler::with(&app_arc, download_release_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_download_episode_release_command() {
      let expected_release_download_body = SonarrReleaseDownloadBody {
        guid: "guid".to_owned(),
        indexer_id: 1,
        episode_id: Some(1),
        ..SonarrReleaseDownloadBody::default()
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          SonarrEvent::DownloadRelease(expected_release_download_body).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let download_release_command = SonarrDownloadCommand::Episode {
        guid: "guid".to_owned(),
        indexer_id: 1,
        episode_id: 1,
      };

      let result =
        SonarrDownloadCommandHandler::with(&app_arc, download_release_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }
  }
}
