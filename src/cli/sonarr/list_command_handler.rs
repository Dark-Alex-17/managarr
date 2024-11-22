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
#[path = "list_command_handler_tests.rs"]
mod list_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum SonarrListCommand {
  #[command(about = "List all items in the Sonarr blocklist")]
  Blocklist,
  #[command(about = "List all active downloads in Sonarr")]
  Downloads,
  #[command(about = "List the episodes for the series with the given ID")]
  Episodes {
    #[arg(
      long,
      help = "The Sonarr ID of the series whose episodes you wish to fetch",
      required = true
    )]
    series_id: i64,
  },
  #[command(about = "Fetch all history events for the episode with the given ID")]
  EpisodeHistory {
    #[arg(
      long,
      help = "The Sonarr ID of the episode whose history you wish to fetch",
      required = true
    )]
    episode_id: i64,
  },
  #[command(about = "Fetch all Sonarr history events")]
  History {
    #[arg(long, help = "How many history events to fetch", default_value_t = 500)]
    events: u64,
  },
  #[command(about = "List all Sonarr indexers")]
  Indexers,
  #[command(about = "Fetch Sonarr logs")]
  Logs {
    #[arg(long, help = "How many log events to fetch", default_value_t = 500)]
    events: u64,
    #[arg(
      long,
      help = "Output the logs in the same format as they appear in the log files"
    )]
    output_in_log_format: bool,
  },
  #[command(about = "List all Sonarr quality profiles")]
  QualityProfiles,
  #[command(about = "List all queued events")]
  QueuedEvents,
  #[command(about = "List all root folders in Sonarr")]
  RootFolders,
  #[command(about = "List all series in your Sonarr library")]
  Series,
  #[command(about = "Fetch all history events for the series with the given ID")]
  SeriesHistory {
    #[arg(
      long,
      help = "The Sonarr ID of the series whose history you wish to fetch",
      required = true
    )]
    series_id: i64,
  },
  #[command(about = "List all Sonarr tags")]
  Tags,
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

  async fn handle(self) -> Result<String> {
    let result = match self.command {
      SonarrListCommand::Blocklist => {
        let resp = self
          .network
          .handle_network_event(SonarrEvent::GetBlocklist.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      SonarrListCommand::Downloads => {
        let resp = self
          .network
          .handle_network_event(SonarrEvent::GetDownloads.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      SonarrListCommand::Episodes { series_id } => {
        let resp = self
          .network
          .handle_network_event(SonarrEvent::GetEpisodes(Some(series_id)).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      SonarrListCommand::EpisodeHistory { episode_id } => {
        let resp = self
          .network
          .handle_network_event(SonarrEvent::GetEpisodeHistory(Some(episode_id)).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      SonarrListCommand::History { events: items } => {
        let resp = self
          .network
          .handle_network_event(SonarrEvent::GetHistory(Some(items)).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      SonarrListCommand::Indexers => {
        let resp = self
          .network
          .handle_network_event(SonarrEvent::GetIndexers.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      SonarrListCommand::Logs {
        events,
        output_in_log_format,
      } => {
        let logs = self
          .network
          .handle_network_event(SonarrEvent::GetLogs(Some(events)).into())
          .await?;

        if output_in_log_format {
          let log_lines = self.app.lock().await.data.sonarr_data.logs.items.clone();

          serde_json::to_string_pretty(&log_lines)?
        } else {
          serde_json::to_string_pretty(&logs)?
        }
      }
      SonarrListCommand::QualityProfiles => {
        let resp = self
          .network
          .handle_network_event(SonarrEvent::GetQualityProfiles.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      SonarrListCommand::QueuedEvents => {
        let resp = self
          .network
          .handle_network_event(SonarrEvent::GetQueuedEvents.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      SonarrListCommand::RootFolders => {
        let resp = self
          .network
          .handle_network_event(SonarrEvent::GetRootFolders.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      SonarrListCommand::Series => {
        let resp = self
          .network
          .handle_network_event(SonarrEvent::ListSeries.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      SonarrListCommand::SeriesHistory { series_id } => {
        let resp = self
          .network
          .handle_network_event(SonarrEvent::GetSeriesHistory(Some(series_id)).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      SonarrListCommand::Tags => {
        let resp = self
          .network
          .handle_network_event(SonarrEvent::GetTags.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
    };

    Ok(result)
  }
}
