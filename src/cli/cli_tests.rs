#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use clap::{error::ErrorKind, CommandFactory};
  use mockall::predicate::eq;
  use rstest::rstest;
  use serde_json::json;
  use tokio::sync::Mutex;

  use crate::{
    app::App,
    cli::{handle_command, mutex_flags_or_option, radarr::RadarrCommand},
    models::{
      radarr_models::{BlocklistItem, BlocklistResponse, RadarrSerdeable},
      Serdeable,
    },
    network::{radarr_network::RadarrEvent, MockNetworkTrait, NetworkEvent},
    Cli,
  };
  use pretty_assertions::assert_eq;

  #[test]
  fn test_radarr_subcommand_requires_subcommand() {
    let result = Cli::command().try_get_matches_from(["managarr", "radarr"]);

    assert!(result.is_err());
    assert_eq!(
      result.unwrap_err().kind(),
      ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
    );
  }

  #[test]
  fn test_radarr_subcommand_delegates_to_radarr() {
    let result =
      Cli::command().try_get_matches_from(["managarr", "radarr", "get", "all-indexer-settings"]);

    assert!(result.is_ok());
  }

  #[test]
  fn test_completions_requires_argument() {
    let result = Cli::command().try_get_matches_from(["managarr", "completions"]);

    assert!(result.is_err());
    assert_eq!(
      result.unwrap_err().kind(),
      ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
    );
  }

  #[test]
  fn test_completions_invalid_argument() {
    let result = Cli::command().try_get_matches_from(["managarr", "completions", "test"]);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
  }

  #[test]
  fn test_completions_satisfied_with_argument() {
    let result = Cli::command().try_get_matches_from(["managarr", "completions", "bash"]);

    assert!(result.is_ok());
  }

  #[rstest]
  #[case(false, false, None)]
  #[case(false, true, Some(false))]
  #[case(true, false, Some(true))]
  fn test_mutex_flags_or_option(
    #[case] positive: bool,
    #[case] negative: bool,
    #[case] expected_output: Option<bool>,
  ) {
    let result = mutex_flags_or_option(positive, negative);

    assert_eq!(result, expected_output);
  }

  #[rstest]
  #[case(false, false, true, true)]
  #[case(false, false, false, false)]
  #[case(false, true, true, false)]
  #[case(true, false, false, true)]
  fn test_mutex_flags_or_default(
    #[case] positive: bool,
    #[case] negative: bool,
    #[case] default_value: bool,
    #[case] expected_output: bool,
  ) {
    use crate::cli::mutex_flags_or_default;

    let result = mutex_flags_or_default(positive, negative, default_value);

    assert_eq!(result, expected_output);
  }

  #[tokio::test]
  async fn test_cli_handler_delegates_radarr_commands_to_the_radarr_cli_handler() {
    let mut mock_network = MockNetworkTrait::new();
    mock_network
      .expect_handle_network_event()
      .with(eq::<NetworkEvent>(RadarrEvent::GetBlocklist.into()))
      .times(1)
      .returning(|_| {
        Ok(Serdeable::Radarr(RadarrSerdeable::BlocklistResponse(
          BlocklistResponse {
            records: vec![BlocklistItem::default()],
          },
        )))
      });
    mock_network
      .expect_handle_network_event()
      .with(eq::<NetworkEvent>(RadarrEvent::ClearBlocklist.into()))
      .times(1)
      .returning(|_| {
        Ok(Serdeable::Radarr(RadarrSerdeable::Value(
          json!({"testResponse": "response"}),
        )))
      });
    let app_arc = Arc::new(Mutex::new(App::default()));
    let claer_blocklist_command = RadarrCommand::ClearBlocklist.into();

    let result = handle_command(&app_arc, claer_blocklist_command, &mut mock_network).await;

    assert!(result.is_ok());
  }
}
