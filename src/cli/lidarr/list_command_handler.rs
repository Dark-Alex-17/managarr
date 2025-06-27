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
#[path = "list_command_handler_tests.rs"]
mod list_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum LidarrListCommand {
  #[command(about = "List all items in the Lidarr blocklist")]
  Blocklist,
  #[command(about = "List all active downloads in Lidarr")]
  Downloads,
  #[command(about = "List disk space details for all provisioned root folders in Lidarr")]
  DiskSpace,
  #[command(about = "List the tracks for the artist with the given ID")]
  Tracks {
    #[arg(
      long,
      help = "The Lidarr ID of the artist whose tracks you wish to fetch",
      required = true
    )]
    artist_id: i64,
  },
  #[command(about = "List the track files for the artist with the given ID")]
  TrackFiles {
    #[arg(
      long,
      help = "The Lidarr ID of the artist whose track files you wish to fetch",
      required = true
    )]
    artist_id: i64,
  },
  #[command(about = "Fetch all history events for the track with the given ID")]
  TrackHistory {
    #[arg(
      long,
      help = "The Lidarr ID of the track whose history you wish to fetch",
      required = true
    )]
    track_id: i64,
  },
  #[command(about = "Fetch all Lidarr history events")]
  History {
    #[arg(long, help = "How many history events to fetch", default_value_t = 500)]
    events: u64,
  },
  #[command(about = "List all Lidarr indexers")]
  Indexers,
  #[command(about = "List all Lidarr metadata profiles")]
  MetadataProfiles,
  #[command(about = "Fetch Lidarr logs")]
  Logs {
    #[arg(long, help = "How many log events to fetch", default_value_t = 500)]
    events: u64,
    #[arg(
      long,
      help = "Output the logs in the same format as they appear in the log files"
    )]
    output_in_log_format: bool,
  },
  #[command(about = "List all Lidarr quality profiles")]
  QualityProfiles,
  #[command(about = "List all queued events")]
  QueuedEvents,
  #[command(about = "List all root folders in Lidarr")]
  RootFolders,
  #[command(
    about = "Fetch all history events for the given album corresponding to the artist with the given ID."
  )]
  AlbumHistory {
    #[arg(
      long,
      help = "The Lidarr ID of the artist whose history you wish to fetch and list",
      required = true
    )]
    artist_id: i64,
    #[arg(
      long,
      help = "The album ID to fetch history events for",
      required = true
    )]
    album_id: i64,
  },
  #[command(about = "List all artists in your Lidarr library")]
  Artists,
  #[command(about = "Fetch all history events for the artist with the given ID")]
  ArtistHistory {
    #[arg(
      long,
      help = "The Lidarr ID of the artist whose history you wish to fetch",
      required = true
    )]
    artist_id: i64,
  },
  #[command(about = "List all Lidarr tags")]
  Tags,
  #[command(about = "List all Lidarr tasks")]
  Tasks,
  #[command(about = "List all Lidarr updates")]
  Updates,
}

impl From<LidarrListCommand> for Command {
  fn from(value: LidarrListCommand) -> Self {
    Command::Lidarr(LidarrCommand::List(value))
  }
}

pub(super) struct LidarrListCommandHandler<'a, 'b> {
  app: &'a Arc<Mutex<App<'b>>>,
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
      app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<String> {
    let result = match self.command {
      LidarrListCommand::Blocklist => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::GetBlocklist.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrListCommand::Downloads => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::GetDownloads.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrListCommand::DiskSpace => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::GetDiskSpace.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrListCommand::Tracks { artist_id } => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::GetTracks(artist_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrListCommand::TrackFiles { artist_id } => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::GetTrackFiles(artist_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrListCommand::TrackHistory { track_id } => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::GetTrackHistory(track_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrListCommand::History { events: items } => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::GetHistory(items).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrListCommand::Indexers => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::GetIndexers.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrListCommand::MetadataProfiles => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::GetMetadataProfiles.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrListCommand::Logs {
        events,
        output_in_log_format,
      } => {
        let logs = self
          .network
          .handle_network_event(LidarrEvent::GetLogs(events).into())
          .await?;

        if output_in_log_format {
          let log_lines = &self.app.lock().await.data.lidarr_data.logs.items;

          serde_json::to_string_pretty(log_lines)?
        } else {
          serde_json::to_string_pretty(&logs)?
        }
      }
      LidarrListCommand::QualityProfiles => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::GetQualityProfiles.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrListCommand::QueuedEvents => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::GetQueuedEvents.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrListCommand::RootFolders => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::GetRootFolders.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrListCommand::AlbumHistory {
        artist_id,
        album_id,
      } => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::GetAlbumHistory((artist_id, album_id)).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrListCommand::Artists => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::ListArtists.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrListCommand::ArtistHistory { artist_id } => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::GetArtistHistory(artist_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrListCommand::Tags => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::GetTags.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrListCommand::Tasks => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::GetTasks.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      LidarrListCommand::Updates => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::GetUpdates.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
    };

    Ok(result)
  }
}
