#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;

  use crate::Cli;
  use crate::cli::{
    Command,
    readarr::{
      ReadarrCommand,
      trigger_automatic_search_command_handler::ReadarrTriggerAutomaticSearchCommand,
    },
  };
  use clap::CommandFactory;

  #[test]
  fn test_readarr_trigger_automatic_search_command_from() {
    let command = ReadarrTriggerAutomaticSearchCommand::Author { author_id: 1 };

    let result = Command::from(command.clone());

    assert_eq!(
      result,
      Command::Readarr(ReadarrCommand::TriggerAutomaticSearch(command))
    );
  }

  mod cli {
    use super::*;
    use clap::Parser;
    use clap::error::ErrorKind;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_trigger_automatic_author_search_requires_author_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "trigger-automatic-search",
        "author",
      ]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_trigger_automatic_author_search_with_author_id() {
      let expected_args = ReadarrTriggerAutomaticSearchCommand::Author { author_id: 1 };

      let result = Cli::try_parse_from([
        "managarr",
        "readarr",
        "trigger-automatic-search",
        "author",
        "--author-id",
        "1",
      ]);

      assert_ok!(&result);
      let Some(Command::Readarr(ReadarrCommand::TriggerAutomaticSearch(
        trigger_automatic_search_command,
      ))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(trigger_automatic_search_command, expected_args);
    }

    #[test]
    fn test_trigger_automatic_book_search_requires_book_id() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "trigger-automatic-search",
        "book",
      ]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_trigger_automatic_book_search_with_book_id() {
      let expected_args = ReadarrTriggerAutomaticSearchCommand::Book { book_id: 1 };

      let result = Cli::try_parse_from([
        "managarr",
        "readarr",
        "trigger-automatic-search",
        "book",
        "--book-id",
        "1",
      ]);

      assert_ok!(&result);
      let Some(Command::Readarr(ReadarrCommand::TriggerAutomaticSearch(
        trigger_automatic_search_command,
      ))) = result.unwrap().command
      else {
        panic!("Unexpected command type");
      };
      assert_eq!(trigger_automatic_search_command, expected_args);
    }
  }

  mod handler {
    use std::sync::Arc;

    use mockall::predicate::eq;
    use serde_json::json;
    use tokio::sync::Mutex;

    use crate::cli::readarr::trigger_automatic_search_command_handler::{
      ReadarrTriggerAutomaticSearchCommand, ReadarrTriggerAutomaticSearchCommandHandler,
    };
    use crate::{app::App, cli::CliCommandHandler};
    use crate::{
      models::{Serdeable, readarr_models::ReadarrSerdeable},
      network::{MockNetworkTrait, NetworkEvent, readarr_network::ReadarrEvent},
    };

    #[tokio::test]
    async fn test_handle_trigger_automatic_author_search_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::TriggerAutomaticAuthorSearch(1).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let trigger_automatic_search_command =
        ReadarrTriggerAutomaticSearchCommand::Author { author_id: 1 };

      let result = ReadarrTriggerAutomaticSearchCommandHandler::with(
        &app_arc,
        trigger_automatic_search_command,
        &mut mock_network,
      )
      .handle()
      .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_trigger_automatic_book_search_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::TriggerAutomaticBookSearch(1).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let trigger_automatic_search_command =
        ReadarrTriggerAutomaticSearchCommand::Book { book_id: 1 };

      let result = ReadarrTriggerAutomaticSearchCommandHandler::with(
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
