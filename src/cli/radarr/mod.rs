use std::sync::Arc;

use add_command_handler::{RadarrAddCommand, RadarrAddCommandHandler};
use clap::Subcommand;
use delete_command_handler::{RadarrDeleteCommand, RadarrDeleteCommandHandler};
use edit_command_handler::{RadarrEditCommand, RadarrEditCommandHandler};
use get_command_handler::{RadarrGetCommand, RadarrGetCommandHandler};
use list_command_handler::{RadarrListCommand, RadarrListCommandHandler};
use refresh_command_handler::{RadarrRefreshCommand, RadarrRefreshCommandHandler};
use tokio::sync::Mutex;

use crate::app::App;

use crate::cli::CliCommandHandler;
use crate::execute_network_event;
use crate::models::radarr_models::{ReleaseDownloadBody, TaskName};
use crate::network::radarr_network::RadarrEvent;
use crate::network::NetworkTrait;
use anyhow::Result;

use super::Command;

mod add_command_handler;
mod delete_command_handler;
mod edit_command_handler;
mod get_command_handler;
mod list_command_handler;
mod refresh_command_handler;

#[cfg(test)]
#[path = "radarr_command_tests.rs"]
mod radarr_command_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum RadarrCommand {
  #[command(
    subcommand,
    about = "Commands to add or create new resources within your Radarr instance"
  )]
  Add(RadarrAddCommand),
  #[command(
    subcommand,
    about = "Commands to delete resources from your Radarr instance"
  )]
  Delete(RadarrDeleteCommand),
  #[command(
    subcommand,
    about = "Commands to edit resources in your Radarr instance"
  )]
  Edit(RadarrEditCommand),
  #[command(
    subcommand,
    about = "Commands to fetch details of the resources in your Radarr instance"
  )]
  Get(RadarrGetCommand),
  #[command(
    subcommand,
    about = "Commands to list attributes from your Radarr instance"
  )]
  List(RadarrListCommand),
  #[command(
    subcommand,
    about = "Commands to refresh the data in your Radarr instance"
  )]
  Refresh(RadarrRefreshCommand),
  #[command(about = "Clear the blocklist")]
  ClearBlocklist,
  #[command(about = "Manually download the given release for the specified movie ID")]
  DownloadRelease {
    #[arg(long, help = "The GUID of the release to download", required = true)]
    guid: String,
    #[arg(
      long,
      help = "The indexer ID to download the release from",
      required = true
    )]
    indexer_id: i64,
    #[arg(
      long,
      help = "The movie ID that the release is associated with",
      required = true
    )]
    movie_id: i64,
  },
  #[command(about = "Trigger a manual search of releases for the movie with the given ID")]
  ManualSearch {
    #[arg(
      long,
      help = "The Radarr ID of the movie whose releases you wish to fetch and list",
      required = true
    )]
    movie_id: i64,
  },
  #[command(about = "Search for a new film to add to Radarr")]
  SearchNewMovie {
    #[arg(
      long,
      help = "The title of the film you want to search for",
      required = true
    )]
    query: String,
  },
  #[command(about = "Start the specified Radarr task")]
  StartTask {
    #[arg(
      long,
      help = "The name of the task to trigger",
      value_enum,
      required = true
    )]
    task_name: TaskName,
  },
  #[command(
    about = "Test the indexer with the given ID. Note that a successful test returns an empty JSON body; i.e. '{}'"
  )]
  TestIndexer {
    #[arg(long, help = "The ID of the indexer to test", required = true)]
    indexer_id: i64,
  },
  #[command(about = "Test all indexers")]
  TestAllIndexers,
  #[command(about = "Trigger an automatic search for the movie with the specified ID")]
  TriggerAutomaticSearch {
    #[arg(
      long,
      help = "The ID of the movie you want to trigger an automatic search for",
      required = true
    )]
    movie_id: i64,
  },
}

impl From<RadarrCommand> for Command {
  fn from(radarr_command: RadarrCommand) -> Command {
    Command::Radarr(radarr_command)
  }
}

pub(super) struct RadarrCliHandler<'a, 'b> {
  app: &'a Arc<Mutex<App<'b>>>,
  command: RadarrCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, RadarrCommand> for RadarrCliHandler<'a, 'b> {
  fn with(
    app: &'a Arc<Mutex<App<'b>>>,
    command: RadarrCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    RadarrCliHandler {
      app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<()> {
    match self.command {
      RadarrCommand::Add(add_command) => {
        RadarrAddCommandHandler::with(self.app, add_command, self.network)
          .handle()
          .await?
      }
      RadarrCommand::Delete(delete_command) => {
        RadarrDeleteCommandHandler::with(self.app, delete_command, self.network)
          .handle()
          .await?
      }
      RadarrCommand::Edit(edit_command) => {
        RadarrEditCommandHandler::with(self.app, edit_command, self.network)
          .handle()
          .await?
      }
      RadarrCommand::Get(get_command) => {
        RadarrGetCommandHandler::with(self.app, get_command, self.network)
          .handle()
          .await?
      }
      RadarrCommand::List(list_command) => {
        RadarrListCommandHandler::with(self.app, list_command, self.network)
          .handle()
          .await?
      }
      RadarrCommand::Refresh(update_command) => {
        RadarrRefreshCommandHandler::with(self.app, update_command, self.network)
          .handle()
          .await?
      }
      RadarrCommand::ClearBlocklist => {
        self
          .network
          .handle_network_event(RadarrEvent::GetBlocklist.into())
          .await?;
        execute_network_event!(self, RadarrEvent::ClearBlocklist);
      }
      RadarrCommand::DownloadRelease {
        guid,
        indexer_id,
        movie_id,
      } => {
        let params = ReleaseDownloadBody {
          guid,
          indexer_id,
          movie_id,
        };
        execute_network_event!(self, RadarrEvent::DownloadRelease(Some(params)));
      }
      RadarrCommand::ManualSearch { movie_id } => {
        println!("Searching for releases. This may take a minute...");
        execute_network_event!(self, RadarrEvent::GetReleases(Some(movie_id)));
      }
      RadarrCommand::SearchNewMovie { query } => {
        execute_network_event!(self, RadarrEvent::SearchNewMovie(Some(query)));
      }
      RadarrCommand::StartTask { task_name } => {
        execute_network_event!(self, RadarrEvent::StartTask(Some(task_name)));
      }
      RadarrCommand::TestIndexer { indexer_id } => {
        execute_network_event!(self, RadarrEvent::TestIndexer(Some(indexer_id)));
      }
      RadarrCommand::TestAllIndexers => {
        execute_network_event!(self, RadarrEvent::TestAllIndexers);
      }
      RadarrCommand::TriggerAutomaticSearch { movie_id } => {
        execute_network_event!(self, RadarrEvent::TriggerAutomaticSearch(Some(movie_id)));
      }
    }

    Ok(())
  }
}
