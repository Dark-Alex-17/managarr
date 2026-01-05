use std::sync::Arc;

use anyhow::Result;
use clap::Subcommand;
use tokio::sync::Mutex;

use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  network::{NetworkTrait, lidarr_network::LidarrEvent},
};

use super::LidarrCommand;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum LidarrListCommand {
  #[command(about = "List all artists in your Lidarr library")]
  Artists,
}

impl From<LidarrListCommand> for Command {
  fn from(value: LidarrListCommand) -> Self {
    Command::Lidarr(LidarrCommand::List(value))
  }
}

pub(super) struct LidarrListCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: LidarrListCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, LidarrListCommand> for LidarrListCommandHandler<'a, 'b> {
  fn with(
    app: &'a Arc<Mutex<App<'b>>>,
    command: LidarrListCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    LidarrListCommandHandler {
      _app: app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<String> {
    let result = match self.command {
      LidarrListCommand::Artists => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::ListArtists.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
    };

    Ok(result)
  }
}
