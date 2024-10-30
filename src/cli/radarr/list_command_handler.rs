use std::sync::Arc;

use anyhow::Result;
use clap::{command, Subcommand};
use tokio::sync::Mutex;

use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  execute_network_event,
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
  Downloads,
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
  #[command(about = "List tasks")]
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

  async fn handle(self) -> Result<()> {
    match self.command {
      RadarrListCommand::Blocklist => {
        execute_network_event!(self, RadarrEvent::GetBlocklist);
      }
      RadarrListCommand::Collections => {
        execute_network_event!(self, RadarrEvent::GetCollections);
      }
      RadarrListCommand::Downloads => {
        execute_network_event!(self, RadarrEvent::GetDownloads);
      }
      RadarrListCommand::Indexers => {
        execute_network_event!(self, RadarrEvent::GetIndexers);
      }
      RadarrListCommand::Logs {
        events,
        output_in_log_format,
      } => {
        let logs = self
          .network
          .handle_network_event(RadarrEvent::GetLogs(Some(events)).into())
          .await?;

        if output_in_log_format {
          let log_lines = self.app.lock().await.data.radarr_data.logs.items.clone();

          let json = serde_json::to_string_pretty(&log_lines)?;
          println!("{}", json);
        } else {
          let json = serde_json::to_string_pretty(&logs)?;
          println!("{}", json);
        }
      }
      RadarrListCommand::Movies => {
        execute_network_event!(self, RadarrEvent::GetMovies);
      }
      RadarrListCommand::MovieCredits { movie_id } => {
        execute_network_event!(self, RadarrEvent::GetMovieCredits(Some(movie_id)));
      }
      RadarrListCommand::QualityProfiles => {
        execute_network_event!(self, RadarrEvent::GetQualityProfiles);
      }
      RadarrListCommand::QueuedEvents => {
        execute_network_event!(self, RadarrEvent::GetQueuedEvents);
      }
      RadarrListCommand::RootFolders => {
        execute_network_event!(self, RadarrEvent::GetRootFolders);
      }
      RadarrListCommand::Tags => {
        execute_network_event!(self, RadarrEvent::GetTags);
      }
      RadarrListCommand::Tasks => {
        execute_network_event!(self, RadarrEvent::GetTasks);
      }
      RadarrListCommand::Updates => {
        execute_network_event!(self, RadarrEvent::GetUpdates);
      }
    }

    Ok(())
  }
}
