use std::sync::Arc;

use clap::Subcommand;
use tokio::sync::Mutex;

use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  network::{sonarr_network::SonarrEvent, NetworkTrait},
};

use super::SonarrCommand;

#[cfg(test)]
#[path = "refresh_command_handler_tests.rs"]
mod refresh_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum SonarrRefreshCommand {
  #[command(about = "Refresh all series data for all series in your Sonarr library")]
  AllSeries,
}

impl From<SonarrRefreshCommand> for Command {
  fn from(value: SonarrRefreshCommand) -> Self {
    Command::Sonarr(SonarrCommand::Refresh(value))
  }
}

pub(super) struct SonarrRefreshCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: SonarrRefreshCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, SonarrRefreshCommand>
  for SonarrRefreshCommandHandler<'a, 'b>
{
  fn with(
    _app: &'a Arc<Mutex<App<'b>>>,
    command: SonarrRefreshCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    SonarrRefreshCommandHandler {
      _app,
      command,
      network,
    }
  }

  async fn handle(self) -> anyhow::Result<String> {
    let result = match self.command {
      SonarrRefreshCommand::AllSeries => {
        let resp = self
          .network
          .handle_network_event(SonarrEvent::UpdateAllSeries.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
    };

    Ok(result)
  }
}
