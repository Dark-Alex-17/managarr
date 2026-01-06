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

    #[test]
    fn test_refresh_all_artists_has_no_arg_requirements() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "lidarr", "refresh", "all-artists"]);

      assert_ok!(&result);
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
  }
}
