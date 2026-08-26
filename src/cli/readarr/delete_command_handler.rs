use std::sync::Arc;

use anyhow::Result;
use clap::Subcommand;
use tokio::sync::Mutex;

use super::ReadarrCommand;
use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  network::{NetworkTrait, readarr_network::ReadarrEvent},
};

#[cfg(test)]
#[path = "delete_command_handler_tests.rs"]
mod delete_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum ReadarrDeleteCommand {
  #[command(about = "Delete the tag with the specified ID")]
  Tag {
    #[arg(long, help = "The ID of the tag to delete", required = true)]
    tag_id: i64,
  },
}

impl From<ReadarrDeleteCommand> for Command {
  fn from(value: ReadarrDeleteCommand) -> Self {
    Command::Readarr(ReadarrCommand::Delete(value))
  }
}

pub(super) struct ReadarrDeleteCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: ReadarrDeleteCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, ReadarrDeleteCommand>
  for ReadarrDeleteCommandHandler<'a, 'b>
{
  fn with(
    app: &'a Arc<Mutex<App<'b>>>,
    command: ReadarrDeleteCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    ReadarrDeleteCommandHandler {
      _app: app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<String> {
    let result = match self.command {
      ReadarrDeleteCommand::Tag { tag_id } => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::DeleteTag(tag_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
    };

    Ok(result)
  }
}
