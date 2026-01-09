#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;

  use crate::Cli;
  use crate::cli::{
    Command,
    lidarr::{LidarrCommand, refresh_command_handler::LidarrRefreshCommand},
  };
  use clap::CommandFactory;

  #[test]
  fn test_lidarr_refresh_command_from() {
    let command = LidarrRefreshCommand::AllArtists;

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Lidarr(LidarrCommand::Refresh(command)));
  }

  mod cli {
    use super::*;
    use clap::{Parser, error::ErrorKind};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_refresh_all_artists_has_no_arg_requirements() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "lidarr", "refresh", "all-artists"]);

      assert_ok!(&result);
    }

    #[test]
    fn test_refresh_artist_requires_artist_id() {
      let result = Cli::command().try_get_matches_from(["managarr", "lidarr", "refresh", "artist"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_refresh_artist_with_artist_id() {
      let expected_args = LidarrRefreshCommand::Artist { artist_id: 1 };
      let result = Cli::try_parse_from([
        "managarr",
        "lidarr",
        "refresh",
        "artist",
        "--artist-id",
        "1",
      ]);

      assert_ok!(&result);
      let Some(Command::Lidarr(LidarrCommand::Refresh(refresh_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(refresh_command, expected_args);
    }
  }

  mod handler {
    use std::sync::Arc;

    use mockall::predicate::eq;
    use serde_json::json;
    use tokio::sync::Mutex;

    use crate::{app::App, cli::lidarr::refresh_command_handler::LidarrRefreshCommandHandler};
    use crate::{
      cli::{CliCommandHandler, lidarr::refresh_command_handler::LidarrRefreshCommand},
      network::lidarr_network::LidarrEvent,
    };
    use crate::{
      models::{Serdeable, lidarr_models::LidarrSerdeable},
      network::{MockNetworkTrait, NetworkEvent},
    };

    #[tokio::test]
    async fn test_handle_refresh_all_artists_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(LidarrEvent::UpdateAllArtists.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let refresh_command = LidarrRefreshCommand::AllArtists;

      let result = LidarrRefreshCommandHandler::with(&app_arc, refresh_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_refresh_artist_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          LidarrEvent::UpdateAndScanArtist(1).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let refresh_command = LidarrRefreshCommand::Artist { artist_id: 1 };

      let result = LidarrRefreshCommandHandler::with(&app_arc, refresh_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }
  }
}
