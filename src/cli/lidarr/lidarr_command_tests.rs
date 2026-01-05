#[cfg(test)]
mod tests {
  use crate::cli::{
    lidarr::{list_command_handler::LidarrListCommand, LidarrCommand},
    Command,
  };
  use crate::Cli;
  use clap::CommandFactory;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_lidarr_command_from() {
    let command = LidarrCommand::List(LidarrListCommand::Artists);

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Lidarr(command));
  }

  mod cli {
    use super::*;

    #[test]
    fn test_list_artists_has_no_arg_requirements() {
      let result = Cli::command().try_get_matches_from(["managarr", "lidarr", "list", "artists"]);

      assert_ok!(&result);
    }

    #[test]
    fn test_lidarr_list_subcommand_requires_subcommand() {
      let result = Cli::command().try_get_matches_from(["managarr", "lidarr", "list"]);

      assert_err!(&result);
    }

    #[test]
    fn test_lidarr_delete_subcommand_requires_subcommand() {
      let result = Cli::command().try_get_matches_from(["managarr", "lidarr", "delete"]);

      assert_err!(&result);
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
        lidarr::{
          LidarrCliHandler, LidarrCommand,
          delete_command_handler::LidarrDeleteCommand,
          list_command_handler::LidarrListCommand,
        },
      },
      models::{
        Serdeable,
        lidarr_models::{Artist, DeleteArtistParams, LidarrSerdeable},
      },
      network::{MockNetworkTrait, NetworkEvent, lidarr_network::LidarrEvent},
    };

    #[tokio::test]
    async fn test_lidarr_cli_handler_delegates_delete_commands_to_the_delete_command_handler() {
      let expected_delete_artist_params = DeleteArtistParams {
        id: 1,
        delete_files: true,
        add_import_list_exclusion: true,
      };
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          LidarrEvent::DeleteArtist(expected_delete_artist_params).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let delete_artist_command = LidarrCommand::Delete(LidarrDeleteCommand::Artist {
        artist_id: 1,
        delete_files_from_disk: true,
        add_list_exclusion: true,
      });

      let result = LidarrCliHandler::with(&app_arc, delete_artist_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_lidarr_cli_handler_delegates_list_commands_to_the_list_command_handler() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(LidarrEvent::ListArtists.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Lidarr(LidarrSerdeable::Artists(vec![
            Artist::default(),
          ])))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let list_artists_command = LidarrCommand::List(LidarrListCommand::Artists);

      let result = LidarrCliHandler::with(&app_arc, list_artists_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }
  }
}
