#[cfg(test)]
mod tests {
  use clap::{CommandFactory, Parser, error::ErrorKind};

  use crate::{
    Cli,
    cli::{
      Command,
      readarr::{ReadarrCommand, delete_command_handler::ReadarrDeleteCommand},
    },
  };
  use pretty_assertions::assert_eq;

  #[test]
  fn test_readarr_delete_command_from() {
    let command = ReadarrDeleteCommand::Tag { tag_id: 1 };

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Readarr(ReadarrCommand::Delete(command)));
  }

  mod cli {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_delete_tag_requires_arguments() {
      let result = Cli::command().try_get_matches_from(["managarr", "readarr", "delete", "tag"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_delete_tag_tag_id_requires_a_number() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "delete",
        "tag",
        "--tag-id",
        "not_a_number",
      ]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::ValueValidation);
    }

    #[test]
    fn test_delete_tag_success() {
      let expected_args = ReadarrDeleteCommand::Tag { tag_id: 1 };

      let result = Cli::try_parse_from(["managarr", "readarr", "delete", "tag", "--tag-id", "1"]);

      assert_ok!(&result);

      let Some(Command::Readarr(ReadarrCommand::Delete(delete_command))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(delete_command, expected_args);
    }
  }

  mod handler {
    use std::sync::Arc;

    use mockall::predicate::eq;
    use serde_json::json;
    use tokio::sync::Mutex;

    use crate::cli::CliCommandHandler;
    use crate::cli::readarr::delete_command_handler::{
      ReadarrDeleteCommand, ReadarrDeleteCommandHandler,
    };
    use crate::models::Serdeable;
    use crate::models::readarr_models::ReadarrSerdeable;
    use crate::network::readarr_network::ReadarrEvent;
    use crate::{
      app::App,
      network::{MockNetworkTrait, NetworkEvent},
    };

    #[tokio::test]
    async fn test_handle_delete_tag_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(ReadarrEvent::DeleteTag(1).into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let delete_tag_command = ReadarrDeleteCommand::Tag { tag_id: 1 };

      let result =
        ReadarrDeleteCommandHandler::with(&app_arc, delete_tag_command, &mut mock_network)
          .handle()
          .await;

      assert_ok!(&result);
    }
  }
}
