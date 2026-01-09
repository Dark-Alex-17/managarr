#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;

  use crate::Cli;
  use crate::cli::{
    Command,
    lidarr::{
      LidarrCommand, trigger_automatic_search_command_handler::LidarrTriggerAutomaticSearchCommand,
    },
  };
  use clap::CommandFactory;

  #[test]
  fn test_lidarr_trigger_automatic_search_command_from() {
    let command = LidarrTriggerAutomaticSearchCommand::Artist { artist_id: 1 };

    let result = Command::from(command.clone());

    assert_eq!(
      result,
      Command::Lidarr(LidarrCommand::TriggerAutomaticSearch(command))
    );
  }

  mod cli {
    use super::*;
    use clap::error::ErrorKind;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_trigger_automatic_artist_search_requires_artist_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "lidarr",
        "trigger-automatic-search",
        "artist",
      ]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_trigger_automatic_artist_search_with_artist_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "lidarr",
        "trigger-automatic-search",
        "artist",
        "--artist-id",
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

    use crate::cli::lidarr::trigger_automatic_search_command_handler::{
      LidarrTriggerAutomaticSearchCommand, LidarrTriggerAutomaticSearchCommandHandler,
    };
    use crate::{app::App, cli::CliCommandHandler};
    use crate::{
      models::{Serdeable, lidarr_models::LidarrSerdeable},
      network::{MockNetworkTrait, NetworkEvent, lidarr_network::LidarrEvent},
    };

    #[tokio::test]
    async fn test_handle_trigger_automatic_artist_search_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          LidarrEvent::TriggerAutomaticArtistSearch(1).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let trigger_automatic_search_command =
        LidarrTriggerAutomaticSearchCommand::Artist { artist_id: 1 };

      let result = LidarrTriggerAutomaticSearchCommandHandler::with(
        &app_arc,
        trigger_automatic_search_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert_ok!(&result);
    }
  }
}
