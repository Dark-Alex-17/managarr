#[cfg(test)]
mod tests {
  use crate::cli::Command;
  use crate::cli::lidarr::LidarrCommand;
  use crate::cli::lidarr::manual_search_command_handler::LidarrManualSearchCommand;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_lidarr_manual_search_command_from() {
    let command = LidarrManualSearchCommand::Discography { artist_id: 1 };

    let result = Command::from(command.clone());

    assert_eq!(
      result,
      Command::Lidarr(LidarrCommand::ManualSearch(command))
    );
  }

  mod cli {
    use crate::Cli;
    use clap::CommandFactory;
    use clap::error::ErrorKind;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_manual_album_search_requires_artist_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "lidarr",
        "manual-search",
        "album",
        "--album-id",
        "1",
      ]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_manual_album_search_requires_album_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "lidarr",
        "manual-search",
        "album",
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
    fn test_manual_album_search_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "lidarr",
        "manual-search",
        "album",
        "--artist-id",
        "1",
        "--album-id",
        "1",
      ]);

      assert_ok!(&result);
    }

    #[test]
    fn test_manual_discography_search_requires_artist_id() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "lidarr", "manual-search", "discography"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_manual_discography_search_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "lidarr",
        "manual-search",
        "discography",
        "--artist-id",
        "1",
      ]);

      assert_ok!(&result);
    }
  }

  mod handler {
    use crate::app::App;
    use crate::cli::CliCommandHandler;
    use crate::cli::lidarr::manual_search_command_handler::{
      LidarrManualSearchCommand, LidarrManualSearchCommandHandler,
    };
    use crate::models::Serdeable;
    use crate::models::lidarr_models::LidarrSerdeable;
    use crate::network::lidarr_network::LidarrEvent;
    use crate::network::{MockNetworkTrait, NetworkEvent};
    use mockall::predicate::eq;
    use serde_json::json;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    #[tokio::test]
    async fn test_manual_album_search_command() {
      let expected_artist_id = 1;
      let expected_album_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          LidarrEvent::GetAlbumReleases(expected_artist_id, expected_album_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let manual_album_search_command = LidarrManualSearchCommand::Album {
        artist_id: 1,
        album_id: 1,
      };

      let result = LidarrManualSearchCommandHandler::with(
        &app_arc,
        manual_album_search_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_manual_discography_search_command() {
      let expected_artist_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          LidarrEvent::GetDiscographyReleases(expected_artist_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let manual_discography_search_command =
        LidarrManualSearchCommand::Discography { artist_id: 1 };

      let result = LidarrManualSearchCommandHandler::with(
        &app_arc,
        manual_discography_search_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert_ok!(&result);
    }
  }
}
