use std::sync::Arc;

use anyhow::Result;
use clap::Subcommand;
use tokio::sync::Mutex;

use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  execute_network_event,
  network::{sonarr_network::SonarrEvent, NetworkTrait},
};

use super::SonarrCommand;

#[cfg(test)]
#[path = "list_command_handler_tests.rs"]
mod list_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum SonarrListCommand {
  #[command(about = "List all items in the Sonarr blocklist")]
  Blocklist,
  #[command(about = "List all series in your Sonarr library")]
  Series,
}

impl From<SonarrListCommand> for Command {
  fn from(value: SonarrListCommand) -> Self {
    Command::Sonarr(SonarrCommand::List(value))
  }
}

pub(super) struct SonarrListCommandHandler<'a, 'b> {
  app: &'a Arc<Mutex<App<'b>>>,
  command: SonarrListCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, SonarrListCommand> for SonarrListCommandHandler<'a, 'b> {
  fn with(
    app: &'a Arc<Mutex<App<'b>>>,
    command: SonarrListCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    SonarrListCommandHandler {
      app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<()> {
    match self.command {
      SonarrListCommand::Blocklist => {
        execute_network_event!(self, SonarrEvent::GetBlocklist);
      }
      SonarrListCommand::Series => {
        execute_network_event!(self, SonarrEvent::ListSeries);
      }
    }

    Ok(())
  }
}
