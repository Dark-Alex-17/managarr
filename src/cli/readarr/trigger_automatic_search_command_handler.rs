use std::sync::Arc;

use anyhow::Result;
use clap::Subcommand;
use tokio::sync::Mutex;

use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  network::{NetworkTrait, readarr_network::ReadarrEvent},
};

use super::ReadarrCommand;

#[cfg(test)]
#[path = "trigger_automatic_search_command_handler_tests.rs"]
mod trigger_automatic_search_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum ReadarrTriggerAutomaticSearchCommand {
  #[command(about = "Trigger an automatic search for the author with the specified ID")]
  Author {
    #[arg(
      long,
      help = "The ID of the author you want to trigger an automatic search for",
      required = true
    )]
    author_id: i64,
  },
}

impl From<ReadarrTriggerAutomaticSearchCommand> for Command {
  fn from(value: ReadarrTriggerAutomaticSearchCommand) -> Self {
    Command::Readarr(ReadarrCommand::TriggerAutomaticSearch(value))
  }
}

pub(super) struct ReadarrTriggerAutomaticSearchCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: ReadarrTriggerAutomaticSearchCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, ReadarrTriggerAutomaticSearchCommand>
  for ReadarrTriggerAutomaticSearchCommandHandler<'a, 'b>
{
  fn with(
    _app: &'a Arc<Mutex<App<'b>>>,
    command: ReadarrTriggerAutomaticSearchCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    ReadarrTriggerAutomaticSearchCommandHandler {
      _app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<String> {
    let result = match self.command {
      ReadarrTriggerAutomaticSearchCommand::Author { author_id } => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::TriggerAutomaticAuthorSearch(author_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
    };

    Ok(result)
  }
}
