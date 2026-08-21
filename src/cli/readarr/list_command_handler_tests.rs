#[cfg(test)]
mod tests {
  use crate::Cli;
  use crate::cli::{
    Command,
    readarr::{ReadarrCommand, list_command_handler::ReadarrListCommand},
  };
  use clap::CommandFactory;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_readarr_list_command_from() {
    let command = ReadarrListCommand::DiskSpace;

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Readarr(ReadarrCommand::List(command)));
  }

  mod cli {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_list_commands_have_no_arg_requirements(
      #[values("disk-space", "updates")] subcommand: &str,
    ) {
      let result = Cli::command().try_get_matches_from(["managarr", "readarr", "list", subcommand]);

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
        readarr::list_command_handler::{ReadarrListCommand, ReadarrListCommandHandler},
      },
      models::{Serdeable, readarr_models::ReadarrSerdeable},
      network::{MockNetworkTrait, NetworkEvent, readarr_network::ReadarrEvent},
    };

    #[tokio::test]
    async fn test_handle_list_disk_space_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(ReadarrEvent::GetDiskSpace.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let list_disk_space_command = ReadarrListCommand::DiskSpace;

      let result =
        ReadarrListCommandHandler::with(&app_arc, list_disk_space_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_list_updates_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(ReadarrEvent::GetUpdates.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let list_updates_command = ReadarrListCommand::Updates;

      let result =
        ReadarrListCommandHandler::with(&app_arc, list_updates_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }
  }
}
