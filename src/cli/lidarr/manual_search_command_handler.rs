use std::sync::Arc;

use anyhow::Result;
use clap::Subcommand;
use tokio::sync::Mutex;

use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  network::{lidarr_network::LidarrEvent, NetworkTrait},
};

use super::LidarrCommand;

#[cfg(test)]
#[path = "manual_search_command_handler_tests.rs"]
mod manual_search_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum LidarrManualSearchCommand {
  #[command(about = "Trigger a manual search of releases for the track with the given ID")]
  Track {
    #[arg(
      long,
      help = "The Lidarr ID of the track whose releases you wish to fetch and list",
      required = true
    )]
    track_id: i64,
  },
  #[command(
    about = "Trigger a manual search of releases for the given album corresponding to the artist with the given ID.\nNote that when downloading an album release, ensure that the release includes 'fullAlbum: true', otherwise you'll run into issues"
  )]
  Album {
    #[arg(
      long,
      help = "The Lidarr ID of the artist whose releases you wish to fetch and list",
      required = true
    )]
    artist_id: i64,
    #[arg(long, help = "The album ID to search for", required = true)]
    album_id: i64,
  },
}

impl From<LidarrManualSearchCommand> for Command {
  fn from(value: LidarrManualSearchCommand) -> Self {
    Command::Lidarr(LidarrCommand::ManualSearch(value))
  }
}

pub(super) struct LidarrManualSearchCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: LidarrManualSearchCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, LidarrManualSearchCommand>
  for LidarrManualSearchCommandHandler<'a, 'b>
{
  fn with(
    _app: &'a Arc<Mutex<App<'b>>>,
    command: LidarrManualSearchCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    LidarrManualSearchCommandHandler {
      _app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<String> {
    let result = match self.command {
      LidarrManualSearchCommand::Track { track_id } => {
        println!("Searching for track releases. This may take a minute...");
        let resp = self
          .network
          .handle_network_event(LidarrEvent::GetTrackReleases(track_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrManualSearchCommand::Album {
        artist_id,
        album_id,
      } => {
        println!("Searching for album releases. This may take a minute...");
        let resp = self
          .network
          .handle_network_event(LidarrEvent::GetAlbumReleases((artist_id, album_id)).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
    };

    Ok(result)
  }
}
