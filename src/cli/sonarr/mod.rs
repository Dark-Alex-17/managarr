use std::sync::Arc;

use anyhow::Result;
use clap::Subcommand;
use get_command_handler::{SonarrGetCommand, SonarrGetCommandHandler};
use list_command_handler::{SonarrListCommand, SonarrListCommandHandler};
use tokio::sync::Mutex;

use crate::{app::App, network::NetworkTrait};

use super::{CliCommandHandler, Command};

mod get_command_handler;
mod list_command_handler;

#[cfg(test)]
#[path = "sonarr_command_tests.rs"]
mod sonarr_command_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum SonarrCommand {
  #[command(
    subcommand,
    about = "Commands to fetch details of the resources in your Sonarr instance"
  )]
  Get(SonarrGetCommand),
  #[command(
    subcommand,
    about = "Commands to list attributes from your Sonarr instance"
  )]
  List(SonarrListCommand),
}

impl From<SonarrCommand> for Command {
  fn from(sonarr_command: SonarrCommand) -> Command {
    Command::Sonarr(sonarr_command)
  }
}

pub(super) struct SonarrCliHandler<'a, 'b> {
  app: &'a Arc<Mutex<App<'b>>>,
  command: SonarrCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, SonarrCommand> for SonarrCliHandler<'a, 'b> {
  fn with(
    app: &'a Arc<Mutex<App<'b>>>,
    command: SonarrCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    SonarrCliHandler {
      app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<()> {
    match self.command {
      SonarrCommand::Get(get_command) => {
        SonarrGetCommandHandler::with(self.app, get_command, self.network)
          .handle()
          .await?
      }
      SonarrCommand::List(list_command) => {
        SonarrListCommandHandler::with(self.app, list_command, self.network)
          .handle()
          .await?
      }
    }

    Ok(())
  }
}
