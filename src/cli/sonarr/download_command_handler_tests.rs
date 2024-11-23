#[cfg(test)]
mod tests {
  use crate::{
    cli::{
      sonarr::{download_command_handler::SonarrDownloadCommand, SonarrCommand},
      Command,
    },
    Cli,
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
    use rstest::rstest;

    #[rstest]
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

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[rstest]
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

      assert!(result.is_err());
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[rstest]
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

      assert!(result.is_err());
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

      assert!(result.is_ok());
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
        sonarr::download_command_handler::{SonarrDownloadCommand, SonarrDownloadCommandHandler},
        CliCommandHandler,
      },
      models::{
        sonarr_models::{SonarrReleaseDownloadBody, SonarrSerdeable},
        Serdeable,
      },
      network::{sonarr_network::SonarrEvent, MockNetworkTrait, NetworkEvent},
    };

    #[tokio::test]
    async fn test_download_release_command() {
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
      let app_arc = Arc::new(Mutex::new(App::default()));
      let download_release_command = SonarrDownloadCommand::Series {
        guid: "guid".to_owned(),
        indexer_id: 1,
        series_id: 1,
      };

      let result =
        SonarrDownloadCommandHandler::with(&app_arc, download_release_command, &mut mock_network)
          .handle()
          .await;

      assert!(result.is_ok());
    }
  }
}
