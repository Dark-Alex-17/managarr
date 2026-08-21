use std::sync::Arc;

use anyhow::Result;
use clap::Subcommand;
use tokio::sync::Mutex;

use super::ReadarrCommand;
use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  network::{NetworkTrait, readarr_network::ReadarrEvent},
};

#[cfg(test)]
#[path = "list_command_handler_tests.rs"]
mod list_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum ReadarrListCommand {
  #[command(about = "List disk space details for all provisioned root folders in Readarr")]
  DiskSpace,
  #[command(about = "Fetch Readarr logs")]
  Logs {
    #[arg(long, help = "How many log events to fetch", default_value_t = 500)]
    events: u64,
    #[arg(
      long,
      help = "Output the logs in the same format as they appear in the log files"
    )]
    output_in_log_format: bool,
  },
  #[command(about = "List all Readarr metadata profiles")]
  MetadataProfiles,
  #[command(about = "List all Readarr quality profiles")]
  QualityProfiles,
  #[command(about = "List all queued events")]
  QueuedEvents,
  #[command(about = "List all Readarr tags")]
  Tags,
  #[command(about = "List all Readarr tasks")]
  Tasks,
  #[command(about = "List all Readarr updates")]
  Updates,
}

impl From<ReadarrListCommand> for Command {
  fn from(value: ReadarrListCommand) -> Self {
    Command::Readarr(ReadarrCommand::List(value))
  }
}

pub(super) struct ReadarrListCommandHandler<'a, 'b> {
  app: &'a Arc<Mutex<App<'b>>>,
  command: ReadarrListCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, ReadarrListCommand> for ReadarrListCommandHandler<'a, 'b> {
  fn with(
    app: &'a Arc<Mutex<App<'b>>>,
    command: ReadarrListCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    ReadarrListCommandHandler {
      app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<String> {
    let result = match self.command {
      ReadarrListCommand::DiskSpace => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::GetDiskSpace.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      ReadarrListCommand::Logs {
        events,
        output_in_log_format,
      } => {
        let logs = self
          .network
          .handle_network_event(ReadarrEvent::GetLogs(events).into())
          .await?;

        if output_in_log_format {
          let log_lines = &self.app.lock().await.data.readarr_data.logs.items;

          serde_json::to_string_pretty(log_lines)?
        } else {
          serde_json::to_string_pretty(&logs)?
        }
      }
      ReadarrListCommand::MetadataProfiles => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::GetMetadataProfiles.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      ReadarrListCommand::QualityProfiles => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::GetQualityProfiles.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      ReadarrListCommand::QueuedEvents => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::GetQueuedEvents.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      ReadarrListCommand::Tags => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::GetTags.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      ReadarrListCommand::Tasks => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::GetTasks.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      ReadarrListCommand::Updates => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::GetUpdates.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
    };

    Ok(result)
  }
}
