#[cfg(test)]
mod tests {
  use crate::Cli;
  use crate::cli::{
    Command,
    lidarr::{LidarrCommand, list_command_handler::LidarrListCommand},
  };
  use clap::CommandFactory;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_lidarr_list_command_from() {
    let command = LidarrListCommand::Artists;

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Lidarr(LidarrCommand::List(command)));
  }

  mod cli {
    use super::*;

    #[test]
    fn test_list_artists_has_no_arg_requirements() {
      let result = Cli::command().try_get_matches_from(["managarr", "lidarr", "list", "artists"]);

      assert_ok!(&result);
    }
  }

  mod handler {
    use std::sync::Arc;

    use mockall::predicate::eq;
    use serde_json::json;
    use tokio::sync::Mutex;

    use crate::cli::CliCommandHandler;
    use crate::cli::lidarr::list_command_handler::{LidarrListCommand, LidarrListCommandHandler};
    use crate::models::Serdeable;
    use crate::models::lidarr_models::LidarrSerdeable;
    use crate::network::lidarr_network::LidarrEvent;
    use crate::{
      app::App,
      network::{MockNetworkTrait, NetworkEvent},
    };

    #[tokio::test]
    async fn test_handle_list_artists_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(LidarrEvent::ListArtists.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));

      let result =
        LidarrListCommandHandler::with(&app_arc, LidarrListCommand::Artists, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }
  }
}
