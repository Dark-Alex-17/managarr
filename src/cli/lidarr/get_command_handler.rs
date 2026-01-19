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

#[cfg(test)]
#[path = "get_command_handler_tests.rs"]
mod get_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum LidarrGetCommand {
  #[command(about = "Get detailed information for the album with the given ID")]
  AlbumDetails {
    #[arg(
      long,
      help = "The Lidarr ID of the album whose details you wish to fetch",
      required = true
    )]
    album_id: i64,
  },
  #[command(about = "Get the shared settings for all indexers")]
  AllIndexerSettings,
  #[command(about = "Get detailed information for the artist with the given ID")]
  ArtistDetails {
    #[arg(
      long,
      help = "The Lidarr ID of the artist whose details you wish to fetch",
      required = true
    )]
    artist_id: i64,
  },
  #[command(about = "Fetch the host config for your Lidarr instance")]
  HostConfig,
  #[command(about = "Fetch the security config for your Lidarr instance")]
  SecurityConfig,
  #[command(about = "Get the system status")]
  SystemStatus,
  #[command(about = "Get detailed information for the track with the given ID")]
  TrackDetails {
    #[arg(
      long,
      help = "The Lidarr ID of the track whose details you wish to fetch",
      required = true
    )]
    track_id: i64,
  },
}

impl From<LidarrGetCommand> for Command {
  fn from(value: LidarrGetCommand) -> Self {
    Command::Lidarr(LidarrCommand::Get(value))
  }
}

pub(super) struct LidarrGetCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: LidarrGetCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, LidarrGetCommand> for LidarrGetCommandHandler<'a, 'b> {
  fn with(
    _app: &'a Arc<Mutex<App<'b>>>,
    command: LidarrGetCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    LidarrGetCommandHandler {
      _app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<String> {
    let result = match self.command {
      LidarrGetCommand::AlbumDetails { album_id } => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::GetAlbumDetails(album_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrGetCommand::AllIndexerSettings => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::GetAllIndexerSettings.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrGetCommand::ArtistDetails { artist_id } => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::GetArtistDetails(artist_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrGetCommand::HostConfig => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::GetHostConfig.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrGetCommand::SecurityConfig => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::GetSecurityConfig.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrGetCommand::SystemStatus => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::GetStatus.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrGetCommand::TrackDetails { track_id } => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::GetTrackDetails(track_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
    };

    Ok(result)
  }
}
