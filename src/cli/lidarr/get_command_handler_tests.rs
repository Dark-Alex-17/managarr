#[cfg(test)]
mod tests {
  use crate::Cli;
  use crate::cli::{
    Command,
    lidarr::{LidarrCommand, get_command_handler::LidarrGetCommand},
  };
  use clap::CommandFactory;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_lidarr_get_command_from() {
    let command = LidarrGetCommand::SystemStatus;

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Lidarr(LidarrCommand::Get(command)));
  }

  mod cli {
    use clap::error::ErrorKind;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_artist_details_requires_artist_id() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "lidarr", "get", "artist-details"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_artist_details_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "lidarr",
        "get",
        "artist-details",
        "--artist-id",
        "1",
      ]);

      assert_ok!(&result);
    }

    #[test]
    fn test_system_status_has_no_arg_requirements() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "lidarr", "get", "system-status"]);

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
        lidarr::get_command_handler::{LidarrGetCommand, LidarrGetCommandHandler},
      },
      models::{Serdeable, lidarr_models::LidarrSerdeable},
      network::{MockNetworkTrait, NetworkEvent, lidarr_network::LidarrEvent},
    };

    #[tokio::test]
    async fn test_handle_get_artist_details_command() {
      let expected_artist_id = 1;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          LidarrEvent::GetArtistDetails(expected_artist_id).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let get_artist_details_command = LidarrGetCommand::ArtistDetails { artist_id: 1 };

      let result =
        LidarrGetCommandHandler::with(&app_arc, get_artist_details_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_get_system_status_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(LidarrEvent::GetStatus.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let get_system_status_command = LidarrGetCommand::SystemStatus;

      let result =
        LidarrGetCommandHandler::with(&app_arc, get_system_status_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }
  }
}
