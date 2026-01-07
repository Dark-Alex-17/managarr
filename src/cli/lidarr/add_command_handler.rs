use std::sync::Arc;

use anyhow::Result;
use clap::{Subcommand, arg};
use tokio::sync::Mutex;

use super::LidarrCommand;
use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  network::{NetworkTrait, lidarr_network::LidarrEvent},
};

#[cfg(test)]
#[path = "add_command_handler_tests.rs"]
mod add_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum LidarrAddCommand {
  #[command(about = "Add new tag")]
  Tag {
    #[arg(long, help = "The name of the tag to be added", required = true)]
    name: String,
  },
}

impl From<LidarrAddCommand> for Command {
  fn from(value: LidarrAddCommand) -> Self {
    Command::Lidarr(LidarrCommand::Add(value))
  }
}

pub(super) struct LidarrAddCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: LidarrAddCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, LidarrAddCommand> for LidarrAddCommandHandler<'a, 'b> {
  fn with(
    app: &'a Arc<Mutex<App<'b>>>,
    command: LidarrAddCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    LidarrAddCommandHandler {
      _app: app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<String> {
    let result = match self.command {
      LidarrAddCommand::Tag { name } => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::AddTag(name).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
    };

    Ok(result)
  }
}
