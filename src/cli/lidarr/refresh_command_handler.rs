use std::sync::Arc;

use clap::Subcommand;
use tokio::sync::Mutex;

use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  network::{NetworkTrait, lidarr_network::LidarrEvent},
};

use super::LidarrCommand;

#[cfg(test)]
#[path = "refresh_command_handler_tests.rs"]
mod refresh_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum LidarrRefreshCommand {
  #[command(about = "Refresh all artist data for all artists in your Lidarr library")]
  AllArtists,
  #[command(about = "Refresh artist data and scan disk for the artist with the given ID")]
  Artist {
    #[arg(
      long,
      help = "The ID of the artist to refresh information on and to scan the disk for",
      required = true
    )]
    artist_id: i64,
  },
  #[command(about = "Refresh all downloads in Lidarr")]
  Downloads,
}

impl From<LidarrRefreshCommand> for Command {
  fn from(value: LidarrRefreshCommand) -> Self {
    Command::Lidarr(LidarrCommand::Refresh(value))
  }
}

pub(super) struct LidarrRefreshCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: LidarrRefreshCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, LidarrRefreshCommand>
  for LidarrRefreshCommandHandler<'a, 'b>
{
  fn with(
    _app: &'a Arc<Mutex<App<'b>>>,
    command: LidarrRefreshCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    LidarrRefreshCommandHandler {
      _app,
      command,
      network,
    }
  }

  async fn handle(self) -> anyhow::Result<String> {
    let result = match self.command {
      LidarrRefreshCommand::AllArtists => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::UpdateAllArtists.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrRefreshCommand::Artist { artist_id } => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::UpdateAndScanArtist(artist_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrRefreshCommand::Downloads => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::UpdateDownloads.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
    };

    Ok(result)
  }
}
