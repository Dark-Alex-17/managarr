#[cfg(test)]
mod tests {
  use clap::{CommandFactory, Parser, error::ErrorKind};

  use crate::{
    Cli,
    cli::{
      Command,
      readarr::{ReadarrCommand, add_command_handler::ReadarrAddCommand},
    },
  };
  use pretty_assertions::assert_eq;

  #[test]
  fn test_readarr_add_command_from() {
    let command = ReadarrAddCommand::Tag {
      name: String::new(),
    };

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Readarr(ReadarrCommand::Add(command)));
  }

  mod cli {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_add_tag_requires_arguments() {
      let result = Cli::command().try_get_matches_from(["managarr", "readarr", "add", "tag"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_add_tag_success() {
      let expected_args = ReadarrAddCommand::Tag {
        name: "test".to_owned(),
      };

      let result = Cli::try_parse_from(["managarr", "readarr", "add", "tag", "--name", "test"]);

      assert_ok!(&result);

      let Some(Command::Readarr(ReadarrCommand::Add(add_command))) = result.unwrap().command else {
        panic!("Unexpected command type");
      };
      assert_eq!(add_command, expected_args);
    }
  }

  mod handler {
    use std::sync::Arc;

    use mockall::predicate::eq;
    use serde_json::json;
    use tokio::sync::Mutex;

    use crate::cli::CliCommandHandler;
    use crate::cli::readarr::add_command_handler::{ReadarrAddCommand, ReadarrAddCommandHandler};
    use crate::models::Serdeable;
    use crate::models::readarr_models::ReadarrSerdeable;
    use crate::network::readarr_network::ReadarrEvent;
    use crate::{
      app::App,
      network::{MockNetworkTrait, NetworkEvent},
    };

    #[tokio::test]
    async fn test_handle_add_tag_command() {
      let expected_tag_name = "test".to_owned();
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::AddTag(expected_tag_name.clone()).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let add_tag_command = ReadarrAddCommand::Tag {
        name: expected_tag_name,
      };

      let result = ReadarrAddCommandHandler::with(&app_arc, add_tag_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }
  }
}
