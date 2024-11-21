use std::sync::Arc;

use anyhow::Result;
use clap::Subcommand;
use tokio::sync::Mutex;

use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  network::{sonarr_network::SonarrEvent, NetworkTrait},
};

use super::SonarrCommand;

#[cfg(test)]
#[path = "delete_command_handler_tests.rs"]
mod delete_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum SonarrDeleteCommand {
  #[command(about = "Delete the specified item from the Sonarr blocklist")]
  BlocklistItem {
    #[arg(
      long,
      help = "The ID of the blocklist item to remove from the blocklist",
      required = true
    )]
    blocklist_item_id: i64,
  },
}

impl From<SonarrDeleteCommand> for Command {
  fn from(value: SonarrDeleteCommand) -> Self {
    Command::Sonarr(SonarrCommand::Delete(value))
  }
}

pub(super) struct SonarrDeleteCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: SonarrDeleteCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, SonarrDeleteCommand> for SonarrDeleteCommandHandler<'a, 'b> {
  fn with(
    _app: &'a Arc<Mutex<App<'b>>>,
    command: SonarrDeleteCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    SonarrDeleteCommandHandler {
      _app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<String> {
    let resp = match self.command {
      SonarrDeleteCommand::BlocklistItem { blocklist_item_id } => {
        let resp = self
          .network
          .handle_network_event((SonarrEvent::DeleteBlocklistItem(Some(blocklist_item_id))).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
    };

    Ok(resp)
  }
}
