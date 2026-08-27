#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;

  use crate::Cli;
  use crate::cli::{
    Command,
    readarr::{ReadarrCommand, refresh_command_handler::ReadarrRefreshCommand},
  };
  use clap::CommandFactory;

  #[test]
  fn test_readarr_refresh_command_from() {
    let command = ReadarrRefreshCommand::Author { author_id: 1 };

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Readarr(ReadarrCommand::Refresh(command)));
  }

  mod cli {
    use super::*;
    use clap::Parser;
    use clap::error::ErrorKind;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_refresh_all_authors_has_no_arg_requirements() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "readarr", "refresh", "all-authors"]);

      assert_ok!(&result);
    }

    #[test]
    fn test_refresh_author_requires_author_id() {
      let result =
        Cli::command().try_get_matches_from(["managarr", "readarr", "refresh", "author"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_refresh_author_with_author_id() {
      let expected_args = ReadarrRefreshCommand::Author { author_id: 1 };

      let result = Cli::try_parse_from([
        "managarr",
        "readarr",
        "refresh",
        "author",
        "--author-id",
        "1",
      ]);

      assert_ok!(&result);
      let Some(Command::Readarr(ReadarrCommand::Refresh(refresh_command))) =
        result.unwrap().command
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

    use crate::cli::readarr::refresh_command_handler::{
      ReadarrRefreshCommand, ReadarrRefreshCommandHandler,
    };
    use crate::{app::App, cli::CliCommandHandler};
    use crate::{
      models::{Serdeable, readarr_models::ReadarrSerdeable},
      network::{MockNetworkTrait, NetworkEvent, readarr_network::ReadarrEvent},
    };

    #[tokio::test]
    async fn test_handle_refresh_all_authors_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(ReadarrEvent::UpdateAllAuthors.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let refresh_command = ReadarrRefreshCommand::AllAuthors;

      let result = ReadarrRefreshCommandHandler::with(&app_arc, refresh_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }

    #[tokio::test]
    async fn test_handle_refresh_author_command() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::UpdateAndScanAuthor(1).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let refresh_command = ReadarrRefreshCommand::Author { author_id: 1 };

      let result = ReadarrRefreshCommandHandler::with(&app_arc, refresh_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }
  }
}
