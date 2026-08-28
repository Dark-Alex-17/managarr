use std::sync::Arc;

use anyhow::Result;
use clap::Subcommand;
use tokio::sync::Mutex;

use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  network::{NetworkTrait, readarr_network::ReadarrEvent},
};

use super::ReadarrCommand;

#[cfg(test)]
#[path = "refresh_command_handler_tests.rs"]
mod refresh_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum ReadarrRefreshCommand {
  #[command(about = "Refresh all author data for all authors in your Readarr library")]
  AllAuthors,
  #[command(about = "Refresh author data and scan disk for the author with the given ID")]
  Author {
    #[arg(
      long,
      help = "The ID of the author to refresh information on and to scan the disk for",
      required = true
    )]
    author_id: i64,
  },
  #[command(about = "Refresh all downloads in Readarr")]
  Downloads,
}

impl From<ReadarrRefreshCommand> for Command {
  fn from(value: ReadarrRefreshCommand) -> Self {
    Command::Readarr(ReadarrCommand::Refresh(value))
  }
}

pub(super) struct ReadarrRefreshCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: ReadarrRefreshCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, ReadarrRefreshCommand>
  for ReadarrRefreshCommandHandler<'a, 'b>
{
  fn with(
    _app: &'a Arc<Mutex<App<'b>>>,
    command: ReadarrRefreshCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    ReadarrRefreshCommandHandler {
      _app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<String> {
    let result = match self.command {
      ReadarrRefreshCommand::AllAuthors => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::UpdateAllAuthors.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      ReadarrRefreshCommand::Author { author_id } => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::UpdateAndScanAuthor(author_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      ReadarrRefreshCommand::Downloads => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::UpdateDownloads.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
    };

    Ok(result)
  }
}
