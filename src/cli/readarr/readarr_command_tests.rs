#[cfg(test)]
mod tests {
  mod cli {
    use clap::CommandFactory;
    use clap::error::ErrorKind;
    use pretty_assertions::assert_eq;

    use crate::Cli;

    #[test]
    fn test_readarr_command_requires_a_subcommand() {
      let result = Cli::command().try_get_matches_from(["managarr", "readarr"]);

      assert_err!(&result);
    }

    #[test]
    fn test_start_task_requires_task_name() {
      let result = Cli::command().try_get_matches_from(["managarr", "readarr", "start-task"]);

      assert_err!(&result);
      assert_eq!(
        result.unwrap_err().kind(),
        ErrorKind::MissingRequiredArgument
      );
    }

    #[test]
    fn test_start_task_task_name_validation() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "start-task",
        "--task-name",
        "test",
      ]);

      assert_err!(&result);
      assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    }

    #[test]
    fn test_start_task_requirements_satisfied() {
      let result = Cli::command().try_get_matches_from([
        "managarr",
        "readarr",
        "start-task",
        "--task-name",
        "check-health",
      ]);

      assert_ok!(&result);
    }
  }

  mod handler {
    use std::sync::Arc;

    use mockall::predicate::eq;
    use serde_json::json;
    use tokio::sync::Mutex;

    use crate::models::readarr_models::{ReadarrSerdeable, ReadarrTaskName};
    use crate::{
      app::App,
      cli::{
        CliCommandHandler,
        readarr::{ReadarrCliHandler, ReadarrCommand},
      },
      models::Serdeable,
      network::{MockNetworkTrait, NetworkEvent, readarr_network::ReadarrEvent},
    };

    #[tokio::test]
    async fn test_start_task_command() {
      let expected_task_name = ReadarrTaskName::CheckHealth;
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(
          ReadarrEvent::StartTask(expected_task_name).into(),
        ))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Readarr(ReadarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::test_default()));
      let start_task_command = ReadarrCommand::StartTask {
        task_name: ReadarrTaskName::CheckHealth,
      };

      let result = ReadarrCliHandler::with(&app_arc, start_task_command, &mut mock_network)
        .handle()
        .await;

      assert_ok!(&result);
    }
  }
}
