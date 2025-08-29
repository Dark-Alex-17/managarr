use std::sync::Arc;

use anyhow::Result;
use clap::{command, Subcommand};
use tokio::sync::Mutex;

use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  network::{radarr_network::RadarrEvent, NetworkTrait},
};

use super::RadarrCommand;

#[cfg(test)]
#[path = "list_command_handler_tests.rs"]
mod list_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum RadarrListCommand {
  #[command(about = "List all items in the Radarr blocklist")]
  Blocklist,
  #[command(about = "List all Radarr collections")]
  Collections,
  #[command(about = "List all active downloads in Radarr")]
  Downloads {
    #[arg(long, help = "How many downloads to fetch", default_value_t = 500)]
    count: u64,
  },
  #[command(about = "List disk space details for all provisioned root folders in Radarr")]
  DiskSpace,
  #[command(about = "List all Radarr indexers")]
  Indexers,
  #[command(about = "Fetch Radarr logs")]
  Logs {
    #[arg(long, help = "How many log events to fetch", default_value_t = 500)]
    events: u64,
    #[arg(
      long,
      help = "Output the logs in the same format as they appear in the log files"
    )]
    output_in_log_format: bool,
  },
  #[command(about = "List all movies in your Radarr library")]
  Movies,
  #[command(about = "Get the credits for the movie with the given ID")]
  MovieCredits {
    #[arg(
      long,
      help = "The Radarr ID of the movie whose credits you wish to fetch",
      required = true
    )]
    movie_id: i64,
  },
  #[command(about = "List all Radarr quality profiles")]
  QualityProfiles,
  #[command(about = "List all queued events")]
  QueuedEvents,
  #[command(about = "List all root folders in Radarr")]
  RootFolders,
  #[command(about = "List all Radarr tags")]
  Tags,
  #[command(about = "List all Radarr tasks")]
  Tasks,
  #[command(about = "List all Radarr updates")]
  Updates,
}

impl From<RadarrListCommand> for Command {
  fn from(value: RadarrListCommand) -> Self {
    Command::Radarr(RadarrCommand::List(value))
  }
}

pub(super) struct RadarrListCommandHandler<'a, 'b> {
  app: &'a Arc<Mutex<App<'b>>>,
  command: RadarrListCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, RadarrListCommand> for RadarrListCommandHandler<'a, 'b> {
  fn with(
    app: &'a Arc<Mutex<App<'b>>>,
    command: RadarrListCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    RadarrListCommandHandler {
      app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<String> {
    let result = match self.command {
      RadarrListCommand::Blocklist => {
        let resp = self
          .network
          .handle_network_event(RadarrEvent::GetBlocklist.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      RadarrListCommand::Collections => {
        let resp = self
          .network
          .handle_network_event(RadarrEvent::GetCollections.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      RadarrListCommand::Downloads { count } => {
        let resp = self
          .network
          .handle_network_event(RadarrEvent::GetDownloads(count).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      RadarrListCommand::DiskSpace => {
        let resp = self
          .network
          .handle_network_event(RadarrEvent::GetDiskSpace.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      RadarrListCommand::Indexers => {
        let resp = self
          .network
          .handle_network_event(RadarrEvent::GetIndexers.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      RadarrListCommand::Logs {
        events,
        output_in_log_format,
      } => {
        let logs = self
          .network
          .handle_network_event(RadarrEvent::GetLogs(events).into())
          .await?;

        if output_in_log_format {
          let log_lines = &self.app.lock().await.data.radarr_data.logs.items;

          serde_json::to_string_pretty(log_lines)?
        } else {
          serde_json::to_string_pretty(&logs)?
        }
      }
      RadarrListCommand::Movies => {
        let resp = self
          .network
          .handle_network_event(RadarrEvent::GetMovies.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      RadarrListCommand::MovieCredits { movie_id } => {
        let resp = self
          .network
          .handle_network_event(RadarrEvent::GetMovieCredits(movie_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      RadarrListCommand::QualityProfiles => {
        let resp = self
          .network
          .handle_network_event(RadarrEvent::GetQualityProfiles.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      RadarrListCommand::QueuedEvents => {
        let resp = self
          .network
          .handle_network_event(RadarrEvent::GetQueuedEvents.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      RadarrListCommand::RootFolders => {
        let resp = self
          .network
          .handle_network_event(RadarrEvent::GetRootFolders.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      RadarrListCommand::Tags => {
        let resp = self
          .network
          .handle_network_event(RadarrEvent::GetTags.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      RadarrListCommand::Tasks => {
        let resp = self
          .network
          .handle_network_event(RadarrEvent::GetTasks.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      RadarrListCommand::Updates => {
        let resp = self
          .network
          .handle_network_event(RadarrEvent::GetUpdates.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
    };

    Ok(result)
  }
}
