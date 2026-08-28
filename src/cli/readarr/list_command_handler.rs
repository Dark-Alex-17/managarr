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
  #[command(about = "Fetch all history events for the author with the given ID")]
  AuthorHistory {
    #[arg(
      long,
      help = "The Readarr ID of the author whose history you wish to fetch",
      required = true
    )]
    author_id: i64,
  },
  #[command(about = "List all authors in your Readarr library")]
  Authors,
  #[command(about = "List all editions for the book with the given ID")]
  BookEditions {
    #[arg(
      long,
      help = "The Readarr ID of the book whose editions you want to list",
      required = true
    )]
    book_id: i64,
  },
  #[command(about = "List all book files for the book with the given ID")]
  BookFiles {
    #[arg(
      long,
      help = "The Readarr ID of the book whose book files you want to list",
      required = true
    )]
    book_id: i64,
  },
  #[command(about = "Fetch all history events for the book with the given ID")]
  BookHistory {
    #[arg(
      long,
      help = "The Readarr ID of the author who wrote the book whose history you wish to fetch",
      required = true
    )]
    author_id: i64,
    #[arg(
      long,
      help = "The Readarr ID of the book whose history you wish to fetch",
      required = true
    )]
    book_id: i64,
  },
  #[command(about = "List all books for the author with the given ID")]
  Books {
    #[arg(
      long,
      help = "The Readarr ID of the author whose books you want to list",
      required = true
    )]
    author_id: i64,
  },
  #[command(about = "List disk space details for all provisioned root folders in Readarr")]
  DiskSpace,
  #[command(about = "List all active downloads in Readarr")]
  Downloads {
    #[arg(long, help = "How many downloads to fetch", default_value_t = 500)]
    count: u64,
  },
  #[command(about = "Fetch all Readarr history events")]
  History {
    #[arg(long, help = "How many history events to fetch", default_value_t = 500)]
    events: u64,
  },
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
  #[command(about = "Trigger a manual search for releases of the book with the given ID")]
  Releases {
    #[arg(
      long,
      help = "The Readarr ID of the book whose releases you wish to search for",
      required = true
    )]
    book_id: i64,
  },
  #[command(about = "List all root folders in Readarr")]
  RootFolders,
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
      ReadarrListCommand::AuthorHistory { author_id } => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::GetAuthorHistory(author_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      ReadarrListCommand::Authors => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::ListAuthors.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      ReadarrListCommand::BookEditions { book_id } => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::GetBookEditions(book_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      ReadarrListCommand::BookFiles { book_id } => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::GetBookFiles(book_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      ReadarrListCommand::BookHistory { author_id, book_id } => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::GetBookHistory(author_id, book_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      ReadarrListCommand::Books { author_id } => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::GetBooks(author_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      ReadarrListCommand::DiskSpace => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::GetDiskSpace.into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      ReadarrListCommand::Downloads { count } => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::GetDownloads(count).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      ReadarrListCommand::History { events } => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::GetHistory(events).into())
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
      ReadarrListCommand::Releases { book_id } => {
        println!("Searching for book releases. This may take a minute...");
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::GetBookReleases(book_id).into())
          .await?;
        serde_json::to_string_pretty(&resp)?
      }
      ReadarrListCommand::RootFolders => {
        let resp = self
          .network
          .handle_network_event(ReadarrEvent::GetRootFolders.into())
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
