use std::sync::Arc;

use anyhow::Result;
use clap::{Subcommand, arg};
use tokio::sync::Mutex;

use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  network::{NetworkTrait, lidarr_network::LidarrEvent},
};

use super::LidarrCommand;

#[cfg(test)]
#[path = "list_command_handler_tests.rs"]
mod list_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum LidarrListCommand {
  #[command(about = "List all albums for the artist with the given ID")]
  Albums {
    #[arg(
      long,
      help = "The Lidarr ID of the artist whose albums you want to list",
      required = true
    )]
    artist_id: i64,
  },
  #[command(about = "List all artists in your Lidarr library")]
  Artists,
  #[command(about = "List all active downloads in Lidarr")]
  Downloads {
    #[arg(long, help = "How many downloads to fetch", default_value_t = 500)]
    count: u64,
  },
  #[command(about = "Fetch all Lidarr history events")]
  History {
    #[arg(long, help = "How many history events to fetch", default_value_t = 500)]
    events: u64,
  },
  #[command(about = "List all Lidarr indexers")]
  Indexers,
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
  #[command(about = "List all Lidarr metadata profiles")]
  MetadataProfiles,
  #[command(about = "List all Lidarr quality profiles")]
  QualityProfiles,
  #[command(about = "List all queued events")]
  QueuedEvents,
  #[command(about = "List all root folders in Lidarr")]
  RootFolders,
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
      LidarrListCommand::Albums { artist_id } => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::GetAlbums(artist_id).into())
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
      LidarrListCommand::Downloads { count } => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::GetDownloads(count).into())
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
      LidarrListCommand::Logs {
        events,
        output_in_log_format,
      } => {
        let logs = self
          .network
          .handle_network_event(LidarrEvent::GetLogs(events).into())
          .await?;

        if output_in_log_format {
          let log_lines = &self.app.lock().await.data.sonarr_data.logs.items;

          serde_json::to_string_pretty(log_lines)?
        } else {
          serde_json::to_string_pretty(&logs)?
        }
      }
      LidarrListCommand::MetadataProfiles => {
        let resp = self
          .network
          .handle_network_event(LidarrEvent::GetMetadataProfiles.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
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
