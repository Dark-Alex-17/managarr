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
#[path = "get_command_handler_tests.rs"]
mod get_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum ReadarrGetCommand {
  #[command(about = "Get detailed information for the author with the given ID")]
  AuthorDetails {
    #[arg(
      long,
      help = "The Readarr ID of the author whose details you wish to fetch",
      required = true
    )]
    author_id: i64,
  },
  #[command(about = "Fetch the host config for your Readarr instance")]
  HostConfig,
  #[command(about = "Fetch the security config for your Readarr instance")]
  SecurityConfig,
  #[command(about = "Get the system status")]
  SystemStatus,
}

impl From<ReadarrGetCommand> for Command {
  fn from(value: ReadarrGetCommand) -> Self {
    Command::Readarr(ReadarrCommand::Get(value))
  }
}

pub(super) struct ReadarrGetCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: ReadarrGetCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, ReadarrGetCommand> for ReadarrGetCommandHandler<'a, 'b> {
  fn with(
    _app: &'a Arc<Mutex<App<'b>>>,
    command: ReadarrGetCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    ReadarrGetCommandHandler {
      _app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<String> {
    let result = match self.command {
      ReadarrGetCommand::AuthorDetails { author_id } => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::GetAuthorDetails(author_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      ReadarrGetCommand::HostConfig => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::GetHostConfig.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      ReadarrGetCommand::SecurityConfig => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::GetSecurityConfig.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      ReadarrGetCommand::SystemStatus => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::GetStatus.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
    };

    Ok(result)
  }
}
