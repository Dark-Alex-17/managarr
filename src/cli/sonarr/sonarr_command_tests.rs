#[cfg(test)]
mod tests {
  use crate::cli::{
    sonarr::{list_command_handler::SonarrListCommand, SonarrCommand},
    Command,
  };

  #[test]
  fn test_sonarr_command_from() {
    let command = SonarrCommand::List(SonarrListCommand::Series);

    let result = Command::from(command.clone());

    assert_eq!(result, Command::Sonarr(command));
  }

  mod cli {}

  mod handler {
    use std::sync::Arc;

    use mockall::predicate::eq;
    use serde_json::json;
    use tokio::sync::Mutex;

    use crate::{
      app::App,
      cli::{
        sonarr::{
          get_command_handler::SonarrGetCommand, list_command_handler::SonarrListCommand,
          SonarrCliHandler, SonarrCommand,
        },
        CliCommandHandler,
      },
      models::{
        sonarr_models::{Series, SonarrSerdeable},
        Serdeable,
      },
      network::{sonarr_network::SonarrEvent, MockNetworkTrait, NetworkEvent},
    };

    #[tokio::test]
    async fn test_sonarr_cli_handler_delegates_get_commands_to_the_get_command_handler() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(SonarrEvent::GetStatus.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::Value(
            json!({"testResponse": "response"}),
          )))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let get_system_status_command = SonarrCommand::Get(SonarrGetCommand::SystemStatus);

      let result = SonarrCliHandler::with(&app_arc, get_system_status_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_sonarr_cli_handler_delegates_list_commands_to_the_list_command_handler() {
      let mut mock_network = MockNetworkTrait::new();
      mock_network
        .expect_handle_network_event()
        .with(eq::<NetworkEvent>(SonarrEvent::ListSeries.into()))
        .times(1)
        .returning(|_| {
          Ok(Serdeable::Sonarr(SonarrSerdeable::SeriesVec(vec![
            Series::default(),
          ])))
        });
      let app_arc = Arc::new(Mutex::new(App::default()));
      let list_series_command = SonarrCommand::List(SonarrListCommand::Series);

      let result = SonarrCliHandler::with(&app_arc, list_series_command, &mut mock_network)
        .handle()
        .await;

      assert!(result.is_ok());
    }
  }
}
