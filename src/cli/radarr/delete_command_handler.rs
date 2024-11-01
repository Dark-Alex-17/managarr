use std::sync::Arc;

use anyhow::Result;
use clap::Subcommand;
use tokio::sync::Mutex;

use crate::{
  app::App,
  cli::{CliCommandHandler, Command},
  execute_network_event,
  models::radarr_models::DeleteMovieParams,
  network::{radarr_network::RadarrEvent, NetworkTrait},
};

use super::RadarrCommand;

#[cfg(test)]
#[path = "delete_command_handler_tests.rs"]
mod delete_command_handler_tests;

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum RadarrDeleteCommand {
  #[command(about = "Delete the specified item from the Radarr blocklist")]
  BlocklistItem {
    #[arg(
      long,
      help = "The ID of the blocklist item to remove from the blocklist",
      required = true
    )]
    blocklist_item_id: i64,
  },
  #[command(about = "Delete the specified download")]
  Download {
    #[arg(long, help = "The ID of the download to delete", required = true)]
    download_id: i64,
  },
  #[command(about = "Delete the indexer with the given ID")]
  Indexer {
    #[arg(long, help = "The ID of the indexer to delete", required = true)]
    indexer_id: i64,
  },
  #[command(about = "Delete a movie from your Radarr library")]
  Movie {
    #[arg(long, help = "The ID of the movie to delete", required = true)]
    movie_id: i64,
    #[arg(long, help = "Delete the movie files from disk as well")]
    delete_files_from_disk: bool,
    #[arg(long, help = "Add a list exclusion for this film")]
    add_list_exclusion: bool,
  },
  #[command(about = "Delete the root folder with the given ID")]
  RootFolder {
    #[arg(long, help = "The ID of the root folder to delete", required = true)]
    root_folder_id: i64,
  },
  #[command(about = "Delete the tag with the specified ID")]
  Tag {
    #[arg(long, help = "The ID of the tag to delete", required = true)]
    tag_id: i64,
  },
}

impl From<RadarrDeleteCommand> for Command {
  fn from(value: RadarrDeleteCommand) -> Self {
    Command::Radarr(RadarrCommand::Delete(value))
  }
}

pub(super) struct RadarrDeleteCommandHandler<'a, 'b> {
  _app: &'a Arc<Mutex<App<'b>>>,
  command: RadarrDeleteCommand,
  network: &'a mut dyn NetworkTrait,
}

impl<'a, 'b> CliCommandHandler<'a, 'b, RadarrDeleteCommand> for RadarrDeleteCommandHandler<'a, 'b> {
  fn with(
    _app: &'a Arc<Mutex<App<'b>>>,
    command: RadarrDeleteCommand,
    network: &'a mut dyn NetworkTrait,
  ) -> Self {
    RadarrDeleteCommandHandler {
      _app,
      command,
      network,
    }
  }

  async fn handle(self) -> Result<()> {
    match self.command {
      RadarrDeleteCommand::BlocklistItem { blocklist_item_id } => {
        execute_network_event!(
          self,
          RadarrEvent::DeleteBlocklistItem(Some(blocklist_item_id))
        );
      }
      RadarrDeleteCommand::Download { download_id } => {
        execute_network_event!(self, RadarrEvent::DeleteDownload(Some(download_id)));
      }
      RadarrDeleteCommand::Indexer { indexer_id } => {
        execute_network_event!(self, RadarrEvent::DeleteIndexer(Some(indexer_id)));
      }
      RadarrDeleteCommand::Movie {
        movie_id,
        delete_files_from_disk,
        add_list_exclusion,
      } => {
        let delete_movie_params = DeleteMovieParams {
          id: movie_id,
          delete_movie_files: delete_files_from_disk,
          add_list_exclusion,
        };
        execute_network_event!(self, RadarrEvent::DeleteMovie(Some(delete_movie_params)));
      }
      RadarrDeleteCommand::RootFolder { root_folder_id } => {
        execute_network_event!(self, RadarrEvent::DeleteRootFolder(Some(root_folder_id)));
      }
      RadarrDeleteCommand::Tag { tag_id } => {
        execute_network_event!(self, RadarrEvent::DeleteTag(tag_id));
      }
    }

    Ok(())
  }
}
