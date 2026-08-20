use std::sync::Arc;

use anyhow::Result;
use clap::Subcommand;
use tokio::sync::Mutex;

use super::{CliCommandHandler, Command};
use crate::{app::App, network::NetworkTrait};

#[cfg(test)]
#[path = "readarr_command_tests.rs"]
mod readarr_command_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum ReadarrCommand {}

impl From<ReadarrCommand> for Command {
  fn from(readarr_command: ReadarrCommand) -> Command {
    Command::Readarr(readarr_command)
  }
}

pub(super) struct ReadarrCliHandler<'a, 'b> {
  app: &'a Arc<Mutex<App<'b>>>,
  command: ReadarrCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, ReadarrCommand> for ReadarrCliHandler<'a, 'b> {
  fn with(
    app: &'a Arc<Mutex<App<'b>>>,
    command: ReadarrCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    ReadarrCliHandler {
      app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<String> {
    match self.command {}
  }
}
